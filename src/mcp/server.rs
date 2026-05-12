// Copyright (c) 2026 Constantin Alexander <constantin@dedomena.io>. All rights reserved.
// Use of this source code is governed by the Apache License 2.0 that can be
// found in the LICENSE file.

use anyhow::{Context, Result};
use axum::{routing::post, Router, Json, extract::State};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use std::path::PathBuf;

use crate::core::dojo::Dojo;
use crate::core::vfs::VirtualFileSystem;
use crate::core::history::OpType;

#[derive(serde::Deserialize)]
struct McpRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    id: u64,
    method: String,
    params: Option<Value>,
}

#[derive(serde::Deserialize)]
struct CallParams {
    name: String,
    arguments: Option<Value>,
}

struct AppState {
    dojo: Mutex<Dojo>,
    workspace_dir: PathBuf,
}

async fn handle_rpc(State(state): State<Arc<AppState>>, Json(req): Json<McpRequest>) -> Json<Value> {
    match req.method.as_str() {
        "tools/list" => {
            Json(json!({
                "jsonrpc": "2.0",
                "id": req.id,
                "result": {
                    "tools": [
                        {
                            "name": "kungfu_read",
                            "description": "Read the current state of a file from the CRDT DNA.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "path": { "type": "string" }
                                },
                                "required": ["path"]
                            }
                        },
                        {
                            "name": "kungfu_move",
                            "description": "Move or rename a file/directory in the VFS.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "old_path": { "type": "string" },
                                    "new_parent_path": { "type": "string" },
                                    "new_name": { "type": "string" }
                                },
                                "required": ["old_path", "new_parent_path", "new_name"]
                            }
                        },
                        {
                            "name": "kungfu_create",
                            "description": "Create a new file or directory in the VFS.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "path": { "type": "string" },
                                    "kind": { "type": "string", "enum": ["file", "dir"] }
                                },
                                "required": ["path", "kind"]
                            }
                        },
                        {
                            "name": "kungfu_list",
                            "description": "List all files and directories in the CRDT DNA.",
                            "inputSchema": { "type": "object", "properties": {} }
                        },
                        {
                            "name": "kungfu_patch",
                            "description": "Surgically edit a file by finding a specific block of code and replacing it. More robust than counting offsets.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "path": { "type": "string" },
                                    "find": { "type": "string" },
                                    "replace": { "type": "string" }
                                },
                                "required": ["path", "find", "replace"]
                            }
                        },
                        {
                            "name": "kungfu_splice",
                            "description": "Surgically edit the CRDT codebase.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "path": { "type": "string" },
                                    "offset": { "type": "integer" },
                                    "delete_len": { "type": "integer" },
                                    "insert_text": { "type": "string" }
                                },
                                "required": ["path", "offset", "delete_len", "insert_text"]
                            }
                        }
                    ]
                }
            }))
        }
        "tools/call" => {
            if let Some(params_val) = req.params
                && let Ok(call) = serde_json::from_value::<CallParams>(params_val) {
                    println!("Waiting for lock..."); let dojo = state.dojo.lock().await; println!("Lock acquired.");
                    let vfs = VirtualFileSystem::new(&dojo.doc);

                    if call.name == "kungfu_list" {
                        match vfs.list_nodes() {
                            Ok(nodes) => {
                                let mut text = String::from("Files in DNA:\n");
                                for (p, kind) in nodes {
                                    text.push_str(&format!("- {} [{}]\n", p, kind));
                                }
                                return Json(json!({ "jsonrpc": "2.0", "id": req.id, "result": { "content": [{ "type": "text", "text": text }] } }));
                            }
                            Err(e) => {
                                return Json(json!({ "jsonrpc": "2.0", "id": req.id, "error": { "code": -32000, "message": e.to_string() } }));
                            }
                        }
                    }

                    if call.name == "kungfu_patch" {
                        if let Some(args) = call.arguments {
                            let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
                            let find = args.get("find").and_then(|v| v.as_str()).unwrap_or("");
                            let replace = args.get("replace").and_then(|v| v.as_str()).unwrap_or("");

                            match vfs.find_by_path(path) {
                                Ok(tid) => {
                                    match vfs.patch("active_intent", tid, find, replace) {
                                        Ok(trace) => {
                                            let trace_id = trace.0.id.clone();
                                            drop(dojo);
                                            let log_path = state.workspace_dir.join(".kungfu/ops.log");
                                            tokio::spawn(async move {
                                                use tokio::io::AsyncWriteExt;
                                                if let Ok(mut file) = tokio::fs::OpenOptions::new().append(true).open(log_path).await {
                                                    let trace_bytes = bincode::serde::encode_to_vec(&trace.0, bincode::config::standard()).unwrap_or_default();
                                                    let _ = file.write_all(&trace_bytes).await;
                                                }
                                            });
                                            return Json(json!({ "jsonrpc": "2.0", "id": req.id, "result": { "content": [{ "type": "text", "text": format!("Patched successfully at trace {}", trace_id) }] } }));
                                        }
                                        Err(e) => { return Json(json!({ "jsonrpc": "2.0", "id": req.id, "error": { "code": -32000, "message": e.to_string() } })); }
                                    }
                                }
                                Err(e) => { return Json(json!({ "jsonrpc": "2.0", "id": req.id, "error": { "code": -32000, "message": e.to_string() } })); }
                            }
                        }
                    }

                    if call.name == "kungfu_read" {
                        let path = call.arguments.as_ref().and_then(|a| a.get("path")).and_then(|v| v.as_str()).unwrap_or("");
                        match vfs.find_by_path(path) {
                            Ok(tid) => {
                                let content = vfs.read_by_id(tid);
                                return Json(json!({
                                    "jsonrpc": "2.0",
                                    "id": req.id,
                                    "result": { "content": [{ "type": "text", "text": content }] }
                                }));
                            }
                            Err(e) => {
                                return Json(json!({
                                    "jsonrpc": "2.0",
                                    "id": req.id,
                                    "error": { "code": -32000, "message": format!("Read failed: {}", e) }
                                }));
                            }
                        }
                    }

                    if call.name == "kungfu_move"
                        && let Some(args) = call.arguments {
                            let old_path = args.get("old_path").and_then(|v| v.as_str()).unwrap_or("");
                            let new_parent = args.get("new_parent_path").and_then(|v| v.as_str()).unwrap_or("");
                            let new_name = args.get("new_name").and_then(|v| v.as_str()).unwrap_or("");

                            let dojo = state.dojo.lock().await;
                            let vfs = VirtualFileSystem::new(&dojo.doc);
                            
                            match vfs.find_by_path(old_path) {
                                Ok(tid) => {
                                    // 1. Rename metadata
                                    if let Ok(meta) = dojo.doc.get_tree("vfs").get_meta(tid) {
                                        let _ = meta.insert("name", new_name);
                                    }
                                    
                                    // 2. Move node
                                    let mut new_parent_id = "".to_string();
                                    if !new_parent.is_empty()
                                        && let Ok(pid) = vfs.find_by_path(new_parent) {
                                            new_parent_id = pid.to_string();
                                        }

                                    let op = OpType::Move { new_parent: new_parent_id };
                                    match vfs.express("active_intent", Some(tid), op) {
                                        Ok(trace) => {
                                            let trace_bytes = bincode::serde::encode_to_vec(&trace.0, bincode::config::standard()).unwrap_or_default();
                                            drop(dojo);
                                            let log_path = state.workspace_dir.join(".kungfu/ops.log");
                                            tokio::spawn(async move {
                                                use tokio::io::AsyncWriteExt;
                                                if let Ok(mut file) = tokio::fs::OpenOptions::new().append(true).open(log_path).await {
                                                    let _ = file.write_all(&trace_bytes).await;
                                                }
                                            });
                                            return Json(json!({ "jsonrpc": "2.0", "id": req.id, "result": { "content": [{ "type": "text", "text": format!("Moved to {} at trace {}", new_name, trace.0.id) }] } }));
                                        }
                                        Err(e) => { return Json(json!({ "jsonrpc": "2.0", "id": req.id, "error": { "code": -32000, "message": e.to_string() } })); }
                                    }
                                }
                                Err(e) => { return Json(json!({ "jsonrpc": "2.0", "id": req.id, "error": { "code": -32000, "message": e.to_string() } })); }
                            }
                        }

                    if call.name == "kungfu_create"
                        && let Some(args) = call.arguments {
                            let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
                            let kind = args.get("kind").and_then(|v| v.as_str()).unwrap_or("file");

                            // Removed double lock
                            let vfs = VirtualFileSystem::new(&dojo.doc);
                            
                            // Using the formatted string trick to pass name:kind
                            let op = OpType::Create { kind: format!("{}:{}", path, kind) };
                            match vfs.express("active_intent", None, op) {
                                Ok(trace) => {
                                    let trace_id = trace.0.id.clone();
                                    let trace_bytes = bincode::serde::encode_to_vec(&trace.0, bincode::config::standard()).unwrap_or_default();
                                    drop(dojo);
                                    
                                    let log_path = state.workspace_dir.join(".kungfu/ops.log");
                                    tokio::spawn(async move {
                                        use tokio::io::AsyncWriteExt;
                                        if let Ok(mut file) = tokio::fs::OpenOptions::new().append(true).open(log_path).await {
                                            let _ = file.write_all(&trace_bytes).await;
                                        }
                                    });

                                    return Json(json!({
                                        "jsonrpc": "2.0", "id": req.id,
                                        "result": { "content": [{ "type": "text", "text": format!("Created {} at trace {}", path, trace_id) }] }
                                    }));
                                }
                                Err(e) => {
                                    return Json(json!({ "jsonrpc": "2.0", "id": req.id, "error": { "code": -32000, "message": e.to_string() } }));
                                }
                            }
                        }

                    if call.name == "kungfu_splice"
                        && let Some(args) = call.arguments {
                            let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
                            let offset = args.get("offset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                            let delete_len = args.get("delete_len").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                            let insert_text = args.get("insert_text").and_then(|v| v.as_str()).unwrap_or("");

                            match vfs.find_by_path(path) {
                                Ok(tid) => {
                                    let op = OpType::Splice { offset, delete_len, insert: insert_text.to_string() };
                                    match vfs.express("active_intent", Some(tid), op) {
                                        Ok(trace) => {
                                            // GAP-003 FIX: Append to the WAL instead of full snapshot
                                            let trace_bytes = bincode::serde::encode_to_vec(&trace.0, bincode::config::standard()).unwrap_or_default();
                                            drop(dojo); println!("Lock released.");
                                            
                                            let log_path = state.workspace_dir.join(".kungfu/ops.log");
                                            tokio::spawn(async move {
                                                use tokio::io::AsyncWriteExt;
                                                if let Ok(mut file) = tokio::fs::OpenOptions::new().append(true).open(log_path).await {
                                                    let _ = file.write_all(&trace_bytes).await;
                                                }
                                            });

                                            return Json(json!({
                                                "jsonrpc": "2.0", "id": req.id,
                                                "result": { "content": [{ "type": "text", "text": format!("Committed trace {}", trace.0.id) }] }
                                            }));
                                        }
                                        Err(e) => {
                                            return Json(json!({ "jsonrpc": "2.0", "id": req.id, "error": { "code": -32000, "message": e.to_string() } }));
                                        }
                                    }
                                }
                                Err(e) => {
                                    return Json(json!({ "jsonrpc": "2.0", "id": req.id, "error": { "code": -32000, "message": e.to_string() } }));
                                }
                            }
                        }
                }
            Json(json!({ "jsonrpc": "2.0", "id": req.id, "error": { "code": -32602, "message": "Invalid parameters" } }))
        }
        _ => { Json(json!({ "jsonrpc": "2.0", "id": req.id, "error": { "code": -32601, "message": "Method not found" } })) }
    }
}

pub async fn run_server() -> Result<()> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let kf_dir = current_dir.join(".kungfu");
    let dojo = if kf_dir.exists() { Dojo::load(&kf_dir.join("snapshot.loro")).unwrap_or_else(|_| Dojo::new()) } else { Dojo::new() };
    let state = Arc::new(AppState { dojo: Mutex::new(dojo), workspace_dir: current_dir });
    let app = Router::new().route("/mcp", post(handle_rpc)).with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8766));
    println!("Starting KungFu MCP Gateway on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

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
use loro::TreeID;

#[derive(serde::Deserialize)]
struct McpRequest {
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
            if let Some(params_val) = req.params {
                if let Ok(call) = serde_json::from_value::<CallParams>(params_val) {
                    if call.name == "kungfu_splice" {
                        if let Some(args) = call.arguments {
                            let _path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
                            let offset = args.get("offset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                            let delete_len = args.get("delete_len").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                            let insert_text = args.get("insert_text").and_then(|v| v.as_str()).unwrap_or("");

                            // Execute the Splice in the Dojo
                            let dojo = state.dojo.lock().await;
                            let vfs = VirtualFileSystem::new(&dojo.doc);
                            
                            // Mocking the TreeID lookup for now
                            let mock_target_id = TreeID::new(loro::PeerID::MAX, 0); 
                            
                            let op = OpType::Splice { 
                                offset, 
                                delete_len, 
                                insert: insert_text.to_string() 
                            };
                            
                            match vfs.express("active_intent", Some(mock_target_id), op) {
                                Ok(trace) => {
                                    let snap_path = state.workspace_dir.join(".kungfu/snapshot.loro");
                                    let _ = dojo.save(&snap_path);

                                    return Json(json!({
                                        "jsonrpc": "2.0",
                                        "id": req.id,
                                        "result": {
                                            "content": [{
                                                "type": "text",
                                                "text": format!("Operation trace {} successfully committed to the CRDT DNA.", trace.id)
                                            }]
                                        }
                                    }));
                                }
                                Err(e) => {
                                    return Json(json!({
                                        "jsonrpc": "2.0",
                                        "id": req.id,
                                        "error": { "code": -32000, "message": format!("CRDT Splice failed: {}", e) }
                                    }));
                                }
                            }
                        }
                    }
                }
            }
            Json(json!({
                "jsonrpc": "2.0",
                "id": req.id,
                "error": { "code": -32602, "message": "Invalid tool parameters" }
            }))
        }
        _ => {
            Json(json!({
                "jsonrpc": "2.0",
                "id": req.id,
                "error": { "code": -32601, "message": "Method not found" }
            }))
        }
    }
}

pub async fn run_server() -> Result<()> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let kf_dir = current_dir.join(".kungfu");
    
    let dojo = if kf_dir.exists() {
        Dojo::load(&kf_dir.join("snapshot.loro")).unwrap_or_else(|_| Dojo::new())
    } else {
        Dojo::new()
    };

    let state = Arc::new(AppState {
        dojo: Mutex::new(dojo),
        workspace_dir: current_dir,
    });

    let app = Router::new()
        .route("/mcp", post(handle_rpc))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8766));
    println!("Starting KungFu MCP Gateway on {}", addr);
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

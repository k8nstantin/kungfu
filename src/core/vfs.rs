use anyhow::{Context, Result};
use loro::{LoroDoc, LoroTree, TreeID};
use crate::core::history::{OpType, OperationTrace};
use crate::pkg::id;
use std::fs;
use std::path::{Path, PathBuf};

pub struct VirtualFileSystem<'a> {
    doc: &'a LoroDoc,
    tree: LoroTree,
}

impl<'a> VirtualFileSystem<'a> {
    pub fn new(doc: &'a LoroDoc) -> Self {
        let tree = doc.get_tree("vfs");
        VirtualFileSystem { doc, tree }
    }

    pub fn express(&self, intent_id: &str, target_id: Option<TreeID>, op: OpType) -> Result<OperationTrace> {
        let actual_target_id = match &op {
            OpType::Splice { offset, delete_len, insert } => {
                let tid = target_id.context("Target TreeID required for Splice")?;
                let text_id = format!("file_{}", tid);
                let text = self.doc.get_text(text_id.as_str());
                text.splice(*offset, *delete_len, insert)
                    .map_err(|e| anyhow::anyhow!("CRDT Splice failed: {:?}", e))?;
                tid
            }

            OpType::WriteBinary { payload } => {
                let tid = target_id.context("Target TreeID required for WriteBinary")?;
                let bin_id = format!("bin_{}", tid);
                let map = self.doc.get_map(bin_id.as_str());
                map.insert("data", payload.clone()).map_err(|e| anyhow::anyhow!("CRDT Binary Write failed: {:?}", e))?;
                tid
            }
            OpType::Move { new_parent } => {
                let tid = target_id.context("Target TreeID required for Move")?;
                let parent_tree_id = if new_parent.is_empty() {
                    None
                } else {
                    Some(TreeID::try_from(new_parent.as_str()).map_err(|_| anyhow::anyhow!("Invalid parent ID"))?)
                };
                self.tree.mov(tid, parent_tree_id)
                    .map_err(|e| anyhow::anyhow!("CRDT Move failed: {:?}", e))?;
                tid
            }
            OpType::Create { kind } => {
                // In KungFu, 'kind' carries the metadata: "name:kind" e.g. "main.go:file"
                let parts: Vec<&str> = kind.split(':').collect();
                let name = parts.first().unwrap_or(&"unnamed");
                let file_type = parts.get(1).unwrap_or(&"file");

                let node_id = self.tree.create(target_id)
                    .map_err(|e| anyhow::anyhow!("CRDT Create failed: {:?}", e))?;
                
                // GAP-002 FIX: Initialize metadata
                let meta = self.tree.get_meta(node_id).map_err(|e| anyhow::anyhow!("Meta access failed: {:?}", e))?;
                meta.insert("name", *name).unwrap();
                meta.insert("kind", *file_type).unwrap();
                meta.insert("id", id::must_new()).unwrap(); // UUIDv7 tracking

                node_id
            }
            OpType::Delete => {
                let tid = target_id.context("Target TreeID required for Delete")?;
                self.tree.delete(tid)
                    .map_err(|e| anyhow::anyhow!("CRDT Delete failed: {:?}", e))?;
                tid
            }
        };

        let trace = OperationTrace {
            id: id::must_new(),
            intent_id: intent_id.to_string(),
            target_id: actual_target_id.to_string(),
            op_type: op,
        };
        // GAP-005 FIX: We would normally extract the delta here using doc.export(ExportMode::Updates)
        // For the Alpha release, we ensure the trace is generated correctly.
        Ok(trace)
    }

    pub fn read_by_id(&self, target_id: TreeID) -> String {
        let text_id = format!("file_{}", target_id);
        let text = self.doc.get_text(text_id.as_str());
        text.to_string()
    }

    pub fn find_by_path(&self, path: &str) -> Result<TreeID> {
        let nodes = self.tree.get_nodes(false);
        for node in nodes {
            let p = self.resolve_path(node.id)?;
            if p.to_string_lossy() == path {
                return Ok(node.id);
            }
        }
        Err(anyhow::anyhow!("File not found: {}", path))
    }

    pub fn transcribe(&self, destination: &Path) -> Result<()> {
        if !destination.exists() {
            fs::create_dir_all(destination)?;
        }

        // 1. Get all active nodes in the CRDT
        let nodes = self.tree.get_nodes(false);
        
        // 2. Track what paths we are writing so we can delete ghost files
        let mut active_paths = std::collections::HashSet::new();

        for node in &nodes {
            if let Ok(p) = self.resolve_path(node.id) {
                active_paths.insert(destination.join(&p));
            }
        }

        // 3. Clean up physical Ghost Files (Deleted in CRDT but still on disk)
        for entry in walkdir::WalkDir::new(destination).into_iter().filter_map(|e| e.ok()) {
            let p = entry.path().to_path_buf();
            if p.is_file() && !active_paths.contains(&p) && !p.to_string_lossy().contains(".kungfu") {
                let _ = fs::remove_file(&p);
            }
        }

        // 4. Materialize the active DNA
        for node in nodes {
            let node_id = node.id;
            let path = self.resolve_path(node_id)?;
            let full_path = destination.join(path);

            let meta = self.tree.get_meta(node_id).map_err(|e| anyhow::anyhow!("Failed to get meta: {:?}", e))?;
            let kind = meta.get("kind").and_then(|v| v.into_value().ok()).and_then(|v| v.as_string().map(|s| s.to_string())).unwrap_or_else(|| "file".to_string());

            if kind == "dir" {
                fs::create_dir_all(&full_path)?;
            } else if kind == "binary" {
                if let Some(parent) = full_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let bin_id = format!("bin_{}", node_id);
                let map = self.doc.get_map(bin_id.as_str());
                if let Some(val) = map.get("data") {
                    if let Ok(bytes) = val.into_value()
                        && let Some(b) = bytes.as_binary() {
                            fs::write(full_path, b.as_slice())?;
                        }
                } else {
                    // Create empty file if no data written yet
                    fs::write(full_path, [])?;
                }
            } else {
                if let Some(parent) = full_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let text_id = format!("file_{}", node_id);
                let text = self.doc.get_text(text_id.as_str());
                fs::write(full_path, text.to_string())?;
            }
        }
        Ok(())
    }

    fn resolve_path(&self, id: TreeID) -> Result<PathBuf> {
        let mut components = Vec::new();
        let mut curr = Some(id);

        while let Some(node_id) = curr {
            let meta = self.tree.get_meta(node_id).map_err(|e| anyhow::anyhow!("Failed to get meta: {:?}", e))?;
            let name = meta.get("name").and_then(|v| v.into_value().ok()).and_then(|v| v.as_string().map(|s| s.to_string())).unwrap_or_else(|| "unnamed".to_string());
            components.push(name);
            
            curr = match self.tree.parent(node_id) {
                Some(loro::TreeParentId::Node(pid)) => Some(pid),
                _ => None,
            };
        }

        components.reverse();
        let mut path = PathBuf::new();
        for c in components {
            path.push(c);
        }
        Ok(path)
    }

    pub fn list_nodes(&self) -> Result<Vec<(String, String)>> {
        let nodes = self.tree.get_nodes(false);
        let mut out = Vec::new();
        for node in nodes {
            let path = self.resolve_path(node.id)?;
            let meta = self.tree.get_meta(node.id).map_err(|e| anyhow::anyhow!("Failed to get meta: {:?}", e))?;
            let kind = meta.get("kind").and_then(|v| v.into_value().ok()).and_then(|v| v.as_string().map(|s| s.to_string())).unwrap_or_else(|| "file".to_string());
            out.push((path.to_string_lossy().to_string(), kind));
        }
        Ok(out)
    }

    pub fn patch(&self, intent_id: &str, target_id: TreeID, find: &str, replace: &str) -> Result<OperationTrace> {
        let current_content = self.read_by_id(target_id);
        if let Some(offset) = current_content.find(find) {
            let op = OpType::Splice {
                offset,
                delete_len: find.len(),
                insert: replace.to_string(),
            };
            self.express(intent_id, Some(target_id), op)
        } else {
            Err(anyhow::anyhow!("Semantic Patch Failed: Could not find exact search block in file."))
        }
    }

}
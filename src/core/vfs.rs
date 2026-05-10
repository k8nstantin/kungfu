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
                let text_id = format!("file_{}", tid.to_string());
                let text = self.doc.get_text(text_id.as_str());
                text.splice(*offset, *delete_len, insert)
                    .map_err(|e| anyhow::anyhow!("CRDT Splice failed: {:?}", e))?;
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
                let name = parts.get(0).unwrap_or(&"unnamed");
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
        let text_id = format!("file_{}", target_id.to_string());
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

        let nodes = self.tree.get_nodes(false);
        for node in nodes {
            let node_id = node.id;
            let path = self.resolve_path(node_id)?;
            let full_path = destination.join(path);

            let meta = self.tree.get_meta(node_id).map_err(|e| anyhow::anyhow!("Failed to get meta: {:?}", e))?;
            let kind = meta.get("kind").and_then(|v| v.into_value().ok()).and_then(|v| v.as_string().map(|s| s.to_string())).unwrap_or_else(|| "file".to_string());

            if kind == "dir" {
                fs::create_dir_all(&full_path)?;
            } else {
                if let Some(parent) = full_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let text_id = format!("file_{}", node_id.to_string());
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
}

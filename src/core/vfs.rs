use anyhow::{Context, Result};
use loro::{LoroDoc, LoroTree, LoroText, TreeID};
use crate::core::history::{Intent, OpType, OperationTrace};
use crate::pkg::id;

/// VirtualFileSystem translates Agent operations into Loro CRDT math.
/// It operates entirely on the Trunk (the LoroDoc).
pub struct VirtualFileSystem<'a> {
    doc: &'a LoroDoc,
    tree: LoroTree,
}

impl<'a> VirtualFileSystem<'a> {
    pub fn new(doc: &'a LoroDoc) -> Self {
        // "vfs" is the root MovableTree in the Loro document.
        let tree = doc.get_tree("vfs");
        VirtualFileSystem { doc, tree }
    }

    /// Express applies a surgical operation to the Trunk and returns the trace.
    /// This is the core of KungFu's Agent-Native design. No full file overwrites.
    pub fn express(&self, intent_id: &str, target_id: TreeID, op: OpType) -> Result<OperationTrace> {
        match &op {
            OpType::Splice { offset, delete_len, insert } => {
                // Get the LoroText object associated with this file's TreeID
                let text_id = format!("file_{}", target_id.to_string());
                let text = self.doc.get_text(text_id.as_str());
                
                text.splice(*offset, *delete_len, insert)
                    .map_err(|e| anyhow::anyhow!("CRDT Splice failed: {:?}", e))?;
            }
            OpType::Move { new_parent } => {
                // Parse the parent ID (handling root case)
                let parent_tree_id = if new_parent.is_empty() {
                    None
                } else {
                    Some(TreeID::try_from(new_parent.as_str()).map_err(|_| anyhow::anyhow!("Invalid parent ID"))?)
                };
                
                self.tree.mov(target_id, parent_tree_id)
                    .map_err(|e| anyhow::anyhow!("CRDT Move failed: {:?}", e))?;
            }
            OpType::Create { kind: _ } => {
                // Create a new node in the MovableTree
                let _new_node = self.tree.create(None)
                    .map_err(|e| anyhow::anyhow!("CRDT Create failed: {:?}", e))?;
                // The actual ID would be returned here, but for this mock we just execute it.
            }
            OpType::Delete => {
                self.tree.delete(target_id)
                    .map_err(|e| anyhow::anyhow!("CRDT Delete failed: {:?}", e))?;
            }
        }

        // Generate the chronological UUIDv7 for the operation trace
        let trace_id = id::must_new();

        Ok(OperationTrace {
            id: trace_id,
            intent_id: intent_id.to_string(),
            target_id: target_id.to_string(),
            op_type: op,
        })
    }

    /// Read retrieves the current CRDT text state for a given file ID.
    pub fn read(&self, target_id: TreeID) -> String {
        let text_id = format!("file_{}", target_id.to_string());
        let text = self.doc.get_text(text_id.as_str());
        text.to_string()
    }
}

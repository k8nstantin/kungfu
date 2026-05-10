use loro::LoroDoc;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Dojo represents the local KungFu workspace.
/// It wraps the Loro CRDT document which is the absolute source of truth.
pub struct Dojo {
    pub doc: LoroDoc,
}

impl Dojo {
    /// Creates a new, empty Dojo with the base CRDT structures initialized.
    pub fn new() -> Self {
        let doc = LoroDoc::new();
        
        // Initialize the Virtual File System (MovableTree)
        // This tracks files and directories without conflicts.
        let _tree = doc.get_tree("vfs");
        
        // Initialize the Metadata Map
        // This tracks the "Intents" and project configuration.
        let _meta = doc.get_map("meta");

        Dojo { doc }
    }

    /// Loads a Dojo from a saved snapshot file.
    pub fn load(path: &Path) -> Result<Self> {
        let bytes = fs::read(path).context("Failed to read snapshot file")?;
        let doc = LoroDoc::new();
        doc.import(&bytes).context("Failed to import Loro snapshot")?;
        Ok(Dojo { doc })
    }

    /// Saves the current Dojo state as a compressed snapshot.
    pub fn save(&self, path: &Path) -> Result<()> {
        let bytes = self.doc.export(loro::ExportMode::Snapshot).unwrap();
        fs::write(path, bytes).context("Failed to write snapshot file")?;
        Ok(())
    }
}

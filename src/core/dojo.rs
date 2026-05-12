// Copyright (c) 2026 Constantin Alexander. All rights reserved.
// Use of this source code is governed by the Apache License 2.0 that can be
// found in the LICENSE file.

use loro::LoroDoc;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub struct Dojo {
    pub doc: LoroDoc,
}

impl Dojo {
    pub fn new() -> Self {
        let doc = LoroDoc::new();
        let _tree = doc.get_tree("vfs");
        let meta = doc.get_map("meta");
        // The Trunk Identity: The chronological birth of this codebase
        meta.insert("trunk_id", crate::pkg::id::must_new()).unwrap();

        Dojo { doc }
    }

    pub fn load(path: &Path) -> Result<Self> {
        let bytes = fs::read(path).context("Failed to read snapshot file")?;
        let doc = LoroDoc::new();
        doc.import(&bytes).context("Failed to import Loro snapshot")?;
        Ok(Dojo { doc })
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let bytes = self.doc.export(loro::ExportMode::Snapshot).unwrap();
        fs::write(path, bytes).context("Failed to write snapshot file")?;
        Ok(())
    }

    pub fn export_snapshot(&self) -> Vec<u8> {
        self.doc.export(loro::ExportMode::Snapshot).unwrap()
    }

    pub fn version_vector(&self) -> Vec<u8> {
        self.doc.oplog_vv().encode()
    }
}

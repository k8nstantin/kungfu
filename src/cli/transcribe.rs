// Copyright (c) 2026 Constantin Alexander. All rights reserved.
// Use of this source code is governed by the Apache License 2.0 that can be
// found in the LICENSE file.

use anyhow::{Context, Result};
use crate::core::dojo::Dojo;
use crate::core::vfs::VirtualFileSystem;
use std::path::PathBuf;

pub fn run(dest: Option<PathBuf>) -> Result<()> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let kf_dir = current_dir.join(".kungfu");
    
    if !kf_dir.exists() {
        return Err(anyhow::anyhow!("No KungFu Dojo found here. Run 'kf init' first."));
    }

    let dojo = Dojo::load(&kf_dir.join("snapshot.loro"))?;
    let vfs = VirtualFileSystem::new(&dojo.doc);

    let dest_path = dest.unwrap_or_else(|| current_dir.clone());
    
    println!("Transcribing DNA to physical files in {}...", dest_path.display());
    vfs.transcribe(&dest_path)?;
    println!("Success. DNA expressed as Organism.");
    
    Ok(())
}

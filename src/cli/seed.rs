// Copyright (c) 2026 Constantin Alexander <constantin@dedomena.io>. All rights reserved.
// Use of this source code is governed by the Apache License 2.0 that can be
// found in the LICENSE file.

use anyhow::{Context, Result};
use crate::core::dojo::Dojo;
use crate::core::identity::Identity;
use crate::core::seed::Seed;
use std::fs;

pub fn run(version: String) -> Result<()> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let kf_dir = current_dir.join(".kungfu");
    
    if !kf_dir.exists() {
        return Err(anyhow::anyhow!("No KungFu Dojo found here."));
    }

    // 1. Load Dojo and Identity
    let dojo = Dojo::load(&kf_dir.join("snapshot.loro"))?;
    let identity = Identity::load(&kf_dir.join("identity.key"))?;

    // 2. Create the immutable Seed
    let snapshot = dojo.export_snapshot();
    let vv = dojo.version_vector();
    let seed = Seed::new(&identity, snapshot, vv);

    // 3. Persist the Seed
    let seeds_dir = kf_dir.join("seeds");
    if !seeds_dir.exists() {
        fs::create_dir(&seeds_dir)?;
    }

    let seed_path = seeds_dir.join(format!("{}.seed", version));
    let seed_json = serde_json::to_string_pretty(&seed)?;
    fs::write(&seed_path, seed_json)?;

    println!("Successfully planted Seed {} at {}", version, seed_path.display());
    println!("Signature (Ed25519): {}", hex::encode(&seed.signature));
    
    Ok(())
}

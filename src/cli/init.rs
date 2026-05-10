use std::fs;
use anyhow::{Context, Result};
use crate::core::dojo::Dojo;

pub fn run() -> Result<()> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let kf_dir = current_dir.join(".kungfu");

    if kf_dir.exists() {
        println!("A Dojo already exists here.");
        return Ok(());
    }

    // Create the .kungfu directory
    fs::create_dir(&kf_dir).context("Failed to create .kungfu directory")?;

    // Initialize the Loro document
    let dojo = Dojo::new();
    dojo.save(&kf_dir.join("snapshot.loro"))?;

    // Create the append-only operations log
    fs::File::create(kf_dir.join("ops.log")).context("Failed to create ops.log")?;

    println!("Initialized empty KungFu Dojo in {}", kf_dir.display());
    Ok(())
}

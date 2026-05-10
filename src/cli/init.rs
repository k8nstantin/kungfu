use std::fs;
use anyhow::{Context, Result};
use crate::core::dojo::Dojo;
use crate::core::identity::Identity;

pub fn run() -> Result<()> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let kf_dir = current_dir.join(".kungfu");

    if kf_dir.exists() {
        println!("A Dojo already exists here.");
        return Ok(());
    }

    fs::create_dir(&kf_dir).context("Failed to create .kungfu directory")?;

    // 1. Initialize the Identity (The Ninja's signature)
    let identity = Identity::generate();
    identity.save(&kf_dir.join("identity.key"))?;
    println!("Generated new identity: {}", identity.public_key_hex());

    // 2. Initialize the Loro document
    let dojo = Dojo::new();
    
    // Seed with a dummy file for Phase 2 testing
    let tree = dojo.doc.get_tree("vfs");
    let file_id = tree.create(None).unwrap();
    let meta = tree.get_meta(file_id).unwrap();
    meta.insert("name", "hello.txt").unwrap();
    meta.insert("kind", "file").unwrap();
    let text = dojo.doc.get_text(format!("file_{}", file_id));
    text.insert(0, "Hello from the CRDT DNA!").unwrap();

    dojo.save(&kf_dir.join("snapshot.loro"))?;


    // 3. Create the append-only operations log
    fs::File::create(kf_dir.join("ops.log")).context("Failed to create ops.log")?;

    println!("Initialized empty KungFu Dojo in {}", kf_dir.display());
    Ok(())
}

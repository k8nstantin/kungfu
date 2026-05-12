// Copyright (c) 2026 Constantin Alexander. All rights reserved.
// Use of this source code is governed by the Apache License 2.0 that can be
// found in the LICENSE file.

use anyhow::{Context, Result};
use crate::core::dojo::Dojo;
use crate::core::identity::Identity;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

/// The Osmosis Handshake payload.
/// When a client connects, it sends its public key and its current Version Vector.
#[derive(Debug, Serialize, Deserialize)]
pub struct Handshake {
    pub pubkey: String,
    pub version_vector: Vec<u8>,
}

/// The Osmosis Update payload.
/// Sent bi-directionally to sync the CRDT state.
#[derive(Debug, Serialize, Deserialize)]
pub struct OsmosisUpdate {
    pub pubkey: String,
    pub signature: Vec<u8>,
    pub delta: Vec<u8>, // The Loro Bincode update
}

pub async fn begin_osmosis(url: &str) -> Result<()> {
    let kf_dir = std::env::current_dir()?.join(".kungfu");
    
    // 1. Load Local State
    let dojo = Dojo::load(&kf_dir.join("snapshot.loro"))?;
    let identity = Identity::load(&kf_dir.join("identity.key"))?;
    let vv = dojo.version_vector();

    // 2. Connect to the Central Dojo
    println!("Initiating Osmosis with {}...", url);
    let ws_url = Url::parse(url)?;
    let (ws_stream, _) = connect_async(ws_url.as_str()).await.context("Failed to connect to Central Dojo")?;
    let (mut write, mut read) = ws_stream.split();

    // 3. The Handshake Exchange
    let handshake = Handshake {
        pubkey: identity.public_key_hex(),
        version_vector: vv,
    };
    
    let handshake_bytes = bincode::serde::encode_to_vec(&handshake, bincode::config::standard()).map_err(|e| anyhow::anyhow!("Bincode error: {}", e))?;
    write.send(Message::Binary(handshake_bytes.into())).await?;
    println!("Handshake sent. Awaiting equilibrium...");

    // 4. The Continuous Flow Loop
    // In a full implementation, this would run two concurrent tasks (tokio::spawn):
    // one listening to local CRDT events to push, and one listening to the socket to pull.
    while let Some(msg) = read.next().await {
        let msg = msg?;
        if let Message::Binary(bytes) = msg
            && let Ok(update) = bincode::serde::decode_from_slice::<OsmosisUpdate, _>(&bytes, bincode::config::standard()).map(|(u, _)| u) {
                // TODO: Cryptographically verify the signature
                println!("Received Osmosis update from: {}", update.pubkey);
                
                // Apply the math
                if let Err(e) = dojo.doc.import(&update.delta) {
                    println!("Failed to weave update: {:?}", e);
                } else {
                    println!("DNA evolved. Equilibrium maintained.");
                    // Save hot state
                    let _ = dojo.save(&kf_dir.join("snapshot.loro"));
                }
            }
    }

    Ok(())
}

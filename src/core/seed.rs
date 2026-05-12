// Copyright (c) 2026 Constantin Alexander <constantin@dedomena.io>. All rights reserved.
// Use of this source code is governed by the Apache License 2.0 that can be
// found in the LICENSE file.

use serde::{Deserialize, Serialize};
use crate::core::identity::Identity;
use ed25519_dalek::Signer;

#[derive(Debug, Serialize, Deserialize)]
pub struct Seed {
    pub version_vector: Vec<u8>,
    pub snapshot: Vec<u8>,
    pub signature: Vec<u8>,
    pub author_pubkey: String,
    pub timestamp: u64,
}

impl Seed {
    pub fn new(identity: &Identity, snapshot: Vec<u8>, version_vector: Vec<u8>) -> Self {
        let mut data_to_sign = version_vector.clone();
        data_to_sign.extend_from_slice(&snapshot);

        let signature = identity.signing_key.sign(&data_to_sign);
        
        Seed {
            version_vector,
            snapshot,
            signature: signature.to_bytes().to_vec(),
            author_pubkey: identity.public_key_hex(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

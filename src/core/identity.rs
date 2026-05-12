// Copyright (c) 2026 Constantin Alexander <constantin@dedomena.io>. All rights reserved.
// Use of this source code is governed by the Apache License 2.0 that can be
// found in the LICENSE file.

use ed25519_dalek::{SigningKey, VerifyingKey};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use rand::RngExt;

pub struct Identity {
    pub signing_key: SigningKey,
}

impl Identity {
    pub fn generate() -> Self {
        let mut bytes = [0u8; 32];
        rand::rng().fill(&mut bytes);
        let signing_key = SigningKey::from_bytes(&bytes);
        Identity { signing_key }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let bytes = self.signing_key.to_bytes();
        fs::write(path, bytes).context("Failed to save identity key")?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let bytes = fs::read(path).context("Failed to read identity key")?;
        let bytes: [u8; 32] = bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid key length"))?;
        let signing_key = SigningKey::from_bytes(&bytes);
        Ok(Identity { signing_key })
    }

    pub fn public_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public_key().to_bytes())
    }
}

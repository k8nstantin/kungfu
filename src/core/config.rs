// Copyright (c) 2026 Constantin Alexander <constantin@dedomena.io>. All rights reserved.
// Use of this source code is governed by the Apache License 2.0 that can be
// found in the LICENSE file.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KungFuConfig {
    pub server: ServerConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub mcp_port: u16,
    pub osmosis_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub snapshot_policy: SnapshotPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum SnapshotPolicy {
    #[serde(rename = "threshold")]
    Threshold { max_unflushed_operations: usize },
    
    #[serde(rename = "temporal")]
    Temporal { interval_seconds: u64 },
    
    #[serde(rename = "event")]
    EventDriven { triggers: Vec<String> },
    
    #[serde(rename = "hybrid")]
    Hybrid { interval_seconds: u64, max_unflushed_operations: usize },
}

impl Default for KungFuConfig {
    fn default() -> Self {
        KungFuConfig {
            server: ServerConfig {
                mcp_port: 8766,
                osmosis_port: 8767,
            },
            storage: StorageConfig {
                snapshot_policy: SnapshotPolicy::Threshold { max_unflushed_operations: 1000 },
            },
        }
    }
}

impl KungFuConfig {
    #[allow(dead_code)]
    pub fn load_or_default(dir: &Path) -> Result<Self> {
        let path = dir.join("config.yaml");
        if path.exists() {
            let s = fs::read_to_string(&path).context("Failed to read config.yaml")?;
            let cfg = serde_yaml::from_str(&s).context("Failed to parse config.yaml")?;
            Ok(cfg)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, dir: &Path) -> Result<()> {
        let path = dir.join("config.yaml");
        let s = serde_yaml::to_string(self).context("Failed to serialize config")?;
        fs::write(path, s).context("Failed to write config.yaml")?;
        Ok(())
    }
}

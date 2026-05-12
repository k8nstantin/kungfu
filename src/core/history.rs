// Copyright (c) 2026 Constantin Alexander. All rights reserved.
// Use of this source code is governed by the Apache License 2.0 that can be
// found in the LICENSE file.

use serde::{Deserialize, Serialize};

/// Intent represents the "Why" behind a sequence of operations.
/// In KungFu, there is only one Trunk. When an agent or human mutates the Trunk,
/// their operations are logically tagged with an Intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Intent {
    pub id: String,          // UUIDv7
    pub author_pubkey: String, // Ed25519 Public Key Hex
    pub body: String,        // The reasoning / description
    pub timestamp: u64,      // Unix epoch
    pub status: IntentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub enum IntentStatus {
    /// The mutation is active and streaming operations. (Unselected)
    Mutating,
    /// The mutation is undergoing Natural Selection (Tests/CI).
    Selecting,
    /// The mutation survived and is permanently woven into the Trunk DNA.
    Evolved,
    /// The mutation failed selection and was discarded from the Trunk view.
    Rejected,
}

/// OperationTrace is the atomic unit of the KungFu Trunk.
/// Instead of branches, we have a continuous stream of traces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationTrace {
    pub id: String,          // UUIDv7
    pub intent_id: String,     // Links to the Intent
    pub target_id: String,   // The Loro ID of the VFS node (File/Directory)
    pub op_type: OpType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpType {
    Splice { offset: usize, delete_len: usize, insert: String },
    /// Replaces the entire binary payload (Last-Write-Wins)
    WriteBinary { payload: Vec<u8> },
    Move { new_parent: String },
    Create { kind: String }, // "file", "binary", or "dir"
    Delete,
}

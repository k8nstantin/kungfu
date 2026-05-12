// Copyright (c) 2026 Constantin Alexander <constantin@dedomena.io>. All rights reserved.
// Use of this source code is governed by the Apache License 2.0 that can be
// found in the LICENSE file.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SpliceArgs {
    pub path: String,
    pub offset: usize,
    pub delete_len: usize,
    pub insert_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct IntentArgs {
    pub body: String,
}

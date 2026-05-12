// Copyright (c) 2026 Constantin Alexander <constantin@dedomena.io>. All rights reserved.
// Use of this source code is governed by the Apache License 2.0 that can be
// found in the LICENSE file.

use uuid::Uuid;

pub fn must_new() -> String {
    Uuid::now_v7().to_string()
}

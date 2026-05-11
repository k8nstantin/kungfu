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

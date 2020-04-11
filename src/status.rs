use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub state: String,
    pub code: u16,
    pub code_message: String,
    pub output: Option<String>,
    pub progression_in_percent: u16,
}

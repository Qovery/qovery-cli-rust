use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub state: String,
    pub code: u16,
    pub code_message: String,
    pub output: Option<String>,
    pub progression_in_percent: u16,
}

impl Status {
    pub fn code_message_colored(&self) -> ColoredString {
        if self.code_message.ends_with("_ERROR") {
            return self.code_message.as_str().bold().red();
        }

        if self.progression_in_percent != 100 {
            return self.code_message.as_str().bold().yellow();
        }

        self.code_message.as_str().bold().green()
    }
}

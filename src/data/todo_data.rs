use chrono::offset::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoData {
    pub todo: String,
    pub desc: String,

    pub time_created: String,
    pub finish: bool,

    pub toggle_desc: bool,
}
impl TodoData {
    pub fn new(input: &str, desc: &str) -> Self {
        Self {
            todo: input.to_string(),
            desc: desc.to_string(),
            time_created: format!(
                "{} {}",
                Local::now().date_naive(),
                Local::now().time().format("%H:%M:%S")
            ),

            finish: false,
            toggle_desc: false,
        }
    }

    pub fn clear(&mut self) {
        self.todo = "".to_string();
        self.desc = "".to_string();
    }
}

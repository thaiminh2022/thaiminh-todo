// There's one field here
// But we use it anyways for better maintain
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub display_name: String,
    pub prefer_dark_mode: bool,
}

impl UserData {
    pub fn new(display_name: String, prefer_dark_mode: bool) -> Self {
        Self {
            display_name: display_name,
            prefer_dark_mode: prefer_dark_mode,
        }
    }

    pub fn to_json(&self) -> String {
        let data = serde_json::to_string_pretty(self).unwrap();

        return data;
    }
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            display_name: "User-69420D".to_string(),
            prefer_dark_mode: true,
        }
    }
}

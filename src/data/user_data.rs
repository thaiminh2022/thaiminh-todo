// There's one field here
// But we use it anyways for better maintain
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    uid: String,
}

impl UserData {
    pub fn new(uid: &str) -> Self {
        Self {
            uid: uid.to_string(),
        }
    }
}

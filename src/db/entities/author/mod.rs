use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Author {
    pub username: String
}

impl Default for Author {
    fn default() -> Self {
        Self { username: "DEFAULT".to_string() }
    }
}
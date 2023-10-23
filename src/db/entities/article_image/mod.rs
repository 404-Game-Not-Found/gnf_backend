use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ArticleImage {
    source: String,
    tags: Vec<String>
}

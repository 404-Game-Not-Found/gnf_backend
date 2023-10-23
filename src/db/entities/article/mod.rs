use serde::{Deserialize, Serialize};

use super::{author::Author, article_image::ArticleImage};


#[derive(Deserialize, Serialize)]
pub struct Article {
    author: Author,
    content: String,
    images: Vec<ArticleImage>,
    tags: Vec<String>
}

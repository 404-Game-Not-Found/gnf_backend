pub mod db {
    pub const DB_NAME: &str = "gnf";
    pub const DB_PATH: &str = "gnf.db";
    pub const ARTICLE_TABLE_NAME: &str = "articles";
    
    pub mod author {
        pub const AUTHOR_TABLE_NAME: &str = "authors";
        pub const AUTHOR_USERNAME: &str = "username";
    }
}
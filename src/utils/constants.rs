pub mod db {
    pub const DB_NAME: &str = "gnf";
    pub const DB_PATH: &str = "gnf.db";
    pub const ARTICLE_TABLE_NAME: &str = "articles";
    pub const DB_SEPERATOR: &str = ",";
    
    pub mod author {
        pub const AUTHOR_TABLE_NAME: &str = "authors";
        pub const AUTHOR_USERNAME: &str = "username";
    }

    pub mod image {
        pub const TABLE_NAME: &str = "images";
        pub const SOURCE_FIELD: &str = "source";
        pub const ID_FIELD: &str = "id";
        pub const TAGS_FIELD: &str = "tags";
    }
}
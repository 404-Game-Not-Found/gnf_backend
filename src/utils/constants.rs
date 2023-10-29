pub mod db {
    pub const DB_NAME: &str = "gnf";
    pub const DB_PATH: &str = "gnf.db";
    pub const DB_SEPERATOR: &str = ",";
    
    pub mod author {
        pub const TABLE_NAME: &str = "user";

        pub const ID: &str = "id";
        pub const FIRST_NAME: &str = "";
        pub const LAST_NAME: &str = "";
        pub const DISPLAY_NAME: &str = "";
        pub const EMAIL: &str = "";
        pub const PASSWORD: &str = "";

        pub const FIELDS: [&str; 6] = [
            ID,
            FIRST_NAME,
            LAST_NAME,
            DISPLAY_NAME,
            EMAIL,
            PASSWORD
        ];
    }

    pub mod image {
        pub const TABLE_NAME: &str = "image";

        pub const ID: &str = "id";
        pub const IMAGE_PATH: &str = "image_path";
        pub const TAGS: &str = "tags";

        pub const FIELDS: [&str; 3] = [
            ID,
            IMAGE_PATH,
            TAGS
        ];
    }

    pub mod image_tags {
        pub const TABLE_NAME: &str = "image_tags";

        pub const ID: &str = "id";
        pub const IMAGE: &str = "image";
        pub const TAG: &str = "tag";

        pub const FIELDS: [&str; 3] = [
            ID,
            IMAGE,
            TAG
        ];
    }


    pub mod article {
        pub const TABLE_NAME: &str = "article";

        pub const ID: &str = "id";
        pub const TITLE: &str = "title";
        pub const MARKDOWN_PATH: &str = "markdown_path";
        pub const IMAGES: &str = "images";
        pub const AUTHOR: &str = "author";
        pub const CREATED_AT: &str = "created_at";
        pub const TAGS: &str = "tags";

        pub const FIELDS: [&str; 6] = [
            ID,
            TITLE,
            MARKDOWN_PATH,
            IMAGES,
            CREATED_AT,
            TAGS
        ];
    }

    pub mod article_tags {
        pub const TABLE_NAME: &str = "article_tags";

        pub const ID: &str = "id";
        pub const ARTICLE: &str = "article";
        pub const TAG: &str = "tag";

        pub const FIELDS: [&str; 3] = [
            ID,
            ARTICLE,
            TAG
        ];
    }

    pub mod tags {
        pub const TABLE_NAME: &str = "tags";

        pub const ID: &str = "id";
        pub const TAG: &str = "tag";

        pub const FIELDS: [&str; 2] = [
            ID,
            TAG
        ];
    }
}
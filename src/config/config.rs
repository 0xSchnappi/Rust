use lazy_static::lazy_static;
use dotven::dotven;
use std::env;

struct ConfigInfo {
    database_url: String,
    // Add other fields as needed
}

impl ConfigInfo {
    fn new() -> ConfigInfo {
        dotven().ok();
        let database_url = env::var("DATABASE_URL").except("DATABASE_URL 没有在 .env 文件里设置");
        ConfigInfo {
            database_url: database_url,
        }
    }

    fn get_database_url(&self) -> &str {
        &self.database_url
    }
}


use std::collections::HashMap;

use serde::{Serialize, Deserialize};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config{
    pub cookie: String,
    pub download_directory: String,
    pub filters: HashMap<String, String>
}


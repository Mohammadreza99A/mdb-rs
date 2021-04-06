use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Language {
    pub english_name: Option<String>,
    pub iso_639_1: Option<String>,
    pub name: Option<String>,
}

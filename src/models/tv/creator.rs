use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Creator {
    pub id: Option<u64>,
    pub credit_id: Option<String>,
    pub name: Option<String>,
    pub gender: Option<u32>,
    pub profile_path: Option<String>,
}

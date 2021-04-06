use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Company {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub logo_path: Option<String>,
    pub origin_country: Option<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Network {
    pub name: Option<String>,
    pub id: Option<u64>,
    pub logo_path: Option<String>,
    pub origin_country: Option<String>,
}

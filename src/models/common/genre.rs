use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Genre {
  pub id: Option<u64>,
  pub name: Option<String>,
}

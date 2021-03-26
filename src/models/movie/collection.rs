use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Collection {
  pub id: Option<u64>,
  pub name: Option<String>,
  pub poster_path: Option<String>,
  pub backdrop_path: Option<String>,
}

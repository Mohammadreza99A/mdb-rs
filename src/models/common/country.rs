use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Country {
  pub iso_3166_1: Option<String>,
  pub name: Option<String>,
}

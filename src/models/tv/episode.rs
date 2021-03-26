use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Episode {
  pub air_date: Option<String>,
  pub episode_number: Option<u32>,
  pub id: Option<u64>,
  pub name: Option<String>,
  pub overview: Option<String>,
  pub production_code: Option<String>,
  pub season_number: Option<u32>,
  pub still_path: Option<String>,
  pub vote_average: Option<f32>,
  pub vote_count: Option<u64>,
}

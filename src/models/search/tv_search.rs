use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TvSearch {
  pub page: Option<u16>,
  pub results: Option<Vec<Tv>>,
  pub total_results: Option<u16>,
  pub total_pages: Option<u16>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tv {
  pub poster_path: Option<String>,
  pub popularity: Option<f64>,
  pub id: Option<u64>,
  pub backdrop_path: Option<String>,
  pub vote_average: Option<f64>,
  pub overview: Option<String>,
  pub first_air_date: Option<String>,
  pub origin_country: Option<Vec<String>>,
  pub genres_id: Option<Vec<u16>>,
  pub vote_count: Option<u64>,
  pub adult: Option<bool>,
  pub name: Option<String>,
  pub original_name: Option<String>,
}

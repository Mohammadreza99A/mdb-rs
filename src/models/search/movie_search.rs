use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MovieSearch {
  pub page: Option<u16>,
  pub results: Option<Vec<Movie>>,
  pub total_results: Option<u16>,
  pub total_pages: Option<u16>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Movie {
  pub poster_path: Option<String>,
  pub adult: Option<bool>,
  pub overview: Option<String>,
  pub release_date: Option<String>,
  pub genres_id: Option<Vec<u16>>,
  pub id: Option<u64>,
  pub original_title: Option<String>,
  pub original_language: Option<String>,
  pub title: Option<String>,
  pub backdrop_path: Option<String>,
  pub popularity: Option<f64>,
  pub vote_count: Option<u64>,
  pub video: Option<bool>,
  pub vote_average: Option<f64>,
}

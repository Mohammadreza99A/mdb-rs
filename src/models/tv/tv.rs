use crate::models::common::company::Company;
use crate::models::common::country::Country;
use crate::models::common::genre::Genre;
use crate::models::common::language::Language;
use crate::models::tv::creator::Creator;
use crate::models::tv::episode::Episode;
use crate::models::tv::network::Network;
use crate::models::tv::season::Season;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Tv {
  pub backdrop_path: Option<String>,
  pub created_by: Option<Vec<Creator>>,
  pub episode_run_time: Option<Vec<u64>>,
  pub first_air_date: Option<String>,
  pub genres: Option<Vec<Genre>>,
  pub homepage: Option<String>,
  pub id: Option<u64>,
  pub in_production: Option<bool>,
  pub languages: Option<Vec<String>>,
  pub last_air_date: Option<String>,
  pub last_episode_to_air: Option<Episode>,
  pub name: Option<String>,
  // pub next_episode_to_air: Option<?>,
  pub networks: Option<Vec<Network>>,
  pub number_of_episodes: Option<u64>,
  pub number_of_seasons: Option<u32>,
  pub origin_country: Option<Vec<String>>,
  pub original_language: Option<String>,
  pub original_name: Option<String>,
  pub overview: Option<String>,
  pub popularity: Option<f32>,
  pub poster_path: Option<String>,
  pub production_companies: Option<Vec<Company>>,
  pub production_countries: Option<Vec<Country>>,
  pub seasons: Option<Vec<Season>>,
  pub spoken_languages: Option<Vec<Language>>,
  pub status: Option<String>,
  pub tagline: Option<String>,
  pub r#type: Option<String>,
  pub vote_average: Option<f32>,
  pub vote_count: Option<u64>,
}

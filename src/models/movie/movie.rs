use serde::{Deserialize, Serialize};

use crate::models::common::company::Company;
use crate::models::common::country::Country;
use crate::models::common::genre::Genre;
use crate::models::common::language::Language;
use crate::models::movie::collection::Collection;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Movie {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub belongs_to_collection: Option<Collection>,
    pub budget: Option<u64>,
    pub genres: Option<Vec<Genre>>,
    pub homepage: Option<String>,
    pub id: Option<u64>,
    pub imdb_id: Option<String>,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f64>,
    pub poster_path: Option<String>,
    pub production_companies: Option<Vec<Company>>,
    pub production_countries: Option<Vec<Country>>,
    pub release_date: Option<String>,
    pub revenue: Option<u64>,
    pub runtime: Option<u64>,
    pub spoken_languages: Option<Vec<Language>>,
    pub status: Option<String>,
    pub tagline: Option<String>,
    pub title: Option<String>,
    pub video: Option<bool>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<u64>,
}

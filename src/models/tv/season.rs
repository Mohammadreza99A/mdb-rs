use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Season {
    pub air_date: Option<String>,
    pub episode_count: Option<u32>,
    pub id: Option<u64>,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub season_number: Option<u32>,
}

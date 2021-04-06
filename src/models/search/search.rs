use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TmdbSearch {
    pub page: Option<u16>,
    pub total_results: Option<u16>,
    pub total_pages: Option<u16>,
}

use crate::models::movie::movie::Movie;
use crate::models::search::movie_search::MovieSearch;
use crate::models::search::tv_search::TvSearch;
use crate::models::tv::tv::Tv;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use dotenv;
use serde::Serialize;
use serde_json::{to_value, Value as JsonValue};
use std::collections::HashMap;
use std::env;

pub struct TmdbClient {
  pub base_url: &'static str,
  pub api_key: String,
}

impl TmdbClient {
  pub fn new() -> TmdbClient {
    const BASE_URL: &'static str = "https://api.themoviedb.org/3";
    if dotenv::dotenv().is_err() {
      eprintln!(".env file not found: {:?}", dotenv::dotenv().err());
      std::process::exit(1);
    }
    dotenv::dotenv().expect(".env file not found");
    let api_key: String = env::var("TMDB_API_KEY").unwrap();
    TmdbClient {
      base_url: BASE_URL,
      api_key: api_key,
    }
  }

  pub fn get_request_url(
    &self,
    end_point: String,
    query: Option<String>,
    page: Option<String>,
  ) -> String {
    if query.is_some() && page.is_some() {
      return format!(
        "{base}/{end_point}?api_key={tmdb_api_key}&language=en-US&query={query}&page={page}&include_adult=false",
        base = self.base_url,
        end_point = end_point,
        tmdb_api_key = self.api_key,
        query= query.unwrap(),
        page= page.unwrap()
      );
    } else {
      return format!(
        "{base}/{end_point}?api_key={tmdb_api_key}&language=en-US",
        base = self.base_url,
        end_point = end_point,
        tmdb_api_key = self.api_key,
      );
    }
  }

  pub async fn send_get_request(
    &self,
    end_point: String,
    query: Option<String>,
    page: Option<String>,
  ) -> reqwest::Result<String> {
    let request_url = self.get_request_url(end_point, query, page);

    let response = reqwest::get(&request_url).await?;

    if !response.status().is_success() {
      eprintln!("Could not connect to TMDB. Check your internet connection");
      std::process::exit(1);
    }

    return Ok(response.text().await?);
  }

  #[tokio::main]
  pub async fn get_movie_by_id(&self, id: u64) -> Result<Movie, anyhow::Error> {
    let end_point = format!("movie/{id}", id = id,);

    let body = &self.send_get_request(end_point, None, None).await?;

    let movie: Movie = serde_json::from_str(body)?;

    return Ok(movie);
  }

  #[tokio::main]
  pub async fn get_movie(&self, name: String, page: String) -> Result<MovieSearch, anyhow::Error> {
    let end_point = format!("search/movie");

    let body = &self
      .send_get_request(end_point, Some(name), Some(page))
      .await?;

    let movie: MovieSearch = serde_json::from_str(body)?;

    return Ok(movie);
  }

  #[tokio::main]
  pub async fn get_tv_by_id(&self, id: u64) -> Result<Tv, anyhow::Error> {
    let end_point = format!("tv/{id}", id = id,);

    let body = &self.send_get_request(end_point, None, None).await?;

    let tv = serde_json::from_str(body)?;

    return Ok(tv);
  }

  #[tokio::main]
  pub async fn get_tv(&self, name: String, page: String) -> Result<TvSearch, anyhow::Error> {
    let end_point = format!("search/tv");

    let body = &self
      .send_get_request(end_point, Some(name), Some(page))
      .await?;

    let tv: TvSearch = serde_json::from_str(body)?;

    return Ok(tv);
  }

  pub fn parse_to_json<T: Serialize>(&self, res: T) -> Result<JsonValue, serde_json::Error> {
    let json_obj: JsonValue = to_value(res)?;

    return Ok(json_obj);
  }

  pub fn print_as_table(&self, json_obj: JsonValue) {
    let mut table = Table::new();
    table
      .load_preset(UTF8_FULL)
      .apply_modifier(UTF8_ROUND_CORNERS)
      .set_content_arrangement(ContentArrangement::Dynamic)
      .set_table_width((term_size::dimensions().unwrap().0) as u16);

    let mut obj_tables_map: HashMap<String, Table> = HashMap::new();
    let mut vec_tables_map: HashMap<String, Vec<Table>> = HashMap::new();

    for (key, value) in json_obj.as_object().unwrap() {
      let key = key.replace("_", " ");

      if value.is_array() {
        let mut inner_tables: Vec<Table> = Vec::new();

        for elem in value.as_array() {
          for obj in elem {
            let mut obj_table = Table::new();
            obj_table
              .load_preset(UTF8_FULL)
              .apply_modifier(UTF8_ROUND_CORNERS)
              .set_content_arrangement(ContentArrangement::Dynamic);

            if obj.is_object() {
              for (k, v) in obj.as_object().unwrap() {
                if v.is_string() {
                  obj_table.add_row(vec![
                    Cell::new(k.replace("_", " ")),
                    Cell::new(v.as_str().unwrap().replace("\"", "")),
                  ]);
                } else {
                  obj_table.add_row(vec![Cell::new(k.replace("_", " ")), Cell::new(v)]);
                }
              }
            } else if obj.is_string() {
              obj_table.add_row(vec![Cell::new(obj.as_str().unwrap().replace("\"", ""))]);
            } else {
              obj_table.add_row(vec![Cell::new(obj)]);
            }
            inner_tables.push(obj_table);
          }
        }
        vec_tables_map.insert(key, inner_tables);
      } else if value.is_object() {
        let mut obj_table = Table::new();
        obj_table
          .load_preset(UTF8_FULL)
          .apply_modifier(UTF8_ROUND_CORNERS)
          .set_content_arrangement(ContentArrangement::Dynamic);

        for (k, v) in value.as_object().unwrap() {
          if v.is_string() {
            obj_table.add_row(vec![
              Cell::new(k.replace("_", " ")),
              Cell::new(v.as_str().unwrap().replace("\"", "")),
            ]);
          } else {
            obj_table.add_row(vec![Cell::new(k.replace("_", " ")), Cell::new(v)]);
          }
        }
        obj_tables_map.insert(key, obj_table);
      } else if value.is_string() {
        table.add_row(vec![
          Cell::new(key).add_attribute(Attribute::Bold),
          Cell::new(value.as_str().unwrap().replace("\"", "")),
        ]);
      } else {
        table.add_row(vec![
          Cell::new(key).add_attribute(Attribute::Bold),
          Cell::new(value),
        ]);
      }
    }

    let inner_table_width =
      (table.get_table_width().unwrap() - table.get_column(0).unwrap().get_max_width()) - 8;
    for (key, mut tab) in obj_tables_map {
      tab.set_table_width(inner_table_width);
      table.add_row(vec![
        Cell::new(key).add_attribute(Attribute::Bold),
        Cell::new(tab),
      ]);
    }

    for (key, tabs) in vec_tables_map {
      let mut inner_table = Table::new();
      inner_table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

      for mut tab in tabs {
        tab.set_table_width(inner_table_width - 3);
        inner_table.add_row(vec![Cell::new(tab)]);
      }
      table.add_row(vec![
        Cell::new(key).add_attribute(Attribute::Bold),
        Cell::new(inner_table),
      ]);
    }

    println!("{}", table);
  }
}

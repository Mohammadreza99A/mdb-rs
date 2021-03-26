extern crate anyhow;
extern crate clap;
extern crate reqwest;
extern crate term_size;
pub mod models;
pub mod tmdb_client;
use clap::{App, AppSettings, Arg};
use tmdb_client::TmdbClient;

fn main() {
  parse_args(cli_args());
}

pub fn cli_args() -> App<'static> {
  let app = App::new("mdb")
    .subcommand(
      App::new("id")
        .short_flag('i')
        .long_flag("id")
        .about("get tv or movie with TMDB id")
        .arg(
          Arg::new("movie")
            .about("the MDB movie id")
            .takes_value(true)
            .short('m')
            .long("movie")
            .conflicts_with_all(&["tv"]),
        )
        .arg(
          Arg::new("tv")
            .about("the MDB tv id")
            .takes_value(true)
            .short('t')
            .long("tv")
            .conflicts_with_all(&["movie"]),
        )
        .setting(AppSettings::ArgRequiredElseHelp),
    )
    .subcommand(
      App::new("search")
        .short_flag('s')
        .long_flag("search")
        .about("search TMDB for tv or movie")
        .arg(
          Arg::new("movie")
            .about("the MDB movie id")
            .takes_value(true)
            .short('m')
            .long("movie")
            .conflicts_with_all(&["tv"]),
        )
        .arg(
          Arg::new("tv")
            .about("the MDB tv id")
            .takes_value(true)
            .short('t')
            .long("tv")
            .conflicts_with_all(&["movie"]),
        )
        .setting(AppSettings::ArgRequiredElseHelp),
    )
    .setting(AppSettings::SubcommandRequiredElseHelp);

  return app;
}

pub fn tv_by_id(tv_id: &&str, mut app: App) {
  let tmdb = TmdbClient::new();

  let id = tv_id.parse::<u64>();
  if id.is_ok() {
    let tv = tmdb.get_tv_by_id(id.unwrap());
    match tv {
      Ok(tv_res) => {
        let json = tmdb.parse_to_json(tv_res);
        match json {
          Ok(json_val) => tmdb.print_as_table(json_val),
          Err(err) => {
            eprintln!("An error happened: {}", err);
            std::process::exit(1);
          }
        }
      }
      Err(err) => {
        eprintln!("An error happened: {}", err);
        std::process::exit(1);
      }
    }
  } else {
    println!("Bad TMDB id. TMDB id must be an integer\n");
    app.print_help().unwrap_or_default();
  }
}

pub fn movie_by_id(movie_id: &&str, mut app: App) {
  let tmdb = TmdbClient::new();

  let id = movie_id.parse::<u64>();
  if id.is_ok() {
    let movie = tmdb.get_movie_by_id(id.unwrap());
    match movie {
      Ok(movie_res) => {
        let json = tmdb.parse_to_json(movie_res);
        match json {
          Ok(json_val) => tmdb.print_as_table(json_val),
          Err(err) => {
            eprintln!("An error happened: {}", err);
            std::process::exit(1);
          }
        }
      }
      Err(err) => {
        eprintln!("An error happened: {}", err);
        std::process::exit(1);
      }
    }
  } else {
    println!("Bad TMDB id. TMDB id must be an integer\n");
    app.print_help().unwrap_or_default();
  }
}

pub fn tv_by_name(tv_name: &&str) {
  let tmdb = TmdbClient::new();

  let tv = tmdb.get_tv(tv_name.to_string(), "1".to_string());
  match tv {
    Ok(tv_res) => {
      let json = tmdb.parse_to_json(tv_res);
      match json {
        Ok(json_val) => tmdb.print_as_table(json_val),
        Err(err) => {
          eprintln!("An error happened: {}", err);
          std::process::exit(1);
        }
      }
    }
    Err(err) => {
      eprintln!("An error happened: {}", err);
      std::process::exit(1);
    }
  }
}

pub fn movie_by_name(movie_name: &&str) {
  let tmdb = TmdbClient::new();

  let movie = tmdb.get_movie(movie_name.to_string(), "1".to_string());
  match movie {
    Ok(movie_res) => {
      let json = tmdb.parse_to_json(movie_res);
      match json {
        Ok(json_val) => tmdb.print_as_table(json_val),
        Err(err) => {
          eprintln!("An error happened: {}", err);
          std::process::exit(1);
        }
      }
    }
    Err(err) => {
      eprintln!("An error happened: {}", err);
      std::process::exit(1);
    }
  }
}

pub fn parse_args(app: App) {
  let matches = app.clone().get_matches();

  match matches.subcommand() {
    Some(("id", id_matches)) => {
      if id_matches.is_present("movie") {
        if let Some(ref movie_id) = id_matches.value_of("movie") {
          movie_by_id(movie_id, app);
        }
      } else if id_matches.is_present("tv") {
        if let Some(ref tv_id) = id_matches.value_of("tv") {
          tv_by_id(tv_id, app);
        }
      }
    }
    Some(("search", search_matches)) => {
      if search_matches.is_present("movie") {
        if let Some(ref movie_name) = search_matches.value_of("movie") {
          movie_by_name(movie_name);
        }
      } else if search_matches.is_present("tv") {
        if let Some(ref tv_name) = search_matches.value_of("tv") {
          tv_by_name(tv_name);
        }
      }
    }
    _ => unreachable!(),
  }
}

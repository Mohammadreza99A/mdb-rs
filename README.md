# mdb-rs

> Command line rust app to get all sorts of information about movies and tv shows

## Quick Start

### Build

```bash
# Build the package
cargo build
```

### Run
```bash
# Run the app and get a help menu
cargo run -- --help

# Search for a tv show by name
cargo run -- -s -t "breaking bad"

# Search for a tv show by TMDb id
cargo run -- -i -t 1396

# Search for a movie name
cargo run -- -s -m "inception"

# Search for a movie by TMDB id
cargo run -- -i -m 27205
```

## App Info

This CLI rust app gets all sort of information about movies and tv shows. It lets you to search for a movie or tv show by name or TMDB id. It shows a maximum number of information about searched movie or tv show in a nice well dynamic printed table.  
**This app is in beta mode. More features to come in the near future. All feedback is welcome**

### Author

Mohammadreza Amini <mohammadreza99a@yahoo.com>  
<a href="https://github.com/Mohammadreza99A/">
  <img src="https://avatars.githubusercontent.com/u/44376323?s=60&v=4" />
</a>

### Version

*1.0.0-b.1*

### License

This project is licensed under the *MIT* License

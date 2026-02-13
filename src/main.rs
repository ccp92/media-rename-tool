use clap::{Parser, ArgGroup};
use regex::Regex;
use std::fs;


#[derive(Parser, Debug)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .multiple(false)
        .args(["tv", "movie"]),
))]
struct Cli {
    /// TV mode: strips text after SxxExx
    #[arg(long, group = "mode")]
    tv: bool,

    /// Movie mode: strips text after year (19xx-20xx), wraps year in parens
    #[arg(long, group = "mode")]
    movie: bool,
}

fn main() {
    let args = Cli::parse();

    let re_movie = Regex::new(r"^(?P<title>.*?)\s*(?P<year>(?:19|20)\d{2}).*(?P<ext>\.[^.]+)$").unwrap();
    // TV regex: Matches string containing SxxExx, keeps everything up to that point.
    let re_tv = Regex::new(r"^(?P<keep>.*?S\d{2}E\d{2}).*(?P<ext>\.[^.]+)$").unwrap();

    let current_dir = ".";
    let paths = match fs::read_dir(current_dir) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

    for path in paths {
        let path = match path {
            Ok(p) => p.path(),
            Err(_) => continue,
        };

        if !path.is_file() {
            continue;
        }

        let filename = match path.file_name() {
            Some(f) => f.to_string_lossy(),
            None => continue,
        };

        let new_filename = if args.movie {
            if let Some(caps) = re_movie.captures(&filename) {
                let title = &caps["title"];
                let year = &caps["year"];
                let ext = &caps["ext"];
                // "strip out any text after a year is found... and wrap the year in partnethesis"
                format!("{} ({}){}", title.replace(".", " ").trim(), year, ext)
            } else {
                continue;
            }
        } else if args.tv {
            if let Some(caps) = re_tv.captures(&filename) {
                let keep = &caps["keep"];
                let ext = &caps["ext"];
                 // "strip out all text after a string in the format S01E01 is found"
                format!("{}{}", keep.replace(".", " ").trim(), ext)
            } else {
                continue;
            }
        } else {
            continue; 
        };

        if new_filename != filename.as_ref() {
            let new_path = path.with_file_name(&new_filename);
            println!("Renaming: '{}' -> '{}'", filename, new_filename);
            if let Err(e) = fs::rename(&path, &new_path) {
                eprintln!("Failed to rename '{}': {}", filename, e);
            }
        }
    }
}

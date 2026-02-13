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

    /// Find and replace: takes two strings, finds the first and replaces with the second
    #[arg(long, num_args = 2, value_names = ["FIND", "REPLACE"])]
    replace: Option<Vec<String>>,

    /// Dry run: prints what would happen without renaming files
    #[arg(long)]
    dry_run: bool,
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

    let mut pending_renames = Vec::new();

    for path in paths {
        let path = match path {
            Ok(p) => p.path(),
            Err(_) => continue,
        };

        if !path.is_file() {
            continue;
        }

        let filename = match path.file_name() {
            Some(f) => f.to_string_lossy().into_owned(),
            None => continue,
        };

        let new_filename = if args.movie {
            if let Some(caps) = re_movie.captures(&filename) {
                let title = &caps["title"];
                let year = &caps["year"];
                let ext = &caps["ext"];
                
                let mut title = title.to_string();
                if let Some(replace_args) = &args.replace {
                    if replace_args.len() == 2 {
                        title = title.replace(&replace_args[0], &replace_args[1]);
                    }
                }
                let title = title.replace(".", " ");

                // "strip out any text after a year is found... and wrap the year in partnethesis"
                format!("{} ({}){}", title.trim(), year, ext)
            } else {
                continue;
            }
        } else if args.tv {
            if let Some(caps) = re_tv.captures(&filename) {
                let keep = &caps["keep"];
                let ext = &caps["ext"];
                
                let mut keep = keep.to_string();
                if let Some(replace_args) = &args.replace {
                     if replace_args.len() == 2 {
                        keep = keep.replace(&replace_args[0], &replace_args[1]);
                    }
                }
                let keep = keep.replace(".", " ");

                 // "strip out all text after a string in the format S01E01 is found"
                format!("{}{}", keep.trim(), ext)
            } else {
                continue;
            }
        } else {
            continue; 
        };

        if new_filename != filename {
            pending_renames.push((path, filename, new_filename));
        }
    }

    if pending_renames.is_empty() {
        println!("No files to rename.");
        return;
    }

    if args.dry_run {
        println!("Dry run: The following files WOULD be renamed:");
        for (_, old_name, new_name) in &pending_renames {
            println!("'{}' -> '{}'", old_name, new_name);
        }
        return;
    }

    for (path, old_name, new_name) in pending_renames {
        let new_path = path.with_file_name(&new_name);
        println!("Renaming: '{}' -> '{}'", old_name, new_name);
        if let Err(e) = fs::rename(&path, &new_path) {
            eprintln!("Failed to rename '{}': {}", old_name, e);
        }
    }
}

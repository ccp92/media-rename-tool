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

    /// Recursive: process files in all subdirectories
    #[arg(long)]
    recursive: bool,
}

fn process_file(
    path: &std::path::Path,
    args: &Cli,
    re_movie: &Regex,
    re_tv: &Regex,
) -> Option<(std::path::PathBuf, String, String)> {
    if !path.is_file() {
        return None;
    }

    let filename = path.file_name()?.to_string_lossy().into_owned();

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

            format!("{} ({}){}", title.trim(), year, ext)
        } else {
            return None;
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

            format!("{}{}", keep.trim(), ext)
        } else {
            return None;
        }
    } else {
        return None;
    };

    if new_filename != filename {
        Some((path.to_path_buf(), filename, new_filename))
    } else {
        None
    }
}

fn collect_files_recursive(dir: &std::path::Path) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_files_recursive(&path)?);
            } else {
                files.push(path);
            }
        }
    }
    Ok(files)
}

fn main() {
    let args = Cli::parse();

    let re_movie = Regex::new(r"^(?P<title>.*?)\s*(?P<year>(?:19|20)\d{2}).*(?P<ext>\.[^.]+)$").unwrap();
    let re_tv = Regex::new(r"^(?P<keep>.*?S\d{2}E\d{2}).*(?P<ext>\.[^.]+)$").unwrap();

    let current_dir = std::path::Path::new(".");
    let mut files = Vec::new();

    if args.recursive {
        match collect_files_recursive(current_dir) {
            Ok(f) => files = f,
            Err(e) => {
                eprintln!("Error reading directory recursively: {}", e);
                return;
            }
        }
    } else {
        match fs::read_dir(current_dir) {
            Ok(paths) => {
                for path in paths {
                    if let Ok(entry) = path {
                        files.push(entry.path());
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading directory: {}", e);
                return;
            }
        }
    }

    let mut pending_renames = Vec::new();

    for path in files {
        if let Some(rename) = process_file(&path, &args, &re_movie, &re_tv) {
            pending_renames.push(rename);
        }
    }

    if pending_renames.is_empty() {
        println!("No files to rename.");
        return;
    }

    // Interactive confirmation if recursive is enabled
    if args.recursive && !args.dry_run {
         println!("Recursive mode enabled. Performing dry run check...");
         println!("The following files WILL be renamed:");
         for (_, old_name, new_name) in &pending_renames {
             println!("'{}' -> '{}'", old_name, new_name);
         }
         
         use std::io::Write;
         print!("\nDo you want to proceed? [y/N]: ");
         std::io::stdout().flush().unwrap();

         let mut input = String::new();
         std::io::stdin().read_line(&mut input).unwrap();
         
         if input.trim().to_lowercase() != "y" {
             println!("Aborted.");
             return;
         }
    } else if args.dry_run {
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

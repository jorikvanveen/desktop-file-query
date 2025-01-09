use std::path::PathBuf;
use std::{fs, path::Path};

use clap::Parser;
use color_eyre::{eyre::WrapErr, Result};

#[derive(Parser, Debug)]
struct Args {
    query: String,
}

fn get_search_text(path: &Path, file_contents: String) -> String {
    file_contents
        .lines()
        .filter(|line| {
            line.starts_with("Categories")
                || line.starts_with("Exec")
                || line.starts_with("Name")
                || line.starts_with("GenericName")
                || line.starts_with("Comment")
                || line.starts_with("Keywords")
        })
        .filter_map(|line| match line.split("=").nth(1) {
            Some(searchable_value) => Some(searchable_value.to_lowercase()),
            None => {
                eprintln!(
                    "WARING: Invalid .desktop file: {path:?}. \"{line}\" is not a valid key-value pair",
                );
                None
            }
        })
        .collect()
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let mut dirs: Vec<PathBuf> = std::env::var("XDG_DATA_DIRS")
        .wrap_err("No XDG_DATA_DIRS environment variable defined, cannot continue.")?
        .split(":")
        .map(PathBuf::from)
        .map(|path| path.join("applications"))
        .filter(|path| fs::exists(path).unwrap_or(false))
        .collect();

    match std::env::var("HOME") {
        Ok(home_dir) => {
            let data_home = match std::env::var("XDG_DATA_HOME") {
                Ok(data_home) => PathBuf::from(data_home),
                _ => PathBuf::from(home_dir).join(".local").join("share"),
            };
            let local_share_apps = data_home.join("applications");

            if local_share_apps.exists() {
                dirs.push(local_share_apps);
            }
        }
        _ => {}
    }

    let matched_files: Vec<(PathBuf, String)> = dirs
        .iter()
        .filter_map(|applications_dir| {
            let dir_entries = match fs::read_dir(applications_dir) {
                Ok(dir_entries) => dir_entries,
                Err(e) => {
                    eprintln!("WARNING: Could not list directory: {applications_dir:?}");
                    eprintln!("{e}");
                    return None;
                }
            };

            Some(
                dir_entries
                    .filter(|file| {
                        file.as_ref()
                            .unwrap()
                            .file_name()
                            .to_str()
                            .unwrap()
                            .ends_with(".desktop")
                    })
                    .filter_map(|file| {
                        let path = file.as_ref().unwrap().path();
                        let contents = match fs::read_to_string(&path) {
                            Ok(contents) => contents,
                            Err(e) => {
                                eprintln!("WARNING: Could not real file contents of {path:?}");
                                eprintln!("{e}");
                                return None;
                            }
                        };
                        Some((path.clone(), get_search_text(&path, contents)))
                    }),
            )
        })
        .flatten()
        .filter(|(_path, search_text)| search_text.contains(&args.query.to_lowercase()))
        .collect();

    let mut matched_file_paths: Vec<String> = matched_files
        .into_iter()
        .map(|(path, _)| String::from(path.to_str().unwrap()))
        .collect();
    matched_file_paths.sort();

    println!("{}", matched_file_paths.join("\n"));

    Ok(())
}

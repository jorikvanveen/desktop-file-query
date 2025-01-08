use std::fs;
use std::path::PathBuf;

use clap::Parser;
use color_eyre::{eyre::WrapErr, Result};

#[derive(Parser, Debug)]
struct Args {
    query: String,
}

fn get_search_text(file_contents: String) -> String {
    file_contents.lines().filter(|line| {
        line.starts_with("Categories")
            || line.starts_with("Exec")
            || line.starts_with("Name")
            || line.starts_with("GenericName")
            || line.starts_with("Comment")
            || line.starts_with("Keywords")
    }).map(|line| line.split("=").nth(1).unwrap().to_lowercase()).collect()
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    //println!("Searching desktop files for: {}", args.query);

    let dirs: Vec<PathBuf> = std::env::var("XDG_DATA_DIRS")
        .wrap_err("No XDG_DATA_DIRS environment variable defined, cannot continue.")?
        .split(":")
        .map(PathBuf::from)
        .map(|path| path.join("applications"))
        .filter(|path| fs::exists(path).unwrap_or(false))
        .collect();

    //println!("Searching the following directories for desktop files:");
    //println!("{:#?}", &dirs);

    let matched_files: Vec<(PathBuf, String)> = dirs
        .iter()
        .flat_map(|applications_dir| {
            fs::read_dir(applications_dir)
                .unwrap()
                .filter(|file| {
                    file.as_ref()
                        .unwrap()
                        .file_name()
                        .to_str()
                        .unwrap()
                        .ends_with(".desktop")
                })
                .map(|file| {
                    (
                        file.as_ref().unwrap().path(),
                        get_search_text(fs::read_to_string(file.unwrap().path()).unwrap()),
                    )
                })
        })
        .filter(|(_path, search_text)| search_text.contains(&args.query.to_lowercase()))
        .collect();

    let mut matched_file_paths: Vec<String> = matched_files.into_iter().map(|(path, _)| String::from(path.to_str().unwrap())).collect();
    matched_file_paths.sort();

    println!("{}", matched_file_paths.join("\n"));

    Ok(())
}

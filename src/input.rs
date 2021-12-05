use std::{
    io::Write,
    path::{Path, PathBuf},
};

use crate::puzzle::Puzzle;
use std::fs;

const INPUTS_STORAGE: &str = "inputs";
const INPUTS_URL: &str = "https://adventofcode.com";

pub fn get_input(puzzle: Puzzle) -> Result<String, String> {
    let path: String = format!("./{}/{}/day{}", INPUTS_STORAGE, puzzle.year, puzzle.day);
    let path = PathBuf::from(path);
    if path.exists() {
        read_input(&path)
    } else {
        download_input(puzzle, &path, resolve_token())
    }
}

fn resolve_token() -> Option<String> {
    std::env::var("AOC_TOKEN").ok()
}

fn read_input(path: &Path) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(input) => Ok(input),
        Err(e) => Err(e.to_string()),
    }
}

fn download_input(puzzle: Puzzle, target: &Path, token: Option<String>) -> Result<String, String> {
    let token: String = match token {
        None => {
            return Err(format!(
                "No input was found for {} and no token was given to download with either",
                puzzle
            ))
        }
        Some(t) => t,
    };
    let url: String = format!("{}/{}/day/{}/input", INPUTS_URL, puzzle.year, puzzle.day);
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(&url)
        .header("Cookie", format!("session={}", token))
        .send();

    match res {
        Ok(response) => match response.text() {
            Ok(input) => save_input(input, target),
            Err(e) => Err(format!("Unable to parse content as text: {:?}", e)),
        },
        Err(e) => Err(format!("Got an error when fetching inputs: {:?}", e)),
    }
}

fn save_input(input: String, target: &Path) -> Result<String, String> {
    let target_dir: &std::path::Path = target.parent().ok_or("Cannot get parent")?;
    fs::create_dir_all(target_dir).map_err(|err| err.to_string())?;
    let mut file: fs::File = fs::File::create(target).map_err(|err| err.to_string())?;
    file.write_all(input.as_bytes())
        .map_err(|err| err.to_string())?;

    Ok(input)
}

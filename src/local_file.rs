use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use path_slash::{PathBufExt, PathExt};

use crate::error::Error::AccessTokenFileDoesNotExist;
use crate::error::QResult;

fn create_dir(path: &str) {
    if !Path::new(path).exists() {
        let _ = fs::create_dir_all(path);
    }
}

fn qovery_directory_path() -> String {
    let home = dirs::home_dir().unwrap_or(PathBuf::from("/tmp")); // TODO change
    let uri = home.to_str().unwrap();
    let dir_path = PathBuf::from_slash(format!("{}/.qovery", uri))
        .to_str().unwrap().to_string();

    create_dir(&dir_path);

    dir_path
}

pub fn authorization_token() -> QResult<String> {
    let uri = format!("{}/access_token", qovery_directory_path());
    let pb = PathBuf::from_slash(uri);

    match fs::read_to_string(pb) {
        Ok(content) if !content.trim().is_empty() => Ok(content),
        _ => Err(AccessTokenFileDoesNotExist)
    }
}

pub fn current_directory_name() -> String {
    let lossy_name = current_dir().unwrap_or(PathBuf::from("unknown")).to_slash_lossy();
    let s_dir_name: Vec<&str> = lossy_name.split("/").collect();

    s_dir_name.as_slice().last().unwrap().to_string()
}

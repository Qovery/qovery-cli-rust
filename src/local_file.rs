use std::fs;
use std::path::{Path, PathBuf};

use path_slash::PathBufExt;

use crate::error::Error::AccessTokenFileDoesNotExist;
use crate::error::QResult;

fn create_dir(path: &str) {
    if !Path::new(path).exists() {
        fs::create_dir_all(path);
    }
}

fn get_qovery_directory_path() -> String {
    match dirs::home_dir() {
        Some(home) => {
            let uri = home.to_str().unwrap();
            let dir_path = PathBuf::from_slash(format!("{}/.qovery", uri))
                .to_str().unwrap().to_string();

            create_dir(&dir_path);

            dir_path
        }
        _ => panic!("can't locate home directory") // TODO change
    }
}

pub fn get_authorization_token() -> QResult<String> {
    let uri = format!("{}/access_token", get_qovery_directory_path());
    let pb = PathBuf::from_slash(uri);

    match fs::read_to_string(pb) {
        Ok(content) if !content.trim().is_empty() => Ok(content),
        _ => Err(AccessTokenFileDoesNotExist)
    }
}

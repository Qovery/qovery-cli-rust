use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::error::Error::{AuthTokenExpired, Unknown};
use crate::error::QResult;
use crate::local_file::authorization_token;

const API_ROOT_URL: &str = "https://api.qovery.com/api/v1";
const JSON_CONTENT_TYPE: &str = "application/json";

#[derive(Debug, Serialize, Deserialize)]
pub struct WrapperResponse<T> {
    pub results: Vec<T>,
}

impl<T> WrapperResponse<T> {
    pub fn new() -> WrapperResponse<T> {
        WrapperResponse::<T> { results: vec![] }
    }
}

fn get_headers() -> QResult<HeaderMap> {
    let auth_token = authorization_token()?;
    let bearer_auth_token = format!("Bearer {}", auth_token);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static(JSON_CONTENT_TYPE));
    headers.insert(AUTHORIZATION, bearer_auth_token.parse().unwrap());

    Ok(headers)
}

pub fn get<T: DeserializeOwned>(uri: &str) -> QResult<T> {
    let url = format!("{}/{}", API_ROOT_URL, uri);

    let res = reqwest::blocking::Client::new()
        .get(&url)
        .headers(get_headers()?)
        .send();

    return match res {
        Ok(r) => {
            match r.json::<T>() {
                Ok(results) => Ok(results),
                Err(err) => Err(Unknown(err.to_string()))
            }
        }
        Err(err) => Err(Unknown(err.to_string()))
    };
}

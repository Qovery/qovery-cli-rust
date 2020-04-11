use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api;
use crate::api::WrapperResponse;
use crate::error::{Error, QResult};
use crate::error::Error::{AuthTokenExpired, AuthTokenNotFound};
use crate::local_file::get_authorization_token;

pub type Projects = WrapperResponse<Project>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub name: String,
}

pub fn list() -> QResult<Projects> {
    api::get::<Projects>("project")
}

pub fn get(id: &str) -> QResult<Project> {
    api::get::<Project>(format!("project/{}", id).as_str())
}

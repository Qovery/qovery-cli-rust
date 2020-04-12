use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api;
use crate::api::WrapperResponse;
use crate::application::Application;
use crate::cloud_provider::CloudProviderRegion;
use crate::error::{Error, QResult};
use crate::error::Error::{AuthTokenExpired, AuthTokenNotFound};
use crate::local_file::get_authorization_token;
use crate::router::Router;
use crate::service::{Database, Service};
use crate::status::Status;

pub type Environments = WrapperResponse<Environment>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub status: Status,
    pub cloud_provider_region: CloudProviderRegion,
    pub applications: Option<Vec<Application>>,
    pub databases: Option<Vec<Database>>,
    pub routers: Option<Vec<Router>>,
}

pub fn list(project_id: &str) -> QResult<Environments> {
    api::get::<Environments>(format!("project/{}/environment", project_id).as_str())
}

pub fn get(project_id: &str, environment_id: &str) -> QResult<Environment> {
    let uri = format!("project/{}/environment/{}", project_id, environment_id);
    api::get::<Environment>(uri.as_str())
}

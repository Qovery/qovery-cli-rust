use serde::{Deserialize, Serialize};

use crate::api;
use crate::api::WrapperResponse;
use crate::error::QResult;
use crate::repository::Repository;
use crate::service::Database;
use crate::status::Status;

pub type Applications = WrapperResponse<Application>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub name: Option<String>,
    pub status: Option<Status>,
    pub connection_uri: Option<String>,
    pub databases: Option<Vec<Database>>,
    pub repository: Option<Repository>,
}

pub struct VecApplication<'a>(pub &'a Vec<Application>);

impl VecApplication<'_> {
    pub fn application_names(&self) -> Vec<Option<String>> {
        self.0.iter()
            .map(|x| x.name.clone())
            .collect::<Vec<Option<String>>>()
    }
}

pub fn list(project_id: &str, environment_id: &str) -> QResult<Applications> {
    api::get::<Applications>(format!("project/{}/environment/{}/application",
                                     project_id, environment_id).as_str())
}

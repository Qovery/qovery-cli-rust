use serde::{Deserialize, Serialize};

use crate::repository::Repository;
use crate::service::Database;
use crate::status::Status;

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub status: Status,
    pub connection_uri: String,
    pub databases: Option<Vec<Database>>,
    pub repository: Repository,
}

pub struct VecApplication<'a>(pub &'a Vec<Application>);

impl VecApplication<'_> {
    pub fn application_names(&self) -> Vec<String> {
        self.0.iter()
            .map(|x| x.name.to_string())
            .collect::<Vec<String>>()
    }
}

pub fn list() {}

use serde::{Deserialize, Serialize};

use crate::repository::Repository;
use crate::service::Database;
use crate::status::Status;

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

pub fn list() {}

use serde::{Deserialize, Serialize};

use crate::application::Application;
use crate::constant::OUT_UNKNOWN;
use crate::status::Status;

pub type Database = Service;

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub category: Option<String>,
    pub version: Option<String>,
    pub status: Option<Status>,
    pub fqdn: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub applications: Option<Vec<Application>>,
}

pub struct VecService<'a>(pub &'a Vec<Service>);

impl VecService<'_> {
    pub fn service_names(&self) -> Vec<Option<String>> {
        self.0.iter()
            .map(|x| x.name.clone())
            .collect::<Vec<Option<String>>>()
    }

    pub fn database_names(&self) -> Vec<Option<String>> {
        self.0.iter()
            .filter(|&x| x.category == Some("DATABASE".to_string()))
            .map(|x| x.name.clone())
            .collect::<Vec<Option<String>>>()
    }
}

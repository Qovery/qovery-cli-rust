use serde::{Deserialize, Serialize};

use crate::application::Application;
use crate::status::Status;

pub type Database = Service;

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub version: String,
    pub status: Status,
    pub fqdn: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub applications: Option<Vec<Application>>,
}

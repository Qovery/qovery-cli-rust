use serde::{Deserialize, Serialize};

use crate::qovery_conf::Error::{MissingApplication, MissingApplicationCloudRegion,
                                MissingApplicationName, MissingApplicationProject};

#[derive(Debug, Serialize, Deserialize)]
pub struct QoveryYML {
    pub application: Option<QoveryYMLApplication>,
    pub databases: Vec<QoveryYMLDatabase>,
    pub routers: Vec<QoveryYMLRouter>,
}

impl QoveryYML {
    fn check_error(self) -> Option<Error> {
        match self.application {
            Some(x) if x.name.is_none() => Some(MissingApplicationName),
            Some(x) if x.project.is_none() => Some(MissingApplicationProject),
            Some(x) if x.cloud_region.is_none() => Some(MissingApplicationCloudRegion),
            Some(x) => None,
            None => Some(MissingApplication)
        }
    }
}

enum Error {
    MissingApplication,
    MissingApplicationName,
    MissingApplicationProject,
    MissingApplicationCloudRegion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QoveryYMLApplication {
    pub name: Option<String>,
    pub project: Option<String>,
    pub cloud_region: Option<String>,
    pub publicly_accessible: Option<bool>,
    pub dockerfile: Option<String>,
}

impl QoveryYMLApplication {
    fn sanitized_name(self) -> Option<String> {
        self.name.map(|x| x.to_lowercase())
    }

    fn dockerfile_path(self) -> String {
        self.dockerfile.map_or_else(|| "Dockerfile".to_string(), |x| x)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QoveryYMLDatabase {
    #[serde(rename = "type")]
    pub type_: String,
    pub version: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QoveryYMLRouter {
    pub name: String,
    pub custom_domain: String,
    pub routes: Vec<QoveryYMLRoute>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QoveryYMLRoute {
    pub application_name: String,
    pub paths: Vec<String>,
}

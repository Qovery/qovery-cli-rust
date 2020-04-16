use serde::{Deserialize, Serialize};

use crate::api;
use crate::api::WrapperResponse;
use crate::error::QResult;

pub type CloudProviders = WrapperResponse<CloudProvider>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudProvider {
    pub id: String,
    pub name: String,
    pub description: String,
    pub regions: Vec<CloudProviderRegion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudProviderRegion {
    pub id: String,
    pub full_name: String,
    pub description: String,
}

pub fn list() -> QResult<CloudProviders> {
    api::get::<CloudProviders>("cloud")
}

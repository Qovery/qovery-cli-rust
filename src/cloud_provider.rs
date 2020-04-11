use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudProvider {
    pub id: String,
    pub name: String,
    pub regions: Vec<CloudProviderRegion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudProviderRegion {
    pub id: String,
    pub full_name: String,
    pub description: String,
}


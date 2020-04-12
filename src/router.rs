use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Router {
    pub connection_uri: String,
}

pub struct VecRouter<'a>(pub &'a Vec<Router>);

impl VecRouter<'_> {
    pub fn connection_uris(&self) -> Vec<String> {
        self.0.iter()
            .map(|x| x.connection_uri.to_string())
            .collect::<Vec<String>>()
    }
}

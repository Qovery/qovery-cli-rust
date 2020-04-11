pub struct Conf {
    pub project_name: Option<String>,
    pub branch_name: Option<String>,
    pub project_id: Option<String>,
    pub environment_id: Option<String>,
    pub application_id: Option<String>,
    pub commit_id: Option<String>,
}

impl Conf {
    pub fn new() -> Conf {
        Conf {
            project_name: None,
            branch_name: None,
            project_id: None,
            application_id: None,
            environment_id: None,
            commit_id: None,
        }
    }
}

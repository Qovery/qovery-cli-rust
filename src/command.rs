use chrono::Utc;
use prettytable::Table;
use timeago::Formatter;

use crate::{environment, project};
use crate::conf::Conf;
use crate::constant::{COL_APPLICATIONS, COL_BRANCH, COL_CREATED_AT, COL_DATABASES,
                      COL_ENDPOINTS, COL_NAME, COL_REGION, COL_STATUS};
use crate::environment::Environments;
use crate::error::Error::AuthTokenExpired;
use crate::error::QResult;
use crate::project::Projects;
use crate::table::get_table;

pub fn list_projects() {
    let res = match project::list() {
        Ok(projects) => projects,
        Err(AuthTokenExpired) => Projects::new(),
        Err(_) => unimplemented!()
    };

    let mut table = get_table();

    table.set_titles(row![
        COL_NAME,
        COL_CREATED_AT
    ]);

    let formatter = Formatter::new();
    let now = Utc::now();

    res.results.into_iter().for_each(|x| {
        table.add_row(row![
            x.name,
            formatter.convert_chrono(x.created_at, now)
        ]);
    });

    table.printstd();
}

pub fn list_environments(conf: &Conf) {
    let res = match environment::list(conf.project_id.as_ref().unwrap()) {
        Ok(environments) => environments,
        //Err(AuthTokenExpired) => Environments::new(),
        Err(_) => unimplemented!()
    };

    let mut table = get_table();

    table.set_titles(row![
        COL_BRANCH,
        COL_STATUS,
        COL_ENDPOINTS,
        COL_REGION,
        COL_APPLICATIONS,
        COL_DATABASES
    ]);

    res.results.iter().for_each(|x| {
        table.add_row(row![
            conf.branch_name.as_ref().unwrap(),
            x.status.code_message,
            x.connection_uris().join(", "),
            format!("{} ({})", x.cloud_provider_region.full_name, x.cloud_provider_region.description),
            "",
            "",
        ]);
    });

    table.printstd();
}

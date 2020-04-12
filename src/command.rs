use chrono::Utc;
use prettytable::Table;
use timeago::Formatter;

use crate::{environment, project};
use crate::application::VecApplication;
use crate::conf::Conf;
use crate::constant::{COL_APPLICATIONS, COL_BRANCH, COL_CREATED_AT, COL_DATABASES, COL_ENDPOINTS, COL_NAME, COL_REGION, COL_STATUS, OUT_NONE, OUT_UNKNOWN};
use crate::environment::Environments;
use crate::error::Error::{AuthTokenExpired, Unknown};
use crate::error::QResult;
use crate::project::Projects;
use crate::router::VecRouter;
use crate::service::VecService;
use crate::table::{get_table, unwrap_or};

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
        Err(Unknown(err)) => {
            println!("{}", err);
            unimplemented!()
        }
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

    res.results.iter().for_each(|env| {
        let endpoints = VecRouter(&env.routers.as_ref().unwrap_or(&vec![]))
            .connection_uris().join(", ");

        let application_names = VecApplication(&env.applications.as_ref().unwrap_or(&vec![]))
            .application_names().iter()
            .map(|x| x.clone().unwrap_or(OUT_UNKNOWN.to_string()))
            .collect::<Vec<String>>()
            .join(", ");

        let database_names = VecService(&env.databases.as_ref().unwrap_or(&vec![]))
            .database_names().iter()
            .map(|x| x.clone().unwrap_or(OUT_UNKNOWN.to_string()))
            .collect::<Vec<String>>()
            .join(", ");

        let region = format!("{} ({})", env.cloud_provider_region.full_name,
                             env.cloud_provider_region.description);

        table.add_row(row![
            conf.branch_name.as_ref().unwrap(),
            env.status.code_message_colored(),
            unwrap_or(endpoints, OUT_NONE),
            unwrap_or(region, OUT_NONE),
            unwrap_or(application_names, OUT_NONE),
            unwrap_or(database_names, OUT_NONE),
        ]);
    });

    table.printstd();
}

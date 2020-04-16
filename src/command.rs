use std::{fmt, fs};
use std::borrow::Borrow;
use std::io::Cursor;
use std::process::exit;

use chrono::Utc;
use colored::Colorize;
use console::Term;
use dialoguer::{Input, Select};
use dialoguer::theme::{ColorfulTheme, CustomPromptCharacterTheme};
use prettytable::Table;
use serde_json::error::Category::Data;
use timeago::Formatter;
use tiny_http::{Header, HeaderField, Response, StatusCode};

use crate::{application, cloud_provider, environment, project};
use crate::application::VecApplication;
use crate::cloud_provider::CloudProviderRegion;
use crate::conf::Conf;
use crate::constant::*;
use crate::environment::Environments;
use crate::error::{Error, QResult};
use crate::error::Error::{AuthTokenExpired, Unknown};
use crate::local_file::current_directory_name;
use crate::project::Projects;
use crate::qovery_conf::{QoveryYML, QoveryYMLApplication};
use crate::router::VecRouter;
use crate::service::VecService;
use crate::table::{get_table, unwrap_or};

fn check_error<T>(err: QResult<T>) {
    match err {
        _ => unimplemented!()
    }
}

pub fn auth(conf: &Conf) {
    println!("Opening your browser, waiting for your authentication...");
    webbrowser::open(AUTH_URL);

    let server = tiny_http::Server::http("localhost:10999").unwrap();

    loop {
        let request = match server.recv() {
            Ok(req) => req,
            Err(err) => {
                println!("error: {}", err);
                break;
            }
        };

        if request.url().starts_with("/authorization/validate") {
            break;
        }

        /*
        let data = Response::from_string(AUTH_RESPONSE);
        let data:&mut  = data.into();
        let data_len = data.len();

        let r = Response::new(
            StatusCode(200),
            vec![
                Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=UTF-8"[..]).unwrap()
            ],
            Cursor::new(data.into_bytes()),
            Some(data_len),
            None,
        );


        &request.respond(r);
        */
        //&request.respond(Response::from_string("Authentication successful. You can close this window."));
    }

    println!("Authentication successful!")
}

pub fn list_projects() {
    let res = match project::list() {
        Ok(projects) => projects,
        err => {
            check_error(err);
            exit(1)
        }
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
        err => {
            check_error(err);
            exit(1)
        }
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
            env.name.clone(),
            env.status.code_message_colored(),
            unwrap_or(endpoints, OUT_NONE),
            unwrap_or(region, OUT_NONE),
            unwrap_or(application_names, OUT_NONE),
            unwrap_or(database_names, OUT_NONE),
        ]);
    });

    table.printstd();
}

pub fn list_applications(conf: &Conf) {
    let res = match application::list(conf.project_id.as_ref().unwrap(),
                                      conf.environment_id.as_ref().unwrap()) {
        Ok(applications) => applications,
        err => {
            check_error(err);
            exit(1)
        }
    };

    let mut table = get_table();

    table.set_titles(row![
        COL_APPLICATION_NAME,
        COL_STATUS,
        COL_ENDPOINT,
        COL_DATABASES
    ]);

    res.results.iter().for_each(|app| {
        let database_names = VecService(&app.databases.as_ref().unwrap_or(&vec![]))
            .database_names().iter()
            .map(|x| x.clone().unwrap_or(OUT_UNKNOWN.to_string()))
            .collect::<Vec<String>>()
            .join(", ");

        table.add_row(row![
            app.name.clone().unwrap_or(OUT_UNKNOWN.to_string()),
            app.status.as_ref().unwrap().code_message_colored(),
            app.connection_uri.clone().unwrap_or(OUT_NONE.to_string()),
            unwrap_or(database_names, OUT_NONE),
        ]);
    });

    table.printstd();
}

pub fn init(conf: &Conf) {
    let projects = match project::list() {
        Ok(results) => results,
        err => {
            check_error(err);
            exit(1)
        }
    };

    let prompt_char_theme = CustomPromptCharacterTheme::new('>');
    let select_theme = ColorfulTheme::default();

    let term = Term::stdout();

    term.set_title("Qovery init...");
    term.write_line(ASCII_NAME);
    term.write_line("Reply to the following questions to initialize Qovery for this application");
    term.write_line(format!("For more info: {}", "https://docs.qovery.com\n".bold()).as_str());

    let is_new_project = if projects.results.len() > 1 {
        // create new project or use existing one?
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose the project you want")
            .default(0)
            .items(&["Use existing project", "New project"])
            .interact()
            .unwrap()
    } else {
        0
    };

    // ask for project
    let project_names = projects.results.into_iter()
        .map(|x| x.name)
        .collect::<Vec<String>>();

    let project_name = if is_new_project == 0 {
        let project_name_idx = Select::with_theme(&select_theme)
            .with_prompt("Choose the project you want")
            .default(0)
            .items(project_names.as_slice())
            .interact()
            .unwrap();

        project_names[project_name_idx].clone()
    } else {
        loop {
            let project_name = Input::with_theme(&prompt_char_theme)
                .with_prompt("Project name")
                .interact()
                .unwrap();

            if !project_names.contains(&project_name) {
                break project_name;
            }

            term.write_line(format!("{} {}", project_name.as_str().bold().yellow(),
                                    "exists already").as_str());
        }
    };

    let application_name = current_directory_name();

    let cloud_providers = match cloud_provider::list() {
        Ok(results) => results,
        err => {
            check_error(err);
            exit(1)
        }
    };

    term.write_line(" ");

    // ask for cloud provider
    let cloud_provider_names = cloud_providers.results.as_slice().into_iter()
        .map(|x| format!("{} ({})", x.name.to_uppercase(), x.description))
        .collect::<Vec<String>>();

    let cloud_provider_name_idx = Select::with_theme(&select_theme)
        .with_prompt("Choose the Cloud provider you want")
        .default(0)
        .items(cloud_provider_names.as_slice())
        .interact()
        .unwrap();

    // ask for cloud provider region
    let region_names = cloud_providers.results.as_slice().into_iter()
        .map(|x| &x.regions)
        .flatten()
        .map(|x| format!("{} ({})", x.full_name.to_uppercase(), x.description))
        .collect::<Vec<String>>();

    let region_name_idx = Select::with_theme(&select_theme)
        .with_prompt("Choose the region you want")
        .default(0)
        .items(region_names.as_slice())
        .interact()
        .unwrap();

    let take_first = |vec: Vec<String>, idx: usize| {
        let c = vec.get(idx).unwrap().split_whitespace().collect::<Vec<_>>();
        c.first().unwrap().to_string()
    };

    let cpn = take_first(cloud_provider_names, cloud_provider_name_idx);
    let cprn = take_first(region_names, region_name_idx);

    let cloud_provider_repr = format!("{}/{}", cpn.to_lowercase(), cprn.to_lowercase());

    term.write_line(" ");

    // ask for database
    let ask_for_a_database = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you need a database? (PostgreSQL, MongoDB, Redis...)")
        .default(0)
        .items(&["No", "Yes"])
        .interact()
        .unwrap();

    let mut databases = vec![];
    if ask_for_a_database == 1 {
        loop {
            let _databases = ["PostgreSQL", "MongoDB", "MySQL"];

            let database_choice = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose the database you need")
                .default(0)
                .items(&_databases)
                .interact()
                .unwrap();

            let database_name = _databases[database_choice];

            let versions = match database_name {
                "PostgreSQL" => vec!["11.5", "11.4", "11.2", "11.1", "10.10", "9.6"],
                "MongoDB" => vec!["3.6"],
                "MySQL" => vec!["8.0", "5.7", "5.6", "5.5"],
                _ => vec![],
            };

            let version_choice = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Choose your needed {} version", database_name).as_str())
                .default(0)
                .items(&versions.as_slice())
                .interact()
                .unwrap();

            let version = versions[version_choice];
            databases.push((database_name, version));

            let add_another_database = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to add another database?")
                .default(0)
                .items(&["No", "Yes"])
                .interact()
                .unwrap();

            term.write_line(" ");

            if add_another_database == 0 {
                break;
            };
        }
    }

    // TODO Create and save .qovery.yml

    term.write_line(format!("{} {}", "✓".green(), "Your Qovery configuration file has \
    been successfully created (.qovery.yml)").as_ref());

    term.write_line(format!("\n{}", "!!! IMPORTANT !!!".bold().yellow()).as_ref());
    term.write_line(format!("{}", "Qovery needs to get access to your git repository".yellow()).as_ref());
    term.write_line("https://github.com/apps/qovery/installations/new");

    let open_the_link = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to open the link above?")
        .default(0)
        .items(&["No", "Yes"])
        .interact()
        .unwrap();

    if open_the_link == 1 {
        webbrowser::open("https://github.com/apps/qovery/installations/new");
    }

    term.write_line(format!("\n{}", "!!! IMPORTANT !!!".bold().yellow()).as_ref());
    term.write_line("1/ Commit and push the \".qovery.yml\" file to get your app deployed");
    term.write_line("➤ Run: git add .qovery.yml && git commit -m \"add .qovery.yml\" && git push -u origin master");
    term.write_line("\n2/ Check the status of your deployment");
    term.write_line("➤ Run: qovery status");
    let _ = term.write_line("\nEnjoy! 👋");
}



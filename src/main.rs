extern crate chrono;
#[macro_use]
extern crate prettytable;
extern crate serde_json;

use clap::{App, AppSettings, Arg, ArgMatches, crate_authors, crate_version, SubCommand};

use constant::{APPLICATION, DELETE, ENV, ENVIRONMENT, LIST, LOG, PROJECT, START, STATUS};

use crate::conf::Conf;

mod constant;
mod application;
mod project;
mod local_file;
mod error;
mod api;
mod command;
mod table;
mod conf;
mod qovery_conf;
mod environment;
mod status;
mod cloud_provider;
mod router;

fn get_app_settings() -> &'static [AppSettings] {
    &[
        AppSettings::ArgRequiredElseHelp,
        AppSettings::SubcommandRequiredElseHelp,
    ]
}

fn get_project_argument<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(PROJECT)
        .short("p")
        .takes_value(true)
        .help("Your project name")
        .required(false)
}

fn get_environment_argument<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(ENVIRONMENT)
        .short("e")
        .takes_value(true)
        .help("Your environment name")
        .required(false)
}

fn get_application_argument<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(APPLICATION)
        .short("a")
        .takes_value(true)
        .help("Your application name")
        .required(false)
}

fn get_conf(args: &ArgMatches) -> Conf {
    let mut conf = Conf::new();
    conf.project_name = Some("simple-example-node-with-postgresql".to_string());
    conf.branch_name = Some("master".to_string());
    conf.project_id = Some("qut1l7vbcsau8wqg".to_string());
    conf.environment_id = Some("klp93nel2kfwo4db".to_string());
    conf.application_id = Some("rzc4hj7il90n7xer".to_string());
    conf
}

fn main() {
    let app = App::new("qovery")
        .version(crate_version!())
        .author(crate_authors!())
        .about("The 'qovery' Command Line Interface (CLI) lets you manage \
        your Qovery environment seamlessly.")
        .settings(get_app_settings())
        .subcommands(vec![
            SubCommand::with_name(PROJECT)
                .about("Perform project actions")
                .settings(get_app_settings())
                .subcommands(vec![
                    SubCommand::with_name(LIST)
                        .about("List all projects"),
                    SubCommand::with_name(ENV)
                        .about("List environment variables"),
                ]),
            SubCommand::with_name(ENVIRONMENT)
                .about("Perform environment actions")
                .alias("env")
                .settings(get_app_settings())
                .subcommands(vec![
                    SubCommand::with_name(LIST)
                        .about("List all environments")
                        .arg(get_project_argument()),
                    SubCommand::with_name(STATUS)
                        .about("Show environment status"),
                    SubCommand::with_name(START)
                        .about("Start/deploy environment"),
                    SubCommand::with_name(DELETE)
                        .about("Delete environment"),
                    SubCommand::with_name(ENV)
                        .about("List environment variables"),
                ]),
            SubCommand::with_name(APPLICATION)
                .about("Perform application actions")
                .alias("app")
                .settings(get_app_settings())
                .subcommands(vec![
                    SubCommand::with_name(LIST)
                        .about("List all applications"),
                    SubCommand::with_name(LOG)
                        .about("Show application logs"),
                    SubCommand::with_name(ENV)
                        .about("List environment variables"),
                ]),
            SubCommand::with_name("status")
                .about("Show status from current project and environment")
        ]);

    let args = app.get_matches();

    let conf = get_conf(&args);

    if let Some(m) = args.subcommand_matches("project") {
        match m.subcommand_name() {
            Some(LIST) => command::list_projects(),
            _ => ()
        }
    } else if let Some(m) = args.subcommand_matches("environment") {
        match m.subcommand_name() {
            Some(LIST) => command::list_environments(&conf),
            _ => ()
        }
    } else if let Some(m) = args.subcommand_matches("application") {
        match m.subcommand_name() {
            Some(LIST) => application::list(),
            Some(LOG) => print!("show logs from application"),
            _ => ()
        }
    }
}

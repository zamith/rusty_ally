#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
extern crate clap;

mod models;
mod schema;
mod database;
mod scheduling;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

fn main() {
    let reschedule_to_monday = SubCommand::with_name("monday")
        .about("Reschedules all the tasks that are incomplete on Friday \
                to Monday. Must be run on a Friday.");

    let schedule = SubCommand::with_name("schedule")
        .about("Schedules a task with the given description to the given date")
        .arg(Arg::with_name("TASK_DATE")
             .help("The date for the task in the format: yyyy-mm-dd")
             .required(true)
        )
        .arg(Arg::with_name("TASK_DESCRIPTION")
             .short("d")
             .long("description")
             .help("The description for the task")
             .takes_value(true)
             .required(true)
        );

    let matches = App::new("rally")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(reschedule_to_monday)
        .subcommand(schedule)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        ("monday", Some(_)) => self::scheduling::reschedule_to_monday(),
        ("schedule", Some(args)) => run_schedule_command(args),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}

fn run_schedule_command(matches: &ArgMatches) {
    let desc = matches.value_of("TASK_DESCRIPTION").unwrap();
    let date = matches.value_of("TASK_DATE").unwrap();

    self::scheduling::schedule(desc, date);
}

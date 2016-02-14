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
mod reschedule_to_monday;

use clap::{App, AppSettings, SubCommand};

fn main() {
    let reschedule_to_monday = SubCommand::with_name("monday")
        .about("Reschedules all the tasks that are incomplete on Friday \
                to Monday. Must be run on a Friday.");

    let matches = App::new("rally")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(reschedule_to_monday)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        ("monday", Some(_)) => run_monday_command(),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}

fn run_monday_command() {
    self::reschedule_to_monday::reschedule();
}

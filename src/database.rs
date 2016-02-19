extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::process::Command;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let os_username = Command::new("whoami").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    }).stdout;

    let database_url = env::var("DATABASE_URL").
        unwrap_or(
            format!(
                "/Users/{}/Library/Application Support/ActionAlly/action_ally.db",
                String::from_utf8_lossy(&os_username).trim()
            )
        );

    SqliteConnection::establish(&database_url).
        expect(&format!("Error connection to {}", database_url))
}

use models::{NewTask};

pub fn create_task(conn: &SqliteConnection, task: &str, day: &str, status: &str) -> () {
    use schema::tasks;

    let new_task = NewTask {
        task: task,
        day: day,
        status: status,
    };

    diesel::insert(&new_task)
        .into(tasks::table)
        .execute(conn)
        .expect("Error saving task");
}

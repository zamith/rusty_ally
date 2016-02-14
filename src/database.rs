extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").
        expect("DATABASE_URL must be set");

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

extern crate rusty_ally;
extern crate diesel;

use self::rusty_ally::*;
// use self::rusty_ally::models::*;
use self::diesel::prelude::*;

fn main() {
    // use rusty_ally::schema::tasks::dsl::*;

    let connection = establish_connection();
    // let results = tasks.limit(5).load::<Task>(&connection).expect("Error loading tasks");

    // println!("Found {} tasks", results.len());
}

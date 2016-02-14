use super::database::*;

use models::{Task};
use chrono::*;
use diesel::prelude::*;

pub fn reschedule() {
    use schema::tasks::dsl::*;

    let connection = establish_connection();

    on_last_friday(|maybe_last_friday| {
        let last_friday = maybe_last_friday
            .expect("You must run this on a Friday");

        let results = tasks
            .filter(day.eq(as_day(last_friday)).and(status.eq("not-done")))
            .load::<Task>(&connection)
            .expect("Error loading tasks");

        let next_monday = as_day(last_friday + Duration::days(3));

        for task_struct in results {
            println!("Adding {}, to {}", task_struct.task, next_monday);
            create_task(&connection,
                task_struct.task.as_str(),
                next_monday.as_str(),
                "not-done"
            );
        }
    })
}

fn on_last_friday<F>(function: F) -> ()
    where F : Fn(Option<Date<Local>>) -> () {

    let today: Date<Local> = Local::today();
    let last_friday = match today.weekday() {
        Weekday::Sun => Some(today.pred().pred()),
        Weekday::Sat => Some(today.pred()),
        Weekday::Fri => Some(today),
        _ => None,
    };
    function(last_friday);
}

fn as_day(date: Date<Local>) -> String {
    date.format("%Y-%m-%d").to_string()
}

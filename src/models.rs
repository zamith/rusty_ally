use super::schema::tasks;

#[insertable_into(tasks)]
pub struct NewTask<'a> {
    pub task: &'a str,
    pub day: &'a str,
    pub status: &'a str,
}

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub task: String,
    pub detail: Option<String>,
    pub why: Option<String>,
    pub day: String,
    pub ordr: Option<i32>,
    pub last_ordr: Option<i32>,
    pub status: String,
}

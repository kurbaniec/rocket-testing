use crate::{db, TestDB};
use rocket::response::content::Json;

#[get("/add")]
pub fn create_post(conn: TestDB) -> Json<String> {
    // Create a post
    let title = String::from("Something");
    let body = String::from("Great");
    // Save it to db
    let post = db::posts::create_post(&*conn, &title, &body);
    // Send JSON
    // See: https://rocket.rs/v0.4/guide/responses/
    Json(format!("{{ 'id': {} }}", &post.id))
}

#[get("/event")]
pub fn create_event(conn: TestDB) -> String {
    db::posts::create_post_event(&*conn);
    String::from("ok!")
}

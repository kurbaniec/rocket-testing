#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use mysql::prelude::*;
use mysql::*;
use rocket::response::content::Json;
use std::borrow::BorrowMut;

// Import database operations
mod db;
// Import defined model structs
mod models;
// Import generated schemas
mod schema;

// Configure Database
#[database("test_db")]
struct TestDB(diesel::MysqlConnection);
#[database("test_db")]
struct EventDB(mysql::Conn);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/db")]
fn create_post(conn: TestDB) -> Json<String> {
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
fn create_event(mut conn: EventDB) -> String {
    db::posts::create_post_event(&mut *conn);
    String::from("ok!")
}

fn main() {
    rocket::ignite()
        .attach(TestDB::fairing())
        .attach(EventDB::fairing())
        .mount("/", routes![index, create_post, create_event])
        .launch();
}

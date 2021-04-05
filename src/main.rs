#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

// Import database operations
mod db;
// Import defined model structs
mod models;
// Import generated schemas
mod schema;
// import routes
mod routes;
// import utilities
mod utils;

// Configure Database
#[database("test_db")]
pub struct TestDB(diesel::MysqlConnection);

// Start
fn main() {
    // TODO check if resources folder is copied successfully
    // println!("{}", Path::new("./resources").exists());
    rocket::ignite()
        .attach(TestDB::fairing())
        .mount(
            "/",
            routes![
                routes::test::index,
                routes::test::image,
                routes::db::create_post,
                routes::db::create_event
            ],
        )
        .launch();
}

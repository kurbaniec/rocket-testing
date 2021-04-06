#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_multipart_form_data;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate crypto;

// Import database operations
mod db;
// Import defined model structs
mod models;
// Import generated schemas
mod schema;
// import routes
mod routes;
// import auth handlers
mod auth;
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
                routes::test::form,
                routes::test::auth_test,
                routes::db::create_post,
                routes::db::create_event,
                routes::db::create_user,
            ],
        )
        .launch();
}

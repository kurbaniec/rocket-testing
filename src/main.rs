#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use rocket_testing::create_post;

#[database("test_db")]
struct TestDB(diesel::MysqlConnection);

#[get("/")]
fn index() -> &'static str {
    let baum: String = String::from("Baunere");
    let baum: String = String::from("Baun");
    "Hello, world!"
}

#[get("/db")]
fn get_db(conn: TestDB) -> &'static str {
    let title = String::from("Something");
    let body = String::from("Great");
    let post = create_post(&*conn, &title, &body);
    "Hello, db!"
}

fn main() {
    let baum: String = String::from("Baun");
    let baum: String = String::from("Baun");
    // rocket::ignite().mount("/", routes![index]).launch();
}

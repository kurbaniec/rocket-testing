#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    let baum: String = String::from("Baunere");
    let kek = kek();
    let baum: String = String::from("Baun");
    "Hello, world!"
}

fn kek() -> String {
    let baum: String = String::from("Baun");
    let baum: String = String::from("Baun");
    baum
}

fn main() {
    let baum: String = String::from("Baun");
    let baum: String = String::from("Baun");

    rocket::ignite().mount("/", routes![index]).launch();
}

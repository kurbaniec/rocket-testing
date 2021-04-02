pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NewPost, Post};

pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts::dsl::{id, posts};

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    posts.order(id.desc()).first(conn).unwrap()
}

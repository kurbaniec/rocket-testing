use crate::models::{NewPost, Post};
use crate::schema::posts::dsl::{id, posts};
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use mysql::prelude::*;
use mysql::*;
use std::borrow::BorrowMut;

pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) -> Post {
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

/**
pub fn create_post_event(conn: &mut Conn) {
    // See: https://docs.rs/mysql/14.0.0/mysql/struct.QueryResult.html
    conn.query(
        r#"
        CREATE EVENT IF NOT EXISTS test_event_01
        ON SCHEDULE AT CURRENT_TIMESTAMP
        DO
          UPDATE posts
          SET title = 'TestRocket!'
          WHERE id = '2';
    "#,
    )
    .unwrap();
}*/

pub fn create_post_event(conn: &MysqlConnection) {
    conn.batch_execute(
        r#"
        CREATE EVENT IF NOT EXISTS test_event_01
        ON SCHEDULE AT CURRENT_TIMESTAMP
        DO
          UPDATE posts
          SET title = 'TestRocket!!!'
          WHERE id = '2';
    "#,
    );
}

use diesel::prelude::*;

use crate::models::{NewPost, Post};
use crate::schema::posts::dsl::{id, posts};

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

pub fn create_post_event(conn: &MysqlConnection) {
    let _ = diesel::sql_query(
        r#"
        CREATE EVENT IF NOT EXISTS test_event_01
        ON SCHEDULE AT CURRENT_TIMESTAMP
        DO
          UPDATE posts 
          SET title = 'TestRocket'
          WHERE id = '2';
    "#,
    )
    .execute(conn);
    //diesel::mysql::Mysql::
    //println!("{}", rows);
}

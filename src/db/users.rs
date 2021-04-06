use crate::models::{NewUser, User};
use crate::schema::users::dsl::{id, users};
use diesel::prelude::*;

pub fn create_user<'a>(conn: &MysqlConnection, username: String, password: String) -> User {
    let new_user = NewUser { username, password };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

    users.order(id.desc()).first(conn).unwrap()
}

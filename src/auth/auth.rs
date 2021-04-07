use crate::models::User;
use crate::TestDB;
use diesel::result::Error;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct AuthenticatedUser {
    user_id: u64,
}

#[derive(Debug)]
pub enum LoginError {
    InvalidData,
    UsernameDoesNotExist,
    WrongPassword,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = LoginError;
    fn from_request(request: &'a Request<'r>) -> Outcome<AuthenticatedUser, LoginError> {
        let username = request.headers().get_one("username");
        let password = request.headers().get_one("password");

        for r in request.headers().iter() {
            println!("{}", r.name);
            println!("{}", r.value);
            println!("---");
        }

        let auth = request.headers().get_one("Authorization");
        match auth {
            None => {
                println!("none")
            }
            Some(str) => {
                let info = str
                    .replace("basic", "")
                    .replace("Basic", "")
                    .replace(" ", "");
                println!("str {}", info);
                let result = String::from_utf8(base64::decode(info).unwrap()).unwrap();
                println!("str {}", &result)
            }
        }

        match (username, password) {
            (Some(u), Some(p)) => {
                // See: https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html#request-local-state
                let auth_result =
                    request.local_cache(|| match request.guard::<TestDB>().succeeded() {
                        None => Err("No db connection"),
                        Some(conn) => {
                            let user = crate::db::users::get_user(&*conn, u);
                            match user {
                                Ok(user) => {
                                    let hashed_password =
                                        crate::auth::crypto::hash_password(&p.to_string());
                                    if user.password == hashed_password {
                                        Ok(user.id)
                                    } else {
                                        Err("Mismatched passwords")
                                    }
                                }
                                Err(_) => Err("No such user"),
                            }
                        }
                    });
                match auth_result {
                    Ok(id) => Outcome::Success(AuthenticatedUser { user_id: *id }),
                    Err(_) => Outcome::Failure((Status::BadRequest, LoginError::InvalidData)),
                }
            }
            _ => Outcome::Failure((Status::BadRequest, LoginError::InvalidData)),
        }
    }
}

use crate::auth::auth::AuthenticatedUser;
use crate::utils::res_path;
use rocket::http::{ContentType, Cookie, Cookies, Status};
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket::Data;
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use std::fs;
use std::option::Option::Some;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/image")]
pub fn image() -> Result<NamedFile, NotFound<String>> {
    // See: https://rocket.rs/v0.4/guide/responses/#result
    let path = res_path().join("doge.jpg");
    NamedFile::open(&path).map_err(|_| NotFound("Image not found".to_string()))
}

#[post("/form", data = "<data>")]
pub fn form(content_type: &ContentType, data: Data) -> &'static str {
    // See: https://docs.rs/rocket-multipart-form-data/0.9.6/rocket_multipart_form_data/
    let mut options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("title"),
        MultipartFormDataField::file("file")
            .content_type_by_string(Some(mime::TEXT_PLAIN))
            .unwrap(),
    ]);

    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();
    let file = multipart_form_data.files.get("file");
    let title = multipart_form_data.texts.get("title");

    if let Some(unwrapped_file) = file {
        let file = &unwrapped_file[0];
        let content = fs::read_to_string(&file.path).unwrap();
        println!("{}", content);

        return "Ok";
    }
    "Not Ok"
}

/// Use Basic Auth header to trigger this.
/// Using cookies will resend the cookie.
#[post("/users/login")]
pub fn login(user: AuthenticatedUser, mut cookies: Cookies) -> Status {
    cookies.add_private(Cookie::new("user_id", user.user_id.to_string()));
    Status::Ok
}

/// Test authentication.
/// Will work with basic auth or cookie.
#[get("/users/test")]
pub fn auth_test(user: AuthenticatedUser) -> &'static str {
    "Hi!"
}

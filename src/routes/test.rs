use crate::utils::res_path;
use rocket::response::status::NotFound;
use rocket::response::NamedFile;

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

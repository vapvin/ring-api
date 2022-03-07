

#[macro_use]
extern crate rocket;


use rocket::form::Form;
use rocket::response::status::NotFound;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket_sync_db_pools::{database, diesel};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[database("ring_db")]
struct RingDBConn(diesel::PgConnection);

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("site/static/").join(file)).await.ok()
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1", routes![files])
        .attach(RingDBConn::fairing())
}


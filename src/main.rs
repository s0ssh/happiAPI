#[macro_use]
extern crate rocket;

pub mod auth;
pub mod profile;
pub mod routes;

use lazy_static::lazy_static;
use profile::*;
use rocket::tokio::sync::Mutex;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref PROFILE_CONFIGS: Mutex<HashMap<String, ProfileConfig>> = Mutex::new(HashMap::new());
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    for entry in glob::glob("./profiles/*.toml").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let profile_raw = fs::read_to_string(path).unwrap();
                let profile: ProfileConfig = toml::from_str(&profile_raw).unwrap();
                PROFILE_CONFIGS
                    .lock()
                    .await
                    .insert(profile.name.clone(), profile);
            }
            Err(e) => println!("Failed to get profile: {:?}", e),
        }
    }

    rocket::build()
        .mount("/", routes![routes::get_status])
        .mount("/v1/auth", routes![routes::v1::auth::get_auth_check])
        .mount(
            "/v1/responses",
            routes![
                routes::v1::responses::get_profile_config,
                routes::v1::responses::get_text_to_text
            ],
        )
}

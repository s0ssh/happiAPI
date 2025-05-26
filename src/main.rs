#[macro_use]
extern crate rocket;

pub mod auth;
pub mod routes;
pub mod profile;

use std::fs;
use profile::*;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    let mut profile_configs: Vec<ProfileConfig> = vec![];

    for entry in glob::glob("./profiles/*.toml").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let profile_raw = fs::read_to_string(path).unwrap();
                let profile: ProfileConfig = toml::from_str(&profile_raw).unwrap();
                println!("{:?}", profile);
                profile_configs.push(profile);
            },
            Err(e) => println!("Failed to get profile: {:?}", e),
        }
    }

    rocket::build()
        .mount("/", routes![routes::get_status])
        .mount("/v1/auth", routes![routes::v1::auth::get_auth_check])
}

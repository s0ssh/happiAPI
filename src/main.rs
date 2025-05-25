#[macro_use]
extern crate rocket;

pub mod auth;
pub mod routes;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    rocket::build()
        .mount("/", routes![routes::get_status])
        .mount("/v1/auth", routes![routes::v1::auth::get_auth_check])
}

#[macro_use] extern crate rocket;

#[get("/status")]
fn get_status() -> &'static str {
    "ok"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_status])
}

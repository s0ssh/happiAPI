pub mod v1;

#[get("/status")]
pub async fn get_status() -> &'static str {
    "ok"
}

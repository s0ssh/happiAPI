use crate::auth::ApiKey;

#[get("/check")]
pub async fn get_auth_check(api_key: ApiKey) -> &'static str {
    "ok"
}

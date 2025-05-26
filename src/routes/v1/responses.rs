use crate::auth::ApiKey;
use crate::profile::ProfileConfig;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap};
use rocket::serde::json::{Json, Value, json, serde_json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
struct DataTTT {
    pub prompt: String,
}

#[get("/<name>/config")]
pub async fn get_profile_config(name: String, api_key: ApiKey) -> Option<String> {
    match crate::PROFILE_CONFIGS.lock().await.get(&name) {
        Some(profile) => Some(format!("{:?}", profile)),
        None => None,
    }
}

#[post("/<name>/ttt", format = "application/json", data = "<data>")]
pub async fn get_text_to_text(
    name: String,
    data: Json<DataTTT>,
    api_key: ApiKey,
) -> Option<String> {
    let mut req_body: HashMap<&str, Value> = HashMap::new();
    let profile_configs = crate::PROFILE_CONFIGS.lock().await;
    let profile = profile_configs.get(&name).unwrap();
    req_body.insert("model", profile.openai.model_id.clone().into());

    let inp = format!(
        "[{{
                     \"role\": \"system\",
                     \"content\": [
                            {{
                                 \"type\": \"input_text\",
                                 \"text\": \"{}\"
                             }}
                         ]
                 }},
                 {{
                      \"role\": \"user\",
                      \"content\": [
                            {{
                                 \"type\": \"input_text\",
                                 \"text\": \"{}\"
                             }}
                        ]
                  }}]",
        profile.openai.system_prompt, data.prompt
    );

    req_body.insert("input", serde_json::from_str(&inp).unwrap());
    req_body.insert(
        "text",
        serde_json::from_str(
            format!(
                "{{\"format\": {{ \"type\": \"{}\" }}}}",
                profile.openai.model_format
            )
            .as_ref(),
        )
        .unwrap(),
    );
    req_body.insert("reasoning", json!({}));
    req_body.insert("tools", json!([]));
    req_body.insert("temperature", profile.openai.model_temperature.into());
    req_body.insert("max_output_tokens", profile.openai.model_tokens.into());
    req_body.insert("top_p", profile.openai.model_top_p.into());
    req_body.insert("store", Value::Bool(profile.openai.model_store));

    let mut req_hdrs = HeaderMap::new();
    req_hdrs.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    req_hdrs.insert(
        AUTHORIZATION,
        format!("Bearer {}", env::var("OPENAPI_API_KEY").unwrap())
            .parse()
            .unwrap(),
    );

    let res = reqwest::Client::new()
        .post("https://api.openai.com/v1/responses")
        .headers(req_hdrs)
        .json(&req_body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // TODO: Proper output format
    Some(format!("{res:?}"))
}

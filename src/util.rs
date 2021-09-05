use crate::{
    config::BotConfig,
    openai::{GPTRequest, GPTResponse},
};
use reqwest::header;

pub async fn get_response(
    client: reqwest::Client,
    prompt: String,
    config: &BotConfig,
) -> GPTResponse {
    let mut req_body = GPTRequest::new();
    req_body.set_prompt(prompt.replace(r#"""#, "\"").as_str());

    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", config.openai_key())
            .as_str()
            .parse()
            .unwrap(),
    );

    let res = client
        .post("https://api.openai.com/v1/engines/davinci/completions")
        .headers(headers)
        .body(serde_json::to_string(&req_body).unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    serde_json::from_str(res.replace("null", "\"null\"").as_str()).unwrap()
}

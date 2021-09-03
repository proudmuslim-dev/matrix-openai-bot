use crate::openai::GPTResponse;
use reqwest::header;
use std::env;

pub async fn get_response(client: reqwest::Client, prompt: String) -> GPTResponse {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", env::var("OPENAI_SK").unwrap())
            .as_str()
            .parse()
            .unwrap(),
    );
    let res = client
        .post("https://api.openai.com/v1/engines/davinci/completions")
        .headers(headers)
        .body("{ \"prompt\": \"Once upon\", \"max_tokens\": 50, \"temperature\": 0.6, \"presence_penalty\": 0.5, \"frequency_penalty\": 0.1}".replace("Once upon", prompt.as_str()))
        .send()
        .await.unwrap()
        .text()
        .await.unwrap();

    serde_json::from_str(res.replace("null", "\"null\"").as_str()).unwrap()
}

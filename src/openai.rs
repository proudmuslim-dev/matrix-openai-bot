use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GPTResponse {
    id: String,
    object: String,
    created: usize,
    model: String,
    choices: Vec<Choices>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choices {
    text: String,
    index: u8,
    logprobs: String,
    finish_reason: String,
}

impl GPTResponse {
    pub fn get_text(&self) -> &str {
        self.choices[0].text.as_str()
        // .replace("\\n", "\n",).replace("\\t", "\t").as_str()
    }
}

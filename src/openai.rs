use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GPTRequest {
    prompt: String,
    max_tokens: u16,
    temperature: u8,
    top_p: u8,
    n: u8,
    stream: bool,
    stop: String,
    echo: bool,
}

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

impl GPTRequest {
    pub fn new() -> Self {
        Self {
            prompt: String::new(),
            max_tokens: 16,
            temperature: 1,
            top_p: 1,
            n: 1,
            stream: false,
            stop: "\n".to_owned(),
            echo: false,
        }
    }

    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_owned();
    }

    pub fn set_tokens(&mut self, tokens: u16) {
        self.max_tokens = tokens
    }

    pub fn set_temperature(&mut self, temperature: u8) {
        self.temperature = temperature;
    }

    pub fn set_completions(&mut self, completions: u8) {
        self.n = completions;
    }

    pub fn set_echo(&mut self, echo: bool) {
        self.echo = echo;
    }
}

impl GPTResponse {
    pub fn get_text(&self) -> &str {
        self.choices[0].text.as_str()
    }
}

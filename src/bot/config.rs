use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Serialize, Deserialize)]
pub struct BotConfig {
    openai_key: String,
    homeserver: String,
    username: String,
    password: String,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            openai_key: "".to_owned(),
            homeserver: "https://matrix.org".to_owned(),
            username: "".to_owned(),
            password: "".to_owned(),
        }
    }
}

impl BotConfig {
    pub fn openai_key(&self) -> &str {
        &self.openai_key
    }
    pub fn homeserver(&self) -> &str {
        &self.homeserver
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn as_array(&self) -> [&str; 3] {
        [&self.username, &self.password, &self.openai_key]
    }
}

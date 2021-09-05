use matrix_sdk::{
    async_trait,
    room::Room,
    ruma::events::{
        room::message::{MessageEventContent, MessageType, TextMessageEventContent},
        AnyMessageEventContent, SyncMessageEvent,
    },
    Client, ClientConfig, EventHandler, SyncSettings,
};

use crate::{config::BotConfig, util};

use std::process::exit;
use url::Url;

pub struct OpenAIBot {
    config: BotConfig,
}

impl OpenAIBot {
    pub fn new(config: BotConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl EventHandler for OpenAIBot {
    async fn on_room_message(&self, room: Room, event: &SyncMessageEvent<MessageEventContent>) {
        if let Room::Joined(room) = room {
            let msg_body = if let SyncMessageEvent {
                content:
                    MessageEventContent {
                        msgtype: MessageType::Text(TextMessageEventContent { body: msg_body, .. }),
                        ..
                    },
                ..
            } = event
            {
                msg_body
            } else {
                return;
            };

            if msg_body.ends_with("--prompt") {
                let prompt_vec: Vec<&str> = msg_body.split("--prompt").collect();
                let prompt = prompt_vec[0].to_owned().clone();

                let client = reqwest::Client::new();
                let res = util::get_response(client, prompt, &self.config).await;

                let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(
                    res.get_text()
                        .replace("\\n", "\n")
                        .replace("\\t", "\t")
                        .as_str(),
                ));

                room.send(content, None).await.unwrap();

                println!("Message sent");
            }
        }
    }
}

pub async fn start() -> Result<(), matrix_sdk::Error> {
    let mut home = dirs::home_dir().expect("No home directory found!");
    home.push("matrix-openai-bot");

    let config_file: BotConfig = confy::load("matrix-openai-bot").unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        exit(1)
    });

    let client_config = ClientConfig::new().store_path(home);

    let homeserver_url =
        Url::parse(&config_file.homeserver()).expect("Couldn't parse homeserver URL.");

    let client = Client::new_with_config(homeserver_url, client_config).unwrap();

    client
        .login(
            &config_file.username(),
            &config_file.password(),
            None,
            Some("OpenAI Bot"),
        )
        .await?;

    println!("Logged in as: {}", &config_file.username());

    client.sync_once(SyncSettings::default()).await.unwrap();

    client
        .set_event_handler(Box::new(OpenAIBot::new(config_file)))
        .await;

    let settings = SyncSettings::default().token(client.sync_token().await.unwrap());
    client.sync(settings).await;

    Ok(())
}

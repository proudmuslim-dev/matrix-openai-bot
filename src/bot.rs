use matrix_sdk::{
    async_trait,
    room::Room,
    ruma::events::{
        room::message::{MessageEventContent, MessageType, TextMessageEventContent},
        AnyMessageEventContent, SyncMessageEvent,
    },
    Client, ClientConfig, EventHandler, SyncSettings,
};

use crate::util;

use url::Url;

pub struct OpenAIBot {}

impl OpenAIBot {
    pub fn new() -> Self {
        Self {}
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
                let res = util::get_response(client, prompt).await;

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

pub async fn login_and_sync(
    homeserver_url: String,
    username: String,
    password: String,
) -> Result<(), matrix_sdk::Error> {
    let mut home = dirs::home_dir().expect("No home directory found!");
    home.push("matrix-openai-bot");

    let client_config = ClientConfig::new().store_path(home);

    let homeserver_url = Url::parse(&homeserver_url).expect("Couldn't parse homeserver URL.");

    let client = Client::new_with_config(homeserver_url, client_config).unwrap();

    client
        .login(&username, &password, None, Some("OpenAI Bot"))
        .await?;

    println!("Logged in as: {}", username);

    client.sync_once(SyncSettings::default()).await.unwrap();

    client.set_event_handler(Box::new(OpenAIBot::new())).await;

    let settings = SyncSettings::default().token(client.sync_token().await.unwrap());
    client.sync(settings).await;

    Ok(())
}

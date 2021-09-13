use matrix_sdk::{
    room::Room,
    ruma::events::{
        room::message::{MessageEventContent, MessageType, TextMessageEventContent},
        AnyMessageEventContent, SyncMessageEvent,
    },
    Client, ClientConfig, SyncSettings,
};

use crate::{bot::BotConfig, utils};

use lazy_static::lazy_static;
use std::process::exit;
use url::Url;

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
    static ref CONFIG_FILE: BotConfig = confy::load("matrix-openai-bot").unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        exit(1)
    });
}

async fn on_room_message(event: SyncMessageEvent<MessageEventContent>, room: Room) {
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
            let prompt = prompt_vec[0].to_owned();

            let res = utils::get_response(&HTTP_CLIENT, prompt, &CONFIG_FILE).await;

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

pub async fn start() -> Result<(), matrix_sdk::Error> {
    let mut home = dirs::home_dir().expect("No home directory found!");
    home.push("matrix-openai-bot");

    for key in CONFIG_FILE.as_array() {
        if key.is_empty() {
            eprintln!("Please ensure you have filled out the config file completely. Your configuration file should be located under the 'matrix-openai-bot' folder inside your config directory.");
            exit(1)
        }
    }

    let client_config = ClientConfig::new().store_path(home);

    let homeserver_url =
        Url::parse(CONFIG_FILE.homeserver()).expect("Couldn't parse homeserver URL.");

    let client = Client::new_with_config(homeserver_url, client_config).unwrap();

    client
        .login(
            CONFIG_FILE.username(),
            CONFIG_FILE.password(),
            None,
            Some("OpenAI Bot"),
        )
        .await?;

    println!("Logged in as: {}", &CONFIG_FILE.username());

    client.sync_once(SyncSettings::default()).await.unwrap();

    client.register_event_handler(on_room_message).await;

    let settings = SyncSettings::default().token(client.sync_token().await.unwrap());
    client.sync(settings).await;

    Ok(())
}

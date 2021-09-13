use matrix_sdk::{Client, ClientConfig, SyncSettings};

use crate::bot::{events, BotConfig};

use lazy_static::lazy_static;
use std::process::exit;
use url::Url;

lazy_static! {
    pub static ref CONFIG_FILE: BotConfig =
        confy::load("matrix-openai-bot").unwrap_or_else(|error| {
            eprintln!("Error: {}", error);
            exit(1)
        });
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

    client.register_event_handler(events::on_room_message).await;

    let settings = SyncSettings::default().token(client.sync_token().await.unwrap());
    client.sync(settings).await;

    Ok(())
}

use matrix_sdk::{
    room::Room,
    ruma::events::{
        room::message::{MessageEventContent, MessageType, TextMessageEventContent},
        AnyMessageEventContent, SyncMessageEvent,
    },
};

use crate::{bot::CONFIG_FILE, utils};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn on_room_message(event: SyncMessageEvent<MessageEventContent>, room: Room) {
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

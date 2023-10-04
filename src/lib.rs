use discord_flows::{application_command_handler, message_handler};
use discord_flows::{
    model::{
        application::interaction::InteractionResponseType,
        prelude::application::interaction::application_command::ApplicationCommandInteraction,
        Message,
    },
    Bot, ProvidedBot,
};
use flowsnet_platform_sdk::logger;
use http_req::request;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use webhook_flows::{create_endpoint, request_handler, send_response};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    logger::init();
    // let token = std::env::var("DISCORD_TOKEN").unwrap();

    // let bot = ProvidedBot::new(token);
    // bot.listen_to_application_commands_from_channel(
    //     std::env::var("LISTENING_DISCORD_CHANNEL_ID")
    //         .unwrap()
    //         .parse()
    //         .unwrap(),
    // )
    // .await;
    // bot.listen_to_messages_from_channel(
    //     std::env::var("LISTENING_DISCORD_CHANNEL_ID")
    //         .unwrap()
    //         .parse()
    //         .unwrap(),
    // )
    // .await;

    create_endpoint().await;
}

#[message_handler]
pub async fn message_handler(msg: Message) {
    logger::init();
    let token = std::env::var("DISCORD_TOKEN").unwrap();

    let bot = ProvidedBot::new(token);
    let client = bot.get_client();
    let channel_id = msg.channel_id;
    let content = msg.content;

    if msg.author.bot {
        log::debug!("message from bot");
        return;
    }

    _ = client
        .send_message(
            channel_id.into(),
            &serde_json::json!({
                "content": content,
            }),
        )
        .await;
}

#[application_command_handler]
pub async fn application_command_handler(ac: ApplicationCommandInteraction) {
    logger::init();
    let token = std::env::var("DISCORD_TOKEN").unwrap();
    let bot = ProvidedBot::new(token);

    let client = bot.get_client();
    log::debug!("-------------------");

    let x = client
        .create_interaction_response(
            ac.id.into(),
            &ac.token,
            &serde_json::json!({
                "type": InteractionResponseType::DeferredChannelMessageWithSource as u8,
            }),
        )
        .await;
    log::debug!("{:?}", x);
    tokio::time::sleep(Duration::from_secs(3)).await;
    client.set_application_id(ac.application_id.into());
    _ = client
        .edit_original_interaction_response(
            &ac.token,
            &serde_json::json!({
                "content": "Pong"
            }),
        )
        .await;

    if let Ok(m) = client
        .create_followup_message(
            &ac.token,
            &serde_json::json!({
                "content": "PongPong"
            }),
        )
        .await
    {
        _ = client
            .edit_followup_message(
                &ac.token,
                m.id.into(),
                &serde_json::json!({
                    "content": "PongPongPong"
                }),
            )
            .await;
    }
}

#[request_handler]
async fn webhook_handler(
    _headers: Vec<(String, String)>,
    subpath: String,
    qry: HashMap<String, Value>,
    _body: Vec<u8>,
) {
    match qry.get("delay") {
        Some(d) => {
            let mut writer = Vec::new(); //container for body of a response
            let url = format!(
                "https://www.google.com",
                // "https://hub.dummyapis.com/delay?seconds={}",
                // d.as_str().unwrap_or("10").parse().unwrap_or(10)
            );
            match request::get(url, &mut writer) {
                Ok(res) => send_response(res.status_code().into(), vec![], writer),
                Err(e) => send_response(500, vec![], e.to_string().as_bytes().to_vec()),
            }
        }
        None => send_response(400, vec![], vec![]),
    }
}

use discord_flows::{get_client, listen_to_event, model::Message};
use flowsnet_platform_sdk::logger;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
    let token = std::env::var("DISCORD_TOKEN").unwrap();

    listen_to_event(token.clone(), move |msg| handle(msg, token)).await;
}

async fn handle(msg: Message, token: String) {
    let client = get_client(token);
    let channel_id = msg.channel_id;
    let content = msg.content;

    log::debug!("-------------");
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

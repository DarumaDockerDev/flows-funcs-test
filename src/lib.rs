use discord_flows::{get_client, listen_to_event, model::Message, Bot, ChannelId};
use flowsnet_platform_sdk::logger;

const CHANNEL_ID: u64 = 995948764542013493;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
    // let token = std::env::var("DISCORD_TOKEN").unwrap();

    listen_to_event(Bot::default_bot(ChannelId(CHANNEL_ID)), move |msg| {
        handle(msg)
    })
    .await;
}

async fn handle(msg: Message) {
    let client = get_client(Bot::default_bot(ChannelId(CHANNEL_ID)));
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

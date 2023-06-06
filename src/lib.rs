use discord_flows::{model::Message, Bot, DefaultBot};
use flowsnet_platform_sdk::logger;

const CHANNEL_ID: u64 = 1090160755522928640;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
    // let token = std::env::var("DISCORD_TOKEN").unwrap();

    let bot = DefaultBot {};
    bot.listen_to_channel(CHANNEL_ID, move |msg| handle(msg))
        .await;
}

async fn handle(msg: Message) {
    let bot = DefaultBot {};
    let client = bot.get_client();
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

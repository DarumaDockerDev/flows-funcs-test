use claude_flows::{chat, ClaudeFlows};
use discord_flows::{model::Message, Bot, ProvidedBot};
use flowsnet_platform_sdk::logger;

const CHANNEL_ID: u64 = 1090160755522928640;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
    let token = std::env::var("DISCORD_TOKEN").unwrap();

    let bot = ProvidedBot::new(token);
    bot.listen(|msg| handle(&bot, msg)).await;
}

async fn handle<B: Bot>(bot: &B, msg: Message) {
    let client = bot.get_client();
    let channel_id = msg.channel_id;
    let content = msg.content;

    if msg.author.bot {
        log::debug!("message from bot");
        return;
    }

    if let Some(history) = chat::chat_history(channel_id.as_u64().to_string().as_str(), 3) {
        log::debug!("chat history: {:?}", history);
    }

    let cf = ClaudeFlows::new();
    let co = chat::ChatOptions::default();

    match cf
        .chat_completion(channel_id.as_u64().to_string().as_str(), &content, &co)
        .await
    {
        Ok(c) => {
            _ = client
                .send_message(
                    channel_id.into(),
                    &serde_json::json!({
                        "content": c,
                    }),
                )
                .await;
        }
        Err(e) => {
            _ = client
                .send_message(
                    channel_id.into(),
                    &serde_json::json!({
                        "content": e,
                    }),
                )
                .await;
        }
    }
}

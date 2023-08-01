use discord_flows::{
    model::application::interaction::InteractionResponseType, Bot, EventModel, ProvidedBot,
};
use flowsnet_platform_sdk::logger;

const CHANNEL_ID: u64 = 1090160755522928640;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
    let token = std::env::var("DISCORD_TOKEN").unwrap();

    let bot = ProvidedBot::new(token);
    bot.listen(|em| handle(&bot, em)).await;
}

async fn handle<B: Bot>(bot: &B, em: EventModel) {
    match em {
        EventModel::ApplicationCommand(ac) => {
            let client = bot.get_client();

            _ = client
                .create_interaction_response(
                    ac.id.into(),
                    &ac.token,
                    &serde_json::json!({
                        "type": InteractionResponseType::ChannelMessageWithSource as u8,
                        "data": {
                            "content": "Pong"
                        }
                    }),
                )
                .await;
        }
        EventModel::Message(msg) => {
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
    }
}

use discord_flows::{
    model::application::interaction::InteractionResponseType, Bot, DefaultBot, EventModel,
};
use flowsnet_platform_sdk::logger;
use std::time::Duration;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
    let token = std::env::var("DISCORD_TOKEN").unwrap();

    let bot = DefaultBot {};
    bot.listen_to_channel(
        std::env::var("LISTENING_DISCORD_CHANNEL_ID")
            .unwrap()
            .parse()
            .unwrap(),
        |em| handle(&bot, em),
    )
    .await;
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
                        "type": InteractionResponseType::DeferredChannelMessageWithSource as u8,
                    }),
                )
                .await;
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

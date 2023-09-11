use flowsnet_platform_sdk::logger;
use openai_flows::{chat, OpenAIFlows};
use tg_flows::{listen_to_update, update_handler, Telegram, UpdateKind};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    let telegram_token = std::env::var("telegram_token").unwrap();
    listen_to_update(telegram_token).await;
}

#[update_handler]
async fn handler(update: tg_flows::Update) {
    logger::init();
    let telegram_token = std::env::var("telegram_token").unwrap();
    let tele = Telegram::new(telegram_token);

    if let UpdateKind::Message(msg) = update.kind {
        let text = msg.text().unwrap_or("");
        let chat_id = msg.chat.id;

        let of = OpenAIFlows::new();
        let mut co = chat::ChatOptions {
            max_tokens: Some(50),
            post_prompt: Some("Do not give me any information about procedures and service features that are not mentioned in the PROVIDED CONTEXT."),
            ..chat::ChatOptions::default()
        };
        log::debug!("chat id: {}", chat_id);

        if text == "/restart" {
            co.restart = true;
        }

        if let Some(history) = chat::chat_history(&chat_id.to_string(), 10) {
            log::debug!("chat history: {:?}", history);
        }

        log::debug!("Received msg {} @{}", text, chat_id);

        let c = of.chat_completion(&chat_id.to_string(), &text, &co).await;

        if let Ok(c) = c {
            if c.restarted {
                _ = tele.send_message(chat_id, "Let's start a new conversation!");
            }

            // _ = tele.edit_message_text(chat_id, m.id, c.choice);
            _ = tele.send_message(chat_id, c.choice);
        } else {
            _ = tele.send_message(chat_id, "I have no choice");
        }
    }
}

use openai_flows::{chat, OpenAIFlows};
use tg_flows::{listen_to_update, Telegram, UpdateKind};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let telegram_token = std::env::var("telegram_token").unwrap();
    let tele = Telegram::new(telegram_token.clone());

    listen_to_update(telegram_token, |update| handler(update, &tele));
}

async fn handler(update: tg_flows::Update, tele: &Telegram) {
    if let UpdateKind::Message(msg) = update.kind {
        let text = msg.text().unwrap_or("");
        let chat_id = msg.chat.id;

        let of = OpenAIFlows::new();
        let co = chat::ChatOptions::default();

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

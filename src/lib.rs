use openai_flows::{chat_completion, ChatModel, ChatOptions};
use tg_flows::{listen_to_update, Telegram, UpdateKind};

#[no_mangle]
pub fn run() {
    let openai_key_name = "Michael";

    let telegram_token = std::env::var("telegram_token").unwrap();
    let tele = Telegram::new(telegram_token.clone());

    listen_to_update(telegram_token, |update| {
        if let UpdateKind::Message(msg) = update.kind {
            let text = msg.text().unwrap_or("");
            let chat_id = msg.chat.id;

            let c = chat_completion(
                &openai_key_name,
                &chat_id.to_string(),
                &text,
                &ChatOptions {
                    model: ChatModel::GPT35Turbo,
                    restart: false,
                    system_prompt: Some("You are awesome!"),
                    retry_times: 2,
                },
            );

            if let Some(c) = c {
                if c.restarted {
                    _ = tele.send_message(chat_id, "Let's start a new conversation!");
                }

                // _ = tele.edit_message_text(chat_id, m.id, c.choice);
                _ = tele.send_message(chat_id, c.choice);
            } else {
                _ = tele.send_message(chat_id, "I have no choice");
            }
        }
    });
}

use openai_flows::{chat_completion, ChatOptions};
use slack_flows::{listen_to_channel, send_message_to_channel};

#[no_mangle]
pub fn run() {
    listen_to_channel("family-wangshi", "chat", |sm| {
        let co = ChatOptions {
            restart: sm.text.eq_ignore_ascii_case("/restart"),
            restarted_sentence: Some("let's change the subject"),
        };
        if let Some(r) = chat_completion("Michael", "any_conversation", &sm.text, &co) {
            send_message_to_channel("family-wangshi", "chat", r.choice);
        }
    });
}

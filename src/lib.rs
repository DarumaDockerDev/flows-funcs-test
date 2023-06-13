use openai_flows::{chat_completion, ChatOptions};
use slack_flows::{listen_to_channel, send_message_to_channel};

#[no_mangle]
pub fn run() {
    listen_to_channel("family-wangshi", "chat", |sm| {
        send_message_to_channel("family-wangshi", "chat", "---".to_string());
    });
}

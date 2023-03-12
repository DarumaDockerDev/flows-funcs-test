use flowsnet_platform_sdk::write_error_log;
use openai_flows::chat_completion;
use slack_flows::{listen_to_channel, send_message_to_channel, SlackMessage};

#[no_mangle]
pub fn run() {
    listen_to_channel("family-wangshi", "chat", handler);
}

fn handler(sm: SlackMessage) {
    if let Some(r) = chat_completion("Michael", "random", &sm.text) {
        send_message_to_channel("family-wangshi", "chat", r.choice);
    }
}

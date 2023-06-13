use slack_flows::{listen_to_channel, send_message_to_channel};

#[no_mangle]
pub fn run() {
    listen_to_channel("family-wangshi", "chat", |sm| handler());
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
async fn handler() {
    send_message_to_channel("family-wangshi", "chat", "---".to_string());
}

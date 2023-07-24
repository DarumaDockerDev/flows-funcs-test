use slack_flows::{listen_to_channel, send_message_to_channel, SlackMessage};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let workspace = std::env::var("SLACK_WORKSPACE").unwrap();
    let channel = std::env::var("SLACK_CHANNEL").unwrap();
    listen_to_channel(workspace.as_str(), channel.as_str(), |sm| {
        handler(workspace.as_str(), channel.as_str(), sm)
    })
    .await;
}

async fn handler(workspace: &str, channel: &str, sm: SlackMessage) {
    send_message_to_channel(workspace, channel, sm.text).await;
}

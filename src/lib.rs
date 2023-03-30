use discord_flows::{create_text_message_in_channel, listen_to_channel, TextMessage};

#[no_mangle]
pub fn run() {
    listen_to_channel("ServerDev", "general", handler);
}

fn handler(tm: TextMessage) {
    if !tm.author.bot {
        create_text_message_in_channel("ServerDev", "general", tm.content);
    }
}

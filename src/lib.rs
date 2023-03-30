use discord_flows::{create_text_message_in_dm, listen_to_dm, TextMessage};

#[no_mangle]
pub fn run() {
    listen_to_dm("DarumaDockerDev", handler);
}

fn handler(tm: TextMessage) {
    if !tm.author.bot {
        create_text_message_in_dm("DarumaDockerDev", tm.content, Some(tm.id));
    }
}

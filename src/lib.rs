use discord_flows::{create_text_message_in_dm, listen_to_dm, TextMessage};

extern "C" {
    fn set_log(p: *const u8, len: i32);
}

#[no_mangle]
pub fn run() {
    listen_to_dm("DarumaDockerDev", handler);
}

fn handler(tm: TextMessage) {
    let log = b"abc".to_vec();
    unsafe {
        set_log(log.as_ptr(), log.len() as i32);
    }

    if !tm.author.bot {
        create_text_message_in_dm("DarumaDockerDev", tm.content, Some(tm.id))
    }
}

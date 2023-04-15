use discord_flows::{create_text_message_in_dm, listen_to_dm, TextMessage};
use openai_flows::{create_image_default_key, ImageRequest, ImageSize};

extern "C" {
    fn set_log(p: *const u8, len: i32);
}

#[no_mangle]
pub fn run() {
    listen_to_dm("DarumaDockerDev", handler);
}

fn handler(tm: TextMessage) {
    let log = "abc";
    unsafe {
        set_log(log.as_ptr(), log.len() as i32);
    }

    if !tm.author.bot {
        let ir = ImageRequest {
            prompt: tm.content,
            n: 2,
            size: ImageSize::S256,
            retry_times: 2,
        };
        let r = create_image_default_key(ir);

        match r.len() {
            0 => {
                create_text_message_in_dm("DarumaDockerDev", String::from("No images"), Some(tm.id))
            }
            _ => create_text_message_in_dm("DarumaDockerDev", r[0].clone(), Some(tm.id)),
        }
    }
}

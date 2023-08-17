use notion_flows::{listen_to_event, notion::models::Page};
use std::env;
use tg_flows::{ChatId, Telegram};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let database = env::var("notion_database").unwrap();
    let token = env::var("telegram_token").unwrap();
    let chat_id = env::var("telegram_chat_id").unwrap();

    let chat_id = ChatId(chat_id.parse().unwrap());
    let tele = Telegram::new(token);

    let send = |msg: String| {
        tele.send_message(chat_id, msg).ok();
    };

    listen_to_event(database, |page| async { handler(page, send).await }).await;
}

async fn handler<F>(page: Page, send: F)
where
    F: Fn(String),
{
    let title = page.title().unwrap_or("<untitled>".to_string());
    let _pros: String = page
        .properties
        .properties
        .iter()
        .map(|(k, v)| format!("- {k}: {v:?}"))
        .collect();

    let action = if page.created_time.eq(&page.last_edited_time) {
        "created"
    } else {
        "modified"
    };

    let msg = format!("Page named '{title}' has been {action}");
    send(msg);
}

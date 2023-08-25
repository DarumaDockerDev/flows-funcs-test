use flowsnet_platform_sdk::logger;
use notion_flows::{database_update_handler, listen_to_database_update, notion::models::Page};
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let database = env::var("notion_database").unwrap();

    listen_to_database_update(database).await;
}

#[database_update_handler]
async fn handler(page: Page) {
    logger::init();

    let title = page.title().unwrap_or("<untitled>".to_string());
    let _pros: String = page
        .properties
        .properties
        .iter()
        .map(|(k, v)| format!("- {}: {:?}", k, v))
        .collect();

    let action = if page.created_time.eq(&page.last_edited_time) {
        "created"
    } else {
        "modified"
    };

    let msg = format!("Page named '{}' has been {}", title, action);
    log::info!("{}", msg);
}

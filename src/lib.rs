use flowsnet_platform_sdk::write_error_log;
use github_flows::{get_octo, listen_to_event, octocrab::models::events::payload::EventPayload};
use openai_flows::{chat_completion, ChatModel, ChatOptions};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    listen_to_event(
        "DarumaDockerDev",
        "DarumaDockerDev",
        "github-func-test",
        vec!["issue_comment"],
        handler,
    )
    .await;
}

async fn handler(payload: EventPayload) {
    let octo = get_octo(Some(String::from("DarumaDockerDev")));
    let issues = octo.issues("DarumaDockerDev", "github-func-test");

    match payload {
        EventPayload::IssueCommentEvent(e) => {
            let last_comment_id = store_flows::get("last_created_comment").unwrap_or_default();
            if e.comment.id.into_inner() != last_comment_id.as_u64().unwrap_or_default() {
                if let Some(b) = e.comment.body {
                    let co = ChatOptions {
                        model: ChatModel::GPT35Turbo,
                        restart: b.eq_ignore_ascii_case("/restart"),
                        restarted_sentence: Some("let's restart a talk"),
                    };
                    if let Some(r) = chat_completion(
                        "DarumaDocker",
                        &format!("issue#{}", e.issue.number),
                        &b,
                        &co,
                    ) {
                        write_error_log!(r.restarted);
                        match issues.create_comment(e.issue.number, r.choice).await {
                            Ok(comment) => {
                                store_flows::set(
                                    "last_created_comment",
                                    serde_json::to_value(comment.id.into_inner()).unwrap(),
                                );
                            }
                            Err(e) => {
                                write_error_log!(e.to_string());
                            }
                        }
                    }
                }
            }
        }
        _ => (),
    };
}

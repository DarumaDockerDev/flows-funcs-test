use github_flows::{
    get_octo, listen_to_event,
    octocrab::models::{events::payload::EventPayload, reactions::ReactionContent},
};
use slack_flows::send_message_to_channel;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    listen_to_event(
        "DarumaDockerOrg",
        "github-func-test",
        vec!["issue_comment", "issues"],
        handler,
    )
    .await;
}

async fn handler(payload: EventPayload) {
    let octo = get_octo(Some(String::from("DarumaDockerOrg")));
    let issues = octo.issues("DarumaDockerOrg", "github-func-test");

    let reaction = match payload {
        EventPayload::IssuesEvent(e) => {
            let issue_id = e.issue.number;
            Some(
                issues
                    .create_reaction(issue_id, ReactionContent::Rocket)
                    .await,
            )
        }
        EventPayload::IssueCommentEvent(e) => {
            let comment_id = e.comment.id.0;
            Some(
                issues
                    .create_comment_reaction(comment_id, ReactionContent::Rocket)
                    .await,
            )
        }
        _ => None,
    };

    if let Some(re) = reaction {
        match re {
            Ok(c) => send_message_to_channel("reactor-space", "t1", c.created_at.to_rfc2822()),
            Err(e) => send_message_to_channel("reactor-space", "t1", e.to_string()),
        }
    }
}

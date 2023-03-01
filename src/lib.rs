use github_flows::{
    get_octo, listen_to_event,
    octocrab::models::{events::payload::EventPayload, reactions::ReactionContent},
};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    listen_to_event(
        "DarumaDockerOrg",
        "github-func-test",
        vec!["issue_comment"],
        handler,
    )
    .await;
}

async fn handler(payload: EventPayload) {
    if let EventPayload::IssueCommentEvent(e) = payload {
        let issue_number = e.comment.id.0;

        // installed app login
        let octo = get_octo(Some(String::from("DarumaDockerOrg")));

        let _reaction = octo
            .issues("DarumaDockerOrg", "github-func-test")
            .create_comment_reaction(issue_number, ReactionContent::Rocket)
            .await
            .unwrap();
    };
}

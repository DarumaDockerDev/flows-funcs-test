use github_flows::{
    get_octo, listen_to_event,
    octocrab::models::{events::payload::EventPayload, reactions::ReactionContent},
    GithubLogin,
};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    listen_to_event(
        &GithubLogin::Default,
        "DarumaDockerDev",
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
        let octo = get_octo(&GithubLogin::Default);

        let _reaction = octo
            .issues("DarumaDockerDev", "github-func-test")
            .create_reaction(issue_number, ReactionContent::Rocket)
            .await
            .unwrap();
    };
}

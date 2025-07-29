#![cfg(test)]

use crate::common::{actions, harness::run_test_harness};
use http::StatusCode;

mod common;

#[tokio::test]
async fn test_auth_request_link() {
    let harness = run_test_harness().await;
    actions::get_and_validate_csrf_token(&harness, None).await;
}

#[tokio::test]
async fn test_gh_contribution_flow() {
    let harness = run_test_harness().await;
    let http_client = reqwest::Client::new();

    let (user, session_id) =
        actions::create_and_login_gh_user(&harness, &http_client, "alice".into()).await;

    let mut contribution = actions::try_contribute(&harness, &http_client, &session_id).await;

    let entropy = actions::entropy_from_str("deterministic entropy for tests");
    contribution
        .add_entropy::<kzg_ceremony_sequencer::Engine>(&entropy, &user.identity())
        .expect("entropy should be added");

    actions::contribute_successfully(
        &harness,
        &http_client,
        &session_id,
        &contribution,
        &user.identity().to_string(),
    )
    .await;

    let transcript = harness.read_transcript_file().await;
    actions::assert_includes_contribution(&transcript, &contribution, &user, false, true);
}

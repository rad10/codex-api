use codex_api_lib::codex::{
    ResponsesOptions,
    r#async::wasm_safe::{models_response, responses_response},
};
use codex_api_types::{
    codex::{ModelsResponse, ResponsesApiRequest},
    response_item::{ContentItem, ResponseItem},
};

use crate::create_client;

/// Checks if the model data that OpenAI provides is still accurate
#[tokio::test]
#[ignore = "Can only confirm if codex credentials are supplied. This needs to be manual"]
pub async fn validate_models_response() {
    // Creating client
    let client = create_client();

    let models_handle = models_response(&client);

    // Checking that response worked as intended
    let models_response = models_handle
        .await
        .expect("models should have returned successfully");

    // Checking portions of response for validity
    assert!(
        !models_response.status().is_client_error(),
        "Response didnt succeed using valid parameters"
    );

    // Checking that body is valid JSON
    let body_text = models_response
        .to_response()
        .bytes()
        .await
        .expect("data should be valid utf8");

    let body_json: ModelsResponse =
        serde_json::from_slice(&body_text).expect("body should be json to begin with");

    assert!(
        !body_json.models.is_empty(),
        "models response should return valid data"
    );
}

/// Checks if the model data that OpenAI provides is still accurate
#[tokio::test]
#[ignore = "Can only confirm if codex credentials are supplied. This needs to be manual"]
pub async fn validate_responses_response() {
    // Creating client
    let client = create_client();

    let request = ResponsesApiRequest {
        model: "gpt-5.5".to_string(),
        input: vec![ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText {
                text: "say hello world".to_string(),
            }],
            phase: None,
            internal_chat_message_metadata_passthrough: None,
        }],
        ..Default::default()
    };

    let options = ResponsesOptions::default();

    let responses_handle = responses_response(&client, request, options);

    // Checking that response worked as intended
    let responses_response = responses_handle
        .await
        .expect("models should have returned successfully");

    // Checking portions of response for validity
    assert!(
        !responses_response.status().is_client_error(),
        "Response didnt succeed using valid parameters"
    );

    // Checking that body is valid JSON
    let body_text = responses_response
        .to_response()
        .bytes()
        .await
        .expect("data should be valid utf8");

    let body_json: ModelsResponse =
        serde_json::from_slice(&body_text).expect("body should be json to begin with");

    assert!(
        !body_json.models.is_empty(),
        "models response should return valid data"
    );
}

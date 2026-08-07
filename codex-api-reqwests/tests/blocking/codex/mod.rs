#[cfg(all(feature = "sync", not(feature = "async")))]
use std::io::BufRead;

#[cfg(all(feature = "sync", not(feature = "async")))]
use codex_api_lib::codex::{ResponsesOptions, sync::{models_response, responses_response}};
#[cfg(all(feature = "sync", not(feature = "async")))]
use codex_api_reqwests::blocking::codex::{models_request, responses_request};
#[cfg(all(feature = "sync", not(feature = "async")))]
use codex_api_types::{codex::{ModelsResponse, ResponsesApiRequest, response_stream_event::ResponsesStreamEvent}, response_item::{ContentItem, ResponseItem}};

#[cfg(all(feature = "sync", not(feature = "async")))]
use crate::create_blocking_client;

#[test]
#[cfg(all(feature = "sync", not(feature = "async")))]
pub fn validate_models_response() {
    // Creating client
    let client = create_blocking_client();

    eprintln!("client: {client:?}");

    let model_request = models_request(&client).expect("failed to create request data");

    eprintln!("request: {model_request:?}");

    // Checking that response worked as intended
    let models_response =
        models_response(&client).expect("models should have returned successfully");

    // Checking portions of response for validity
    eprintln!("response status: {}", models_response.status());
    assert!(
        !models_response.status().is_client_error(),
        "Response didnt succeed using valid parameters. Message {:?}",
        models_response.to_response().text()
    );

    // Checking that body is valid JSON
    let body_text = models_response
        .to_response()
        .bytes()
        .expect("data should be valid utf8");

    let body_json: ModelsResponse =
        serde_json::from_slice(&body_text).expect("body should be json to begin with");

    assert!(
        !body_json.models.is_empty(),
        "models response should return valid data"
    );
}

#[test]
#[cfg(all(feature = "sync", not(feature = "async")))]
pub fn validate_responses_response() {
    // Creating client
    let client = create_blocking_client();

    eprintln!("client: {client:?}");

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

    let responses_request = responses_request(&client, &request, ResponsesOptions::default())
        .expect("failed to create request data");

    eprintln!("request: {responses_request:?}");

    // Checking that response worked as intended
    let responses_response = responses_response(&client, request, options)
        .expect("models should have returned successfully");

    // Checking portions of response for validity
    assert!(
        !responses_response.status().is_client_error(),
        "Response didnt succeed using valid parameters"
    );

    // Checking portions of response for validity
    eprintln!("response status: {}", responses_response.status());
    assert!(
        !responses_response.status().is_client_error(),
        "Response didnt succeed using valid parameters"
    );

    // Checking that body is valid JSON
    let body_text = responses_response
        .to_response()
        .bytes()
        .expect("data should be valid utf8");

    eprintln!("body: {body_text:?}");

    let data_lines = body_text
        .as_ref()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| line.strip_prefix("data: ").map(serde_json::from_str))
        .collect::<Result<Vec<ResponsesStreamEvent>, _>>()
        .expect("data didnt convert cleanly");

    assert!(!data_lines.is_empty(), "response should return valid data");
}

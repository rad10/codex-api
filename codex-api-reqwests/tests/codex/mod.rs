#[cfg(feature = "async")]
use std::io::BufRead;

#[cfg(feature = "async")]
use codex_api_lib::codex::ResponsesOptions;
#[cfg(feature = "async")]
use codex_api_reqwests::codex::{models_request, responses_request};
#[cfg(feature = "async")]
use codex_api_types::codex::response_stream_event::ResponsesStreamEvent;
use codex_api_types::{
    codex::{ModelsResponse, ResponsesApiRequest},
    response_item::{ContentItem, ResponseItem},
};

#[cfg(feature = "async")]
use crate::create_client;

/// Checks if the model data that OpenAI provides is still accurate
#[tokio::test]
#[ignore = "Can only confirm if codex credentials are supplied. This needs to be manual"]
#[cfg(feature = "async")]
pub async fn validate_models_response() {
    // Creating client
    let client = create_client();

    eprintln!("client: {client:?}");

    let model_request = models_request(&client).expect("failed to create request data");

    eprintln!("request: {model_request:?}");

    cfg_select! {
        feature = "threaded" => {
            let models_handle = codex_api_lib::codex::r#async::wasm_safe::models_response(&client);
        }
        _ => {
            let models_handle = codex_api_lib::codex::r#async::models_response(&client);
        }
    }

    // Checking that response worked as intended
    let models_response = models_handle
        .await
        .expect("models should have returned successfully");

    // Checking portions of response for validity
    eprintln!("response status: {}", models_response.status());
    assert!(
        !models_response.status().is_client_error(),
        "Response didnt succeed using valid parameters. Message {:?}",
        models_response.to_response().text().await
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
#[cfg(feature = "async")]
pub async fn validate_responses_response() {
    // Creating client
    let client = create_client();

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

    cfg_select! {
        feature = "threaded" => {
            let responses_handle = codex_api_lib::codex::r#async::wasm_safe::responses_response(&client, request, options);
        }
        _ => {
            let responses_handle = codex_api_lib::codex::r#async::responses_response(&client, request, options);
        }
    }

    // Checking that response worked as intended
    let responses_response = responses_handle
        .await
        .expect("models should have returned successfully");

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
        .await
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

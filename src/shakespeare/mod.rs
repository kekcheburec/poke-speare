use crate::ps_error::{PSError, PSResult};
use log::error;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

const SHAKESPEARE_API_PATH: &str = "/translate/shakespeare.json";

#[derive(Deserialize, Debug)]
struct Translation {
    success: Success,
    contents: Contents,
}

#[derive(Deserialize, Debug)]
struct Contents {
    translated: String,
}

#[derive(Deserialize, Debug)]
struct Success {
    total: u32,
}

impl Translation {
    fn translated_text(&self) -> PSResult<String> {
        match self.success.total {
            1 => Ok(self.contents.translated.clone()),
            _ => Err(PSError::ShakespeareError),
        }
    }
}

fn request_url(server_uri: &str) -> String {
    format!("{}{}", server_uri, SHAKESPEARE_API_PATH)
}

pub async fn translate(text: &str) -> PSResult<String> {
    let request_url = request_url("https://api.funtranslations.com");
    retrieve_translation(&request_url, text).await
}

async fn retrieve_translation(request_url: &str, text: &str) -> PSResult<String> {
    let mut json_param = HashMap::new();
    json_param.insert("text", text);
    let response = Client::new()
        .post(request_url)
        .json(&json_param)
        .send()
        .await
        .map_err(|e| {
            error!("while making shakespeare request: {}", e);
            PSError::ShakespeareError
        })?;
    let status = response.status();

    match status.as_u16() {
        200 => {
            let translation: Translation = response.json().await.map_err(|e| {
                error!("while interpreting shakespeare json payload: {}", e);
                PSError::ShakespeareError
            })?;
            translation.translated_text()
        }
        429 => Err(PSError::QuotaError),
        _ => {
            error!(
                "shakespeare response: unexpected status code. request_url: {}, text: {}, response: {:#?}",
                request_url, text, response
            );
            Err(PSError::ShakespeareError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{
        matchers::{body_json, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn shakespeare_translation_is_correctly_retrieved() {
        let mock_server = MockServer::start().await;

        let shakespeare_response = json!({
            "success": {
                "total": 1
            },
            "contents": {
                "translated": "Thee did giveth mr. Tim a hearty meal,  but unfortunately what he did doth englut did maketh him kicketh the bucket.",
                "text": "You gave Mr. Tim a hearty meal, but unfortunately what he ate made him die.",
                "translation": "shakespeare"
            }
        });

        let input_text =
            "You gave Mr. Tim a hearty meal, but unfortunately what he ate made him die.";

        let expected_json_body = body_json(json!({ "text": input_text }));
        let response = ResponseTemplate::new(200).set_body_json(shakespeare_response);

        Mock::given(method("POST"))
            .and(path(SHAKESPEARE_API_PATH))
            .and(expected_json_body)
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let expected_translation = "Thee did giveth mr. Tim a hearty meal,  but unfortunately what he did doth englut did maketh him kicketh the bucket.";
        let request_url = request_url(&mock_server.uri());
        dbg!(&request_url);
        let actual_translation = retrieve_translation(&request_url, input_text)
            .await
            .unwrap();
        assert_eq!(expected_translation, actual_translation);
    }
}
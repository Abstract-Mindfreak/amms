use crate::core::{
    error::{Error, Result},
    types::GeometricTaskCommand,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

const MISTRAL_ENDPOINT: &str = "https://api.mistral.ai/v1/chat/completions";

#[derive(Clone)]
pub struct LlmGateway {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

impl LlmGateway {
    pub fn new(api_key: Option<String>) -> Result<Self> {
        let key = api_key
            .or_else(|| env::var("MISTRAL_API_KEY").ok())
            .ok_or_else(|| Error::LlmCommunication("Missing MISTRAL_API_KEY".into()))?;

        Ok(Self {
            client: reqwest::Client::new(),
            api_key: key,
            model: env::var("MISTRAL_MODEL").unwrap_or_else(|_| "mistral-small-latest".into()),
        })
    }

    pub async fn submit_geometric_query(
        &self,
        query: &str,
        context: &Value,
    ) -> Result<GeometricTaskCommand> {
        let payload = LlmRequest {
            model: self.model.clone(),
            response_format: ResponseFormat {
                r#type: "json_object".into(),
            },
            messages: vec![
                Message {
                    role: "system".into(),
                    content: SYSTEM_PROMPT.into(),
                },
                Message {
                    role: "user".into(),
                    content: format!("Context: {}\n\nQuery: {}", context, query),
                },
            ],
        };

        let response = self
            .client
            .post(MISTRAL_ENDPOINT)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|err| Error::LlmCommunication(format!("HTTP error: {err}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::LlmCommunication(format!(
                "Mistral API error {status}: {body}"
            )));
        }

        let body: LlmResponse = response
            .json()
            .await
            .map_err(|err| Error::LlmCommunication(format!("Failed to parse response: {err}")))?;

        let content = body
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .ok_or_else(|| Error::LlmCommunication("Empty response from Mistral".into()))?;

        let mut raw: Value = serde_json::from_str(&content).map_err(Error::Serialization)?;
        normalize_geometric_operator(&mut raw);
        serde_json::from_value(raw).map_err(Error::Serialization)
    }
}

const SYSTEM_PROMPT: &str = "You are the MMSS Pure Logic agent. Respond strictly with JSON in the GeometricTaskCommand schema (task_name, geometric_operator, target_module, parameters, expected_output_metric, optional task_id).";

#[derive(Debug, Serialize)]
struct LlmRequest {
    model: String,
    messages: Vec<Message>,
    response_format: ResponseFormat,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Debug, Deserialize)]
struct LlmResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChoiceMessage {
    content: Option<String>,
}

fn normalize_geometric_operator(payload: &mut Value) {
    if let Some(operator_value) = payload.get_mut("geometric_operator") {
        if let Some(raw_text) = operator_value.as_str() {
            let normalized = map_llm_response_to_operator(raw_text);
            *operator_value = Value::String(normalized.to_string());
        }
    }
}

fn map_llm_response_to_operator(raw: &str) -> &'static str {
    let lowered = raw.trim().to_lowercase();

    if lowered.contains("zitter") || lowered.contains("oscillation") {
        "Zitterbewegung"
    } else if lowered.contains("stabilize") || lowered.contains("derivation") {
        "GeometricDerivation"
    } else if lowered.contains("semantic") || lowered.contains("anchor") {
        "SemanticSynthesis"
    } else if lowered.contains("coherence")
        || lowered.contains("optimize")
        || lowered.contains("quaternion")
    {
        "QuaternionRotation"
    } else if matches!(
        lowered.as_str(),
        "quaternionrotation" |
            "zitterbewegung" |
            "geometricderivation" |
            "semanticsynthesis"
    ) {
        match lowered.as_str() {
            "quaternionrotation" => "QuaternionRotation",
            "zitterbewegung" => "Zitterbewegung",
            "geometricderivation" => "GeometricDerivation",
            "semanticsynthesis" => "SemanticSynthesis",
            _ => "QuaternionRotation",
        }
    } else {
        "QuaternionRotation"
    }
}

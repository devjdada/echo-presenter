use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AiError {
    #[error("Provider error: {0}")]
    Provider(String),
    #[error("Action '{0}' requires operator approval")]
    ApprovalRequired(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionSafety {
    Unrestricted,
    RequiresApproval,
}

pub fn check_action_safety(action: &str) -> ActionSafety {
    match action {
        "go_live" | "clear" | "blackout" | "switch_output" | "delete_content" => {
            ActionSafety::RequiresApproval
        }
        _ => ActionSafety::Unrestricted,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRequest {
    pub model: String,
    pub system_prompt: String,
    pub user_prompt: String,
}

pub async fn send_openrouter_chat(
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String, AiError> {
    if api_key.trim().is_empty() {
        return Err(AiError::Provider("OpenRouter API key is not configured".into()));
    }
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": model,
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": user_prompt }
        ]
    });
    let resp = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| AiError::Provider(e.to_string()))?;
    let val: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| AiError::Provider(e.to_string()))?;
    val["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| AiError::Provider(format!("Unexpected OpenRouter response: {val}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_safety_gate() {
        assert_eq!(check_action_safety("create_slides"), ActionSafety::Unrestricted);
        assert_eq!(check_action_safety("go_live"), ActionSafety::RequiresApproval);
        assert_eq!(check_action_safety("blackout"), ActionSafety::RequiresApproval);
    }
}

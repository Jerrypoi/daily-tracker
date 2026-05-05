use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateApiKeyRequest {
    pub name: String,
}

/// Returned only at creation time — the plaintext `token` is never readable
/// again, so the caller must store it.
#[derive(Serialize, Deserialize, Clone)]
pub struct CreateApiKeyResponse {
    pub id: i64,
    pub name: String,
    pub key_prefix: String,
    pub token: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiKeyResponse {
    pub id: i64,
    pub name: String,
    pub key_prefix: String,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_api_key_request_deserializes() {
        let json = r#"{"name":"ci-bot"}"#;
        let req: CreateApiKeyRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "ci-bot");
    }

    #[test]
    fn create_api_key_request_rejects_missing_name() {
        let json = r#"{}"#;
        let res: Result<CreateApiKeyRequest, _> = serde_json::from_str(json);
        assert!(res.is_err());
    }

    #[test]
    fn create_api_key_response_serializes() {
        let resp = CreateApiKeyResponse {
            id: 7,
            name: "ci-bot".to_string(),
            key_prefix: "dt_a1b2c3d4".to_string(),
            token: "dt_a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6".to_string(),
            created_at: "2026-04-25T10:00:00+00:00".to_string(),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"token\":\"dt_a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6\""));
        assert!(json.contains("\"id\":7"));
        assert!(json.contains("\"key_prefix\":\"dt_a1b2c3d4\""));
    }

    #[test]
    fn api_key_response_with_last_used_at() {
        let resp = ApiKeyResponse {
            id: 1,
            name: "k".to_string(),
            key_prefix: "dt_xxxx".to_string(),
            created_at: "2026-04-25T10:00:00+00:00".to_string(),
            last_used_at: Some("2026-04-25T11:00:00+00:00".to_string()),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"last_used_at\":\"2026-04-25T11:00:00+00:00\""));
    }

    #[test]
    fn api_key_response_without_last_used_at_serializes_null() {
        let resp = ApiKeyResponse {
            id: 1,
            name: "k".to_string(),
            key_prefix: "dt_xxxx".to_string(),
            created_at: "2026-04-25T10:00:00+00:00".to_string(),
            last_used_at: None,
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"last_used_at\":null"));
    }

    #[test]
    fn api_key_response_roundtrip() {
        let resp = ApiKeyResponse {
            id: 42,
            name: "rt".to_string(),
            key_prefix: "dt_pfx".to_string(),
            created_at: "2026-04-25T10:00:00+00:00".to_string(),
            last_used_at: None,
        };
        let json = serde_json::to_string(&resp).unwrap();
        let back: ApiKeyResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, 42);
        assert_eq!(back.name, "rt");
        assert_eq!(back.key_prefix, "dt_pfx");
        assert!(back.last_used_at.is_none());
    }
}

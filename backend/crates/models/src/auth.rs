use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VerifyEmailResponse {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_request_deserializes() {
        let json = r#"{"username":"alice","email":"alice@example.com","password":"secret123"}"#;
        let req: RegisterRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.username, "alice");
        assert_eq!(req.email, "alice@example.com");
        assert_eq!(req.password, "secret123");
    }

    #[test]
    fn login_request_deserializes() {
        let json = r#"{"username":"bob","password":"pass456"}"#;
        let req: LoginRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.username, "bob");
        assert_eq!(req.password, "pass456");
    }

    #[test]
    fn token_response_serializes() {
        let resp = TokenResponse {
            token: "abc.def.ghi".to_string(),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"token\":\"abc.def.ghi\""));
    }

    #[test]
    fn token_response_roundtrip() {
        let resp = TokenResponse {
            token: "mytoken".to_string(),
        };
        let json = serde_json::to_string(&resp).unwrap();
        let deser: TokenResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deser.token, "mytoken");
    }

    #[test]
    fn user_response_serializes() {
        let resp = UserResponse {
            id: 1234,
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            email_verified: false,
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"id\":1234"));
        assert!(json.contains("\"email_verified\":false"));
    }

    #[test]
    fn user_response_roundtrip() {
        let resp = UserResponse {
            id: 255,
            username: "bob".to_string(),
            email: "bob@test.com".to_string(),
            email_verified: true,
        };
        let json = serde_json::to_string(&resp).unwrap();
        let deser: UserResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deser.id, 255);
        assert!(deser.email_verified);
    }

    #[test]
    fn verify_email_request_deserializes() {
        let json = r#"{"email":"test@test.com","code":"123456"}"#;
        let req: VerifyEmailRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.email, "test@test.com");
        assert_eq!(req.code, "123456");
    }

    #[test]
    fn verify_email_response_serializes() {
        let resp = VerifyEmailResponse {
            message: "ok".to_string(),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"message\":\"ok\""));
    }

    #[test]
    fn register_request_clone() {
        let req = RegisterRequest {
            username: "alice".to_string(),
            email: "a@b.com".to_string(),
            password: "pass".to_string(),
        };
        let cloned = req.clone();
        assert_eq!(cloned.username, "alice");
        assert_eq!(cloned.email, "a@b.com");
    }
}

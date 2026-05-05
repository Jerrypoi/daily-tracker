use axum::{
    body::Body,
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static JWT_SECRET: Lazy<String> =
    Lazy::new(|| std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable must be set"));

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

/// Auth source recorded in request extensions so handlers can distinguish a
/// logged-in user (JWT) from a programmatic caller (API key).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthMethod {
    Jwt,
    ApiKey,
}

fn extract_bearer(req: &Request<Body>) -> Option<String> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

/// Accepts either a JWT or a `dt_`-prefixed API key in the `Authorization: Bearer ...`
/// header. Tokens are dispatched by prefix: API-key lookups hit the database;
/// everything else is decoded as a JWT.
pub async fn auth_middleware(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    if req.method() == axum::http::Method::OPTIONS {
        return Ok(next.run(req).await);
    }

    let token = extract_bearer(&req).ok_or(StatusCode::UNAUTHORIZED)?;

    let (user_id, method) = if token.starts_with(db::API_KEY_PREFIX) {
        let user_id = db::lookup_api_key(&token)
            .map_err(|e| {
                log::error!("Failed to look up API key: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .ok_or(StatusCode::UNAUTHORIZED)?;
        (user_id, AuthMethod::ApiKey)
    } else {
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
        let user_id = token_data
            .claims
            .sub
            .parse::<i64>()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        (user_id, AuthMethod::Jwt)
    };

    req.extensions_mut().insert(user_id);
    req.extensions_mut().insert(method);
    Ok(next.run(req).await)
}

/// Like `auth_middleware`, but rejects API-key-authenticated requests. Used to
/// gate API-key management — an API key cannot be used to mint or revoke other
/// API keys.
pub async fn jwt_only_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if req.method() == axum::http::Method::OPTIONS {
        return Ok(next.run(req).await);
    }

    let token = extract_bearer(&req).ok_or(StatusCode::UNAUTHORIZED)?;

    if token.starts_with(db::API_KEY_PREFIX) {
        return Err(StatusCode::FORBIDDEN);
    }

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id = token_data
        .claims
        .sub
        .parse::<i64>()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(user_id);
    req.extensions_mut().insert(AuthMethod::Jwt);
    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{DecodingKey, Validation, decode};

    fn setup_jwt_secret() {
        // Ensure JWT_SECRET is set for tests
        // SAFETY: Tests run sequentially via --test-threads=1 or are independent of
        // each other's env reads; no other thread reads JWT_SECRET concurrently here.
        unsafe {
            std::env::set_var("JWT_SECRET", "test_secret_key_for_unit_tests");
        }
    }

    #[test]
    fn create_jwt_returns_valid_token() {
        setup_jwt_secret();
        let user_id = 42;
        let token = create_jwt(user_id).unwrap();
        assert!(!token.is_empty());

        // Decode and verify
        let secret = std::env::var("JWT_SECRET").unwrap();
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .unwrap();

        assert_eq!(token_data.claims.sub, user_id.to_string());
    }

    #[test]
    fn create_jwt_sub_is_decimal_user_id() {
        setup_jwt_secret();
        let user_id = 3_735_928_559;
        let token = create_jwt(user_id).unwrap();

        let secret = std::env::var("JWT_SECRET").unwrap();
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .unwrap();

        assert_eq!(token_data.claims.sub, "3735928559");
    }

    #[test]
    fn create_jwt_expiry_is_in_the_future() {
        setup_jwt_secret();
        let user_id = 1;
        let token = create_jwt(user_id).unwrap();

        let secret = std::env::var("JWT_SECRET").unwrap();
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .unwrap();

        let now = chrono::Utc::now().timestamp() as usize;
        assert!(token_data.claims.exp > now);
        // Should expire ~7 days from now
        let seven_days = 7 * 24 * 60 * 60;
        assert!(token_data.claims.exp <= now + seven_days + 10); // small tolerance
    }

    #[test]
    fn create_jwt_different_users_produce_different_tokens() {
        setup_jwt_secret();
        let token1 = create_jwt(1).unwrap();
        let token2 = create_jwt(2).unwrap();
        assert_ne!(token1, token2);
    }

    #[test]
    fn jwt_decode_with_wrong_secret_fails() {
        setup_jwt_secret();
        let token = create_jwt(1).unwrap();

        let result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(b"wrong_secret"),
            &Validation::default(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn create_jwt_with_zero_user_id() {
        setup_jwt_secret();
        let token = create_jwt(0).unwrap();
        assert!(!token.is_empty());

        let secret = std::env::var("JWT_SECRET").unwrap();
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .unwrap();

        assert_eq!(token_data.claims.sub, "0");
    }

    // --- auth_middleware tests ---
    use axum::{Router, body::Body, middleware as axum_mw, routing::get};
    use tower::ServiceExt as _;

    async fn protected_handler(
        axum::extract::Extension(user_id): axum::extract::Extension<i64>,
    ) -> String {
        user_id.to_string()
    }

    fn test_app_with_auth() -> Router {
        Router::new()
            .route("/protected", get(protected_handler))
            .route_layer(axum_mw::from_fn(auth_middleware))
    }

    #[tokio::test]
    async fn middleware_rejects_missing_auth_header() {
        setup_jwt_secret();
        let app = test_app_with_auth();
        let request = axum::extract::Request::builder()
            .uri("/protected")
            .body(Body::empty())
            .unwrap();

        let response: axum::response::Response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn middleware_rejects_non_bearer_token() {
        setup_jwt_secret();
        let app = test_app_with_auth();
        let request = axum::extract::Request::builder()
            .uri("/protected")
            .header("Authorization", "Basic abc123")
            .body(Body::empty())
            .unwrap();

        let response: axum::response::Response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn middleware_rejects_invalid_token() {
        setup_jwt_secret();
        let app = test_app_with_auth();
        let request = axum::extract::Request::builder()
            .uri("/protected")
            .header("Authorization", "Bearer invalid.token.here")
            .body(Body::empty())
            .unwrap();

        let response: axum::response::Response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn middleware_accepts_valid_token() {
        setup_jwt_secret();
        let user_id = 2_882_400_001;
        let token = create_jwt(user_id).unwrap();

        let app = test_app_with_auth();
        let request = axum::extract::Request::builder()
            .uri("/protected")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response: axum::response::Response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = http_body_util::BodyExt::collect(response.into_body())
            .await
            .unwrap()
            .to_bytes();
        assert_eq!(&body[..], b"2882400001");
    }

    #[tokio::test]
    async fn middleware_allows_options_without_auth() {
        setup_jwt_secret();
        let app = test_app_with_auth();
        let request = axum::extract::Request::builder()
            .uri("/protected")
            .method("OPTIONS")
            .body(Body::empty())
            .unwrap();

        let response: axum::response::Response = app.oneshot(request).await.unwrap();
        // OPTIONS bypasses auth — should not be 401
        assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
    }

    // --- jwt_only_middleware tests ---

    async fn auth_method_handler(
        axum::extract::Extension(method): axum::extract::Extension<AuthMethod>,
    ) -> String {
        format!("{:?}", method)
    }

    fn test_app_with_jwt_only() -> Router {
        Router::new()
            .route("/jwt-only", get(auth_method_handler))
            .route_layer(axum_mw::from_fn(jwt_only_middleware))
    }

    #[tokio::test]
    async fn jwt_only_middleware_rejects_missing_auth() {
        setup_jwt_secret();
        let app = test_app_with_jwt_only();
        let request = axum::extract::Request::builder()
            .uri("/jwt-only")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn jwt_only_middleware_rejects_non_bearer() {
        setup_jwt_secret();
        let app = test_app_with_jwt_only();
        let request = axum::extract::Request::builder()
            .uri("/jwt-only")
            .header("Authorization", "Basic abc")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn jwt_only_middleware_rejects_api_key_token_with_403() {
        // The dt_-prefixed branch must short-circuit before any DB lookup.
        setup_jwt_secret();
        let app = test_app_with_jwt_only();
        let request = axum::extract::Request::builder()
            .uri("/jwt-only")
            .header(
                "Authorization",
                "Bearer dt_a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6",
            )
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn jwt_only_middleware_rejects_invalid_jwt() {
        setup_jwt_secret();
        let app = test_app_with_jwt_only();
        let request = axum::extract::Request::builder()
            .uri("/jwt-only")
            .header("Authorization", "Bearer not.a.jwt")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn jwt_only_middleware_accepts_valid_jwt_and_marks_method() {
        setup_jwt_secret();
        let token = create_jwt(305_419_896).unwrap();

        let app = test_app_with_jwt_only();
        let request = axum::extract::Request::builder()
            .uri("/jwt-only")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = http_body_util::BodyExt::collect(response.into_body())
            .await
            .unwrap()
            .to_bytes();
        assert_eq!(&body[..], b"Jwt");
    }

    #[tokio::test]
    async fn jwt_only_middleware_allows_options() {
        setup_jwt_secret();
        let app = test_app_with_jwt_only();
        let request = axum::extract::Request::builder()
            .uri("/jwt-only")
            .method("OPTIONS")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
        assert_ne!(response.status(), StatusCode::FORBIDDEN);
    }

    // --- extract_bearer tests ---

    fn req_with_auth(value: Option<&str>) -> Request<Body> {
        let mut builder = axum::extract::Request::builder().uri("/x");
        if let Some(v) = value {
            builder = builder.header("Authorization", v);
        }
        builder.body(Body::empty()).unwrap()
    }

    #[test]
    fn extract_bearer_returns_token() {
        let req = req_with_auth(Some("Bearer abc.def.ghi"));
        assert_eq!(extract_bearer(&req).as_deref(), Some("abc.def.ghi"));
    }

    #[test]
    fn extract_bearer_returns_none_when_missing() {
        let req = req_with_auth(None);
        assert!(extract_bearer(&req).is_none());
    }

    #[test]
    fn extract_bearer_returns_none_for_non_bearer_scheme() {
        let req = req_with_auth(Some("Basic abc"));
        assert!(extract_bearer(&req).is_none());
    }

    #[test]
    fn extract_bearer_preserves_dt_prefix() {
        let req = req_with_auth(Some("Bearer dt_a1b2c3d4"));
        assert_eq!(extract_bearer(&req).as_deref(), Some("dt_a1b2c3d4"));
    }

    #[test]
    fn auth_method_debug_format() {
        // The jwt_only test relies on the Debug repr of AuthMethod.
        assert_eq!(format!("{:?}", AuthMethod::Jwt), "Jwt");
        assert_eq!(format!("{:?}", AuthMethod::ApiKey), "ApiKey");
    }
}

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{
        request::Parts, HeaderName, StatusCode
    }
};
use crate::api::models::user::User;

pub struct ExtractUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let api_key_header = HeaderName::from_static("x-api-key");

        let api_key = parts.headers.get(&api_key_header)
            .ok_or((StatusCode::BAD_REQUEST, "API key header is missing"))?
            .to_str()
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid API key format"))?;
        
        let user = fetch_user_by_api_key(api_key).await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid API key"))?;
        
        Ok(ExtractUser(user))
    }
}

async fn fetch_user_by_api_key(api_key: &str) -> Result<User, ()> {
    Ok(User { key: api_key.to_string() })
}
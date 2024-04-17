use crate::{api::models::user, AppState};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderName, StatusCode},
};

pub struct ExtractUser(pub user::User);

#[async_trait]
impl FromRequestParts<AppState> for ExtractUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let api_key_header = HeaderName::from_static("x-api-key");
        let api_key = parts
            .headers
            .get(&api_key_header)
            .ok_or((
                StatusCode::BAD_REQUEST,
                "API key header is missing, check /docs for more information",
            ))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "Invalid API key format, check /docs for more information",
                )
            })?;

        let mut user = user::find_user_by_key(&state.database.user_collection, api_key)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occured while trying to fetch user",
                )
            })?
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Invalid API key, check /docs for more information",
            ))?;

        let method = parts.method.as_str();
        let path = parts.uri.path();

        user.use_endpoint(method, path);
        user.save(&state.database.user_collection)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occured while trying to save user",
                )
            })?;

        Ok(ExtractUser(user))
    }
}

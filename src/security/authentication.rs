use std::sync::Arc;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{
        request::Parts, HeaderName, StatusCode
    }, Extension
};
use tokio::sync::Mutex;
use crate::api::{database::db::DB, models::user};

pub struct ExtractUser(pub user::User);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(db) = Extension::<Arc<Mutex<DB>>>::from_request_parts(parts, state)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "An error occured while trying to access database"))?;
        
        let api_key_header = HeaderName::from_static("x-api-key");

        let api_key = parts.headers.get(&api_key_header)
            .ok_or((StatusCode::BAD_REQUEST, "API key header is missing"))?
            .to_str()
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid API key format"))?;
        
        let db_lock = db.lock().await;
        let user = user::find_user_by_key(&db_lock.user_collection, api_key).await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "An error occured while trying to fetch user"))?
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid API key"))?;
        
        Ok(ExtractUser(user))
    }
}
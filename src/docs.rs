use utoipa::{openapi::security::{ApiKey, ApiKeyValue, SecurityScheme}, Modify, OpenApi};
use crate::api::{self, models::{enums::PrivacyLevel, response_models::{MessageResponse, Pagination, UserList, UserPrivateInformation, UserPublicInformation}, user_settings::UserSettings}};

#[derive(OpenApi)]
#[openapi(
    info(
        title="LemCom API",
        description="A webservice for handling LemCom online services. LemCom will be a messaging application for desktop written in Rust.\n\nAll available docs: Rapidoc (/docs), Swagger (/swagger) and Redoc (/redoc)."
    ),
    paths(
        api::resources::ping::get_ping,  
        api::resources::user::get_user,
        api::resources::user::get_user_search,
        api::resources::user::get_user_settings,
        api::resources::user::patch_user_settings,
        api::resources::users::get_users
    ),
    tags(
        (name = "Misc", description = "Miscellaneous endppoints"),
        (name = "User", description = "User management endpoints"),
        (name = "Users", description = "Endpoint for handling multiple users"),
    ),
    modifiers(&SecurityAddon),
    components(
        schemas(MessageResponse, UserPublicInformation, UserPrivateInformation, UserSettings, UserList, Pagination, PrivacyLevel),
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-api-key"))),
            )
        }
    }
}
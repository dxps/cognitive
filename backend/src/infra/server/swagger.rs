use crate::infra::http_api::{__path_login, __path_logout, __path_update_user_password, __path_update_user_primary_info};
use shlib::http_dtos::LoginResponse;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(login, logout, update_user_primary_info, update_user_password),
    components(schemas(LoginResponse)),
    tags(
        (name = "Auth", description = "Authentication"),
        (name = "User Settings", description = "User Profile, Password")
    )
)]
pub struct ApiDoc;

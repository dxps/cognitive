use crate::infra::http_api::{
    __path_add_attribute_template, __path_is_admin, __path_list_attribute_templates, 
    __path_get_attribute_template, __path_update_attribute_template,
    __path_login, __path_logout,
    __path_remove_attribute_template, __path_update_user_password, __path_update_user_primary_info,
};
use shlib::http_dtos::LoginResponse;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(login, logout, update_user_primary_info, update_user_password, is_admin,  
        add_attribute_template, list_attribute_templates, 
        get_attribute_template, update_attribute_template, remove_attribute_template
    ),
    components(schemas(LoginResponse)),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "User Info & Settings", description = "User related details endpoints"),
        (name = "Attribute Templates", description = "Attributes Templates related endpoints")
    )
)]
pub struct ApiDoc;

use crate::infra::ServerState;
use axum::{
    Json,
    extract::{Path, State},
};
use http::StatusCode;
use shlib::{
    domain::model::{AttributeTemplate, Id},
    http_dtos::ErrorResponse,
};

#[utoipa::path(
    get,
    path = "/data/templates/attributes",
    description = "List all attribute templates.",
    responses(
        (status = 200, description = "The list of attribute templates", body = [AttributeTemplate])
    ),
    tag = "Attribute Templates"
)]
pub async fn list_attribute_templates(
    State(state): State<ServerState>,
) -> Result<(StatusCode, Json<Vec<AttributeTemplate>>), (StatusCode, Json<ErrorResponse>)> {
    //
    let items = state.attr_tmpl_mgmt.list().await;
    match items {
        Err(e) => {
            log::error!("Failed to list attribute templates. Reason: '{}'.", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_owned(),
                }),
            ));
        }
        Ok(items) => Ok((StatusCode::OK, Json(items))),
    }
}

#[utoipa::path(
    post,
    path = "/data/templates/attributes",
    description = "Add an attribute templates.",
    request_body(
        content = AttributeTemplate,
        description = "The attribute template to add.",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The attribute template was added successfully.", body = [AttributeTemplate])
    ),
    tag = "Attribute Templates"
)]
pub async fn add_attribute_template(
    State(state): State<ServerState>,
    Json(mut attr_tmpl): Json<AttributeTemplate>,
) -> Result<(StatusCode, Json<AttributeTemplate>), (StatusCode, Json<ErrorResponse>)> {
    //
    let id_result = state.attr_tmpl_mgmt.add(attr_tmpl.clone()).await;
    match id_result {
        Err(e) => {
            log::error!("Failed to add attribute template. Reason: '{}'.", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_owned(),
                }),
            ));
        }
        Ok(id) => {
            attr_tmpl.id = id;
            Ok((StatusCode::CREATED, Json(attr_tmpl)))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/data/templates/attributes/{id}",
    description = "Remove an attribute template.",
    params(
        ("id" = String, Path, description = "The ID of the attribute template to remove.")
    ),
    responses(
        (status = 204, description = "The attribute template was removed successfully.", body = [AttributeTemplate])
    ),
    tag = "Attribute Templates"
)]
pub async fn remove_attribute_template(
    State(state): State<ServerState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    //
    let id = Id::from(id);
    let result = state.attr_tmpl_mgmt.remove(id).await;
    match result {
        Err(e) => {
            log::error!("Failed to remove attribute template. Reason: '{}'.", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_owned(),
                }),
            ));
        }
        Ok(_) => Ok(StatusCode::NO_CONTENT),
    }
}

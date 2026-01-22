use crate::infra::ServerState;
use axum::{Json, extract::State};
use http::StatusCode;
use shlib::{domain::model::AttributeTemplate, http_dtos::ErrorResponse};

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

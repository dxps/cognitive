use crate::infra::ServerState;
use axum::{Json, extract::State};
use http::StatusCode;
use log::debug;
use shlib::http_dtos::{ErrorResponse, UserProfileUpdateRequest};

#[utoipa::path(
    put,
    path = "/user/profile",
    request_body(
        content = UserProfileUpdateRequest,
        description = "User profile primary info update request",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "User profile updated successfully")
    ),
    tag = "User Info & Settings"
)]
pub async fn update_user_primary_info(
    State(state): State<ServerState>,
    Json(payload): Json<UserProfileUpdateRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    //
    debug!("[update_user_primary_info] Received payload='{:#?}'.", payload);

    match state.user_mgmt.update_user_account(payload.into()).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            debug!("[update_user_primary_info] Error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_owned(),
                }),
            ))
        }
    }
}

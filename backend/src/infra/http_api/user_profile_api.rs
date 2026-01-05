use axum::{Json, extract::State};
use http::StatusCode;
use log::debug;
use shlib::http_dtos::{ErrorResponse, UserPasswordUpdateRequest, UserProfileUpdateRequest};

use crate::infra::ServerState;

pub async fn update_user_primary_info(
    State(state): State<ServerState>,
    Json(payload): Json<UserProfileUpdateRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    //
    debug!("[save_user_profile_primary_info] Received payload='{:#?}'.", payload);

    match state.user_mgmt.update_user_account(payload.into()).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            debug!("[save_user_profile_primary_info] Error: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_owned(),
                }),
            ))
        }
    }
}

pub async fn update_user_password(
    State(state): State<ServerState>,
    Json(payload): Json<UserPasswordUpdateRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    //
    debug!("[update_user_password] Received '{:#?}'.", payload);

    match state
        .user_mgmt
        .update_password(&payload.id, payload.curr_password, payload.new_password)
        .await
    {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            debug!("[update_user_password] Error: {}", err);
            match err {
                shlib::AppError::Unauthorized(msg) => Err((StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: msg }))),
                _ => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Internal server error".to_owned(),
                    }),
                )),
            }
        }
    }
}

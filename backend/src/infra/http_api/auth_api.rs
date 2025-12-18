use axum::{Json, extract::State};
use axum_session::Session;
use axum_session_sqlx::SessionPgPool;
use http::StatusCode;
use log::debug;
use shlib::http_dtos::{ErrorResponse, LoginRequest, LoginResponse};

use crate::infra::{SESSION_MAX_LIFESPAN, ServerState};

pub async fn login(
    State(state): State<ServerState>,
    session: Session<SessionPgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, Json<ErrorResponse>)> {
    //
    debug!("Received login request '{:?}'.", payload);

    state
        .user_mgmt
        .authenticate_user(payload.email, payload.password)
        .await
        .map_err(|err| match err {
            shlib::AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, Json(msg.into())),
            _ => {
                debug!("Login error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Internal server error".to_owned(),
                    }),
                )
            }
        })?;

    session.set_store(true);

    let response = LoginResponse {
        session: session.get_session_id(),
        expires_in_seconds: SESSION_MAX_LIFESPAN.num_seconds(),
    };
    Ok((StatusCode::OK, Json(response)))
}

pub async fn logout(session: Session<SessionPgPool>) -> StatusCode {
    //
    let sid = session.get_session_id();
    session.destroy();
    debug!("User logged out, session {} cleared.", sid);
    StatusCode::NO_CONTENT
}

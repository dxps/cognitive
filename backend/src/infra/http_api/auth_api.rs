use crate::infra::{AuthUserAccount, SESSION_CURRENT_USER_KEY, SESSION_MAX_LIFESPAN, ServerState};
use axum::{Json, extract::State};
use axum_session::Session;
use axum_session_sqlx::SessionPgPool;
use http::StatusCode;
use log::debug;
use shlib::http_dtos::{ErrorResponse, LoginRequest, LoginResponse, UserPasswordUpdateRequest};

#[utoipa::path(
    post,
    path = "/auth/login",
    responses(
        (status = 200, description = "Login successful", body = LoginResponse)
    ),
    tag = "Auth"
)]
pub async fn login(
    State(state): State<ServerState>,
    session: Session<SessionPgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, Json<ErrorResponse>)> {
    //
    debug!("[login] Received '{:?}'.", payload);

    let user_account = state
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

    session.set(SESSION_CURRENT_USER_KEY, AuthUserAccount::from(user_account.clone()));

    let response = LoginResponse {
        session: session.get_session_id(),
        expires_in_seconds: SESSION_MAX_LIFESPAN.num_seconds(),
        user: Some(user_account),
    };
    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = 200, description = "Logout successful")
    ),
    tag = "Auth"
)]
pub async fn logout(session: Session<SessionPgPool>) -> StatusCode {
    //
    let sid = session.get_session_id();
    session.destroy();
    debug!("User logged out, session '{}' cleared.", sid);
    StatusCode::NO_CONTENT
}

#[utoipa::path(
    get,
    path = "/auth/is_admin",
    description = "Check if the current user has admin permissions.",
    responses(
        (status = 200, description = "User has admin permissions."),
        (status = 401, description = "User does not have admin permissions.")
    ),
    tag = "Auth"
)]
pub async fn is_admin(State(state): State<ServerState>, session: Session<SessionPgPool>) -> StatusCode {
    //
    if let Some(curr_account) = session.get::<AuthUserAccount>(SESSION_CURRENT_USER_KEY) {
        // Let's check permissions only and not worry about if the user is anonymous or not.
        if state.user_mgmt.is_admin(&curr_account).await {
            return StatusCode::OK;
        }
    }
    StatusCode::UNAUTHORIZED
}

#[utoipa::path(
    put,
    path = "/auth/password",
    description = "Update user password.",
    request_body(
        content = UserPasswordUpdateRequest,
        description = "User profile primary info update request.",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "User password updated successfully.")
    ),
    tag = "Auth"
)]
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

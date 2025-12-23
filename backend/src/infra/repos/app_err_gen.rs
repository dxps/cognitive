use shlib::AppError;

pub fn new_app_error_from_sqlx(err: sqlx::Error, msg: Option<String>) -> AppError {
    log::trace!("mapping AppError from sqlx err={:?}", err);

    let msg = match &err {
        sqlx::Error::RowNotFound => AppError::ResourceNotFound,
        _ => {
            // FYI: For now, any specifically unhandled error is considered as internal error.
            if let Some(db_err) = err.as_database_error() {
                if let Some(code) = db_err.code() {
                    if code.as_ref() == "23505" {
                        return AppError::AlreadyExists(msg.unwrap_or_default());
                    }
                }
            }
            AppError::InternalErr(msg.unwrap_or_default())
        }
    };

    AppError::from(msg)
}

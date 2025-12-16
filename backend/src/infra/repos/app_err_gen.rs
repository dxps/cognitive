use shlib::AppError;

pub fn new_app_error_from_sqlx(err: sqlx::Error) -> AppError {
    log::debug!("from(sqlx:Error): err={:?}", err);

    let msg = match &err {
        sqlx::Error::RowNotFound => AppError::Err("Record not found".to_string()),
        _ => {
            // FYI: For now, any specifically unhandled error is considered as internal error.
            if let Some(db_err) = err.as_database_error() {
                match db_err.code() {
                    Some(code) => match code.as_ref() {
                        "23505" => AppError::AlreadyExists("".into()),
                        _ => AppError::InternalErr,
                    },
                    None => AppError::InternalErr,
                }
            } else {
                AppError::InternalErr
            }
        }
    };

    AppError::from(msg)
}

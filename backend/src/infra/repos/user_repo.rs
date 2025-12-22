use crate::{
    domain::model::new_id,
    infra::{AuthUserAccount, new_app_error_from_sqlx},
};
use shlib::{
    AppError, AppResult,
    domain::model::{Id, UserAccount, UserEntry, UserPasswordSalt},
};
use sqlx::{PgPool, Row, postgres::PgRow};
use std::sync::Arc;

pub struct UserRepo {
    dbcp: Arc<PgPool>,
}

impl UserRepo {
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get_by_email(&self, email: &String) -> AppResult<UserEntry> {
        //
        let row = sqlx::query(
            "SELECT id, email, username, password, salt, bio, is_anonymous FROM user_accounts 
             WHERE email = $1",
        )
        .bind(email)
        .fetch_one(self.dbcp.as_ref())
        .await
        .map_err(|err| new_app_error_from_sqlx(err))?;

        let user_account = UserAccount {
            id: Id::new_from(row.get("id")),
            email: row.get("email"),
            username: row.get("username"),
            bio: row.get("bio"),
            is_anonymous: row.get("is_anonymous"),
            permissions: Vec::new(),
        };

        Ok(UserEntry {
            user: user_account,
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }

    pub async fn get_by_id(id: &Id, pool: &PgPool) -> Option<AuthUserAccount> {
        //
        let row = sqlx::query(
            "SELECT id, email, username, bio, is_anyonymous FROM user_accounts WHERE id = $1",
        )
        .bind(id.as_str())
        .fetch_one(pool)
        .await
        .ok()?;

        let mut user_account = UserAccount {
            id: Id::new_from(row.get("id")),
            email: row.get("email"),
            username: row.get("username"),
            bio: row.get("bio"),
            is_anonymous: row.get("is_anonymous"),
            permissions: Vec::new(),
        };

        let mut permissions =
            sqlx::query("SELECT permission FROM user_permissions WHERE user_id = $1;")
                .map(|r: PgRow| r.get("permission"))
                .fetch_all(pool)
                .await
                .ok()?;

        user_account.permissions.append(&mut permissions);
        Some(user_account.into())
    }

    pub async fn get_password_by_id(&self, user_id: &Id) -> AppResult<UserPasswordSalt> {
        //
        let row = sqlx::query("SELECT password, salt FROM user_accounts WHERE id = $1")
            .bind(user_id.as_str())
            .fetch_one(self.dbcp.as_ref())
            .await
            .map_err(|err| new_app_error_from_sqlx(err))?;

        Ok(UserPasswordSalt {
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }

    pub async fn update_password(&self, user_id: &Id, pwd: String) -> AppResult<()> {
        //
        match sqlx::query("UPDATE user_accounts SET password = $1 WHERE id = $2")
            .bind(pwd)
            .bind(user_id.as_str())
            .execute(self.dbcp.as_ref())
            .await
            .map_err(|err| new_app_error_from_sqlx(err))
        {
            Ok(_) => Ok(()),
            Err(err) => Err(AppError::from(err)),
        }
    }

    pub async fn save(
        &self,
        email: String,
        username: String,
        pwd: String,
        salt: String,
    ) -> AppResult<Id> {
        //
        let id = new_id();
        match sqlx::query(
            "INSERT INTO user_accounts (id, email, username, password, salt) 
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(id.as_str())
        .bind(email)
        .bind(username)
        .bind(pwd)
        .bind(salt)
        .execute(self.dbcp.as_ref())
        .await
        {
            Ok(_) => Ok(id),
            Err(err) => Err(new_app_error_from_sqlx(err)),
        }
    }

    pub async fn save_with_permissions(
        &self,
        email: &String,
        username: &String,
        pwd: &String,
        salt: &String,
        permissions: Vec<String>,
    ) -> AppResult<Id> {
        //
        let id = new_id();
        let res = sqlx::query(
            "INSERT INTO user_accounts (id, email, username, password, salt) 
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&id.as_str())
        .bind(&email)
        .bind(&username)
        .bind(pwd)
        .bind(salt)
        .execute(self.dbcp.as_ref())
        .await
        .map_err(|err| {
            log::error!("Failed to save user: {}", err);
            new_app_error_from_sqlx(err)
        });

        if res.is_ok() {
            for permission in permissions.iter() {
                let res = sqlx::query(
                    "INSERT INTO user_permissions (user_id, permission) VALUES ($1, $2)",
                )
                .bind(&id.as_str())
                .bind(&permission)
                .execute(self.dbcp.as_ref())
                .await
                .map_err(|err| {
                    log::error!("Failed to save user permissions: {}", err);
                    new_app_error_from_sqlx(err)
                });
                if res.is_err() {
                    return AppResult::Err(res.err().unwrap());
                }
            }
        } else {
            return AppResult::Err(res.err().unwrap());
        }
        AppResult::Ok(id)
    }

    pub async fn update(&self, ua: UserAccount) -> AppResult<()> {
        //
        match sqlx::query("UPDATE user_accounts SET username=$1, email=$2, bio=$3 WHERE id = $4")
            .bind(ua.username)
            .bind(ua.email)
            .bind(ua.bio)
            .bind(ua.id.as_str())
            .execute(self.dbcp.as_ref())
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(new_app_error_from_sqlx(err)),
        }
    }
}

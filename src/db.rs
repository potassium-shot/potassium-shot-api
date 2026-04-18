use chrono::{DateTime, Utc};
use tracing::info;

use crate::{
    prelude::*,
    types::{LoginToken, UserId},
};

#[derive(Clone)]
pub struct Db {
    conn: sqlx::SqlitePool,
}

impl Db {
    pub async fn new() -> Result<Self> {
        let path = std::env::var(crate::env::DB_PATH)
            .unwrap_or_else(|_| "/usr/share/potassium-shot-api/db.sqlite".into());

        let conn = sqlx::SqlitePool::connect(&format!("sqlite:{}:rwc", path)).await?;
        sqlx::migrate!().run(&conn).await?;
        info!("Database loaded.");
        Ok(Self { conn })
    }

    pub async fn add_user(&self, name: &str, password_hash: &str) -> Result<UserId> {
        let id = sqlx::query(
            r#"
            INSERT INTO users (name, password_hash) VALUES (?, ?);
            "#,
        )
        .bind(name)
        .bind(password_hash)
        .execute(&self.conn)
        .await?
        .last_insert_rowid();

        info!("User '{}' (id: {}) added.", name, id);

        Ok(UserId(id))
    }

    pub async fn get_user_name(&self, userid: UserId) -> Result<String> {
        let (name,): (String,) = sqlx::query_as(
            r#"
            SELECT name FROM user WHERE id = ?;
            "#,
        )
        .bind(*userid)
        .fetch_one(&self.conn)
        .await?;

        Ok(name)
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<UserId> {
        let (id,): (i64,) = sqlx::query_as(
            r#"
            SELECT id FROM users WHERE name = ?;
            "#,
        )
        .bind(name)
        .fetch_one(&self.conn)
        .await?;

        Ok(UserId(id))
    }

    pub async fn get_user_password_hash(&self, userid: UserId) -> Result<String> {
        let (password_hash,): (String,) = sqlx::query_as(
            r#"
            SELECT password_hash FROM users WHERE id = ?;
            "#,
        )
        .bind(*userid)
        .fetch_one(&self.conn)
        .await?;

        Ok(password_hash)
    }

    pub async fn remove_user(&self, userid: UserId) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user WHERE id = ?;
            "#,
        )
        .bind(*userid)
        .execute(&self.conn)
        .await?;

        info!(
            "User '{}' (id: {}) removed.",
            self.get_user_name(userid).await?,
            userid
        );

        Ok(())
    }

    pub async fn add_login_token(
        &self,
        userid: UserId,
        token: LoginToken,
        expiration: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO login_tokens (user, token, expiration) VALUES (?, ?, ?);
            "#,
        )
        .bind(*userid)
        .bind((*token).as_slice())
        .bind(expiration)
        .execute(&self.conn)
        .await?;

        info!(
            "Login token added for user '{}' (id: {}).",
            self.get_user_name(userid).await?,
            userid
        );

        Ok(())
    }

    pub async fn get_user_by_token(&self, token: LoginToken) -> Result<UserId> {
        let (userid,): (i64,) = sqlx::query_as(
            r#"
            SELECT user FROM login_tokens WHERE token = ?;
            "#,
        )
        .bind((*token).as_slice())
        .fetch_one(&self.conn)
        .await?;

        Ok(UserId(userid))
    }

    pub async fn cleanup(&self) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM login_tokens WHERE expiration < strftime('%s','now');
            VACUUM;
            "#,
        )
        .execute(&self.conn)
        .await?;

        info!("Database cleaned up.");

        Ok(())
    }
}

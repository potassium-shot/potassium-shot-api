use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::{
    prelude::*,
    types::{LoginToken, LoginTokenError, UserId},
};

const FIXED_TIMESTAMP_DELAY: Duration = Duration::from_millis(500);

pub struct Db {
    conn: sqlx::SqlitePool,
}

impl Db {
    pub async fn new() -> Result<Self> {
        let path = std::env::var(crate::env::DB_PATH)
            .unwrap_or_else(|_| "/usr/share/potassium-shot-api/db.sqlite".into());

        let conn = sqlx::SqlitePool::connect(&format!("sqlite:{}:rwc", path)).await?;
        sqlx::migrate!().run(&conn).await?;
        Ok(Self { conn })
    }

    pub async fn add_user(&self, name: &str, password_hash: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO users (name, password_hash) VALUES (?, ?);
            "#,
        )
        .bind(name)
        .bind(password_hash)
        .execute(&self.conn)
        .await?;

        Ok(())
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
            INSERT INTO login_tokens (user, token, expiration) VALUES (?, ?, ?);
            "#,
        )
        .bind(*userid)
        .bind((*token).as_str())
        .bind(expiration)
        .execute(&self.conn)
        .await?;

        Ok(())
    }

    pub async fn user_hash_token(&self, userid: UserId, token: LoginToken) -> Result<bool> {
        let fixed_timestamp_end = tokio::time::Instant::now() + FIXED_TIMESTAMP_DELAY;

        let tokens: Vec<(String,)> = sqlx::query_as(
            r#"
            SELECT token FROM login_tokens WHERE userid = ? AND expiration < strftime('%s','now');
            "#,
        )
        .bind(*userid)
        .fetch_all(&self.conn)
        .await?;

        if tokens
            .into_iter()
            .map(|(token,)| LoginToken::try_new(token))
            .collect::<Result<Vec<_>, LoginTokenError>>()?
            .contains(&token)
        {
            Ok(true)
        } else {
            tokio::time::sleep_until(fixed_timestamp_end).await;
            Ok(false)
        }
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

        Ok(())
    }
}

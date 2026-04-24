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
        let path = crate::env::DB_PATH.get();
        std::fs::create_dir_all(
            std::path::PathBuf::from(path.as_ref())
                .parent()
                .expect("DB path should point to a file."),
        )?;
        let conn = sqlx::SqlitePool::connect(&format!("sqlite:{}?mode=rwc", path)).await?;
        sqlx::migrate!().run(&conn).await?;
        info!("Database loaded.");
        Ok(Self { conn })
    }

    pub async fn add_user(&self, name: &str, password_hash: &str) -> Result<UserId> {
        let first = sqlx::query_as::<_, (i64,)>(
            r#"
            SELECT id FROM users
            "#,
        )
        .fetch_optional(&self.conn)
        .await?
        .is_none();

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

        if first {
            self.promote_user(UserId(id)).await?;
        }

        Ok(UserId(id))
    }

    pub async fn get_user_name(&self, userid: UserId) -> Result<String> {
        let (name,): (String,) = sqlx::query_as(
            r#"
            SELECT name FROM users WHERE id = ?;
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
        let username = self.get_user_name(userid).await?;

        sqlx::query(
            r#"
            DELETE FROM users WHERE id = ?;
            "#,
        )
        .bind(*userid)
        .execute(&self.conn)
        .await?;

        info!("User '{}' (id: {}) removed.", username, userid);

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

    pub async fn promote_user(&self, userid: UserId) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO admins (user) VALUES (?);
            "#,
        )
        .bind(*userid)
        .execute(&self.conn)
        .await?;

        info!(
            "User '{}' (id: {}) promoted to admin.",
            self.get_user_name(userid).await?,
            userid
        );

        Ok(())
    }

    pub async fn is_user_admin(&self, userid: UserId) -> Result<bool> {
        let found = sqlx::query_as::<_, (i64,)>(
            r#"
            SELECT id FROM admins WHERE user = ?;
            "#,
        )
        .bind(*userid)
        .fetch_optional(&self.conn)
        .await?
        .is_some();

        Ok(found)
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

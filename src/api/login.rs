use axum::Json;
use chrono::Utc;
use zeroize::Zeroizing;

use crate::prelude::*;

#[derive(serde::Deserialize)]
pub struct LoginArgs {
    username: String,
    password: Zeroizing<String>,
    remember: bool,
}

pub async fn login(
    Json(LoginArgs {
        username,
        password,
        remember,
    }): Json<LoginArgs>,
) -> AxumResult<Json<LoginToken>> {
    let db = super::db();
    let userid = db.get_user_by_name(username.as_str()).await?;
    let password_hash = db.get_user_password_hash(userid).await?;

    let token = LoginToken::generate();

    let expiration = Utc::now()
        + if remember {
            crate::constants::TOKEN_EXPIRATION_LONG
        } else {
            crate::constants::TOKEN_EXPIRATION_BASE
        };

    db.add_login_token(userid, token, expiration).await?;

    if crate::pswd::password_verify(password.as_str(), password_hash.as_str()).await? {
        Ok(Json(token))
    } else {
        Err(anyhow!("Wrong password.").into())
    }
}

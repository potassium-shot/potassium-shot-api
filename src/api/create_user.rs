use axum::Json;
use zeroize::Zeroizing;

use crate::prelude::*;

#[derive(serde::Deserialize, Debug)]
pub struct CreateUserArgs {
    username: String,
    password: Zeroizing<String>,
}

#[derive(serde::Serialize)]
pub struct CreateUserResult {
    userid: UserId,
}

pub async fn create_user(
    Json(CreateUserArgs { username, password }): Json<CreateUserArgs>,
) -> AxumResult<Json<CreateUserResult>> {
    let db = super::db();
    let password_hash = crate::pswd::password_hash(password.as_str())?;

    let userid = db
        .add_user(username.as_str(), password_hash.as_str())
        .await?;

    Ok(Json(CreateUserResult { userid }))
}

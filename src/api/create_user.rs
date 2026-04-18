use axum::Json;
use zeroize::Zeroizing;

use crate::prelude::*;

#[derive(serde::Deserialize, Debug)]
pub struct CreateUser {
    username: String,
    password: Zeroizing<String>,
}

pub async fn create_user(
    Json(CreateUser { username, password }): Json<CreateUser>,
) -> AxumResult<()> {
    let db = super::db();
    let password_hash = crate::pswd::password_hash(password.as_str())?;

    db.add_user(username.as_str(), password_hash.as_str())
        .await?;

    Ok(())
}

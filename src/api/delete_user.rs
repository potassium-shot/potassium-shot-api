use axum::Json;
use zeroize::Zeroizing;

use crate::prelude::*;

#[derive(serde::Deserialize)]
pub struct DeleteUserArgs {
    token: LoginToken,
    password: Zeroizing<String>,
}

pub async fn delete_user(
    Json(DeleteUserArgs { token, password }): Json<DeleteUserArgs>,
) -> AxumResult<()> {
    let db = super::db();
    let userid = db.get_user_by_token(token).await?;
    let password_hash = db.get_user_password_hash(userid).await?;

    if crate::pswd::password_verify(password.as_str(), password_hash.as_str()).await? {
        db.remove_user(userid).await?;
        Ok(())
    } else {
        Err(anyhow!("Wrong password.").into())
    }
}

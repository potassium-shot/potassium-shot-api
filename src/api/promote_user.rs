use axum::Json;

use crate::prelude::*;

#[derive(serde::Deserialize)]
pub struct PromoteUserArgs {
    token: LoginToken,
    user_to_promote: UserId,
}

pub async fn promote_user(
    Json(PromoteUserArgs {
        token,
        user_to_promote,
    }): Json<PromoteUserArgs>,
) -> AxumResult<()> {
    let db = super::db();

    let requesting_user = db.get_user_by_token(token).await?;

    if db.is_user_admin(requesting_user).await? {
        db.promote_user(user_to_promote).await?;
        Ok(())
    } else {
        Err(anyhow!("Must be an admin to promote other users to admin.").into())
    }
}

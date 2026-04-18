use axum::Json;

use crate::prelude::*;

#[derive(serde::Deserialize)]
pub struct GetUseridFromTokenArgs {
    token: LoginToken,
}

pub async fn get_userid_from_token(
    Json(GetUseridFromTokenArgs { token }): Json<GetUseridFromTokenArgs>,
) -> AxumResult<Json<UserId>> {
    let db = super::db();
    let userid = db.get_user_by_token(token).await?;
    Ok(Json(userid))
}

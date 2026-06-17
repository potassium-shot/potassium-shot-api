use axum::Json;

use crate::prelude::*;

pub async fn get_userid_from_token(
    Json(GetUserIdFromTokenArgs { token }): Json<GetUserIdFromTokenArgs>,
) -> AxumResult<Json<GetUserIdFromTokenResult>> {
    let db = super::db();
    let userid = db.get_user_by_token(token).await?;
    Ok(Json(GetUserIdFromTokenResult { userid }))
}

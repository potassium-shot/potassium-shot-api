use axum::Json;

use crate::prelude::*;

#[derive(serde::Deserialize)]
pub struct IsAdminArgs {
    userid: UserId,
}

#[derive(serde::Serialize)]
pub struct IsAdminResult {
    is_admin: bool,
}

pub async fn is_admin(
    Json(IsAdminArgs { userid }): Json<IsAdminArgs>,
) -> AxumResult<Json<IsAdminResult>> {
    let db = super::db();
    let is_admin = db.is_user_admin(userid).await?;
    Ok(Json(IsAdminResult { is_admin }))
}

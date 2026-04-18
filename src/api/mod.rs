use axum::routing::post;

use crate::db::Db;

mod create_user;
mod delete_user;
mod get_userid_from_token;
mod is_admin;
mod login;
mod promote_user;

pub fn make_router() -> axum::Router {
    axum::Router::new()
        .route("/create-user", post(create_user::create_user))
        .route("/delete-user", post(delete_user::delete_user))
        .route("/login", post(login::login))
        .route(
            "/get-userid-from-token",
            post(get_userid_from_token::get_userid_from_token),
        )
        .route("/is-admin", post(is_admin::is_admin))
        .route("/promote-user", post(promote_user::promote_user))
}

fn db() -> Db {
    Db::clone(
        crate::DB
            .get()
            .expect("axum should be serving only after crate::DB was intialized"),
    )
}

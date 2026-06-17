use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::types::{LoginToken, UserId};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserArgs {
    pub username: String,
    pub password: Zeroizing<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserResult {
    pub userid: UserId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUserArgs {
    pub token: LoginToken,
    pub password: Zeroizing<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserIdFromTokenArgs {
    pub token: LoginToken,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserIdFromTokenResult {
    pub userid: UserId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsAdminArgs {
    pub userid: UserId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsAdminResult {
    pub is_admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginArgs {
    pub username: String,
    pub password: Zeroizing<String>,
    pub remember: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResult {
    pub token: LoginToken,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromoteUserArgs {
    pub token: LoginToken,
    pub user_to_promote: UserId,
}

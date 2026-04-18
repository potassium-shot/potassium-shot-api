use deref_derive::Deref;

#[derive(Deref, Clone, Copy, PartialEq, Eq, Debug)]
pub struct UserId(pub i64);

#[nutype::nutype(
    validate(len_char_min = 20, len_char_max = 20),
    derive(Deref, Clone, PartialEq, Eq, Debug)
)]
pub struct LoginToken(String);

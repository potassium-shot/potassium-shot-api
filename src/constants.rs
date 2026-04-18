use std::time::Duration;

pub const FIXED_TIMESTAMP_DELAY: Duration = Duration::from_millis(500);
pub const TOKEN_EXPIRATION_BASE: Duration = Duration::from_mins(5);
pub const TOKEN_EXPIRATION_LONG: Duration = Duration::from_hours(24 * 15);

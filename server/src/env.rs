use crate::utils::env::EnvOverride;

pub const DB_PATH: EnvOverride = EnvOverride::new(
    "POTASSIUM_SHOT_API_DB_PATH",
    "/usr/share/potassium-shot-api/db.sqlite",
);
pub const PLUGINS_PATH: EnvOverride = EnvOverride::new(
    "POTASSIUM_SHOT_API_PLUGINS_PATH",
    "/usr/share/potassium-shot-api/plugins",
);
pub const LISTEN_ADDR: EnvOverride = EnvOverride::new("POTASSIUM_SHOT_API_LISTEN_ADDR", "0.0.0.0");
pub const LISTEN_PORT: EnvOverride = EnvOverride::new("POTASSIUM_SHOT_API_LISTEN_PORT", "8080");

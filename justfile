[private]
default:
    @just --list

run:
    POTASSIUM_SHOT_API_DB_PATH="server/test-db.sqlite" POTASSIUM_SHOT_API_PLUGINS_PATH="server/test-plugins" cargo run -p potassium-shot-api

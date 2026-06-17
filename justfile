[private]
default:
    @just --list

run:
    POTASSIUM_SHOT_API_DB_PATH="server/test-db.sqlite" cargo run -p potassium-shot-api


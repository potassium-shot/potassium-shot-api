[private]
default:
    @just --list

run:
    POTASSIUM_SHOT_API_DB_PATH="./test-db.sqlite" cargo run

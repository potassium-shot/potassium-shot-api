FROM rust:1.95.0-slim AS builder
WORKDIR /usr/src/potassium-shot-api
COPY . .
RUN cargo build --release

FROM debian:oldstable-20260316-slim
COPY --from=builder /usr/src/potassium-shot-api/target/release/potassium-shot-api /usr/local/bin
CMD [ "/usr/local/bin/potassium-shot-api" ]

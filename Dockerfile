FROM rust:1.95.0-slim AS builder
WORKDIR /usr/src/potassium-shot-api
COPY . .
RUN cargo build --release

FROM alpine:3.23.4
COPY --from=builder /usr/src/potassium-shot-api/target/release/potassium-shot-api /usr/local/bin
CMD [ "/usr/local/bin/potassium-shot-api" ]

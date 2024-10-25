FROM rust:alpine AS builder

WORKDIR /app

RUN apk update --no-cache && \
    apk upgrade --no-cache && \
    apk add musl-dev --no-cache

COPY Cargo.toml Cargo.lock ./
RUN mkdir src/ && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src src
RUN touch src/main.rs
RUN cargo build --release

FROM alpine:latest

WORKDIR /app
COPY --from=builder /app/target/release/sperve .

ENTRYPOINT ["./sperve"]
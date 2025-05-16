# 1️⃣ Stage: Build using musl
FROM rust:latest as builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

# 2️⃣ Stage: Run in minimal container
FROM alpine:latest

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /usr/local/bin/server

EXPOSE 3001

CMD ["server"]

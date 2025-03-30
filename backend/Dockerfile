FROM rust:1.85.1 AS builder

WORKDIR /app

RUN cargo new dummy

WORKDIR /app/dummy

COPY Cargo.toml .
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM debian:12.10-slim AS runner

COPY --from=builder /app/dummy/target/release/backend /usr/local/bin/backend

EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/backend"]
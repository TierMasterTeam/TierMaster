FROM rust:1.85.1 AS builder

WORKDIR /app

COPY Cargo.toml ./

RUN cargo new domain --lib
RUN cargo new application --lib
RUN cargo new data --lib
RUN cargo new presentation --lib
RUN cargo new core

COPY domain/Cargo.toml domain/Cargo.toml
COPY application/Cargo.toml application/Cargo.toml
COPY data/Cargo.toml data/Cargo.toml
COPY presentation/Cargo.toml presentation/Cargo.toml
COPY core/Cargo.toml core/Cargo.toml

RUN cargo build --release


COPY . .

RUN find core domain application data presentation -type f -name "*.rs" -exec touch {} \; \
 && cargo build --release


FROM debian:12.10-slim AS runner

COPY --from=builder /app/target/release/tiermaster-backend /usr/local/bin/tiermaster-backend

EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/tiermaster-backend"]
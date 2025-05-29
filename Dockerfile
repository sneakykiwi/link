FROM rust:1.85-alpine AS builder

RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY src ./src
COPY migrations ./migrations

ENV SQLX_OFFLINE=true

# Use --locked to ensure reproducible builds with exact dependency versions
RUN cargo build --release --locked

FROM alpine:3.20

RUN apk add --no-cache ca-certificates
RUN addgroup -g 1000 app && adduser -D -s /bin/sh -u 1000 -G app app

WORKDIR /app

COPY --from=builder /app/target/release/link-shortener-backend /usr/local/bin/link-shortener-backend
COPY --from=builder /app/migrations ./migrations

USER app

EXPOSE 8080

CMD ["link-shortener-backend"] 
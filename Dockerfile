FROM rust:1.76-alpine3.19 AS builder

WORKDIR /usr/src/pickacord

RUN apk add --no-cache libc-dev

COPY Cargo.toml Cargo.lock ./
COPY ./src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/pickacord/target \
    cargo build --release --locked && cp /usr/src/pickacord/target/release/pickacord /usr/local/bin/pickacord

FROM alpine:3.19

RUN apk add --no-cache ca-certificates
COPY --from=builder /usr/local/bin/pickacord /usr/local/bin/pickacord

CMD ["pickacord"]

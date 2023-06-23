FROM rust AS builder

WORKDIR /usr/src/pikacord
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=builder /usr/local/cargo/bin/pickacord /usr/local/bin/pickacord

CMD ["pickacord"]
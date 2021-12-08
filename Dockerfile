FROM rust:1.56.1 AS builder

WORKDIR /app
COPY ./server .
RUN cargo build --release

FROM debian:10.11

COPY --from=builder /app/target/release/actix_server /usr/bin/actix_server

ENTRYPOINT [ "actix_server" ]

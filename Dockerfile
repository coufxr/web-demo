FROM rust:1.77-slim-bookworm as builder

WORKDIR /app

COPY ./build/config /root/.cargo/config

COPY . .

RUN cargo install --path .


FROM debian:bookworm-slim

COPY ./build/config /root/.cargo/config

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/web-demo /usr/local/bin/web-demo

ENTRYPOINT ["web-demo"]

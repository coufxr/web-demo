FROM rust:1.77-slim-buster as builder

WORKDIR /app

COPY ./build/config /root/.cargo/config

# 将 Cargo.lock 和 Cargo.toml 添加到镜像中，确保依赖版本固定
COPY ./Cargo.lock ./Cargo.toml /app/

# 写入一个初始化
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 先安装依赖，这一步的缓存可以复用，只要 Cargo.lock 不变
RUN cargo build --release --locked

# 再复制源码并构建，如果只有源码变动，依赖层的缓存依然有效
COPY src ./src

RUN cargo install --path .


FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/web-demo ./web-demo

ENTRYPOINT ["/app/web-demo"]

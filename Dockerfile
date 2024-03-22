FROM rust:1.77-slim-buster as builder

RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list \
    && apt update  \
    && apt-get install -y libssl-dev pkg-config

WORKDIR /app
COPY ./build/config /root/.cargo/config

# 将 Cargo.lock 和 Cargo.toml 添加到镜像中，确保依赖版本固定
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

# 写入一个初始化
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 先安装依赖，这一步的缓存可以复用，只要 Cargo.lock 不变
RUN cargo build --release --locked

# 再复制源码并构建，如果只有源码变动，依赖层的缓存依然有效
COPY src ./src
RUN cargo build --release --locked

# RUN strip target/release/web-demo

FROM builder as release
WORKDIR /app

COPY --from=builder /app/target/release/web-demo ./web-demo

CMD ["./web-demo"]

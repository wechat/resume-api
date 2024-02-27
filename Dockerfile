# 使用官方 Rust 镜像作为基础镜像
FROM rust:1.71.0 as builder

# 设置工作目录
WORKDIR /app

# 复制整个项目到容器中
COPY . .

# 构建应用程序
RUN cargo build --release

# 使用另一个基础镜像作为最终镜像
FROM debian:bullseye-slim

# 设置工作目录
WORKDIR /app

# 复制构建好的可执行文件到最终镜像中
COPY --from=builder /app/target/release/resume-api .
COPY --from=builder /app/.env .
COPY --from=builder /app/data ./data

EXPOSE 3000

# 设置容器启动命令
CMD ["./resume-api"]

# Stage 1: Build Stage
FROM rust:latest AS builder

# Cài đặt các thư viện cần thiết
RUN apt-get update && apt-get install -y libpq-dev

# Tạo thư mục làm việc
WORKDIR /usr/src/app

# Sao chép các tập tin Cargo vào thư mục làm việc
COPY Cargo.toml Cargo.lock ./

# Sao chép mã nguồn vào thư mục làm việc
COPY . .

# Build dự án ở chế độ release
RUN cargo build --release

# Stage 2: Final Stage
FROM ubuntu:22.04

# Cài đặt thư viện cần thiết cho runtime
RUN apt-get update && apt-get install -y libpq-dev

# Tạo thư mục làm việc
WORKDIR /usr/src/app

# Sao chép file nhị phân từ giai đoạn build
COPY --from=builder /usr/src/app/target/release/clean-arch ./
COPY --from=builder /usr/src/app/.env .env
EXPOSE 9999
# Chạy ứng dụng
# Keep the container running
CMD ["sleep", "infinity"]
CMD ["./clean-arch"]


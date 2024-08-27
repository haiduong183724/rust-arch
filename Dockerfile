# Stage 1: Build the application
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs file and build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/clean-arch*

# Copy the source code
COPY . .

# Build the actual application
RUN cargo build --release

# Stage 2: Create the runtime image
FROM ubuntu:22.04

# Install required libraries
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/clean-arch .

# Command to run the binary
CMD ["./clean-arch"]

# Keep the container running
CMD ["sleep", "infinity"]
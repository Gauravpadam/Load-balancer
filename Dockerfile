# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml ./
COPY Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a minimal base image for the final stage
FROM ubuntu:22.04

# Install OpenSSL 3
RUN apt-get update && apt-get upgrade -y

# Set the working directory
WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/loadbalancer .

# Expose the port that the application will run on
EXPOSE 8080

# Run the application
CMD ["./loadbalancer"]
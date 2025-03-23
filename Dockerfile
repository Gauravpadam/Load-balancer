# Part 1 -- Building app
FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY Cargo.toml ./
COPY Cargo.lock ./

COPY src ./src

RUN cargo build --release

# Part 2 -- Running app
FROM ubuntu:22.04

RUN apt-get update && apt-get upgrade -y

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/load-balancing-server .

EXPOSE 5173

CMD ["./load-balancing-server"]
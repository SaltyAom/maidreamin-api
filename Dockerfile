# FROM rust:1.42 as builder

# # muslc is required in order to build the rust image.
# RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

# COPY . .
# RUN rustup target add x86_64-unknown-linux-musl
# # Sets the environment variable for the cargo build command that follows.
# ENV PKG_CONFIG_ALLOW_CROSS=1
# RUN cargo build --target x86_64-unknown-linux-musl --release

# FROM alpine:3.8

# RUN apk --no-cache add ca-certificates 
# COPY --from=builder /target/x86_64-unknown-linux-musl/release/dreamin-api .

# CMD ["/dreamin-api"]

# Build Stage
FROM rust:1.42 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new dreamin-api
WORKDIR /usr/src/dreamin-api
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

COPY src ./src
COPY static ./static

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle Stage
FROM alpine:3.8

COPY --from=builder /usr/local/cargo/bin/dreamin-api .
COPY static ./static

CMD ["./dreamin-api"]

EXPOSE 80
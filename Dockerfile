FROM rust:1.83 as builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

COPY Cargo.toml . 
COPY src ./src

RUN apt-get update && apt-get install -y libc6

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM debian:buster-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/MCserver-status .

CMD ["./MCserver-status"]


FROM rust:1.70-slim as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/app/target/release/rust-compressor /usr/local/bin/compress

ENTRYPOINT ["compress"]
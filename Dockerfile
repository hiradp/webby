FROM rust:1-stretch as builder

WORKDIR /usr/webby
RUN rustup default nightly && rustup update
RUN USER=root cargo init
COPY Cargo.* /usr/webby/
COPY src src
COPY templates templates
RUN cargo build --release

FROM debian:stretch-slim
COPY --from=builder /usr/webby/target/release/webby /bin/

EXPOSE 8000
CMD webby

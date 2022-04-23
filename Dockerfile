FROM rust:bullseye AS builder

WORKDIR /usr/src/ding-server

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN touch src/main.rs
RUN cargo build --release

RUN strip target/release/ding-server

FROM debian:bullseye

RUN apt-get update
RUN apt-get -y install ca-certificates

WORKDIR /config
COPY --from=builder /usr/src/ding-server/target/release/ding-server /usr/local/bin/ding-server

ENTRYPOINT ["/usr/local/bin/ding-server"]

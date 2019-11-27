FROM rust:1.39.0-slim-buster as builder

WORKDIR /usr/src/app

RUN apt-get update -qq && apt-get install -yq ca-certificates musl-dev \
    && rustup target add x86_64-unknown-linux-musl \
    && mkdir -p src \
    && echo "fn main() {}" >> src/main.rs

COPY Cargo.toml .
COPY Cargo.lock .

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/echo /echo
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

ENTRYPOINT ["/echo"]

# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:1.43 as rcon-build

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y git

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src

# COPY Cargo.toml Cargo.toml

# RUN mkdir src/

# RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/rcon*

COPY ./ /usr/src/rcon

WORKDIR /usr/src/rcon

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 rcon

RUN adduser -D -s /bin/sh -u 1000 -G rcon rcon

WORKDIR /home/rcon/bin/

EXPOSE 3030

COPY --from=rcon-build /usr/src/rcon/target/x86_64-unknown-linux-musl/release/rcon .

RUN chown rcon:rcon rcon

USER rcon

CMD ["./rcon"]

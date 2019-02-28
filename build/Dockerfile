FROM rust

RUN apt-get update && apt-get install -y \
  curl \
  gcc-arm-linux-gnueabihf

ADD src /src
ADD target /target
ADD .cargo /.cargo
ADD Cargo.toml /

RUN rustup target add armv7-unknown-linux-gnueabihf



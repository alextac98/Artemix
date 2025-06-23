FROM rust:1.85

RUN mkdir /artemix
RUN chmod -R a+rwx /artemix
WORKDIR /artemix

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
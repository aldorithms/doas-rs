FROM rust:latest

WORKDIR /usr/src/doas-rs
COPY . .

RUN cargo install --path .

CMD ["doas-rs"]

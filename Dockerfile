FROM rust:1.73

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path crates/api

CMD ["api"]

FROM rust:1.61-buster

WORKDIR /app
COPY . .

RUN cargo install --path .

CMD ["cargo", "run"]

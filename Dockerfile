FROM rust:1.79-bookworm as build

RUN cargo new api
WORKDIR /api
COPY ./Cargo.toml .
COPY ./Cargo.lock .
RUN cargo build -r
RUN rm src/*.rs target/release/api

COPY . .
RUN touch src/main.rs
RUN cargo build -r

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y sqlite3 && rm -rf /var/lib/apt/lists/*
COPY --from=build /api/target/release/api .
EXPOSE 2004

CMD ./api

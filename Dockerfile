FROM rust:1.79 as build

# cache dependencies
RUN cargo new api
WORKDIR /api
COPY ./Cargo.toml .
COPY ./Cargo.lock .
RUN cargo build -r
RUN rm src/*.rs

COPY . .

EXPOSE 2004

RUN cargo build -r

FROM debian:bookworm-slim

COPY --from=build /api/target/release/api .

CMD ./api

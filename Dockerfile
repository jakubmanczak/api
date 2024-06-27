FROM rust:1.79

WORKDIR /rustapi

COPY . .

EXPOSE 2004

RUN cargo build -r

CMD cargo run -r

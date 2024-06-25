FROM rust:1.79

WORKDIR /rustapi

COPY . .

EXPOSE 2004

CMD ["cargo", "run", "-r"]

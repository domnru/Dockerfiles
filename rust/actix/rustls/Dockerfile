FROM rust:latest AS build-env
WORKDIR /app


ADD src src
ADD Cargo.toml Cargo.toml

RUN cargo build --release

FROM ubuntu:latest AS prod-env
WORKDIR /app

COPY --from=build-env /app/target/release/hello-world hello-world

CMD [ "/app/hello-world" ]

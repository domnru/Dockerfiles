FROM rust:latest AS build-env
WORKDIR /app


ADD src src
ADD Cargo.toml Cargo.toml

# Install Dependencies
RUN apt update && apt install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch AS prod-env
WORKDIR /app

COPY --from=build-env /app/target/x86_64-unknown-linux-musl/release/hello-world hello-world

CMD [ "/app/hello-world" ]

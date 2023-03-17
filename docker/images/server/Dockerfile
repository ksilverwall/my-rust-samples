FROM rust:latest as builder

WORKDIR /usr/src/myapp

COPY Cargo.toml Cargo.lock ./
COPY src ./src/

RUN cargo build --release


FROM rust:latest
RUN apt-get update && apt-get -y install libssl1.1

COPY --from=builder /usr/src/myapp/target/release/ /usr/local/bin/

WORKDIR /work
CMD ["server"]

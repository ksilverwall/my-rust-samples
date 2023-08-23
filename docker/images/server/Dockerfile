FROM rust:latest as builder

WORKDIR /usr/src/myapp

ARG SRC_DIR

COPY ${SRC_DIR}/Cargo.toml ${SRC_DIR}/Cargo.lock ./
RUN mkdir -p src/bin
RUN echo "fn main() {}" > ./src/bin/server.rs
RUN echo "fn main() {}" > ./src/bin/client.rs
RUN cargo build --release
RUN rm -r ./src
COPY server/src ./src/

RUN cargo build --release

FROM rust:latest
RUN apt-get update && apt-get -y install libssl1.1

COPY --from=builder /usr/src/myapp/target/release/ /usr/local/bin/

WORKDIR /work
CMD ["server"]

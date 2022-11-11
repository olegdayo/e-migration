FROM rust:latest
WORKDIR /krusty
COPY . .

RUN cargo build --release
CMD ["./target/release/krusty"]

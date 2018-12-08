FROM rust:1.30.1

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/primarch"]

FROM rust:1.67
EXPOSE 80
EXPOSE 443

RUN apt update && apt install -y protobuf-compiler
WORKDIR /app
COPY . .

RUN cargo build --release --bin grpc-server

CMD ["./target/release/grpc-server"]
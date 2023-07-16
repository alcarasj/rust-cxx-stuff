FROM rust:1.67 as builder
RUN apt update && apt install -y protobuf-compiler
WORKDIR /repo
COPY . .
RUN cargo build --release --bin grpc-server

FROM scratch
EXPOSE 80
EXPOSE 443
COPY --from=builder . .
WORKDIR /repo
CMD ["./target/release/grpc-server"]
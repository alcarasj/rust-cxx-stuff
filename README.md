# rust-cxx-stuff
Exploring Rust and C++ interoperability using [CXX](https://cxx.rs/).

## Run using Docker
1. Build the image with `docker build -t rust-cxx-stuff .`
2. Start the container with `docker run -it --rm --name rust-cxx-stuff -p 8080:50051 rust-cxx-stuff`
3. Make a request using the client with `cargo run --bin grpc-client`
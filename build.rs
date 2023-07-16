fn main() -> Result<(), Box<dyn std::error::Error>> {
    cxx_build::bridge("src/server.rs")
        .file("src/native.cc")
        .flag_if_supported("-std=c++14")
        .compile("rust-cxx-stuff");
    tonic_build::compile_protos("protos/greet.proto")?;
    Ok(())
}
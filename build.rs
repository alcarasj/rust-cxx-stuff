fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/native.cc")
        .compile("rust-cxx-stuff");
}
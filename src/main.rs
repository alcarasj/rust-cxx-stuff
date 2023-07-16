#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("rust-cxx-stuff/src/native.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
    }
}

fn main() {
    let _client = ffi::new_blobstore_client();
}
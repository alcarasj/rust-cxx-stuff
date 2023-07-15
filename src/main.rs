#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("rust-cxx-stuff/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
    }
}

fn main() {
    let client = ffi::new_blobstore_client();
}
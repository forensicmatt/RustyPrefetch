use std::env;
fn main() {
    // LIBSCCA_BIN
    let library_path = env::var("LIBSCCA_BIN").unwrap();

    println!("cargo:rustc-link-search={}", library_path);
}

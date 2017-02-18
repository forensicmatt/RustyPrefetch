use std::env;
fn main() {
    // LIBFWNT_BIN
    let library_path = match env::var("LIBFWNT_BIN") {
        Ok(library_path) => library_path,
        Err(error) => panic!("{}", error)
    };
    println!("cargo:rustc-link-search={}", library_path);
}

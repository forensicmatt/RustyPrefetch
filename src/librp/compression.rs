use libc::{c_int,size_t,intptr_t};
#[link(name="libfwnt",kind="dylib")]
extern {
    fn libfwnt_lzxpress_huffman_decompress(
        compressed_data: *const u8,
        compressed_data_size: size_t,
        uncompressed_data: *mut u8,
        uncompressed_data_size: *mut size_t,
        error: *mut intptr_t
    ) -> c_int;
}

pub fn decompress(compressed_data: &[u8],uncompressed_size: usize) -> Option<Vec<u8>> {
    unsafe {
        // Get size of compressed data
        let compressed_data_size = compressed_data.len() as size_t;

        // Create compressed data pointer
        let compressed_data_ptr = compressed_data.as_ptr();

        let mut uncompressed_data_size: size_t = uncompressed_size;

        // To hold uncompressed data
        let mut uncompressed_data: Vec<u8> = Vec::with_capacity(
            uncompressed_data_size as usize
        );
        let uncompressed_data_ptr = uncompressed_data.as_mut_ptr();

        // Error ptr... not sure if this is how it should be handled...
        let mut error: intptr_t = 0;

        let result: c_int = libfwnt_lzxpress_huffman_decompress(
            compressed_data_ptr,
            compressed_data_size,
            uncompressed_data_ptr,
            &mut uncompressed_data_size,
            &mut error
        );

        if result == 1 {
            uncompressed_data.set_len(
                uncompressed_data_size as usize
            );
            Some(uncompressed_data)
        } else {
            // TODO Handle Errors. -1 == ERROR
            None
        }
    }
}

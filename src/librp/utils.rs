use std::fmt;

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    strs.join("")
}

// A type for larger byte arrays so we can customize serialization for output
#[derive(Serialize)]
pub struct ByteArray(pub Vec<u8>);
impl fmt::Debug for ByteArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            to_hex_string(&self.0),
        )
    }
}

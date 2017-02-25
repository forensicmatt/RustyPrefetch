use std::fmt;
use serde::{ser};

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    strs.join("")
}

// A type for larger byte arrays so we can customize serialization for output
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
impl ser::Serialize for ByteArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(&format!("{}", to_hex_string(&self.0)))
    }
}

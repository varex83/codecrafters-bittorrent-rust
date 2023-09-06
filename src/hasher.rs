use sha1::{Digest, Sha1};

pub fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha1::default();
    hasher.update(bytes);
    bytes_to_hex(&hasher.finalize())
}

pub fn hash_bytes_urlencode(bytes: &[u8]) -> String {
    let mut hasher = Sha1::default();
    hasher.update(bytes);

    let mut result = String::new();
    for byte in hasher.finalize() {
        result.push_str(format!("%{:02x}", byte).as_str());
    }

    result
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_bytes() {
        let bytes = b"hello world";
        let expected = "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed";
        let actual = hash_bytes(bytes);
        assert_eq!(expected, actual);
    }
}

pub(crate) fn encode_utf8_scalar(input: &str) -> Vec<u8> {
    input.as_bytes().to_vec()
}

pub fn encode_utf8(input: &str) -> Vec<u8> {
    encode_utf8_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_utf8() {
        let input = "Utf8 test";
        let encoded = encode_utf8(input);
        assert_eq!(encoded, b"Utf8 test");
    }
}

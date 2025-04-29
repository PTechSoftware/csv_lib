
pub(crate) fn encode_windows1252_scalar(input: &str) -> Vec<u8> {
    input.chars().map(|c| {
        if c as u32 <= 0xFF {
            c as u8
        } else {
            b'?'
        }
    }).collect()
}

pub fn encode_windows1252(input: &str) -> Vec<u8> {
    encode_windows1252_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_windows1252() {
        let input = "Hello€";
        let encoded = encode_windows1252(input);
        assert_eq!(encoded[0], b'H');
    }
}

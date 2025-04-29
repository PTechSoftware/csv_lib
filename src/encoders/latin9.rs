
pub(crate) fn encode_latin9_scalar(input: &str) -> Vec<u8> {
    input.chars().map(|c| {
        if c as u32 <= 0xFF {
            c as u8
        } else {
            b'?'
        }
    }).collect()
}

pub fn encode_latin9(input: &str) -> Vec<u8> {
    encode_latin9_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_latin9() {
        let input = "Euro €";
        let encoded = encode_latin9(input);
        assert_eq!(encoded[0], b'E');
    }
}

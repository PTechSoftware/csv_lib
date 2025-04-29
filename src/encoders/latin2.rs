
pub(crate) fn encode_latin2_scalar(input: &str) -> Vec<u8> {
    input.chars().map(|c| {
        if c as u32 <= 0xFF {
            c as u8
        } else {
            b'?'
        }
    }).collect()
}

pub fn encode_latin2(input: &str) -> Vec<u8> {
    encode_latin2_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_latin2() {
        let input = "Test";
        let encoded = encode_latin2(input);
        assert_eq!(encoded, b"Test");
    }
}

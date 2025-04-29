
pub(crate) fn encode_latin1_scalar(input: &str) -> Vec<u8> {
    input.chars().map(|c| {
        if c as u32 <= 0xFF {
            c as u8
        } else {
            b'?'
        }
    }).collect()
}

pub fn encode_latin1(input: &str) -> Vec<u8> {
    encode_latin1_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_latin1() {
        let input = "Héllo";
        let encoded = encode_latin1(input);
        assert_eq!(encoded, vec![b'H', 0xE9, b'l', b'l', b'o']);
    }
}

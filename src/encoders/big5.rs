
pub(crate) fn encode_big5_scalar(input: &str) -> Vec<u8> {
    input.chars().map(|c| {
        if c.is_ascii() {
            c as u8
        } else {
            b'?'
        }
    }).collect()
}

pub fn encode_big5(input: &str) -> Vec<u8> {
    encode_big5_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_big5() {
        let input = "Hello!";
        let encoded = encode_big5(input);
        assert_eq!(encoded, b"Hello!");
    }
}

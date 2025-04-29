
pub(crate) fn encode_koi8r_scalar(input: &str) -> Vec<u8> {
    input.chars().map(|c| {
        if c.is_ascii() {
            c as u8
        } else {
            b'?'
        }
    }).collect()
}

pub fn encode_koi8r(input: &str) -> Vec<u8> {
    encode_koi8r_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_koi8r() {
        let input = "Test!";
        let encoded = encode_koi8r(input);
        assert_eq!(encoded, b"Test!");
    }
}

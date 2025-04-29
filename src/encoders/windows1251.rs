
pub(crate) fn encode_windows1251_scalar(input: &str) -> Vec<u8> {
    input.chars().map(|c| {
        if c.is_ascii() {
            c as u8
        } else {
            b'?'
        }
    }).collect()
}

pub fn encode_windows1251(input: &str) -> Vec<u8> {
    encode_windows1251_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_windows1251() {
        let input = "Win1251";
        let encoded = encode_windows1251(input);
        assert_eq!(encoded, b"Win1251");
    }
}

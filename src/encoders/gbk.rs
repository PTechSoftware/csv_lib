
pub(crate) fn encode_gbk_scalar(input: &str) -> Vec<u8> {
    input.chars().map(|c| {
        if c.is_ascii() {
            c as u8
        } else {
            b'?'
        }
    }).collect()
}

pub fn encode_gbk(input: &str) -> Vec<u8> {
    encode_gbk_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_gbk() {
        let input = "World!";
        let encoded = encode_gbk(input);
        assert_eq!(encoded, b"World!");
    }
}

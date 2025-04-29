
pub(crate) fn encode_shiftjis_scalar(input: &str) -> Vec<u8> {
    input.chars().map(|c| {
        if c.is_ascii() {
            c as u8
        } else {
            b'?'
        }
    }).collect()
}

pub fn encode_shiftjis(input: &str) -> Vec<u8> {
    encode_shiftjis_scalar(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_shiftjis() {
        let input = "Shift";
        let encoded = encode_shiftjis(input);
        assert_eq!(encoded, b"Shift");
    }
}

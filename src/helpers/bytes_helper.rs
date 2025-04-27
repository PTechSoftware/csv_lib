use memchr::memchr2;

pub fn find_line_break(slice: &[u8],cursor:usize) -> Option<(usize, usize)> {
    memchr2(b'\n', b'\r', slice).map(|i| {
        if slice[i] == b'\r' && slice.get(i + 1) == Some(&b'\n') {
            (cursor, cursor + i + 2) // \r\n
        } else {
            (cursor, cursor + i + 1) // \n o \r solo
        }
    })
}
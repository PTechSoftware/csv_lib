use memmap2::Mmap;

pub fn find_line_break(map: &Mmap, cursor: usize, new_line: u8) -> Option<(usize, usize)> {
    // if the cursor es less than the file lenght, it has no sense, return none
    if cursor >= map.len() {
        return None;
    }
    // Get Reference of bytes of file
    let slice = &map[cursor..];

    // Buscamos el próximo salto de línea
    if let Some(rel_end) = slice.iter().position(|&b| b == new_line) {
        let end = cursor + rel_end + 1;
        return Some((cursor, end));
    }
    None
}
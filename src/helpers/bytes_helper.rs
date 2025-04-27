use memchr::{memchr2, memchr3};

#[allow(unreachable_code)]
/// ## Line break function.
///
/// * Intends to capture the position where the line ends.
/// * Needs to receive the cursor position.
///
/// Versions:
/// - memchr2
/// - AVX2 (Advanced Vector Extensions 2)
/// - NEON (for arm, like MacBooks)
///
/// `return` => The position where the next line ends.
pub fn find_line_break(slice: &[u8], cursor: usize, separator: u8) -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { locate_line_break_avx2(slice, cursor, separator) };
        }
    }
    // Fallback for any other CPU
    locate_line_break_memchr2(slice, cursor, separator)
}

//=================================================================//
//=====================   PRIVATE FUNCTIONS   =====================//
//=================================================================//

/// Locates the line break, using memchr2. Is compatible with CPU made before 2013. (Don't have AVX2 compatibility)
fn locate_line_break_memchr2(slice: &[u8], cursor:usize, separator: u8) -> usize {
    let r = if separator == b'\r' || separator == b'\n' {
        memchr2(b'\n', b'\r', slice).map(|i| {
            if slice[i] == b'\r' && slice.get(i + 1) == Some(&b'\n') {
                cursor + i + 2
            }else {
                cursor + i + 1
            }
        })
    }else {
        memchr3(b'\n', b'\r',separator, slice).map(|i| {
            if slice[i] == b'\r' && slice.get(i + 1) == Some(&b'\n') {
                cursor + i + 2
            }else {
                cursor + i + 1
            }
        })
    };
    match r {
        None => {0 },
        Some(i) => { i}
    }
}


/// AVX2 Compatible function, to find the line break.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn locate_line_break_avx2(slice: &[u8], cursor:usize, separator: u8) -> usize {
    let len = slice.len();
    let needle = _mm256_set1_epi8(line_break as i8);
    let mut i = 0;

    while i + 32 <= len {
        let chunk = _mm256_loadu_si256(buffer.as_ptr().add(i) as *const __m256i);
        let cmp = _mm256_cmpeq_epi8(chunk, needle);
        let mask = _mm256_movemask_epi8(cmp);

        if mask != 0 {
            // Encontramos un match
            let first_match = mask.trailing_zeros() as usize;
            return i + first_match;
        }

        i += 32;
    }

    // Procesar el resto
    while i < len {
        if buffer[i] == line_break {
            return i;
        }
        i += 1;
    }

    // No encontrado
    buffer.len()
}

#[cfg(target_arch = "aarch64")]
#[allow(dead_code)]
unsafe fn locate_line_break_neon(buffer: &[u8],cursor: usize, line_break: u8) -> usize {
    use std::arch::aarch64::*;
    // Get the buffer size
    let len = buffer.len();
    // Set the index mutable
    let mut i = cursor;
    // Get a uint8x16_t filled whith the bytes of the line break
    let fill = unsafe{ vdupq_n_u8(line_break) };
    // Check taking 16 bytes en each round
    while i + 16 <= len {
        //Load the 16 bytes to SIMD
        let chunk = unsafe {vld1q_u8(buffer.as_ptr().add(i)) };
        // Make vectorial comparision in this 16 bytes, against the bytes of line break
        let cmp = unsafe{ vceqq_u8(chunk, fill) };
        // Search max value in vector. No result , returns 0.
        let mask = unsafe{ vmaxvq_u8(cmp) };
        //If we have  a result
        if mask != 0 {
            // Temporal array to handle result of comparison
            let mut tmp = [0u8; 16];
            // Store the result in the temporal array
            unsafe{ vst1q_u8(tmp.as_mut_ptr(), cmp) };
            // search the first coincidence in the array
            for (j, &val) in tmp.iter().enumerate() {
                //if match, returns the position
                if val == 0xFF {
                    return i + j;
                }
            }
        }
        i += 16;
    }
    // Traditional byte a byte process for the rest of bytes not handled by Single Instruction Multiple Data (SIMD)
    while i < len {
        // Compare the index in the buffer , with line break literal u8
        if buffer[i] == line_break {
            //Returns the position
            return i;
        }
        // Iter
        i += 1;
    }
    // Return
    buffer.len()
}


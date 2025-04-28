use memchr::{memchr2, memchr3};
//=================================================================//
//=====================   PUBLIC  FUNCTIONS   =====================//
//=================================================================//

/// ## Locate Line Break using Memchr3
/// Locates the line break, using memchr2. Is compatible with CPU made before 2013. (Don't have AVX2 compatibility)
pub fn locate_line_break_memchr3(slice: &[u8], cursor: usize, separator: u8) -> usize {
    let r = if separator == b'\r' || separator == b'\n' {
        memchr2(b'\n', b'\r', slice).map(|i| {
            if slice[i] == b'\r' && slice.get(i + 1) == Some(&b'\n') {
                cursor + i + 2
            } else {
                cursor + i + 1
            }
        })
    } else {
        memchr3(b'\n', b'\r', separator, slice).map(|i| {
            if slice[i] == b'\r' && slice.get(i + 1) == Some(&b'\n') {
                cursor + i + 2
            } else {
                cursor + i + 1
            }
        })
    };
    r.unwrap_or_else(|| 0)
}

/// ## Locate Line Break AVX2
/// AVX2 Compatible function, to find the line break.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn locate_line_break_avx2(slice: &[u8], cursor: usize, separator: u8) -> usize {
    use std::arch::x86_64::*;

    let len = slice.len();
    let mut i = cursor;

    // Crear patrones SIMD
    let pat_n = _mm256_set1_epi8(b'\n' as i8);
    let pat_r = _mm256_set1_epi8(b'\r' as i8);
    let pat_sep = _mm256_set1_epi8(separator as i8);

    while i + 32 <= len {
        let chunk = _mm256_loadu_si256(slice.as_ptr().add(i) as *const __m256i);

        // Comparaciones paralelas
        let cmp_n = _mm256_cmpeq_epi8(chunk, pat_n);
        let cmp_r = _mm256_cmpeq_epi8(chunk, pat_r);
        let cmp_sep = _mm256_cmpeq_epi8(chunk, pat_sep);

        // Masks
        let mask_n = _mm256_movemask_epi8(cmp_n);
        let mask_r = _mm256_movemask_epi8(cmp_r);
        let mask_sep = _mm256_movemask_epi8(cmp_sep);

        // Si hay algún match
        if mask_n != 0 || mask_r != 0 || mask_sep != 0 {
            let mut first = 32usize; // grande para ir comparando
            let mut source = 0; // tipo de match encontrado (1: \r, 2: \n, 3: sep)

            // Buscar el primero entre los tres
            if mask_r != 0 {
                let pos_r = mask_r.trailing_zeros() as usize;
                if pos_r < first {
                    first = pos_r;
                    source = 1;
                }
            }

            if mask_n != 0 {
                let pos_n = mask_n.trailing_zeros() as usize;
                if pos_n < first {
                    first = pos_n;
                    source = 2;
                }
            }

            if mask_sep != 0 {
                let pos_sep = mask_sep.trailing_zeros() as usize;
                if pos_sep < first {
                    first = pos_sep;
                    source = 3;
                }
            }

            match source {
                1 => {
                    // Encontramos \r
                    if i + first + 1 < len && slice[i + first + 1] == b'\n' {
                        return i + first + 2;
                    } else {
                        return i + first + 1;
                    }
                }
                2 => {
                    // Encontramos \n
                    return i + first + 1;
                }
                3 => {
                    // Encontramos separator
                    return i + first + 1;
                }
                _ => unreachable!(),
            }
        }

        i += 32;
    }

    // Procesar el resto byte a byte
    while i < len {
        match slice[i] {
            b'\r' => {
                if i + 1 < len && slice[i + 1] == b'\n' {
                    return i + 2;
                } else {
                    return i + 1;
                }
            }
            b'\n' => {
                return i + 1;
            }
            sep if sep == separator => {
                return i + 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    slice.len()
}

/// ## Locate Line Break NEON
///
/// - Finds the next line break, using aarch64 feature NEON.
#[cfg(target_arch = "aarch64")]
#[allow(dead_code, unsafe_code)]
pub unsafe fn locate_line_break_neon(buffer: &[u8], separator: u8) -> usize {
    unsafe {
        use std::arch::aarch64::*;

        let mut i = 0;
        let len = buffer.len();

        let pattern_n = vdupq_n_u8(b'\n');
        let pattern_r = vdupq_n_u8(b'\r');
        // Only create separator pattern if separator is different from \r and \n
        let check_separator = separator != b'\r' && separator != b'\n';
        let pattern_sep = if check_separator {
            Some(vdupq_n_u8(separator))
        } else {
            None
        };

        while i + 16 <= len {
            let chunk = vld1q_u8(buffer.as_ptr().add(i));
            let cmp_n = vceqq_u8(chunk, pattern_n);
            let cmp_r = vceqq_u8(chunk, pattern_r);
            let cmp_sep = if let Some(pattern_sep) = pattern_sep {
                Some(vceqq_u8(chunk, pattern_sep))
            } else {
                None
            };

            let mask_n = vmaxvq_u8(cmp_n);
            let mask_r = vmaxvq_u8(cmp_r);
            let mask_sep = cmp_sep.map_or(0, |c| vmaxvq_u8(c));

            if mask_n != 0 || mask_r != 0 || mask_sep != 0 {
                let mut tmp_n = [0u8; 16];
                let mut tmp_r = [0u8; 16];
                let mut tmp_sep = [0u8; 16];

                vst1q_u8(tmp_n.as_mut_ptr(), cmp_n);
                vst1q_u8(tmp_r.as_mut_ptr(), cmp_r);
                if let Some(cmp) = cmp_sep {
                    vst1q_u8(tmp_sep.as_mut_ptr(), cmp);
                }

                for j in 0..16 {
                    if tmp_r[j] == 0xFF {
                        if i + j + 1 < len && buffer[i + j + 1] == b'\n' {
                            return i + j + 2; // \r\n combo
                        } else {
                            return i + j + 1; // Solo \r
                        }
                    } else if tmp_n[j] == 0xFF {
                        return i + j + 1; // Solo \n
                    } else if check_separator && tmp_sep[j] == 0xFF {
                        return i + j + 1; // Custom separator
                    }
                }
            }

            i += 16;
        }
        while i < len {
            match buffer[i] {
                b'\r' if i + 1 < len && buffer[i + 1] == b'\n' => return i + 2,
                b'\r' | b'\n' => return i + 1,
                byte if check_separator && byte == separator => return i + 1,
                _ => i += 1,
            }
        }

        buffer.len()
    }
}
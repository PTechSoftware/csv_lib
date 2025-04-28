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

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe  fn locate_line_break_avx2(buffer: &[u8], separator: u8) -> usize {
    unsafe {
        use std::arch::x86_64::*;

        let mut i = 0;
        let len = buffer.len();

        let pattern_n = _mm256_set1_epi8(b'\n' as i8);
        let pattern_r = _mm256_set1_epi8(b'\r' as i8);
        let check_separator = separator != b'\r' && separator != b'\n';
        let pattern_sep = if check_separator {
            Some(_mm256_set1_epi8(separator as i8))
        } else {
            None
        };

        while i + 32 <= len {
            let chunk = _mm256_loadu_si256(buffer.as_ptr().add(i) as *const __m256i);

            let cmp_n = _mm256_cmpeq_epi8(chunk, pattern_n);
            let cmp_r = _mm256_cmpeq_epi8(chunk, pattern_r);
            let cmp_sep = if let Some(pattern_sep) = pattern_sep {
                Some(_mm256_cmpeq_epi8(chunk, pattern_sep))
            } else {
                None
            };

            let mask_n = _mm256_movemask_epi8(cmp_n);
            let mask_r = _mm256_movemask_epi8(cmp_r);
            let mask_sep = cmp_sep.map_or(0, |c| _mm256_movemask_epi8(c));

            if mask_n != 0 || mask_r != 0 || mask_sep != 0 {
                for j in 0..32 {
                    let pos = i + j;
                    if pos >= len {
                        break;
                    }
                    match buffer[pos] {
                        b'\r' => {
                            if pos + 1 < len && buffer[pos + 1] == b'\n' {
                                return pos + 2;
                            } else {
                                return pos + 1;
                            }
                        }
                        b'\n' => return pos + 1,
                        byte if check_separator && byte == separator => return pos + 1,
                        _ => {}
                    }
                }
            }

            i += 32;
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
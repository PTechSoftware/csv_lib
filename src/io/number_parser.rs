//! simd_parse: SIMD-powered numeric parsing for &[u8] slices

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;


fn parse_u64_fallback(bytes: &[u8]) -> u64 {
    let mut result = 0u64;
    for &b in bytes {
        result = result * 10 + (b - b'0') as u64;
    }
    result
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn parse_u64_avx2(bytes: &[u8]) -> u64 {
    parse_u32_avx2(bytes) as u64
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn parse_u64_neon(bytes: &[u8]) -> u64 {
    parse_u32_neon(bytes) as u64
}

#[inline(always)]
pub fn parse_u64(bytes: &[u8]) -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return unsafe { parse_u64_avx2(bytes) };
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        return unsafe { parse_u64_neon(bytes) };
    }
    parse_u64_fallback(bytes)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn parse_u32_avx2(bytes: &[u8]) -> u32 {
    let ptr = bytes.as_ptr();
    let zero = unsafe { _mm256_set1_epi8(b'0' as i8) };
    let raw = unsafe { _mm256_loadu_si256(ptr as *const __m256i) };
    let digits = unsafe { _mm256_sub_epi8(raw, zero) };
    let mut temp: [u8; 32] = [0; 32];
    unsafe { _mm256_storeu_si256(temp.as_mut_ptr() as *mut __m256i, digits) };
    let mut result = 0u32;
    for &d in &temp[..bytes.len()] {
        result = result * 10 + d as u32;
    }
    result
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
fn parse_u32_neon(bytes: &[u8]) -> u32 {
    let ptr = bytes.as_ptr();
    let input = unsafe { vld1q_u8(ptr) };
    let zero = unsafe { vdupq_n_u8(b'0') };
    let digits = unsafe { vsubq_u8(input, zero) };
    let mut temp = [0u8; 16];
    unsafe { vst1q_u8(temp.as_mut_ptr(), digits) };
    let mut result = 0u32;
    for &d in &temp[..bytes.len()] {
        result = result * 10 + d as u32;
    }
    result
}

#[inline(always)]
fn parse_u32_fallback(bytes: &[u8]) -> u32 {
    let mut result = 0u32;
    for &b in bytes {
        result = result * 10 + (b - b'0') as u32;
    }
    result
}

#[inline(always)]
pub fn parse_u32(bytes: &[u8]) -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return unsafe { parse_u32_avx2(bytes) };
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        return parse_u32_neon(bytes);
    }
    parse_u32_fallback(bytes)
}

#[inline(always)]
fn parse_i8_fallback(bytes: &[u8]) -> i8 {
    let mut result = 0i8;
    let mut neg = false;
    let mut i = 0;
    if bytes.get(0) == Some(&b'-') { neg = true; i += 1; }
    while i < bytes.len() {
        result = result * 10 + (bytes[i] - b'0') as i8;
        i += 1;
    }
    if neg { -result } else { result }
}

#[inline(always)]
pub fn parse_i8(bytes: &[u8]) -> i8 {
    parse_i8_fallback(bytes)
}

#[inline(always)]
fn parse_i16_fallback(bytes: &[u8]) -> i16 {
    let mut result = 0i16;
    let mut neg = false;
    let mut i = 0;
    if bytes.get(0) == Some(&b'-') { neg = true; i += 1; }
    while i < bytes.len() {
        result = result * 10 + (bytes[i] - b'0') as i16;
        i += 1;
    }
    if neg { -result } else { result }
}

#[inline(always)]
pub fn parse_i16(bytes: &[u8]) -> i16 {
    parse_i16_fallback(bytes)
}

#[inline(always)]
fn parse_i32_fallback(bytes: &[u8]) -> i32 {
    let mut result = 0i32;
    let mut neg = false;
    let mut i = 0;
    if bytes.get(0) == Some(&b'-') { neg = true; i += 1; }
    while i < bytes.len() {
        result = result * 10 + (bytes[i] - b'0') as i32;
        i += 1;
    }
    if neg { -result } else { result }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn parse_i32_avx2(bytes: &[u8]) -> i32 { unsafe {
    parse_u32_avx2(bytes) as i32 
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[inline(always)]
fn parse_i32_neon(bytes: &[u8]) -> i32 {
    parse_u32_neon(bytes) as i32
}

#[inline(always)]
pub fn parse_i32(bytes: &[u8]) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return unsafe { parse_i32_avx2(bytes) };
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        return parse_i32_neon(bytes);
    }
    parse_i32_fallback(bytes)
}

#[inline(always)]
fn parse_i64_fallback(bytes: &[u8]) -> i64 {
    let mut result = 0i64;
    let mut neg = false;
    let mut i = 0;
    if bytes.get(0) == Some(&b'-') { neg = true; i += 1; }
    while i < bytes.len() {
        result = result * 10 + (bytes[i] - b'0') as i64;
        i += 1;
    }
    if neg { -result } else { result }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn parse_i64_avx2(bytes: &[u8]) -> i64 {
    unsafe {
        parse_u32_avx2(bytes) as i64
    }
}
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[inline(always)]
unsafe fn parse_i64_neon(bytes: &[u8]) -> i64 {
    parse_u32_neon(bytes) as i64
}

#[inline(always)]
pub fn parse_i64(bytes: &[u8]) -> i64 {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return unsafe { parse_i64_avx2(bytes) };
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        return parse_i64_neon(bytes);
    }
    parse_i64_fallback(bytes)
}

#[inline(always)]
fn parse_f32_fallback(bytes: &[u8]) -> f32 {
    let s = unsafe { std::str::from_utf8_unchecked(bytes) };
    s.parse::<f32>().unwrap_or(0.0)
}

#[inline(always)]
fn parse_f64_fallback(bytes: &[u8]) -> f64 {
    let s = unsafe { std::str::from_utf8_unchecked(bytes) };
    s.parse::<f64>().unwrap_or(0.0)
}

#[inline(always)]
pub fn parse_f32(bytes: &[u8]) -> f32 {
    parse_f32_fallback(bytes)
}

#[inline(always)]
pub fn parse_f64(bytes: &[u8]) -> f64 {
    parse_f64_fallback(bytes)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_u32() {
        assert_eq!(parse_u32(b"0"), 0);
        assert_eq!(parse_u32(b"123"), 123);
        assert_eq!(parse_u32(b"4294967295"), 4294967295);
    }

    #[test]
    fn test_parse_i32() {
        assert_eq!(parse_i32(b"0"), 0);
        assert_eq!(parse_i32(b"123"), 123);
        assert_eq!(parse_i32(b"-123"), -123);
        assert_eq!(parse_i32(b"2147483647"), 2147483647);
        assert_eq!(parse_i32(b"-2147483648"), -2147483648);
    }

    #[test]
    fn test_parse_f32() {
        assert_eq!(parse_f32(b"0.0"), 0.0);
        assert_eq!(parse_f32(b"3.14"), 3.14);
        assert_eq!(parse_f32(b"-2.5"), -2.5);
        assert_eq!(parse_f32(b"1e10"), 1e10);
    }

    #[test]
    fn test_parse_f64() {
        assert_eq!(parse_f64(b"0.0"), 0.0);
        assert_eq!(parse_f64(b"3.1415926535"), 3.1415926535);
        assert_eq!(parse_f64(b"-2.718281828"), -2.718281828);
        assert_eq!(parse_f64(b"1e100"), 1e100);
    }


}


#[derive(Debug)]
/// ## PlatformInfo struct
/// - A internal struct, that collect info about the flatform were the code is running.
/// - Set, which function is used, according the features.
pub struct PlatformInfo{
    pub supports_neon: bool,
    pub supports_avx2: bool,
    pub supports_memcacher3 : bool,
}

/// ## Default implement for PlatformInfo
/// - By default, disable NEON/ AVX2 support, to improve compatibility.
impl Default for PlatformInfo {
    fn default() -> Self {
        Self {
            supports_avx2 : false,
            supports_memcacher3: true,
            supports_neon: false,
        }
    }
}


impl PlatformInfo {
    /// ## New Function:
    /// - Creates a new instance of `PlatformInfo`, and check of the mapped features are available.
    pub fn new() -> Self{
        let mut plat = Self::default();
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                plat.supports_avx2 = true;
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            plat.supports_neon = true;
        }
        plat
    }
}




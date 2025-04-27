

#[derive(Debug)]
pub struct PlatformInfo{
    pub supports_neon: bool,
    pub supports_avx2: bool,
    pub supports_memcacher3 : bool,
}


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




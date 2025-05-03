fn main() {
    println!("I will tell you if something is wrong my friend");
    // Solo advertir si es x86_64
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if arch == "x86_64" {
        // Detect avx2
        let target = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
        let has_avx2 = target.split(',').any(|f| f.trim() == "avx2");

        if !has_avx2 {
            println!("cargo:warning=🚨 WARNING: AVX2 not enabled! Compile with: RUSTFLAGS=\"-C target-cpu=native\" o $env:RUSTFLAGS = \"-C target-cpu=native\" in powershell");
        }
    }
}

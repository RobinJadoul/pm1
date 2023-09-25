fn main() {
    let mut b = autocxx_build::Builder::new("src/primes.rs", &[&""])
        .build()
        .unwrap();
    b.flag_if_supported("-std=c++14").compile("primesieve-rs");
    println!("cargo:rustc-link-lib=primesieve")
}

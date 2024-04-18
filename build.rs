fn main() {
    let version = rustc_version::version().expect("Failed to get Rust version");
    println!("Cargo:rustc-rustc_version={}", version);
}

fn main() {
    // CARGO_PKG_VERSION comes from Cargo.toml
    // GIT_COMMIT is set in build.rs (or "unknown" if git isn't available)
    let version = env!("CARGO_PKG_VERSION");
    let commit = option_env!("GIT_COMMIT").unwrap_or("unknown");
    // Print just the version info and exit
    println!("{version} ({commit})");
}

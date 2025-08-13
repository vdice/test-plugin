use std::process::Command;

fn main() {
    // Re-run if the HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/");

    // Try to get short commit hash
    let commit = Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .output()
        .ok()
        .and_then(|o| if o.status.success() {
            Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
        } else { None });

    // Mark as dirty if there are uncommitted changes
    let dirty = Command::new("git")
        .args(["diff", "--quiet", "--ignore-submodules", "HEAD"])
        .status()
        .ok()
        .map(|s| !s.success()) // non-zero => dirty
        .unwrap_or(false);

    let mut tag = commit.unwrap_or_else(|| "unknown".to_string());
    if dirty {
        tag.push_str("-dirty");
    }

    // Export as an env var available at compile time
    println!("cargo:rustc-env=GIT_COMMIT={}", tag);
}

use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let lock_path = PathBuf::from(&manifest_dir).join("Cargo.lock");
    println!("cargo:rerun-if-changed={}", lock_path.display());

    let lock = match fs::read_to_string(&lock_path) {
        Ok(contents) => contents,
        Err(_) => return,
    };

    if let Some(version) = find_package_version(&lock, "koto") {
        println!("cargo:rustc-env=KOTO_DEP_VERSION={version}");
    }

    if let Some(version) = find_package_version(&lock, "algebraeon") {
        println!("cargo:rustc-env=ALGEBRAEON_DEP_VERSION={version}");
    }
}

fn find_package_version(contents: &str, package_name: &str) -> Option<String> {
    let mut name: Option<&str> = None;

    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed == "[[package]]" {
            name = None;
            continue;
        }

        if let Some(value) = trimmed.strip_prefix("name = \"") {
            name = Some(value.trim_end_matches('"'));
            continue;
        }

        if let Some(version) = trimmed.strip_prefix("version = \"") {
            if name == Some(package_name) {
                return Some(version.trim_end_matches('"').to_string());
            }
        }
    }

    None
}

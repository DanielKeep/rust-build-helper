#[macro_use] mod util;

#[test]
fn test_pkgs() {
    cargo!("build", "--manifest-path", "tests/pkgs/basic/Cargo.toml")
        .expect("pkgs/basic")
        .succeeded("pkgs/basic");
}

#[test]
#[cfg(feature = "nightly")]
fn test_pkgs_nightly() {
    cargo!("build", "--manifest-path", "tests/pkgs/basic-nightly/Cargo.toml")
        .expect("pkgs/basic-nightly")
        .succeeded("pkgs/basic-nightly")
}

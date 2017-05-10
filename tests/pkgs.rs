#[macro_use] mod util;

#[test]
fn test_pkgs() {
    cargo!("build", "--manifest-path", "tests/pkgs/basic/Cargo.toml")
        .expect("pkgs/basic")
        .succeeded("pkgs/basic");
}

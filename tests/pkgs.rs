/*
Copyright ⓒ 2017 contributors.
Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
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

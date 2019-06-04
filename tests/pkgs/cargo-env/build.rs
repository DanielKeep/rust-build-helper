/*
Copyright ⓒ 2017 contributors.
Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
extern crate build_helper;

use std::collections::HashSet;
use std::env;
use build_helper::*;

const KNOWN_VARS: &'static [&'static str] = &[
    // Supported
    "CARGO",
    "CARGO_CFG_TARGET_ENDIAN",
    "CARGO_CFG_TARGET_FEATURE",
    "CARGO_CFG_TARGET_HAS_ATOMIC",
    "CARGO_CFG_TARGET_POINTER_WIDTH",
    "CARGO_CFG_UNIX",
    "CARGO_CFG_WINDOWS",
    "CARGO_MANIFEST_DIR",
    "CARGO_MANIFEST_LINKS",
    "CARGO_PKG_AUTHORS",
    "CARGO_PKG_DESCRIPTION",
    "CARGO_PKG_HOMEPAGE",
    "CARGO_PKG_NAME",
    "CARGO_PKG_VERSION",
    "DEBUG",
    "HOST",
    "NUM_JOBS",
    "OPT_LEVEL",
    "OUT_DIR",
    "PROFILE",
    "RUSTC",
    "RUSTDOC",
    "TARGET",
    // Redundant
    "CARGO_PKG_VERSION_MINOR",
    "CARGO_PKG_VERSION_PRE",
    "CARGO_PKG_VERSION_PATCH",
    "CARGO_PKG_VERSION_MAJOR",
    "CARGO_CFG_TARGET_ARCH",
    "CARGO_CFG_TARGET_OS",
    "CARGO_CFG_TARGET_VENDOR",
    "CARGO_CFG_TARGET_ENV",
    "CARGO_CFG_TARGET_FAMILY",
    // Not relevant
    "CARGO_CFG_PROC_MACRO",
    "CARGO_HOME",
    "CARGO_MAKEFLAGS",
    "CARGO_PKG_REPOSITORY",
    "CARGO_TARGET_DIR",
    // Unknown
    "CARGO_CFG_DEBUG_ASSERTIONS", // Appears to always be present and empty.
    "CARGO_CFG_TARGET_THREAD_LOCAL", // Unknown meaning
];

const PREFIX_CHECK: &'static [&'static str] = &["CARGO_"];

const PREFIX_IGNORE: &'static [&'static str] = &["CARGO_FEATURE_"];

fn main() {
    let known: HashSet<_> = KNOWN_VARS.iter().cloned().collect();
    let check = PREFIX_CHECK;
    let ignore = PREFIX_IGNORE;
    let mut fail = false;

    for (k, v) in env::vars() {
        if known.contains(&*k) {
            continue;
        }
        let must_know = check.iter().any(|&e| k.starts_with(e));
        if must_know {
            let can_ignore = ignore.iter().any(|&e| k.starts_with(e));
            if !can_ignore {
                println!("unknown env var: {:?} = {:?}", k, v);
                fail = true;
            }
        }
    }

    if fail {
        panic!("one or more unknown variables found");
    }
}

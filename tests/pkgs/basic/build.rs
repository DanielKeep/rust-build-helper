/*
Copyright ⓒ 2017 contributors.
Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
extern crate build_helper;

use build_helper::*;
use build_helper::semver::Version;

macro_rules! show {
    ($value:expr) => {
        println!(concat!("DEBUG: ", stringify!($value), ": {:?}"), $value);
    };
    ($value:expr, |$i:ident| $map:expr) => {
        {
            let $i = $value;
            println!(concat!("DEBUG: ", stringify!($value), ": {:?}"), $map);
        }
    };
}

fn main() {
    // For things which have a stable value, test for that value.  For everything else, just ensure can read the value at all.
    let _ = bin::cargo();
    let _ = bin::rustc();
    let _ = bin::rustdoc();
    assert_eq!(cargo::features::all().collect::<Vec<_>>().sorted(), vec!["a", "default"]);
    assert!(cargo::features::enabled("a"));
    assert!(!cargo::features::enabled("b"));
    assert!(cargo::features::enabled("default"));
    let _ = cargo::manifest::dir();
    assert_eq!(cargo::manifest::links(), Some("awakening".into()));
    assert_eq!(cargo::pkg::authors().sorted(), vec!["Daniel Keep <daniel.keep@gmail.com>", "John Smith <null@null>"]);
    assert_eq!(cargo::pkg::description().unwrap_or("".into()), "A description of this crate.");
    assert_eq!(cargo::pkg::homepage().unwrap_or("".into()), "http://example.org/basic.rs");
    assert_eq!(cargo::pkg::name(), "basic");
    assert_eq!(cargo::pkg::version(), Version::parse("1.2.3-pre").unwrap());
    let _ = debug();
    let _ = host();
    let _ = num_jobs();
    let _ = opt_level();
    let _ = out_dir();
    let _ = profile();
    let _ = target::endian();
    let _ = target::pointer_width();
    {
        let triple = target::triple();
        let mut s = format!("{}-{}-{}", triple.arch(), triple.family(), triple.os());
        if let Some(env) = triple.env() {
            s.push_str("-");
            s.push_str(env);
        }
        assert_eq!(triple.as_str(), s);
    }
    let _ = windows();

    // Emit whatever we can which *should* be inert.
    metadata::emit_raw("key", "value");
    rustc::cfg("a_cfg_flag");
    rustc::flags("-L .");
    rustc::link_lib(None, "dummy");
    rustc::link_search(None, ".");
    rerun_if_changed("dne.rs");
    warning("not that big a deal");
}

trait Sorted {
    fn sorted(self) -> Self;
}

impl<T: Ord> Sorted for Vec<T> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

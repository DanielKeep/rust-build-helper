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

fn main() {
    // For things which have a stable value, test for that value.  For everything else, just ensure can read the value at all.
    let _ = target::features();
    let _ = target::has_atomic();
}

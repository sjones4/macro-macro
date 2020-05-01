// Copyright (c) 2020 Steve Jones
// SPDX-License-Identifier: MIT

use macro_macro::{macro_macro, macro_template};

// Template matches the foo field by name and the bar field by type
macro_template!(
    cfg_struct = {
        struct __ {
            #[cfg(foo)]
            foo: !,

            #[cfg(foo)]
            __: u32,
        }
    }
);

#[macro_macro(cfg_struct)]
#[derive(Default, Debug, Clone)]
struct Structure {
    foo: i32,
    bar: u32,
    value: String,
}

// Test that the Structure only has a value field
#[test]
fn cfg() {
    let _ = Structure {
        value: String::from("foo"),
    }
    .clone();
}

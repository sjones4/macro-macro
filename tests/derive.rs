// Copyright (c) 2020 Steve Jones
// SPDX-License-Identifier: MIT

use macro_macro::{macro_macro, macro_template};

// Template adds derive to any struct
macro_template!(
    derive_struct = {
        #[derive(Default, Debug, Clone)]
        struct __ {}
    }
);

#[macro_macro(derive_struct)]
struct Structure {
    value: String,
}

// Test that clone can be called on Structure
#[test]
fn derive() {
    let _ = Structure {
        value: String::from("foo"),
    }
    .clone();
}

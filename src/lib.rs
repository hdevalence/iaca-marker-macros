// -*- coding: utf-8; mode: rust; -*-
//
// To the extent possible under law, the authors have waived all
// copyright and related or neighboring rights to iaca-marker-macros,
// using the Creative Commons "CC0" public domain dedication.  See
// <http://creativecommons.org/publicdomain/zero/1.0/> for full
// details.
//
// Authors:
// - Henry de Valence <hdevalence@hdevalence.ca>

//! This crate has macros to generate markers for use by the Intel
//! Architecture Code Analyzer.  Note that it is UNOFFICIAL and not
//! affiliated with or endorsed by Intel IN ANY WAY.
//!
//! The `iaca_start_marker` macro marks the start of a block.  The
//! `iaca_end_marker` macro marks the end of a block.  After building
//! with `cargo build --release`, you can then point Intel's `iaca`
//! tool at the generated `.rlib` file.  See the [Intel
//! documentation](https://software.intel.com/en-us/articles/intel-architecture-code-analyzer/)
//! for more details.
//!
//! These macros insert inline asm, so `#![feature(asm)]` is required.

/// Place this macro to mark the beginning of a code block.
#[macro_export]
macro_rules! iaca_start_marker {
    () => {
        unsafe {
            asm! (
                "mov ebx, 111 
                .byte 0x64, 0x67, 0x90"
                : : : "ebx" : "intel", "volatile"
            );
        }
    };
}

/// Place this macro to mark the end of a code block.
#[macro_export]
macro_rules! iaca_end_marker {
    () => {
        unsafe {
            asm! (
                "mov ebx, 222 
                .byte 0x64, 0x67, 0x90"
                : : : "ebx" : "intel", "volatile"
            );
        }
    };
}

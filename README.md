This crate has macros to generate markers for use by the Intel
Architecture Code Analyzer.  Note that it is UNOFFICIAL and not
affiliated with or endorsed by Intel IN ANY WAY.

The `iaca_start_marker` macro marks the start of a block.  The
`iaca_end_marker` macro marks the end of a block.  After building
with `cargo build --release`, you can then point Intel's `iaca`
tool at the generated `.rlib` file.  See the [Intel
documentation](https://software.intel.com/en-us/articles/intel-architecture-code-analyzer/)
for more details.

These macros insert inline asm, so `#![feature(asm)]` is required.

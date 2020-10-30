# A Custom Oscillator in Rust

This directory contains Rust code implementing the same custom
oscillator includes in the `waves` demo.

Building this example requires slightly different tools than the rest of
the code in this repository. First, install `rustup` using the
appropriate mechanism for your operating system:

    https://rustup.rs/

Next, use `rustup` to install the `core` library for the ARMv7 Thumb
architecture:

    rustup target add armv7r-none-eabihf

Finally, install the architecture-specific LLVM binutils:

    cargo install cargo-binutils
    rustup component add llvm-tools-preview

Finally, a single command is sufficient to create `raves.ntkdigunit`:

    ./scripts/pkg.sh

The resulting file can be loaded with the Librarian or `logue-cli`.

# Im crate bug report

Bug: `m.get_prev(&key)` function returns `None`, while `m.range(..=key).rev().next()` returns `Some(..)`.

To reproduce, run `cargo fuzz run target1 fuzz/artifacts/target1/minimized-from-e3fb0078c2bfd242e75a98432416c16c5f38c112`, which will both display the trace of operations, and fail the assertion.

NOTE: You might have to install cargo-fuzz using `cargo install cargo-fuzz` and use rust nightly (required by cargo-fuzz) using `rustup override set nightly`. The bug was discovered on rust stable.

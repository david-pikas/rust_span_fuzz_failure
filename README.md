This repository is basically what you would get after running `cargo init` and `cargo fuzz init`, in addition to a dependency on `rustc_ap_rustc-span`, a trivial use of this dependency in `src/lib.rs` and a reference to that use in `fuzz/fuzz_targets/fuzz_target_1.rs`. 

To get the error message simply run `cargo fuzz run fuzz_target_1`

The error message (with RUST_BACKTRACE=full) can be found in `error_msg.txt`

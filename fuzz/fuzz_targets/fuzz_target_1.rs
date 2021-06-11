#![no_main]
use libfuzzer_sys::fuzz_target;
use rust_span_fuzz_failure;

fuzz_target!(|data: &[u8]| {
    println!("{}", rust_span_fuzz_failure::MY_EDITION);
    // fuzzed code goes here
});

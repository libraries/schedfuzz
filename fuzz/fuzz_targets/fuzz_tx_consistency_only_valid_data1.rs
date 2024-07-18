#![no_main]

use libfuzzer_sys::{Corpus,fuzz_target};
use schedfuzz::{patch, sched};

fuzz_target!(|data: &[u8]| -> Corpus {
    // Fuzzed code goes here
    let r_patch = patch::run(data, 2).map_err(|e| format!("{:?}", e));
    let r_sched = sched::run(data, 2).map_err(|e| format!("{:?}", e));
    assert_eq!(r_patch, r_sched);
    match r_patch {
            Ok(_) => {
                return Corpus::Keep;
            }
            Err(_) => {
                return Corpus::Reject;
            }
    }
});

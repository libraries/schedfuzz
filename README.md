# Schedfuzz


### run fuzz
```sh
# Install component and tools (require rust nightly)
$ rustup component add llvm-tools-preview --toolchain nightly
$ cargo install cargo-binutils
$ cargo install rustfilt

# Run fuzz: fuzz_tx_consistency
$ cargo +nightly fuzz run -j$(nproc) fuzz_tx_consistency

# Generate coverage report
$ cargo +nightly fuzz coverage fuzz_tx_consistency
$ cargo +nightly cov -- show target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/fuzz_tx_consistency --Xdemangler=rustfilt --format=html -instr-profile=fuzz/coverage/fuzz_tx_consistency/coverage.profdata --line-coverage-gt=1 --name=ckb_script > res/report.html
```

### update corpus
After running any fuzzing tests, all corpus should be compacted via:
```sh
$ cargo +nightly fuzz cmin fuzz_tx_consistency
```
Then commit the corpus changes for future use.
# Schedfuzz

```sh
# Install component and tools (require rust nightly)
$ rustup component add llvm-tools-preview --toolchain nightly
$ cargo install cargo-binutils
$ cargo install rustfilt

# Run fuzz
$ cargo +nightly fuzz run -j$(nproc) fuzz_target_1

# Generate coverage report
$ cargo +nightly fuzz coverage fuzz_target_1
$ cargo +nightly cov -- show target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/fuzz_target_1 --Xdemangler=rustfilt --format=html -instr-profile=fuzz/coverage/fuzz_target_1/coverage.profdata --line-coverage-gt=1 --name=ckb_script > res/report.html
```

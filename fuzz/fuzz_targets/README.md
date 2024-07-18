# Fuzzing Targets for Transaction Consistency

## Overview

This repository contains fuzzing scenarios designed to validate the consistency of transaction processing across different versions. The fuzz targets ensure that execution results remain stable and predictable. The scenarios are defined in the following Rust files:

- [`fuzz_tx_consistency.rs`](fuzz/fuzz_targets/fuzz_tx_consistency.rs)
- [`fuzz_tx_consistency_only_valid_data1.rs`](fuzz/fuzz_targets/fuzz_tx_consistency_only_valid_data1.rs)

## Fuzzing Scenarios

### 1. [`fuzz_tx_consistency.rs`](fuzz/fuzz_targets/fuzz_tx_consistency.rs)

#### Purpose
This fuzz target verifies that the results of executing transactions with `spawn` branches are consistent with the results from older versions of the system when using both `data` and `data1`.

#### How It Works
- Executes transactions with `spawn` branches using `data` and `data1`.
- Executes transactions with older versions of the system using `data` and `data1`.
- Compares the execution results to ensure consistency.
- Both valid and invalid transaction data are added to the corpus.

### 2. [`fuzz_tx_consistency_only_valid_data1.rs`](fuzz/fuzz_targets/fuzz_tx_consistency_only_valid_data1.rs)

#### Purpose
This fuzz target addresses the limitation of the first scenario, where a large proportion of invalid transactions might dominate the corpus. To ensure that the execution cycles for valid transactions are consistent, this scenario focuses exclusively on valid transactions with `data1`.

#### How It Works
- Executes valid transactions with `spawn` branches using `data1`.
- Executes valid transactions with older versions of the system using `data1`.
- Compares the execution results to ensure consistency.
- Only valid transaction data are added to the corpus.

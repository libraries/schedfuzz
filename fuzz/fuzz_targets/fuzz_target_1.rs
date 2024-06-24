#![no_main]

use libfuzzer_sys::fuzz_target;

mod patch {
    use ckb_chain_spec_patch::consensus::ConsensusBuilder;
    use ckb_error_patch::Error;
    use ckb_script_patch::{TransactionScriptsVerifier, TxVerifyEnv};
    use ckb_traits_patch::{CellDataProvider, ExtensionProvider, HeaderProvider};
    use ckb_types_patch::{
        bytes::Bytes,
        core::{
            capacity_bytes,
            cell::{CellMetaBuilder, ResolvedTransaction},
            Capacity, Cycle, HeaderView, ScriptHashType, TransactionBuilder, TransactionInfo,
        },
        h256,
        packed::{
            self, Byte32, CellInput, CellOutput, CellOutputBuilder, OutPoint, Script,
            TransactionInfoBuilder, TransactionKeyBuilder,
        },
        prelude::*,
    };

    #[derive(Default, PartialEq, Eq, Clone)]
    struct MockDataLoader {}

    impl CellDataProvider for MockDataLoader {
        fn get_cell_data(&self, _out_point: &OutPoint) -> Option<Bytes> {
            None
        }

        fn get_cell_data_hash(&self, _out_point: &OutPoint) -> Option<Byte32> {
            None
        }
    }

    impl HeaderProvider for MockDataLoader {
        fn get_header(&self, _block_hash: &Byte32) -> Option<HeaderView> {
            None
        }
    }

    impl ExtensionProvider for MockDataLoader {
        fn get_block_extension(&self, _hash: &Byte32) -> Option<packed::Bytes> {
            None
        }
    }

    fn mock_transaction_info() -> TransactionInfo {
        TransactionInfoBuilder::default()
            .block_number(1u64.pack())
            .block_epoch(0u64.pack())
            .key(
                TransactionKeyBuilder::default()
                    .block_hash(Byte32::zero())
                    .index(1u32.pack())
                    .build(),
            )
            .build()
            .unpack()
    }

    pub fn run(data: &[u8], version: u8) -> Result<Cycle, Error> {
        let transaction = TransactionBuilder::default()
            .input(CellInput::new(OutPoint::null(), 0))
            .build();

        let data: Bytes = (Vec::from(data)).into();
        let script = Script::new_builder()
            .hash_type(ScriptHashType::try_from(version).unwrap().into())
            .code_hash(CellOutput::calc_data_hash(&data))
            .build();
        let dep_cell = CellMetaBuilder::from_cell_output(
            CellOutput::new_builder()
                .capacity(Capacity::bytes(data.len()).unwrap().pack())
                .build(),
            data,
        )
        .transaction_info(mock_transaction_info())
        .out_point(OutPoint::new(h256!("0x0").pack(), 0))
        .build();

        let input_cell = CellMetaBuilder::from_cell_output(
            CellOutputBuilder::default()
                .capacity(capacity_bytes!(100).pack())
                .lock(script)
                .build(),
            Bytes::new(),
        )
        .transaction_info(mock_transaction_info())
        .build();

        let rtx = ResolvedTransaction {
            transaction,
            resolved_cell_deps: vec![dep_cell],
            resolved_inputs: vec![input_cell],
            resolved_dep_groups: vec![],
        };

        let provider = MockDataLoader {};
        let consensus = ConsensusBuilder::default().build();
        let tx_verify_env =
            TxVerifyEnv::new_submit(&HeaderView::new_advanced_builder().epoch(0.pack()).build());
        let verifier = TransactionScriptsVerifier::new(
            rtx.into(),
            provider,
            consensus.into(),
            tx_verify_env.into(),
        );
        verifier.verify(10_000_000)
    }
}

mod sched {
    use ckb_chain_spec_sched::consensus::ConsensusBuilder;
    use ckb_error_sched::Error;
    use ckb_script_sched::{TransactionScriptsVerifier, TxVerifyEnv};
    use ckb_traits_sched::{CellDataProvider, ExtensionProvider, HeaderProvider};
    use ckb_types_sched::{
        bytes::Bytes,
        core::{
            capacity_bytes,
            cell::{CellMetaBuilder, ResolvedTransaction},
            Capacity, Cycle, HeaderView, ScriptHashType, TransactionBuilder, TransactionInfo,
        },
        h256,
        packed::{
            self, Byte32, CellInput, CellOutput, CellOutputBuilder, OutPoint, Script,
            TransactionInfoBuilder, TransactionKeyBuilder,
        },
        prelude::*,
    };

    #[derive(Default, PartialEq, Eq, Clone)]
    struct MockDataLoader {}

    impl CellDataProvider for MockDataLoader {
        fn get_cell_data(&self, _out_point: &OutPoint) -> Option<Bytes> {
            None
        }

        fn get_cell_data_hash(&self, _out_point: &OutPoint) -> Option<Byte32> {
            None
        }
    }

    impl HeaderProvider for MockDataLoader {
        fn get_header(&self, _block_hash: &Byte32) -> Option<HeaderView> {
            None
        }
    }

    impl ExtensionProvider for MockDataLoader {
        fn get_block_extension(&self, _hash: &Byte32) -> Option<packed::Bytes> {
            None
        }
    }

    fn mock_transaction_info() -> TransactionInfo {
        TransactionInfoBuilder::default()
            .block_number(1u64.pack())
            .block_epoch(0u64.pack())
            .key(
                TransactionKeyBuilder::default()
                    .block_hash(Byte32::zero())
                    .index(1u32.pack())
                    .build(),
            )
            .build()
            .unpack()
    }

    pub fn run(data: &[u8], version: u8) -> Result<Cycle, Error> {
        let transaction = TransactionBuilder::default()
            .input(CellInput::new(OutPoint::null(), 0))
            .build();

        let data: Bytes = (Vec::from(data)).into();
        let script = Script::new_builder()
            .hash_type(ScriptHashType::try_from(version).unwrap().into())
            .code_hash(CellOutput::calc_data_hash(&data))
            .build();
        let dep_cell = CellMetaBuilder::from_cell_output(
            CellOutput::new_builder()
                .capacity(Capacity::bytes(data.len()).unwrap().pack())
                .build(),
            data,
        )
        .transaction_info(mock_transaction_info())
        .out_point(OutPoint::new(h256!("0x0").pack(), 0))
        .build();

        let input_cell = CellMetaBuilder::from_cell_output(
            CellOutputBuilder::default()
                .capacity(capacity_bytes!(100).pack())
                .lock(script)
                .build(),
            Bytes::new(),
        )
        .transaction_info(mock_transaction_info())
        .build();

        let rtx = ResolvedTransaction {
            transaction,
            resolved_cell_deps: vec![dep_cell],
            resolved_inputs: vec![input_cell],
            resolved_dep_groups: vec![],
        };

        let provider = MockDataLoader {};
        let consensus = ConsensusBuilder::default().build();
        let tx_verify_env =
            TxVerifyEnv::new_submit(&HeaderView::new_advanced_builder().epoch(0.pack()).build());
        let verifier = TransactionScriptsVerifier::new(
            rtx.into(),
            provider,
            consensus.into(),
            tx_verify_env.into(),
        );
        verifier.verify(10_000_000)
    }
}

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let r_patch = patch::run(data, 0).map_err(|e| format!("{:?}", e));
    let r_sched = sched::run(data, 0).map_err(|e| format!("{:?}", e));
    assert_eq!(r_patch, r_sched);

    let r_patch = patch::run(data, 2).map_err(|e| format!("{:?}", e));
    let r_sched = sched::run(data, 2).map_err(|e| format!("{:?}", e));
    assert_eq!(r_patch, r_sched);
});

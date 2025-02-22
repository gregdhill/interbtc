/// Tests for BTC-Relay
use sp_core::U256;

use crate::{ext, mock::*, types::*, BtcAddress, Error};

type Event = crate::Event<Test>;

use crate::{Chains, ChainsIndex};
use bitcoin::{formatter::TryFormattable, merkle::*, parser::*, types::*};
use frame_support::{assert_err, assert_ok};
use mocktopus::mocking::*;
use sp_std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};
/// # Getters and setters
///
/// get_chain_position_from_chain_id
/// set_chain_from_position_and_id
#[test]
fn get_chain_position_from_chain_id_succeeds() {
    run_test(|| {
        // position and id of chains
        let mut chains_pos_id: Vec<(u32, u32)> = Vec::new();
        chains_pos_id.append(&mut vec![(0, 0), (1, 1), (2, 3), (3, 6)]);

        for (pos, id) in chains_pos_id.iter() {
            // insert chain
            BTCRelay::set_chain_from_position_and_id(*pos, *id);
            // check that chain is in right position
            let curr_pos = BTCRelay::get_chain_position_from_chain_id(*id).unwrap();

            assert_eq!(curr_pos, *pos);
        }
    })
}

/// get_block_header_from_hash
/// set_block_header_from_hash
#[test]
fn get_block_header_from_hash_succeeds() {
    run_test(|| {
        let chain_ref: u32 = 2;
        let block_height: u32 = 100;

        let rich_header = RichBlockHeader::<BlockNumber> {
            block_header: sample_block_header(),
            block_height,
            chain_ref,
            para_height: Default::default(),
        };

        BTCRelay::set_block_header_from_hash(rich_header.block_hash(), &rich_header);

        let curr_header = BTCRelay::get_block_header_from_hash(rich_header.block_hash()).unwrap();
        assert_eq!(rich_header, curr_header);
    })
}

#[test]
fn get_block_header_from_hash_fails() {
    run_test(|| {
        let block_hash = H256Le::zero();

        assert_err!(
            BTCRelay::get_block_header_from_hash(block_hash),
            TestError::BlockNotFound
        );
    })
}

/// next_best_fork_chain
/// set_block_chain_from_id
#[test]
fn next_best_fork_chain_succeeds() {
    run_test(|| {
        let chain_ref: u32 = 1;
        let start_height: u32 = 10;
        let block_height: u32 = 100;

        let blockchain = get_empty_block_chain_from_chain_id_and_height(chain_ref, start_height, block_height);

        BTCRelay::set_block_chain_from_id(chain_ref, &blockchain);

        let curr_blockchain = BTCRelay::get_block_chain_from_id(chain_ref).unwrap();

        assert_eq!(curr_blockchain, blockchain);
    })
}

#[test]
fn test_get_block_chain_from_id_empty_chain_fails() {
    run_test(|| {
        assert_err!(BTCRelay::get_block_chain_from_id(1), TestError::InvalidChainID);
    })
}

/// # Main functions
///
/// initialize
#[test]
fn initialize_once_succeeds() {
    run_test(|| {
        let block_height: u32 = 1;
        let block_header = sample_block_header();
        let block_header_hash = block_header.hash;
        BTCRelay::best_block_exists.mock_safe(|| MockResult::Return(false));

        assert_ok!(BTCRelay::initialize(3, block_header, block_height));

        let init_event = TestEvent::BTCRelay(Event::Initialized(block_height, block_header_hash, 3));
        assert!(System::events().iter().any(|a| a.event == init_event));
    })
}

#[test]
fn initialize_best_block_already_set_fails() {
    run_test(|| {
        let block_height: u32 = 1;

        BTCRelay::best_block_exists.mock_safe(|| MockResult::Return(true));

        assert_err!(
            BTCRelay::initialize(3, sample_block_header(), block_height),
            TestError::AlreadyInitialized
        );
    })
}

/// store_block_header function
#[test]
fn store_block_header_on_mainchain_succeeds() {
    run_test(|| {
        BTCRelay::verify_block_header.mock_safe(|_, _, _| MockResult::Return(Ok(())));
        BTCRelay::block_header_exists.mock_safe(|_| MockResult::Return(true));

        let chain_ref: u32 = 0;
        let start_height: u32 = 0;
        let block_height: u32 = 100;
        let block_header = sample_block_header();

        let rich_header = RichBlockHeader::<BlockNumber> {
            block_header,
            block_height,
            chain_ref,
            para_height: Default::default(),
        };
        BTCRelay::get_block_header_from_hash.mock_safe(move |_| MockResult::Return(Ok(rich_header)));

        let prev_blockchain = get_empty_block_chain_from_chain_id_and_height(chain_ref, start_height, block_height);
        BTCRelay::get_block_chain_from_id.mock_safe(move |_: u32| MockResult::Return(Ok(prev_blockchain.clone())));

        let block_header_hash = block_header.hash;
        assert_ok!(BTCRelay::store_block_header(&3, rich_header.block_header));

        let store_main_event = TestEvent::BTCRelay(Event::StoreMainChainHeader(block_height + 1, block_header_hash, 3));
        assert!(System::events().iter().any(|a| a.event == store_main_event));
    })
}

#[test]
fn store_block_header_on_fork_succeeds() {
    run_test(|| {
        BTCRelay::verify_block_header.mock_safe(|_, _, _| MockResult::Return(Ok(())));
        BTCRelay::block_header_exists.mock_safe(|_| MockResult::Return(true));

        let chain_ref: u32 = 1;
        let start_height: u32 = 20;
        let block_height: u32 = 100;
        let block_header = sample_block_header();

        let rich_header = RichBlockHeader::<BlockNumber> {
            block_header,
            block_height: block_height - 1,
            chain_ref,
            para_height: Default::default(),
        };
        BTCRelay::get_block_header_from_hash.mock_safe(move |_| MockResult::Return(Ok(rich_header)));

        let prev_blockchain = get_empty_block_chain_from_chain_id_and_height(chain_ref, start_height, block_height);
        BTCRelay::get_block_chain_from_id.mock_safe(move |_: u32| MockResult::Return(Ok(prev_blockchain.clone())));

        let block_header_hash = block_header.hash;

        // simulate that initialize has been called
        BTCRelay::increment_chain_counter().unwrap();

        assert_ok!(BTCRelay::store_block_header(&3, block_header));

        let store_fork_event =
            TestEvent::BTCRelay(Event::StoreForkHeader(chain_ref, block_height, block_header_hash, 3));
        assert!(System::events().iter().any(|a| a.event == store_fork_event));
    })
}

mod store_block_header_tests {
    use crate::MAIN_CHAIN_ID;

    use super::*;
    fn from_prev(nonce: u32, prev: H256Le) -> BlockHeader {
        let mut ret = BlockHeader {
            nonce,
            hash_prev_block: prev,
            ..sample_block_header()
        };
        ret.update_hash().unwrap();
        ret
    }

    fn check_store_block_header_invariants() {
        let mainchain = ChainsIndex::<Test>::get(0).unwrap();

        // cache chains and chains_index for readability
        let mut chains = Chains::<Test>::iter().collect::<Vec<_>>();
        chains.sort_by_key(|k| k.0);
        let mut chains_index = ChainsIndex::<Test>::iter().collect::<Vec<_>>();
        chains_index.sort_by_key(|k| k.0);

        // The keys in ``Chains`` MUST be consecutive, i.e. for each ``i``, if ``Chains[i]`` does not exist,
        // ``Chains[i+1]`` MUST NOT exist either.
        for (arr_idx, (key, _value)) in chains.iter().enumerate() {
            assert_eq!(arr_idx as u32, *key);
        }
        let chains = chains.into_iter().map(|(_, value)| value).collect::<Vec<_>>();

        // The keys in ``ChainsIndex`` MUST be consecutive, i.e. for each ``i``, if ``ChainsIndex[i]`` does not exist,
        // ``ChainsIndex[i+1]`` MUST NOT exist either.
        for (arr_idx, (key, _value)) in chains_index.iter().enumerate() {
            assert_eq!(arr_idx as u32, *key);
        }
        let chains_index = chains_index.into_iter().map(|(_, value)| value).collect::<Vec<_>>();

        // for all i > 0, `ChainsIndex[i].maxHeight < ChainsIndex[0].maxHeight + STABLE_BITCOIN_CONFIRMATIONS`
        for chain in chains_index.iter().skip(1) {
            assert!(chain.max_height < mainchain.max_height + BTCRelay::get_stable_transaction_confirmations());
        }

        // The values in ``Chains`` MUST be such that for each ``0 < i < j``, ``ChainsIndex[Chains[i]].maxHeight >=
        // ChainsIndex[Chains[j]].maxHeight``.
        for i in 1..chains.len() - 1 {
            if chains_index[chains[i] as usize].max_height < chains_index[chains[i + 1] as usize].max_height {
                assert!(chains_index[chains[i] as usize].max_height >= chains_index[chains[i + 1] as usize].max_height);
            }
        }

        // ChainsIndex[i].chainRef = i
        for (idx, chain) in chains_index.iter().enumerate() {
            assert_eq!(idx as u32, chain.chain_id);
        }

        // BestBlock MUST refer the latest block from the main chain
        assert_eq!(
            BTCRelay::get_block_hash(MAIN_CHAIN_ID, mainchain.max_height).unwrap(),
            BTCRelay::get_best_block()
        );

        // BestBlockHeight MUST be equal to ``ChainsIndex[0].maxHeight``
        assert_eq!(BTCRelay::get_best_block_height(), mainchain.max_height);

        // the ``chainRef`` stored inside blocks MUST agree with the actual chain they are stored on
        for (chain_idx, _, hash) in crate::ChainsHashes::<Test>::iter() {
            assert_eq!(crate::BlockHeaders::<Test>::get(hash).chain_ref, chain_idx);
        }
    }

    fn assert_is_block(height: u32, block_header: &BlockHeader) {
        Security::set_active_block_number(ext::security::active_block_number::<Test>() + 1000);

        BTCRelay::ensure_no_ongoing_fork.mock_safe(|_| MockResult::Return(Ok(())));
        assert_ok!(BTCRelay::verify_block_header_inclusion(block_header.hash, Some(0)));
        BTCRelay::ensure_no_ongoing_fork.clear_mock();

        let chain = BTCRelay::get_block_chain_from_id(0).unwrap();
        assert_eq!(
            BTCRelay::get_block_header_from_height(&chain, height)
                .unwrap()
                .block_header,
            block_header.clone()
        );
    }

    #[test]
    fn store_block_header_simple_fork_succeeds() {
        run_test(|| {
            BTCRelay::verify_block_header.mock_safe(|_, _, _| MockResult::Return(Ok(())));

            let mut genesis = sample_block_header();
            genesis.nonce = 11;
            genesis.update_hash().unwrap();

            assert_ok!(BTCRelay::initialize(3, genesis, 10));

            // second block to the mainchain - otherwise we can't create a fork
            let a2 = from_prev(12, genesis.hash);

            let mut blocks = vec![a2];

            // create a new fork, and make it overtake the main chain
            let mut prev = genesis.hash;
            for i in 0..10 {
                blocks.push(from_prev(31 + i, prev));
                prev = blocks[blocks.len() - 1].hash;
            }

            for x in blocks.iter() {
                store_header_and_check_invariants(x);
            }

            assert_is_block(10, &genesis);
            for (idx, block) in blocks.iter().skip(1).enumerate() {
                assert_is_block(11 + idx as u32, block);
            }
            // block a2 used to be in the mainchain, but is not anymore
            assert_err!(
                BTCRelay::verify_block_header_inclusion(a2.hash, Some(0)),
                TestError::InvalidChainID
            );
        })
    }

    fn assert_best_block(block_header: &BlockHeader, height: u32) {
        assert_eq!(BTCRelay::get_best_block_height(), height);
        assert_is_block(height, &block_header);
    }

    fn assert_ongoing_fork() {
        assert_err!(
            BTCRelay::ensure_no_ongoing_fork(BTCRelay::get_best_block_height()),
            TestError::OngoingFork
        );
    }

    fn store_header_and_check_invariants(block: &BlockHeader) {
        check_store_block_header_invariants();
        assert_ok!(BTCRelay::store_block_header(&3, block.clone()));
        check_store_block_header_invariants();
    }

    #[test]
    fn store_block_header_fork_of_fork_succeeds() {
        run_test(|| {
            BTCRelay::verify_block_header.mock_safe(|_, _, _| MockResult::Return(Ok(())));

            let mut genesis = sample_block_header();
            genesis.update_hash().unwrap();

            // create the following tree shape:
            // f1 --> temp_fork_1
            //    \-> f2 --> temp_fork_2
            //           \-> f3 --> ... --> f10

            // corresponds to f1..f10
            let final_chain =
                sp_std::iter::successors(Some(genesis), |prev| Some(from_prev(prev.nonce + 1, prev.hash)))
                    .take(10)
                    .collect::<Vec<_>>();

            assert_ok!(BTCRelay::initialize(3, final_chain[0].clone(), 10));
            assert_best_block(&final_chain[0], 10);

            // submit a temporary block such that final_chain[1] will be considered a fork
            let temp_fork_1 = from_prev(100, final_chain[0].hash);
            store_header_and_check_invariants(&temp_fork_1);

            store_header_and_check_invariants(&final_chain[1]);
            assert_ongoing_fork();

            // submit a temporary block such that final_chain[2] will be considered a fork
            let temp_fork_2 = from_prev(101, final_chain[1].hash);
            store_header_and_check_invariants(&temp_fork_2);
            assert_ongoing_fork();

            // main chain and fork currently have same height - we can submit CONFIRMATION-1 block without reorg
            for block in final_chain
                .iter()
                .skip(2) // 2 already submitted
                .take(BTCRelay::get_stable_transaction_confirmations() as usize - 1)
            {
                store_header_and_check_invariants(&block);
                assert_is_block(11, &temp_fork_1);
            }

            // we did reorg, but temp_fork_2 has height of 12, so it is still considered an ongoing fork
            store_header_and_check_invariants(
                &final_chain[BTCRelay::get_stable_transaction_confirmations() as usize + 1],
            );

            assert_ongoing_fork();

            for (idx, block) in final_chain
                .iter()
                .enumerate()
                .skip(BTCRelay::get_stable_transaction_confirmations() as usize + 2)
            {
                // all blocks in final chain that have been submitted so far must now be usable
                for (idx, block) in final_chain.iter().enumerate().take(idx - 1) {
                    assert_is_block(10 + idx as u32, block);
                }

                store_header_and_check_invariants(block);
            }

            // temp_fork_1 used to be in the mainchain, but is not anymore
            assert_err!(
                BTCRelay::verify_block_header_inclusion(temp_fork_1.hash, Some(0)),
                TestError::InvalidChainID
            );
            // temp_fork_2 is not included
            assert_err!(
                BTCRelay::verify_block_header_inclusion(temp_fork_2.hash, Some(0)),
                TestError::InvalidChainID
            );
        })
    }
}

#[test]
fn store_block_header_parachain_shutdown_fails() {
    run_test(|| {
        ext::security::ensure_parachain_status_not_shutdown::<Test>
            .mock_safe(|| MockResult::Return(Err(SecurityError::ParachainShutdown.into())));

        assert_err!(
            BTCRelay::store_block_header(&3, sample_block_header()),
            SecurityError::ParachainShutdown,
        );
    })
}

#[test]
fn store_block_header_no_prev_block_fails() {
    run_test(|| {
        ext::security::ensure_parachain_status_not_shutdown::<Test>.mock_safe(|| MockResult::Return(Ok(())));

        assert_err!(
            BTCRelay::store_block_header(&3, sample_block_header()),
            TestError::BlockNotFound,
        );
    })
}

#[test]
fn check_and_do_reorg_fork_id_not_found() {
    run_test(|| {
        let chain_ref: u32 = 99;
        let start_height: u32 = 3;
        let block_height: u32 = 10;

        let blockchain = get_empty_block_chain_from_chain_id_and_height(chain_ref, start_height, block_height);

        assert_err!(BTCRelay::reorganize_chains(&blockchain), TestError::ForkIdNotFound);
    })
}

#[test]
fn check_and_do_reorg_swap_fork_position() {
    run_test(|| {
        // insert the main chain in Chains and ChainsIndex
        let main_chain_ref: u32 = 0;
        let main_start_height: u32 = 3;
        let main_block_height: u32 = 110;
        let main_position: u32 = 0;
        let main = get_empty_block_chain_from_chain_id_and_height(main_chain_ref, main_start_height, main_block_height);
        BTCRelay::set_chain_from_position_and_id(main_position, main_chain_ref);
        BTCRelay::set_block_chain_from_id(main_chain_ref, &main);

        // insert the fork chain in Chains and ChainsIndex
        let fork_chain_ref: u32 = 4;
        let fork_start_height: u32 = 20;
        let fork_block_height: u32 = 100;
        let fork_position: u32 = 2;
        let fork = get_empty_block_chain_from_chain_id_and_height(fork_chain_ref, fork_start_height, fork_block_height);
        BTCRelay::set_chain_from_position_and_id(fork_position, fork_chain_ref);
        BTCRelay::set_block_chain_from_id(fork_chain_ref, &fork);

        // insert the swap chain in Chains and ChainsIndex
        let swap_chain_ref: u32 = 3;
        let swap_start_height: u32 = 43;
        let swap_block_height: u32 = 99;
        let swap_position: u32 = 1;
        let swap = get_empty_block_chain_from_chain_id_and_height(swap_chain_ref, swap_start_height, swap_block_height);
        BTCRelay::set_chain_from_position_and_id(swap_position, swap_chain_ref);
        BTCRelay::set_block_chain_from_id(swap_chain_ref, &swap);

        // check that fork is at its initial position
        let current_position = BTCRelay::get_chain_position_from_chain_id(fork_chain_ref).unwrap();

        assert_eq!(current_position, fork_position);

        assert_ok!(BTCRelay::reorganize_chains(&fork));
        // assert that positions have been swapped
        let new_position = BTCRelay::get_chain_position_from_chain_id(fork_chain_ref).unwrap();
        assert_eq!(new_position, swap_position);

        // assert the main chain has not changed
        let curr_main_chain = BTCRelay::get_block_chain_from_id(main_chain_ref);
        assert_eq!(curr_main_chain, Ok(main));
    })
}

#[test]
fn check_and_do_reorg_new_fork_is_main_chain() {
    run_test(|| {
        // insert the main chain in Chains and ChainsIndex
        let main_chain_ref: u32 = 0;
        let main_start_height: u32 = 4;
        let main_block_height: u32 = 110;
        let main_position: u32 = 0;
        let main = get_empty_block_chain_from_chain_id_and_height(main_chain_ref, main_start_height, main_block_height);
        BTCRelay::set_chain_from_position_and_id(main_position, main_chain_ref);
        BTCRelay::set_block_chain_from_id(main_chain_ref, &main);

        // insert the fork chain in Chains and ChainsIndex
        let fork_chain_ref: u32 = 4;
        let fork_block_height: u32 = 117;
        let fork_position: u32 = 1;
        let fork = get_empty_block_chain_from_chain_id_and_height(fork_chain_ref, main_start_height, fork_block_height);
        BTCRelay::set_chain_from_position_and_id(fork_position, fork_chain_ref);
        BTCRelay::set_block_chain_from_id(fork_chain_ref, &fork);

        // set the best block
        let best_block_hash = H256Le::zero();
        BTCRelay::set_best_block(best_block_hash);
        BTCRelay::set_best_block_height(fork_block_height);

        // check that fork is at its initial position
        let current_position = BTCRelay::get_chain_position_from_chain_id(fork_chain_ref).unwrap();

        assert_eq!(current_position, fork_position);

        BTCRelay::swap_main_blockchain.mock_safe(move |_| MockResult::Return(Ok((best_block_hash, fork_block_height))));

        assert_ok!(BTCRelay::reorganize_chains(&fork));
        // assert that the new main chain is set
        let reorg_event = TestEvent::BTCRelay(Event::ChainReorg(
            best_block_hash,
            fork_block_height,
            fork.max_height - fork.start_height,
        ));
        assert!(System::events().iter().any(|a| a.event == reorg_event));
    })
}
#[test]
fn check_and_do_reorg_new_fork_below_stable_transaction_confirmations() {
    run_test(|| {
        // insert the main chain in Chains and ChainsIndex
        let main_chain_ref: u32 = 0;
        let main_start_height: u32 = 4;
        let main_block_height: u32 = 110;
        let main_position: u32 = 0;
        let main = get_empty_block_chain_from_chain_id_and_height(main_chain_ref, main_start_height, main_block_height);
        BTCRelay::set_chain_from_position_and_id(main_position, main_chain_ref);
        BTCRelay::set_block_chain_from_id(main_chain_ref, &main);

        // insert the fork chain in Chains and ChainsIndex
        let fork_chain_ref: u32 = 4;
        let fork_block_height: u32 = 113;
        let fork_position: u32 = 1;
        let fork = get_empty_block_chain_from_chain_id_and_height(fork_chain_ref, main_start_height, fork_block_height);
        BTCRelay::set_chain_from_position_and_id(fork_position, fork_chain_ref);
        BTCRelay::set_block_chain_from_id(fork_chain_ref, &fork);

        // set the best block
        let best_block_hash = H256Le::zero();
        BTCRelay::set_best_block(best_block_hash);
        BTCRelay::set_best_block_height(fork_block_height);

        // check that fork is at its initial position
        let current_position = BTCRelay::get_chain_position_from_chain_id(fork_chain_ref).unwrap();

        assert_eq!(current_position, fork_position);

        BTCRelay::swap_main_blockchain.mock_safe(move |_| MockResult::Return(Ok((best_block_hash, fork_block_height))));

        assert_ok!(BTCRelay::reorganize_chains(&fork));
        // assert that the fork has not overtaken the main chain
        let ahead_event = TestEvent::BTCRelay(Event::ForkAheadOfMainChain(
            main_block_height,
            fork_block_height,
            fork_chain_ref,
        ));
        assert!(System::events().iter().any(|a| a.event == ahead_event));
    })
}

/// insert_sorted
#[test]
fn insert_sorted_succeeds() {
    run_test(|| {
        // insert the main chain in Chains and ChainsIndex
        let main_chain_ref: u32 = 0;
        let main_start_height: u32 = 60;
        let main_block_height: u32 = 110;
        let main_position: u32 = 0;
        let main = get_empty_block_chain_from_chain_id_and_height(main_chain_ref, main_start_height, main_block_height);
        BTCRelay::set_block_chain_from_id(main_chain_ref, &main);
        assert_eq!(Ok(()), BTCRelay::insert_sorted(&main));

        let curr_main_pos = BTCRelay::get_chain_position_from_chain_id(main_chain_ref).unwrap();
        assert_eq!(curr_main_pos, main_position);
        // insert the swap chain in Chains and ChainsIndex
        let swap_chain_ref: u32 = 3;
        let swap_start_height: u32 = 70;
        let swap_block_height: u32 = 99;
        let swap_position: u32 = 1;
        let swap = get_empty_block_chain_from_chain_id_and_height(swap_chain_ref, swap_start_height, swap_block_height);
        BTCRelay::set_block_chain_from_id(swap_chain_ref, &swap);
        assert_eq!(Ok(()), BTCRelay::insert_sorted(&swap));

        let curr_swap_pos = BTCRelay::get_chain_position_from_chain_id(swap_chain_ref).unwrap();
        assert_eq!(curr_swap_pos, swap_position);

        // insert the fork chain in Chains and ChainsIndex
        let fork_chain_ref: u32 = 4;
        let fork_start_height: u32 = 77;
        let fork_block_height: u32 = 100;
        let fork_position: u32 = 1;
        let new_swap_pos: u32 = 2;
        let fork = get_empty_block_chain_from_chain_id_and_height(fork_chain_ref, fork_start_height, fork_block_height);
        BTCRelay::set_block_chain_from_id(fork_chain_ref, &fork);
        assert_eq!(Ok(()), BTCRelay::insert_sorted(&fork));

        let curr_fork_pos = BTCRelay::get_chain_position_from_chain_id(fork_chain_ref).unwrap();
        assert_eq!(curr_fork_pos, fork_position);
        let curr_swap_pos = BTCRelay::get_chain_position_from_chain_id(swap_chain_ref).unwrap();
        assert_eq!(curr_swap_pos, new_swap_pos);
    })
}

/// verify_block_header
#[test]
fn test_verify_block_header_no_retarget_succeeds() {
    run_test(|| {
        let chain_ref: u32 = 0;
        // no retarget at block 100
        let block_height: u32 = 100;
        let genesis_header = sample_parsed_genesis_header(chain_ref, block_height);

        let raw_first_header = RawBlockHeader::from_hex(sample_raw_first_header()).unwrap();

        // Not duplicate block
        BTCRelay::block_header_exists.mock_safe(move |_| MockResult::Return(false));

        let block_header = BTCRelay::parse_raw_block_header(&raw_first_header).unwrap();

        assert_ok!(BTCRelay::verify_block_header(
            &block_header,
            genesis_header.block_height + 1,
            genesis_header
        ));
    })
}

#[test]
fn test_verify_block_header_correct_retarget_increase_succeeds() {
    run_test(|| {
        let chain_ref: u32 = 0;
        // Next block requires retarget
        let block_height: u32 = 2015;
        // Sample interval with INCREASING target
        let retarget_headers = sample_retarget_interval_increase();

        let prev_block_header_rich = RichBlockHeader::<BlockNumber>::new(
            parse_block_header(&retarget_headers[1]).unwrap(),
            chain_ref,
            block_height,
            Default::default(),
        );

        let curr_block_header = parse_block_header(&retarget_headers[2]).unwrap();
        // Prev block exists
        BTCRelay::get_block_header_from_hash.mock_safe(move |_| MockResult::Return(Ok(prev_block_header_rich)));
        // Not duplicate block
        BTCRelay::block_header_exists.mock_safe(move |_| MockResult::Return(false));
        // Compute new target returns target of submitted header (i.e., correct)
        BTCRelay::compute_new_target.mock_safe(move |_, _| MockResult::Return(Ok(curr_block_header.target)));

        let block_header = BTCRelay::parse_raw_block_header(&retarget_headers[2]).unwrap();
        assert_ok!(BTCRelay::verify_block_header(
            &block_header,
            prev_block_header_rich.block_height + 1,
            prev_block_header_rich
        ));
    })
}

#[test]
fn test_verify_block_header_correct_retarget_decrease_succeeds() {
    run_test(|| {
        let chain_ref: u32 = 0;
        // Next block requires retarget
        let block_height: u32 = 2015;
        // Sample interval with DECREASING target
        let retarget_headers = sample_retarget_interval_decrease();

        let prev_block_header_rich = RichBlockHeader::<BlockNumber>::new(
            parse_block_header(&retarget_headers[1]).unwrap(),
            chain_ref,
            block_height,
            Default::default(),
        );

        let curr_block_header = parse_block_header(&retarget_headers[2]).unwrap();
        // Not duplicate block
        BTCRelay::block_header_exists.mock_safe(move |_| MockResult::Return(false));
        // Compute new target returns target of submitted header (i.e., correct)
        BTCRelay::compute_new_target.mock_safe(move |_, _| MockResult::Return(Ok(curr_block_header.target)));

        let block_header = BTCRelay::parse_raw_block_header(&retarget_headers[2]).unwrap();
        assert_ok!(BTCRelay::verify_block_header(
            &block_header,
            prev_block_header_rich.block_height + 1,
            prev_block_header_rich
        ));
    })
}

#[test]
fn test_verify_block_header_missing_retarget_succeeds() {
    run_test(|| {
        let chain_ref: u32 = 0;
        // Next block requires retarget
        let block_height: u32 = 2015;
        let retarget_headers = sample_retarget_interval_increase();

        let prev_block_header_rich = RichBlockHeader::<BlockNumber>::new(
            parse_block_header(&retarget_headers[1]).unwrap(),
            chain_ref,
            block_height,
            Default::default(),
        );

        let curr_block_header = parse_block_header(&retarget_headers[2]).unwrap();
        // Not duplicate block
        BTCRelay::block_header_exists.mock_safe(move |_| MockResult::Return(false));
        // Compute new target returns HIGHER target
        BTCRelay::compute_new_target.mock_safe(move |_, _| MockResult::Return(Ok(curr_block_header.target + 1)));

        let block_header = BTCRelay::parse_raw_block_header(&retarget_headers[2]).unwrap();
        assert_err!(
            BTCRelay::verify_block_header(
                &block_header,
                prev_block_header_rich.block_height + 1,
                prev_block_header_rich
            ),
            TestError::DiffTargetHeader
        );
    })
}

#[test]
fn test_compute_new_target() {
    let chain_ref: u32 = 0;
    // no retarget at block 100
    let block_height: u32 = 2016;
    let retarget_headers = sample_retarget_interval_increase();

    let last_retarget_time = parse_block_header(&retarget_headers[0]).unwrap().timestamp as u64;
    let prev_block_header = RichBlockHeader::<BlockNumber>::new(
        parse_block_header(&retarget_headers[1]).unwrap(),
        chain_ref,
        block_height,
        Default::default(),
    );

    let curr_block_header = parse_block_header(&retarget_headers[2]).unwrap();

    BTCRelay::get_last_retarget_time.mock_safe(move |_, _| MockResult::Return(Ok(last_retarget_time)));

    let new_target = BTCRelay::compute_new_target(&prev_block_header, block_height).unwrap();

    assert_eq!(new_target, curr_block_header.target);
}

#[test]
fn test_verify_block_header_duplicate_fails() {
    run_test(|| {
        let chain_ref: u32 = 0;
        // no retarget at block 100
        let block_height: u32 = 100;
        let genesis_header = sample_parsed_genesis_header(chain_ref, block_height);

        let rich_first_header = sample_parsed_first_block(chain_ref, 101);

        // Prev block is genesis
        BTCRelay::get_block_header_from_hash.mock_safe(move |_| MockResult::Return(Ok(genesis_header)));
        // submitted block ALREADY EXISTS
        BTCRelay::block_header_exists.mock_safe(move |block_hash| {
            assert_eq!(&block_hash, &rich_first_header.block_header.hash);
            MockResult::Return(true)
        });

        let raw_first_header = RawBlockHeader::from_hex(sample_raw_first_header()).unwrap();
        let first_header = BTCRelay::parse_raw_block_header(&raw_first_header).unwrap();
        assert_err!(
            BTCRelay::verify_block_header(&first_header, genesis_header.block_height + 1, genesis_header),
            TestError::DuplicateBlock
        );
    })
}

#[test]
fn test_verify_block_header_low_diff_fails() {
    run_test(|| {
        let chain_ref: u32 = 0;
        // no retarget at block 100
        let block_height: u32 = 100;
        let genesis_header = sample_parsed_genesis_header(chain_ref, block_height);

        // block header with high target but weak hash
        let raw_first_header_weak = RawBlockHeader::from_hex(sample_raw_first_header_low_diff()).unwrap();

        // submitted block does not yet exist
        BTCRelay::block_header_exists.mock_safe(move |_| MockResult::Return(false));

        let first_header_weak = BTCRelay::parse_raw_block_header(&raw_first_header_weak).unwrap();
        assert_err!(
            BTCRelay::verify_block_header(&first_header_weak, genesis_header.block_height + 1, genesis_header),
            TestError::LowDiff
        );
    });
}

#[test]
fn test_validate_transaction_succeeds_with_payment() {
    run_test(|| {
        let raw_tx = hex::decode(sample_accepted_transaction()).unwrap();
        let minimum_btc: i64 = 2500200000;
        let recipient_btc_address =
            BtcAddress::P2SH(H160::from_str(&"66c7060feb882664ae62ffad0051fe843e318e85").unwrap());

        let outputs = vec![sample_valid_payment_output()];

        BTCRelay::parse_transaction.mock_safe(move |_| MockResult::Return(Ok(sample_transaction_parsed(&outputs))));

        assert_ok!(BTCRelay::validate_transaction(
            Origin::signed(3),
            raw_tx,
            minimum_btc,
            recipient_btc_address,
            None,
        ));
    });
}

#[test]
fn test_validate_transaction_succeeds_with_payment_and_op_return() {
    run_test(|| {
        let raw_tx = hex::decode(sample_accepted_transaction()).unwrap();
        let minimum_btc: i64 = 2500200000;
        let recipient_btc_address =
            BtcAddress::P2SH(H160::from_str(&"66c7060feb882664ae62ffad0051fe843e318e85").unwrap());
        let op_return_id =
            hex::decode("e5c17d15b8b1fa2811b7e6da66ffa5e1aaa05922c69068bf90cd585b95bb4675".to_owned()).unwrap();

        let outputs = vec![sample_valid_payment_output(), sample_valid_data_output()];

        BTCRelay::parse_transaction.mock_safe(move |_| MockResult::Return(Ok(sample_transaction_parsed(&outputs))));

        assert_ok!(BTCRelay::validate_transaction(
            Origin::signed(3),
            raw_tx,
            minimum_btc,
            recipient_btc_address,
            Some(H256::from_slice(&op_return_id))
        ));
    });
}

#[test]
fn test_validate_transaction_succeeds_with_op_return_and_payment() {
    run_test(|| {
        let raw_tx = hex::decode(sample_accepted_transaction()).unwrap();
        let minimum_btc: i64 = 2500200000;
        let recipient_btc_address =
            BtcAddress::P2SH(H160::from_str(&"66c7060feb882664ae62ffad0051fe843e318e85").unwrap());
        let op_return_id =
            hex::decode("e5c17d15b8b1fa2811b7e6da66ffa5e1aaa05922c69068bf90cd585b95bb4675".to_owned()).unwrap();

        let outputs = vec![sample_valid_data_output(), sample_valid_payment_output()];

        BTCRelay::parse_transaction.mock_safe(move |_| MockResult::Return(Ok(sample_transaction_parsed(&outputs))));

        assert_ok!(BTCRelay::validate_transaction(
            Origin::signed(3),
            raw_tx,
            minimum_btc,
            recipient_btc_address,
            Some(H256::from_slice(&op_return_id))
        ));
    });
}

#[test]
fn test_validate_transaction_succeeds_with_payment_and_refund_and_op_return() {
    run_test(|| {
        let raw_tx = hex::decode(sample_accepted_transaction()).unwrap();
        let minimum_btc: i64 = 2500200000;
        let recipient_btc_address =
            BtcAddress::P2SH(H160::from_str(&"66c7060feb882664ae62ffad0051fe843e318e85").unwrap());
        let op_return_id =
            hex::decode("e5c17d15b8b1fa2811b7e6da66ffa5e1aaa05922c69068bf90cd585b95bb4675".to_owned()).unwrap();

        let outputs = vec![
            sample_valid_payment_output(),
            sample_wrong_recipient_payment_output(),
            sample_valid_data_output(),
        ];

        BTCRelay::parse_transaction.mock_safe(move |_| MockResult::Return(Ok(sample_transaction_parsed(&outputs))));

        assert_ok!(BTCRelay::validate_transaction(
            Origin::signed(3),
            raw_tx,
            minimum_btc,
            recipient_btc_address,
            Some(H256::from_slice(&op_return_id))
        ));
    });
}

#[test]
fn test_validate_transaction_invalid_no_outputs_fails() {
    run_test(|| {
        // Simulate input (we mock the parsed transaction)
        let raw_tx = hex::decode(sample_accepted_transaction()).unwrap();

        let minimum_btc: i64 = 2500200000;
        let recipient_btc_address =
            BtcAddress::P2SH(H160::from_str(&"66c7060feb882664ae62ffad0051fe843e318e85").unwrap());
        let op_return_id =
            hex::decode("ede5c17d15b8b1fa2811b7e6da66ffa5e1aaa05922c69068bf90cd585b95bb46".to_owned()).unwrap();
        // missing required data output
        let outputs = vec![sample_valid_payment_output()];

        BTCRelay::parse_transaction.mock_safe(move |_| MockResult::Return(Ok(sample_transaction_parsed(&outputs))));

        assert_err!(
            BTCRelay::validate_transaction(
                Origin::signed(3),
                raw_tx,
                minimum_btc,
                recipient_btc_address,
                Some(H256::from_slice(&op_return_id))
            ),
            TestError::InvalidOpReturnTransaction
        )
    });
}

#[test]
fn test_validate_transaction_insufficient_payment_value_fails() {
    run_test(|| {
        // Simulate input (we mock the parsed transaction)
        let raw_tx = vec![0u8; 342];

        let minimum_btc: i64 = 2500200000;
        let recipient_btc_address =
            BtcAddress::P2SH(H160::from_str(&"66c7060feb882664ae62ffad0051fe843e318e85").unwrap());
        let op_return_id =
            hex::decode("e5c17d15b8b1fa2811b7e6da66ffa5e1aaa05922c69068bf90cd585b95bb4675".to_owned()).unwrap();

        let outputs = vec![sample_insufficient_value_payment_output(), sample_valid_data_output()];

        BTCRelay::parse_transaction.mock_safe(move |_| MockResult::Return(Ok(sample_transaction_parsed(&outputs))));

        assert_err!(
            BTCRelay::validate_transaction(
                Origin::signed(3),
                raw_tx,
                minimum_btc,
                recipient_btc_address,
                Some(H256::from_slice(&op_return_id))
            ),
            TestError::InvalidPaymentAmount
        )
    });
}

#[test]
fn test_validate_transaction_wrong_recipient_fails() {
    run_test(|| {
        // Simulate input (we mock the parsed transaction)
        let raw_tx = vec![0u8; 342];

        let minimum_btc: i64 = 2500200000;
        let recipient_btc_address =
            BtcAddress::P2SH(H160::from_str(&"66c7060feb882664ae62ffad0051fe843e318e85").unwrap());
        let op_return_id =
            hex::decode("e5c17d15b8b1fa2811b7e6da66ffa5e1aaa05922c69068bf90cd585b95bb4675".to_owned()).unwrap();

        let outputs = vec![
            sample_wrong_recipient_payment_output(),
            sample_wrong_recipient_payment_output(),
            sample_valid_data_output(),
        ];

        BTCRelay::parse_transaction.mock_safe(move |_| MockResult::Return(Ok(sample_transaction_parsed(&outputs))));

        assert_err!(
            BTCRelay::validate_transaction(
                Origin::signed(3),
                raw_tx,
                minimum_btc,
                recipient_btc_address,
                Some(H256::from_slice(&op_return_id))
            ),
            TestError::InvalidOpReturnTransaction
        )
    });
}

#[test]
fn test_validate_transaction_incorrect_opreturn_fails() {
    run_test(|| {
        // Simulate input (we mock the parsed transaction)
        let raw_tx = vec![0u8; 342];

        let minimum_btc: i64 = 2500200000;
        let recipient_btc_address =
            BtcAddress::P2SH(H160::from_str(&"66c7060feb882664ae62ffad0051fe843e318e85").unwrap());
        let op_return_id =
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000".to_owned()).unwrap();

        let outputs = vec![sample_valid_payment_output(), sample_incorrect_data_output()];

        BTCRelay::parse_transaction.mock_safe(move |_| MockResult::Return(Ok(sample_transaction_parsed(&outputs))));

        assert_err!(
            BTCRelay::validate_transaction(
                Origin::signed(3),
                raw_tx,
                minimum_btc,
                recipient_btc_address,
                Some(H256::from_slice(&op_return_id))
            ),
            TestError::InvalidOpReturnTransaction
        )
    });
}

#[test]
fn test_verify_and_validate_transaction_succeeds() {
    run_test(|| {
        BTCRelay::get_block_chain_from_id.mock_safe(|_| MockResult::Return(Ok(BlockChain::default())));

        let raw_tx = hex::decode(sample_example_real_rawtx()).unwrap();
        let transaction = parse_transaction(&raw_tx).unwrap();
        // txid are returned by Bitcoin-core
        let real_txid = H256Le::from_hex_be(&sample_example_real_txid());
        let real_tx_hash = H256Le::from_hex_be(&sample_example_real_transaction_hash());

        // see https://learnmeabitcoin.com/explorer/transaction/json.php?txid=c586389e5e4b3acb9d6c8be1c19ae8ab2795397633176f5a6442a261bbdefc3a
        assert_eq!(transaction.hash(), real_tx_hash);
        assert_eq!(transaction.tx_id(), real_txid);

        // rest are example values -- not checked in this test.
        // let block_height = 0;
        let raw_merkle_proof = vec![0u8; 100];
        let confirmations = None;
        let minimum_btc: i64 = 0;
        let recipient_btc_address =
            BtcAddress::P2SH(H160::from_str(&"66c7060feb882664ae62ffad0051fe843e318e85").unwrap());
        let op_return_id =
            hex::decode("e5c17d15b8b1fa2811b7e6da66ffa5e1aaa05922c69068bf90cd585b95bb4675".to_owned()).unwrap();
        BTCRelay::parse_merkle_proof.mock_safe(|_| MockResult::Return(Ok(sample_merkle_proof())));
        BTCRelay::_validate_transaction.mock_safe(move |_, _, _, _| MockResult::Return(Ok(())));
        BTCRelay::_verify_transaction_inclusion.mock_safe(move |_, _, _| MockResult::Return(Ok(())));

        assert_ok!(BTCRelay::verify_and_validate_transaction(
            Origin::signed(3),
            raw_merkle_proof,
            confirmations,
            raw_tx,
            minimum_btc,
            recipient_btc_address,
            Some(H256::from_slice(&op_return_id))
        ));
    });
}

#[test]
fn test_verify_transaction_inclusion_succeeds() {
    run_test(|| {
        let chain_ref = 0;
        let fork_ref = 1;
        let start = 10;
        let main_chain_height = 300;
        let fork_chain_height = 280;
        // Random init since we mock this
        let raw_merkle_proof = vec![0u8; 100];
        let confirmations = None;
        let rich_block_header = sample_rich_tx_block_header(chain_ref, main_chain_height);

        let proof = sample_merkle_proof();
        let proof_result = sample_valid_proof_result();

        let main = get_empty_block_chain_from_chain_id_and_height(chain_ref, start, main_chain_height);

        let fork = get_empty_block_chain_from_chain_id_and_height(fork_ref, start, fork_chain_height);

        BTCRelay::get_chain_id_from_position.mock_safe(move |_| MockResult::Return(Ok(fork_ref)));
        BTCRelay::get_block_chain_from_id.mock_safe(move |id| {
            if id == chain_ref {
                MockResult::Return(Ok(main.clone()))
            } else {
                MockResult::Return(Ok(fork.clone()))
            }
        });

        BTCRelay::get_best_block_height.mock_safe(move || MockResult::Return(main_chain_height));

        BTCRelay::parse_merkle_proof.mock_safe(move |_| MockResult::Return(Ok(proof.clone())));
        BTCRelay::verify_merkle_proof.mock_safe(move |_| MockResult::Return(Ok(proof_result)));

        BTCRelay::get_block_header_from_hash.mock_safe(move |_| MockResult::Return(Ok(rich_block_header)));

        BTCRelay::check_bitcoin_confirmations.mock_safe(|_, _, _| MockResult::Return(Ok(())));

        BTCRelay::check_parachain_confirmations.mock_safe(|_| MockResult::Return(Ok(())));

        assert_ok!(BTCRelay::verify_transaction_inclusion(
            Origin::signed(3),
            proof_result.transaction_hash,
            raw_merkle_proof,
            confirmations
        ));
    });
}

#[test]
fn test_verify_transaction_inclusion_empty_fork_succeeds() {
    run_test(|| {
        let chain_ref = 0;
        let start = 10;
        let main_chain_height = 300;
        // Random init since we mock this
        let raw_merkle_proof = vec![0u8; 100];
        let confirmations = None;
        let rich_block_header = sample_rich_tx_block_header(chain_ref, main_chain_height);

        let proof = sample_merkle_proof();
        let proof_result = sample_valid_proof_result();

        let main = get_empty_block_chain_from_chain_id_and_height(chain_ref, start, main_chain_height);

        BTCRelay::get_block_chain_from_id.mock_safe(move |id| {
            if id == chain_ref {
                MockResult::Return(Ok(main.clone()))
            } else {
                MockResult::Return(Err(TestError::InvalidChainID.into()))
            }
        });

        BTCRelay::get_best_block_height.mock_safe(move || MockResult::Return(main_chain_height));

        BTCRelay::parse_merkle_proof.mock_safe(move |_| MockResult::Return(Ok(proof.clone())));
        BTCRelay::verify_merkle_proof.mock_safe(move |_| MockResult::Return(Ok(proof_result)));

        BTCRelay::get_block_header_from_hash.mock_safe(move |_| MockResult::Return(Ok(rich_block_header)));

        BTCRelay::check_bitcoin_confirmations.mock_safe(|_, _, _| MockResult::Return(Ok(())));

        BTCRelay::check_parachain_confirmations.mock_safe(|_| MockResult::Return(Ok(())));

        assert_ok!(BTCRelay::verify_transaction_inclusion(
            Origin::signed(3),
            proof_result.transaction_hash,
            raw_merkle_proof,
            confirmations,
        ));
    });
}

#[test]
fn test_verify_transaction_inclusion_invalid_tx_id_fails() {
    run_test(|| {
        let chain_ref = 0;
        let fork_ref = 1;
        let start = 10;
        let main_chain_height = 300;
        let fork_chain_height = 280;
        // Random init since we mock this
        let raw_merkle_proof = vec![0u8; 100];
        let confirmations = None;
        let rich_block_header = sample_rich_tx_block_header(chain_ref, main_chain_height);

        // Mismatching TXID
        let invalid_tx_id = H256Le::from_bytes_le(
            &hex::decode("0000000000000000000000000000000000000000000000000000000000000000".to_owned()).unwrap(),
        );

        let proof = sample_merkle_proof();
        let proof_result = sample_valid_proof_result();

        let main = get_empty_block_chain_from_chain_id_and_height(chain_ref, start, main_chain_height);

        let fork = get_empty_block_chain_from_chain_id_and_height(fork_ref, start, fork_chain_height);

        BTCRelay::get_chain_id_from_position.mock_safe(move |_| MockResult::Return(Ok(fork_ref)));
        BTCRelay::get_block_chain_from_id.mock_safe(move |id| {
            if id == chain_ref {
                MockResult::Return(Ok(main.clone()))
            } else {
                MockResult::Return(Ok(fork.clone()))
            }
        });

        BTCRelay::get_best_block_height.mock_safe(move || MockResult::Return(main_chain_height));

        BTCRelay::parse_merkle_proof.mock_safe(move |_| MockResult::Return(Ok(proof.clone())));
        BTCRelay::verify_merkle_proof.mock_safe(move |_| MockResult::Return(Ok(proof_result)));

        BTCRelay::get_block_header_from_hash.mock_safe(move |_| MockResult::Return(Ok(rich_block_header)));

        BTCRelay::check_bitcoin_confirmations.mock_safe(|_, _, _| MockResult::Return(Ok(())));

        BTCRelay::check_parachain_confirmations.mock_safe(|_| MockResult::Return(Ok(())));

        assert_err!(
            BTCRelay::verify_transaction_inclusion(Origin::signed(3), invalid_tx_id, raw_merkle_proof, confirmations,),
            TestError::InvalidTxid
        );
    });
}

#[test]
fn test_verify_transaction_inclusion_invalid_merkle_root_fails() {
    run_test(|| {
        let chain_ref = 0;
        let fork_ref = 1;
        let start = 10;
        let main_chain_height = 300;
        let fork_chain_height = 280;
        // Random init since we mock this
        let raw_merkle_proof = vec![0u8; 100];
        let confirmations = None;
        let mut rich_block_header = sample_rich_tx_block_header(chain_ref, main_chain_height);

        // Mismatching merkle root
        let invalid_merkle_root = H256Le::from_bytes_le(
            &hex::decode("0000000000000000000000000000000000000000000000000000000000000000".to_owned()).unwrap(),
        );
        rich_block_header.block_header.merkle_root = invalid_merkle_root;

        let proof = sample_merkle_proof();
        let proof_result = sample_valid_proof_result();

        let main = get_empty_block_chain_from_chain_id_and_height(chain_ref, start, main_chain_height);

        let fork = get_empty_block_chain_from_chain_id_and_height(fork_ref, start, fork_chain_height);

        BTCRelay::get_chain_id_from_position.mock_safe(move |_| MockResult::Return(Ok(fork_ref)));
        BTCRelay::get_block_chain_from_id.mock_safe(move |id| {
            if id == chain_ref {
                MockResult::Return(Ok(main.clone()))
            } else {
                MockResult::Return(Ok(fork.clone()))
            }
        });

        BTCRelay::get_best_block_height.mock_safe(move || MockResult::Return(main_chain_height));

        BTCRelay::parse_merkle_proof.mock_safe(move |_| MockResult::Return(Ok(proof.clone())));

        BTCRelay::get_block_header_from_hash.mock_safe(move |_| MockResult::Return(Ok(rich_block_header)));

        BTCRelay::check_bitcoin_confirmations.mock_safe(|_, _, _| MockResult::Return(Ok(())));

        BTCRelay::check_parachain_confirmations.mock_safe(|_| MockResult::Return(Ok(())));

        assert_err!(
            BTCRelay::verify_transaction_inclusion(
                Origin::signed(3),
                proof_result.transaction_hash,
                raw_merkle_proof,
                confirmations,
            ),
            TestError::InvalidMerkleProof
        );
    });
}

#[test]
fn test_verify_transaction_inclusion_fails_with_ongoing_fork() {
    run_test(|| {
        BTCRelay::get_chain_id_from_position.mock_safe(|_| MockResult::Return(Ok(1)));
        BTCRelay::get_block_chain_from_id.mock_safe(|_| MockResult::Return(Ok(BlockChain::default())));
        BTCRelay::parse_merkle_proof.mock_safe(|_| MockResult::Return(Ok(sample_merkle_proof())));
        BTCRelay::verify_merkle_proof.mock_safe(|_| MockResult::Return(Ok(sample_valid_proof_result())));

        let tx_id = sample_valid_proof_result().transaction_hash;
        let raw_merkle_proof = vec![0u8; 100];
        let confirmations = None;

        assert_err!(
            BTCRelay::verify_transaction_inclusion(Origin::signed(3), tx_id, raw_merkle_proof, confirmations,),
            TestError::OngoingFork
        );
    });
}

#[test]
fn test_check_bitcoin_confirmations_insecure_succeeds() {
    run_test(|| {
        let main_chain_height = 100;
        let tx_block_height = 90;

        let req_confs = Some(5);
        assert_ok!(BTCRelay::check_bitcoin_confirmations(
            main_chain_height,
            req_confs,
            tx_block_height,
        ));
    });
}

#[test]
fn test_check_bitcoin_confirmations_insecure_insufficient_confs_fails() {
    run_test(|| {
        let main_chain_height = 100;
        let tx_block_height = 99;

        let req_confs = Some(5);

        assert_err!(
            BTCRelay::check_bitcoin_confirmations(main_chain_height, req_confs, tx_block_height,),
            TestError::BitcoinConfirmations
        )
    });
}

#[test]
fn test_check_bitcoin_confirmations_secure_stable_confs_succeeds() {
    run_test(|| {
        let main_chain_height = 100;
        let tx_block_height = 90;

        let req_confs = None;
        // relevant check: ok
        let stable_confs = 10;

        BTCRelay::get_stable_transaction_confirmations.mock_safe(move || MockResult::Return(stable_confs));
        assert_ok!(BTCRelay::check_bitcoin_confirmations(
            main_chain_height,
            req_confs,
            tx_block_height,
        ));
    });
}

#[test]
fn test_check_bitcoin_confirmations_secure_user_confs_succeeds() {
    run_test(|| {
        let main_chain_height = 100;
        let tx_block_height = 91;
        // relevant check: ok
        let req_confs = None;
        let stable_confs = 10;

        BTCRelay::get_stable_transaction_confirmations.mock_safe(move || MockResult::Return(stable_confs));
        assert_ok!(BTCRelay::check_bitcoin_confirmations(
            main_chain_height,
            req_confs,
            tx_block_height,
        ));
    });
}

#[test]
fn test_check_bitcoin_confirmations_secure_insufficient_stable_confs_succeeds() {
    run_test(|| {
        let main_chain_height = 100;
        let tx_block_height = 92;

        let req_confs = None;
        // relevant check: fails
        let stable_confs = 10;

        BTCRelay::get_stable_transaction_confirmations.mock_safe(move || MockResult::Return(stable_confs));

        assert_err!(
            BTCRelay::check_bitcoin_confirmations(main_chain_height, req_confs, tx_block_height,),
            TestError::BitcoinConfirmations
        )
    });
}

#[test]
fn test_check_parachain_confirmations_succeeds() {
    run_test(|| {
        Security::set_active_block_number(5 + PARACHAIN_CONFIRMATIONS);
        assert_ok!(BTCRelay::check_parachain_confirmations(0));
    });
}

#[test]
fn test_check_parachain_confirmations_insufficient_confs_fails() {
    run_test(|| {
        Security::set_active_block_number(0);
        assert_err!(
            BTCRelay::check_parachain_confirmations(0),
            TestError::ParachainConfirmations
        );
    });
}

#[test]
fn get_chain_from_id_err() {
    run_test(|| {
        assert_err!(BTCRelay::get_block_chain_from_id(0), TestError::InvalidChainID);
    });
}

#[test]
fn get_chain_from_id_ok() {
    run_test(|| {
        // insert the main chain in Chains and ChainsIndex
        let main_chain_ref: u32 = 0;
        let main_start_height: u32 = 3;
        let main_block_height: u32 = 110;
        let main_position: u32 = 0;
        let main = get_empty_block_chain_from_chain_id_and_height(main_chain_ref, main_start_height, main_block_height);
        BTCRelay::set_chain_from_position_and_id(main_position, main_chain_ref);
        BTCRelay::set_block_chain_from_id(main_chain_ref, &main);

        assert_eq!(Ok(main), BTCRelay::get_block_chain_from_id(main_chain_ref));
    });
}

#[test]
fn store_generated_block_headers() {
    let target = U256::from(2).pow(254.into());
    let miner = BtcAddress::P2PKH(H160::from_str(&"66c7060feb882664ae62ffad0051fe843e318e85").unwrap());
    let get_header = |block: &Block| RawBlockHeader::from_bytes(&block.header.try_format().unwrap()).unwrap();

    run_test(|| {
        let mut last_block = BlockBuilder::new().with_coinbase(&miner, 50, 0).mine(target).unwrap();
        let last_block_header = BTCRelay::parse_raw_block_header(&get_header(&last_block)).unwrap();
        assert_ok!(BTCRelay::initialize(3, last_block_header, 0));
        for i in 1..20 {
            last_block = BlockBuilder::new()
                .with_coinbase(&miner, 50, i)
                .with_previous_hash(last_block.header.hash)
                .mine(target)
                .unwrap();
            let raw_header = get_header(&last_block);
            let block_header = parse_block_header(&raw_header).unwrap();
            assert_ok!(BTCRelay::store_block_header(&3, block_header));
        }
        let main_chain: BlockChain = BTCRelay::get_block_chain_from_id(crate::MAIN_CHAIN_ID).unwrap();
        assert_eq!(main_chain.start_height, 0);
        assert_eq!(main_chain.max_height, 19);
    })
}

mod op_return_payment_data_tests {
    use super::*;
    use itertools::Itertools;

    fn permutations(transaction: Transaction) -> impl Iterator<Item = Transaction> {
        let n = transaction.outputs.len();
        let outputs = transaction.outputs.clone();
        outputs.into_iter().permutations(n).map(move |x| Transaction {
            outputs: x.clone(),
            ..transaction.clone()
        })
    }

    fn dummy_address1() -> BtcAddress {
        BtcAddress::P2SH(H160::from_str(&"66c7060feb882664ae62ffad0051fe843e318e85").unwrap())
    }

    fn dummy_address2() -> BtcAddress {
        BtcAddress::P2SH(H160::from_str(&"0000000000000000000000000000000000000000").unwrap())
    }
    fn dummy_address3() -> BtcAddress {
        BtcAddress::P2SH(H160::from_str(&"1000000000000000000000000000000000000000").unwrap())
    }

    #[test]
    fn test_constructing_op_return_payment_data_with_zero_outputs_fails() {
        run_test(|| {
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::op_return(0, &[0; 32]))
                .build();

            for transaction in permutations(transaction) {
                assert_err!(
                    OpReturnPaymentData::<Test>::try_from(transaction),
                    Error::<Test>::InvalidOpReturnTransaction
                );
            }
        })
    }

    #[test]
    fn test_constructing_op_return_payment_data_with_one_output_succeeds() {
        run_test(|| {
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(252345, &dummy_address1()))
                .add_output(TransactionOutput::op_return(0, &[0; 32]))
                .build();

            for transaction in permutations(transaction) {
                assert_ok!(OpReturnPaymentData::<Test>::try_from(transaction));
            }
        })
    }

    #[test]
    fn test_constructing_op_return_payment_data_with_two_outputs_succeeds() {
        run_test(|| {
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(252345, &dummy_address1()))
                .add_output(TransactionOutput::payment(252345, &dummy_address2()))
                .add_output(TransactionOutput::op_return(0, &[0; 32]))
                .build();

            for transaction in permutations(transaction) {
                assert_ok!(OpReturnPaymentData::<Test>::try_from(transaction));
            }
        })
    }

    #[test]
    fn test_constructing_op_return_payment_data_with_too_many_outputs_fails() {
        run_test(|| {
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(252345, &dummy_address1()))
                .add_output(TransactionOutput::payment(252345, &dummy_address2()))
                .add_output(TransactionOutput::payment(252345, &dummy_address3()))
                .add_output(TransactionOutput::op_return(0, &[0; 32]))
                .build();

            for transaction in permutations(transaction) {
                assert_err!(
                    OpReturnPaymentData::<Test>::try_from(transaction),
                    Error::<Test>::InvalidOpReturnTransaction
                );
            }
        })
    }
    #[test]
    fn test_constructing_op_return_payment_data_with_two_identical_outputs_fails() {
        run_test(|| {
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(252345, &dummy_address1()))
                .add_output(TransactionOutput::payment(252344145, &dummy_address1()))
                .add_output(TransactionOutput::op_return(0, &[0; 32]))
                .build();

            for transaction in permutations(transaction) {
                assert_err!(
                    OpReturnPaymentData::<Test>::try_from(transaction),
                    Error::<Test>::InvalidOpReturnTransaction
                );
            }
        })
    }
    #[test]
    fn test_constructing_op_return_payment_data_with_invalid_op_return_len_fails() {
        run_test(|| {
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(252345, &dummy_address1()))
                .add_output(TransactionOutput::op_return(0, &[0; 31]))
                .build();

            for transaction in permutations(transaction) {
                assert_err!(
                    OpReturnPaymentData::<Test>::try_from(transaction),
                    Error::<Test>::InvalidOpReturnTransaction
                );
            }
        })
    }
    #[test]
    fn test_constructing_op_return_payment_data_with_two_op_returns_fails() {
        run_test(|| {
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(252345, &dummy_address1()))
                .add_output(TransactionOutput::op_return(0, &[0; 32]))
                .add_output(TransactionOutput::op_return(0, &[1; 32]))
                .build();

            for transaction in permutations(transaction) {
                assert_err!(
                    OpReturnPaymentData::<Test>::try_from(transaction),
                    Error::<Test>::InvalidOpReturnTransaction
                );
            }
        })
    }
    #[test]
    fn test_constructing_op_return_payment_data_with_op_return_value_fails() {
        run_test(|| {
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(252345, &dummy_address1()))
                .add_output(TransactionOutput::op_return(1, &[0; 32]))
                .build();

            for transaction in permutations(transaction) {
                assert_err!(
                    OpReturnPaymentData::<Test>::try_from(transaction),
                    Error::<Test>::InvalidOpReturnTransaction
                );
            }
        })
    }

    #[test]
    fn test_ensure_valid_payment_to_succeeds() {
        run_test(|| {
            let amount = 12345;
            let op_return = H256::from_slice(&[5; 32]);
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(amount, &dummy_address1()))
                .add_output(TransactionOutput::payment(123, &dummy_address2()))
                .add_output(TransactionOutput::op_return(0, op_return.as_bytes()))
                .build();

            for transaction in permutations(transaction) {
                let payment_data = OpReturnPaymentData::<Test>::try_from(transaction).unwrap();
                assert_ok!(
                    payment_data.ensure_valid_payment_to(amount, dummy_address1(), Some(op_return)),
                    Some(dummy_address2())
                );
            }
        })
    }

    #[test]
    fn test_ensure_valid_payment_to_single_payment_succeeds() {
        run_test(|| {
            let amount = 12345;
            let op_return = H256::from_slice(&[5; 32]);
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(amount, &dummy_address1()))
                .add_output(TransactionOutput::op_return(0, op_return.as_bytes()))
                .build();

            for transaction in permutations(transaction) {
                let payment_data = OpReturnPaymentData::<Test>::try_from(transaction).unwrap();
                assert_ok!(
                    payment_data.ensure_valid_payment_to(amount, dummy_address1(), Some(op_return)),
                    None
                );
            }
        })
    }

    #[test]
    fn test_ensure_valid_payment_to_without_opreturn_check_succeeds() {
        run_test(|| {
            let amount = 12345;
            let op_return = H256::from_slice(&[5; 32]);
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(amount, &dummy_address1()))
                .add_output(TransactionOutput::payment(123, &dummy_address2()))
                .add_output(TransactionOutput::op_return(0, op_return.as_bytes()))
                .build();

            for transaction in permutations(transaction) {
                let payment_data = OpReturnPaymentData::<Test>::try_from(transaction).unwrap();
                assert_ok!(
                    payment_data.ensure_valid_payment_to(amount, dummy_address1(), None),
                    Some(dummy_address2())
                );
            }
        })
    }

    #[test]
    fn test_ensure_valid_payment_to_wrong_op_return_fails() {
        run_test(|| {
            let amount = 12345;
            let op_return = H256::from_slice(&[5; 32]);
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(amount, &dummy_address1()))
                .add_output(TransactionOutput::op_return(0, &[0; 32]))
                .build();

            for transaction in permutations(transaction) {
                let payment_data = OpReturnPaymentData::<Test>::try_from(transaction).unwrap();
                assert_err!(
                    payment_data.ensure_valid_payment_to(amount, dummy_address1(), Some(op_return)),
                    Error::<Test>::InvalidPayment
                );
            }
        })
    }

    #[test]
    fn test_ensure_valid_payment_to_wrong_recipient_fails() {
        run_test(|| {
            let amount = 12345;
            let op_return = H256::from_slice(&[5; 32]);
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(amount, &dummy_address1()))
                .add_output(TransactionOutput::payment(123, &dummy_address2()))
                .add_output(TransactionOutput::op_return(0, op_return.as_bytes()))
                .build();

            for transaction in permutations(transaction) {
                let payment_data = OpReturnPaymentData::<Test>::try_from(transaction).unwrap();
                assert_err!(
                    payment_data.ensure_valid_payment_to(amount, dummy_address3(), Some(op_return)),
                    Error::<Test>::InvalidPayment
                );
            }
        })
    }

    #[test]
    fn test_ensure_valid_payment_to_with_invalid_amount_fails() {
        run_test(|| {
            let amount = 12345;
            let op_return = H256::from_slice(&[5; 32]);
            let transaction = TransactionBuilder::new()
                .with_version(2)
                .add_output(TransactionOutput::payment(amount, &dummy_address1()))
                .add_output(TransactionOutput::payment(123, &dummy_address2()))
                .add_output(TransactionOutput::op_return(0, op_return.as_bytes()))
                .build();

            for transaction in permutations(transaction) {
                let payment_data = OpReturnPaymentData::<Test>::try_from(transaction).unwrap();
                assert_err!(
                    payment_data.ensure_valid_payment_to(amount - 1, dummy_address1(), Some(op_return)),
                    Error::<Test>::InvalidPaymentAmount
                );
            }
        })
    }
}

#[test]
fn test_check_and_do_reorg() {
    use crate::{Chains, ChainsIndex};
    use bitcoin::types::BlockChain;

    // data taken from testnet fork
    run_test(|| {
        Chains::<Test>::insert(0, 0);
        Chains::<Test>::insert(2, 7);

        ChainsIndex::<Test>::insert(
            0,
            BlockChain {
                chain_id: 0,
                start_height: 1_892_642,
                max_height: 1_897_317,
            },
        );

        ChainsIndex::<Test>::insert(
            2,
            BlockChain {
                chain_id: 2,
                start_height: 1_893_831,
                max_height: 1_893_831,
            },
        );

        ChainsIndex::<Test>::insert(
            4,
            BlockChain {
                chain_id: 4,
                start_height: 1_895_256,
                max_height: 1_895_256,
            },
        );

        ChainsIndex::<Test>::insert(
            6,
            BlockChain {
                chain_id: 6,
                start_height: 1_896_846,
                max_height: 1_896_846,
            },
        );

        ChainsIndex::<Test>::insert(
            7,
            BlockChain {
                chain_id: 7,
                start_height: 1_897_317,
                max_height: 1_897_910,
            },
        );

        BTCRelay::swap_main_blockchain.mock_safe(|_| MockResult::Return(Ok((Default::default(), Default::default()))));

        // we should skip empty `Chains`, this can occur if the
        // previous index is accidentally deleted
        assert_ok!(BTCRelay::reorganize_chains(&BlockChain {
            chain_id: 7,
            start_height: 1_897_317,
            max_height: 1_897_910,
        }));
    })
}

#[test]
pub fn test_has_request_expired() {
    run_test(|| {
        fn has_request_expired_after(period: u64, parachain_blocks: u64, bitcoin_blocks: u32) -> bool {
            let opentime = 1000;
            let btc_open_height = 100;

            BTCRelay::set_best_block_height(btc_open_height + bitcoin_blocks);
            Security::set_active_block_number(opentime + parachain_blocks);
            BTCRelay::has_request_expired(opentime, btc_open_height, period).unwrap()
        }

        // NOTE: mocks configure 100 parachain blocks per bitcoin block

        // boundary - just barely expired
        assert!(has_request_expired_after(600, 601, 7));
        // blockchain blocks not expired
        assert!(!has_request_expired_after(600, 600, 7));
        // bitcoin blocks not expired
        assert!(!has_request_expired_after(600, 601, 6));

        // test that the number of bitcoin blocks required for expiry is rounded up

        // boundary - just barely expired
        assert!(has_request_expired_after(601, 602, 8));
        // blockchain blocks not expired
        assert!(!has_request_expired_after(601, 601, 8));
        // bitcoin blocks not expired
        assert!(!has_request_expired_after(601, 602, 7));
    })
}

/// # Util functions

const SAMPLE_TX_ID: &str = "c8589f304d3b9df1d4d8b3d15eb6edaaa2af9d796e9d9ace12b31f293705c5e9";

const SAMPLE_MERKLE_ROOT: &str = "1EE1FB90996CA1D5DCD12866BA9066458BF768641215933D7D8B3A10EF79D090";

fn sample_merkle_proof() -> MerkleProof {
    MerkleProof {
        block_header: sample_block_header(),
        transactions_count: 1,
        hashes: vec![H256Le::from_hex_le(SAMPLE_TX_ID)],
        flag_bits: vec![true],
    }
}

fn sample_block_header() -> BlockHeader {
    let mut ret = BlockHeader {
        merkle_root: H256Le::from_hex_le(SAMPLE_MERKLE_ROOT),
        target: 123.into(),
        timestamp: 1601494682,
        version: 2,
        hash_prev_block: H256Le::from_hex_be("0000000000000000000e84948eaacb9b03382782f16f2d8a354de69f2e5a2a68"),
        hash: H256Le::from_hex_be("0000000000000000000000000000000000000000000000000000000000000000"),
        nonce: 0,
    };
    ret.update_hash().unwrap();
    ret
}

fn sample_valid_proof_result() -> ProofResult {
    let tx_id = H256Le::from_hex_le(SAMPLE_TX_ID);
    let merkle_root = H256Le::from_hex_le(SAMPLE_MERKLE_ROOT);

    ProofResult {
        extracted_root: merkle_root,
        transaction_hash: tx_id,
        transaction_position: 0,
    }
}

fn get_empty_block_chain_from_chain_id_and_height(chain_id: u32, start_height: u32, block_height: u32) -> BlockChain {
    let blockchain = BlockChain {
        chain_id,
        start_height,
        max_height: block_height,
    };

    blockchain
}

fn sample_raw_genesis_header() -> String {
    "01000000".to_owned() + "a7c3299ed2475e1d6ea5ed18d5bfe243224add249cce99c5c67cc9fb00000000601c73862a0a7238e376f497783c8ecca2cf61a4f002ec8898024230787f399cb575d949ffff001d3a5de07f"
}

fn sample_parsed_genesis_header(chain_ref: u32, block_height: u32) -> RichBlockHeader<BlockNumber> {
    let genesis_header = RawBlockHeader::from_hex(sample_raw_genesis_header()).unwrap();
    RichBlockHeader::<BlockNumber> {
        block_header: parse_block_header(&genesis_header).unwrap(),
        block_height,
        chain_ref,
        para_height: Default::default(),
    }
}

fn sample_raw_first_header_low_diff() -> String {
    "01000000".to_owned() +
    "cb60e68ead74025dcfd4bf4673f3f71b1e678be9c6e6585f4544c79900000000" +
    "c7f42be7f83eddf2005272412b01204352a5fddbca81942c115468c3c4ec2fff" +
    "827ad949" +
    "413b1417" +  // high target
    "21e05e45"
}

fn sample_raw_first_header() -> String {
    "01000000".to_owned() + "cb60e68ead74025dcfd4bf4673f3f71b1e678be9c6e6585f4544c79900000000c7f42be7f83eddf2005272412b01204352a5fddbca81942c115468c3c4ec2fff827ad949ffff001d21e05e45"
}

fn sample_parsed_first_block(chain_ref: u32, block_height: u32) -> RichBlockHeader<BlockNumber> {
    let block_header = RawBlockHeader::from_hex(sample_raw_first_header()).unwrap();
    RichBlockHeader::<BlockNumber> {
        block_header: parse_block_header(&block_header).unwrap(),
        block_height,
        chain_ref,
        para_height: Default::default(),
    }
}

fn sample_retarget_interval_increase() -> [RawBlockHeader; 3] {
    // block height 66528
    let last_retarget_header = RawBlockHeader::from_hex("01000000".to_owned() + "4e8e5cf3c4e4b8f63a9cf88beb2dbaba1949182101ae4e5cf54ad100000000009f2a2344e8112b0d7bd8089414106ee5f17bb6cd64078883e1b661fa251aac6bed1d3c4cf4a3051c4dcd2b02").unwrap();
    // block height 66543
    let prev_block_header = RawBlockHeader::from_hex("01000000".to_owned()  + "1e321d88cb25946c4ca521eece3752803c021f9403fc4e0171203a0500000000317057f8b50414848a5a3a26d9eb8ace3d6f5495df456d0104dd1421159faf5029293c4cf4a3051c73199005").unwrap();
    // block height 68544
    let curr_header = RawBlockHeader::from_hex("01000000".to_owned() + "fb57c71ccd211b3de4ccc2e23b50a7cdb72aab91e60737b3a2bfdf030000000088a88ad9df68925e880e5d52b7e50cef225871c68b40a2cd0bca1084cd436037f388404cfd68011caeb1f801").unwrap();

    [last_retarget_header, prev_block_header, curr_header]
}

fn sample_retarget_interval_decrease() -> [RawBlockHeader; 3] {
    // block height 558432
    let last_retarget_header = RawBlockHeader::from_hex("00c0ff2f".to_owned() + "6550b5dae76559589e3e3e135237072b6bc498949da6280000000000000000005988783435f506d2ccfbadb484e56d6f1d5dfdd480650acae1e3b43d3464ea73caf13b5c33d62f171d508fdb").unwrap();
    // block height 560447
    let prev_block_header = RawBlockHeader::from_hex("00000020".to_owned()  + "d8e8e54ca5e33522b94fbba5de736efc55ff75e832cf2300000000000000000007b395f80858ee022c9c3c2f0f5cee4bd807039f0729b0559ae4326c3ba77d6b209f4e5c33d62f1746ee356d").unwrap();
    // block height 560448
    let curr_header = RawBlockHeader::from_hex("00000020".to_owned() + "6b05bd2c4a06b3d8503a033c2593396a25a79e1dcadb140000000000000000001b08df3d42cd9a38d8b66adf9dc5eb464f503633bd861085ffff723634531596a1a24e5c35683017bf67b72a").unwrap();

    [last_retarget_header, prev_block_header, curr_header]
}

fn sample_accepted_transaction() -> String {
    "020000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff0502cb000101ffffffff02400606950000000017a91466c7060feb882664ae62ffad0051fe843e318e85870000000000000000266a24aa21a9ede5c17d15b8b1fa2811b7e6da66ffa5e1aaa05922c69068bf90cd585b95bb46750120000000000000000000000000000000000000000000000000000000000000000000000000".to_owned()
}

fn sample_rich_tx_block_header(chain_ref: u32, block_height: u32) -> RichBlockHeader<BlockNumber> {
    let raw_header = RawBlockHeader::from_hex("0000003096cb3d93696c4f56c10da153963d35abf4692c07b2b3bf0702fb4cb32a8682211ee1fb90996ca1d5dcd12866ba9066458bf768641215933d7d8b3a10ef79d090e8a13a5effff7f2005000000".to_owned()).unwrap();
    RichBlockHeader::<BlockNumber> {
        block_header: parse_block_header(&raw_header).unwrap(),
        block_height,
        chain_ref,
        para_height: Default::default(),
    }
}

fn sample_valid_payment_output() -> TransactionOutput {
    TransactionOutput {
        value: 2500200000,
        script: "a91466c7060feb882664ae62ffad0051fe843e318e8587".try_into().unwrap(),
    }
}

fn sample_insufficient_value_payment_output() -> TransactionOutput {
    TransactionOutput {
        value: 100,
        script: "a91466c7060feb882664ae62ffad0051fe843e318e8587".try_into().unwrap(),
    }
}

fn sample_wrong_recipient_payment_output() -> TransactionOutput {
    TransactionOutput {
        value: 2500200000,
        script: "a914000000000000000000000000000000000000000087".try_into().unwrap(),
    }
}

fn sample_valid_data_output() -> TransactionOutput {
    TransactionOutput {
        value: 0,
        script: "6a20e5c17d15b8b1fa2811b7e6da66ffa5e1aaa05922c69068bf90cd585b95bb4675"
            .try_into()
            .unwrap(),
    }
}

fn sample_incorrect_data_output() -> TransactionOutput {
    TransactionOutput {
        value: 0,
        script: "6a24000000000000000000000000000000000000000000000000000000000000000000000000"
            .try_into()
            .unwrap(),
    }
}

fn sample_transaction_parsed(outputs: &Vec<TransactionOutput>) -> Transaction {
    let mut inputs: Vec<TransactionInput> = Vec::new();

    let spent_output_txid =
        hex::decode("b28f1e58af1d4db02d1b9f0cf8d51ece3dd5f5013fd108647821ea255ae5daff".to_owned()).unwrap();
    let input = TransactionInput {
        previous_hash: H256Le::from_bytes_le(&spent_output_txid),
        previous_index: 0,
        coinbase: false,
        height: None,
        script: hex::decode("16001443feac9ca9d20883126e30e962ca11fda07f808b".to_owned()).unwrap(),
        sequence: 4294967295,
        flags: 0,
        witness: vec![],
    };

    inputs.push(input);

    Transaction {
        version: 2,
        inputs,
        outputs: outputs.to_vec(),
        lock_at: LockTime::BlockHeight(203),
    }
}

fn sample_example_real_rawtx() -> String {
    "0200000000010140d43a99926d43eb0e619bf0b3d83b4a31f60c176beecfb9d35bf45e54d0f7420100000017160014a4b4ca48de0b3fffc15404a1acdc8dbaae226955ffffffff0100e1f5050000000017a9144a1154d50b03292b3024370901711946cb7cccc387024830450221008604ef8f6d8afa892dee0f31259b6ce02dd70c545cfcfed8148179971876c54a022076d771d6e91bed212783c9b06e0de600fab2d518fad6f15a2b191d7fbd262a3e0121039d25ab79f41f75ceaf882411fd41fa670a4c672c23ffaf0e361a969cde0692e800000000".to_owned()
}

fn sample_example_real_txid() -> String {
    "c586389e5e4b3acb9d6c8be1c19ae8ab2795397633176f5a6442a261bbdefc3a".to_owned()
}

fn sample_example_real_transaction_hash() -> String {
    "b759d39a8596b70b3a46700b83e1edb247e17ba58df305421864fe7a9ac142ea".to_owned()
}

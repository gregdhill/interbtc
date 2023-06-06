
//! Autogenerated weights for btc_relay
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `dev-benchmark`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("interlay-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// interlay-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 10
// --output
// ../tmp-weight/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for btc_relay using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> btc_relay::WeightInfo for WeightInfo<T> {

	/// Storage: BTCRelay BestBlock (r:1 w:1)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainCounter (r:1 w:1)
	/// Proof: BTCRelay ChainCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:1 w:1)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:0 w:1)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: BTCRelay StartBlockHeight (r:0 w:1)
	/// Proof: BTCRelay StartBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:0 w:1)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:0 w:1)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:0 w:1)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	fn initialize	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430`
		//  Estimated: `9470`
		// Minimum execution time: 84_891_000 picoseconds.
		Weight::from_parts(89_997_000, 9470)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: BTCRelay ChainCounter (r:1 w:0)
	/// Proof: BTCRelay ChainCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:2 w:1)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:1 w:1)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:1 w:1)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlock (r:0 w:1)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:0 w:1)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn store_block_header	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `901`
		//  Estimated: `17838`
		// Minimum execution time: 96_734_000 picoseconds.
		Weight::from_parts(104_906_000, 17838)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: BTCRelay ChainCounter (r:1 w:1)
	/// Proof: BTCRelay ChainCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:2 w:1)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:6 w:1)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:7 w:1)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlock (r:1 w:0)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:0 w:1)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// The range of component `f` is `[1, 6]`.
	fn store_block_header_new_fork_sorted	(f: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `841 + f * (92 ±0)`
		//  Estimated: `16800 + f * (5006 ±0)`
		// Minimum execution time: 122_921_000 picoseconds.
		Weight::from_parts(115_873_657, 16800)
			// Standard Error: 233_264
			.saturating_add(Weight::from_parts(13_772_544, 0).saturating_mul(f.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(f.into())))
			.saturating_add(T::DbWeight::get().writes(5_u64))
			.saturating_add(Weight::from_parts(0, 5006).saturating_mul(f.into()))
	}
	/// Storage: BTCRelay ChainCounter (r:1 w:1)
	/// Proof: BTCRelay ChainCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:2 w:1)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:2 w:1)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:7 w:6)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlock (r:1 w:0)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:0 w:1)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// The range of component `f` is `[1, 6]`.
	fn store_block_header_new_fork_unsorted	(f: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `920 + f * (98 ±0)`
		//  Estimated: `19871 + f * (2975 ±31)`
		// Minimum execution time: 124_552_000 picoseconds.
		Weight::from_parts(119_807_286, 19871)
			// Standard Error: 219_739
			.saturating_add(Weight::from_parts(16_025_685, 0).saturating_mul(f.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(f.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(f.into())))
			.saturating_add(Weight::from_parts(0, 2975).saturating_mul(f.into()))
	}
	/// Storage: BTCRelay ChainCounter (r:1 w:0)
	/// Proof: BTCRelay ChainCounter (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:20 w:18)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsIndex (r:3 w:2)
	/// Proof: BTCRelay ChainsIndex (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableDifficultyCheck (r:1 w:0)
	/// Proof: BTCRelay DisableDifficultyCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay ChainsHashes (r:13 w:24)
	/// Proof: BTCRelay ChainsHashes (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:6 w:0)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	/// Proof: BTCRelay StableBitcoinConfirmations (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlock (r:0 w:1)
	/// Proof: BTCRelay BestBlock (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:0 w:1)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// The range of component `f` is `[3, 6]`.
	fn store_block_header_reorganize_chains	(f: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4937 + f * (199 ±0)`
		//  Estimated: `109561 + f * (1340 ±45)`
		// Minimum execution time: 650_977_000 picoseconds.
		Weight::from_parts(615_353_151, 109561)
			// Standard Error: 994_819
			.saturating_add(Weight::from_parts(22_332_173, 0).saturating_mul(f.into()))
			.saturating_add(T::DbWeight::get().reads(42_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(f.into())))
			.saturating_add(T::DbWeight::get().writes(46_u64))
			.saturating_add(Weight::from_parts(0, 1340).saturating_mul(f.into()))
	}
}
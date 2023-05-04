
//! Autogenerated weights for loans
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-13, STEPS: `100`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `sander-dell`, CPU: `11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("interlay-testnet-latest"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// interlay-testnet-latest
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 100
// --repeat
// 10
// --output
// parachain/runtime/testnet-interlay/src/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for loans using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> loans::WeightInfo for WeightInfo<T> {
	/// Storage: Loans Markets (r:2 w:1)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans UnderlyingAssetId (r:1 w:1)
	/// Proof Skipped: Loans UnderlyingAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans MinExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MinExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Loans ExchangeRate (r:0 w:1)
	/// Proof Skipped: Loans ExchangeRate (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans BorrowIndex (r:0 w:1)
	/// Proof Skipped: Loans BorrowIndex (max_values: None, max_size: None, mode: Measured)
	fn add_market() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1207`
		//  Estimated: `13955`
		// Minimum execution time: 41_618_000 picoseconds.
		Weight::from_parts(42_808_000, 13955)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Loans Markets (r:1 w:1)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	fn activate_market() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1504`
		//  Estimated: `3979`
		// Minimum execution time: 26_773_000 picoseconds.
		Weight::from_parts(28_878_000, 3979)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Loans Markets (r:1 w:1)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	fn update_rate_model() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1504`
		//  Estimated: `3979`
		// Minimum execution time: 26_702_000 picoseconds.
		Weight::from_parts(28_451_000, 3979)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Loans Markets (r:1 w:1)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	fn update_market() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1504`
		//  Estimated: `3979`
		// Minimum execution time: 28_263_000 picoseconds.
		Weight::from_parts(29_581_000, 3979)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Loans UnderlyingAssetId (r:1 w:1)
	/// Proof Skipped: Loans UnderlyingAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans Markets (r:1 w:1)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	fn force_update_market() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1515`
		//  Estimated: `7980`
		// Minimum execution time: 32_751_000 picoseconds.
		Weight::from_parts(33_602_000, 7980)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn add_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2242`
		//  Estimated: `7783`
		// Minimum execution time: 61_282_000 picoseconds.
		Weight::from_parts(62_808_000, 7783)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplySpeed (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplySpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowSpeed (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowSpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplyState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowState (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowState (max_values: None, max_size: None, mode: Measured)
	fn update_market_reward_speed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1518`
		//  Estimated: `22440`
		// Minimum execution time: 47_353_000 picoseconds.
		Weight::from_parts(48_353_000, 22440)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplyState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardSupplySpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplierIndex (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplierIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardAccrued (r:1 w:1)
	/// Proof Skipped: Loans RewardAccrued (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Loans RewardBorrowState (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardBorrowSpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans TotalBorrows (r:1 w:0)
	/// Proof Skipped: Loans TotalBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans BorrowIndex (r:1 w:0)
	/// Proof Skipped: Loans BorrowIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowerIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans AccountBorrows (r:1 w:0)
	/// Proof Skipped: Loans AccountBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3492`
		//  Estimated: `80995`
		// Minimum execution time: 138_438_000 picoseconds.
		Weight::from_parts(144_472_000, 80995)
			.saturating_add(T::DbWeight::get().reads(17_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplyState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardSupplySpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans Markets (r:1 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplierIndex (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplierIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardAccrued (r:1 w:1)
	/// Proof Skipped: Loans RewardAccrued (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Loans RewardBorrowState (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardBorrowSpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans TotalBorrows (r:1 w:0)
	/// Proof Skipped: Loans TotalBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans BorrowIndex (r:1 w:0)
	/// Proof Skipped: Loans BorrowIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowerIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans AccountBorrows (r:1 w:0)
	/// Proof Skipped: Loans AccountBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_reward_for_market() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3492`
		//  Estimated: `78520`
		// Minimum execution time: 127_833_000 picoseconds.
		Weight::from_parts(133_696_000, 78520)
			.saturating_add(T::DbWeight::get().reads(16_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof Skipped: Loans LastAccruedInterestTime (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplyState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardSupplySpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplierIndex (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplierIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardAccrued (r:1 w:1)
	/// Proof Skipped: Loans RewardAccrued (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof Skipped: Loans UnderlyingAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Loans TotalBorrows (r:1 w:0)
	/// Proof Skipped: Loans TotalBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans TotalReserves (r:1 w:0)
	/// Proof Skipped: Loans TotalReserves (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans MinExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MinExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Loans AccountDeposits (r:1 w:0)
	/// Proof Skipped: Loans AccountDeposits (max_values: None, max_size: None, mode: Measured)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2944`
		//  Estimated: `73490`
		// Minimum execution time: 175_466_000 picoseconds.
		Weight::from_parts(179_716_000, 73490)
			.saturating_add(T::DbWeight::get().reads(18_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof Skipped: Loans LastAccruedInterestTime (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans TotalBorrows (r:1 w:1)
	/// Proof Skipped: Loans TotalBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Loans TotalReserves (r:1 w:0)
	/// Proof Skipped: Loans TotalReserves (max_values: None, max_size: None, mode: Measured)
	/// Storage: Security ParachainStatus (r:1 w:0)
	/// Proof Skipped: Security ParachainStatus (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle Aggregate (r:1 w:0)
	/// Proof Skipped: Oracle Aggregate (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans AccountDeposits (r:1 w:0)
	/// Proof Skipped: Loans AccountDeposits (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof Skipped: Loans UnderlyingAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Loans MinExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MinExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Loans MaxExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MaxExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Loans AccountBorrows (r:1 w:1)
	/// Proof Skipped: Loans AccountBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowState (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardBorrowSpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowerIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardAccrued (r:1 w:1)
	/// Proof Skipped: Loans RewardAccrued (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans BorrowIndex (r:1 w:0)
	/// Proof Skipped: Loans BorrowIndex (max_values: None, max_size: None, mode: Measured)
	fn borrow() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3564`
		//  Estimated: `103955`
		// Minimum execution time: 173_830_000 picoseconds.
		Weight::from_parts(180_461_000, 103955)
			.saturating_add(T::DbWeight::get().reads(22_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof Skipped: Loans UnderlyingAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof Skipped: Loans LastAccruedInterestTime (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Loans TotalBorrows (r:1 w:0)
	/// Proof Skipped: Loans TotalBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans TotalReserves (r:1 w:0)
	/// Proof Skipped: Loans TotalReserves (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans MinExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MinExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Loans MaxExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MaxExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Loans AccountDeposits (r:1 w:0)
	/// Proof Skipped: Loans AccountDeposits (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplyState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardSupplySpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplierIndex (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplierIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardAccrued (r:1 w:1)
	/// Proof Skipped: Loans RewardAccrued (max_values: None, max_size: None, mode: Measured)
	fn redeem() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3368`
		//  Estimated: `82017`
		// Minimum execution time: 197_346_000 picoseconds.
		Weight::from_parts(220_509_000, 82017)
			.saturating_add(T::DbWeight::get().reads(19_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof Skipped: Loans UnderlyingAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof Skipped: Loans LastAccruedInterestTime (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Loans TotalBorrows (r:1 w:0)
	/// Proof Skipped: Loans TotalBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans TotalReserves (r:1 w:0)
	/// Proof Skipped: Loans TotalReserves (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans MinExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MinExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Loans MaxExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MaxExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Loans AccountDeposits (r:1 w:0)
	/// Proof Skipped: Loans AccountDeposits (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplyState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardSupplySpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplierIndex (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplierIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardAccrued (r:1 w:1)
	/// Proof Skipped: Loans RewardAccrued (max_values: None, max_size: None, mode: Measured)
	fn redeem_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3368`
		//  Estimated: `82017`
		// Minimum execution time: 195_823_000 picoseconds.
		Weight::from_parts(208_789_000, 82017)
			.saturating_add(T::DbWeight::get().reads(19_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof Skipped: Loans LastAccruedInterestTime (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans AccountBorrows (r:1 w:1)
	/// Proof Skipped: Loans AccountBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans BorrowIndex (r:1 w:0)
	/// Proof Skipped: Loans BorrowIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowState (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardBorrowSpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowerIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardAccrued (r:1 w:1)
	/// Proof Skipped: Loans RewardAccrued (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Loans TotalBorrows (r:1 w:1)
	/// Proof Skipped: Loans TotalBorrows (max_values: None, max_size: None, mode: Measured)
	fn repay_borrow() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3407`
		//  Estimated: `61096`
		// Minimum execution time: 116_486_000 picoseconds.
		Weight::from_parts(124_461_000, 61096)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof Skipped: Loans LastAccruedInterestTime (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans AccountBorrows (r:1 w:1)
	/// Proof Skipped: Loans AccountBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans BorrowIndex (r:1 w:0)
	/// Proof Skipped: Loans BorrowIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowState (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardBorrowSpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowerIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardAccrued (r:1 w:1)
	/// Proof Skipped: Loans RewardAccrued (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Loans TotalBorrows (r:1 w:1)
	/// Proof Skipped: Loans TotalBorrows (max_values: None, max_size: None, mode: Measured)
	fn repay_borrow_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3407`
		//  Estimated: `61096`
		// Minimum execution time: 124_777_000 picoseconds.
		Weight::from_parts(137_936_000, 61096)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof Skipped: Loans UnderlyingAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans AccountDeposits (r:1 w:1)
	/// Proof Skipped: Loans AccountDeposits (max_values: None, max_size: None, mode: Measured)
	fn deposit_all_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2295`
		//  Estimated: `19375`
		// Minimum execution time: 65_891_000 picoseconds.
		Weight::from_parts(70_468_000, 19375)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans AccountDeposits (r:1 w:1)
	/// Proof Skipped: Loans AccountDeposits (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof Skipped: Loans UnderlyingAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof Skipped: Loans LastAccruedInterestTime (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Loans TotalBorrows (r:1 w:0)
	/// Proof Skipped: Loans TotalBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans TotalReserves (r:1 w:0)
	/// Proof Skipped: Loans TotalReserves (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans MinExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MinExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Loans MaxExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MaxExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Security ParachainStatus (r:1 w:0)
	/// Proof Skipped: Security ParachainStatus (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle Aggregate (r:1 w:0)
	/// Proof Skipped: Oracle Aggregate (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans AccountBorrows (r:1 w:0)
	/// Proof Skipped: Loans AccountBorrows (max_values: None, max_size: None, mode: Measured)
	fn withdraw_all_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3494`
		//  Estimated: `72990`
		// Minimum execution time: 152_292_000 picoseconds.
		Weight::from_parts(154_881_000, 72990)
			.saturating_add(T::DbWeight::get().reads(17_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:2 w:2)
	/// Proof Skipped: Loans LastAccruedInterestTime (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans Markets (r:3 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans AccountDeposits (r:4 w:1)
	/// Proof Skipped: Loans AccountDeposits (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof Skipped: Loans UnderlyingAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:6 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:4 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Loans TotalBorrows (r:2 w:1)
	/// Proof Skipped: Loans TotalBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans TotalReserves (r:1 w:0)
	/// Proof Skipped: Loans TotalReserves (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans MinExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MinExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Loans MaxExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MaxExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Security ParachainStatus (r:1 w:0)
	/// Proof Skipped: Security ParachainStatus (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Oracle Aggregate (r:2 w:0)
	/// Proof Skipped: Oracle Aggregate (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans AccountBorrows (r:3 w:1)
	/// Proof Skipped: Loans AccountBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans BorrowIndex (r:1 w:0)
	/// Proof Skipped: Loans BorrowIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowState (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowSpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardBorrowSpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardBorrowerIndex (r:1 w:1)
	/// Proof Skipped: Loans RewardBorrowerIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardAccrued (r:3 w:3)
	/// Proof Skipped: Loans RewardAccrued (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplyState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardSupplySpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplierIndex (r:3 w:3)
	/// Proof Skipped: Loans RewardSupplierIndex (max_values: None, max_size: None, mode: Measured)
	fn liquidate_borrow() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5267`
		//  Estimated: `204773`
		// Minimum execution time: 484_507_000 picoseconds.
		Weight::from_parts(498_696_000, 204773)
			.saturating_add(T::DbWeight::get().reads(45_u64))
			.saturating_add(T::DbWeight::get().writes(21_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof Skipped: Loans LastAccruedInterestTime (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof Skipped: Loans UnderlyingAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Loans TotalBorrows (r:1 w:0)
	/// Proof Skipped: Loans TotalBorrows (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans TotalReserves (r:1 w:0)
	/// Proof Skipped: Loans TotalReserves (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans MinExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MinExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Loans MaxExchangeRate (r:1 w:0)
	/// Proof Skipped: Loans MaxExchangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Loans AccountDeposits (r:1 w:0)
	/// Proof Skipped: Loans AccountDeposits (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplyState (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof Skipped: Loans RewardSupplySpeed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardSupplierIndex (r:1 w:1)
	/// Proof Skipped: Loans RewardSupplierIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans RewardAccrued (r:1 w:1)
	/// Proof Skipped: Loans RewardAccrued (max_values: None, max_size: None, mode: Measured)
	fn reduce_incentive_reserves() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4362`
		//  Estimated: `99138`
		// Minimum execution time: 256_764_000 picoseconds.
		Weight::from_parts(269_959_000, 99138)
			.saturating_add(T::DbWeight::get().reads(21_u64))
			.saturating_add(T::DbWeight::get().writes(10_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof Skipped: Loans LastAccruedInterestTime (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Loans TotalReserves (r:1 w:1)
	/// Proof Skipped: Loans TotalReserves (max_values: None, max_size: None, mode: Measured)
	fn add_reserves() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2768`
		//  Estimated: `26490`
		// Minimum execution time: 85_693_000 picoseconds.
		Weight::from_parts(92_952_000, 26490)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof Skipped: Loans Markets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof Skipped: Loans LastAccruedInterestTime (max_values: None, max_size: None, mode: Measured)
	/// Storage: Loans TotalReserves (r:1 w:1)
	/// Proof Skipped: Loans TotalReserves (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn reduce_reserves() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2857`
		//  Estimated: `26757`
		// Minimum execution time: 73_803_000 picoseconds.
		Weight::from_parts(76_275_000, 26757)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}
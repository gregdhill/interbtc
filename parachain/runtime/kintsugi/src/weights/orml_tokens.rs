
//! Autogenerated weights for orml_tokens
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `dev-benchmark`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// kintsugi-dev
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

/// Weights for orml_tokens using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> orml_tokens::WeightInfo for WeightInfo<T> {

	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof: Loans UnderlyingAssetId (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof: Loans RewardSupplyState (max_values: None, max_size: Some(47), added: 2522, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof: Loans RewardSupplySpeed (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof: Loans Markets (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplierIndex (r:2 w:2)
	/// Proof: Loans RewardSupplierIndex (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Loans RewardAccrued (r:2 w:2)
	/// Proof: Loans RewardAccrued (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Loans AccountDeposits (r:1 w:1)
	/// Proof: Loans AccountDeposits (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn transfer	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1517`
		//  Estimated: `42199`
		// Minimum execution time: 261_154_000 picoseconds.
		Weight::from_parts(272_290_000, 42199)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof: Loans UnderlyingAssetId (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof: Loans RewardSupplyState (max_values: None, max_size: Some(47), added: 2522, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof: Loans RewardSupplySpeed (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof: Loans Markets (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplierIndex (r:2 w:2)
	/// Proof: Loans RewardSupplierIndex (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Loans RewardAccrued (r:2 w:2)
	/// Proof: Loans RewardAccrued (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Loans AccountDeposits (r:1 w:1)
	/// Proof: Loans AccountDeposits (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn transfer_all	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1517`
		//  Estimated: `42199`
		// Minimum execution time: 275_185_000 picoseconds.
		Weight::from_parts(301_987_000, 42199)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof: Loans UnderlyingAssetId (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof: Loans RewardSupplyState (max_values: None, max_size: Some(47), added: 2522, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof: Loans RewardSupplySpeed (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof: Loans Markets (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplierIndex (r:2 w:2)
	/// Proof: Loans RewardSupplierIndex (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Loans RewardAccrued (r:2 w:2)
	/// Proof: Loans RewardAccrued (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Loans AccountDeposits (r:1 w:1)
	/// Proof: Loans AccountDeposits (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn transfer_keep_alive	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1517`
		//  Estimated: `42199`
		// Minimum execution time: 262_301_000 picoseconds.
		Weight::from_parts(281_639_000, 42199)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof: Loans UnderlyingAssetId (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof: Loans RewardSupplyState (max_values: None, max_size: Some(47), added: 2522, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof: Loans RewardSupplySpeed (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof: Loans Markets (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplierIndex (r:2 w:2)
	/// Proof: Loans RewardSupplierIndex (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Loans RewardAccrued (r:2 w:2)
	/// Proof: Loans RewardAccrued (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Loans AccountDeposits (r:1 w:1)
	/// Proof: Loans AccountDeposits (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn force_transfer	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1517`
		//  Estimated: `42199`
		// Minimum execution time: 264_521_000 picoseconds.
		Weight::from_parts(282_929_000, 42199)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	fn set_balance	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `218`
		//  Estimated: `7080`
		// Minimum execution time: 67_303_000 picoseconds.
		Weight::from_parts(70_396_000, 7080)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

//! Autogenerated weights for `pallet_property_management`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-07-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `recrafter-Legion-5-16IRX9`, CPU: `Intel(R) Core(TM) i7-14650HX`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_property_management
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/property-management/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

pub trait WeightInfo {
	fn add_letting_agent() -> Weight;
	fn letting_agent_deposit() -> Weight;
	fn add_letting_agent_to_location() -> Weight;
	fn set_letting_agent() -> Weight;
	fn distribute_income() -> Weight;
	fn withdraw_funds() -> Weight;
}

/// Weight functions for `pallet_property_management`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `NftMarketplace::RegionCollections` (r:1 w:0)
	/// Proof: `NftMarketplace::RegionCollections` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::LocationRegistration` (r:1 w:0)
	/// Proof: `NftMarketplace::LocationRegistration` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `PropertyManagement::LettingInfo` (r:1 w:1)
	/// Proof: `PropertyManagement::LettingInfo` (`max_values`: None, `max_size`: Some(5189), added: 7664, mode: `MaxEncodedLen`)
	fn add_letting_agent() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `294`
		//  Estimated: `8654`
		// Minimum execution time: 19_380_000 picoseconds.
		Weight::from_parts(20_176_000, 0)
			.saturating_add(Weight::from_parts(0, 8654))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `PropertyManagement::LettingInfo` (r:1 w:1)
	/// Proof: `PropertyManagement::LettingInfo` (`max_values`: None, `max_size`: Some(5189), added: 7664, mode: `MaxEncodedLen`)
	/// Storage: `PropertyManagement::LettingAgentLocations` (r:1 w:1)
	/// Proof: `PropertyManagement::LettingAgentLocations` (`max_values`: None, `max_size`: Some(3249), added: 5724, mode: `MaxEncodedLen`)
	fn letting_agent_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `168`
		//  Estimated: `8654`
		// Minimum execution time: 29_393_000 picoseconds.
		Weight::from_parts(30_367_000, 0)
			.saturating_add(Weight::from_parts(0, 8654))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `PropertyManagement::LettingInfo` (r:1 w:1)
	/// Proof: `PropertyManagement::LettingInfo` (`max_values`: None, `max_size`: Some(5189), added: 7664, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::LocationRegistration` (r:1 w:0)
	/// Proof: `NftMarketplace::LocationRegistration` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `PropertyManagement::LettingAgentLocations` (r:1 w:1)
	/// Proof: `PropertyManagement::LettingAgentLocations` (`max_values`: None, `max_size`: Some(3249), added: 5724, mode: `MaxEncodedLen`)
	fn add_letting_agent_to_location() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `546`
		//  Estimated: `8654`
		// Minimum execution time: 26_592_000 picoseconds.
		Weight::from_parts(27_335_000, 0)
			.saturating_add(Weight::from_parts(0, 8654))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `NftMarketplace::AssetIdDetails` (r:1 w:0)
	/// Proof: `NftMarketplace::AssetIdDetails` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `PropertyManagement::LettingStorage` (r:1 w:1)
	/// Proof: `PropertyManagement::LettingStorage` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `PropertyManagement::LettingAgentLocations` (r:1 w:0)
	/// Proof: `PropertyManagement::LettingAgentLocations` (`max_values`: None, `max_size`: Some(3249), added: 5724, mode: `MaxEncodedLen`)
	/// Storage: `PropertyManagement::LettingInfo` (r:1 w:1)
	/// Proof: `PropertyManagement::LettingInfo` (`max_values`: None, `max_size`: Some(5189), added: 7664, mode: `MaxEncodedLen`)
	fn set_letting_agent() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `692`
		//  Estimated: `8654`
		// Minimum execution time: 31_100_000 picoseconds.
		Weight::from_parts(32_057_000, 0)
			.saturating_add(Weight::from_parts(0, 8654))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `PropertyManagement::LettingStorage` (r:1 w:0)
	/// Proof: `PropertyManagement::LettingStorage` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::PropertyOwner` (r:1 w:0)
	/// Proof: `NftMarketplace::PropertyOwner` (`max_values`: None, `max_size`: Some(8022), added: 10497, mode: `MaxEncodedLen`)
	/// Storage: `PropertyManagement::PropertyReserve` (r:1 w:0)
	/// Proof: `PropertyManagement::PropertyReserve` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::AssetIdDetails` (r:1 w:0)
	/// Proof: `NftMarketplace::AssetIdDetails` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `PropertyManagement::PropertyDebts` (r:1 w:0)
	/// Proof: `PropertyManagement::PropertyDebts` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `NftMarketplace::PropertyOwnerToken` (r:1 w:0)
	/// Proof: `NftMarketplace::PropertyOwnerToken` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `PropertyManagement::StoredFunds` (r:1 w:1)
	/// Proof: `PropertyManagement::StoredFunds` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	fn distribute_income() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `825`
		//  Estimated: `11487`
		// Minimum execution time: 68_133_000 picoseconds.
		Weight::from_parts(69_996_000, 0)
			.saturating_add(Weight::from_parts(0, 11487))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `PropertyManagement::StoredFunds` (r:1 w:1)
	/// Proof: `PropertyManagement::StoredFunds` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn withdraw_funds() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `312`
		//  Estimated: `3593`
		// Minimum execution time: 45_140_000 picoseconds.
		Weight::from_parts(46_695_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2022  Nodle International
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Autogenerated weights for pallet_multisig
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-3403be85`, CPU: `AMD EPYC 7B13`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --template=./.maintain/external_pallet_weights.hbs
// --output=runtimes/eden/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use core::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Minimum execution time: 18_070 nanoseconds.
		Weight::from_parts(19_143_628_u64, 0)
			// Standard Error: 4
			.saturating_add(Weight::from_parts(529_u64, 0).saturating_mul(z as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 68_090 nanoseconds.
		Weight::from_parts(61_578_593_u64, 0)
			// Standard Error: 1_487
			.saturating_add(Weight::from_parts(86_648_u64, 0).saturating_mul(s as u64))
			// Standard Error: 14
			.saturating_add(Weight::from_parts(1_433_u64, 0).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 42_440 nanoseconds.
		Weight::from_parts(36_057_225_u64, 0)
			// Standard Error: 810
			.saturating_add(Weight::from_parts(79_147_u64, 0).saturating_mul(s as u64))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(1_463_u64, 0).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 75_550 nanoseconds.
		Weight::from_parts(65_802_118_u64, 0)
			// Standard Error: 1_536
			.saturating_add(Weight::from_parts(114_949_u64, 0).saturating_mul(s as u64))
			// Standard Error: 15
			.saturating_add(Weight::from_parts(1_558_u64, 0).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Minimum execution time: 55_760 nanoseconds.
		Weight::from_parts(59_086_958_u64, 0)
			// Standard Error: 1_917
			.saturating_add(Weight::from_parts(83_080_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Minimum execution time: 31_790 nanoseconds.
		Weight::from_parts(33_473_418_u64, 0)
			// Standard Error: 863
			.saturating_add(Weight::from_parts(83_895_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Proof: Multisig Multisigs (max_values: None, max_size: Some(3346), added: 5821, mode: MaxEncodedLen)
	// Storage: System Number (r:1 w:0)
	// Proof: System Number (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Proof: System ExecutionPhase (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: System EventCount (r:1 w:1)
	// Proof: System EventCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: System Events (r:1 w:1)
	// Proof Skipped: System Events (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Minimum execution time: 56_970 nanoseconds.
		Weight::from_parts(59_895_460_u64, 0)
			// Standard Error: 1_539
			.saturating_add(Weight::from_parts(87_760_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

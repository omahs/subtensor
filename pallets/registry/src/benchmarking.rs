//! Benchmarking setup
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Registry;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

use sp_runtime::traits::{StaticLookup, Bounded};
use frame_support::traits::Get;
use sp_std::mem::size_of;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

// This creates an `IdentityInfo` object with `num_fields` extra fields.
// All data is pre-populated with some arbitrary bytes.
fn create_identity_info<T: Config>(num_fields: u32) -> IdentityInfo<T::MaxAdditionalFields> {
	let data = Data::Raw(vec![0; 32].try_into().unwrap());

	IdentityInfo {
		additional: Default::default(),
		display: data.clone(),
		legal: data.clone(),
		web: data.clone(),
		riot: data.clone(),
		email: data.clone(),
		pgp_fingerprint: Some([0; 20]),
		image: data.clone(),
		twitter: data,
	}
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_identity() {
		// The target user
		let caller: T::AccountId = whitelisted_caller();
		let _ = T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), Box::new(create_identity_info::<T>(0)));

		assert_last_event::<T>(Event::<T>::IdentitySet { who: caller }.into());
	}

	#[benchmark]
	fn clear_identity() {
		// The target user
		let caller: T::AccountId = whitelisted_caller();
		let _ = T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

		Registry::<T>::set_identity(RawOrigin::Signed(caller.clone()).into(), Box::new(create_identity_info::<T>(0)));

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()));

		assert_last_event::<T>(Event::<T>::IdentityDissolved { who: caller }.into());
	}

	impl_benchmark_test_suite!(Registry, crate::mock::new_test_ext(), crate::mock::Test);
}

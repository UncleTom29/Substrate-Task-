//! Benchmarking setup for RotaryClub
#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	set_transaction_benchmark {
		let b in 1 .. 1000;
	}: set_transaction(RawOrigin::Root, b.into())

	verify {
		assert_eq!(RotaryClub::add_to_awaitinglist(Origin::signed(100)), Ok(b.into()))
	}
	
	accumulate_transaction {
		let b in 1 .. 1000;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), b.into())
	
	sort_vector {
		let x in 0 .. 10000;
		let mut m = Vec::<u32>::new();
		for i in (0..x).rev() {
			m.push(i);
		}
	}: {
		m.sort_unstable();
	}


	impl_benchmark_test_suite!(RotaryClub, crate::test::new_test_ext(), crate::tests::Test);
}

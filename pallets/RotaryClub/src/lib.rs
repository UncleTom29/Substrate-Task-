#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::storage::bounded_vec::BoundedVec;
	use frame_support::{
		dispatch::{DispatchResult, PartialEq},
		pallet_prelude::*,
	
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MAXIMUM: Get<u32>;
	}

	// The pallet's runtime storage items.
	
	#[pallet::storage]
	#[pallet::getter(fn get_initial_members)]

	pub type InitialMembers<T: Config> = StorageValue<_, BoundedVec<RotaryClubMembers<T>, T::MAXIMUM>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_awaiting_origin_approval)]

	pub type AwaitingOriginApproval<T: Config> = StorageValue<_, BoundedVec<T::AccountId>, T::MAXIMUM>, ValueQuery>;
	
	#[derive(Encode, Decode, PartialEq, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	
	pub struct RotaryClubMembers<T: Config> {
		member_details: T::AccountId,

		block_details: T::BlockNumber
	}
	// Pallets use events to inform users when important changes are made.

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		AwaitingListMemberAdded(T::AccountId),
		AwaitingListMemberRemoved(T::AccountId),
		RotaryClubMemberAdded(T::AccountId),
		RotaryClubMemberRemoved(T::AccountId),

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		// The applicant already included in the awaiting list
		AlreadyOnAwaitingList,
		// Not on Awaiting List
		NotOnAwaitingList,
		// AwaitingListAlreadyFull
		AwaitingListFull,
		// The member already exist 
		RotaryClubMemberAlready, 
		//Too many members
		RotaryClubFull,
		// Not a Member of Rotary Club
		NotMember,

	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_to_awaitinglist(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
	
			let applicant = ensure_signed(origin)?;

			ensure!(
				<AwaitingOriginApproval<T>>::get().contains(&applicant) == false,
				<Error<T>>::AlreadyOnAwaitingList
			);
			ensure!(
				<InitialMembers<T>>::get().contains(&applicant) == false,
				<Error<T>>::RotaryClubMemberAlready
			);

			<AwaitingOriginApproval<T>>::try_mutate(|b_vec| b_vec.try_push(applicant.clone()))
				.map_err(|_| <Error<T>>::AwaitingListFull)?;

			<InitialMembers<T>>::try_mutate(|b_vec| b_vec.try_push(applicant.clone()))
				.map_err(|_| <Error<T>>::RotaryClubFull)?;

			// Emit an event.
			Self::deposit_event(Event::AwaitingListMemberAdded(applicant));

			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}

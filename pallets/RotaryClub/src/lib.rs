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
	}
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
		NotRotaryClubMember,

	}
// This trransaction can be signed by anyone interested in joining rotary club, except from the 'Root' origin account.
// It will return an error if the applicant is already on the 'Awaiting list', or already a Rotary club member.
// It will also return an error if the Awaiting list or Rotary club is filled up.


	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000)]
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

			<AwaitingOriginApproval<T>>::try_mutate(|awaiting_list| awaiting_list.try_push(applicant.clone()))
				.map_err(|_| <Error<T>>::AwaitingListFull)?;

			<InitialMembers<T>>::try_mutate(|awaiting_list| awaiting_list.try_push(applicant.clone()))
				.map_err(|_| <Error<T>>::RotaryClubFull)?;

			// Emit an event.
			Self::deposit_event(Event::AwaitingListMemberAdded(applicant));

			Ok(())
		}

		// This trransaction can be signed by applicants (people already on awaiting list) interested in leaving the awaiting list without entering the Rotary club.
		// It will return an error if the signer is not on awaiting list. 

		#[pallet::weight(10_000)]
		pub fn remove_from_awaitinglist(origin: OriginFor<T>, applicant: T::AccountId) -> DispatchResult {
			let applicant = ensure_signed(origin)?;
		let mut awaiting_list_members = <AwaitingOriginApproval<T>>::get();

		<AwaitingOriginApproval<T>>::try_mutate(|awaiting_list| {
		
			if let Some(index) = awaiting_list.iter().position(|awaiting_list_members| awaiting_list_members.member_details == applicant) {
				awaiting_list.remove(index);
				return Ok(());
			}
			Err(())
		})
		.map_err(|_| <Error<T>>::NotOnAwaitingList)?;
			Self::deposit_event(Event::AwaitingListMemberRemoved(applicant));
			Ok(())
		}


		// This function can only be executed by the 'Root' origin account.
		// If any other account attempts to execute it, it will return a custom error.
		// This function will add applicants to the rotary club, and also automatically remove them from awaiting list.
		// It will also return an error if the rotary club is full.

		#[pallet::weight(10000)]
		pub fn add_to_rotaryclub(origin: OriginFor<T>, member: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(Self::get_awaiting_origin_approval.contains(&member) == true, <Error<T>>::NotOnAwaitingList);
			let BlockNumber = <frame_system::Pallet<T>>::block_number();
			let value = RotaryClubMembers { member_details: member.clone(), block_details: BlockNumber};
			<InitialMembers<T>>::try_mutate(|awaiting_list| awaiting_list.try_push(value))
			.map_err(|_| <Error<T>>::RotaryClubFull)?;

			<AwaitingOriginApproval<T>>::try_mutate(|remove_from_awaitinglist_after_induction|{
				if let Some(index) = remove_from_awaitinglist_after_induction.iter().position(|value| *value == member) {
					remove_from_awaitinglist_after_induction.remove(index);
					return Ok(());
				}
				Err(())
			})
			.map_err(|_| <Error<T>>::NotOnAwaitingList)?;

			Self::deposit_event(Event::AwaitingListMemberRemoved(member.clone()));
			Self::deposit_event(Event::RotaryClubMemberAdded(member.clone()));
			Ok(())

		}
		// This function can only be executed by the 'Root' origin account.
		// If any other account attempts to execute it, it will return a custom error.
		// This function will remove members from the rotary club.
		// It will return an error if the account to be removed is not a member of the rotary club.

		#[pallet::weight(10000)]
		pub fn remove_from_rotary_club(origin: OriginFor<T>, member: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<InitialMembers<T>>::try_mutate(|rotaryclub_members| {
				if let Some(index) = rotaryclub_members.iter().position(|value| value.member_details == member) {
					rotaryclub_members.remove(index);
					return Ok(());
				}
				Err(())
			})
			.map_err(|_| <Error<T>>::NotRotaryClubMember)?;
			Self::deposit_event(Event::RotaryClubMemberRemoved(member));
			Ok(())

		}


		}
	


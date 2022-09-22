use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, assert_err, error::OriginError };

#[test]
fn add_to_awaitinglist() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(RotaryClub::add_to_awaitinglist(Origin::signed(1)));
		// Read pallet storage and assert an expected result.
		assert_eq!(RotaryClub::add_to_awaitinglist(Origin::signed(100)), Ok());
	});
}

#[test]
fn remove_from_awaitinglist() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(RotaryClub::remove_from_awaitinglist(Origin::signed(1)));
		// Read pallet storage and assert an expected result.
		assert_eq!(RotaryClub::remove_from_awaitinglist(Origin::signed(100)), Ok());
	});
}

#[test]
fn add_to_rotaryclub() {
    new_test_ext().execute_with(|| {

		
        assert_err!(
            RotaryClub::add_to_rotaryclub(Origin::root(), 100), 
			Error::<Test>::NotRotaryClubMember
        );
    })
}


#[test]
fn remove_from_rotary_club() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(RotaryClub::remove_from_rotary_club(Origin::signed(1),(100), OriginError);
	});
}



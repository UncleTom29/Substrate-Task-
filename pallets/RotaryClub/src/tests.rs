use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, assert_err, error::OriginError };

#[test]
fn add_to_awaitinglist() {
	new_test_ext().execute_with(|| {
		// It confirms if anyone is anyone can join to awaiting list.
		assert_ok!(RotaryClub::add_to_awaitinglist(Origin::signed(1)));
		// Should return OK
		assert_eq!(RotaryClub::add_to_awaitinglist(Origin::signed(100)), Ok());
	});
}

#[test]
fn remove_from_awaitinglist() {
	new_test_ext().execute_with(|| {
		// It confirms if anyone is anyone can leave the awaiting list.
		assert_ok!(RotaryClub::remove_from_awaitinglist(Origin::signed(1)));
		// Should return OK
		assert_eq!(RotaryClub::remove_from_awaitinglist(Origin::signed(100)), Ok());
	});
}

#[test]
fn add_to_rotaryclub() {
    new_test_ext().execute_with(|| {

		// Adding a user not on awaiting list 
        assert_err!(
            RotaryClub::add_to_rotaryclub(Origin::root(), 100), 
			Error::<Test>::NotOnAwaitingList
        );
    })
}


#[test]
fn remove_from_rotary_club() {
	new_test_ext().execute_with(|| {
		// Transaction can only be submitted by the origin, should return an error if another user tries to perform it.
		assert_noop!(RotaryClub::remove_from_rotary_club(Origin::signed(1),(100), OriginError);
	});
}



use super::*;
use crate::{mock::*};
use frame_support::{assert_ok, assert_noop};
use codec::{Decode, Encode};

#[test]
fn request_no_register() {
	new_test_ext().execute_with(|| {
		assert!(AresModule::initiate_request(Origin::signed(2), 1, vec![], 1, vec![]).is_err());
	});
}

#[test]
fn operators_can_be_registered() {
	new_test_ext().execute_with(|| {
		assert!(AresModule::register_operator(Origin::signed(1)).is_ok());
		assert!(AresModule::unregister_operator(Origin::signed(1)).is_ok());
	});

	new_test_ext().execute_with(|| {
		assert!(AresModule::unregister_operator(Origin::signed(1)).is_err());
	});

}

#[test]
fn unknown_operator() {
	new_test_ext().execute_with(|| {
		assert!(AresModule::register_operator(Origin::signed(1)).is_ok());
		assert!(<Operators<Test>>::contains_key(1));
		assert!(AresModule::initiate_request(Origin::signed(1), 2, vec![], 1, vec![]).is_err());
	});
}

#[test]
fn operator_no_register() {
	new_test_ext().execute_with(|| {
		assert!(AresModule::callback(Origin::signed(1), 0, 10).is_err());
	});
}

#[test]
fn callback_not_match_operator() {
	new_test_ext().execute_with(|| {
		assert!(AresModule::register_operator(Origin::signed(1)).is_ok());
		assert!(AresModule::initiate_request(Origin::signed(2), 1, vec![], 1, vec![]).is_ok());
		assert!(AresModule::callback(Origin::signed(3), 0, 10).is_err());
	});
}

#[test]
pub fn on_finalize() {
	new_test_ext().execute_with(|| {
		assert!(AresModule::register_operator(Origin::signed(1)).is_ok());
		assert!(AresModule::initiate_request(Origin::signed(1), 1, vec![], 1, vec![]).is_ok());
		// Request has been killed, too old
		assert!(AresModule::callback(Origin::signed(1), 0, 10).is_ok());
	});
}
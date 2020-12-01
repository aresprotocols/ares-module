use super::*;
use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use frame_system::{EventRecord, Phase};
use sp_runtime::{
	traits::{BlakeTwo256, OnFinalize, IdentityLookup}};
use codec::{Codec, Decode, Encode};

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
fn initiate_requests() {

	new_test_ext().execute_with(|| {
		assert!(AresModule::register_operator(Origin::signed(1)).is_ok());
		assert!(AresModule::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 0, module2::Call::<Runtime>::callback(vec![]).into()).is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(AresModule::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 1, module2::Call::<Runtime>::callback(vec![]).into()).is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(AresModule::register_operator(Origin::signed(1)).is_ok());
		assert!(AresModule::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 2, module2::Call::<Runtime>::callback(vec![]).into()).is_ok());
		assert!(AresModule::callback(Origin::signed(3), 0, 10.encode()).is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(AresModule::callback(Origin::signed(1), 0, 10.encode()).is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(AresModule::register_operator(Origin::signed(1)).is_ok());

		assert_eq!(
			*System::events().last().unwrap(),
			EventRecord {
				phase: Phase::ApplyExtrinsic(0),
				event: TestEvent::areslink(RawEvent::OperatorRegistered(1)),
				topics: vec![],
			}
		);

		let parameters = ("a", "b");
		let data = parameters.encode();
		assert!(AresModule::initiate_request(Origin::signed(2), 1, vec![], 1, data.clone(), 2, module2::Call::<Runtime>::callback(vec![]).into()).is_ok());

		assert_eq!(
			*System::events().last().unwrap(),
			EventRecord {
				phase: Phase::ApplyExtrinsic(0),
				event: TestEvent::areslink(RawEvent::OracleRequest(1, vec![], 0, 2, 1, data.clone(), "Ares.callback".into(),)),
				topics: vec![],
			}
		);

		let r = <(Vec<u8>, Vec<u8>)>::decode(&mut &data[..]).unwrap().0;
		assert_eq!("a", std::str::from_utf8(&r).unwrap());

		let result = 10;
		assert!(AresModule::callback(Origin::signed(1), 0, result).is_ok());
		assert_eq!(module2::Result::get(), result);
	});

}

#[test]
pub fn on_finalize() {

	new_test_ext().execute_with(|| {
		assert!(AresModule::register_operator(Origin::signed(1)).is_ok());
		assert!(AresModule::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 2, module2::Call::<Runtime>::callback(vec![]).into()).is_ok());
		// Request has been killed, too old
		assert!(AresModule::callback(Origin::signed(1), 0, 10).is_err());
	});

}
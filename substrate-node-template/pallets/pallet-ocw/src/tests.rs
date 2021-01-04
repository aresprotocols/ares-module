use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}

#[test]
fn correct_error_for_value() {
	new_test_ext().execute_with(|| {

		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));

		assert_ok!(TemplateModule::cause_error(Origin::signed(1)));

	});
}

#[test]
fn parse_price_works() {
	let test_data = vec![
		("{\"USD\":6536.92}", Some(653692)),
		("{\"USD\":65.92}", Some(6592)),
		("{\"USD\":6536.924565}", Some(653692)),
		("{\"USD\":6536}", Some(653600)),
		("{\"USD2\":6536}", None),
		("{\"USD\":\"6432\"}", None),
		( "{\"msg\":\"success\",\"code\":0,\"data\":{\"market\":null,\"symbol\":\"btcusdt\",\"price\":23383.08,\"nodes\":null,\"sn\":null,\"systs\":1608654228412,\"ts\":1608654228412}}",
		  Some(2338308)),
	];

	for (json, expected) in test_data {
		assert_eq!(expected, TemplateModule::parse_price(json));
	}
}
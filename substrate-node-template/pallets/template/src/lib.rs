#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, debug};
use frame_system::{ensure_signed, ensure_root, Trait as SystemTrait};
use pallet_ares::{CallbackWithParameter, Event, Trait as AresTrait, BalanceOf};
use codec::{Decode, Encode};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: AresTrait {
    type Event: From<Event<Self>> + Into<<Self as SystemTrait>::Event>;
	type Callback: From<Call<Self>> + Into<<Self as AresTrait>::Callback>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// the result of the oracle call
		pub Result get(fn get_result): i128;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
// decl_event!(
// 	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
// 		/// Event documentation should end with an array that provides descriptive names for event
// 		/// parameters. [something, who]
// 		SomethingStored(u32, AccountId),
// 	}
// );

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn send_request(origin, operator: T::AccountId, specid: Vec<u8>) -> dispatch::DispatchResult {
			ensure_signed(origin.clone())?;

			let parameters = ("get", "https://min-api.cryptocompare.com/data/pricemultifull?fsyms=ETH&tsyms=USD", "path", "RAW.ETH.USD.PRICE", "times", "100000000");
			// Update storage.
			let call: <T as Trait>::Callback = Call::callback(vec![]).into();
            <pallet_ares::Module<T>>::initiate_request(origin, operator, specid, 0, parameters.encode(), BalanceOf::<T>::from(100), call.into())?;

			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn callback(origin, result: Vec<u8>) -> dispatch::DispatchResult {
			ensure_root(origin)?;

			let r : i128 = i128::decode(&mut &result[..]).map_err(|err| err.what())?;
            <Result>::put(r);

            Ok(())
		}
	}
}

impl <T: Trait> CallbackWithParameter for Call<T> {
    fn with_result(&self, result: Vec<u8>) -> Option<Self> {
        match *self {
            Call::callback(_) => {
                Some(Call::callback(result))
            },
            _ => None
        }
    }
}

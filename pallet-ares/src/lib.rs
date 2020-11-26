#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

#[warn(unused_imports)]
use codec::Codec;
use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, Parameter,  dispatch, debug };
use frame_support::traits::{Get, ReservableCurrency, Currency, BalanceStatus, UnfilteredDispatchable};
use sp_std::prelude::*;
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


// A trait allowing to inject Operator results back into the specified Call
pub trait CallbackWithParameter {
	fn with_result(&self, result: Vec<u8>) -> Option<Self> where Self: core::marker::Sized;
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Currency: ReservableCurrency<Self::AccountId>;

	type Callback: Parameter + UnfilteredDispatchable<Origin = Self::Origin> + Codec + Eq + CallbackWithParameter;

	// Period during which a request is valid
	type ValidityPeriod: Get<Self::BlockNumber>;
}

// REVIEW: Use this for transfering currency.
pub type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

// Uniquely identify a request's specification understood by an Operator
pub type SpecIndex = Vec<u8>;
// Uniquely identify a request for a considered Operator
//pub type RequestIdentifier = u64;
// The version of the serialized data format
pub type DataVersion = u64;

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as AresModule {
		// A set of all registered Operator
		// TODO migrate to 'natural' hasher once migrated to 2.0
		pub Operators get(fn operator): map hasher(blake2_128_concat) T::AccountId => bool;

		// A running counter used internally to identify the next request
		pub NextRequestIdentifier get(fn request_identifier): u64;

		// A map of details of each running request
		// TODO migrate to 'natural' hasher once migrated to 2.0
		// REVIEW: Consider using a struct for the Requests instead of a tuple to increase
		//         readability.
		pub Requests get(fn request): map hasher(blake2_128_concat) u64 => (T::AccountId, Vec<T::Callback>, T::BlockNumber, BalanceOf<T>);
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId, Balance = BalanceOf<T> {
		// A request has been accepted. Corresponding fee paiement is reserved
		OracleRequest(AccountId, SpecIndex, u64, AccountId, DataVersion, Vec<u8>, Vec<u8>, Balance),

		// A request has been answered. Corresponding fee paiement is transfered
		OracleAnswer(AccountId, u64, AccountId, Vec<u8>, Balance),

		// A new operator has been registered
		OperatorRegistered(AccountId),

		// An existing operator has been unregistered
		OperatorUnregistered(AccountId),

		// A request didn't receive any result in time
		KillRequest(u64),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
	    // Manipulating an unknown operator
		UnknownOperator,
		// Manipulating an unknown request
		UnknownRequest,
		// Not the expected operator
		WrongOperator,
		// An operator is already registered.
		OperatorAlreadyRegistered,
		// Callback cannot be deserialized
		UnknownCallback,
		// Fee provided does not match minimum required fee
		InsufficientFee,
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

		#[weight = 10_000]
		pub fn initiate_request(origin, operator: T::AccountId, spec_index: SpecIndex, data_version: DataVersion, data: Vec<u8>, fee: BalanceOf<T>, callback: <T as Trait>::Callback) -> dispatch::DispatchResult {
			//let _who = ensure_signed(origin)?;
			let who : <T as frame_system::Trait>::AccountId = ensure_signed(origin.clone())?;
			debug::info!("------------------------------initiate_request. 1"); 
			//ensure!(<Operators<T>>::get(&operator), Error::<T>::UnknownOperator);
			// REVIEW: Should probably be at least `ExistentialDeposit`
			//ensure!(fee > BalanceOf::<T>::from(0), Error::<T>::InsufficientFee);

			//T::Currency::reserve(&who, fee.into())?;

			let request_id = NextRequestIdentifier::get();
			// REVIEW: This can overflow. You can make a maximum of `u64::max_value()` requests.
			//         Default behavior for `u64` is to wrap around to 0, but you might want to
			//         make this explicit.
			//         I think using `wrapping_add` could be fine here, because it should be fine to
			//         start at 0 when you reach `u64::max_value()`.
			NextRequestIdentifier::put(request_id + 1);

			// REVIEW: Is it intentional that requests are only valid during the current block?
			let now = frame_system::Module::<T>::block_number();
			// REVIEW: You might want to think about and document that your requests can be overwritten
			//         as soon as the request id wraps around.
			// REVIEW: Is the `Vec` intended for forward compatibility? It seems superfluous here.
			Requests::<T>::insert(request_id.clone(), (operator.clone(), vec![callback], now, fee));

			Self::deposit_event(RawEvent::OracleRequest(operator, spec_index, request_id, who, data_version, data, "Ares.callback".into(), fee));
			debug::info!("------------------------------initiate_request. 2"); 

			Ok(())
		}

		#[weight = 10_000]
        fn callback(origin, request_id: u64, result: Vec<u8>) -> dispatch::DispatchResult {
			//let _who = ensure_signed(origin)?;
			let who : <T as frame_system::Trait>::AccountId = ensure_signed(origin.clone())?;

			ensure!(<Requests<T>>::contains_key(&request_id), Error::<T>::UnknownRequest);
			let (operator, callback, _, fee) = <Requests<T>>::get(&request_id);
			ensure!(operator == who, Error::<T>::WrongOperator);

			// REVIEW: This does not make sure that the fee is payed. `repatriate_reserved` removes
			//         *up to* the amount passed. [See here](https://substrate.dev/rustdocs/master/frame_support/traits/trait.ReservableCurrency.html#tymethod.repatriate_reserved)
			//         Check `reserved_balance()` to make sure that the fee is payable via this method.
			//         Maybe use a different payment method and check `total_balance()`. I don't know
			//         Substrate's Currency module well enough to tell.
			// REVIEW: This happens *after* the request is `take`n from storage. Is that intended?
			//         See ["verify first, write last"](https://substrate.dev/recipes/2-appetizers/1-hello-substrate.html#inside-a-dispatchable-call) motto.
			// TODO chec whether to use BalanceStatus::Reserved or Free?
			//T::Currency::repatriate_reserved(&who, &operator, fee.into(), BalanceStatus::Free)?;

			// Dispatch the result to the original callback registered by the caller
			// TODO fix the "?" - not sure how to proceed there
			callback[0].with_result(result.clone()).ok_or(Error::<T>::UnknownCallback)?.dispatch_bypass_filter(frame_system::RawOrigin::Root.into()).ok();
			// callback[0].with_result(result.clone()).ok_or(Error::<T>::UnknownCallback)?.dispatch(frame_system::RawOrigin::Root.into())?;

			Self::deposit_event(RawEvent::OracleAnswer(operator, request_id, who, result, fee));
			debug::info!("------------------------------callback. 1"); 

			Ok(())
		}

		// Identify requests that are considered dead and remove them
		fn on_finalize(n: T::BlockNumber) {
			for (request_identifier, (_account_id, _data, block_number, _fee)) in Requests::<T>::iter() {
				if n > block_number + T::ValidityPeriod::get() {
					// No result has been received in time
					Requests::<T>::remove(request_identifier);

					Self::deposit_event(RawEvent::KillRequest(request_identifier));
				}
			}
		}
	}
}

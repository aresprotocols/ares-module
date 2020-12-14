#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure,  dispatch, traits::Get};
use sp_std::prelude::*;
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	// Period during which a request is valid
	type ValidityPeriod: Get<Self::BlockNumber>;
}

// Uniquely identify a request's specification understood by an Aggregator
pub type SpecIndex = Vec<u8>;
// Uniquely identify a request for a considered Aggregator
//pub type RequestIdentifier = u64;
// The version of the serialized data format
pub type DataVersion = u64;

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	trait Store for Module<T: Trait> as AresModule {
		// A set of all registered Aggregator
		pub Aggregators get(fn aggregator): map hasher(blake2_128_concat) T::AccountId => (T::AccountId, T::BlockNumber, Vec<u8>);

		// A running counter used internally to identify the next request
		pub NextRequestId get(fn request_id): u64;

		// A map of details of each running request
		pub Requests get(fn request): map hasher(blake2_128_concat) u64 => (T::AccountId, T::BlockNumber, SpecIndex);

		pub OracleResults get(fn oracle_results): map hasher(blake2_128_concat) SpecIndex => u64;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		// A request has been accepted.
		OracleRequest(AccountId, SpecIndex, u64, AccountId, DataVersion, Vec<u8>, Vec<u8>),

		// A request has been answered.
		OracleResult(AccountId, u64, AccountId, u64),

		// A new aggregator has been registered
		AggregatorRegistered(AccountId),

		// An existing aggregator has been unregistered
		AggregatorUnregistered(AccountId),

		// A request didn't receive any result in time
		RemoveRequest(u64),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
	    // Manipulating an unknown aggregator
		UnknownAggregator,
		// Manipulating an unknown request
		UnknownRequest,
		// Not the expected aggregator
		WrongAggregator,
		// An aggregator is already registered.
		AggregatorAlreadyRegistered,
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

		// Register a new Aggregator.
		// Fails with `AggregatorAlreadyRegistered` if this Aggregator (identified by `origin`) has already been registered.
		#[weight = 10_000]
		pub fn register_aggregator(origin, source: Vec<u8>) -> dispatch::DispatchResult {
			let who : <T as frame_system::Trait>::AccountId = ensure_signed(origin)?;

			ensure!(!<Aggregators<T>>::contains_key(who.clone()), Error::<T>::AggregatorAlreadyRegistered);

			let now = frame_system::Module::<T>::block_number();

			Aggregators::<T>::insert(&who, (who.clone(), now, source));

			Self::deposit_event(RawEvent::AggregatorRegistered(who));

			Ok(())
		}

		// Unregisters an existing Aggregator
		#[weight = 10_000]
		pub fn unregister_aggregator(origin) -> dispatch::DispatchResult {
			let who : <T as frame_system::Trait>::AccountId = ensure_signed(origin)?;

			ensure!(<Aggregators<T>>::contains_key(who.clone()), Error::<T>::UnknownAggregator);

			let (aggregator, _, _) = <Aggregators<T>>::take(who.clone());

			if who == aggregator {
				Self::deposit_event(RawEvent::AggregatorUnregistered(who));
				Ok(())
			} else {
				Err(Error::<T>::UnknownAggregator.into())
			}
		}

		// Identify oracle request from outside
		// spec_index mark btc or eth price
		#[weight = 10_000]
		pub fn initiate_request(origin, aggregator: T::AccountId, spec_index: SpecIndex, data_version: DataVersion, data: Vec<u8>) -> dispatch::DispatchResult {
			let who : <T as frame_system::Trait>::AccountId = ensure_signed(origin.clone())?;

			ensure!(<Aggregators<T>>::contains_key(aggregator.clone()), Error::<T>::UnknownAggregator);

			let request_id = NextRequestId::get();
			NextRequestId::put(request_id + 1);

			let now = frame_system::Module::<T>::block_number();
			Requests::<T>::insert(request_id.clone(), (aggregator.clone(), now, spec_index.clone()));

			Self::deposit_event(RawEvent::OracleRequest(aggregator, spec_index, request_id, who, data_version, data, "Ares.callback".into()));

			Ok(())
		}

		// when aggregator get price from outside will callback token price
		#[weight = 10_000]
        fn feed_result(origin, request_id: u64, result: u64) -> dispatch::DispatchResult {
			//let _who = ensure_signed(origin)?;
			let who : <T as frame_system::Trait>::AccountId = ensure_signed(origin.clone())?;

			ensure!(<Requests<T>>::contains_key(&request_id), Error::<T>::UnknownRequest);
			let (aggregator, _, _) = <Requests<T>>::get(&request_id);
			ensure!(aggregator == who, Error::<T>::WrongAggregator);

			let (aggregator, _, spec_index) = <Requests<T>>::take(request_id.clone());

			OracleResults::insert(&spec_index,result.clone());

			Self::deposit_event(RawEvent::OracleResult(aggregator, request_id, who, result));

			Ok(())
		}

		// Identify requests that are considered dead and remove them
		fn on_finalize(n: T::BlockNumber) {
			for (request_identifier, (_account_id, block_number, _spec_index)) in Requests::<T>::iter() {
				if n > block_number + T::ValidityPeriod::get() {
					// No result has been received in time
					Requests::<T>::remove(request_identifier);

					Self::deposit_event(RawEvent::RemoveRequest(request_identifier));
				}
			}
		}
	}
}

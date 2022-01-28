#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;


use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency};
use frame_support::sp_runtime::SaturatedConversion;
use frame_system::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::vec::Vec;
use serde::{Serialize, Deserialize};
use frame_support::sp_runtime::traits::Hash;



#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {

	use super::*;
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, Hash)]
	#[scale_info(skip_type_params(T))]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Gender {
		Male,
		Female,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, Hash)]
	#[scale_info(skip_type_params(T))]
	pub struct Animal {
		pub class: Vec<u8>,
		pub gender: Gender,
		pub color: Vec<u8>,
	}


	#[pallet::config]
	pub trait Config: frame_system::Config{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber), ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a proof has been claimed. [who, claim]
		ClaimCreated(T::AccountId,T::Hash),
		/// Event emitted when a claim is revoked by the owner. [who, claim]
		ClaimRevoked(T::AccountId, T::Hash),
		ProofTransferred(T::AccountId, T::AccountId, T::Hash),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The proof has already been claimed.
		ProofAlreadyClaimed,
		/// The proof does not exist, so it cannot be revoked.
		NoSuchProof,
		/// The proof is claimed by another account, so caller can't revoke it.
		NotProofOwner,
		DestinationIsSame
		

	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(1_000)]
		pub fn create_claim(
			origin: OriginFor<T>,
			class: Vec<u8>,
			gender: Gender,
			color: Vec<u8>
		) -> DispatchResult {

			let owner = ensure_signed(origin)?;
			
			let animal = Animal{class, gender, color};
			let animal_hash = <T as frame_system::Config>::Hashing::hash_of(&animal);
			// Verify that the specified proof has not already been claimed.
			ensure!(!Proofs::<T>::contains_key(&animal_hash), Error::<T>::ProofAlreadyClaimed);
	
			// Get the block number from the FRAME System pallet.
			let current_block = <frame_system::Pallet<T>>::block_number();
	
			// Store the proof with the sender and block number.
			Proofs::<T>::insert(&animal_hash, (&owner, current_block));
	
			// Emit an event that the claim was created.
			Self::deposit_event(Event::ClaimCreated(owner, animal_hash));
	
			Ok(())
		}

		
		#[pallet::weight(10_000)]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			proof: T::Hash,
		) -> DispatchResult {

			let sender = ensure_signed(origin)?;
	
			// Verify that the specified proof has been claimed.
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
	
			// Get owner of the claim.
			let (owner, _) = Proofs::<T>::get(&proof);
	
			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotProofOwner);
	
			// Remove claim from storage.
			Proofs::<T>::remove(&proof);
	
			// Emit an event that the claim was erased.
			Self::deposit_event(Event::ClaimRevoked(sender, proof));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn transfer_owner(
			origin: OriginFor<T>,
			des: T::AccountId,
			proof: T::Hash,
		) -> DispatchResult {

            let sender = ensure_signed(origin)?;

			// Verify that the specified proof has been claimed.
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			// Get owner of the claim.
			let (owner, _) = Proofs::<T>::get(&proof);
            ensure!(owner == sender, Error::<T>::NotProofOwner);
            ensure!(owner != des, Error::<T>::DestinationIsSame);
            Proofs::<T>::remove(&proof);
            Proofs::<T>::insert(
                &proof,
                (des.clone(), <frame_system::Pallet::<T>>::block_number()),
            );
            Self::deposit_event(Event::ProofTransferred(sender, des, proof));
            Ok(())
		}



	}


	
}





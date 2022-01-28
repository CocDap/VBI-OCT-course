use super::*;
use crate::{mock::*, Error, Gender, Animal, Proofs};
use frame_support::{assert_noop, assert_ok};
use frame_support::sp_runtime::traits::Hash;
use sp_core::H256;

pub fn str2vec(s: &str) -> Vec<u8> {
	s.as_bytes().to_vec()
}

#[test]
fn create_claim_should_work() {
	new_test_ext().execute_with(|| {
		let class = str2vec("cat");
		let gender = Gender::Male;
		let color = str2vec("white");

		// Dispatch a signed extrinsic.
		assert_ok!(Poe::create_claim(Origin::signed(1), class,gender, color));
		// Read pallet storage and assert an expected result.

		
	});
}

#[test]
fn revoke_claim_should_work() {
	new_test_ext().execute_with(|| {
		let class = str2vec("cat");
		let gender = Gender::Male;
		let color = str2vec("white");
		// Dispatch a signed extrinsic.
		assert_ok!(Poe::create_claim(Origin::signed(1), class.clone(),gender.clone(), color.clone()));


		let animal = Animal { class: class.clone(), gender: gender.clone(), color: color.clone() };
		let animal_proof = <Test as frame_system::Config>::Hashing::hash_of(&animal);
		//let animal_proof = frame_system::Config::Hashing::hash_of(&animal);
		assert_ok!(Poe::revoke_claim(Origin::signed(1),animal_proof));


	});
}


#[test]
fn transfer_should_work() {
	new_test_ext().execute_with(|| {
		let class = str2vec("cat");
		let gender = Gender::Male;
		let color = str2vec("white");
		// Dispatch a signed extrinsic.
		assert_ok!(Poe::create_claim(Origin::signed(1), class.clone(),gender.clone(), color.clone()));


		let animal = Animal { class: class.clone(), gender: gender.clone(), color: color.clone() };
		let animal_proof = <Test as frame_system::Config>::Hashing::hash_of(&animal);
		//assert_ok!(Poe::revoke_claim(Origin::signed(1),animal_proof));

		assert_ok!(Poe::transfer_owner(Origin::signed(1),2,animal_proof));

	});
}



#[test]
fn create_claim_should_fail() {
	new_test_ext().execute_with(|| {
		let class = str2vec("cat");
		let gender = Gender::Male;
		let color = str2vec("white");

		// Dispatch a signed extrinsic.
		assert_ok!(Poe::create_claim(Origin::signed(1), class.clone(),gender.clone(), color.clone()));
		// Read pallet storage and assert an expected result.

		assert_noop!(Poe::create_claim(Origin::signed(2), class,gender, color), Error::<Test>::ProofAlreadyClaimed);

		
	});
}

#[test]
fn revoke_claim_should_fail() {
	new_test_ext().execute_with(|| {
		let class = str2vec("cat");
		let gender = Gender::Male;
		let color = str2vec("white");

		let animal = Animal { class: class.clone(), gender: gender.clone(), color: color.clone() };
		let animal_proof = <Test as frame_system::Config>::Hashing::hash_of(&animal);
		assert_noop!(Poe::revoke_claim(Origin::signed(1),animal_proof), Error::<Test>::NoSuchProof);

		assert_ok!(Poe::create_claim(Origin::signed(1), class.clone(),gender.clone(), color.clone()));

		assert_noop!(Poe::revoke_claim(Origin::signed(2),animal_proof), Error::<Test>::NotProofOwner);

		assert_ok!(Poe::revoke_claim(Origin::signed(1),animal_proof));

		assert_noop!(Poe::revoke_claim(Origin::signed(1),animal_proof), Error::<Test>::NoSuchProof);

	});
}


#[test]
fn transfer_should_fail() {
	new_test_ext().execute_with(|| {
		let class = str2vec("cat");
		let gender = Gender::Male;
		let color = str2vec("white");
		// Dispatch a signed extrinsic.
		assert_ok!(Poe::create_claim(Origin::signed(1), class.clone(),gender.clone(), color.clone()));


		let animal = Animal { class: class.clone(), gender: gender.clone(), color: color.clone() };
		let animal_proof = <Test as frame_system::Config>::Hashing::hash_of(&animal);
		//assert_ok!(Poe::revoke_claim(Origin::signed(1),animal_proof));

		assert_noop!(Poe::transfer_owner(Origin::signed(1),1,animal_proof),Error::<Test>::DestinationIsSame);
		assert_noop!(Poe::transfer_owner(Origin::signed(2),3,animal_proof),Error::<Test>::NotProofOwner);
	});
}



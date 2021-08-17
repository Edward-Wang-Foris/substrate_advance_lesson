use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

//create claim  successful test case
#[test]
fn create_claim_works() {
	new_test_ext().execute_with(||{
		let claim = vec![0; ClaimSize::get()];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1,frame_system::Pallet::<Test>::block_number())
		));
	});
}


//create claim which one existed failed test case
#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(||{
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

//revoke claim successful test case
#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), None);
	})

}

//revoke claim which one doesn't existed failed test case
#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(||{
		let claim = vec![0, 1];
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

//revoke claim which one's onwer changes faield test case 
#[test]
fn revoke_claim_failed_when_origin_is_not_owner() {
	new_test_ext().execute_with(||{
		let claim = vec![0, 1];
		let _  = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotProofOwner
		);
	})
}

//transfer claim successful test case 
#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(||{
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::NotProofOwner
		);
	})	
}

//transfer claim which one doesn't exist failed test case 
#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(||{
		let claim = vec![0, 1];
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	})
}

//transfer claim which one's owner changes failed test case
#[test]
fn transfer_claim_failed_when_origin_is_not_owner() {
	new_test_ext().execute_with(||{
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 2),
			Error::<Test>::NotProofOwner
		);
	})
}


//creat claim failed when size is too big
#[test]
fn create_claim_failed_when_size_too_big() {
	new_test_ext().execute_with(||{
		let claim = vec![0; ClaimSize::get() + 1];
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimSizeTooBig
		);
	});	
}
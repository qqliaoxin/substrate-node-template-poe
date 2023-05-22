use super::*; // 引入模块的所用存储项
use crate::{mock::*, Error}; // 引入 mock 的一些定义
use frame_support::{assert_noop, assert_ok, BoundedVec}; // 引入断言的一些方法

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap(); // key
													   // 传入两个参数，发送方 AccountId = 1， key
													   // 断言返回结果
		assert_ok!(PoeModule::create_claim(RuntimeOrign::signed(1), claim.clone()));

		// 断言储存值 是否相等
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap(); // key

		// 断言已存在出错
		assert_noop!(
			PoeModule::create_claim(RuntimeOrign::signed(1), claim.clone()),
			Error::<Test>::ProofAleadyExist
		);
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap(); // key
		let _ = PoeModule::create_claim(RuntimeOrign::signed(1), claim.clone());

		// 断言是否删除成功
		assert_ok!(PoeModule::revoke_claim(RuntimeOrign::signed(1), claim.clone()));
	});
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap(); // key

		// 断言不存在，出错
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrign::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	});
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap(); // key
		let _ = PoeModule::create_claim(RuntimeOrign::signed(1), claim.clone());

		// 断言用户不是owner，出错
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrign::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	});
}

#[test]
fn transfer_claim_works_chack() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap(); // key
		let _ = PoeModule::create_claim(RuntimeOrign::signed(1), claim.clone());

		// 断言转移是否成功
		assert_ok!(PoeModule::transfer_claim(RuntimeOrign::signed(1), 2, claim.clone()));

		let bouned_claim: BoundedVec<u8, <T as Config>::MaxClaimLength> =
			BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone());

		assert_noop!(&bouned_claim, Error::<Test>::ClaimTooLong);

		// 断言转换 长度是否出错
		assert_noop!(PoeModule::get(&bouned_claim), Error::<Test>::ClaimNotExist);

		// 断言储存值 是否相等
		assert_eq!(
			Proofs::<Test>::get(&bouned_claim),
			Some((2, frame_system::system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap(); // key
		let _ = PoeModule::create_claim(RuntimeOrign::signed(1), claim.clone());

		// 断言权限
		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrign::signed(2), 3, claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	});
}

#[test]
fn transfer_claim_failed_with_wrong_destination_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap(); // key
		let _ = PoeModule::create_claim(RuntimeOrign::signed(1), claim.clone());
		// 断言是否转给自己
		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrign::signed(1), 1, claim.clone()),
			Error::<Test>::DestinationIsClaimOwner
		);
	});
}

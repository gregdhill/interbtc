use crate::{
    ext,
    mock::{
        run_test, set_default_thresholds, Extrinsic, GetWrappedCurrencyId, Origin, SecurityError, System, Test,
        TestError, TestEvent, TokensError, VaultRegistry, DEFAULT_COLLATERAL, DEFAULT_ID, MULTI_VAULT_TEST_COLLATERAL,
        MULTI_VAULT_TEST_IDS, OTHER_ID, RICH_COLLATERAL, RICH_ID,
    },
    types::{BtcAddress, Collateral, Wrapped},
    BtcPublicKey, CurrencySource, DispatchError, Error, UpdatableVault, Vault, VaultStatus, Wallet, H256,
};
use codec::Decode;
use currency::ParachainCurrency;
use frame_support::{assert_err, assert_noop, assert_ok, traits::OnInitialize};
use mocktopus::mocking::*;
use orml_tokens::CurrencyAdapter;
use security::Pallet as Security;
use sp_arithmetic::{traits::One, FixedPointNumber, FixedU128};
use sp_core::U256;
use sp_runtime::{
    offchain::{testing::TestTransactionPoolExt, TransactionPoolExt},
    traits::Header,
};
use sp_std::convert::TryInto;

type Event = crate::Event<Test>;

// use macro to avoid messing up stack trace
macro_rules! assert_emitted {
    ($event:expr) => {
        let test_event = TestEvent::VaultRegistry($event);
        assert!(System::events().iter().any(|a| a.event == test_event));
    };
    ($event:expr, $times:expr) => {
        let test_event = TestEvent::VaultRegistry($event);
        assert_eq!(
            System::events().iter().filter(|a| a.event == test_event).count(),
            $times
        );
    };
}

macro_rules! assert_not_emitted {
    ($event:expr) => {
        let test_event = TestEvent::VaultRegistry($event);
        assert!(!System::events().iter().any(|a| a.event == test_event));
    };
}

fn dummy_public_key() -> BtcPublicKey {
    BtcPublicKey([
        2, 205, 114, 218, 156, 16, 235, 172, 106, 37, 18, 153, 202, 140, 176, 91, 207, 51, 187, 55, 18, 45, 222, 180,
        119, 54, 243, 97, 173, 150, 161, 169, 230,
    ])
}

fn create_vault_with_collateral(id: u64, collateral: u128) -> <Test as frame_system::Config>::AccountId {
    VaultRegistry::get_minimum_collateral_vault.mock_safe(move || MockResult::Return(collateral));
    let origin = Origin::signed(id);
    let result = VaultRegistry::register_vault(origin, collateral, dummy_public_key());
    assert_ok!(result);
    id
}

fn create_vault(id: u64) -> <Test as frame_system::Config>::AccountId {
    create_vault_with_collateral(id, DEFAULT_COLLATERAL)
}

fn create_sample_vault() -> <Test as frame_system::Config>::AccountId {
    create_vault(DEFAULT_ID)
}

fn create_vault_and_issue_tokens(
    issue_tokens: u128,
    collateral: u128,
    id: u64,
) -> <Test as frame_system::Config>::AccountId {
    // vault has no tokens issued yet
    let id = create_vault_with_collateral(id, collateral);

    // exchange rate 1 Satoshi = 10 Planck (smallest unit of DOT)
    ext::oracle::collateral_to_wrapped::<Test>.mock_safe(move |x| MockResult::Return(Ok(x / 10)));

    // issue tokens with 200% collateralization of DEFAULT_COLLATERAL
    assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(&id, issue_tokens,));
    let res = VaultRegistry::issue_tokens(&id, issue_tokens);
    assert_ok!(res);

    // mint tokens to the vault
    assert_ok!(<CurrencyAdapter<Test, GetWrappedCurrencyId>>::mint(&id, issue_tokens));

    id
}

fn create_sample_vault_and_issue_tokens(issue_tokens: u128) -> <Test as frame_system::Config>::AccountId {
    create_vault_and_issue_tokens(issue_tokens, DEFAULT_COLLATERAL, DEFAULT_ID)
}

#[test]
fn register_vault_succeeds() {
    run_test(|| {
        let id = create_sample_vault();
        assert_emitted!(Event::RegisterVault(id, DEFAULT_COLLATERAL));
    });
}

#[test]
fn register_vault_fails_when_given_collateral_too_low() {
    run_test(|| {
        VaultRegistry::get_minimum_collateral_vault.mock_safe(|| MockResult::Return(200));
        let id = 3;
        let collateral = 100;
        let result = VaultRegistry::register_vault(Origin::signed(id), collateral, dummy_public_key());
        assert_err!(result, TestError::InsufficientVaultCollateralAmount);
        assert_not_emitted!(Event::RegisterVault(id, collateral));
    });
}

#[test]
fn register_vault_fails_when_account_funds_too_low() {
    run_test(|| {
        let collateral = DEFAULT_COLLATERAL + 1;
        let result = VaultRegistry::register_vault(Origin::signed(DEFAULT_ID), collateral, dummy_public_key());
        assert_err!(result, TokensError::BalanceTooLow);
        assert_not_emitted!(Event::RegisterVault(DEFAULT_ID, collateral));
    });
}

#[test]
fn register_vault_fails_when_already_registered() {
    run_test(|| {
        let id = create_sample_vault();
        let result = VaultRegistry::register_vault(Origin::signed(id), DEFAULT_COLLATERAL, dummy_public_key());
        assert_err!(result, TestError::VaultAlreadyRegistered);
        assert_emitted!(Event::RegisterVault(id, DEFAULT_COLLATERAL), 1);
    });
}

#[test]
fn deposit_collateral_succeeds() {
    run_test(|| {
        let id = create_vault(RICH_ID);
        let additional = RICH_COLLATERAL - DEFAULT_COLLATERAL;
        let res = VaultRegistry::deposit_collateral(Origin::signed(id), additional);
        assert_ok!(res);
        let new_collateral = ext::collateral::get_reserved_balance::<Test>(&id);
        assert_eq!(new_collateral, DEFAULT_COLLATERAL + additional);
        assert_emitted!(Event::DepositCollateral(
            id,
            additional,
            RICH_COLLATERAL,
            RICH_COLLATERAL
        ));
    });
}

#[test]
fn deposit_collateral_fails_when_vault_does_not_exist() {
    run_test(|| {
        let res = VaultRegistry::deposit_collateral(Origin::signed(3), 50);
        assert_err!(res, TestError::VaultNotFound);
    })
}

#[test]
fn withdraw_collateral_succeeds() {
    run_test(|| {
        let id = create_sample_vault();
        let res = VaultRegistry::withdraw_collateral(Origin::signed(id), 50);
        assert_ok!(res);
        let new_collateral = ext::collateral::get_reserved_balance::<Test>(&id);
        assert_eq!(new_collateral, DEFAULT_COLLATERAL - 50);
        assert_emitted!(Event::WithdrawCollateral(id, 50, DEFAULT_COLLATERAL - 50));
    });
}

#[test]
fn withdraw_collateral_fails_when_vault_does_not_exist() {
    run_test(|| {
        let res = VaultRegistry::withdraw_collateral(Origin::signed(3), 50);
        assert_err!(res, TestError::VaultNotFound);
    })
}

#[test]
fn withdraw_collateral_fails_when_not_enough_collateral() {
    run_test(|| {
        let id = create_sample_vault();
        let res = VaultRegistry::withdraw_collateral(Origin::signed(id), DEFAULT_COLLATERAL + 1);
        assert_err!(res, TestError::InsufficientCollateral);
    })
}

#[test]
fn try_increase_to_be_issued_tokens_succeeds() {
    run_test(|| {
        let id = create_sample_vault();
        let res = VaultRegistry::try_increase_to_be_issued_tokens(&id, 50);
        let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
        assert_ok!(res);
        assert_eq!(vault.data.to_be_issued_tokens, 50);
        assert_emitted!(Event::IncreaseToBeIssuedTokens(id, 50));
    });
}

#[test]
fn try_increase_to_be_issued_tokens_fails_with_insufficient_collateral() {
    run_test(|| {
        let id = create_sample_vault();
        let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
        let res = VaultRegistry::try_increase_to_be_issued_tokens(&id, vault.issuable_tokens().unwrap() + 1);
        // important: should not change the storage state
        assert_noop!(res, TestError::ExceedingVaultLimit);
    });
}

#[test]
fn decrease_to_be_issued_tokens_succeeds() {
    run_test(|| {
        let id = create_sample_vault();
        assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(&id, 50),);
        let res = VaultRegistry::decrease_to_be_issued_tokens(&id, 50);
        assert_ok!(res);
        let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
        assert_eq!(vault.data.to_be_issued_tokens, 0);
        assert_emitted!(Event::DecreaseToBeIssuedTokens(id, 50));
    });
}

#[test]
fn decrease_to_be_issued_tokens_fails_with_insufficient_tokens() {
    run_test(|| {
        let id = create_sample_vault();

        let res = VaultRegistry::decrease_to_be_issued_tokens(&id, 50);
        assert_err!(res, TestError::ArithmeticUnderflow);
    });
}

#[test]
fn issue_tokens_succeeds() {
    run_test(|| {
        let id = create_sample_vault();
        assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(&id, 50),);
        let res = VaultRegistry::issue_tokens(&id, 50);
        assert_ok!(res);
        let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
        assert_eq!(vault.data.to_be_issued_tokens, 0);
        assert_eq!(vault.data.issued_tokens, 50);
        assert_emitted!(Event::IssueTokens(id, 50));
    });
}

#[test]
fn issue_tokens_fails_with_insufficient_tokens() {
    run_test(|| {
        let id = create_sample_vault();

        assert_err!(VaultRegistry::issue_tokens(&id, 50), TestError::ArithmeticUnderflow);
    });
}

#[test]
fn try_increase_to_be_replaced_tokens_succeeds() {
    run_test(|| {
        let id = create_sample_vault();

        assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(&id, 50),);
        assert_ok!(VaultRegistry::issue_tokens(&id, 50));
        let res = VaultRegistry::try_increase_to_be_replaced_tokens(&id, 50, 50);
        assert_ok!(res);
        let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
        assert_eq!(vault.data.issued_tokens, 50);
        assert_eq!(vault.data.to_be_replaced_tokens, 50);
        assert_emitted!(Event::IncreaseToBeReplacedTokens(id, 50));
    });
}

#[test]
fn try_increase_to_be_replaced_tokens_fails_with_insufficient_tokens() {
    run_test(|| {
        let id = create_sample_vault();

        let res = VaultRegistry::try_increase_to_be_replaced_tokens(&id, 50, 50);

        // important: should not change the storage state
        assert_noop!(res, TestError::InsufficientTokensCommitted);
    });
}

#[test]
fn try_increase_to_be_redeemed_tokens_succeeds() {
    run_test(|| {
        let id = create_sample_vault();

        assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(&id, 50),);
        assert_ok!(VaultRegistry::issue_tokens(&id, 50));
        let res = VaultRegistry::try_increase_to_be_redeemed_tokens(&id, 50);
        assert_ok!(res);
        let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
        assert_eq!(vault.data.issued_tokens, 50);
        assert_eq!(vault.data.to_be_redeemed_tokens, 50);
        assert_emitted!(Event::IncreaseToBeRedeemedTokens(id, 50));
    });
}

#[test]
fn try_increase_to_be_redeemed_tokens_fails_with_insufficient_tokens() {
    run_test(|| {
        let id = create_sample_vault();

        let res = VaultRegistry::try_increase_to_be_redeemed_tokens(&id, 50);

        // important: should not change the storage state
        assert_noop!(res, TestError::InsufficientTokensCommitted);
    });
}

#[test]
fn decrease_to_be_redeemed_tokens_succeeds() {
    run_test(|| {
        let id = create_sample_vault();
        assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(&id, 50),);
        assert_ok!(VaultRegistry::issue_tokens(&id, 50));
        assert_ok!(VaultRegistry::try_increase_to_be_redeemed_tokens(&id, 50));
        let res = VaultRegistry::decrease_to_be_redeemed_tokens(&id, 50);
        assert_ok!(res);
        let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
        assert_eq!(vault.data.issued_tokens, 50);
        assert_eq!(vault.data.to_be_redeemed_tokens, 0);
        assert_emitted!(Event::DecreaseToBeRedeemedTokens(id, 50));
    });
}

#[test]
fn decrease_to_be_redeemed_tokens_fails_with_insufficient_tokens() {
    run_test(|| {
        let id = create_sample_vault();

        let res = VaultRegistry::decrease_to_be_redeemed_tokens(&id, 50);
        assert_err!(res, TestError::ArithmeticUnderflow);
    });
}

#[test]
fn decrease_tokens_succeeds() {
    run_test(|| {
        let id = create_sample_vault();
        let user_id = 5;
        VaultRegistry::try_increase_to_be_issued_tokens(&id, 50).unwrap();
        assert_ok!(VaultRegistry::issue_tokens(&id, 50));
        assert_ok!(VaultRegistry::try_increase_to_be_redeemed_tokens(&id, 50));
        let res = VaultRegistry::decrease_tokens(&id, &user_id, 50);
        assert_ok!(res);
        let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
        assert_eq!(vault.data.issued_tokens, 0);
        assert_eq!(vault.data.to_be_redeemed_tokens, 0);
        assert_emitted!(Event::DecreaseTokens(id, user_id, 50));
    });
}

#[test]
fn decrease_tokens_fails_with_insufficient_tokens() {
    run_test(|| {
        let id = create_sample_vault();
        let user_id = 5;
        VaultRegistry::try_increase_to_be_issued_tokens(&id, 50).unwrap();
        assert_ok!(VaultRegistry::issue_tokens(&id, 50));
        let res = VaultRegistry::decrease_tokens(&id, &user_id, 50);
        assert_err!(res, TestError::ArithmeticUnderflow);
    });
}

#[test]
fn redeem_tokens_succeeds() {
    run_test(|| {
        let id = create_sample_vault();
        VaultRegistry::try_increase_to_be_issued_tokens(&id, 50).unwrap();
        assert_ok!(VaultRegistry::issue_tokens(&id, 50));
        assert_ok!(VaultRegistry::try_increase_to_be_redeemed_tokens(&id, 50));
        let res = VaultRegistry::redeem_tokens(&id, 50, 0, &0);
        assert_ok!(res);
        let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
        assert_eq!(vault.data.issued_tokens, 0);
        assert_eq!(vault.data.to_be_redeemed_tokens, 0);
        assert_emitted!(Event::RedeemTokens(id, 50));
    });
}

#[test]
fn redeem_tokens_fails_with_insufficient_tokens() {
    run_test(|| {
        let id = create_sample_vault();
        VaultRegistry::try_increase_to_be_issued_tokens(&id, 50).unwrap();
        assert_ok!(VaultRegistry::issue_tokens(&id, 50));
        let res = VaultRegistry::redeem_tokens(&id, 50, 0, &0);
        assert_err!(res, TestError::ArithmeticUnderflow);
    });
}

#[test]
fn redeem_tokens_premium_succeeds() {
    run_test(|| {
        let id = create_sample_vault();
        let user_id = 5;
        // TODO: emulate assert_called
        VaultRegistry::transfer_funds.mock_safe(move |sender, receiver, _amount| {
            assert_eq!(sender, CurrencySource::Collateral(id));
            assert_eq!(receiver, CurrencySource::FreeBalance(user_id));
            MockResult::Return(Ok(()))
        });

        VaultRegistry::try_increase_to_be_issued_tokens(&id, 50).unwrap();
        assert_ok!(VaultRegistry::issue_tokens(&id, 50));
        assert_ok!(VaultRegistry::try_increase_to_be_redeemed_tokens(&id, 50));
        assert_ok!(VaultRegistry::redeem_tokens(&id, 50, 30, &user_id));

        let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
        assert_eq!(vault.data.issued_tokens, 0);
        assert_eq!(vault.data.to_be_redeemed_tokens, 0);
        assert_emitted!(Event::RedeemTokensPremium(id, 50, 30, user_id));
    });
}

#[test]
fn redeem_tokens_premium_fails_with_insufficient_tokens() {
    run_test(|| {
        let id = create_sample_vault();
        let user_id = 5;
        VaultRegistry::try_increase_to_be_issued_tokens(&id, 50).unwrap();
        assert_ok!(VaultRegistry::issue_tokens(&id, 50));
        let res = VaultRegistry::redeem_tokens(&id, 50, 30, &user_id);
        assert_err!(res, TestError::ArithmeticUnderflow);
        assert_not_emitted!(Event::RedeemTokensPremium(id, 50, 30, user_id));
    });
}

#[test]
fn redeem_tokens_liquidation_succeeds() {
    run_test(|| {
        let mut liquidation_vault = VaultRegistry::get_rich_liquidation_vault();
        let user_id = 5;

        // TODO: emulate assert_called
        VaultRegistry::transfer_funds.mock_safe(move |sender, receiver, _amount| {
            assert_eq!(sender, CurrencySource::LiquidationVault);
            assert_eq!(receiver, CurrencySource::FreeBalance(user_id));
            MockResult::Return(Ok(()))
        });

        // liquidation vault collateral
        ext::collateral::get_reserved_balance::<Test>.mock_safe(|_| MockResult::Return(1000u32.into()));

        assert_ok!(liquidation_vault.increase_to_be_issued(50));
        assert_ok!(liquidation_vault.increase_issued(50));

        assert_ok!(VaultRegistry::redeem_tokens_liquidation(&user_id, 50));
        let liquidation_vault = VaultRegistry::get_rich_liquidation_vault();
        assert_eq!(liquidation_vault.data.issued_tokens, 0);
        assert_emitted!(Event::RedeemTokensLiquidation(user_id, 50, 500));
    });
}

#[test]
fn redeem_tokens_liquidation_does_not_call_recover_when_unnecessary() {
    run_test(|| {
        let mut liquidation_vault = VaultRegistry::get_rich_liquidation_vault();
        let user_id = 5;

        VaultRegistry::transfer_funds.mock_safe(move |sender, receiver, _amount| {
            assert_eq!(sender, CurrencySource::LiquidationVault);
            assert_eq!(receiver, CurrencySource::FreeBalance(user_id));
            MockResult::Return(Ok(()))
        });

        // liquidation vault collateral
        ext::collateral::get_reserved_balance::<Test>.mock_safe(|_| MockResult::Return(1000u32.into()));

        assert_ok!(liquidation_vault.increase_to_be_issued(25));
        assert_ok!(liquidation_vault.increase_issued(25));

        assert_ok!(VaultRegistry::redeem_tokens_liquidation(&user_id, 10));
        let liquidation_vault = VaultRegistry::get_rich_liquidation_vault();
        assert_eq!(liquidation_vault.data.issued_tokens, 15);
        assert_emitted!(Event::RedeemTokensLiquidation(user_id, 10, (1000 * 10) / 50));
    });
}

#[test]
fn redeem_tokens_liquidation_fails_with_insufficient_tokens() {
    run_test(|| {
        let user_id = 5;
        let res = VaultRegistry::redeem_tokens_liquidation(&user_id, 50);
        assert_err!(res, TestError::InsufficientTokensCommitted);
        assert_not_emitted!(Event::RedeemTokensLiquidation(user_id, 50, 50));
    });
}

#[test]
fn replace_tokens_liquidation_succeeds() {
    run_test(|| {
        let old_id = create_sample_vault();
        let new_id = create_vault(OTHER_ID);

        ext::collateral::lock::<Test>.mock_safe(move |sender, amount| {
            assert_eq!(sender, &new_id);
            assert_eq!(amount, 20);
            MockResult::Return(Ok(()))
        });

        VaultRegistry::try_increase_to_be_issued_tokens(&old_id, 50).unwrap();
        assert_ok!(VaultRegistry::issue_tokens(&old_id, 50));
        assert_ok!(VaultRegistry::try_increase_to_be_redeemed_tokens(&old_id, 50));
        assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(&new_id, 50));

        assert_ok!(VaultRegistry::replace_tokens(&old_id, &new_id, 50, 20));

        let old_vault = VaultRegistry::get_active_rich_vault_from_id(&old_id).unwrap();
        let new_vault = VaultRegistry::get_active_rich_vault_from_id(&new_id).unwrap();
        assert_eq!(old_vault.data.issued_tokens, 0);
        assert_eq!(old_vault.data.to_be_redeemed_tokens, 0);
        assert_eq!(new_vault.data.issued_tokens, 50);
        assert_eq!(new_vault.data.to_be_issued_tokens, 0);
        assert_emitted!(Event::ReplaceTokens(old_id, new_id, 50, 20));
    });
}

#[test]
fn cancel_replace_tokens_succeeds() {
    run_test(|| {
        let old_id = create_sample_vault();
        let new_id = create_vault(OTHER_ID);

        ext::collateral::lock::<Test>.mock_safe(move |sender, amount| {
            assert_eq!(sender, &new_id);
            assert_eq!(amount, 20);
            MockResult::Return(Ok(()))
        });

        VaultRegistry::try_increase_to_be_issued_tokens(&old_id, 50).unwrap();
        assert_ok!(VaultRegistry::issue_tokens(&old_id, 50));
        assert_ok!(VaultRegistry::try_increase_to_be_redeemed_tokens(&old_id, 50));
        assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(&new_id, 50));

        assert_ok!(VaultRegistry::cancel_replace_tokens(&old_id, &new_id, 50));

        let old_vault = VaultRegistry::get_active_rich_vault_from_id(&old_id).unwrap();
        let new_vault = VaultRegistry::get_active_rich_vault_from_id(&new_id).unwrap();
        assert_eq!(old_vault.data.issued_tokens, 50);
        assert_eq!(old_vault.data.to_be_redeemed_tokens, 0);
        assert_eq!(new_vault.data.issued_tokens, 0);
        assert_eq!(new_vault.data.to_be_issued_tokens, 0);
    });
}

#[test]
fn liquidate_at_most_secure_threshold() {
    run_test(|| {
        let vault_id = DEFAULT_ID;

        let issued_tokens = 100;
        let to_be_issued_tokens = 25;
        let to_be_redeemed_tokens = 40;

        let backing_collateral = 10000;
        let liquidated_collateral = 1875; // 125 * 10 * 1.5
        let liquidated_for_issued = 1275; // 125 * 10 * 1.5

        assert_ok!(VaultRegistry::register_vault(
            Origin::signed(vault_id),
            backing_collateral,
            dummy_public_key()
        ));
        let liquidation_vault_before = VaultRegistry::get_rich_liquidation_vault();

        VaultRegistry::set_secure_collateral_threshold(
            FixedU128::checked_from_rational(150, 100).unwrap(), // 150%
        );

        let collateral_before = ext::collateral::get_reserved_balance::<Test>(&vault_id);
        assert_eq!(collateral_before, backing_collateral); // sanity check

        // required for `issue_tokens` to work
        assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(
            &vault_id,
            issued_tokens
        ));
        assert_ok!(VaultRegistry::issue_tokens(&vault_id, issued_tokens));
        assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(
            &vault_id,
            to_be_issued_tokens
        ));
        assert_ok!(VaultRegistry::try_increase_to_be_redeemed_tokens(
            &vault_id,
            to_be_redeemed_tokens
        ));

        let vault_orig = <crate::Vaults<Test>>::get(&vault_id);

        // set exchange rate
        ext::oracle::wrapped_to_collateral::<Test>.mock_safe(|v| MockResult::Return(Ok(v * 10)));
        assert_ok!(VaultRegistry::liquidate_vault(&vault_id));

        // should only be able to withdraw excess above secure threshold
        assert_err!(
            VaultRegistry::withdraw_collateral(Origin::signed(vault_id), backing_collateral),
            TestError::InsufficientCollateral
        );
        assert_ok!(VaultRegistry::withdraw_collateral(
            Origin::signed(vault_id),
            backing_collateral - liquidated_collateral
        ));

        let liquidation_vault_after = VaultRegistry::get_rich_liquidation_vault();
        let liquidated_vault = <crate::Vaults<Test>>::get(&vault_id);
        assert!(matches!(liquidated_vault.status, VaultStatus::Liquidated));
        assert_emitted!(Event::LiquidateVault(
            vault_id,
            vault_orig.issued_tokens,
            vault_orig.to_be_issued_tokens,
            vault_orig.to_be_redeemed_tokens,
            vault_orig.to_be_replaced_tokens,
            vault_orig.backing_collateral,
            VaultStatus::Liquidated,
            vault_orig.replace_collateral,
        ));

        // check liquidation_vault tokens & collateral
        assert_eq!(
            liquidation_vault_after.data.issued_tokens,
            liquidation_vault_before.data.issued_tokens + issued_tokens
        );
        assert_eq!(
            liquidation_vault_after.data.to_be_issued_tokens,
            liquidation_vault_before.data.to_be_issued_tokens + to_be_issued_tokens
        );
        assert_eq!(
            liquidation_vault_after.data.to_be_redeemed_tokens,
            liquidation_vault_before.data.to_be_redeemed_tokens + to_be_redeemed_tokens
        );
        assert_eq!(
            ext::collateral::get_reserved_balance::<Test>(&liquidation_vault_before.id()),
            liquidated_for_issued
        );

        // check vault tokens & collateral
        let user_vault_after = VaultRegistry::get_rich_vault_from_id(&vault_id).unwrap();
        assert_eq!(user_vault_after.data.issued_tokens, 0);
        assert_eq!(user_vault_after.data.to_be_issued_tokens, 0);
        assert_eq!(user_vault_after.data.to_be_redeemed_tokens, to_be_redeemed_tokens);
        assert_eq!(
            ext::collateral::get_reserved_balance::<Test>(&vault_id),
            liquidated_collateral - liquidated_for_issued
        );
        assert_eq!(
            ext::collateral::get_free_balance::<Test>(&vault_id),
            DEFAULT_COLLATERAL - liquidated_collateral
        );
    });
}

#[test]
fn is_collateral_below_threshold_true_succeeds() {
    run_test(|| {
        let collateral = DEFAULT_COLLATERAL;
        let btc_amount = DEFAULT_COLLATERAL / 2;
        let threshold = FixedU128::checked_from_rational(201, 100).unwrap(); // 201%

        ext::oracle::collateral_to_wrapped::<Test>.mock_safe(move |_| MockResult::Return(Ok(collateral)));

        assert_eq!(
            VaultRegistry::is_collateral_below_threshold(collateral, btc_amount, threshold),
            Ok(true)
        );
    })
}

#[test]
fn is_collateral_below_threshold_false_succeeds() {
    run_test(|| {
        let collateral = DEFAULT_COLLATERAL;
        let btc_amount = 50;
        let threshold = FixedU128::checked_from_rational(200, 100).unwrap(); // 200%

        ext::oracle::collateral_to_wrapped::<Test>.mock_safe(move |_| MockResult::Return(Ok(collateral)));

        assert_eq!(
            VaultRegistry::is_collateral_below_threshold(collateral, btc_amount, threshold),
            Ok(false)
        );
    })
}

#[test]
fn calculate_max_wrapped_from_collateral_for_threshold_succeeds() {
    run_test(|| {
        let collateral: u128 = u64::MAX as u128;
        let threshold = FixedU128::checked_from_rational(200, 100).unwrap(); // 200%

        ext::oracle::collateral_to_wrapped::<Test>.mock_safe(move |_| MockResult::Return(Ok(collateral)));

        assert_eq!(
            VaultRegistry::calculate_max_wrapped_from_collateral_for_threshold(collateral, threshold),
            Ok((u64::MAX / 2) as u128)
        );
    })
}

#[test]
fn test_threshold_equivalent_to_legacy_calculation() {
    /// old version
    fn legacy_calculate_max_wrapped_from_collateral_for_threshold(
        collateral: Collateral<Test>,
        threshold: u128,
    ) -> Result<Wrapped<Test>, DispatchError> {
        let granularity = 5;
        // convert the collateral to wrapped
        let collateral_in_wrapped = ext::oracle::collateral_to_wrapped::<Test>(collateral)?;
        let collateral_in_wrapped = U256::from(collateral_in_wrapped);

        // calculate how many tokens should be maximally issued given the threshold
        let scaled_collateral_in_wrapped = collateral_in_wrapped
            .checked_mul(U256::from(10).pow(granularity.into()))
            .ok_or(Error::<Test>::ArithmeticOverflow)?;
        let scaled_max_tokens = scaled_collateral_in_wrapped
            .checked_div(threshold.into())
            .unwrap_or(0.into());

        Ok(scaled_max_tokens.try_into()?)
    }

    run_test(|| {
        let threshold = FixedU128::checked_from_rational(199999, 100000).unwrap(); // 199.999%
        let random_start = 987529462328 as u128;
        for btc in random_start..random_start + 199999 {
            ext::oracle::collateral_to_wrapped::<Test>.mock_safe(move |x| MockResult::Return(Ok(x)));
            ext::oracle::wrapped_to_collateral::<Test>.mock_safe(move |x| MockResult::Return(Ok(x)));
            let old = legacy_calculate_max_wrapped_from_collateral_for_threshold(btc, 199999).unwrap();
            let new = VaultRegistry::calculate_max_wrapped_from_collateral_for_threshold(btc, threshold).unwrap();
            assert_eq!(old, new);
        }
    })
}

#[test]
fn test_get_required_collateral_threshold_equivalent_to_legacy_calculation_() {
    // old version
    fn legacy_get_required_collateral_for_wrapped_with_threshold(
        btc: Wrapped<Test>,
        threshold: u128,
    ) -> Result<Collateral<Test>, DispatchError> {
        let granularity = 5;
        let btc = U256::from(btc);

        // Step 1: inverse of the scaling applied in calculate_max_wrapped_from_collateral_for_threshold

        // inverse of the div
        let btc = btc
            .checked_mul(threshold.into())
            .ok_or(Error::<Test>::ArithmeticOverflow)?;

        // To do the inverse of the multiplication, we need to do division, but
        // we need to round up. To round up (a/b), we need to do ((a+b-1)/b):
        let rounding_addition = U256::from(10).pow(granularity.into()) - U256::from(1);
        let btc = (btc + rounding_addition)
            .checked_div(U256::from(10).pow(granularity.into()))
            .ok_or(Error::<Test>::ArithmeticUnderflow)?;

        // Step 2: convert the amount to collateral
        let amount_in_collateral = ext::oracle::wrapped_to_collateral::<Test>(btc.try_into()?)?;
        Ok(amount_in_collateral)
    }

    run_test(|| {
        let threshold = FixedU128::checked_from_rational(199999, 100000).unwrap(); // 199.999%
        let random_start = 987529462328 as u128;
        for btc in random_start..random_start + 199999 {
            ext::oracle::collateral_to_wrapped::<Test>.mock_safe(move |x| MockResult::Return(Ok(x)));
            ext::oracle::wrapped_to_collateral::<Test>.mock_safe(move |x| MockResult::Return(Ok(x)));
            let old = legacy_get_required_collateral_for_wrapped_with_threshold(btc, 199999);
            let new = VaultRegistry::get_required_collateral_for_wrapped_with_threshold(btc, threshold);
            assert_eq!(old, new);
        }
    })
}

#[test]
fn get_required_collateral_for_wrapped_with_threshold_succeeds() {
    run_test(|| {
        let threshold = FixedU128::checked_from_rational(19999, 10000).unwrap(); // 199.99%
        let random_start = 987529387592 as u128;
        for btc in random_start..random_start + 19999 {
            ext::oracle::collateral_to_wrapped::<Test>.mock_safe(move |x| MockResult::Return(Ok(x)));
            ext::oracle::wrapped_to_collateral::<Test>.mock_safe(move |x| MockResult::Return(Ok(x)));

            let min_collateral =
                VaultRegistry::get_required_collateral_for_wrapped_with_threshold(btc, threshold).unwrap();

            let max_btc_for_min_collateral =
                VaultRegistry::calculate_max_wrapped_from_collateral_for_threshold(min_collateral, threshold).unwrap();

            let max_btc_for_below_min_collateral =
                VaultRegistry::calculate_max_wrapped_from_collateral_for_threshold(min_collateral - 1, threshold)
                    .unwrap();

            // Check that the amount we found is indeed the lowest amount that is sufficient for `btc`
            assert!(max_btc_for_min_collateral >= btc);
            assert!(max_btc_for_below_min_collateral < btc);
        }
    })
}

// Security integration tests
#[test]
fn register_vault_parachain_not_running_fails() {
    run_test(|| {
        ext::security::ensure_parachain_status_not_shutdown::<Test>
            .mock_safe(|| MockResult::Return(Err(SecurityError::ParachainNotRunning.into())));

        assert_noop!(
            VaultRegistry::register_vault(Origin::signed(DEFAULT_ID), DEFAULT_COLLATERAL, dummy_public_key()),
            SecurityError::ParachainNotRunning
        );
    });
}

#[test]
fn deposit_collateral_parachain_not_running_fails() {
    run_test(|| {
        let id = create_vault(RICH_ID);
        let additional = RICH_COLLATERAL - DEFAULT_COLLATERAL;
        ext::security::ensure_parachain_status_not_shutdown::<Test>
            .mock_safe(|| MockResult::Return(Err(SecurityError::ParachainShutdown.into())));

        assert_noop!(
            VaultRegistry::deposit_collateral(Origin::signed(id), additional),
            SecurityError::ParachainShutdown
        );
    })
}

mod liquidation_threshold_tests {
    use super::*;

    fn setup() -> Vault<u64, u64, u128, u128, <Test as crate::Config>::SignedFixedPoint> {
        let id = create_sample_vault();

        assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(&id, 50),);
        let res = VaultRegistry::issue_tokens(&id, 50);
        assert_ok!(res);

        ext::oracle::wrapped_to_collateral::<Test>.mock_safe(move |x| MockResult::Return(Ok(x)));

        let mut vault = VaultRegistry::get_vault_from_id(&id).unwrap();
        vault.issued_tokens = 50;
        vault.to_be_issued_tokens = 40;
        vault.to_be_redeemed_tokens = 20;

        vault
    }

    #[test]
    fn is_vault_below_liquidation_threshold_false_succeeds() {
        run_test(|| {
            let mut vault = setup();
            vault.backing_collateral = vault.issued_tokens * 2;
            assert_eq!(
                VaultRegistry::is_vault_below_liquidation_threshold(&vault, FixedU128::from(2)),
                Ok(false)
            );
        })
    }

    #[test]
    fn is_vault_below_liquidation_threshold_true_succeeds() {
        run_test(|| {
            let mut vault = setup();
            vault.backing_collateral = vault.issued_tokens * 2 - 1;
            assert_eq!(
                VaultRegistry::is_vault_below_liquidation_threshold(&vault, FixedU128::from(2)),
                Ok(true)
            );
        })
    }
}

#[test]
fn get_collateralization_from_vault_fails_with_no_tokens_issued() {
    run_test(|| {
        // vault has no tokens issued yet
        let id = create_sample_vault();

        assert_err!(
            VaultRegistry::get_collateralization_from_vault(id, false),
            TestError::NoTokensIssued
        );
    })
}

#[test]
fn get_collateralization_from_vault_succeeds() {
    run_test(|| {
        let issue_tokens: u128 = DEFAULT_COLLATERAL / 10 / 2; // = 5
        let id = create_sample_vault_and_issue_tokens(issue_tokens);

        assert_eq!(
            VaultRegistry::get_collateralization_from_vault(id, false),
            Ok(FixedU128::checked_from_rational(200, 100).unwrap())
        );
    })
}

#[test]
fn get_unsettled_collateralization_from_vault_succeeds() {
    run_test(|| {
        let issue_tokens: u128 = DEFAULT_COLLATERAL / 10 / 5; // = 2
        let id = create_sample_vault_and_issue_tokens(issue_tokens);

        assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(&id, issue_tokens),);

        assert_eq!(
            VaultRegistry::get_collateralization_from_vault(id, true),
            Ok(FixedU128::checked_from_rational(500, 100).unwrap())
        );
    })
}

#[test]
fn get_settled_collateralization_from_vault_succeeds() {
    run_test(|| {
        // wrapped_to_collateral is / 10 and we issue 2 * amount
        let issue_tokens: u128 = 100000 / 10 / 5; // 2000
        let id = create_sample_vault_and_issue_tokens(issue_tokens);

        assert_ok!(VaultRegistry::try_increase_to_be_issued_tokens(&id, issue_tokens));

        assert_eq!(
            VaultRegistry::get_collateralization_from_vault(id, false),
            Ok(FixedU128::checked_from_rational(250, 100).unwrap())
        );
    })
}

mod get_vaults_below_premium_collaterlization_tests {
    use super::*;

    /// sets premium_redeem threshold to 1
    pub fn run_test(test: impl FnOnce()) {
        super::run_test(|| {
            VaultRegistry::set_secure_collateral_threshold(FixedU128::from_float(0.001));
            VaultRegistry::set_premium_redeem_threshold(FixedU128::one());

            test()
        })
    }

    fn add_vault(id: u64, issued_tokens: u128, collateral: u128) {
        create_vault_with_collateral(id, collateral);

        VaultRegistry::try_increase_to_be_issued_tokens(&id, issued_tokens).unwrap();
        assert_ok!(VaultRegistry::issue_tokens(&id, issued_tokens));

        // sanity check
        let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
        assert_eq!(vault.data.issued_tokens, issued_tokens);
        assert_eq!(vault.data.to_be_redeemed_tokens, 0);
    }

    #[test]
    fn get_vaults_below_premium_collateralization_fails() {
        run_test(|| {
            add_vault(4, 50, 100);

            assert_err!(
                VaultRegistry::get_premium_redeem_vaults(),
                TestError::NoVaultUnderThePremiumRedeemThreshold
            );
        })
    }

    #[test]
    fn get_vaults_below_premium_collateralization_succeeds() {
        run_test(|| {
            let id1 = 3;
            let issue_tokens1: u128 = 50;
            let collateral1 = 49;

            let id2 = 4;
            let issue_tokens2: u128 = 50;
            let collateral2 = 48;

            add_vault(id1, issue_tokens1, collateral1);
            add_vault(id2, issue_tokens2, collateral2);

            assert_eq!(
                VaultRegistry::get_premium_redeem_vaults(),
                Ok(vec!((id1, issue_tokens1), (id2, issue_tokens2)))
            );
        })
    }

    #[test]
    fn get_vaults_below_premium_collateralization_filters_banned_and_sufficiently_collateralized_vaults() {
        run_test(|| {
            // not returned, because is is not under premium threshold (which is set to 100% for this test)
            let id1 = 3;
            let issue_tokens1: u128 = 50;
            let collateral1 = 50;
            add_vault(id1, issue_tokens1, collateral1);

            // returned
            let id2 = 4;
            let issue_tokens2: u128 = 50;
            let collateral2 = 49;
            add_vault(id2, issue_tokens2, collateral2);

            // not returned because it's banned
            let id3 = 5;
            let issue_tokens3: u128 = 50;
            let collateral3 = 49;
            add_vault(id3, issue_tokens3, collateral3);
            let mut vault3 = VaultRegistry::get_active_rich_vault_from_id(&id3).unwrap();
            vault3.ban_until(1000);

            assert_eq!(
                VaultRegistry::get_premium_redeem_vaults(),
                Ok(vec!((id2, issue_tokens2)))
            );
        })
    }
}

mod get_vaults_with_issuable_tokens_tests {
    use super::*;

    #[test]
    fn get_vaults_with_issuable_tokens_succeeds() {
        run_test(|| {
            let id1 = 3;
            let collateral1 = 100;
            create_vault_with_collateral(id1, collateral1);
            let issuable_tokens1 =
                VaultRegistry::get_issuable_tokens_from_vault(id1).expect("Sample vault is unable to issue tokens");

            let id2 = 4;
            let collateral2 = 50;
            create_vault_with_collateral(id2, collateral2);
            let issuable_tokens2 =
                VaultRegistry::get_issuable_tokens_from_vault(id2).expect("Sample vault is unable to issue tokens");

            // Check result is ordered in descending order
            assert_eq!(issuable_tokens1.gt(&issuable_tokens2), true);
            assert_eq!(
                VaultRegistry::get_vaults_with_issuable_tokens(),
                Ok(vec!((id1, issuable_tokens1), (id2, issuable_tokens2)))
            );
        })
    }
    #[test]
    fn get_vaults_with_issuable_tokens_succeeds_when_there_are_liquidated_vaults() {
        run_test(|| {
            let id1 = 3;
            let collateral1 = 100;
            create_vault_with_collateral(id1, collateral1);
            let issuable_tokens1 =
                VaultRegistry::get_issuable_tokens_from_vault(id1).expect("Sample vault is unable to issue tokens");

            let id2 = 4;
            let collateral2 = 50;
            create_vault_with_collateral(id2, collateral2);

            // liquidate vault
            assert_ok!(VaultRegistry::liquidate_vault(&id2));

            assert_eq!(
                VaultRegistry::get_vaults_with_issuable_tokens(),
                Ok(vec!((id1, issuable_tokens1)))
            );
        })
    }

    #[test]
    fn get_vaults_with_issuable_tokens_filters_out_banned_vaults() {
        run_test(|| {
            let id1 = 3;
            let collateral1 = 100;
            create_vault_with_collateral(id1, collateral1);
            let issuable_tokens1 =
                VaultRegistry::get_issuable_tokens_from_vault(id1).expect("Sample vault is unable to issue tokens");

            let id2 = 4;
            let collateral2 = 50;
            create_vault_with_collateral(id2, collateral2);

            // ban the vault
            let mut vault = VaultRegistry::get_rich_vault_from_id(&id2).unwrap();
            vault.ban_until(1000);

            let issuable_tokens2 =
                VaultRegistry::get_issuable_tokens_from_vault(id2).expect("Sample vault is unable to issue tokens");

            assert_eq!(issuable_tokens2, 0);

            // Check that the banned vault is not returned by get_vaults_with_issuable_tokens
            assert_eq!(
                VaultRegistry::get_vaults_with_issuable_tokens(),
                Ok(vec!((id1, issuable_tokens1)))
            );
        })
    }

    #[test]
    fn get_vaults_with_issuable_tokens_filters_out_vault_that_do_not_accept_new_issues() {
        run_test(|| {
            let id1 = 3;
            let collateral1 = 100;
            create_vault_with_collateral(id1, collateral1);
            let issuable_tokens1 =
                VaultRegistry::get_issuable_tokens_from_vault(id1).expect("Sample vault is unable to issue tokens");

            let id2 = 4;
            let collateral2 = 50;
            create_vault_with_collateral(id2, collateral2);
            assert_ok!(VaultRegistry::accept_new_issues(Origin::signed(id2), false));

            // Check that the vault that does not accept issues is not returned by get_vaults_with_issuable_tokens
            assert_eq!(
                VaultRegistry::get_vaults_with_issuable_tokens(),
                Ok(vec!((id1, issuable_tokens1)))
            );
        })
    }

    #[test]
    fn get_vaults_with_issuable_tokens_returns_empty() {
        run_test(|| {
            let issue_tokens: u128 = DEFAULT_COLLATERAL / 2;
            let id = create_sample_vault();

            VaultRegistry::try_increase_to_be_issued_tokens(&id, issue_tokens).unwrap();
            // issue DEFAULT_COLLATERAL / 2 tokens at 200% rate
            assert_ok!(VaultRegistry::issue_tokens(&id, issue_tokens));
            let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
            assert_eq!(vault.data.issued_tokens, issue_tokens);
            assert_eq!(vault.data.to_be_redeemed_tokens, 0);

            // update the exchange rate
            ext::oracle::collateral_to_wrapped::<Test>.mock_safe(move |x| MockResult::Return(Ok(x / 2)));

            assert_eq!(VaultRegistry::get_vaults_with_issuable_tokens(), Ok(vec!()));
        })
    }
}

mod get_vaults_with_redeemable_tokens_test {
    use super::*;

    fn create_vault_with_issue(id: u64, to_issue: u128) {
        create_vault(id);
        VaultRegistry::try_increase_to_be_issued_tokens(&id, to_issue).unwrap();
        assert_ok!(VaultRegistry::issue_tokens(&id, to_issue));
        let vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
        assert_eq!(vault.data.issued_tokens, to_issue);
        assert_eq!(vault.data.to_be_redeemed_tokens, 0);
    }

    #[test]
    fn get_vaults_with_redeemable_tokens_returns_empty() {
        run_test(|| {
            // create a vault with no redeemable tokens
            create_sample_vault();
            // nothing issued, so nothing can be redeemed
            assert_eq!(VaultRegistry::get_vaults_with_redeemable_tokens(), Ok(vec!()));
        })
    }

    #[test]
    fn get_vaults_with_redeemable_tokens_succeeds() {
        run_test(|| {
            let id1 = 3;
            let issued_tokens1: u128 = 10;
            create_vault_with_issue(id1, issued_tokens1);

            let id2 = 4;
            let issued_tokens2: u128 = 20;
            create_vault_with_issue(id2, issued_tokens2);

            // Check result is ordered in descending order
            assert_eq!(issued_tokens2.gt(&issued_tokens1), true);
            assert_eq!(
                VaultRegistry::get_vaults_with_redeemable_tokens(),
                Ok(vec!((id2, issued_tokens2), (id1, issued_tokens1)))
            );
        })
    }

    #[test]
    fn get_vaults_with_redeemable_tokens_filters_out_banned_vaults() {
        run_test(|| {
            let id1 = 3;
            let issued_tokens1: u128 = 10;
            create_vault_with_issue(id1, issued_tokens1);

            let id2 = 4;
            let issued_tokens2: u128 = 20;
            create_vault_with_issue(id2, issued_tokens2);

            // ban the vault
            let mut vault = VaultRegistry::get_rich_vault_from_id(&id2).unwrap();
            vault.ban_until(1000);

            // Check that the banned vault is not returned by get_vaults_with_redeemable_tokens
            assert_eq!(
                VaultRegistry::get_vaults_with_redeemable_tokens(),
                Ok(vec!((id1, issued_tokens1)))
            );
        })
    }

    #[test]
    fn get_vaults_with_issuable_tokens_filters_out_liquidated_vaults() {
        run_test(|| {
            let id1 = 3;
            let issued_tokens1: u128 = 10;
            create_vault_with_issue(id1, issued_tokens1);

            let id2 = 4;
            let issued_tokens2: u128 = 20;
            create_vault_with_issue(id2, issued_tokens2);

            // liquidate vault
            assert_ok!(VaultRegistry::liquidate_vault(&id2));

            assert_eq!(
                VaultRegistry::get_vaults_with_redeemable_tokens(),
                Ok(vec!((id1, issued_tokens1)))
            );
        })
    }
}

#[test]
fn get_total_collateralization_with_tokens_issued() {
    run_test(|| {
        let issue_tokens: u128 = DEFAULT_COLLATERAL / 10 / 2; // = 5
        let _id = create_sample_vault_and_issue_tokens(issue_tokens);

        assert_eq!(
            VaultRegistry::get_total_collateralization(),
            Ok(FixedU128::checked_from_rational(200, 100).unwrap())
        );
    })
}

// #[test]
// fn wallet_add_btc_address_succeeds() {
//     run_test(|| {
//         let address1 = BtcAddress::random();
//         let address2 = BtcAddress::random();
//         let address3 = BtcAddress::random();

//         let mut wallet = Wallet::new(address1);
//         assert_eq!(wallet.get_btc_address(), address1);

//         wallet.add_btc_address(address2);
//         assert_eq!(wallet.get_btc_address(), address2);

//         wallet.add_btc_address(address3);
//         assert_eq!(wallet.get_btc_address(), address3);
//     });
// }

#[test]
fn wallet_has_btc_address_succeeds() {
    use sp_std::collections::btree_set::BTreeSet;

    run_test(|| {
        let address1 = BtcAddress::random();
        let address2 = BtcAddress::random();

        let mut addresses = BTreeSet::new();
        addresses.insert(address1);

        let wallet = Wallet {
            addresses,
            public_key: dummy_public_key(),
        };
        assert_eq!(wallet.has_btc_address(&address1), true);
        assert_eq!(wallet.has_btc_address(&address2), false);
    });
}

// #[test]
// fn update_btc_address_fails_with_btc_address_taken() {
//     run_test(|| {
//         let origin = DEFAULT_ID;
//         let address = BtcAddress::random();

//         let mut vault = Vault::default();
//         vault.id = origin;
//         vault.wallet = Wallet::new(address);
//         VaultRegistry::insert_vault(&origin, vault);

//         assert_err!(
//             VaultRegistry::update_btc_address(Origin::signed(origin), address),
//             TestError::BtcAddressTaken
//         );
//     });
// }

// #[test]
// fn update_btc_address_succeeds() {
//     run_test(|| {
//         let origin = DEFAULT_ID;
//         let address1 = BtcAddress::random();
//         let address2 = BtcAddress::random();

//         let mut vault = Vault::default();
//         vault.id = origin;
//         vault.wallet = Wallet::new(address1);
//         VaultRegistry::insert_vault(&origin, vault);

//         assert_ok!(VaultRegistry::update_btc_address(
//             Origin::signed(origin),
//             address2
//         ));
//     });
// }

fn setup_block(i: u64, parent_hash: H256) -> H256 {
    System::initialize(&i, &parent_hash, &Default::default(), frame_system::InitKind::Full);
    <pallet_randomness_collective_flip::Pallet<Test>>::on_initialize(i);

    let header = System::finalize();
    Security::<Test>::set_active_block_number(*header.number());
    header.hash()
}

fn setup_blocks(blocks: u64) {
    let mut parent_hash = System::parent_hash();
    for i in 1..(blocks + 1) {
        parent_hash = setup_block(i, parent_hash);
    }
}

mod get_first_vault_with_sufficient_tokens_tests {
    use super::*;

    #[test]
    fn get_first_vault_with_sufficient_tokens_succeeds() {
        run_test(|| {
            let issue_tokens: u128 = DEFAULT_COLLATERAL / 10 / 2; // = 5
            let id = create_sample_vault_and_issue_tokens(issue_tokens);

            assert_eq!(
                VaultRegistry::get_first_vault_with_sufficient_tokens(issue_tokens),
                Ok(id)
            );
        })
    }

    #[test]
    fn get_first_vault_with_sufficient_tokens_considers_to_be_redeemed() {
        run_test(|| {
            let issue_tokens: u128 = DEFAULT_COLLATERAL / 10 / 2; // = 5
            let id = create_sample_vault_and_issue_tokens(issue_tokens);
            let mut vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();

            assert_ok!(vault.increase_to_be_redeemed(2));

            assert_noop!(
                VaultRegistry::get_first_vault_with_sufficient_tokens(issue_tokens),
                TestError::NoVaultWithSufficientTokens
            );

            assert_eq!(VaultRegistry::get_first_vault_with_sufficient_tokens(3), Ok(id));
        })
    }

    #[test]
    fn get_first_vault_with_sufficient_tokens_filters_banned_vaults() {
        run_test(|| {
            let issue_tokens: u128 = DEFAULT_COLLATERAL / 10 / 2; // = 5
            let id = create_sample_vault_and_issue_tokens(issue_tokens);
            let mut vault = VaultRegistry::get_active_rich_vault_from_id(&id).unwrap();
            vault.ban_until(1000);

            assert_noop!(
                VaultRegistry::get_first_vault_with_sufficient_tokens(issue_tokens),
                TestError::NoVaultWithSufficientTokens
            );
        })
    }

    #[test]
    fn get_first_vault_with_sufficient_tokens_returns_different_vaults_for_different_amounts() {
        run_test(|| {
            setup_blocks(100);

            let vault_ids = MULTI_VAULT_TEST_IDS
                .iter()
                .map(|&i| {
                    create_vault_and_issue_tokens(MULTI_VAULT_TEST_COLLATERAL / 100, MULTI_VAULT_TEST_COLLATERAL, i)
                })
                .collect::<Vec<_>>();
            let selected_ids = (1..50)
                .map(|i| VaultRegistry::get_first_vault_with_sufficient_tokens(i).unwrap())
                .collect::<Vec<_>>();

            // check that all vaults have been selected at least once
            assert!(vault_ids.iter().all(|&x| selected_ids.iter().any(|&y| x == y)));
        });
    }

    #[test]
    fn get_first_vault_with_sufficient_tokens_returns_different_vaults_for_different_blocks() {
        run_test(|| {
            setup_blocks(100);

            let vault_ids = MULTI_VAULT_TEST_IDS
                .iter()
                .map(|&i| {
                    create_vault_and_issue_tokens(MULTI_VAULT_TEST_COLLATERAL / 100, MULTI_VAULT_TEST_COLLATERAL, i)
                })
                .collect::<Vec<_>>();
            let selected_ids = (101..150)
                .map(|i| {
                    setup_block(i, System::parent_hash());
                    VaultRegistry::get_first_vault_with_sufficient_tokens(5).unwrap()
                })
                .collect::<Vec<_>>();

            // check that all vaults have been selected at least once
            assert!(vault_ids.iter().all(|&x| selected_ids.iter().any(|&y| x == y)));
        });
    }
}

mod get_first_vault_with_sufficient_collateral_test {
    use super::*;

    #[test]
    fn get_first_vault_with_sufficient_collateral_succeeds() {
        run_test(|| {
            let issue_tokens: u128 = 4;
            let id = create_sample_vault_and_issue_tokens(issue_tokens);

            assert_eq!(
                VaultRegistry::get_first_vault_with_sufficient_collateral(issue_tokens),
                Ok(id)
            );
        })
    }

    #[test]
    fn get_first_vault_with_sufficient_collateral_with_no_suitable_vaults_fails() {
        run_test(|| {
            create_sample_vault_and_issue_tokens(4);

            assert_err!(
                VaultRegistry::get_first_vault_with_sufficient_collateral(DEFAULT_COLLATERAL),
                TestError::NoVaultWithSufficientCollateral
            );
        })
    }

    #[test]
    fn get_first_vault_with_sufficient_collateral_filters_vaults_that_do_not_accept_new_issues() {
        run_test(|| {
            let issue_tokens: u128 = 4;
            let id = create_sample_vault_and_issue_tokens(issue_tokens);
            assert_ok!(VaultRegistry::accept_new_issues(Origin::signed(id), false));

            assert_err!(
                VaultRegistry::get_first_vault_with_sufficient_collateral(issue_tokens),
                TestError::NoVaultWithSufficientCollateral
            );
        })
    }

    #[test]
    fn get_first_vault_with_sufficient_collateral_returns_different_vaults_for_different_amounts() {
        run_test(|| {
            setup_blocks(100);

            let vault_ids = MULTI_VAULT_TEST_IDS
                .iter()
                .map(|&i| {
                    create_vault_and_issue_tokens(MULTI_VAULT_TEST_COLLATERAL / 100, MULTI_VAULT_TEST_COLLATERAL, i)
                })
                .collect::<Vec<_>>();
            let selected_ids = (1..50)
                .map(|i| VaultRegistry::get_first_vault_with_sufficient_collateral(i).unwrap())
                .collect::<Vec<_>>();

            // check that all vaults have been selected at least once
            assert!(vault_ids.iter().all(|&x| selected_ids.iter().any(|&y| x == y)));
        });
    }

    #[test]
    fn get_first_vault_with_sufficient_collateral_returns_different_vaults_for_different_blocks() {
        run_test(|| {
            setup_blocks(100);

            let vault_ids = MULTI_VAULT_TEST_IDS
                .iter()
                .map(|&i| {
                    create_vault_and_issue_tokens(MULTI_VAULT_TEST_COLLATERAL / 100, MULTI_VAULT_TEST_COLLATERAL, i)
                })
                .collect::<Vec<_>>();
            let selected_ids = (101..150)
                .map(|i| {
                    setup_block(i, System::parent_hash());
                    VaultRegistry::get_first_vault_with_sufficient_collateral(5).unwrap()
                })
                .collect::<Vec<_>>();

            // check that all vaults have been selected at least once
            assert!(vault_ids.iter().all(|&x| selected_ids.iter().any(|&y| x == y)));
        });
    }
}

#[test]
fn test_try_increase_to_be_replaced_tokens() {
    run_test(|| {
        let issue_tokens: u128 = 4;
        let vault_id = create_sample_vault_and_issue_tokens(issue_tokens);
        assert_ok!(VaultRegistry::try_increase_to_be_redeemed_tokens(&vault_id, 1));

        let (total_wrapped, total_collateral) =
            VaultRegistry::try_increase_to_be_replaced_tokens(&vault_id, 2, 10).unwrap();
        assert!(total_wrapped == 2);
        assert!(total_collateral == 10);

        // check that we can't request more than we have issued tokens
        assert_noop!(
            VaultRegistry::try_increase_to_be_replaced_tokens(&vault_id, 3, 10),
            TestError::InsufficientTokensCommitted
        );

        // check that we can't request replacement for tokens that are marked as to-be-redeemed
        assert_noop!(
            VaultRegistry::try_increase_to_be_replaced_tokens(&vault_id, 2, 10),
            TestError::InsufficientTokensCommitted
        );

        let (total_wrapped, total_collateral) =
            VaultRegistry::try_increase_to_be_replaced_tokens(&vault_id, 1, 20).unwrap();
        assert!(total_wrapped == 3);
        assert!(total_collateral == 30);

        // check that is was written to storage
        let vault = VaultRegistry::get_active_vault_from_id(&vault_id).unwrap();
        assert_eq!(vault.to_be_replaced_tokens, 3);
        assert_eq!(vault.replace_collateral, 30);
    })
}

#[test]
fn test_decrease_to_be_replaced_tokens_over_capacity() {
    run_test(|| {
        let issue_tokens: u128 = 4;
        let vault_id = create_sample_vault_and_issue_tokens(issue_tokens);

        assert_ok!(VaultRegistry::try_increase_to_be_replaced_tokens(&vault_id, 4, 10));

        let (tokens, collateral) = VaultRegistry::decrease_to_be_replaced_tokens(&vault_id, 5).unwrap();
        assert_eq!(tokens, 4);
        assert_eq!(collateral, 10);
    })
}

#[test]
fn test_decrease_to_be_replaced_tokens_below_capacity() {
    run_test(|| {
        let issue_tokens: u128 = 4;
        let vault_id = create_sample_vault_and_issue_tokens(issue_tokens);

        assert_ok!(VaultRegistry::try_increase_to_be_replaced_tokens(&vault_id, 4, 10));

        let (tokens, collateral) = VaultRegistry::decrease_to_be_replaced_tokens(&vault_id, 3).unwrap();
        assert_eq!(tokens, 3);
        assert_eq!(collateral, 7);
    })
}

#[test]
fn test_offchain_worker_unsigned_transaction_submission() {
    let mut externalities = crate::mock::ExtBuilder::build();
    let (pool, pool_state) = TestTransactionPoolExt::new();
    externalities.register_extension(TransactionPoolExt::new(pool));

    externalities.execute_with(|| {
        // setup state:
        let id = 7;
        System::set_block_number(1);
        Security::<Test>::set_active_block_number(1);
        set_default_thresholds();
        VaultRegistry::insert_vault(
            &id,
            Vault {
                id,
                ..Default::default()
            },
        );

        // mock that all vaults need to be liquidated
        VaultRegistry::is_vault_below_liquidation_threshold.mock_safe(move |_, _| MockResult::Return(Ok(true)));

        // call the actual function we want to test
        VaultRegistry::_offchain_worker();

        // check that a transaction has been added to liquidate the vault
        let tx = pool_state.write().transactions.pop().unwrap();
        assert!(pool_state.read().transactions.is_empty());
        let tx = Extrinsic::decode(&mut &*tx).unwrap();
        assert_eq!(tx.signature, None); // unsigned
        assert_eq!(
            tx.call,
            crate::mock::Call::VaultRegistry(crate::Call::report_undercollateralized_vault(id))
        );
    })
}

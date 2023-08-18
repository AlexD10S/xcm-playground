#[cfg(test)]
mod tests {
	use crate::setup::*;
	use frame_support::assert_ok;
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	/// Scenario:
	/// ALICE from parachain A locks 5 cents of relay chain native assets of its Sovereign account on the relay chain 
	/// and assigns Parachain B as unlocker.
	/// Parachain A then asks Parachain B to unlock the funds partly.
	///  Parachain B responds by sending an UnlockAssets instruction to the relay chain.
	///
	#[ignore] // TODO: Fixed updating the version to last polkadot1.0.0 (bug fixed)
    #[test]
	fn remote_locking() {
		MockNet::reset();
        sp_tracing::init_for_tests();

		let locked_amount = 100;

		ParaC::execute_with(|| {
			let message = Xcm(vec![LockAsset {
				asset: (Here, locked_amount).into(),
				unlocker: Parachain(1).into(),
			}]);
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
		});

		Relay::execute_with(|| {
			use pallet_balances::{BalanceLock, Reasons};
			assert_eq!(
				relay_chain::Balances::locks(&parachain_sovereign_account_id(3)),
				vec![BalanceLock {
					id: *b"py/xcmlk",
					amount: locked_amount,
					reasons: Reasons::All
				}]
			);
		});

		ParaA::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![NoteUnlockable {
					owner: (Parent, Parachain(3)).into(),
					asset: (Parent, locked_amount).into()
				}])]
			);
		});

		ParaC::execute_with(|| {
			// Request unlocking part of the funds on the relay chain
			let message = Xcm(vec![RequestUnlock {
				asset: (Parent, locked_amount - 50).into(),
				locker: Parent.into(),
			}]);
			assert_ok!(ParachainPalletXcm::send_xcm(Here, (Parent, Parachain(1)), message));
		});

		Relay::execute_with(|| {
			use pallet_balances::{BalanceLock, Reasons};
			// Lock is reduced
			assert_eq!(
				relay_chain::Balances::locks(&parachain_sovereign_account_id(3)),
				vec![BalanceLock {
					id: *b"py/xcmlk",
					amount: locked_amount - 50,
					reasons: Reasons::All
				}]
			);
		});
	}
}

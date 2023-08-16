#[cfg(test)]
mod tests {
	use crate::setup::*;
	use codec::Encode;
	use frame_support::{assert_ok, pallet_prelude::Weight};
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

    fn create_nft_on_relay_chain(mint_to: sp_runtime::AccountId32) {
        Relay::execute_with(|| {
            // Create a Collection.
            assert_ok!(relay_chain::Uniques::create(
                relay_chain::RuntimeOrigin::signed(ALICE),
                1,
				ALICE
			));
            // Mint an NFT.
			assert_ok!(relay_chain::Uniques::mint(
				relay_chain::RuntimeOrigin::signed(ALICE),
				1,
				77,
				mint_to,
			));
		});
    }
    

	/// Scenario:
	/// The relay-chain teleports an NFT to a parachain.
	#[test]
	fn teleport_nft() {
		MockNet::reset();

        // Create an NFT on the Relay-chain.
        let owner_nft = parachain_sovereign_account_id(1);
        create_nft_on_relay_chain(owner_nft.clone());

        // Simple test, to see if the NFT has been minted
        Relay::execute_with(|| {
			assert_eq!(
				relay_chain::Uniques::owner(1, 77),
				Some(owner_nft)
			);
		});

        // Create the collection in the parachain
        ParaA::execute_with(|| {
			assert_ok!(parachain::ForeignUniques::force_create(
				parachain::RuntimeOrigin::root(),
				1,
				ALICE,
				false,
			));
			assert_eq!(
				parachain::ForeignUniques::owner(1, 77),
				None,
			);
        });

        // XCM transfer
        ParaA::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((GeneralIndex(1),  77).into()),
				InitiateTeleport {
					assets: AllCounted(1).into(),
					dest: Parachain(1).into(),
					xcm: Xcm(vec![DepositAsset {
						assets: AllCounted(1).into(),
						beneficiary: (AccountId32 { id: ALICE.into(), network: None },).into(),
					}]),
				},
			]);
            // Send teleport
			let alice = AccountId32 { id: ALICE.into(), network: None };
			assert_ok!(ParachainPalletXcm::send_xcm(alice, Parent, message));
        });

        // Check if the NFT has been teleported
        ParaA::execute_with(|| {
			// assert_eq!(
			// 	parachain::ForeignUniques::owner(1, 77),
			// 	Some(ALICE),
			// );
			// assert_eq!(parachain::Balances::reserved_balance(&ALICE), 1000);
		});
		Relay::execute_with(|| {
			assert_eq!(relay_chain::Uniques::owner(1, 69), None);
		});



    }
}


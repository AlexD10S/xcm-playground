#[cfg(test)]
mod tests {
	use crate::setup::*;
	use frame_support::assert_ok;
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;
	use frame_support::sp_tracing;

    fn create_nft_on_relay_chain(mint_to: sp_runtime::AccountId32) {
        Relay::execute_with(|| {
            // Create a Collection.
            assert_ok!(relay_chain::Uniques::force_create(
                relay_chain::RuntimeOrigin::root(),
                2,
				ALICE,
				false
			));
            // Mint an NFT.
			assert_ok!(relay_chain::Uniques::mint(
				relay_chain::RuntimeOrigin::signed(ALICE),
				2,
				77,
				mint_to,
			));
		});
    }
    

	/// Scenario:
	/// The relay-chain transfers an NFT into a parachain's sovereign account, who then mints a
	/// trustless-backed-derivated locally.
	#[test]
	fn reserve_asset_transfer_nft() {
		//sp_tracing::init_for_tests();
		MockNet::reset();
         // Create an NFT on the Relay-chain.
         let owner_nft = parachain_account_sovereign_account_id(2, ALICE);
         create_nft_on_relay_chain(owner_nft.clone());
		 // Simple test, to see if the NFT has been minted
		 Relay::execute_with(|| {
			assert_eq!(
				relay_chain::Uniques::owner(2, 77),
				Some(owner_nft)
			);
		});

         // Create the collection in the parachain
        ParaB::execute_with(|| {
			assert_ok!(foreign_parachain::ForeignUniques::force_create(
				foreign_parachain::RuntimeOrigin::root(),
				(Parent, GeneralIndex(2)).into(),
				ALICE,
				false,
			));
			assert_eq!(
				foreign_parachain::ForeignUniques::owner((Parent, GeneralIndex(2)).into(), 77u32.into()),
				None,
			);
        });


         // XCM
         ParaB::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((GeneralIndex(2), 77u32).into()),
				DepositReserveAsset {
					assets: AllCounted(1).into(),
					dest: Parachain(2).into(),
					xcm: Xcm(vec![
						DepositAsset {
							assets: AllCounted(1).into(),
							beneficiary: (AccountId32 { id: ALICE.into(), network: None },).into(),
						}
					]),
				},
			]);
			// Send transfer
			let alice = AccountId32 { id: ALICE.into(), network: None };
			assert_ok!(ForeignParachainPalletXcm::send_xcm(alice, Parent, message));
        });

        ParaB::execute_with(|| {
			assert_eq!(
				foreign_parachain::ForeignUniques::collection_owner((Parent, GeneralIndex(2)).into()),
				Some(ALICE),
			);
            assert_eq!(
				foreign_parachain::ForeignUniques::owner((Parent, GeneralIndex(2)).into(), 77u32.into()),
				Some(ALICE),
			);
		});
		

        Relay::execute_with(|| {
			assert_eq!(relay_chain::Uniques::owner(2, 77), Some(parachain_sovereign_account_id(2)));
		});


    }
}


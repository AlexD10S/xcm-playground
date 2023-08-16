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
				77u32,
				mint_to,
			));
		});
    }
    

	/// Scenario:
	/// ALICE transfers an NFT from parachain A to parachain B.
	#[test]
	fn reserve_asset_transfer_nft_para_to_para() {
		MockNet::reset();
         // Create an NFT on the Relay-chain.
         let owner_nft = parachain_sovereign_account_id(2);
         create_nft_on_relay_chain(owner_nft.clone());

         // Create the collection in the parachain
        ParaB::execute_with(|| {
			assert_ok!(foreign_parachain::ForeignUniques::force_create(
				foreign_parachain::RuntimeOrigin::root(),
				(Parent, GeneralIndex(1)).into(),
				ALICE,
				false,
			));
			assert_eq!(
				foreign_parachain::ForeignUniques::owner((Parent, GeneralIndex(1)).into(), 77u32.into()),
				None,
			);
        });

         // XCM
         ParaB::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((GeneralIndex(1), 77u32).into()),
				DepositReserveAsset {
					assets: All.into(),
					dest: Parachain(2).into(),
					xcm: Xcm(vec![DepositAsset {
						assets: All.into(),
						beneficiary: (AccountId32 { id: ALICE.into(), network: None },).into(),
					}]),
				},
			]);
			// Send transfer
			let alice = AccountId32 { id: ALICE.into(), network: None };
			assert_ok!(ParachainPalletXcm::send_xcm(alice, Parent, message));
        });

        ParaB::execute_with(|| {
            assert_eq!(
				foreign_parachain::ForeignUniques::owner((Parent, GeneralIndex(1)).into(), 77u32.into()),
				Some(ALICE),
			);
		});

        Relay::execute_with(|| {
			assert_eq!(relay_chain::Uniques::owner(1, 77u32), Some(parachain_sovereign_account_id(2)));
		});


    }
}


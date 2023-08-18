#[cfg(test)]
mod tests {
	use crate::setup::*;
	use codec::Encode;
	use frame_support::{assert_ok, pallet_prelude::Weight};
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	/// Scenario:
	/// Parachain A sends two transact instructions to the relay chain.
	/// The first instruction creates a NFT collection with as admin Parachain A.
	/// The second instruction mints a NFT for the collection with as Owner ALICE.
	#[test]
	fn transact_mint_nft() {
		MockNet::reset();

		let create_collection = relay_chain::RuntimeCall::Uniques(pallet_uniques::Call::<
			relay_chain::Runtime,
		>::create {
			collection: 1u32,
			admin: parachain_sovereign_account_id(1),
		});

		// let message_fee = relay_chain::estimate_message_fee(4);
		let create_collection_weight_estimation = Weight::from_parts(1_000_000_000, 10_000);
		// let create_collection_fee_estimation =
		// 	relay_chain::estimate_fee_for_weight(create_collection_weight_estimation);
		let mint_nft_weight_estimation = Weight::from_parts(1_000_000_000, 10_000);
		// let mint_nft_fee_estimation =
		// 	relay_chain::estimate_fee_for_weight(mint_nft_weight_estimation);
		// let fees = message_fee + create_collection_fee_estimation + mint_nft_fee_estimation;

		let mint =
			relay_chain::RuntimeCall::Uniques(pallet_uniques::Call::<relay_chain::Runtime>::mint {
				collection: 1u32,
				item: 1u32,
				owner: ALICE,
			});

		let message = Xcm(vec![
			// WithdrawAsset((Here, fees).into()),
			// BuyExecution { fees: (Here, fees).into(), weight_limit: WeightLimit::Unlimited },
			Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: create_collection_weight_estimation,
				call: create_collection.encode().into(),
			},
			Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: mint_nft_weight_estimation,
				call: mint.encode().into(),
			},
		]);

		// Create collection with Alice as owner.
		ParaA::execute_with(|| {
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
		});

		Relay::execute_with(|| {
			assert_eq!(
				relay_chain::Uniques::collection_owner(1u32),
				Some(parachain_sovereign_account_id(1))
			);
			assert_eq!(relay_chain::Uniques::owner(1u32, 1u32), Some(ALICE));
		});
	}
}

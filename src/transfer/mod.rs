#[cfg(test)]
mod tests {
	use crate::setup::*;
	use frame_support::assert_ok;
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;
    /// Scenario:
	/// ALICE transfers relay native tokens from parachain A to parachain B.
	#[test]
	fn transfer_para_to_para() {
		MockNet::reset();

		let withdraw_amount = 50 * CENTS;

		// Estimated from the number of instructions and knowledge of the config
		let fee_in_source = parachain::estimate_message_fee(3);
		let fee_in_relay = relay_chain::estimate_message_fee(4);
		let fee_in_destination = parachain::estimate_message_fee(4);

		// In this case, we know exactly how much fees we need for each step of the process
		let message: Xcm<parachain::RuntimeCall> = Xcm(vec![
			WithdrawAsset((Parent, withdraw_amount).into()), // Fees are paid in the relay's token
			BuyExecution {
				fees: (Parent, fee_in_source).into(),
				weight_limit: WeightLimit::Unlimited,
			},
			InitiateReserveWithdraw {
				assets: All.into(),
				reserve: Parent.into(),
				xcm: Xcm(vec![
					BuyExecution {
						fees: (Here, fee_in_relay).into(),
						weight_limit: WeightLimit::Unlimited,
					},
					DepositReserveAsset {
						assets: All.into(),
						dest: Parachain(3).into(),
						xcm: Xcm(vec![
							BuyExecution {
								fees: (Parent, fee_in_destination).into(),
								weight_limit: WeightLimit::Unlimited,
							},
							DepositAsset {
								assets: All.into(),
								beneficiary: Junction::AccountId32 {
									id: ALICE.into(),
									network: None,
								}
								.into(),
							},
						]),
					},
				]),
			},
		]);

		let fee_until_relay = fee_in_source + fee_in_relay;
		let fee_until_destination = fee_until_relay + fee_in_destination;

		ParaA::execute_with(|| {
			assert_ok!(parachain::PolkadotXcm::execute(
				parachain::RuntimeOrigin::signed(ALICE),
				Box::new(xcm::VersionedXcm::V3(message.into())),
				(100_000_000_000, 100_000_000_000).into(),
			));

			assert_eq!(parachain::Assets::balance(0, &ALICE), INITIAL_BALANCE - withdraw_amount);
		});

		Relay::execute_with(|| {
			assert_eq!(
				relay_chain::Balances::free_balance(&parachain_sovereign_account_id(3)),
				INITIAL_BALANCE + withdraw_amount - fee_until_relay
			);
		});

		ParaC::execute_with(|| {
			assert_eq!(
				parachain::Assets::balance(0, &ALICE),
				INITIAL_BALANCE + withdraw_amount - fee_until_destination
			);
		});
	}
}
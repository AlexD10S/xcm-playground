# XCM-playground
This repository contains the xcm examples for the Deep Dives session.
The examples are set up using the [XCM-simulator](https://github.com/paritytech/polkadot/tree/master/xcm/xcm-simulator)
and for the integration tests the [Parachains Integration Tests tool](https://github.com/paritytech/parachains-integration-tests).

To create the setup for testing and some of the examples, has been very useful the [XCM Docs](https://paritytech.github.io/xcm-docs).
You can clone their repo and play with the examples there:
```sh
https://github.com/paritytech/xcm-docs.git
```

#### How to run
To run the examples, do the following:
1. Clone the repository:
`git clone https://github.com/AlexD10S/xcm-playground.git`

2. cd to the examples folder:
`cd xcm-playground`

3. Run all the tests: 
`cargo test`
or a single test:
`cargo test -p xcm-examples trap_and_claim_assets -- --nocapture`


#### Examples

##### Mint NFT
 Parachain A sends two transact instructions to the relay chain.
The first instruction creates a NFT collection with as admin Parachain A.
The second instruction mints a NFT for the collection with as Owner ALICE.
```sh
cd mint_nft
cargo test -p xcm-playground transact_mint_nft -- --nocapture
```

<!-- To play, delete the  -->

##### Transfer NFT
The relay-chain transfers an NFT into a parachain's sovereign account, who then mints a  trustless-backed-derivated locally.
```sh
cd transfer_nft
cargo test -p xcm-playground reserve_asset_transfer_nft -- --nocapture
```

##### Teleport Asset
Basic ALICE teleports her native assets from the relay chain to parachain A.
```sh
cd teleport
cargo test -p xcm-playground teleport_fungible -- --nocapture
```

Play with Config, to change Barrier, and add instructions to pay fees.

##### Lock Asset
ALICE from parachain A locks 5 cents of relay chain native assets of its Sovereign account on the relay chain and assigns Parachain B as unlocker.
Parachain A then asks Parachain B to unlock the funds partly. Parachain B responds by sending an UnlockAssets instruction to the relay chain.
```sh
cd lock
cargo test -p xcm-playground remote_locking -- --nocapture
```


#### Integrations tests
We will be using [Zombienet](https://github.com/paritytech/zombienet/). A cli tool to easily spawn ephemeral Polkadot/Substrate networks and perform tests against them.

And the [Parachains Integration Tests](https://github.com/paritytech/parachains-integration-tests) tool to perform XCM tests against this networks.

For Zombienet we specify the networks we want to run in the .toml file: `xcm_playground.toml`.

For that we need to create some binaries and add them in the `integration_tests/bin folder`:
- `polkadot` (which you can download from [the releases](https://github.com/paritytech/polkadot/releases))
- `polkadot-parachain` (which you will build from [cumulus](https://github.com/paritytech/cumulus))
- `trappist-node`. Our parachain. Trappist is a web3 developer playground used by the Delivery Services team at Parity for experimenting with XCM and different new Polkadot features.
You can create your binary here: https://github.com/paritytech/trappist

For this demo we are using the version `polkadot-v0.9.42`, so binaries will need to use that version.

First install integrations-tests tool following the instructions from the repo: https://github.com/paritytech/parachains-integration-tests
```sh
yarn global add ts-node

yarn global add @parity/parachains-integration-tests
```
Then install Zombienet using  the instructions from the repo: https://github.com/paritytech/zombienet/ 
```sh
cd src/integration_tests
chmod +x zombienet-macos
```

In the repo integration_tests we have the .toml file with the networks we will run with zombienet, run it:
```sh
./zombienet-macos spawn xcm_playground.toml -p native
```

When everything is running, run the tests, specified in the file `0_reserve_transfer.yml`:
```sh
npx parachains-integration-tests -m test -t 0_reserve_transfer.yml
```
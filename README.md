# xcm-playground
This repository contains the xcm examples for the Deep Dives session. 
The examples are set up using the [XCM-simulator](https://github.com/paritytech/polkadot/tree/master/xcm/xcm-simulator).

#### How to run
To run the examples, do the following:
1. Clone the repository:
`git clone https://github.com/paritytech/xcm-docs.git`

2. cd to the examples folder:
`cd xcm-playground

3. Run all the tests: 
`cargo test`
or a single test:
`cargo test -p xcm-examples trap_and_claim_assets -- --nocapture`


#### How to configure your playground
1. Create a repo: cargo new playground
2. You can copy the basic setup from https://github.com/paritytech/xcm-docs.git where it has a setup of relaychain, parachains and asset hub with examples.
3. You can customize this setup to test different scenarios
4. Create your own tests:
    #[cfg(test)]
    mod tests {
        use crate::setup::*;
        ... 
        #[test]
        fn test() {
            ...
        }
    }

Teleport
`cargo test -p xcm-examples trap_and_claim_assets -- --nocapture`



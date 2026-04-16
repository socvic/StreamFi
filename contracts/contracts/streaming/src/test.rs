#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(StreamingContract, ());
    let client = StreamingClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin).unwrap();
}

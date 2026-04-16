#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(IncomeTokenContract, ());
    let client = IncomeTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin).unwrap();
}

#[test]
fn test_mint() {
    let env = Env::default();
    let contract_id = env.register(IncomeTokenContract, ());
    let client = IncomeTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let start = env.ledger().timestamp();
    let end = start + 86400 * 30;

    let stream_id = client.mint(&admin, &1000i128, &start, &end).unwrap();
    assert_eq!(stream_id, 1);
}

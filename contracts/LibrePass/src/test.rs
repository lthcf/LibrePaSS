#![cfg(test)]
use super::{LibrePassContract, LibrePassContractClient, FineRecord, DataKey};
use soroban_sdk::{testutils::Address as _, token, Address, Env, Symbol};

fn setup_test_env<'a>() -> (Env, LibrePassContractClient<'a>, Address, Address, token::Client<'a>) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, LibrePassContract);
    let client = LibrePassContractClient::new(&env, &contract_id);

    let student = Address::generate(&env);
    let token_admin = Address::generate(&env);
    
    // Register standard token for mock payment handling (USDC alternative)
    let token_contract_id = env.register_stellar_asset_contract(token_admin);
    let token_client = token::Client::new(&env, &token_contract_id);

    // Mint tokens to student balance
    token_client.mint(&student, &1000);

    (env, client, student, token_contract_id, token_client)
}

#[test]
fn test_1_happy_path_pay_fine() {
    let (env, client, student, token_id, token_client) = setup_test_env();
    let book_id = Symbol::new(&env, "BOOK99");
    let fine_amount = 150i128;

    client.issue_fine(&book_id, &student, &fine_amount);
    
    // Execute contract payment
    client.pay_fine(&book_id, &student, &token_id);

    // Verify balances shifted correctly
    assert_eq!(token_client.balance(&student), 850);
    assert_eq!(token_client.balance(&client.address), 150);
    assert!(client.is_cleared(&book_id));
}

#[test]
#[should_panic(expected = "Fine has already been paid.")]
fn test_2_edge_case_duplicate_payment_fails() {
    let (env, client, student, token_id, _) = setup_test_env();
    let book_id = Symbol::new(&env, "BOOK101");

    client.issue_fine(&book_id, &student, &50);
    client.pay_fine(&book_id, &student, &token_id);
    
    // Attempting secondary transaction on same item causes failure
    client.pay_fine(&book_id, &student, &token_id);
}

#[test]
fn test_3_state_verification_reflects_in_storage() {
    let (env, client, student, _, _) = setup_test_env();
    let book_id = Symbol::new(&env, "BOOK777");

    client.issue_fine(&book_id, &student, &300);
    
    // Confirm initial state is unpaid
    assert!(!client.is_cleared(&book_id));
}

#[test]
#[should_panic(expected = "Unauthorized: Payer must be the student assigned to the fine.")]
fn test_4_edge_case_unauthorized_payer_fails() {
    let (env, client, student, token_id, _) = setup_test_env();
    let book_id = Symbol::new(&env, "BOOK55");
    let malicious_user = Address::generate(&env);

    client.issue_fine(&book_id, &student, &200);
    
    // Malicious user attempts to pass payment payload under their footprint authority
    client.pay_fine(&book_id, &malicious_user, &token_id);
}

#[test]
#[should_panic(expected = "Fine record not found.")]
fn test_5_edge_case_pay_nonexistent_fine_fails() {
    let (env, client, student, token_id, _) = setup_test_env();
    let invalid_book_id = Symbol::new(&env, "FAKEBOOK");

    // Attempting settlement execution flow on record data never registered
    client.pay_fine(&invalid_book_id, &student, &token_id);
}
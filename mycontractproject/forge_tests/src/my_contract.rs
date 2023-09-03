#![cfg(test)]

use crate::{forge::execute, runner};
use primitive_types::U256;

pub async fn test_mycontract_with_zero_token_minted() {
    // runner
    let mut runner = runner();

    // executor
    let tokens_minted =
        execute::<_, U256>(&mut runner, "ContractTest", "testDropWithZeroTokens", ())
            .await
            .unwrap();

    assert_eq!(tokens_minted, U256::from(0));
}

pub async fn test_mycontract_with_custom_tokens_minted(amount: U256) {
    // runner
    let mut runner = runner();

    // executor
    let tokens_minted = execute::<_, U256>(
        &mut runner,
        "ContractTest",
        "testDropWithCustomTokens",
        amount,
    )
    .await
    .unwrap();

    assert_eq!(tokens_minted, amount);
}

async fn test_mycontract_with_two_tokens_minted() {
    let mut runner = runner();

    let tokes_minted =
        execute::<_, U256>(&mut runner, "ContractTest", "testDropWithTwoTokens", ())
            .await
            .unwrap();

    assert_eq!(tokes_minted, U256::from(2));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_minted_zero_tokens() {
    test_mycontract_with_zero_token_minted().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_minted_two_tokens() {
    test_mycontract_with_two_tokens_minted().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_minted_custom_tokens() {
    test_mycontract_with_custom_tokens_minted(U256::from(3)).await;
}



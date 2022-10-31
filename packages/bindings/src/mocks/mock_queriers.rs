//! Contains some useful functions to perform unit testing of smart contracts.

use cosmwasm_std::testing::MockQuerier;
use cosmwasm_std::{Coin, QueryRequest, QuerierResult, Empty};
use std::marker::PhantomData;

/// Custom querier that can be used during unit testing to simulate what a contract receive when
/// perform a query toward Desmos’s modules.
pub struct MockDesmosQuerier {
    /// Default CosmWASM mock querier.
    pub mock_querier: MockQuerier,
}

impl MockDesmosQuerier {
    /// Initialize a new [`MockDesmosQuerier`].
    /// * `balances` - Slice of balances passed to the `bank` module, where the first element
    /// is the user address and the second element is the user's balance.
    pub fn new(balances: &[(&str, &[Coin])]) -> Self {
        MockDesmosQuerier {
            mock_querier: MockQuerier::new(balances),
        }
    }

    /// Handle the query request.
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match request {
            _ => self.mock_querier.handle_query(request),
        }
    }
}

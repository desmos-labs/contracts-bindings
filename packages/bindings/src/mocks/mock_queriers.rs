//! Contains some useful functions to perform unit testing of smart contracts.

use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    from_slice, Binary, Coin, ContractResult, Empty, OwnedDeps, Querier, QuerierResult,
    QueryRequest, SystemError, SystemResult,
};
use std::collections::HashMap;
use std::marker::PhantomData;

/// Custom querier that can be used during unit testing to simulate what a contract receive when
/// perform a query toward Desmos’s modules.
pub struct MockDesmosQuerier {
    /// Default CosmWASM mock querier.
    pub mock_querier: MockQuerier,
    /// Registered custom queries using proto request for testing.
    pub registered_custom_queries: HashMap<String, Box<dyn Fn(&Binary) -> QuerierResult>>,
}

impl MockDesmosQuerier {
    /// Initialize a new [`MockDesmosQuerier`].
    /// * `balances` - Slice of balances passed to the `bank` module, where the first element
    /// is the user address and the second element is the user's balance.
    pub fn new(balances: &[(&str, &[Coin])]) -> Self {
        MockDesmosQuerier {
            mock_querier: MockQuerier::new(balances),
            registered_custom_queries: HashMap::new(),
        }
    }

    pub fn with_custom_query<CH>(mut self, path: String, response_fn: CH) -> Self
    where
        CH: Fn(&Binary) -> ContractResult<Binary> + 'static,
    {
        self.registered_custom_queries
            .insert(path, Self::wrap_handler(response_fn));
        return self;
    }

    /// Handle the query request.
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match request {
            QueryRequest::Stargate { path, data } => {
                if let Some(response_fn) = self.registered_custom_queries.get(path) {
                    return response_fn(data);
                }
                SystemResult::Err(SystemError::UnsupportedRequest { kind: path.into() })
            }
            _ => self.mock_querier.handle_query(request),
        }
    }

    /// Utility function to wrap the handler that returns a ContractResult<Binary>
    /// to make it return a SystemResult<ContractResult<Binary>>
    fn wrap_handler<'f, CH>(handler: CH) -> Box<dyn Fn(&Binary) -> QuerierResult>
    where
        CH: Fn(&Binary) -> ContractResult<Binary> + 'static,
    {
        Box::new(move |data| {
            let result = handler(data);
            if result.is_ok() {
                SystemResult::Ok(result)
            } else {
                SystemResult::Err(SystemError::UnsupportedRequest {
                    kind: result.unwrap_err(),
                })
            }
        })
    }
}

impl Querier for MockDesmosQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl Default for MockDesmosQuerier {
    fn default() -> Self {
        MockDesmosQuerier::new(&[])
    }
}

/// Creates an instance of [`OwnedDeps`](cosmwasm_std::OwnedDeps) with a custom [`MockDesmosQuerier`]
/// to allow the user to mock the query responses of one or more Desmos's modules.
///
pub fn mock_desmos_dependencies_with_custom_querier(
    querier: MockDesmosQuerier,
) -> OwnedDeps<MockStorage, MockApi, MockDesmosQuerier, Empty> {
    OwnedDeps::<_, _, _, Empty> {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier,
        custom_query_type: PhantomData,
    }
}

/// Creates an instance of [`OwnedDeps`](cosmwasm_std::OwnedDeps) that is capable of
/// handling queries towards Desmos's modules.
pub fn mock_desmos_dependencies() -> OwnedDeps<MockStorage, MockApi, MockDesmosQuerier, Empty> {
    mock_desmos_dependencies_with_custom_querier(MockDesmosQuerier::default())
}

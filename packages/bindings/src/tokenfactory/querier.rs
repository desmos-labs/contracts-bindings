//! Contains a querier to query data from the Desmos x/tokenfactory module.

use crate::tokenfactory::types::*;
use cosmwasm_std::{Empty, QuerierWrapper, StdResult};

/// Querier allows to query data from the Desmos x/tokenfactory module.
pub struct TokenfactoryQuerier<'a> {
    querier: crate::tokenfactory::types::TokenfactoryQuerier<'a, Empty>,
}

impl<'a> TokenfactoryQuerier<'a> {
    /// Create a new [TokenfactoryQuerier]
    ///
    /// # Example
    /// ```
    /// use cosmwasm_std::{DepsMut, MessageInfo};
    /// use desmos_bindings::tokenfactory::querier::TokenfactoryQuerier;
    ///
    /// pub fn contract_action(deps: DepsMut, _: MessageInfo) {
    ///     let querier = TokenfactoryQuerier::new(&deps.querier);
    ///     let response = querier.query_subspace_denoms(1);
    /// }
    /// ```
    pub fn new(querier: &'a QuerierWrapper<'a, Empty>) -> Self {
        Self {
            querier: crate::tokenfactory::types::TokenfactoryQuerier::new(querier),
        }
    }
}

impl<'a> TokenfactoryQuerier<'a> {
    /// Queries all the denoms created by the given subspace.
    /// 
    /// * `subspace_id` - Id of the subspace to query the managed denoms for.
    pub fn query_subspace_denoms(
        &self,
        subspace_id: u64,
    ) -> StdResult<QuerySubspaceDenomsResponse> {
        self.querier.subspace_denoms(subspace_id)
    }
}

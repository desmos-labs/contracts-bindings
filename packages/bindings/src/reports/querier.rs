//! Contains the querier that can be used to query data related to the x/reports module.

use crate::reports::proto::*;
use crate::reports::types::ReportTarget;
use crate::types::PageRequest;
use cosmwasm_std::{Addr, Empty, QuerierWrapper, StdResult};
#[cfg(feature = "iterators")]
use {
    crate::iter::page_iterator::{Page, PageIterator},
    crate::reports::models::{Reason, Report},
    cosmwasm_std::Binary,
};

/// Querier able to query data from the Desmos x/reports module.
pub struct ReportsQuerier<'a> {
    querier: crate::reports::proto::ReportsQuerier<'a, Empty>,
}

impl<'a> ReportsQuerier<'a> {
    /// Creates a new instance of [`ReportsQuerier`].
    ///
    /// # Example
    /// ```
    /// use std::ops::Deref;
    /// use cosmwasm_std::{DepsMut, MessageInfo};
    /// use desmos_bindings::reports::querier::ReportsQuerier;
    ///
    /// pub fn contract_action(deps: DepsMut, _: MessageInfo) {
    ///     let querier = ReportsQuerier::new(&deps.querier);
    /// }
    /// ```
    pub fn new(querier: &'a QuerierWrapper<'a, Empty>) -> Self {
        Self {
            querier: crate::reports::proto::ReportsQuerier::new(querier),
        }
    }

    /// Queries the reports for a specific target.
    ///
    /// * `subspace_id` - Id of the subspace to query the reports for.
    /// * `target` - Target to query the reports for.
    /// * `reporter` - User that reported the target.
    /// This is going to be used only if `target` is not `None`.
    /// * `pagination` - Pagination configs.
    pub fn query_reports(
        &self,
        subspace_id: u64,
        target: Option<ReportTarget>,
        reporter: Option<Addr>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryReportsResponse> {
        Ok(self.querier.reports(
            subspace_id,
            target.map(Into::into),
            reporter.unwrap_or_else(|| Addr::unchecked("")).into(),
            pagination.map(Into::into),
        )?)
    }

    /// Gives an iterator to scan over the reports for a specific target.
    ///
    /// * `subspace_id` - Id of the subspace to query the reports for.
    /// * `target` - Target to query the reports for.
    /// * `reporter` - User that reported the target.
    /// This is going to be used only if `target` is not `None`.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_reports(
        &self,
        subspace_id: u64,
        target: Option<ReportTarget>,
        reporter: Option<Addr>,
        page_size: u64,
    ) -> PageIterator<Report, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_reports(
                    subspace_id,
                    target.clone(),
                    reporter.clone(),
                    Some(PageRequest {
                        key,
                        offset: None,
                        limit: limit.into(),
                        count_total: false,
                        reverse: false,
                    }),
                )
                .map(|response| Page {
                    items: response.reports,
                    next_page_key: response
                        .pagination
                        .and_then(|page_response| page_response.next_key),
                })
            }),
            page_size,
        )
    }

    /// Queries the report having the provided id.
    ///
    /// * `subspace_id` - Id of the subspace that holds the report to query for.
    /// * `report_id` - Id of the report to query for.
    pub fn query_report(&self, subspace_id: u64, report_id: u64) -> StdResult<QueryReportResponse> {
        Ok(self.querier.report(subspace_id, report_id)?)
    }

    /// Queries the supported reporting reasons for a subspace.
    ///
    /// * `subspace_id` - Id of the subspace to query the supported reporting reasons for.
    /// * `pagination` - Pagination configs.
    pub fn query_reasons(
        &self,
        subspace_id: u64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryReasonsResponse> {
        Ok(self.querier.reasons(subspace_id, pagination.map(Into::into))?)
    }

    /// Gives an iterator to scan over the supported reporting reasons for a subspace.
    ///
    /// * `subspace_id` - Id of the subspace to query the supported reporting reasons for.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_reasons(
        &self,
        subspace_id: u64,
        page_size: u64,
    ) -> PageIterator<Reason, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_reasons(
                    subspace_id,
                    Some(PageRequest {
                        key,
                        offset: None,
                        limit: limit.into(),
                        count_total: false,
                        reverse: false,
                    }),
                )
                .map(|response| Page {
                    items: response.reasons,
                    next_page_key: response
                        .pagination
                        .and_then(|page_response| page_response.next_key),
                })
            }),
            page_size,
        )
    }

    /// Queries the reason having the given id.
    ///
    /// * `subspace_id` - Id of the subspace that holds the reason to query for.
    /// * `reason_id` - Id of the reason to query for.
    pub fn query_reason(&self, subspace_id: u64, reason_id: u32) -> StdResult<QueryReasonResponse> {
        Ok(self.querier.reason(subspace_id, reason_id)?)
    }
}

#[cfg(test)]
mod tests {}

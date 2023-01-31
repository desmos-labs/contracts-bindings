//! Contains the querier that can be used to query data related to the x/reports module.

use crate::legacy::query::DesmosQuery;
use crate::legacy::reports::models::{RawReportTarget, ReportTarget};
use crate::legacy::reports::models_query::{
    QueryReasonResponse, QueryReasonsResponse, QueryReportResponse, QueryReportsResponse,
};
use crate::legacy::reports::query::ReportsQuery;
use crate::legacy::types::PageRequest;
use cosmwasm_std::{Addr, Querier, QuerierWrapper, StdResult};
#[cfg(feature = "iterators")]
use {
    crate::iter::page_iterator::{Page, PageIterator},
    crate::legacy::reports::models::{Reason, Report},
    cosmwasm_std::Binary,
};

/// Querier able to query data from the Desmos x/reports module.
pub struct ReportsQuerier<'a> {
    querier: QuerierWrapper<'a, DesmosQuery>,
}

impl<'a> ReportsQuerier<'a> {
    /// Creates a new instance of [`ReportsQuerier`].
    ///
    /// # Example
    /// ```
    /// use std::ops::Deref;
    /// use cosmwasm_std::{DepsMut, MessageInfo};
    /// use desmos_bindings::legacy::reports::querier::ReportsQuerier;
    ///
    /// pub fn contract_action(deps: DepsMut, _: MessageInfo) {
    ///     let querier = ReportsQuerier::new(deps.querier.deref());
    /// }
    /// ```
    pub fn new(querier: &'a dyn Querier) -> Self {
        Self {
            querier: QuerierWrapper::<'a, DesmosQuery>::new(querier),
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
        let request = DesmosQuery::Reports(ReportsQuery::Reports {
            subspace_id: subspace_id.into(),
            target: target.map(RawReportTarget::from),
            reporter,
            pagination,
        });

        self.querier.query(&request.into())
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
        let request = DesmosQuery::Reports(ReportsQuery::Report {
            subspace_id: subspace_id.into(),
            report_id: report_id.into(),
        });

        self.querier.query(&request.into())
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
        let request = DesmosQuery::Reports(ReportsQuery::Reasons {
            subspace_id: subspace_id.into(),
            pagination,
        });

        self.querier.query(&request.into())
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
        let request = DesmosQuery::Reports(ReportsQuery::Reason {
            subspace_id: subspace_id.into(),
            reason_id,
        });

        self.querier.query(&request.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::legacy::mocks::mock_queriers::mock_desmos_dependencies;
    use crate::legacy::reports::mocks::MockReportsQueries;
    use crate::legacy::reports::querier::ReportsQuerier;
    use cosmwasm_std::Uint64;
    use std::ops::Deref;

    #[test]
    fn test_query_reports() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let reports_querier = ReportsQuerier::new(deps.querier.deref());

        let response = reports_querier.query_reports(1, None, None, None).unwrap();
        assert_eq!(
            MockReportsQueries::get_mocked_reports(&Uint64::new(1)),
            response.reports
        );
        assert_eq!(None, response.pagination);
    }

    #[test]
    fn test_iterate_reports() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let reports_querier = ReportsQuerier::new(deps.querier.deref());

        let mut it = reports_querier.iterate_reports(1, None, None, 32);
        let expected_items = MockReportsQueries::get_mocked_reports(&Uint64::new(1));

        let first_element = it.next().unwrap().unwrap();
        let second_element = it.next().unwrap().unwrap();

        assert_eq!(expected_items.get(0).unwrap(), &first_element);
        assert_eq!(expected_items.get(1).unwrap(), &second_element);
    }

    #[test]
    fn test_query_report() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let reports_querier = ReportsQuerier::new(deps.querier.deref());

        let response = reports_querier.query_report(1, 1).unwrap();
        assert_eq!(
            MockReportsQueries::get_mocked_report(&Uint64::new(1)),
            response.report
        );
    }

    #[test]
    fn test_query_reasons() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let reports_querier = ReportsQuerier::new(deps.querier.deref());

        let response = reports_querier.query_reasons(1, None).unwrap();
        assert_eq!(
            MockReportsQueries::get_mocked_reasons(&Uint64::new(1)),
            response.reasons
        );
        assert_eq!(None, response.pagination);
    }

    #[test]
    fn test_iterate_reasons() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let reports_querier = ReportsQuerier::new(deps.querier.deref());

        let mut it = reports_querier.iterate_reasons(1, 32);
        let expected_items = MockReportsQueries::get_mocked_reasons(&Uint64::new(1));

        let first_element = it.next().unwrap().unwrap();
        let second_element = it.next().unwrap().unwrap();

        assert_eq!(expected_items.get(0).unwrap(), &first_element);
        assert_eq!(expected_items.get(1).unwrap(), &second_element);
    }

    #[test]
    fn test_query_reason() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let reports_querier = ReportsQuerier::new(deps.querier.deref());

        let response = reports_querier.query_reason(1, 1).unwrap();
        assert_eq!(
            MockReportsQueries::get_mocked_reason(&Uint64::new(1)),
            response.reason
        );
    }
}

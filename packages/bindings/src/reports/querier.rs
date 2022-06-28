use crate::iter::page_iterator::{Page, PageIterator};
use crate::query::DesmosQuery;
use crate::reports::models::{RawReportTarget, Reason, Report, ReportTarget};
use crate::reports::models_query::{
    QueryReasonResponse, QueryReasonsResponse, QueryReportResponse, QueryReportsResponse,
};
use crate::reports::query::ReportsQuery;
use crate::types::PageRequest;
use cosmwasm_std::{Addr, Binary, Querier, QuerierWrapper, StdResult, Uint64};

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
    /// use desmos_bindings::reports::querier::ReportsQuerier;
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

    pub fn query_reports(
        &self,
        subspace_id: u64,
        target: Option<ReportTarget>,
        reporter: Option<Addr>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryReportsResponse> {
        let request = DesmosQuery::Reports(ReportsQuery::Reports {
            subspace_id: Uint64::new(subspace_id),
            target: target.map(|raw| RawReportTarget::from(raw)),
            reporter,
            pagination,
        });

        self.querier.query(&request.into())
    }

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
                        limit: Uint64::from(limit),
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

    pub fn query_report(&self, subspace_id: u64, report_id: u64) -> StdResult<QueryReportResponse> {
        let request = DesmosQuery::Reports(ReportsQuery::Report {
            subspace_id: Uint64::new(subspace_id),
            report_id: Uint64::new(report_id),
        });

        self.querier.query(&request.into())
    }

    pub fn query_reasons(
        &self,
        subspace_id: u64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryReasonsResponse> {
        let request = DesmosQuery::Reports(ReportsQuery::Reasons {
            subspace_id: Uint64::new(subspace_id),
            pagination,
        });

        self.querier.query(&request.into())
    }

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
                        limit: Uint64::from(limit),
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

    pub fn query_reason(&self, subspace_id: u64, reason_id: u32) -> StdResult<QueryReasonResponse> {
        let request = DesmosQuery::Reports(ReportsQuery::Reason {
            subspace_id: Uint64::new(subspace_id),
            reason_id,
        });

        self.querier.query(&request.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::mock_dependencies_with_custom_querier;
    use crate::reports::mocks::{
        get_mocked_reason, get_mocked_reasons, get_mocked_report, get_mocked_reports,
    };
    use crate::reports::querier::ReportsQuerier;
    use cosmwasm_std::Uint64;
    use std::ops::Deref;

    #[test]
    fn test_query_reports() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let reports_querier = ReportsQuerier::new(deps.querier.deref());

        let response = reports_querier.query_reports(1, None, None, None).unwrap();
        assert_eq!(get_mocked_reports(&Uint64::new(1)), response.reports);
        assert_eq!(None, response.pagination);
    }

    #[test]
    fn test_iterate_reports() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let reports_querier = ReportsQuerier::new(deps.querier.deref());

        let mut it = reports_querier.iterate_reports(1, None, None, 32);
        let expected_items = get_mocked_reports(&Uint64::new(1));

        let first_element = it.next().unwrap().unwrap();
        let second_element = it.next().unwrap().unwrap();

        assert_eq!(expected_items.get(0).unwrap(), &first_element);
        assert_eq!(expected_items.get(1).unwrap(), &second_element);
    }

    #[test]
    fn test_query_report() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let reports_querier = ReportsQuerier::new(deps.querier.deref());

        let response = reports_querier.query_report(1, 1).unwrap();
        assert_eq!(get_mocked_report(&Uint64::new(1)), response.report);
    }

    #[test]
    fn test_query_reasons() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let reports_querier = ReportsQuerier::new(deps.querier.deref());

        let response = reports_querier.query_reasons(1, None).unwrap();
        assert_eq!(get_mocked_reasons(&Uint64::new(1)), response.reasons);
        assert_eq!(None, response.pagination);
    }

    #[test]
    fn test_iterate_reasons() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let reports_querier = ReportsQuerier::new(deps.querier.deref());

        let mut it = reports_querier.iterate_reasons(1, 32);
        let expected_items = get_mocked_reasons(&Uint64::new(1));

        let first_element = it.next().unwrap().unwrap();
        let second_element = it.next().unwrap().unwrap();

        assert_eq!(expected_items.get(0).unwrap(), &first_element);
        assert_eq!(expected_items.get(1).unwrap(), &second_element);
    }

    #[test]
    fn test_query_reason() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let reports_querier = ReportsQuerier::new(deps.querier.deref());

        let response = reports_querier.query_reason(1, 1).unwrap();
        assert_eq!(get_mocked_reason(&Uint64::new(1)), response.reason);
    }
}

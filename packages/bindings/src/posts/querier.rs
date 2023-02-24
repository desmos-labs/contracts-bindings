//! Contains the querier that can be used to query data related to the x/posts module.

use crate::posts::types::*;
use crate::types::PageRequest;
use cosmwasm_std::{Addr, Empty, QuerierWrapper, StdResult};
#[cfg(feature = "iterators")]
use {
    crate::iter::page_iterator::{Page, PageIterator},
    crate::posts::types::{Attachment, Post, UserAnswer},
    cosmwasm_std::Binary,
};

/// Querier allows to query data from the Desmos x/posts module.
pub struct PostsQuerier<'a> {
    querier: crate::posts::types::PostsQuerier<'a, Empty>,
}

impl<'a> PostsQuerier<'a> {
    /// Creates a new instance of [`PostsQuerier`].
    ///
    /// # Example
    /// ```
    /// use cosmwasm_std::{DepsMut, MessageInfo};
    /// use desmos_bindings::posts::querier::PostsQuerier;
    ///
    /// pub fn contract_action(deps: DepsMut, _: MessageInfo) {
    ///     let querier = PostsQuerier::new(&deps.querier);
    /// }
    /// ```
    pub fn new(querier: &'a QuerierWrapper<'a, Empty>) -> Self {
        Self {
            querier: crate::posts::types::PostsQuerier::new(querier),
        }
    }

    /// Queries posts created inside a subspace.
    ///
    /// * `subspace_id` - Subspace to query the posts for.
    /// * `pagination` - Optional pagination configs.
    pub fn query_subspace_posts(
        &self,
        subspace_id: u64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QuerySubspacePostsResponse> {
        self.querier
            .subspace_posts(subspace_id, pagination.map(Into::into))
    }

    /// Gives an iterator to scan over the posts created inside a subspace.
    ///
    /// * `subspace_id` - Subspace to query the posts for.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_subspace_posts(
        &self,
        subspace_id: u64,
        page_size: u64,
    ) -> PageIterator<Post, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_subspace_posts(
                    subspace_id,
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.posts,
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }

    /// Queries all the posts inside a give section.
    ///
    /// * `subspace_id` - Subspace to query the posts for.
    /// * `section_id` - Section to query the post for.
    /// * `pagination` - Optional pagination configs.
    pub fn query_section_posts(
        &self,
        subspace_id: u64,
        section_id: u32,
        pagination: Option<PageRequest>,
    ) -> StdResult<QuerySectionPostsResponse> {
        self.querier
            .section_posts(subspace_id, section_id, pagination.map(Into::into))
    }

    /// Gives an iterator to scan over the posts created inside a give section.
    ///
    /// * `subspace_id` - Subspace to query the posts for.
    /// * `section_id` - Section to query the post for.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_section_posts(
        &self,
        subspace_id: u64,
        section_id: u32,
        page_size: u64,
    ) -> PageIterator<Post, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_section_posts(
                    subspace_id,
                    section_id,
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.posts,
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }

    /// Queries a single post inside a given subspace.
    ///
    /// * `subspace_id` - Id of the subspace where the post is stored.
    /// * `post_id` - Id of the post to query for.
    pub fn query_post(&self, subspace_id: u64, post_id: u64) -> StdResult<QueryPostResponse> {
        self.querier.post(subspace_id, post_id)
    }

    /// Queries the attachments of the post having the given `post_id`.
    ///
    /// * `subspace_id` - Id of the subspace where the post is stored.
    /// * `post_id` - Id of the post to query the attachments for.
    /// * `pagination` - Optional pagination configs.
    pub fn query_post_attachments(
        &self,
        subspace_id: u64,
        post_id: u64,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryPostAttachmentsResponse> {
        self.querier
            .post_attachments(subspace_id, post_id, pagination.map(Into::into))
    }

    /// Gives an iterator to scan over the attachments of the post having the given `post_id`.
    ///
    /// * `subspace_id` - Id of the subspace where the post is stored.
    /// * `post_id` - Id of the post to query the attachments for.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_post_attachments(
        &self,
        subspace_id: u64,
        post_id: u64,
        page_size: u64,
    ) -> PageIterator<Attachment, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_post_attachments(
                    subspace_id,
                    post_id,
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.attachments,
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }

    /// Queries the answers for the poll having the given `post_id`.
    ///
    /// * `subspace_id` - Id of the subspace where the post is stored.
    /// * `poll_id` - Id of the post that holds the poll.
    /// * `user` - Optional address of the user to query the responses for.
    /// * `pagination` - Optional pagination configs.
    pub fn query_poll_answers(
        &self,
        subspace_id: u64,
        post_id: u64,
        poll_id: u32,
        user: Option<Addr>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryPollAnswersResponse> {
        self.querier.poll_answers(
            subspace_id,
            post_id,
            poll_id,
            user.unwrap_or_else(|| Addr::unchecked("")).into(),
            pagination.map(Into::into),
        )
    }

    /// Gives an iterator to scan over the answers for the poll having the given `post_id`.
    ///
    /// * `subspace_id` - Id of the subspace where the post is stored.
    /// * `poll_id` - Id of the post that holds the poll.
    /// * `user` - Optional address of the user to query the responses for.
    /// * `page_size` - Size of the page requested to the chain.
    #[cfg(feature = "iterators")]
    pub fn iterate_poll_answers(
        &self,
        subspace_id: u64,
        post_id: u64,
        poll_id: u32,
        user: Option<Addr>,
        page_size: u64,
    ) -> PageIterator<UserAnswer, Binary> {
        PageIterator::new(
            Box::new(move |key, limit| {
                self.query_poll_answers(
                    subspace_id,
                    post_id,
                    poll_id,
                    user.clone(),
                    Some(PageRequest {
                        key,
                        limit: limit.into(),
                        reverse: false,
                        count_total: false,
                        offset: None,
                    }),
                )
                .map(|response| Page {
                    items: response.answers,
                    next_page_key: response.pagination.and_then(|response| {
                        (!response.next_key.is_empty()).then_some(Binary::from(response.next_key))
                    }),
                })
            }),
            page_size,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::mock_queriers::mock_desmos_dependencies;
    use crate::posts::mocks::MockPostsQueries;
    #[test]
    fn test_query_subspace_posts() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(&deps.querier);

        let result = querier.query_subspace_posts(1, None);
        let response = result.unwrap();
        let expected = MockPostsQueries::get_mocked_subspace_posts_response();

        assert_eq!(expected, response)
    }

    #[test]
    fn test_iterate_subspace_posts() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(&deps.querier);

        let mut iterator = querier.iterate_subspace_posts(1, 32);
        let expected = MockPostsQueries::get_mocked_subspace_posts_response();

        assert_eq!(expected.posts[0], iterator.next().unwrap().unwrap());
        assert_eq!(expected.posts[1], iterator.next().unwrap().unwrap());
        assert!(iterator.next().is_none())
    }

    #[test]
    fn test_query_section_posts() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(&deps.querier);

        let result = querier.query_section_posts(1, 1, None);
        let response = result.unwrap();
        let expected = MockPostsQueries::get_mocked_section_posts_response();

        assert_eq!(expected, response)
    }

    #[test]
    fn test_iterate_section_posts() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(&deps.querier);

        let mut iterator = querier.iterate_section_posts(1, 1, 32);
        let expected = MockPostsQueries::get_mocked_section_posts_response();

        assert_eq!(expected.posts[0], iterator.next().unwrap().unwrap());
        assert_eq!(expected.posts[1], iterator.next().unwrap().unwrap());
        assert!(iterator.next().is_none())
    }

    #[test]
    fn test_query_post() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(&deps.querier);

        let result = querier.query_post(1, 1);
        let response = result.unwrap();
        let expected = MockPostsQueries::get_mocked_post_response();

        assert_eq!(expected, response)
    }

    #[test]
    fn test_query_post_attachments() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(&deps.querier);

        let result = querier.query_post_attachments(1, 1, None);
        let response = result.unwrap();
        let expected = MockPostsQueries::get_mocked_post_attachments_response();

        assert_eq!(expected, response)
    }

    #[test]
    fn test_iterate_post_attachments() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(&deps.querier);

        let mut iterator = querier.iterate_post_attachments(1, 1, 32);
        let expected = MockPostsQueries::get_mocked_post_attachments_response();

        assert_eq!(expected.attachments[0], iterator.next().unwrap().unwrap());
        assert_eq!(expected.attachments[1], iterator.next().unwrap().unwrap());
        assert!(iterator.next().is_none())
    }

    #[test]
    fn test_query_poll_answers() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(&deps.querier);

        let result = querier.query_poll_answers(1, 1, 1, None, None);
        let response = result.unwrap();
        let expected = MockPostsQueries::get_mocked_poll_answers_response();

        assert_eq!(expected, response)
    }

    #[test]
    fn test_iterate_poll_answers() {
        let owned_deps = mock_desmos_dependencies();
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(&deps.querier);

        let mut iterator = querier.iterate_poll_answers(1, 1, 1, None, 32);
        let expected = MockPostsQueries::get_mocked_poll_answers_response();

        assert_eq!(expected.answers[0], iterator.next().unwrap().unwrap());
        assert!(iterator.next().is_none())
    }
}

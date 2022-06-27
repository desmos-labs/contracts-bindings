//! Contains the querier that can be used to query data related to the x/posts module.

use crate::posts::models_query::{
    QueryPollAnswersResponse, QueryPostAttachmentsResponse, QueryPostResponse,
    QuerySectionPostsResponse, QuerySubspacePostsResponse,
};
use crate::posts::query::PostsQuery;
use crate::query::DesmosQuery;
use crate::types::PageRequest;
use cosmwasm_std::{Addr, Querier, QuerierWrapper, StdResult, Uint64};
#[cfg(feature = "iterators")]
use {
    crate::iter::page_iterator::{Page, PageIterator},
    crate::posts::models::{Attachment, Post, UserAnswer},
    cosmwasm_std::Binary,
};

/// Querier able to query data from the Desmos x/profiles module.
pub struct PostsQuerier<'a> {
    querier: cosmwasm_std::QuerierWrapper<'a, DesmosQuery>,
}

impl<'a> PostsQuerier<'a> {
    /// Creates a new instance of [`PostsQuerier`].
    ///
    /// # Example
    /// ```
    /// use std::ops::Deref;
    /// use cosmwasm_std::{DepsMut, MessageInfo};
    /// use desmos_bindings::posts::querier::PostsQuerier;
    ///
    /// pub fn contract_action(deps: DepsMut, _: MessageInfo) {
    ///     let querier = PostsQuerier::new(deps.querier.deref());
    /// }
    /// ```
    pub fn new(querier: &'a dyn Querier) -> Self {
        Self {
            querier: QuerierWrapper::<'a, DesmosQuery>::new(querier),
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
        let request = DesmosQuery::Posts(PostsQuery::SubspacePosts {
            subspace_id: Uint64::new(subspace_id),
            pagination,
        });

        self.querier.query(&request.into())
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
                    next_page_key: response.pagination.and_then(|response| response.next_key),
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
        self.querier.query(
            &DesmosQuery::Posts(PostsQuery::SectionPosts {
                subspace_id: subspace_id.into(),
                section_id,
                pagination,
            })
            .into(),
        )
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
                    next_page_key: response.pagination.and_then(|response| response.next_key),
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
        self.querier.query(
            &DesmosQuery::Posts(PostsQuery::Post {
                subspace_id: subspace_id.into(),
                post_id: post_id.into(),
            })
            .into(),
        )
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
        self.querier.query(
            &DesmosQuery::Posts(PostsQuery::PostAttachments {
                subspace_id: subspace_id.into(),
                post_id: post_id.into(),
                pagination,
            })
            .into(),
        )
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
                    next_page_key: response.pagination.and_then(|response| response.next_key),
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
        self.querier.query(
            &DesmosQuery::Posts(PostsQuery::PollAnswers {
                subspace_id: subspace_id.into(),
                post_id: post_id.into(),
                poll_id,
                user,
                pagination,
            })
            .into(),
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
                    next_page_key: response.pagination.and_then(|response| response.next_key),
                })
            }),
            page_size,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::mock_dependencies_with_custom_querier;
    use crate::posts::mocks::{
        get_mocked_poll_answers, get_mocked_post, get_mocked_post_attachments,
        get_mocked_section_posts, get_mocked_subspace_posts,
    };
    use crate::posts::querier::PostsQuerier;
    use cosmwasm_std::Uint64;
    use std::ops::Deref;

    #[test]
    fn test_query_subspace_posts() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(deps.querier.deref());

        let result = querier.query_subspace_posts(0, None);
        let response = result.unwrap();

        assert!(response.pagination.is_none());
        assert_eq!(2, response.posts.len());

        let posts = response.posts;
        assert_eq!(get_mocked_subspace_posts(&Uint64::zero()), posts);
    }

    #[test]
    fn test_iterate_subspace_posts() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(deps.querier.deref());

        let mut iterator = querier.iterate_subspace_posts(0, 32);
        let expected_posts = get_mocked_subspace_posts(&Uint64::zero());

        // The first item returned from the iterators should be the first item returned from the mock function.
        assert_eq!(
            expected_posts.get(0).unwrap(),
            &iterator.next().unwrap().unwrap()
        );
        // The second item returned from the iterators should be the second item returned from the mock function.
        assert_eq!(
            expected_posts.get(1).unwrap(),
            &iterator.next().unwrap().unwrap()
        );
        // The third item should be none since the mock function provides only 2 posts.
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_query_section_posts() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(deps.querier.deref());

        let result = querier.query_section_posts(0, 0, None);
        let response = result.unwrap();

        assert!(response.pagination.is_none());
        assert_eq!(2, response.posts.len());

        let posts = response.posts;
        assert_eq!(get_mocked_section_posts(&Uint64::zero(), &0), posts);
    }

    #[test]
    fn test_iterate_section_posts() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(deps.querier.deref());

        let mut iterator = querier.iterate_section_posts(0, 0, 32);
        let expected_posts = get_mocked_section_posts(&Uint64::zero(), &0);

        // The first item returned from the iterators should be the first item returned from the mock function.
        assert_eq!(
            expected_posts.get(0).unwrap(),
            &iterator.next().unwrap().unwrap()
        );
        // The second item returned from the iterators should be the second item returned from the mock function.
        assert_eq!(
            expected_posts.get(1).unwrap(),
            &iterator.next().unwrap().unwrap()
        );
        // The third item should be none since the mock function provides only 2 posts.
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_query_post() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(deps.querier.deref());

        let result = querier.query_post(0, 42);
        let expected_post = get_mocked_post(Uint64::zero(), Uint64::new(42));

        assert_eq!(expected_post, result.unwrap().post);
    }

    #[test]
    fn test_query_post_attachments() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(deps.querier.deref());

        let result = querier.query_post_attachments(0, 0, None);
        let response = result.unwrap();

        assert!(response.pagination.is_none());
        assert_eq!(2, response.attachments.len());

        let attachments = response.attachments;
        assert_eq!(
            get_mocked_post_attachments(&Uint64::zero(), &Uint64::zero()),
            attachments
        );
    }

    #[test]
    fn test_iterate_post_attachments() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(deps.querier.deref());

        let mut iterator = querier.iterate_post_attachments(0, 0, 32);
        let expected_attachments = get_mocked_post_attachments(&Uint64::zero(), &Uint64::zero());

        // The first item returned from the iterators should be the first item returned from the mock function.
        assert_eq!(
            expected_attachments.get(0).unwrap(),
            &iterator.next().unwrap().unwrap()
        );
        // The second item returned from the iterators should be the second item returned from the mock function.
        assert_eq!(
            expected_attachments.get(1).unwrap(),
            &iterator.next().unwrap().unwrap()
        );
        // The third item should be none since the mock function provides only 2 attachments.
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_query_poll_answers() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(deps.querier.deref());

        let result = querier.query_poll_answers(0, 0, 0, None, None);
        let response = result.unwrap();

        assert!(response.pagination.is_none());
        assert_eq!(1, response.answers.len());

        let answers = response.answers;
        assert_eq!(
            get_mocked_poll_answers(&Uint64::zero(), &Uint64::zero(), &0, &None),
            answers
        );
    }

    #[test]
    fn test_iterate_poll_answers() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = PostsQuerier::new(deps.querier.deref());

        let mut iterator = querier.iterate_poll_answers(0, 0, 0, None, 32);
        let expected_answers = get_mocked_poll_answers(&Uint64::zero(), &Uint64::zero(), &0, &None);

        // The first item returned from the iterators should be the first item returned from the mock function.
        assert_eq!(
            expected_answers.get(0).unwrap(),
            &iterator.next().unwrap().unwrap()
        );

        // The second item should be none since the mock function provides only 1 response.
        assert!(iterator.next().is_none());
    }
}

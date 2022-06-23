//! Contains a querier to query data from the Desmos x/reactions module.

/// Querier able to query data from the Desmos x/reactions module.
pub struct ReactionsQuerier<'a> {
    querier: QuerierWrapper<'a, DesmosQuery>,
}


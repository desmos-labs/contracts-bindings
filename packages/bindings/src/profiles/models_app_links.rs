//! Contains structs and enums related to the application links.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

/// Defines the state of an application link.
#[cw_serde]
pub enum ApplicationLinkState {
    /// A link has just been initialized.
    #[serde(rename = "ApplicationLinkStateInitialized")]
    Initialized,

    /// A link has just started being verified.
    #[serde(rename = "AppLinkStateVerificationStarted")]
    VerificationStarted,

    /// A link has errored during the verification process.
    #[serde(rename = "AppLinkStateVerificationError")]
    VerificationError,

    /// A link has being verified successfully.
    #[serde(rename = "AppLinkStateVerificationSuccess")]
    VerificationSuccess,

    /// A link has timed out while waiting for the verification.
    #[serde(rename = "AppLinkStateVerificationTimedOut")]
    TimedOut,
}

/// Represent a link between a Desmos profile and a centralized application.
#[cw_serde]
pub struct ApplicationLink {
    /// User to which the link is associated.
    pub user: Addr,
    /// The details of this application link.
    pub data: Data,
    /// State of the link.
    pub state: ApplicationLinkState,
    /// Request that has been made to the oracle.
    pub oracle_request: OracleRequest,
    /// Data coming from the result of the verification.
    /// Only available when the state is `VerificationSuccess`.
    pub result: Option<AppLinkResult>,
    /// The time when the link was created.
    pub creation_time: String,
    /// The time when the link is expired.
    pub expiration_time: String,
}

/// Represents the data associated to a specific user of a
/// generic centralized application.
#[cw_serde]
pub struct Data {
    /// The application name (eg. Twitter, GitHub, etc).
    pub application: String,
    /// Username on the application (eg. Twitter tag, GitHub profile, etc).
    pub username: String,
}

/// Represents a generic oracle request used to verify the ownership of
/// a centralized application account.
#[cw_serde]
pub struct OracleRequest {
    /// Id of the request.
    pub id: Uint64,
    /// id of an oracle script.
    pub oracle_script_id: Uint64,
    /// Contains the data used to perform the oracle request
    pub call_data: CallData,
    /// represents the ID of the client that has called the oracle script
    pub client_id: String,
}

/// Represents the data sent to a single oracle request in order to
/// verify the ownership of a centralized application by a Desmos profile.
#[cw_serde]
pub struct CallData {
    /// The application for which the ownership should be verified.
    pub application: String,
    /// The hex encoded call data that should be used to verify the application
    /// account ownership.
    pub call_data: String,
}

/// Represents a verification result.
#[cw_serde]
pub enum AppLinkResult {
    /// Result of an application link that has been successfully verified.
    Success {
        /// Hex-encoded value that has be signed by the profile.
        value: String,
        /// Hex-encoded signature that has been produced by signing the value.
        signature: String,
    },
    /// result of an application link that has not been verified successfully.
    Failed {
        /// Error that is associated with the failure.
        error: String,
    },
}

/// Contains the details of a single app link owner.
#[cw_serde]
pub struct ApplicationLinkOwnerDetails {
    /// Address of the link owner.
    pub user: Addr,
    /// Name of the application.
    pub application: String,
    /// Unique name of the application target.
    pub username: String,
}

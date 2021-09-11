use serde::{Deserialize, Serialize};

use super::{Extensions, OAuthFlowObject};

/// Allows configuration of the supported OAuth Flows.
#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthFlowsObject {
    /// Configuration for the OAuth Implicit flow
    pub implicit: Option<OAuthFlowObject>,
    /// Configuration for the OAuth Resource Owner Password flow
    pub password: Option<OAuthFlowObject>,
    /// Configuration for the OAuth Client Credentials flow. Previously called `application` in OpenAPI 2.0.
    #[serde(rename = "clientCredentials")]
    pub client_credentials: Option<OAuthFlowObject>,
    /// Configuration for the OAuth Authorization Code flow. Previously called `accessCode` in OpenAPI 2.0.
    #[serde(rename = "authorizationCode")]
    pub authorization_code: Option<OAuthFlowObject>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

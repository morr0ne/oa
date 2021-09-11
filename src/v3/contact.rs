use serde::{Deserialize, Serialize};
use url::Url;

use super::Extensions;

/// Contact information for the exposed API.
#[derive(Debug, Deserialize, Serialize)]
pub struct ContactObject {
    /// The identifying name of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the contact information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    /// The email address of the contact person/organization. MUST be in the format of an email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

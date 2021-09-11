use serde::{Deserialize, Serialize};
use url::Url;

use super::{ContactObject, Extensions, LicenseObject};

/// The object provides metadata about the API. The metadata MAY be used by the clients if needed, and MAY be presented in editing or documentation generation tools for convenience.
#[derive(Debug, Deserialize, Serialize)]
pub struct InfoObject {
    /// The title of the API.
    pub title: String,
    /// A short description of the API. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A URL to the Terms of Service for the API.
    #[serde(rename = "termsOfService", skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<Url>,
    /// The contact information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactObject>,
    /// The license information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseObject>,
    /// The version of the OpenAPI document (which is distinct from the [OpenAPI Specification version](https://spec.openapis.org/oas/v3.0.3#oasVersion) or the API implementation version).
    pub version: String,
    #[serde(flatten)]
    pub extensions: Extensions,
}

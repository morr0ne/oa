use serde::{Deserialize, Serialize};
use url::Url;

use super::Extensions;

/// License information for the exposed API.
#[derive(Debug, Deserialize, Serialize)]
pub struct LicenseObject {
    /// The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

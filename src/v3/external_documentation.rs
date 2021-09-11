use serde::{Deserialize, Serialize};
use url::Url;

use super::Extensions;

/// Allows referencing an external resource for extended documentation.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalDocumentationObject {
    /// A short description of the target documentation. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL for the target documentation.
    pub url: Url,
    #[serde(flatten)]
    pub extensions: Extensions,
}

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::MediaTypeObject;

/// Describes a single request body.
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestBodyObject {
    /// A brief description of the request body. This could contain examples of use. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The content of the request body. The key is a media type or [media type range](https://tools.ietf.org/html/rfc7231#appendix-D) and the value describes it. For requests that match multiple keys, only the most specific key is applicable. e.g. text/plain overrides text/*
    pub content: IndexMap<String, MediaTypeObject>,
    /// Determines if the request body is required in the request.
    #[serde(default)]
    pub required: bool,
}

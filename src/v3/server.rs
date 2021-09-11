use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{Extensions, ServerVariableObject};

/// An object representing a Server.
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerObject {
    /// A URL to the target host. This URL supports Server Variables and MAY be relative, to indicate that the host location is relative to the location where the OpenAPI document is being served. Variable substitutions will be made when a variable is named in {brackets}.
    pub url: String,
    /// An optional string describing the host designated by the URL. [CommonMark syntax](https://spec.commonmark.org/) be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A map between a variable name and its value. The value is used for substitution in the server's URL template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<IndexMap<String, ServerVariableObject>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

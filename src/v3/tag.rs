use serde::{Deserialize, Serialize};

use super::{Extensions, ExternalDocumentationObject};

/// Adds metadata to a single tag that is used by the [Operation Object](OperationObject). It is not mandatory to have a Tag Object per tag defined in the Operation Object instances.
#[derive(Debug, Deserialize, Serialize)]
pub struct TagObject {
    /// The name of the tag.
    pub name: String,
    /// A short description for the tag. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

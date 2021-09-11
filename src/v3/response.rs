use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{LinkObject, MediaTypeObject, ReferenceObjectOr};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseObject {
    pub description: String,
    // TODO: headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<IndexMap<String, MediaTypeObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<IndexMap<String, ReferenceObjectOr<LinkObject>>>,
}

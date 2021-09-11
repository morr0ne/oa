use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{EncodingObject, ExampleObject, ReferenceObjectOr, SchemaObject};

/// Each Media Type Object provides schema and examples for the media type identified by its key.
#[derive(Debug, Deserialize, Serialize)]
pub struct MediaTypeObject {
    /// The schema defining the content of the request, response, or parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ReferenceObjectOr<SchemaObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<IndexMap<String, ExampleObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<IndexMap<String, EncodingObject>>,
}

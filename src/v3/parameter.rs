use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{ExampleObject, SchemaOrContentObject};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "in")]
pub enum ParameterObject {
    #[serde(rename = "path")]
    Path {
        #[serde(flatten)]
        fixed_fields: ParameterObjectFixedFields,
        // TODO: style
    },
    #[serde(rename = "query")]
    Query {
        #[serde(flatten)]
        fixed_fields: ParameterObjectFixedFields,
        #[serde(default, rename = "allowEmptyValue")]
        allow_empty_value: bool,
        #[serde(default, rename = "allowReserved")]
        allow_reserved: bool,
        // TODO: style
    },
    #[serde(rename = "header")]
    Header {
        #[serde(flatten)]
        fixed_fields: ParameterObjectFixedFields,
        // TODO: style
    },
    #[serde(rename = "cookie")]
    Cookie {
        #[serde(flatten)]
        fixed_fields: ParameterObjectFixedFields,
        // TODO: style
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParameterObjectFixedFields {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,
    #[serde(flatten)]
    pub schema: SchemaOrContentObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<IndexMap<String, ExampleObject>>,
}

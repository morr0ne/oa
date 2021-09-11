use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::num::NonZeroU32;

use super::{
    DiscriminatorObject, ExternalDocumentationObject, MediaTypeObject, ReferenceObjectOr, XmlObject,
};

/// The Schema Object allows the definition of input and output data types. These types can be objects, but also primitives and arrays. This object is an extended subset of the [JSON Schema Specification Wright Draft 00](https://json-schema.org/).
/// For more information about the properties, see [JSON Schema Core](https://tools.ietf.org/html/draft-wright-json-schema-00) and [JSON Schema Validation](https://tools.ietf.org/html/draft-wright-json-schema-validation-00). Unless stated otherwise, the property definitions follow the JSON Schema.
#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaObject {
    // This are all fixed fields
    #[serde(default)]
    pub nullable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<DiscriminatorObject>,
    #[serde(default, rename = "readOnly")]
    pub read_only: bool,
    #[serde(default, rename = "writeOnly")]
    pub write_only: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml: Option<XmlObject>,
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    #[serde(default)]
    pub deprecated: bool,
    // This fields are taken from the JSON Schema definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "multipleOf", skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<NonZeroU32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
    #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(default, rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u32>,
    // TODO
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SchemaOrContentObject {
    #[serde(rename = "schema")]
    Schema(ReferenceObjectOr<SchemaObject>),
    #[serde(rename = "conent")]
    Content(IndexMap<String, MediaTypeObject>),
}

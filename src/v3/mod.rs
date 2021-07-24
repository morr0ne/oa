use http::StatusCode as HttpStatusCode;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::num::{NonZeroU16, NonZeroU32};
use url::Url;

use crate::utils::default_true;

pub type SecurityRequirementObject = IndexMap<String, Vec<String>>;
pub type CallbackObject = IndexMap<String, PathsItemObject>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ReferenceObjectOr<T> {
    Object(T),
    ReferenceObject {
        #[serde(rename = "$ref")]
        reference: String,
    },
}

/// This is the root document object of the [OpenAPI document](https://spec.openapis.org/oas/v3.0.3#oasDocument).
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAPIObject {
    /// This string MUST be the [semantic version](https://semver.org/spec/v2.0.0.html) number of the [OpenAPI Specification version](https://spec.openapis.org/oas/v3.0.3#versions) that the OpenAPI document uses. The openapi field SHOULD be used by tooling specifications and clients to interpret the OpenAPI document. This is not related to the API info.version string.
    pub openapi: String,
    /// Provides metadata about the API. The metadata MAY be used by tooling as required.
    pub info: InfoObject,
    pub servers: Vec<ServerObject>,
    pub paths: IndexMap<String, ReferenceObjectOr<PathsItemObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<ComponentsObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirementObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagObject>>,
    /// Additional external documentation.
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
}

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
    // TODO: Specification Extensions.
}

/// Contact information for the exposed API.
#[derive(Debug, Deserialize, Serialize)]
pub struct ContactObject {
    /// The identifying name of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the contact information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    /// The email address of the contact person/organization. MUST be in the format of an email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    // TODO: Specification Extensions.
}

/// License information for the exposed API.
#[derive(Debug, Deserialize, Serialize)]
pub struct LicenseObject {
    /// The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    // TODO: Specification Extensions.
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerObject {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<IndexMap<String, ServerVariableObject>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerVariableObject {
    pub enumeration: Vec<String>,
    pub default: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PathsItemObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ReferenceObjectOr<ParameterObject>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OperationObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
    #[serde(rename = "operationId", skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ReferenceObjectOr<ParameterObject>>>,
    #[serde(rename = "requestBody", skip_serializing_if = "Option::is_none")]
    pub request_body: Option<ReferenceObjectOr<RequestBodyObject>>,
    pub responses: ResponsesObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<IndexMap<String, ReferenceObjectOr<CallbackObject>>>,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirementObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
}

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
pub struct DiscriminatorObject {
    #[serde(rename = "propertyName")]
    pub property_name: String,
    pub mapping: IndexMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct XmlObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default)]
    pub attribute: bool,
    #[serde(default)]
    pub wrapped: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MediaTypeObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ReferenceObjectOr<SchemaObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<IndexMap<String, ExampleObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<IndexMap<String, EncodingObject>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EncodingObject {
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>, // TODO: default value
    // TODO: headers
    // TODO: style
    #[serde(default)]
    pub explode: bool,
    #[serde(default, rename = "allowReserved")]
    pub allow_reserved: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SchemaOrContentObject {
    #[serde(rename = "schema")]
    Schema(ReferenceObjectOr<SchemaObject>),
    #[serde(rename = "conent")]
    Content(IndexMap<String, MediaTypeObject>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExampleObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "externalValue", skip_serializing_if = "Option::is_none")]
    pub external_value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestBodyObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub content: IndexMap<String, MediaTypeObject>,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponsesObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<ReferenceObjectOr<ResponseObject>>,
    #[serde(flatten)]
    pub responses: IndexMap<String, ReferenceObjectOr<ResponseObject>>, // TODO: Should be a http status code instead of a String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseObject {
    pub description: String,
    // TODO: headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<IndexMap<String, MediaTypeObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<IndexMap<String, ReferenceObjectOr<LinkObject>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LinkObject {
    #[serde(rename = "operationRef", skip_serializing_if = "Option::is_none")]
    pub operation_ref: Option<String>,
    #[serde(rename = "operationId", skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    // TODO: parameters
    // TODO: requestBody
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<ServerObject>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentsObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<IndexMap<String, ReferenceObjectOr<SchemaObject>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<IndexMap<String, ReferenceObjectOr<ResponseObject>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<IndexMap<String, ReferenceObjectOr<ParameterObject>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<IndexMap<String, ReferenceObjectOr<ExampleObject>>>,
    #[serde(rename = "requestBodies", skip_serializing_if = "Option::is_none")]
    pub request_bodies: Option<IndexMap<String, ReferenceObjectOr<RequestBodyObject>>>,
    // TODO:  headers
    // TODO:  securitySchemes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<IndexMap<String, ReferenceObjectOr<LinkObject>>>,
    // TODO: callbacks
    // TODO
}

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
    // TODO: Specification Extensions.
}

/// Allows referencing an external resource for extended documentation.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalDocumentationObject {
    /// A short description of the target documentation. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL for the target documentation.
    pub url: Url,
    // TODO: Specification Extensions.
}

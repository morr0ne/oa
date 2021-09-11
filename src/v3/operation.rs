use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{
    CallbackObject, Extensions, ExternalDocumentationObject, ParameterObject, ReferenceObjectOr,
    RequestBodyObject, ResponsesObject, SecurityRequirementObject, ServerObject,
};

/// Describes a single API operation on a path.
#[derive(Debug, Deserialize, Serialize)]
pub struct OperationObject {
    /// A list of tags for API documentation control. Tags can be used for logical grouping of operations by resources or any other qualifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// A short summary of what the operation does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the operation behavior. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this operation.
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
    /// Unique string used to identify the operation. The id MUST be unique among all operations described in the API. The operationId value is case-sensitive. Tools and libraries MAY use the operationId to uniquely identify an operation, therefore, it is RECOMMENDED to follow common programming naming conventions.
    #[serde(rename = "operationId", skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ReferenceObjectOr<ParameterObject>>>,
    /// The request body applicable for this operation. The requestBody is only supported in HTTP methods where the HTTP 1.1 specification [RFC7231](https://tools.ietf.org/html/rfc7231#section-4.3.1) has explicitly defined semantics for request bodies. In other cases where the HTTP spec is vague, requestBody SHALL be ignored by consumers.
    #[serde(rename = "requestBody", skip_serializing_if = "Option::is_none")]
    pub request_body: Option<ReferenceObjectOr<RequestBodyObject>>,
    pub responses: ResponsesObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<IndexMap<String, ReferenceObjectOr<CallbackObject>>>,
    /// Declares this operation to be deprecated. Consumers SHOULD refrain from usage of the declared operation.
    #[serde(default)]
    pub deprecated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirementObject>>,
    /// An alternative server array to service this operation. If an alternative server object is specified at the Path Item Object or Root level, it will be overridden by this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

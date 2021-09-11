use indexmap::IndexMap;

/// Lists the required security schemes to execute this operation. The name used for each property MUST correspond to a security scheme declared in the [Security Schemes](https://spec.openapis.org/oas/v3.0.3#componentsSecuritySchemes) under the [Components Object](ComponentsObject).
/// Security Requirement Objects that contain multiple schemes require that all schemes MUST be satisfied for a request to be authorized. This enables support for scenarios where multiple query parameters or HTTP headers are required to convey security information.
/// When a list of Security Requirement Objects is defined on the [OpenAPI Object](OpenAPIObject) or [Operation Object](OperationObject), only one of the Security Requirement Objects in the list needs to be satisfied to authorize the request.
pub type SecurityRequirementObject = IndexMap<String, Vec<String>>;

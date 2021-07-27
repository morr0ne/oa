use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::num::NonZeroU32;
use url::Url;

/// Lists the required security schemes to execute this operation. The name used for each property MUST correspond to a security scheme declared in the [Security Schemes](https://spec.openapis.org/oas/v3.0.3#componentsSecuritySchemes) under the [Components Object](ComponentsObject).
/// Security Requirement Objects that contain multiple schemes require that all schemes MUST be satisfied for a request to be authorized. This enables support for scenarios where multiple query parameters or HTTP headers are required to convey security information.
/// When a list of Security Requirement Objects is defined on the [OpenAPI Object](OpenAPIObject) or [Operation Object](OperationObject), only one of the Security Requirement Objects in the list needs to be satisfied to authorize the request.
pub type SecurityRequirementObject = IndexMap<String, Vec<String>>;
pub type CallbackObject = IndexMap<String, PathsItemObject>;
/// Holds the relative paths to the individual endpoints and their operations. The path is appended to the URL from the [Server Object](ServerObject) in order to construct the full URL. The Paths MAY be empty, due to [ACL constraints](https://spec.openapis.org/oas/v3.0.3#securityFiltering).
pub type PathsObject = IndexMap<String, ReferenceObjectOr<PathsItemObject>>; // TODO: Specification Extensions. // TODO: better implement this object
pub type Extensions = IndexMap<String, Value>;

// TODO: It might me a better idea, even if more complicated, to drop this enum and implement ref for some objects.
// That way it would better documented and more comformant to the spec
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
    /// An array of Server Objects, which provide connectivity information to a target server. If the servers property is not provided, or is an empty array, the default value would be a Server Object with a url value of /.
    pub servers: Vec<ServerObject>, // TODO: add custom deserialization
    /// The available paths and operations for the API.
    pub paths: PathsObject, // TODO
    /// An element to hold various schemas for the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<ComponentsObject>, // TODO
    /// A declaration of which security mechanisms can be used across the API. The list of values includes alternative security requirement objects that can be used. Only one of the security requirement objects need to be satisfied to authorize a request. Individual operations can override this definition. To make security optional, an empty security requirement ({}) can be included in the array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirementObject>>,
    /// A list of tags used by the specification with additional metadata. The order of the tags can be used to reflect on their order by the parsing tools. Not all tags that are used by the [Operation Object](OperationObject) must be declared. The tags that are not declared MAY be organized randomly or based on the tools' logic. Each tag name in the list MUST be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagObject>>,
    /// Additional external documentation.
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
    #[serde(flatten)]
    pub extensions: Extensions,
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
    #[serde(flatten)]
    pub extensions: Extensions,
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
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// License information for the exposed API.
#[derive(Debug, Deserialize, Serialize)]
pub struct LicenseObject {
    /// The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

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

/// An object representing a Server Variable for server URL template substitution.
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerVariableObject {
    /// An enumeration of string values to be used if the substitution options are from a limited set. The array SHOULD NOT be empty.
    #[serde(rename = "enum")]
    pub enumeration: Vec<String>, // TODO: ensure it's not empty
    /// The default value to use for substitution, which SHALL be sent if an alternate value is not supplied. Note this behavior is different than the [Schema Object's](SchemaObject) treatment of default values, because in those cases parameter values are optional. If the enum is defined, the value SHOULD exist in the enum's values.
    pub default: String,
    /// An optional description for the server variable. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
    pub description: String,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Describes the operations available on a single path. A Path Item MAY be empty, due to ACL constraints. The path itself is still exposed to the documentation viewer but they will not know which operations and parameters are available.
#[derive(Debug, Deserialize, Serialize)]
pub struct PathsItemObject {
    /// An optional, string summary, intended to apply to all operations in this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// An optional, string description, intended to apply to all operations in this path. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<OperationObject>,
    /// A definition of a PUT operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<OperationObject>,
    /// A definition of a POST operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<OperationObject>,
    /// A definition of a DELETE operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<OperationObject>,
    /// A definition of a OPTIONS operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OperationObject>,
    /// A definition of a HEAD operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<OperationObject>,
    /// A definition of a PATCH operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<OperationObject>,
    /// A definition of a TRACE operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<OperationObject>,
    /// An alternative server array to service all operations in this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ReferenceObjectOr<ParameterObject>>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

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
    /// An object to hold reusable [Security Scheme Objects](SecuritySchemeObject).
    #[serde(rename = "securitySchemes")]
    pub security_schemes: IndexMap<String, ReferenceObjectOr<SecuritySchemeObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<IndexMap<String, ReferenceObjectOr<LinkObject>>>,
    // TODO: callbacks
    #[serde(flatten)]
    pub extensions: Extensions,
    // TODO
}

/// Defines a security scheme that can be used by the operations. Supported schemes are HTTP authentication, an API key (either as a header, a cookie parameter or as a query parameter), OAuth2's common flows (implicit, password, client credentials and authorization code) as defined in [RFC6749](https://tools.ietf.org/html/rfc6749), and [OpenID Connect Discovery](https://tools.ietf.org/html/draft-ietf-oauth-discovery-06).
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SecuritySchemeObject {
    #[serde(rename = "apiKey")]
    ApiKey {
        /// A short description for security scheme. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// The name of the header, query or cookie parameter to be used.
        name: String,
        /// The location of the API key
        #[serde(rename = "in")]
        location: ApiKeyLocation,
    },
    #[serde(rename = "http")]
    Http {
        /// A short description for security scheme. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// The name of the HTTP Authorization scheme to be used in the [Authorization header as defined in RFC7235](https://tools.ietf.org/html/rfc7235#section-5.1). The values used SHOULD be registered in the [IANA Authentication Scheme registry](https://www.iana.org/assignments/http-authschemes/http-authschemes.xhtml).
        scheme: String,
        /// A hint to the client to identify how the bearer token is formatted. Bearer tokens are usually generated by an authorization server, so this information is primarily for documentation purposes.
        #[serde(rename = "bearerFormat", skip_serializing_if = "Option::is_none")]
        bearer_format: Option<String>,
    },
    #[serde(rename = "oauth2")]
    Oauth2 {
        /// A short description for security scheme. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// An object containing configuration information for the flow types supported.
        flows: OAuthFlowsObject,
    },
    #[serde(rename = "openIdConnect")]
    OpenIdConnect {
        /// A short description for security scheme. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// OpenId Connect URL to discover OAuth2 configuration values.
        #[serde(rename = "openIdConnectUrl")]
        open_id_connect_url: Url,
    },
    // TODO: Specification Extensions.
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ApiKeyLocation {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "cookie")]
    Cookie,
}

/// Allows configuration of the supported OAuth Flows.
#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthFlowsObject {
    /// Configuration for the OAuth Implicit flow
    pub implicit: Option<OAuthFlowObject>,
    /// Configuration for the OAuth Resource Owner Password flow
    pub password: Option<OAuthFlowObject>,
    /// Configuration for the OAuth Client Credentials flow. Previously called `application` in OpenAPI 2.0.
    #[serde(rename = "clientCredentials")]
    pub client_credentials: Option<OAuthFlowObject>,
    /// Configuration for the OAuth Authorization Code flow. Previously called `accessCode` in OpenAPI 2.0.
    #[serde(rename = "authorizationCode")]
    pub authorization_code: Option<OAuthFlowObject>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Configuration details for a supported OAuth Flow
#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthFlowObject {
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
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Allows referencing an external resource for extended documentation.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalDocumentationObject {
    /// A short description of the target documentation. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL for the target documentation.
    pub url: Url,
    #[serde(flatten)]
    pub extensions: Extensions,
}

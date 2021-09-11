use serde::{Deserialize, Serialize};

use super::{
    ComponentsObject, Extensions, ExternalDocumentationObject, InfoObject, PathsObject,
    SecurityRequirementObject, ServerObject, TagObject,
};

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

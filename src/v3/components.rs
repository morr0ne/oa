use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{
    ExampleObject, Extensions, LinkObject, ParameterObject, ReferenceObjectOr, RequestBodyObject,
    ResponseObject, SchemaObject, SecuritySchemeObject,
};

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

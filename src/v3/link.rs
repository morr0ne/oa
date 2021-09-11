use serde::{Deserialize, Serialize};

use super::ServerObject;

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

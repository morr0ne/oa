use serde::{Deserialize, Serialize};

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

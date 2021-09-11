use serde::{Deserialize, Serialize};

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

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscriminatorObject {
    #[serde(rename = "propertyName")]
    pub property_name: String,
    pub mapping: IndexMap<String, String>,
}

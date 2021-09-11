use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{ReferenceObjectOr, ResponseObject};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponsesObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<ReferenceObjectOr<ResponseObject>>,
    #[serde(flatten)]
    pub responses: IndexMap<String, ReferenceObjectOr<ResponseObject>>, // TODO: Should be a http status code instead of a String
}

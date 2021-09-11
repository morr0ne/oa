use serde::{Deserialize, Serialize};

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

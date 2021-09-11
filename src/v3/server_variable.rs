use serde::{Deserialize, Serialize};

use super::Extensions;

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

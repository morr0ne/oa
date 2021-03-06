pub type JsonValue = serde_json::Value;
pub type JsonObject = indexmap::IndexMap<String, JsonValue>;
pub type Extensions = JsonObject;

mod callback;
mod components;
mod contact;
mod discriminator;
mod encoding;
mod example;
mod external_documentation;
mod header;
mod info;
mod license;
mod link;
mod media_type;
mod oauth_flow;
mod oauth_flows;
mod openapi;
mod operation;
mod parameter;
mod path_item;
mod paths;
mod reference;
mod request_body;
mod response;
mod responses;
mod schema;
mod security_requiriment;
mod security_scheme;
mod server;
mod server_variable;
mod tag;
mod xml;

pub use callback::*;
pub use components::*;
pub use contact::*;
pub use discriminator::*;
pub use encoding::*;
pub use example::*;
pub use external_documentation::*;
pub use header::*;
pub use info::*;
pub use license::*;
pub use link::*;
pub use media_type::*;
pub use oauth_flow::*;
pub use oauth_flows::*;
pub use openapi::*;
pub use operation::*;
pub use parameter::*;
pub use path_item::*;
pub use paths::*;
pub use reference::*;
pub use request_body::*;
pub use response::*;
pub use responses::*;
pub use schema::*;
pub use security_requiriment::*;
pub use security_scheme::*;
pub use server::*;
pub use server_variable::*;
pub use tag::*;
pub use xml::*;

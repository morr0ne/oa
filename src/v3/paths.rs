use indexmap::IndexMap;

use super::{PathsItemObject, ReferenceObjectOr};

pub type PathsObject = IndexMap<String, ReferenceObjectOr<PathsItemObject>>; // TODO: Specification Extensions. // TODO: better implement this object

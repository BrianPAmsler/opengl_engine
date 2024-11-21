use std::collections::HashSet;

use super::{ComponentID, ObjectID};


#[derive(Clone)]
pub(in crate::engine) struct GameObject {
    pub name: String,
    pub parent: ObjectID,
    pub components: Vec<ComponentID>,
    pub children: HashSet<ObjectID>
}
use std::collections::HashSet;

use super::{ComponentID, ObjectID};


#[derive(Clone)]
pub(in crate::engine::game_object) struct GameObject {
    pub name: String,
    pub parent: ObjectID,
    pub components: Vec<ComponentID>,
    pub children: HashSet<ObjectID>
}
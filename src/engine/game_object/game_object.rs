use std::{collections::{HashSet, VecDeque}, ops::{Deref, DerefMut}, cell::{RefCell, Ref}};

use anyhow::{Result, anyhow, Error, bail};

use crate::engine::{Engine, errors::ObjectError};

use super::{World, component::Component};

#[derive(Clone)]
pub(in crate::engine::game_object) struct _GameObject {
    pub name: String,
    pub parent: usize,
    pub components: Vec<Option<RefCell<Box<dyn Component>>>>,
    pub children: HashSet<usize>
}

pub struct GameObject;
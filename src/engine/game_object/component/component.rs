
use anyhow::{Error, Result};
use downcast_rs::{Downcast, impl_downcast};

use crate::engine::{game_object::ObjectID, graphics::Graphics};

pub trait Component: Downcast + CloneRequirement {
    fn init(&mut self, _graphics: &Graphics, _owner: ObjectID) -> Result<(), Error> {Ok(())}
    fn update(&mut self, _graphics: &Graphics, _owner: ObjectID, _delta_time: f32) -> Result<(), Error> {Ok(())}
    fn fixed_update(&mut self, _graphics: &Graphics, _owner: ObjectID, _delta_time: f32) -> Result<(), Error> {Ok(())}
}

impl_downcast!(Component);

pub trait CloneRequirement {
    fn clone_box(&self) -> Box<dyn Component>;
}

impl<T> CloneRequirement for T
where
    T: Component + Clone
{   
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Box<dyn Component> {
        self.clone_box()
    }
}

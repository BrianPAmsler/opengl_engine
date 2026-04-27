use crate::engine::game_object::component::Component;

pub struct Sprite {

}

impl Component for Sprite {
    fn init(&mut self, gfx: &crate::engine::graphics::Graphics, world: &crate::engine::game_object::World, owner: crate::engine::game_object::ObjectID) -> crate::engine::errors::Result<()> {Ok(())}

    fn update(&mut self, gfx: &crate::engine::graphics::Graphics, world: &crate::engine::game_object::World, owner: crate::engine::game_object::ObjectID, delta_time: f32, input: &crate::engine::input::Input) -> crate::engine::errors::Result<()> {Ok(())}

    fn fixed_update(&mut self, gfx: &crate::engine::graphics::Graphics, world: &crate::engine::game_object::World, owner: crate::engine::game_object::ObjectID, delta_time: f32, input: &crate::engine::input::Input) -> crate::engine::errors::Result<()> {Ok(())}
}
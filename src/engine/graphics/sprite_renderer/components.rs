use std::{fs::File, path::Path};

use crate::engine::{errors::Result, game_object::{ObjectID, World, component::Component}, graphics::{Graphics, image::Image, sprite_renderer::SpriteSheetID}, input::Input};

pub struct SpriteSheet {
    id: Option<SpriteSheetID>,
    filename: Option<String>
}

impl SpriteSheet {
    pub fn id(&self) -> SpriteSheetID {
        self.id.expect("Sprite sheet not initialized.")
    }
}

impl Component for SpriteSheet {
    fn init(&mut self, gfx: &Graphics, _world: &World, _owner: ObjectID) -> Result<()> {
        let path = Path::new(self.filename.as_ref().unwrap());
        let sprite_map = Image::load_from_file(path)?;

        // If add_sprite_sheet returns None it should panic, so rewrap the unwrapped result.
        self.id = Some(gfx.sprite_renderer().add_sprite_sheet(path.file_name().unwrap().to_str().unwrap(), gfx, 1024, sprite_map).unwrap());
        self.filename = None;

        Ok(())
    }

    fn on_remove(&mut self, gfx: &Graphics, _world: &World, _owner: ObjectID) -> Result<()> {
        gfx.sprite_renderer().remove_sprite_sheet(gfx, self.id.unwrap());

        Ok(())
    }
}

pub struct Sprite {
    sprite_sheet_id: SpriteSheetID,
    sprite_index: usize,

}

impl Component for Sprite {
    fn init(&mut self, gfx: &Graphics, world: &World, owner: ObjectID) -> Result<()> {Ok(())}

    fn update(&mut self, gfx: &Graphics, world: &World, owner: ObjectID, delta_time: f32, input: &Input) -> Result<()> {Ok(())}

    fn fixed_update(&mut self, gfx: &Graphics, world: &World, owner: ObjectID, delta_time: f32, input: &Input) -> Result<()> {Ok(())}
}
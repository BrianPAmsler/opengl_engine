use std::{i32, path::Path};

use gl_types::vectors::Vec2;

use crate::engine::{Engine, errors::Result, game_object::{ObjectID, component::Component}, graphics::{sprite_renderer::{SpriteDefinition, SpriteSheetID}}};

use super::SpriteData;

pub struct SpriteSheet {
    id: Option<SpriteSheetID>,
    filename: Option<String>,
    sprite_definitions: Vec<SpriteDefinition>,
    count: usize
}

impl SpriteSheet {
    pub fn new(file_name: &str) -> SpriteSheet {
        SpriteSheet { id: None, filename: Some(file_name.to_owned()), sprite_definitions: Vec::new(), count: 0 }
    }

    pub fn add_sprite(&mut self, x: u32, y: u32, width: u32, height: u32) -> u32 {
        let idx = self.sprite_definitions.len() + self.count;

        self.sprite_definitions.push(SpriteDefinition {
            x,
            y,
            width,
            height,
        });

        idx as u32
    }
}

impl SpriteSheet {
    pub fn id(&self) -> SpriteSheetID {
        self.id.expect("Sprite sheet not initialized.")
    }
}

impl Component for SpriteSheet {
    fn init(&mut self, engine: &mut Engine, _owner: ObjectID) -> Result<()> {
        let path = Path::new(self.filename.as_ref().unwrap());
        let sprite_sheet = image::open(path)?;
        let sprite_map = std::mem::take(&mut self.sprite_definitions);
        println!("init sprite sheet: {:?}", path);
        let name: Option<_> = (|| Some(path.file_name()?.to_str()?))();

        // If add_sprite_sheet returns None it should panic, so rewrap the unwrapped result.
        self.id = Some(engine.sprite_renderer.add_sprite_sheet(name.ok_or("None value")?, &mut engine.gfx, 1024, sprite_sheet, &sprite_map)?);
        self.filename = None;

        Ok(())
    }

    fn fixed_update(&mut self, engine: &mut Engine, _owner: ObjectID, _delta_time: f32) -> Result<()> { Ok(()) }

    fn on_remove(&mut self, engine: &mut Engine, _owner: ObjectID) -> Result<()> {
        engine.sprite_renderer.remove_sprite_sheet(&mut engine.gfx, self.id.unwrap());

        Ok(())
    }

    fn priority(&self) -> &'static i32 {
        &i32::MIN
    }
}

enum SpriteSheetEnum {
    ID(SpriteSheetID),
    Name(String)
}

pub struct Sprite {
    sprite_sheet_id: SpriteSheetEnum,
    pub anchor: Vec2,
    sprite_index: u32
}

impl Sprite {
    pub fn new(sprite_sheet_name: &str, sprite_index: u32) -> Sprite {
        Sprite {
            sprite_sheet_id: SpriteSheetEnum::Name(sprite_sheet_name.to_owned()),
            anchor: Vec2::ZERO,
            sprite_index
        }
    }
}

impl Component for Sprite {
    fn init(&mut self, engine: &mut Engine, _owner: ObjectID) -> Result<()> {
        self.sprite_sheet_id = SpriteSheetEnum::ID(match &self.sprite_sheet_id {
            SpriteSheetEnum::ID(_) => panic!("no"),
            SpriteSheetEnum::Name(name) => engine.sprite_renderer.get_sprite_sheet_by_name(name).ok_or(format!("Sprite sheet \"{}\" not found.", name))?,
        });

        Ok(())
    }

    fn update(&mut self, engine: &mut Engine, owner: ObjectID, _delta_time: f32) -> Result<()> {
        let SpriteSheetEnum::ID(sprite_sheet) = self.sprite_sheet_id else { return Ok(()); };
        let transform = engine.world.get_transform(owner)?;

        engine.sprite_renderer.queue_sprite_instance(
            SpriteData { position: *transform.position(), anchor: self.anchor, dimensions: transform.scale().xy(), sprite_id: self.sprite_index },
            sprite_sheet
        );

        Ok(())
    }

    fn priority(&self) -> &'static i32 {
        &i32::MAX
    }
}
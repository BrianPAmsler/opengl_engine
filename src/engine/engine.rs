use glfw::{WindowEvent, Action};

use crate::engine::errors::{Result, Error, GraphicsError};

use super::{game_object::World, graphics::Graphics, input::Input};

pub struct Engine {
    pub(in crate::engine) gfx: Option<Graphics>,
    pub(in crate::engine) world: World,
    pub(in crate::engine) fixed_tick_duration: f64,
    pub(in crate::engine) input: Input,
    pub(in crate::engine) fixed_input: Input,
    pub(in crate::engine) error_queue: Vec<Error>
}

#[derive(Debug)]
pub enum WindowMode {
    FullScreen(Option<u64>),
    Windowed
}

impl Engine {
    pub fn new() -> Result<Engine> {
        let world = World::new();
        
        Ok(Engine { gfx: None, world, fixed_tick_duration: 1.0 / 60.0, error_queue: Vec::new(), input: Input::new(), fixed_input: Input::new() })
    }

    pub fn create_window(&mut self, window_title: &str, width: u32, height: u32, window_mode: WindowMode) -> Result<()> {
        if self.gfx.is_some() {
            return Err(GraphicsError::WindowCreatedError.into());
        }
        
        self.gfx = Some(Graphics::init(window_title, width, height, window_mode)?);

        Ok(())
    }

    pub fn get_graphics(&self) -> Result<&Graphics> {
        self.gfx.as_ref().ok_or(GraphicsError::GraphicsNotInitializedError.into())
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn get_input(&self) -> &Input {
        &self.input
    }

    pub(in crate) fn get_input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub(in crate::engine) fn log_errors(&mut self) {
        // Take erorr queue from error_queue, turn it into a Box and log them
        let mut errors = Vec::new();
        std::mem::swap(&mut errors, &mut self.error_queue);
        let errors = errors.into_boxed_slice();

        errors.iter().for_each(|error| eprintln!("{}", error))
    }

    // fn init(&mut self) {
    //     let all_objs = self.world.get_root().get_all_children().unwrap_or_else(|err| {self.error_queue.push(err); Box::new([])});

    //     for obj in all_objs.to_vec().into_iter() {
    //         obj.init(&self).unwrap_or_else(|err| self.error_queue.push(err));
    //     }
    // }

    // fn game_tick(&mut self, delta_time: f32) {
    //     let all_objs = self.world.get_root().get_all_children().unwrap_or_else(|err| {self.error_queue.push(err); Box::new([])});

    //     for obj in all_objs.to_vec().into_iter() {
    //         obj.update(&self, delta_time).unwrap_or_else(|err| self.error_queue.push(err));
    //     }
    // }

    // fn fixed_game_tick(&mut self, delta_time: f32) {
    //     let all_objs = self.world.get_root().get_all_children().unwrap_or_else(|err| {self.error_queue.push(err); Box::new([])});

    //     for obj in all_objs.to_vec().into_iter() {
    //         obj.fixed_update(&self, delta_time).unwrap_or_else(|err| self.error_queue.push(err));
    //     }
    // }
}
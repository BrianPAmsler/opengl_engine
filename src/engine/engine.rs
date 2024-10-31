use glfw::{WindowEvent, Action};

use crate::engine::errors::{Result, Error, GraphicsError};

use super::{game_object::World, graphics::Graphics, input::Input};

pub struct Engine {
    gfx: Option<Graphics>,
    world: World,
    fixed_tick_duration: f64,
    input: Input,
    fixed_input: Input,
    error_queue: Vec<Error>
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

    pub fn run(&mut self) -> Result<()> {
        let mut last_tick = self.gfx.as_ref().unwrap().get_glfw_time();
        let mut last_fixed_tick = last_tick;
        let mut fixed_tick_overflow = 0.0;

        self.world.init(self.gfx.as_ref().unwrap())?;
        self.log_errors();

        while !self.gfx.as_ref().unwrap().should_close() {
            let gfx = self.gfx.as_ref().unwrap();

            gfx.poll_events();
            for msg in gfx.flush_messages() {
                match msg {
                    (_, WindowEvent::Key(key, _, Action::Press, _)) => {
                        let key_state = self.input.modify_key_state(key);
                        key_state.press = true;
                        key_state.is_down = true;

                        let fixed_key_state = self.fixed_input.modify_key_state(key);
                        fixed_key_state.press = true;
                        fixed_key_state.is_down = true;
                    },
                    (_, WindowEvent::Key(key, _, Action::Release, _)) => {
                        let key_state = self.input.modify_key_state(key);
                        key_state.release = true;
                        key_state.is_down = false;

                        let fixed_key_state = self.fixed_input.modify_key_state(key);
                        fixed_key_state.release = true;
                        fixed_key_state.is_down = false;
                    },
                    // (_, WindowEvent::Key(Key::Escape, _, Action::Press, _)) => gfx.set_should_close(true),
                    // (_, WindowEvent::Key(Key::Space, _, Action::Press, _)) => gfx.set_fullscreen(Monitor::from_primary()),
                    _ => ()
                }
            }
            
            // Game tick
            let current_time = gfx.get_glfw_time();
            self.world.update(self.gfx.as_ref().unwrap(), (current_time - last_tick) as f32, &self.input)?;
            last_tick = current_time;

            self.input.modify_all_key_states(|key| {
                key.press = false;
                key.release = false;
            });

            let fixed_diff = current_time - last_fixed_tick - self.fixed_tick_duration;

            // Add overflow to adjust for errors in timing
            if fixed_diff + fixed_tick_overflow >= 0.0 {
                fixed_tick_overflow = f64::max(0.0, fixed_diff * 2.0);
                self.world.fixed_update(self.gfx.as_ref().unwrap(), (current_time - last_fixed_tick) as f32, &self.fixed_input)?;
                last_fixed_tick = current_time;

                self.fixed_input.modify_all_key_states(|key| {
                    key.press = false;
                    key.release = false;
                });
            }

            self.log_errors();

            let gfx = self.gfx.as_ref().unwrap();

            // Render
            // gfx.render();

            // Swap front and back buffers
            gfx.swap_buffers();
        }

        Ok(())
    }

    fn log_errors(&mut self) {
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
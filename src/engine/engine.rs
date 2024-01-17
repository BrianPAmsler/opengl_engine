use anyhow::{Result, Ok, anyhow, bail, Error};
use glfw::{WindowEvent, Key, Action, Monitor};

use crate::clean_backtrace;

use super::{graphics::Graphics, game_object::World};

pub struct Engine {
    gfx: Option<Graphics>,
    pub world: &'static World,
    fixed_tick_duration: f64,
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
        
        Ok(Engine { gfx: None, world, fixed_tick_duration: 1.0 / 60.0, error_queue: Vec::new() })
    }

    pub fn create_window(&mut self, window_title: &str, width: u32, height: u32, window_mode: WindowMode) -> Result<()> {
        if self.gfx.is_some() {
            bail!("Window already created!");
        }
        
        self.gfx = Some(Graphics::init(window_title, width, height, window_mode)?);

        Ok(())
    }

    pub fn get_graphics(&self) -> Result<&Graphics> {
        self.gfx.as_ref().ok_or(anyhow!("Graphics not initialized!"))
    }
    
    pub fn get_time(&self) -> f32 {
        self.gfx.as_ref().unwrap().get_glfw_time() as f32
    }

    pub fn run(&mut self) -> Result<()> {
        let mut last_tick = self.gfx.as_ref().unwrap().get_glfw_time();
        let mut last_fixed_tick = last_tick;
        let mut fixed_tick_overflow = 0.0;

        self.init();
        self.log_errors();

        while !self.gfx.as_ref().unwrap().should_close() {
            let gfx = self.gfx.as_ref().unwrap();

            gfx.poll_events();
            for msg in gfx.flush_messages() {
                match msg {
                    (_, WindowEvent::Key(Key::Escape, _, Action::Press, _)) => gfx.set_should_close(true),
                    (_, WindowEvent::Key(Key::Space, _, Action::Press, _)) => gfx.set_fullscreen(Monitor::from_primary()),
                    _ => ()
                }
            }
            
            // Game tick
            let current_time = gfx.get_glfw_time();
            self.game_tick((current_time - last_tick) as f32);
            last_tick = current_time;

            let fixed_diff = current_time - last_fixed_tick - self.fixed_tick_duration;

            // Add overflow to adjust for errors in timing
            if fixed_diff + fixed_tick_overflow >= 0.0 {
                fixed_tick_overflow = f64::max(0.0, fixed_diff * 2.0);
                self.fixed_game_tick((current_time - last_fixed_tick) as f32);
                last_fixed_tick = current_time;
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

        errors.into_iter().for_each(|error| eprintln!("{}", clean_backtrace(error, "opengl_engine")))
    }

    fn init(&mut self) {
        let all_objs = self.world.get_root().get_all_children().unwrap_or_else(|err| {self.error_queue.push(err); Box::new([])});

        for obj in all_objs.to_vec().into_iter() {
            obj.init(&self).unwrap_or_else(|err| self.error_queue.push(err));
        }
    }

    fn game_tick(&mut self, delta_time: f32) {
        let all_objs = self.world.get_root().get_all_children().unwrap_or_else(|err| {self.error_queue.push(err); Box::new([])});

        for obj in all_objs.to_vec().into_iter() {
            obj.update(&self, delta_time).unwrap_or_else(|err| self.error_queue.push(err));
        }
    }

    fn fixed_game_tick(&mut self, delta_time: f32) {
        let all_objs = self.world.get_root().get_all_children().unwrap_or_else(|err| {self.error_queue.push(err); Box::new([])});

        for obj in all_objs.to_vec().into_iter() {
            obj.fixed_update(&self, delta_time).unwrap_or_else(|err| self.error_queue.push(err));
        }
    }
}
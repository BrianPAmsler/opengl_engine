use crate::engine::errors::{Result, Error, GraphicsError};

use super::{graphics::Graphics, game_object::World};

pub struct Engine {
    gfx: Option<Graphics>,
    world: World,
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

    pub fn get_time_64() -> f64 {
        std::time::Instant::now().elapsed().as_nanos() as f64 / 1_000_000_000f64
    }

    pub fn get_time() -> f32 {
        Engine::get_time() as f32
    }

    pub fn run(&mut self) -> Result<()> {
        let mut last_tick = Engine::get_time_64();
        let mut last_fixed_tick = last_tick;
        let mut fixed_tick_overflow = 0.0;

        self.world.init(self.gfx.as_ref().unwrap())?;
        self.log_errors();

        while true {
            let gfx = self.gfx.as_ref().unwrap();

            // gfx.poll_events();
            // for msg in gfx.flush_messages() {
            //     match msg {
            //         (_, WindowEvent::Key(Key::Escape, _, Action::Press, _)) => gfx.set_should_close(true),
            //         (_, WindowEvent::Key(Key::Space, _, Action::Press, _)) => gfx.set_fullscreen(Monitor::from_primary()),
            //         _ => ()
            //     }
            // }
            
            // Game tick
            let current_time = Engine::get_time_64();
            self.world.update(self.gfx.as_ref().unwrap(), (current_time - last_tick) as f32)?;
            last_tick = current_time;

            let fixed_diff = current_time - last_fixed_tick - self.fixed_tick_duration;

            // Add overflow to adjust for errors in timing
            if fixed_diff + fixed_tick_overflow >= 0.0 {
                fixed_tick_overflow = f64::max(0.0, fixed_diff * 2.0);
                self.world.fixed_update(self.gfx.as_ref().unwrap(), (current_time - last_fixed_tick) as f32)?;
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

        errors.into_iter().for_each(|error| eprintln!("{}", error))
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
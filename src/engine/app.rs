use glfw::{Action, WindowEvent};

use crate::engine::Engine;

use crate::Result;

pub struct Application {
    engine: Engine
}

impl Application {
    pub fn new() -> Result<Application> {
        let engine = Engine::new()?;

        Ok(Application { engine })
    }
    
    pub fn run(&mut self) -> Result<()> {
        let mut last_tick = self.engine.get_graphics()?.get_glfw_time();
        let mut last_fixed_tick = last_tick;
        let mut fixed_tick_overflow = 0.0;

        let components = self.engine.world.get_all_components()?;
        components.into_iter().try_for_each(|(owner, component)| {
            component.borrow_mut().init(&self.engine, owner)
        })?;
        self.engine.log_errors();

        while !self.engine.get_graphics()?.should_close() {
            let gfx = self.engine.gfx.as_ref().unwrap();

            gfx.poll_events();
            for msg in gfx.flush_messages() {
                match msg {
                    (_, WindowEvent::Key(key, _, Action::Press, _)) => {
                        let key_state = self.engine.input.modify_key_state(key);
                        key_state.press = true;
                        key_state.is_down = true;

                        let fixed_key_state = self.engine.fixed_input.modify_key_state(key);
                        fixed_key_state.press = true;
                        fixed_key_state.is_down = true;
                    },
                    (_, WindowEvent::Key(key, _, Action::Release, _)) => {
                        let key_state = self.engine.input.modify_key_state(key);
                        key_state.release = true;
                        key_state.is_down = false;

                        let fixed_key_state = self.engine.fixed_input.modify_key_state(key);
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
            let components = self.engine.world.get_all_components()?;
            components.into_iter().try_for_each(|(owner, component)| {
                component.borrow_mut().update(&self.engine, owner, (current_time - last_tick) as f32)
            })?;
            last_tick = current_time;

            self.engine.input.modify_all_key_states(|key| {
                key.press = false;
                key.release = false;
            });

            let fixed_diff = current_time - last_fixed_tick - self.engine.fixed_tick_duration;

            // Add overflow to adjust for errors in timing
            if fixed_diff + fixed_tick_overflow >= 0.0 {
                fixed_tick_overflow = f64::max(0.0, fixed_diff * 2.0);
                let components = self.engine.world.get_all_components()?;
                components.into_iter().try_for_each(|(owner, component)| {
                    component.borrow_mut().fixed_update(&self.engine, owner, (current_time - last_fixed_tick) as f32)
                })?;
                last_fixed_tick = current_time;

                self.engine.fixed_input.modify_all_key_states(|key| {
                    key.press = false;
                    key.release = false;
                });
            }

            self.engine.log_errors();

            let gfx = self.engine.get_graphics()?;

            // Render
            // gfx.render();

            // Swap front and back buffers
            gfx.swap_buffers();
        }

        Ok(())
    }

    pub fn run_with<F: FnMut(&mut Engine) -> Result<()>>(&mut self, mut f: F) -> Result<()> {
        f(&mut self.engine)?;

        self.run()?;

        Ok(())
    }
}
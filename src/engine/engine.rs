use std::{default, sync::Arc, time::{Duration, Instant}};

use winit::{application::ApplicationHandler, dpi::{PhysicalPosition, PhysicalSize}, event::{ElementState, KeyEvent, WindowEvent}, event_loop::{self, EventLoop}, monitor::MonitorHandle, platform::pump_events::EventLoopExtPumpEvents, window::{Fullscreen, Window, WindowAttributes}};

use crate::engine::{game_object::World, graphics::{Graphics, sprite_renderer::{self, SpriteRenderer}, terrain::terrain_renderer::TerrainRenderer}, input::{self, Input, Key}};

use super::{errors::{Error, Result}};

#[derive(Debug)]
pub enum WindowMode {
    FullScreen(Option<MonitorHandle>),
    Windowed
}

impl Into<Option<Fullscreen>> for WindowMode {
    fn into(self) -> Option<Fullscreen> {
        match self {
            WindowMode::FullScreen(monitor_handle) => Some(Fullscreen::Borderless(monitor_handle)),
            WindowMode::Windowed => None,
        }
    }
}

pub struct Engine {
    pub gfx: Graphics,
    pub world: World,
    pub input: Input,
    pub(in crate::engine) sprite_renderer: SpriteRenderer,
    pub(in crate::engine) terrain_renderer: TerrainRenderer,
    fixed_tick_duration: f64,
    // error_queue: Vec<Error>
    pub(in crate::engine) window: Arc<Window>,
    initialization_time: Instant,
    last_tick: f64,
    last_fixed_tick: f64,
    fixed_tick_overflow: f64,
    should_close: bool,
    _event_loop: Option<EventLoop<()>>
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn new_events(&mut self, event_loop: &event_loop::ActiveEventLoop, cause: winit::event::StartCause) {
        self.input.reset();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::KeyboardInput { event, .. } => {
                match event {
                    KeyEvent { physical_key, state: ElementState::Pressed, .. } => {
                        let key_state = self.input.modify_key_state(Key(physical_key));
                        key_state.press = true;
                        key_state.is_down = true;

                    },
                    KeyEvent { physical_key, state: ElementState::Released, .. } => {
                        let key_state = self.input.modify_key_state(Key(physical_key));
                        key_state.release = true;
                        key_state.is_down = false;
                    }
                }
            },
            WindowEvent::MouseInput { button, state, .. } => {
                let button = match button {
                    winit::event::MouseButton::Left => 1,
                    winit::event::MouseButton::Right => 2,
                    winit::event::MouseButton::Middle => 3,
                    winit::event::MouseButton::Back => 4,
                    winit::event::MouseButton::Forward => 5,
                    winit::event::MouseButton::Other(button) => button as u32 + 1, // Winit starts at 0 (I like starting at 1)
                };

                let key_state = self.input.modify_mouse_button_state(button);
                match state {
                    ElementState::Pressed => {
                        key_state.press = true;
                        key_state.is_down = true;
                    },
                    ElementState::Released => {
                        key_state.release = true;
                        key_state.is_down = false;
                    },
                }
            },
            WindowEvent::MouseWheel { delta, .. } => {
                let (x, y) = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => (x as f64, y as f64),
                    winit::event::MouseScrollDelta::PixelDelta(physical_position) => {
                        let PhysicalPosition { x, y } = physical_position;
                        (x / input::LINE_HEIGHT, y / input::LINE_HEIGHT)
                    },
                };

                self.input.add_scroll_delta(x, y);
            },
            WindowEvent::Resized(size) => {
                self.gfx.window_resized();
                let new_aspect = size.width as f32 / size.height as f32;
                if let Some(main_camera) = self.world.get_main_camera() {
                    main_camera.borrow_mut().update_aspect(new_aspect);
                }
            },
            WindowEvent::RedrawRequested => {
                if self.should_close {
                    event_loop.exit();
                    return;
                }

                self.update().unwrap();

                self.window.request_redraw();
            }
            _ => ()
        }
    }
}

impl Engine {
    pub fn new(window_title: &str, width: u32, height: u32, window_mode: WindowMode) -> Result<Engine> {
        let mut event_loop = EventLoop::new()?;
        event_loop.set_control_flow(event_loop::ControlFlow::Poll);

        let window_attributes = WindowAttributes::default()
            .with_title(window_title)
            .with_inner_size(PhysicalSize::new(width, height))
            .with_fullscreen(window_mode.into())
            .with_resizable(true);
        // let window = event_loop.create_window(window_attributes)?;

        #[derive(Default)]
        enum WindowStatus {
            Uninitialized(Box<WindowAttributes>),
            Initialized(Window),
            #[default]
            Null
        }

        struct WindowInitializer(WindowStatus);

        impl ApplicationHandler for WindowInitializer {
            fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
                let WindowStatus::Uninitialized(window_attributes) = std::mem::take(&mut self.0) else { return };
                self.0 = WindowStatus::Initialized(#[allow(clippy::unwrap_used)] event_loop.create_window(*window_attributes).unwrap());
            }
        
            fn window_event(&mut self, _: &event_loop::ActiveEventLoop, _: winit::window::WindowId, _: WindowEvent) {}
        }

        let mut app = WindowInitializer(WindowStatus::Uninitialized(Box::new(window_attributes)));
        
        event_loop.pump_app_events(Some(Duration::ZERO), &mut app);

        let WindowStatus::Initialized(window) = app.0 else { Err("Invalid window state.")? };
        let window = Arc::new(window);

        let world = World::new();

        let mut gfx = Graphics::new(window.clone(), &event_loop)?;
        let sprite_renderer = SpriteRenderer::new();
        let terrain_renderer = TerrainRenderer::new(&mut gfx)?;
        let engine = Engine { window, gfx, world, input: Input::new(), sprite_renderer, terrain_renderer, fixed_tick_duration: 1.0 / 60.0, initialization_time: Instant::now(), last_tick: 0.0, last_fixed_tick: 0.0, fixed_tick_overflow: 0.0, should_close: false, _event_loop: Some(event_loop) };

        // let gfx = Graphics::init(window_title, width, height, window_mode)?;

        // let sprite_renderer = SpriteRenderer::new(&gfx)?;
        // let terrain_renderer = TerrainRenderer::new(&gfx)?;
        
        // Ok(Engine { gfx, world, sprite_renderer, terrain_renderer, fixed_tick_duration: 1.0 / 60.0, error_queue: Vec::new(), input: Input::new(), fixed_input: Input::new() })

        Ok(engine)
    }

    pub fn run(&mut self) -> Result<()> {
        let event_loop = self._event_loop.take().ok_or("No event loop")?;
        // event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        self.window.request_redraw();
        event_loop.run_app(self)?;

        Ok(())
    }

    pub fn get_time(&self) -> f64 {
        (Instant::now() - self.initialization_time).as_secs_f64()
    }

    fn update(&mut self) -> Result<()> {
        self.log_errors();
        self.gfx.validate_pipelines(&self.window)?;

        // self.gfx.poll_events();
        // for msg in self.gfx.flush_messages() {
        //     match msg {
        //         (_, WindowEvent::Key(key, _, Action::Press, _)) => {
        //             let key_state = self.input.modify_key_state(key);
        //             key_state.press = true;
        //             key_state.is_down = true;

        //             let fixed_key_state = self.fixed_input.modify_key_state(key);
        //             fixed_key_state.press = true;
        //             fixed_key_state.is_down = true;
        //         },
        //         (_, WindowEvent::Key(key, _, Action::Release, _)) => {
        //             let key_state = self.input.modify_key_state(key);
        //             key_state.release = true;
        //             key_state.is_down = false;

        //             let fixed_key_state = self.fixed_input.modify_key_state(key);
        //             fixed_key_state.release = true;
        //             fixed_key_state.is_down = false;
        //         },
        //         (_, WindowEvent::MouseButton(button, Action::Press, _)) => {
        //             let key_state = self.input.modify_mouse_button_state(button as u32);
        //             key_state.press = true;
        //             key_state.is_down = true;

        //             let fixed_key_state = self.fixed_input.modify_mouse_button_state(button as u32);
        //             fixed_key_state.press = true;
        //             fixed_key_state.is_down = true;
        //         },
        //         (_, WindowEvent::MouseButton(button, Action::Release, _)) => {
        //             let key_state = self.input.modify_mouse_button_state(button as u32);
        //             key_state.release = true;
        //             key_state.is_down = false;

        //             let fixed_key_state = self.fixed_input.modify_mouse_button_state(button as u32);
        //             fixed_key_state.release = true;
        //             fixed_key_state.is_down = false;
        //         },
        //         (_, WindowEvent::Scroll(x, y)) => {
        //             self.input.add_scroll_delta(x, y);
        //             self.fixed_input.add_scroll_delta(x, y);
        //         }
        //         // (_, WindowEvent::Key(Key::Escape, _, Action::Press, _)) => gfx.set_should_close(true),
        //         // (_, WindowEvent::Key(Key::Space, _, Action::Press, _)) => gfx.set_fullscreen(Monitor::from_primary()),
        //         _ => ()
        //     }
        // }

        // TODO: move clear call to after game tick

        // self.gfx.glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        
        // Game tick
        let current_time = self.get_time();
        World::update(self, (current_time - self.last_tick) as f32)?; // TODO: This is not supposed to crash, catch and log errors
        self.last_tick = current_time;

        // self.input.modify_all_key_states(|key| {
        //     key.press = false;
        //     key.release = false;
        // });
        // self.input.modify_all_mouse_button_states(|button| {
        //     button.press = false;
        //     button.release = false;
        // });
        // self.input.set_scroll_delta(0.0, 0.0);

        let fixed_diff = current_time - self.last_fixed_tick - self.fixed_tick_duration;

        // Add overflow to adjust for errors in timing
        if fixed_diff + self.fixed_tick_overflow >= 0.0 {
            self.fixed_tick_overflow = f64::max(0.0, fixed_diff * 2.0);
            World::fixed_update(self, (current_time - self.last_fixed_tick) as f32)?; // TODO: This is not supposed to crash, catch and log errors
            self.last_fixed_tick = current_time;

            // self.fixed_input.modify_all_key_states(|key| {
            //     key.press = false;
            //     key.release = false;
            // });
            // self.fixed_input.modify_all_mouse_button_states(|button| {
            //     button.press = false;
            //     button.release = false;
            // });
            // self.fixed_input.set_scroll_delta(0.0, 0.0);
        }

        self.log_errors();
        if let Some(camera) = self.world.get_main_camera() {
            let mut camera = camera.borrow_mut();
            self.sprite_renderer.update(&self.gfx, &camera.view_matrix(), &camera.projection_matrix());
            self.terrain_renderer.update(&self.gfx, camera.view_matrix(), camera.projection_matrix(), camera.position());
        }

        for (owner, mut component) in self.world.get_removed_components() {
            component.on_remove(self, owner)?; // TODO: This is not supposed to crash, catch and log errors
        }

        // Render
        // self.gfx.render();

        // Swap front and back buffers
        // self.gfx.swap_buffers();
        self.gfx.draw();

        Ok(())
    }

    fn log_errors(&mut self) {
        // Take erorr queue from error_queue, turn it into a Box and log them
        // let mut errors = Vec::new();
        // std::mem::swap(&mut errors, &mut self.error_queue);
        // let errors = errors.into_boxed_slice();

        // errors.iter().for_each(|error| eprintln!("{}", error))
    }

    pub fn set_should_close(&mut self, should_close: bool) {
        self.should_close = should_close;
    }
}
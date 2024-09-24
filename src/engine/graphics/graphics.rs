use std::{cell::{Ref, RefCell, RefMut}, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, ops::{Deref, Not}, os::raw::c_void};

use glfw::{fail_on_errors, Glfw, Context, PWindow, GlfwReceiver, WindowEvent, Monitor};

use libc::{exit, strlen};
use libffi::high::Closure0;

use crate::engine::{WindowMode, errors::{Result, Error, GraphicsError}};

use super::GLWrapper;

#[derive(Clone, Copy, Default, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Clone, Copy, Default, Debug)]
pub struct RGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

#[derive(Clone, Copy, Default, Debug)]
pub struct UV {
    pub u: f32,
    pub v: f32
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Tangent {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

unsafe fn unsupported_opengl_function(name: String) -> *const c_void {
    let func = Box::new(move || {
        let msg = format!("Unsupported OpenGL function: {}", name);

        native_dialog::MessageDialog::new()
            .set_text(&msg)
            .set_title("OpenGL Error")
            .set_type(native_dialog::MessageType::Error)
            .show_alert().ok();

        exit(1);
    });

    let func: &'static _ = Box::leak(func);
    let callback = Closure0::new(func);

    let &ptr = callback.code_ptr();
    std::mem::forget(callback);

    std::mem::transmute(ptr)
}

fn get_monitor_fingerprint(monitor: &Monitor) -> u64 {
    let mut hasher = DefaultHasher::new();
    monitor.get_workarea().hash(&mut hasher);
    match monitor.get_name() {
        Some(s) => s,
        None => "None".to_owned()
    }.hash(&mut hasher);
    monitor.get_physical_size().hash(&mut hasher);

    hasher.finish()
}

pub struct Graphics {
    gl: GLWrapper,
    glfw: RefCell<Glfw>,
    window: RefCell<PWindow>,
    events: GlfwReceiver<(f64, WindowEvent)>
}

impl Graphics {
    pub fn init(window_title: &str, width: u32, height: u32, window_mode: WindowMode) -> Result<Graphics> {
        let mut glfw = glfw::init(fail_on_errors!())?;
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));

        let (mut window, events) = match window_mode {
            WindowMode::FullScreen(Some(fingerprint)) => {
                glfw.with_connected_monitors(|glfw, monitors| {
                    for m in monitors {
                        let id = get_monitor_fingerprint(m);
                        if id == fingerprint {
                            let window_mode = glfw::WindowMode::FullScreen(m);
                            return glfw.create_window(width, height, window_title, window_mode).ok_or::<Error>(GraphicsError::WindowCreationFailError.into());
                        }
                    }

                    eprintln!("Monitor not found, defaulting to primary monitor!");
                    glfw.create_window(width, height, window_title, glfw::WindowMode::FullScreen(&Monitor::from_primary())).ok_or(GraphicsError::WindowCreationFailError.into())
                })?
            },
            WindowMode::FullScreen(None) => {
                glfw.create_window(width, height, window_title, glfw::WindowMode::FullScreen(&Monitor::from_primary()))
                .ok_or::<Error>(GraphicsError::WindowCreationFailError.into())?
            },
            WindowMode::Windowed => glfw.create_window(width, height, window_title, glfw::WindowMode::Windowed).ok_or::<Error>(GraphicsError::WindowCreationFailError.into())?
        };

        // let (mut window, events) = glfw.create_window(width, height, window_title, window_mode).ok_or(anyhow!(GraphicsError::WindowCreationFailError))?;

        window.make_current();
        println!("version: {:?}", window.get_context_version());
        window.set_key_polling(true);

        let window = RefCell::new(window);

        let gl = GLWrapper::init_gl(|t| {
            // freaking c strings...
            unsafe {
                let len = strlen(t as *const i8);
                let s = std::str::from_utf8_unchecked(std::slice::from_raw_parts(t, len));
                let address = window.borrow_mut().get_proc_address(s);

                if address.is_null() {
                    unsupported_opengl_function(s.to_owned())
                } else {
                    address
                }
            }
        })?;

        let glfw = RefCell::new(glfw);

        Ok(Graphics { gl, glfw, window, events })
    }

    #[cfg(test)]
    pub fn init_unsupported() -> Result<Graphics> {
        let mut glfw = glfw::init(fail_on_errors!())?;

        let (mut window, events) = glfw.create_window(100, 100, "test", glfw::WindowMode::Windowed).ok_or::<Error>(GraphicsError::WindowCreationFailError.into())?;
        
        window.make_current();
        window.set_key_polling(true);

        let window = RefCell::new(window);

        let gl = GLWrapper::init_gl(|t| {
            // freaking c strings...
            unsafe {
                let len = strlen(t as *const i8);
                let s = std::str::from_utf8_unchecked(std::slice::from_raw_parts(t, len));
                unsupported_opengl_function(s.to_owned())
            }
        })?;

        let glfw = RefCell::new(glfw);

        Ok(Graphics { gl, glfw, window, events })
    }

    pub fn get_window_mode(&self) -> WindowMode {
        self.window.borrow().with_window_mode(|mode| {
            match mode {
                glfw::WindowMode::FullScreen(monitor) => WindowMode::FullScreen(Some(get_monitor_fingerprint(monitor))),
                glfw::WindowMode::Windowed => WindowMode::Windowed,
            }
        })
    }

    pub fn swap_buffers(&self) {
        self.window.borrow_mut().swap_buffers();
    }

    pub fn poll_events(&self) {
        self.glfw.borrow_mut().poll_events();
    }

    pub fn get_glfw_time(&self) -> f64 {
        self.glfw.borrow().get_time()
    }

    pub fn flush_messages(&self) -> std::vec::IntoIter<(f64, WindowEvent)> {
        glfw::flush_messages(&self.events).collect::<Vec<(f64, WindowEvent)>>().into_iter()
    }

    pub fn should_close(&self) -> bool {
        self.window.borrow().should_close()
    }

    pub fn set_should_close(&self, value: bool) {
        self.window.borrow_mut().set_should_close(value);
    }

    pub fn set_fullscreen(&self, monitor: Monitor) {
        let mode = monitor.get_video_mode().unwrap();
        self.window.borrow_mut().set_monitor(glfw::WindowMode::FullScreen(&monitor), 0, 0, mode.width, mode.height, None);
    }

    // This will be deleted once glfw is properly wrapped
    pub fn __get_glfw(&self) -> Ref<Glfw> {
        self.glfw.borrow()
    }

    // This will be deleted once glfw is properly wrapped
    pub fn __get_glfw_mut(&self) -> RefMut<Glfw> {
        self.glfw.borrow_mut()
    }

    pub fn is_supported(&self, gl_fn_name: &'static str) -> bool {
        self.window.borrow_mut().get_proc_address(&gl_fn_name).is_null().not()
    }

    // // This will be deleted once window is properly wrapped
    // pub fn __get_window(&self) -> &PWindow {
    //     &self.window
    // }

    // // This will be deleted once window is properly wrapped
    // pub fn __get_window_mut(&mut self) -> &mut PWindow {
    //     &mut self.window
    // }
}

impl Deref for Graphics {
    type Target = GLWrapper;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

#[test]
fn gl_unsupported() {
    let gfx = Graphics::init_unsupported().unwrap();

    gfx.glActiveTexture(gl46::GLenum(0));
}
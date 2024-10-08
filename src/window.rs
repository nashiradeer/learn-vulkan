use std::{
    cell::RefCell,
    fmt::{self},
    ptr::null,
    rc::Rc,
};

use ash::{
    prelude::VkResult,
    vk::{Instance, SurfaceKHR},
};
use glfw::{fail_on_errors, ClientApiHint, Glfw, InitError, PWindow, WindowHint, WindowMode};

#[derive(Debug, Clone)]
pub struct Window(Rc<RefCell<InnerWindow>>);

impl Window {
    pub fn new(
        window_name: &str,
        window_mode: WindowMode,
        height: u32,
        width: u32,
    ) -> Result<Self, WindowError> {
        let mut glfw = glfw::init(glfw::fail_on_errors!()).map_err(WindowError::from)?;

        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        glfw.window_hint(WindowHint::Resizable(false));

        let (window, _events) = glfw
            .create_window(width, height, window_name, window_mode)
            .ok_or(WindowError::CreateWindow)?;

        Ok(Self(Rc::new(RefCell::new(InnerWindow { glfw, window }))))
    }

    pub fn get_required_instance_extensions(&self) -> Option<Vec<String>> {
        self.0.borrow().glfw.get_required_instance_extensions()
    }

    pub fn should_close(&self) -> bool {
        self.0.borrow().window.should_close()
    }

    pub fn poll_events(&self) {
        self.0.borrow_mut().glfw.poll_events();
    }

    pub(crate) unsafe fn create_window_surface(&self, instance: Instance) -> VkResult<SurfaceKHR> {
        let window = &self.0.borrow_mut().window;

        let mut surface = SurfaceKHR::null();

        window
            .create_window_surface(instance, null(), &mut surface)
            .result()?;

        Ok(surface)
    }

    pub fn get_framebuffer_size(&self) -> (i32, i32) {
        self.0.borrow().window.get_framebuffer_size()
    }
}

#[derive(Debug)]
struct InnerWindow {
    glfw: Glfw,
    window: PWindow,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowError {
    Init(InitError),
    CreateWindow,
}

impl From<InitError> for WindowError {
    fn from(value: InitError) -> Self {
        Self::Init(value)
    }
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WindowError::Init(e) => e.fmt(f),
            WindowError::CreateWindow => write!(f, "failed to create GLFW window"),
        }
    }
}

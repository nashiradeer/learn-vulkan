//! GLFW window implementation.

use std::{error, fmt, ptr::null};

use ash::{khr::surface, prelude::*, vk};
use glfw::{fail_on_errors, ClientApiHint, Glfw, GlfwReceiver, InitError, PWindow, WindowHint};

use super::super::{Extensions, Instance};

/// Entry point for GLFW.
pub struct GlfwEntry {
    /// The GLFW context.
    pub glfw: Glfw,
}

impl GlfwEntry {
    /// Initializes the [Glfw] context with [glfw::fail_on_errors!] and sets the required window hints.
    pub fn new() -> Result<Self, InitError> {
        let glfw = glfw::init(glfw::fail_on_errors!())?;

        Ok(Self::with(glfw))
    }

    /// Uses your own [Glfw] context and sets the required window hints.
    pub fn with(mut glfw: Glfw) -> Self {
        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        glfw.window_hint(WindowHint::Resizable(false));

        Self { glfw }
    }

    /// Returns the required Vulkan extensions for GLFW.
    pub fn required_extensions(&self) -> Option<Extensions> {
        self.glfw
            .get_required_instance_extensions()
            .and_then(|v| Extensions::try_from(v).ok())
    }

    /// Creates a new GLFW window.
    pub fn create_window<T: AsRef<Instance>>(
        &mut self,
        instance: T,
        title: &str,
        width: u32,
        height: u32,
        window_mode: glfw::WindowMode,
    ) -> Result<GlfwWindow<T>, GlfwError> {
        let (window, events) = self
            .glfw
            .create_window(width, height, title, window_mode)
            .ok_or(GlfwError::CreateWindow)?;

        GlfwWindow::new(instance, window, events).map_err(GlfwError::from)
    }
}

/// A GLFW window with a Vulkan surface.
pub struct GlfwWindow<T: AsRef<Instance>> {
    /// The GLFW window.
    pub window: PWindow,
    /// The GLFW event receiver.
    pub events: GlfwReceiver<(f64, glfw::WindowEvent)>,
    /// The Vulkan surface.
    pub surface: vk::SurfaceKHR,
    /// The Vulkan surface instance, which is used to create the surface and destroy it.
    pub surface_instance: surface::Instance,
    /// The Vulkan instance.
    pub instance: T,
}

impl<T: AsRef<Instance>> GlfwWindow<T> {
    /// Creates a new surface for the given GLFW window.
    pub fn new(
        instance: T,
        window: PWindow,
        events: GlfwReceiver<(f64, glfw::WindowEvent)>,
    ) -> VkResult<Self> {
        let surface_instance =
            surface::Instance::new(&instance.as_ref().entry, &instance.as_ref().instance);

        let mut surface = vk::SurfaceKHR::null();

        window
            .create_window_surface(instance.as_ref().instance.handle(), null(), &mut surface)
            .result()?;

        Ok(Self {
            window,
            events,
            surface,
            instance,
            surface_instance,
        })
    }

    /// Returns the framebuffer size of the window, converting to the type used in Vulkan.
    pub fn framebuffer_size(&self) -> (u32, u32) {
        let (width, height) = self.window.get_framebuffer_size();
        (width as u32, height as u32)
    }
}

impl<T: AsRef<Instance>> Drop for GlfwWindow<T> {
    fn drop(&mut self) {
        unsafe {
            self.surface_instance.destroy_surface(self.surface, None);
        }
    }
}

/// Error type for GLFW window operations.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum GlfwError {
    /// Error initializing the Vulkan instance.
    Vulkan(vk::Result),

    /// Error creating the GLFW window.
    CreateWindow,
}

impl From<vk::Result> for GlfwError {
    fn from(result: vk::Result) -> Self {
        Self::Vulkan(result)
    }
}

impl fmt::Display for GlfwError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Vulkan(e) => e.fmt(f),
            Self::CreateWindow => write!(f, "failed to create GLFW window"),
        }
    }
}

impl error::Error for GlfwError {}

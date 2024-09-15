use std::rc::Rc;

use ash::{khr::surface, prelude::VkResult, vk::SurfaceKHR};

use crate::{instance::Instance, window::Window};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Surface(Rc<InnerSurface>);

impl Surface {
    pub fn new(instance: Instance, window: Window) -> VkResult<Self> {
        let surface = unsafe { window.create_window_surface(instance.instance().handle()) }?;
        let surface_instance = surface::Instance::new(instance.entry(), instance.instance());

        Ok(Self(Rc::new(InnerSurface {
            instance,
            surface_instance,
            surface,
            window,
        })))
    }

    pub fn surface(&self) -> SurfaceKHR {
        self.0.surface
    }

    pub fn surface_instance(&self) -> &surface::Instance {
        &self.0.surface_instance
    }
}

struct InnerSurface {
    #[allow(dead_code)]
    instance: Instance,

    #[allow(dead_code)]
    window: Window,

    surface_instance: surface::Instance,
    surface: SurfaceKHR,
}

impl Drop for InnerSurface {
    fn drop(&mut self) {
        unsafe {
            self.surface_instance.destroy_surface(self.surface, None);
        }
    }
}

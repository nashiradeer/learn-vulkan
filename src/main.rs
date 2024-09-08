use ash::{vk::make_api_version, Entry};
use debug_layer::DebugLayer;
use instance::Instance;
use logical_device::LogicalDevice;
use physical_device::PhysicalDevice;
use utils::{check_validation_layer_support, print_available_extensions};
use window::Window;

pub const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

pub const ENABLE_VALIDATION_LAYERS: bool = cfg!(debug_assertions);

pub mod debug_layer;
pub mod instance;
pub mod logical_device;
pub mod physical_device;
pub mod utils;
pub mod window;

fn main() {
    let mut app = HelloTriangleApplication::new();
    app.run();
}

pub struct HelloTriangleApplication {
    window: Window,
    instance: Instance,
    debug_layer: Option<DebugLayer>,
    physical_device: PhysicalDevice,
    logical_device: LogicalDevice,
}

impl HelloTriangleApplication {
    pub fn new() -> Self {
        let entry = unsafe { Entry::load().unwrap() };

        if ENABLE_VALIDATION_LAYERS && !check_validation_layer_support(&entry).unwrap() {
            panic!("validation layers requested, but not available!");
        }

        print_available_extensions(&entry);

        let window = Window::new("Vulkan", glfw::WindowMode::Windowed, 800, 600).unwrap();
        let instance = Instance::new(
            entry,
            window.get_required_instance_extensions().unwrap(),
            "Vulkan Tutorial",
            make_api_version(0, 1, 0, 0),
            "No Engine",
            make_api_version(0, 1, 0, 0),
        )
        .unwrap();

        let mut debug_layer = None;
        if ENABLE_VALIDATION_LAYERS {
            debug_layer = Some(DebugLayer::new(instance.clone()).unwrap());
        }

        let physical_device = PhysicalDevice::new(instance.clone()).unwrap();

        let logical_device = LogicalDevice::new(physical_device.clone()).unwrap();

        Self {
            window,
            instance,
            debug_layer,
            physical_device,
            logical_device,
        }
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    pub fn debug_layer(&self) -> &Option<DebugLayer> {
        &self.debug_layer
    }

    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.physical_device
    }

    pub fn logical_device(&self) -> &LogicalDevice {
        &self.logical_device
    }

    pub fn run(&mut self) {
        self.window.run();
    }
}

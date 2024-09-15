use ash::{vk::make_api_version, Entry};
use debug_layer::DebugLayer;
use instance::Instance;
use logical_device::LogicalDevice;
use physical_device::PhysicalDevice;
use surface::Surface;
use utils::{check_validation_layer_support, print_available_extensions};
use window::Window;

pub const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

pub const ENABLE_VALIDATION_LAYERS: bool = cfg!(debug_assertions);

mod debug_layer;
mod instance;
mod logical_device;
mod physical_device;
mod surface;
mod utils;
mod window;

fn main() {
    let mut app = HelloTriangleApplication::new();
    app.run();
}

struct HelloTriangleApplication {
    window: Window,

    #[allow(dead_code)]
    instance: Instance,

    #[allow(dead_code)]
    debug_layer: Option<DebugLayer>,

    #[allow(dead_code)]
    surface: Surface,

    #[allow(dead_code)]
    physical_device: PhysicalDevice,

    #[allow(dead_code)]
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

        let surface = Surface::new(instance.clone(), window.clone()).unwrap();

        let physical_device = PhysicalDevice::new(instance.clone(), &surface).unwrap();

        let logical_device = LogicalDevice::new(physical_device.clone()).unwrap();

        Self {
            window,
            instance,
            debug_layer,
            surface,
            physical_device,
            logical_device,
        }
    }

    pub fn run(&mut self) {
        self.window.run();
    }
}

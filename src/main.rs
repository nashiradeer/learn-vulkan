use ash::{vk::make_api_version, Entry};
use command_buffer::CommandBuffers;
use command_pool::CommandPool;
use debug_layer::DebugLayer;
use framebuffers::Framebuffers;
use graphics_pipeline::GraphicsPipeline;
use image_views::ImageViews;
use instance::Instance;
use logical_device::LogicalDevice;
use physical_device::PhysicalDevice;
use render_pass::RenderPass;
use surface::Surface;
use swapchain::Swapchain;
use utils::{check_validation_layer_support, print_available_extensions};
use window::Window;

const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

const ENABLE_VALIDATION_LAYERS: bool = cfg!(debug_assertions);

const SHADER_VERT: &[u8; 1504] = include_bytes!("../shaders/vert.spv");
const SHADER_FRAG: &[u8; 572] = include_bytes!("../shaders/frag.spv");

mod command_buffer;
mod command_pool;
mod debug_layer;
mod framebuffers;
mod graphics_pipeline;
mod image_views;
mod instance;
mod logical_device;
mod physical_device;
mod render_pass;
mod shader_module;
mod surface;
mod swapchain;
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

    #[allow(dead_code)]
    swapchain: Swapchain,

    #[allow(dead_code)]
    image_views: ImageViews,

    #[allow(dead_code)]
    graphics_pipeline: GraphicsPipeline,

    #[allow(dead_code)]
    framebuffers: Framebuffers,

    #[allow(dead_code)]
    command_pool: CommandPool,

    #[allow(dead_code)]
    command_buffers: CommandBuffers,
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

        let swapchain = Swapchain::new(
            physical_device.clone(),
            logical_device.clone(),
            surface.clone(),
            &window,
        )
        .unwrap();

        let image_views = ImageViews::new(&swapchain, logical_device.clone()).unwrap();

        let render_pass = RenderPass::new(swapchain.clone()).unwrap();

        let graphics_pipeline = GraphicsPipeline::new(render_pass.clone()).unwrap();

        let framebuffers = Framebuffers::new(render_pass.clone(), image_views.clone()).unwrap();

        let command_pool = CommandPool::new(logical_device.clone(), &physical_device).unwrap();

        let command_buffers = CommandBuffers::new(
            command_pool.clone(),
            framebuffers.clone(),
            graphics_pipeline.clone(),
        )
        .unwrap();

        Self {
            window,
            instance,
            debug_layer,
            surface,
            physical_device,
            logical_device,
            swapchain,
            image_views,
            graphics_pipeline,
            framebuffers,
            command_pool,
            command_buffers,
        }
    }

    pub fn run(&mut self) {
        self.window.run();
    }
}

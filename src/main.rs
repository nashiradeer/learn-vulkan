use ash::{
    vk::{make_api_version, PipelineStageFlags, SubmitInfo},
    Entry,
};
use command_buffers::CommandBuffers;
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
use sync_objects::SyncObjects;
use utils::{check_validation_layer_support, print_available_extensions};
use window::Window;

const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

const ENABLE_VALIDATION_LAYERS: bool = cfg!(debug_assertions);

const SHADER_VERT: &[u8; 1504] = include_bytes!("../shaders/vert.spv");
const SHADER_FRAG: &[u8; 572] = include_bytes!("../shaders/frag.spv");

mod command_buffers;
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
mod sync_objects;
mod utils;
mod window;

fn main() {
    let mut app = HelloTriangleApplication::new();
    app.run();
}

struct HelloTriangleApplication {
    window: Window,
    logical_device: LogicalDevice,
    swapchain: Swapchain,
    command_buffers: CommandBuffers,
    sync_objects: SyncObjects,

    #[allow(dead_code)]
    debug_layer: Option<DebugLayer>,
}

impl HelloTriangleApplication {
    pub fn new() -> Self {
        let entry = unsafe { Entry::load().unwrap() };

        if ENABLE_VALIDATION_LAYERS && !check_validation_layer_support(&entry).unwrap() {
            panic!("validation layers requested, but not available!");
        }

        print_available_extensions(&entry);

        let window = Window::new("Vulkan", glfw::WindowMode::Windowed, 600, 800).unwrap();
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

        let sync_objects = SyncObjects::new(logical_device.clone()).unwrap();

        Self {
            window,
            logical_device,
            swapchain,
            command_buffers,
            sync_objects,
            debug_layer,
        }
    }

    pub fn draw_frame(&mut self) {
        self.sync_objects.wait_in_flight_fence().unwrap();

        self.sync_objects.reset_in_flight_fence().unwrap();

        let (image_index, _) = self
            .swapchain
            .acquire_next_image(
                u64::MAX,
                Some(*self.sync_objects.image_available_semaphore()),
                None,
            )
            .unwrap();

        self.command_buffers.reset().unwrap();

        self.command_buffers
            .record(0, image_index.try_into().unwrap(), 0, 0, 0)
            .unwrap();

        let wait_semaphores = [*self.sync_objects.image_available_semaphore()];
        let signal_semaphores = [*self.sync_objects.render_finished_semaphore()];

        let wait_stages = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let submit_info = SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(self.command_buffers.command_buffers())
            .signal_semaphores(&signal_semaphores);

        let submit_infos = [submit_info];

        unsafe {
            self.logical_device
                .device()
                .queue_submit(
                    *self.logical_device.queue(),
                    &submit_infos,
                    *self.sync_objects.in_flight_fence(),
                )
                .unwrap();
        }

        let image_indices = [image_index.try_into().unwrap()];

        self.swapchain
            .queue_present(&signal_semaphores, &image_indices)
            .unwrap();
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            self.window.poll_events();
            self.draw_frame();
        }

        self.logical_device.wait_idle().unwrap();
    }
}

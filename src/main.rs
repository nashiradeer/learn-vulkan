use std::rc::Rc;

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
const MAX_FRAMES_IN_FLIGHT: usize = 2;

mod api2;
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

struct HelloTriangleApplication2 {
    glfw_entry: api2::GlfwEntry,
    window: api2::GlfwWindow<Rc<api2::Instance>>,
    instance: Rc<api2::Instance>,
}

impl HelloTriangleApplication2 {
    pub fn new() -> Self {
        let mut glfw_entry = api2::GlfwEntry::new().unwrap();

        let instance_builder = api2::InstanceBuilder::default()
            .application_name("Hello Triangle")
            .engine_name("No Engine")
            .enable_debug_layer(true)
            .extensions(glfw_entry.required_extensions().unwrap());

        println!("Available extensions:");
        instance_builder
            .available_extensions()
            .unwrap()
            .iter()
            .for_each(|v| {
                if let Ok(extension) = v.to_str() {
                    println!("    {}", extension);
                }
            });

        println!("Available layers:");
        instance_builder
            .available_layers()
            .unwrap()
            .iter()
            .for_each(|v| {
                if let Ok(layer) = v.to_str() {
                    println!("    {}", layer);
                }
            });

        let instance = Rc::new(instance_builder.build().unwrap());

        let window = glfw_entry
            .create_window(
                instance.clone(),
                "Hello Triangle",
                800,
                600,
                glfw::WindowMode::Windowed,
            )
            .unwrap();

        Self {
            glfw_entry,
            window,
            instance,
        }
    }
}

struct HelloTriangleApplication {
    window: Window,
    logical_device: LogicalDevice,
    swapchain: Swapchain,
    command_buffers: CommandBuffers,
    sync_objects: SyncObjects,
    current_frame: usize,

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

        let sync_objects = SyncObjects::new(logical_device.clone(), MAX_FRAMES_IN_FLIGHT).unwrap();

        Self {
            current_frame: 0,
            window,
            logical_device,
            swapchain,
            command_buffers,
            sync_objects,
            debug_layer,
        }
    }

    pub fn draw_frame(&mut self) {
        self.sync_objects
            .wait_in_flight_fence(self.current_frame)
            .unwrap();

        self.sync_objects
            .reset_in_flight_fence(self.current_frame)
            .unwrap();

        let (image_index, _) = self
            .swapchain
            .acquire_next_image(
                u64::MAX,
                Some(
                    *self
                        .sync_objects
                        .image_available_semaphore(self.current_frame),
                ),
                None,
            )
            .unwrap();

        self.command_buffers.reset().unwrap();

        self.command_buffers
            .record(0, image_index.try_into().unwrap(), 0, 0, 0)
            .unwrap();

        let wait_semaphores = [*self
            .sync_objects
            .image_available_semaphore(self.current_frame)];
        let signal_semaphores = [*self
            .sync_objects
            .render_finished_semaphore(self.current_frame)];

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
                    *self.sync_objects.in_flight_fence(self.current_frame),
                )
                .unwrap();
        }

        let image_indices = [image_index.try_into().unwrap()];

        self.swapchain
            .queue_present(&signal_semaphores, &image_indices)
            .unwrap();

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            self.window.poll_events();
            self.draw_frame();
        }

        self.logical_device.wait_idle().unwrap();
    }
}

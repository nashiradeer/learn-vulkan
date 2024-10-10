use ash::vk;

pub struct Swapchain2 {
    pub image_views: Vec<vk::ImageView>,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub swapchain: vk::SwapchainKHR,
    pub render_pass: vk::RenderPass,
}

impl Swapchain2 {
    pub fn new() -> Self {
        Self {
            swapchain,
            image_views,
            framebuffers,
            render_pass,
        }
    }
}

use std::rc::Rc;

use ash::{
    prelude::VkResult,
    vk::{Framebuffer, FramebufferCreateInfo},
};

use crate::{image_views::ImageViews, render_pass::RenderPass};

#[derive(Clone)]
pub struct Framebuffers(Rc<InnerFramebuffers>);

impl Framebuffers {
    pub fn new(render_pass: RenderPass, image_views: ImageViews) -> VkResult<Self> {
        let mut framebuffers = Vec::with_capacity(image_views.image_views().len());

        for image_view in image_views.image_views() {
            let image_views = [*image_view];

            let framebuffer_create_info = FramebufferCreateInfo::default()
                .render_pass(*render_pass.render_pass())
                .attachments(&image_views)
                .width(render_pass.swapchain().extent().width)
                .height(render_pass.swapchain().extent().height)
                .layers(1);

            let framebuffer = unsafe {
                render_pass
                    .swapchain()
                    .device()
                    .device()
                    .create_framebuffer(&framebuffer_create_info, None)?
            };

            framebuffers.push(framebuffer);
        }

        Ok(Self(Rc::new(InnerFramebuffers {
            framebuffers,
            render_pass,
            image_views,
        })))
    }

    pub fn framebuffers(&self) -> &[Framebuffer] {
        &self.0.framebuffers
    }

    pub fn render_pass(&self) -> &RenderPass {
        &self.0.render_pass
    }
}

struct InnerFramebuffers {
    framebuffers: Vec<Framebuffer>,

    #[allow(dead_code)]
    render_pass: RenderPass,

    #[allow(dead_code)]
    image_views: ImageViews,
}

impl Drop for InnerFramebuffers {
    fn drop(&mut self) {
        unsafe {
            for framebuffer in self.framebuffers.iter() {
                self.render_pass
                    .swapchain()
                    .device()
                    .device()
                    .destroy_framebuffer(*framebuffer, None);
            }
        }
    }
}

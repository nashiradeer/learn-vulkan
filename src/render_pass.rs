use std::rc::Rc;

use ash::{
    prelude::VkResult,
    vk::{
        self, AccessFlags, AttachmentDescription, AttachmentLoadOp, AttachmentReference,
        AttachmentStoreOp, ImageLayout, PipelineBindPoint, PipelineStageFlags,
        RenderPassCreateInfo, SampleCountFlags, SubpassDependency, SubpassDescription,
        SUBPASS_EXTERNAL,
    },
};

use crate::swapchain::Swapchain;

#[derive(Clone)]
pub struct RenderPass(Rc<InnerRenderPass>);

impl RenderPass {
    pub fn new(swapchain: Swapchain) -> VkResult<Self> {
        let attachment_description = [AttachmentDescription::default()
            .format(swapchain.format().format)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::PRESENT_SRC_KHR)];

        let attachment_reference = [AttachmentReference::default()
            .attachment(0)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

        let subpass = [SubpassDescription::default()
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .color_attachments(&attachment_reference)];

        let dependencies = [SubpassDependency::default()
            .src_subpass(SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(Default::default())
            .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(AccessFlags::COLOR_ATTACHMENT_WRITE)];

        let render_pass_info = RenderPassCreateInfo::default()
            .attachments(&attachment_description)
            .subpasses(&subpass)
            .dependencies(&dependencies);

        let render_pass = unsafe {
            swapchain
                .device()
                .device()
                .create_render_pass(&render_pass_info, None)
        }?;

        Ok(Self(Rc::new(InnerRenderPass {
            render_pass,
            swapchain,
        })))
    }

    pub fn render_pass(&self) -> &vk::RenderPass {
        &self.0.render_pass
    }

    pub fn swapchain(&self) -> &Swapchain {
        &self.0.swapchain
    }
}

struct InnerRenderPass {
    render_pass: vk::RenderPass,

    swapchain: Swapchain,
}

impl Drop for InnerRenderPass {
    fn drop(&mut self) {
        unsafe {
            self.swapchain
                .device()
                .device()
                .destroy_render_pass(self.render_pass, None);
        }
    }
}

use std::rc::Rc;

use ash::{
    prelude::VkResult,
    vk::{
        ClearColorValue, ClearValue, CommandBuffer, CommandBufferAllocateInfo,
        CommandBufferBeginInfo, CommandBufferLevel, Offset2D, PipelineBindPoint, Rect2D,
        RenderPassBeginInfo, SubpassContents,
    },
};

use crate::{
    command_pool::CommandPool, framebuffers::Framebuffers, graphics_pipeline::GraphicsPipeline,
};

#[derive(Clone)]
pub struct CommandBuffers(Rc<InnerCommandBuffers>);

impl CommandBuffers {
    pub fn new(
        command_pool: CommandPool,
        framebuffers: Framebuffers,
        graphics_pipeline: GraphicsPipeline,
    ) -> VkResult<Self> {
        let command_buffer_alloc_info = CommandBufferAllocateInfo::default()
            .command_pool(*command_pool.command_pool())
            .level(CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let command_buffers = unsafe {
            command_pool
                .logical_device()
                .device()
                .allocate_command_buffers(&command_buffer_alloc_info)?
        };

        Ok(Self(Rc::new(InnerCommandBuffers {
            command_buffers,
            command_pool,
            framebuffers,
            graphics_pipeline,
        })))
    }

    pub fn command_buffers(&self) -> &[CommandBuffer] {
        &self.0.command_buffers
    }

    pub fn reset(&self) -> VkResult<()> {
        let command_buffer = self.0.command_buffers[0];

        let command_buffer_reset_flags = Default::default();

        unsafe {
            self.0
                .command_pool
                .logical_device()
                .device()
                .reset_command_buffer(command_buffer, command_buffer_reset_flags)
        }
    }

    pub fn record(
        &self,
        command_buffer_index: usize,
        image_index: usize,
        pipeline_index: usize,
        viewport_index: u32,
        scissor_index: u32,
    ) -> VkResult<()> {
        let command_buffer_begin_info = CommandBufferBeginInfo::default();

        let command_buffer = self.0.command_buffers[command_buffer_index];

        unsafe {
            self.0
                .command_pool
                .logical_device()
                .device()
                .begin_command_buffer(command_buffer, &command_buffer_begin_info)?;
        }

        let swapchain_extend = self.0.framebuffers.render_pass().swapchain().extent();

        let clear_values = [ClearValue {
            color: ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        }];

        let render_pass_info = RenderPassBeginInfo::default()
            .render_pass(*self.0.framebuffers.render_pass().render_pass())
            .framebuffer(self.0.framebuffers.framebuffers()[image_index])
            .render_area(
                Rect2D::default()
                    .extent(swapchain_extend)
                    .offset(Offset2D::default()),
            )
            .clear_values(&clear_values);

        unsafe {
            self.0
                .command_pool
                .logical_device()
                .device()
                .cmd_begin_render_pass(command_buffer, &render_pass_info, SubpassContents::INLINE);

            self.0
                .command_pool
                .logical_device()
                .device()
                .cmd_set_viewport(
                    command_buffer,
                    viewport_index,
                    self.0.graphics_pipeline.viewports(),
                );

            self.0
                .command_pool
                .logical_device()
                .device()
                .cmd_set_scissor(
                    command_buffer,
                    scissor_index,
                    self.0.graphics_pipeline.scissors(),
                );

            self.0
                .command_pool
                .logical_device()
                .device()
                .cmd_bind_pipeline(
                    command_buffer,
                    PipelineBindPoint::GRAPHICS,
                    self.0.graphics_pipeline.pipeline()[pipeline_index],
                );

            self.0
                .command_pool
                .logical_device()
                .device()
                .cmd_draw(command_buffer, 3, 1, 0, 0);

            self.0
                .command_pool
                .logical_device()
                .device()
                .cmd_end_render_pass(command_buffer);

            self.0
                .command_pool
                .logical_device()
                .device()
                .end_command_buffer(command_buffer)?;
        }

        Ok(())
    }
}

struct InnerCommandBuffers {
    command_buffers: Vec<CommandBuffer>,
    framebuffers: Framebuffers,
    graphics_pipeline: GraphicsPipeline,
    command_pool: CommandPool,
}

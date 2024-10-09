use std::{ffi::CString, io::Cursor, rc::Rc};

use ash::{
    prelude::VkResult,
    util::read_spv,
    vk::{
        ColorComponentFlags, CullModeFlags, DynamicState, FrontFace, GraphicsPipelineCreateInfo,
        Offset2D, Pipeline, PipelineCache, PipelineColorBlendAttachmentState,
        PipelineColorBlendStateCreateInfo, PipelineDynamicStateCreateInfo,
        PipelineInputAssemblyStateCreateInfo, PipelineLayout, PipelineLayoutCreateInfo,
        PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
        PipelineShaderStageCreateInfo, PipelineVertexInputStateCreateInfo,
        PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, Rect2D, SampleCountFlags,
        ShaderStageFlags, Viewport,
    },
};

use crate::{render_pass::RenderPass, shader_module::ShaderModule, SHADER_FRAG, SHADER_VERT};

#[derive(Clone)]
pub struct GraphicsPipeline(Rc<InnerGraphicsPipeline>);

impl GraphicsPipeline {
    pub fn new(render_pass: RenderPass) -> VkResult<Self> {
        let shader_modules = [
            ShaderModule::new(
                render_pass.swapchain().device().clone(),
                &read_spv(&mut Cursor::new(SHADER_VERT)).unwrap(),
            )
            .unwrap(),
            ShaderModule::new(
                render_pass.swapchain().device().clone(),
                &read_spv(&mut Cursor::new(SHADER_FRAG)).unwrap(),
            )
            .unwrap(),
        ];

        let main_function_name = CString::new("main").unwrap();

        let pipeline_shader_info = [
            PipelineShaderStageCreateInfo::default()
                .stage(ShaderStageFlags::VERTEX)
                .module(*shader_modules[0].shader_module())
                .name(&main_function_name),
            PipelineShaderStageCreateInfo::default()
                .stage(ShaderStageFlags::FRAGMENT)
                .module(*shader_modules[1].shader_module())
                .name(&main_function_name),
        ];

        let dynamic_stages = [DynamicState::VIEWPORT, DynamicState::SCISSOR];

        let dynamic_state_info =
            PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_stages);

        let vertex_input_info = PipelineVertexInputStateCreateInfo::default();

        let input_assembly_info = PipelineInputAssemblyStateCreateInfo::default()
            .topology(PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewports = vec![Viewport::default()
            .x(0.0)
            .y(0.0)
            .height(render_pass.swapchain().extent().height as f32)
            .width(render_pass.swapchain().extent().width as f32)
            .min_depth(0.0)
            .max_depth(1.0)];

        let scissors = vec![Rect2D::default()
            .extent(render_pass.swapchain().extent())
            .offset(Offset2D::default().x(0).y(0))];

        let viewport_info = PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasterizer_info = PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(CullModeFlags::BACK)
            .front_face(FrontFace::CLOCKWISE)
            .depth_bias_enable(false);

        let multisample_info = PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(SampleCountFlags::TYPE_1);

        let color_blend_attachments = [PipelineColorBlendAttachmentState::default()
            .color_write_mask(ColorComponentFlags::RGBA)
            .blend_enable(false)];

        let color_blend_info = PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .attachments(&color_blend_attachments);

        let pipeline_layout_info = PipelineLayoutCreateInfo::default();

        let pipeline_layout = unsafe {
            render_pass
                .swapchain()
                .device()
                .device()
                .create_pipeline_layout(&pipeline_layout_info, None)?
        };

        let pipeline_info = [GraphicsPipelineCreateInfo::default()
            .stages(&pipeline_shader_info)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly_info)
            .viewport_state(&viewport_info)
            .rasterization_state(&rasterizer_info)
            .multisample_state(&multisample_info)
            .color_blend_state(&color_blend_info)
            .layout(pipeline_layout)
            .dynamic_state(&dynamic_state_info)
            .render_pass(*render_pass.render_pass())];

        let pipeline = unsafe {
            render_pass
                .swapchain()
                .device()
                .device()
                .create_graphics_pipelines(PipelineCache::null(), &pipeline_info, None)
                .map_err(|(_, err)| err)?
        };

        Ok(GraphicsPipeline(Rc::new(InnerGraphicsPipeline {
            viewports,
            scissors,
            pipeline_layout,
            pipeline,
            render_pass,
        })))
    }

    pub fn pipeline(&self) -> &[Pipeline] {
        &self.0.pipeline
    }

    pub fn viewports(&self) -> &[Viewport] {
        &self.0.viewports
    }

    pub fn scissors(&self) -> &[Rect2D] {
        &self.0.scissors
    }
}

struct InnerGraphicsPipeline {
    pipeline_layout: PipelineLayout,
    pipeline: Vec<Pipeline>,
    viewports: Vec<Viewport>,
    scissors: Vec<Rect2D>,

    #[allow(dead_code)]
    render_pass: RenderPass,
}

impl Drop for InnerGraphicsPipeline {
    fn drop(&mut self) {
        unsafe {
            for pipeline in self.pipeline.iter() {
                self.render_pass
                    .swapchain()
                    .device()
                    .device()
                    .destroy_pipeline(*pipeline, None);
            }

            self.render_pass
                .swapchain()
                .device()
                .device()
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}

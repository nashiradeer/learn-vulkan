use std::rc::Rc;

use ash::{
    prelude::VkResult,
    vk::{
        ComponentMapping, ComponentSwizzle, Image, ImageAspectFlags, ImageSubresourceRange,
        ImageView, ImageViewCreateInfo, ImageViewType, SurfaceFormatKHR,
    },
};

use crate::{logical_device::LogicalDevice, swapchain::Swapchain};

#[derive(Clone)]
pub struct ImageViews(Rc<InnerImageViews>);

impl ImageViews {
    pub fn new(swapchain: &Swapchain, logical_device: LogicalDevice) -> VkResult<Self> {
        let mut image_views = Vec::with_capacity(swapchain.images().len());

        for image in swapchain.images() {
            let image_view_create_info = image_view_create_info(image, swapchain.format());
            let image_view = unsafe {
                logical_device
                    .device()
                    .create_image_view(&image_view_create_info, None)?
            };

            image_views.push(image_view);
        }

        Ok(ImageViews(Rc::new(InnerImageViews {
            image_views,
            logical_device,
        })))
    }

    pub fn image_views(&self) -> &[ImageView] {
        &self.0.image_views
    }
}

struct InnerImageViews {
    image_views: Vec<ImageView>,
    logical_device: LogicalDevice,
}

impl Drop for InnerImageViews {
    fn drop(&mut self) {
        unsafe {
            for image_view in self.image_views.iter() {
                self.logical_device
                    .device()
                    .destroy_image_view(*image_view, None);
            }
        }
    }
}

fn image_view_create_info(image: &Image, format: SurfaceFormatKHR) -> ImageViewCreateInfo {
    ImageViewCreateInfo::default()
        .image(*image)
        .view_type(ImageViewType::TYPE_2D)
        .format(format.format)
        .components(ComponentMapping {
            r: ComponentSwizzle::IDENTITY,
            g: ComponentSwizzle::IDENTITY,
            b: ComponentSwizzle::IDENTITY,
            a: ComponentSwizzle::IDENTITY,
        })
        .subresource_range(ImageSubresourceRange {
            aspect_mask: ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        })
}

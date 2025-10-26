use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::vulkan::bindings::{
    VkResult, VkResult_VK_ERROR_INCOMPATIBLE_DRIVER, VkResult_VK_SUCCESS,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(VkResult);

impl From<VkResult> for Error {
    fn from(result: VkResult) -> Self {
        Error(result)
    }
}

impl Display for Error {
    #[allow(non_snake_case)]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.0 {
            VkResult_VK_SUCCESS => write!(f, "VkResult::VK_SUCCESS"),
            VkResult_VK_ERROR_INCOMPATIBLE_DRIVER => {
                write!(f, "VkResult::VK_ERROR_INCOMPATIBLE_DRIVER")
            }
            err => write!(f, "Unknown VkResult error code: {}", err),
        }
    }
}

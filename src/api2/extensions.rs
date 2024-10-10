//! Vulkan extensions and layers.

use std::{
    borrow::{Borrow, BorrowMut},
    error,
    ffi::{c_char, CString, FromBytesUntilNulError, NulError},
    fmt,
    ops::{Deref, DerefMut},
};

use ash::vk;

/// A collection of Vulkan extensions or layers.
#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Extensions {
    /// Internal buffer of extensions or layers in an intermediary type.
    pub extensions: Vec<CString>,
}

impl Extensions {
    /// Create a new empty collection of extensions or layers.
    pub fn new() -> Self {
        Self {
            extensions: Vec::new(),
        }
    }

    /// Create a new collection of extensions or layers with a specific capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            extensions: Vec::with_capacity(capacity),
        }
    }

    /// Create a new collection of extensions or layers of [c_char] pointers that're references to this collection's internal buffer.
    pub fn as_vec_ptr(&self) -> Vec<*const c_char> {
        self.extensions.iter().map(|s| s.as_ptr()).collect()
    }

    /// Create a new collection of extensions or layers of &[str] that're references to this collection's internal buffer.
    pub fn as_vec_str(&self) -> Vec<&str> {
        self.extensions
            .iter()
            .map(|s| s.to_str())
            .flatten()
            .collect()
    }
}

impl AsRef<Vec<CString>> for Extensions {
    fn as_ref(&self) -> &Vec<CString> {
        &self.extensions
    }
}

impl AsMut<Vec<CString>> for Extensions {
    fn as_mut(&mut self) -> &mut Vec<CString> {
        &mut self.extensions
    }
}

impl Deref for Extensions {
    type Target = Vec<CString>;

    fn deref(&self) -> &Self::Target {
        &self.extensions
    }
}

impl DerefMut for Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensions
    }
}

impl Borrow<Vec<CString>> for Extensions {
    fn borrow(&self) -> &Vec<CString> {
        &self.extensions
    }
}

impl BorrowMut<Vec<CString>> for Extensions {
    fn borrow_mut(&mut self) -> &mut Vec<CString> {
        &mut self.extensions
    }
}

impl From<Vec<CString>> for Extensions {
    fn from(value: Vec<CString>) -> Self {
        Self { extensions: value }
    }
}

impl TryFrom<Vec<String>> for Extensions {
    type Error = NulError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        Ok(Self {
            extensions: value
                .into_iter()
                .map(|s| CString::new(s))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<Vec<vk::LayerProperties>> for Extensions {
    type Error = PropertiesConversionError;

    fn try_from(value: Vec<vk::LayerProperties>) -> Result<Self, Self::Error> {
        Ok(Self {
            extensions: value
                .into_iter()
                .map(|s| {
                    CString::new(
                        s.layer_name_as_c_str()
                            .map_err(PropertiesConversionError::from)?
                            .to_bytes(),
                    )
                    .map_err(PropertiesConversionError::from)
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<Vec<vk::ExtensionProperties>> for Extensions {
    type Error = PropertiesConversionError;

    fn try_from(value: Vec<vk::ExtensionProperties>) -> Result<Self, Self::Error> {
        Ok(Self {
            extensions: value
                .into_iter()
                .map(|s| {
                    CString::new(
                        s.extension_name_as_c_str()
                            .map_err(PropertiesConversionError::from)?
                            .to_bytes(),
                    )
                    .map_err(PropertiesConversionError::from)
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PropertiesConversionError {
    FromBytesUntilNul(FromBytesUntilNulError),
    Nul(NulError),
}

impl From<FromBytesUntilNulError> for PropertiesConversionError {
    fn from(error: FromBytesUntilNulError) -> Self {
        Self::FromBytesUntilNul(error)
    }
}

impl From<NulError> for PropertiesConversionError {
    fn from(error: NulError) -> Self {
        Self::Nul(error)
    }
}

impl fmt::Display for PropertiesConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FromBytesUntilNul(e) => e.fmt(f),
            Self::Nul(e) => e.fmt(f),
        }
    }
}

impl error::Error for PropertiesConversionError {}

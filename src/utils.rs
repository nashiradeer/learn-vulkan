use std::ffi::CString;

use ash::{prelude::VkResult, Entry};

use crate::VALIDATION_LAYERS;

pub fn print_available_extensions(entry: &Entry) {
    let extensions = unsafe { entry.enumerate_instance_extension_properties(None) };

    if let Ok(extensions) = extensions {
        println!("available extensions:");
        for extension in extensions {
            println!(
                "  {}",
                extension
                    .extension_name_as_c_str()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
        }
    }
}

pub fn check_validation_layer_support(entry: &Entry) -> VkResult<bool> {
    let layers = unsafe { entry.enumerate_instance_layer_properties() }?;

    for required_layer in VALIDATION_LAYERS {
        let mut layer_found = false;

        for layer in layers.iter() {
            if layer.layer_name_as_c_str().unwrap().to_str().unwrap() == required_layer {
                layer_found = true;
                break;
            }
        }

        if !layer_found {
            return Ok(false);
        }
    }

    Ok(true)
}

pub fn to_vec_cstring<V: Into<Vec<u8>>, I: IntoIterator<Item = V>>(iter: I) -> Vec<CString> {
    iter.into_iter().map(|s| CString::new(s).unwrap()).collect()
}

pub fn to_vec_pointer(vector: &Vec<CString>) -> Vec<*const i8> {
    vector.iter().map(|s| s.as_ptr()).collect()
}

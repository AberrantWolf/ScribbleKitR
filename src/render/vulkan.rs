use std::fmt::Debug;
use std::{ffi::CString, ptr};

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;
use ash::vk;

use crate::render::Renderer;

#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;
#[cfg(target_os = "macos")]
use ash::extensions::mvk::MacOSSurface;

fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

pub struct VulkanRenderer {
    instance: ash::Instance,
}

impl VulkanRenderer {}

impl Debug for VulkanRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VulkanRenderer")
            // .field("instance", &self.instance.type_id())
            .finish()
    }
}

impl Renderer for VulkanRenderer {
    fn create(name: &str) -> Self {
        let app_name_c = CString::new(name).unwrap();
        let engine_name_c = CString::new("Scribble Vulkan Engine").unwrap();

        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name_c.as_ptr(),
            application_version: 0u32,
            p_engine_name: engine_name_c.as_ptr(),
            engine_version: 0u32,
            api_version: vk::API_VERSION_1_3,
        };

        let extension_names = required_extension_names();

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            enabled_layer_count: 0,
            pp_enabled_layer_names: ptr::null(),
            enabled_extension_count: extension_names.len() as u32,
            pp_enabled_extension_names: extension_names.as_ptr(),
        };

        let entry = unsafe { ash::Entry::load().unwrap() };
        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create Vulkan instance")
        };

        VulkanRenderer { instance }
    }

    fn render(&self) {
        // TODO
    }
}

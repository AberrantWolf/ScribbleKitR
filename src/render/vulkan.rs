use std::ffi::CStr;
use std::fmt::Debug;
use std::os::raw::{c_char, c_void};
use std::{ffi::CString, ptr};

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;
use ash::vk;

use crate::render::Renderer;

// Windows-specific setup
#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;
#[cfg(target_os = "windows")]
fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

// Mac-specific setup
#[cfg(target_os = "macos")]
use ash::extensions::mvk::MacOSSurface;
#[cfg(target_os = "macos")]
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        MacOSSurface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

// Linux-specific setup
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

fn vk_to_str(raw_string_array: &[c_char]) -> String {
    let raw_string = unsafe {
        let pointer = raw_string_array.as_ptr();
        CStr::from_ptr(pointer)
    };

    raw_string
        .to_str()
        .expect("Failed to convert vulkan raw string.")
        .to_owned()
}

#[cfg(feature = "validation")]
const VALIDATION_ON: bool = true;

#[cfg(not(feature = "validation"))]
const VALIDATION_ON: bool = true;

#[cfg(feature = "validation")]
const VALIDATION_LAYERS: [&'static str; 1] = ["VK_LAYER_KHRONOS_validation"];

#[cfg(not(feature = "validation"))]
const VALIDATION_LAYERS: [&'static str; 0] = [];

// Borrowed from https://github.com/unknownue/vulkan-tutorial-rust/blob/master/src/tutorials/02_validation_layers.rs
unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    let severity = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[Error]",
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[Info]",
        _ => "[Unknown]",
    };
    let types = match message_type {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
        _ => "[Unknown]",
    };
    let message = CStr::from_ptr((*p_callback_data).p_message);
    println!("[Debug]{}{}{:?}", severity, types, message);

    vk::FALSE
}

// Borrowed from https://github.com/unknownue/vulkan-tutorial-rust/blob/master/src/tutorials/02_validation_layers.rs
fn populate_debug_messenger_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        p_next: ptr::null(),
        flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
            // vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
            // vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        pfn_user_callback: Some(vulkan_debug_utils_callback),
        p_user_data: ptr::null_mut(),
    }
}

pub struct VulkanRenderer {
    _api_entry: ash::Entry,
    instance: ash::Instance,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

impl VulkanRenderer {
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    fn check_validation_layer_support(entry: &ash::Entry) -> bool {
        let layer_properties = entry
            .enumerate_instance_layer_properties()
            .expect("Unable to enumerate Vulkan layer properties.");

        if layer_properties.len() < 1 {
            eprintln!("No layers available");
            return false;
        }

        for required_layer_name in VALIDATION_LAYERS.iter() {
            let mut was_found = false;

            for layer_property in layer_properties.iter() {
                let test_layer_name = vk_to_str(&layer_property.layer_name);
                if (*required_layer_name) == test_layer_name {
                    was_found = true;
                    break;
                }
            }

            if !was_found {
                return false;
            }
        }

        true
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    fn setup_debug_utils(
        entry: &ash::Entry,
        instance: &ash::Instance,
    ) -> (ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT) {
        let debug_utils_loader = ash::extensions::ext::DebugUtils::new(entry, instance);

        if cfg!(feature = "validation") {
            (debug_utils_loader, ash::vk::DebugUtilsMessengerEXT::null())
        } else {
            let messenger_ci = populate_debug_messenger_create_info();

            let utils_messenger = unsafe {
                debug_utils_loader
                    .create_debug_utils_messenger(&messenger_ci, None)
                    .expect("Debug Utils Callback")
            };

            (debug_utils_loader, utils_messenger)
        }
    }

    /// Create the app info struct required by Vulkan initialization.
    fn init_app_info(name: &str) -> vk::ApplicationInfo {
        let app_name_c = CString::new(name).unwrap();
        let engine_name_c = CString::new("Scribble Vulkan Engine").unwrap();

        vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name_c.as_ptr(),
            application_version: 0u32,
            p_engine_name: engine_name_c.as_ptr(),
            engine_version: 0u32,
            api_version: vk::API_VERSION_1_3,
        }
    }

    /// Create the InstanceCreateInfo struct required by Vulkan initialization.
    fn init_create_info(app_info: vk::ApplicationInfo) -> vk::InstanceCreateInfo {
        let extension_names = required_extension_names();

        let debug_utils_create_info = populate_debug_messenger_create_info();

        let requred_validation_layer_raw_names: Vec<CString> = VALIDATION_LAYERS
            .iter()
            .map(|layer_name| CString::new(*layer_name).unwrap())
            .collect();
        let enable_layer_names: Vec<*const i8> = requred_validation_layer_raw_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let p_next = if cfg!(feature = "validation") {
            &debug_utils_create_info as *const vk::DebugUtilsMessengerCreateInfoEXT as *const c_void
        } else {
            ptr::null()
        };

        let pp_enabled_layer_names = if cfg!(feature = "validation") {
            enable_layer_names.as_ptr()
        } else {
            ptr::null()
        };

        let enabled_layer_count = if cfg!(feature = "validation") {
            enable_layer_names.len() as u32
        } else {
            0
        };

        vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next,
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            enabled_layer_count,
            pp_enabled_layer_names,
            enabled_extension_count: extension_names.len() as u32,
            pp_enabled_extension_names: extension_names.as_ptr(),
        }
    }
}

impl Debug for VulkanRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VulkanRenderer")
            // .field("instance", &self.instance.type_id())
            .finish()
    }
}

impl Renderer for VulkanRenderer {
    fn create(name: &str) -> Self {
        // Load the Vulkan API (dynamically) -- this needs to be kept alive while Vulkan is in use
        let entry = unsafe { ash::Entry::load().unwrap() };

        // If we need validation layers and can't load them, screw it, give up I guess... (for now)
        // TODO: Maybe don't panic...?
        if VALIDATION_ON && !VulkanRenderer::check_validation_layer_support(&entry) {
            panic!("Validation layers required but were not found.");
        }

        // Initialize info structs used to create the instance
        let app_info = VulkanRenderer::init_app_info(name);
        let create_info = VulkanRenderer::init_create_info(app_info);

        // Create the Vulkan insance we're going to use
        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create Vulkan instance")
        };

        // Load the debug utils from Vulkan -- these need to be kept alive while Vulkan is in use
        let (debug_utils_loader, debug_messenger) =
            VulkanRenderer::setup_debug_utils(&entry, &instance);

        // Here we go, constructing the usable(?) renderer struct
        VulkanRenderer {
            _api_entry: entry,
            instance,
            debug_utils_loader,
            debug_messenger,
        }
    }

    fn render(&self) {
        // TODO
    }
}

impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        unsafe {
            // Unload validation layers if they've been included
            if cfg!(feature = "validation") {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_messenger, None);
            }

            // Destroy the Vulkan instance we've been using
            self.instance.destroy_instance(None);
        }
    }
}

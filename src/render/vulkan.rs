use std::borrow::Cow;
use std::ffi::{self, CString};
use std::fmt::Debug;
use std::os::raw::c_char;

use ash::ext::debug_utils;
use ash::vk;
use winit::raw_window_handle::RawDisplayHandle;

use crate::render::Renderer;

// Borrowed from https://github.com/unknownue/vulkan-tutorial-rust/blob/master/src/tutorials/02_validation_layers.rs
unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
    );

    vk::FALSE
}

pub struct VulkanRenderer {
    _api_entry: ash::Entry,
    instance: ash::Instance,
    debug_utils_loader: ash::ext::debug_utils::Instance,
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

impl VulkanRenderer {
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    fn setup_debug_utils(
        entry: &ash::Entry,
        instance: &ash::Instance,
    ) -> (ash::ext::debug_utils::Instance, vk::DebugUtilsMessengerEXT) {
        let debug_utils_loader = ash::ext::debug_utils::Instance::new(entry, instance);

        if cfg!(feature = "validation") {
            (debug_utils_loader, ash::vk::DebugUtilsMessengerEXT::null())
        } else {
            let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(
                    vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                        | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                    // | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
                )
                .message_type(
                    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .pfn_user_callback(Some(vulkan_debug_callback));

            let debug_callback = unsafe {
                debug_utils_loader
                    .create_debug_utils_messenger(&debug_info, None)
                    .expect("Debug Utils Callback")
            };

            (debug_utils_loader, debug_callback)
        }
    }

    /// Create the app info struct required by Vulkan initialization.
    fn init_app_info<'a>(
        app_name: &'a CString,
        engine_name: &'a CString,
    ) -> vk::ApplicationInfo<'a> {
        vk::ApplicationInfo::default()
            .application_name(app_name.as_c_str())
            .application_version(0)
            .engine_name(engine_name.as_c_str())
            .api_version(vk::API_VERSION_1_3)
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
    fn create(name: &str, display_handle: &RawDisplayHandle) -> Self {
        // Load the Vulkan API (dynamically) -- this needs to be kept alive while Vulkan is in use
        let entry = unsafe { ash::Entry::load().unwrap() };

        // Initialize info structs used to create the instance
        let app_name = CString::new(name).unwrap();
        let engine_name = CString::new("Scribble Vulkan Engine").unwrap();

        let app_info = VulkanRenderer::init_app_info(&app_name, &engine_name);

        // layer names
        let layer_names = unsafe {
            [ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_LAYER_KHRONOS_validation\0",
            )]
        };
        let layers_names_raw: Vec<*const c_char> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        // extension_names
        let mut extension_names = ash_window::enumerate_required_extensions(display_handle.clone())
            .unwrap()
            .to_vec();
        extension_names.push(debug_utils::NAME.as_ptr());

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_layer_names(&layers_names_raw)
            .enabled_extension_names(&extension_names)
            .flags(vk::InstanceCreateFlags::empty());

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

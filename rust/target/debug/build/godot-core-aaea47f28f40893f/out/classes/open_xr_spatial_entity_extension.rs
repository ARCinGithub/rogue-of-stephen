#![doc = "Sidecar module for class [`OpenXrSpatialEntityExtension`][crate::classes::OpenXrSpatialEntityExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRSpatialEntityExtension` enums](https://docs.godotengine.org/en/stable/classes/class_openxrspatialentityextension.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, ClassId, CowArg, InParamTuple, OutParamTuple, ParamTuple, RawPtr, RefArg
};
use crate::private::Signature;
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::classes::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `OpenXRSpatialEntityExtension`.\n\nInherits [`OpenXrExtensionWrapper`][crate::classes::OpenXrExtensionWrapper].\n\nRelated symbols:\n\n* [`open_xr_spatial_entity_extension`][crate::classes::open_xr_spatial_entity_extension]: sidecar module with related enum/flag types\n* [`IOpenXrSpatialEntityExtension`][crate::classes::IOpenXrSpatialEntityExtension]: virtual methods\n* [`SignalsOfOpenXrSpatialEntityExtension`][crate::classes::open_xr_spatial_entity_extension::SignalsOfOpenXrSpatialEntityExtension]: signal collection\n\n\nSee also [Godot docs for `OpenXRSpatialEntityExtension`](https://docs.godotengine.org/en/stable/classes/class_openxrspatialentityextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`OpenXrSpatialEntityExtension::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nOpenXR extension that handles spatial entities and, when enabled, allows querying those spatial entities. This extension will also automatically manage [`XRTracker`][crate::classes::XrTracker] objects for static entities."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrSpatialEntityExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrSpatialEntityExtension`][crate::classes::OpenXrSpatialEntityExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IOpenXrExtensionWrapper`][crate::classes::IOpenXrExtensionWrapper] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `OpenXRSpatialEntityExtension` methods](https://docs.godotengine.org/en/stable/classes/class_openxrspatialentityextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrSpatialEntityExtension: crate::obj::GodotClass < Base = OpenXrSpatialEntityExtension > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn on_get(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn on_set(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `on_get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        fn on_validate_property(&self, property: &mut crate::registry::info::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        #[cfg_attr(published_docs, doc(cfg(since_api = "4.3")))]
        fn on_get_property_list(&mut self) -> Vec < crate::registry::info::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn on_property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = "Returns a [`Dictionary`][crate::builtin::Dictionary] of OpenXR extensions related to this extension. `xr_version` specifies the OpenXR version we're instantiating. This will be zero if the editor requests this list to flag supported features. The [`Dictionary`][crate::builtin::Dictionary] should contain the name of the extension, mapped to a `bool *` cast to an integer:\n\n- If the `bool *` is a `nullptr` this extension is mandatory.\n\n- If the `bool *` points to a boolean, the boolean will be updated to `true` if the extension is enabled."]
        fn get_requested_extensions(&mut self, xr_version: u64,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures when querying OpenXR system abilities."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_system_properties_and_get_next_pointer_rawptr(&mut self, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures when the OpenXR instance is created. `xr_version` specifies the OpenXR version we're instantiating."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_instance_create_info_and_get_next_pointer_rawptr(&mut self, xr_version: u64, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures when the OpenXR session is created."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_session_create_and_get_next_pointer_rawptr(&mut self, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures when creating OpenXR swapchains."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_swapchain_create_info_and_get_next_pointer_rawptr(&mut self, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures when each hand tracker is created."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_hand_joint_locations_and_get_next_pointer_rawptr(&mut self, hand_index: i32, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures to the projection view of the given `view_index`.\n\n**Note:** This virtual method will be called on the render thread. Additionally, the data it returns will be used shortly after this method is called, so it needs to remain valid until the next time [`on_pre_render`][`crate::classes::IOpenXrExtensionWrapper::on_pre_render`] runs."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_projection_views_and_get_next_pointer_rawptr(&mut self, view_index: i32, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures to `XrFrameWaitInfo`.\n\nThis will only be called if the extension previously registered itself with [`register_frame_info_extension`][`crate::classes::OpenXrApiExtension::register_frame_info_extension`].\n\n**Note:** This virtual method will be called on the render thread."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_frame_wait_info_and_get_next_pointer_rawptr(&mut self, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures to `XrFrameEndInfo`.\n\nThis will only be called if the extension previously registered itself with [`register_frame_info_extension`][`crate::classes::OpenXrApiExtension::register_frame_info_extension`].\n\n**Note:** This virtual method will be called on the render thread. Additionally, the data it returns will be used shortly after this method is called, so it needs to remain valid until the next time [`on_pre_render`][`crate::classes::IOpenXrExtensionWrapper::on_pre_render`] runs."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_frame_end_info_and_get_next_pointer_rawptr(&mut self, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures to `XrViewLocateInfo`.\n\nThis will only be called if the extension previously registered itself with [`register_frame_info_extension`][`crate::classes::OpenXrApiExtension::register_frame_info_extension`].\n\n**Note:** This virtual method will be called on the render thread. Additionally, the data it returns will be used shortly after this method is called, so it needs to remain valid until the next time [`on_pre_render`][`crate::classes::IOpenXrExtensionWrapper::on_pre_render`] runs."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_view_locate_info_and_get_next_pointer_rawptr(&mut self, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures to `XrReferenceSpaceCreateInfo`."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_reference_space_create_info_and_get_next_pointer_rawptr(&mut self, reference_space_type: i32, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
        #[doc = "Called before [`set_view_configuration_and_get_next_pointer_rawptr`][`crate::classes::IOpenXrExtensionWrapper::set_view_configuration_and_get_next_pointer_rawptr`] to allow the extension to reserve data for the given number of views."]
        fn prepare_view_configuration(&mut self, view_count: i32,) {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures when querying OpenXR view configuration."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_view_configuration_and_get_next_pointer_rawptr(&mut self, view: u32, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
        #[doc = "Called to allow an extension to print additional information about its view configuration, if applicable. This will only be called if verbose output is enabled."]
        fn print_view_configuration_info(&self, view: i32,) {
            unimplemented !()
        }
        #[doc = "Returns the number of composition layers this extension wrapper provides via [`get_composition_layer`][`crate::classes::IOpenXrExtensionWrapper::get_composition_layer`].\n\nThis will only be called if the extension previously registered itself with [`register_composition_layer_provider`][`crate::classes::OpenXrApiExtension::register_composition_layer_provider`].\n\n**Note:** This virtual method will be called on the render thread. Additionally, the data it returns will be used shortly after this method is called, so it needs to remain valid until the next time [`on_pre_render`][`crate::classes::IOpenXrExtensionWrapper::on_pre_render`] runs."]
        fn get_composition_layer_count(&mut self,) -> i32 {
            unimplemented !()
        }
        #[doc = "Returns a pointer to an `XrCompositionLayerBaseHeader` struct to provide the given composition layer.\n\nThis will only be called if the extension previously registered itself with [`register_composition_layer_provider`][`crate::classes::OpenXrApiExtension::register_composition_layer_provider`].\n\n**Note:** This virtual method will be called on the render thread. Additionally, the data it returns will be used shortly after this method is called, so it needs to remain valid until the next time [`on_pre_render`][`crate::classes::IOpenXrExtensionWrapper::on_pre_render`] runs."]
        fn get_composition_layer(&mut self, index: i32,) -> u64 {
            unimplemented !()
        }
        #[doc = "Returns an integer that will be used to sort the given composition layer provided via [`get_composition_layer`][`crate::classes::IOpenXrExtensionWrapper::get_composition_layer`]. Lower numbers will move the layer to the front of the list, and higher numbers to the end. The default projection layer has an order of `0`, so layers provided by this method should probably be above or below (but not exactly) `0`.\n\nThis will only be called if the extension previously registered itself with [`register_composition_layer_provider`][`crate::classes::OpenXrApiExtension::register_composition_layer_provider`].\n\n**Note:** This virtual method will be called on the render thread. Additionally, the data it returns will be used shortly after this method is called, so it needs to remain valid until the next time [`on_pre_render`][`crate::classes::IOpenXrExtensionWrapper::on_pre_render`] runs."]
        fn get_composition_layer_order(&mut self, index: i32,) -> i32 {
            unimplemented !()
        }
        #[doc = "Returns a [`PackedStringArray`][crate::builtin::PackedStringArray] of positional tracker names that are used within the extension wrapper."]
        fn get_suggested_tracker_names(&mut self,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Allows extensions to register additional controller metadata. This function is called even when the OpenXR API is not constructed as the metadata needs to be available to the editor.\n\nExtensions should also provide metadata regardless of whether they are supported on the host system. The controller data is used to setup action maps for users who may have access to the relevant hardware."]
        fn on_register_metadata(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called before the OpenXR instance is created.\n\n**Note:** This virtual method will be called on the main thread, however, it will be called _before_ OpenXR becomes involved in rendering, so it is safe to write to data that will be used by the render thread."]
        fn on_before_instance_created(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called right after the OpenXR instance is created.\n\n**Note:** This virtual method will be called on the main thread, however, it will be called _before_ OpenXR becomes involved in rendering, so it is safe to write to data that will be used by the render thread."]
        fn on_instance_created(&mut self, instance: u64,) {
            unimplemented !()
        }
        #[doc = "Called right before the OpenXR instance is destroyed.\n\n**Note:** This virtual method will be called on the main thread, however, it will be called _after_ OpenXR is done being involved in rendering, so it is safe to write to data that was used by the render thread."]
        fn on_instance_destroyed(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called right after the OpenXR session is created.\n\n**Note:** This virtual method will be called on the main thread, however, it will be called _before_ OpenXR becomes involved in rendering, so it is safe to write to data that will be used by the render thread."]
        fn on_session_created(&mut self, session: u64,) {
            unimplemented !()
        }
        #[doc = "Called as part of the OpenXR process handling. This happens right before general and physics processing steps of the main loop. During this step controller data is queried and made available to game logic."]
        fn on_process(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when OpenXR has performed its action sync."]
        fn on_sync_actions(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called right before the XR viewports begin their rendering step.\n\n**Note:** This virtual method will be called on the render thread."]
        fn on_pre_render(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called right after the main swapchains are (re)created.\n\n**Note:** This virtual method will be called on the render thread."]
        fn on_main_swapchains_created(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called right before the given viewport is rendered.\n\n**Note:** This virtual method will be called on the render thread."]
        fn on_pre_draw_viewport(&mut self, viewport: Rid,) {
            unimplemented !()
        }
        #[doc = "Called right after the given viewport is rendered.\n\n**Note:** The draw commands might only be queued at this point, not executed.\n\n**Note:** This virtual method will be called on the render thread."]
        fn on_post_draw_viewport(&mut self, viewport: Rid,) {
            unimplemented !()
        }
        #[doc = "Called right before the OpenXR session is destroyed.\n\n**Note:** This virtual method will be called on the main thread, however, it will be called _after_ OpenXR is done being involved in rendering, so it is safe to write to data that was used by the render thread."]
        fn on_session_destroyed(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the OpenXR session state is changed to idle."]
        fn on_state_idle(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the OpenXR session state is changed to ready. This means OpenXR is ready to set up the session."]
        fn on_state_ready(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the OpenXR session state is changed to synchronized. OpenXR also returns to this state when the application loses focus."]
        fn on_state_synchronized(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the OpenXR session state is changed to visible. This means OpenXR is now ready to receive frames."]
        fn on_state_visible(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the OpenXR session state is changed to focused. This state is the active state when the game runs."]
        fn on_state_focused(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the OpenXR session state is changed to stopping."]
        fn on_state_stopping(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the OpenXR session state is changed to loss pending."]
        fn on_state_loss_pending(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the OpenXR session state is changed to exiting."]
        fn on_state_exiting(&mut self,) {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nCalled when there is an OpenXR event to process. When implementing, return `true` if the event was handled, return `false` otherwise."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn on_event_polled_rawptr(&mut self, event: crate::meta::RawPtr < * const c_void >,) -> bool {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures to composition layers created by [`OpenXRCompositionLayer`][crate::classes::OpenXrCompositionLayer].\n\n`property_values` contains the values of the properties returned by [`get_viewport_composition_layer_extension_properties`][`crate::classes::IOpenXrExtensionWrapper::get_viewport_composition_layer_extension_properties`].\n\n`layer` is a pointer to an `XrCompositionLayerBaseHeader` struct.\n\n**Note:** This virtual method will be called on the render thread. Additionally, the data it returns will be used shortly after this method is called, so it needs to remain valid until the next time [`on_pre_render`][`crate::classes::IOpenXrExtensionWrapper::on_pre_render`] runs."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_viewport_composition_layer_and_get_next_pointer_rawptr(&mut self, layer: crate::meta::RawPtr < * const c_void >, property_values: VarDictionary, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
        #[doc = "Gets an array of [`Dictionary`][crate::builtin::Dictionary]s that represent properties, just like [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`], that will be added to [`OpenXRCompositionLayer`][crate::classes::OpenXrCompositionLayer] nodes.\n\n**Note:** This virtual method will be called on the render thread."]
        fn get_viewport_composition_layer_extension_properties(&mut self,) -> Array < AnyDictionary > {
            unimplemented !()
        }
        #[doc = "Gets a [`Dictionary`][crate::builtin::Dictionary] containing the default values for the properties returned by [`get_viewport_composition_layer_extension_properties`][`crate::classes::IOpenXrExtensionWrapper::get_viewport_composition_layer_extension_properties`]."]
        fn get_viewport_composition_layer_extension_property_defaults(&mut self,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nCalled when a composition layer created via [`OpenXRCompositionLayer`][crate::classes::OpenXrCompositionLayer] is destroyed.\n\n`layer` is a pointer to an `XrCompositionLayerBaseHeader` struct."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn on_viewport_composition_layer_destroyed_rawptr(&mut self, layer: crate::meta::RawPtr < * const c_void >,) {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nAdd additional data structures to Android surface swapchains created by [`OpenXRCompositionLayer`][crate::classes::OpenXrCompositionLayer].\n\n`property_values` contains the values of the properties returned by [`get_viewport_composition_layer_extension_properties`][`crate::classes::IOpenXrExtensionWrapper::get_viewport_composition_layer_extension_properties`].\n\n**Note:** This virtual method will be called on the render thread."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_android_surface_swapchain_create_info_and_get_next_pointer_rawptr(&mut self, property_values: VarDictionary, next_pointer: crate::meta::RawPtr < * mut c_void >,) -> u64 {
            unimplemented !()
        }
    }
    impl OpenXrSpatialEntityExtension {
        #[doc = "Returns `true` if this spatial entity `capability` is supported by the hardware used."]
        pub fn supports_capability(&mut self, capability: crate::classes::open_xr_spatial_entity_extension::Capability,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::open_xr_spatial_entity_extension::Capability,);
            let args = (capability,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5743usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "supports_capability", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this `capability` supports the `component_type`."]
        pub fn supports_component_type(&mut self, capability: crate::classes::open_xr_spatial_entity_extension::Capability, component_type: crate::classes::open_xr_spatial_entity_extension::ComponentType,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::open_xr_spatial_entity_extension::Capability, crate::classes::open_xr_spatial_entity_extension::ComponentType,);
            let args = (capability, component_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5744usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "supports_component_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new spatial context that handles entities for the provided capability configurations. `capability_configurations` is an array of [`OpenXRSpatialCapabilityConfigurationBaseHeader`][crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader] with the needed capability configuration data.\n\n`next` is an optional parameter that can contain additional information for creating our spatial context.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the creation process. On success `user_callback` will be called if specified. The result data for this function is the [`RID`][crate::builtin::Rid] for our spatial context."]
        pub(crate) fn create_spatial_context_full(&mut self, capability_configurations: RefArg < Array < Gd < crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader > > >, next: CowArg < Option < Gd < crate::classes::OpenXrStructureBase > > >, user_callback: RefArg < Callable >,) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
            type CallRet = Option < Gd < crate::classes::OpenXrFutureResult > >;
            type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Array < Gd < crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader > > >, CowArg < 'a1, Option < Gd < crate::classes::OpenXrStructureBase > > >, RefArg < 'a2, Callable >,);
            let args = (capability_configurations, next, user_callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5745usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "create_spatial_context", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_spatial_context_ex`][Self::create_spatial_context_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new spatial context that handles entities for the provided capability configurations. `capability_configurations` is an array of [`OpenXRSpatialCapabilityConfigurationBaseHeader`][crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader] with the needed capability configuration data.\n\n`next` is an optional parameter that can contain additional information for creating our spatial context.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the creation process. On success `user_callback` will be called if specified. The result data for this function is the [`RID`][crate::builtin::Rid] for our spatial context."]
        #[inline]
        pub fn create_spatial_context(&mut self, capability_configurations: &Array < Gd < crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader > >,) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
            self.create_spatial_context_ex(capability_configurations,) . done()
        }
        #[doc = "Creates a new spatial context that handles entities for the provided capability configurations. `capability_configurations` is an array of [`OpenXRSpatialCapabilityConfigurationBaseHeader`][crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader] with the needed capability configuration data.\n\n`next` is an optional parameter that can contain additional information for creating our spatial context.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the creation process. On success `user_callback` will be called if specified. The result data for this function is the [`RID`][crate::builtin::Rid] for our spatial context."]
        #[inline]
        pub fn create_spatial_context_ex < 'ex > (&'ex mut self, capability_configurations: &'ex Array < Gd < crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader > >,) -> ExCreateSpatialContext < 'ex > {
            ExCreateSpatialContext::new(self, capability_configurations,)
        }
        #[doc = "Returns `true` if the spatial context finished its creation and is ready to be used."]
        pub fn get_spatial_context_ready(&self, spatial_context: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (spatial_context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5746usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_spatial_context_ready", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Frees a spatial context previously created when calling [`create_spatial_context`][`crate::classes::OpenXrSpatialEntityExtension::create_spatial_context`]. If the spatial context creation is still ongoing, the asynchronous process is cancelled."]
        pub fn free_spatial_context(&mut self, spatial_context: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (spatial_context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5747usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "free_spatial_context", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the OpenXR spatial context handle for this snapshot.\n\n**Note:** This method is intended to be used from GDExtensions that implement spatial entity capability handlers."]
        pub fn get_spatial_context_handle(&self, spatial_context: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (spatial_context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5748usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_spatial_context_handle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Starts a new discovery query, this will gather all objects tracked by the `spatial_context` that have at least one of the component types specified in `component_types`.\n\n`next` is an optional parameter that can contain additional information for executing the discovery query.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the discovery process. On success `user_callback` will be called if specified. The result data for this function is the [`RID`][crate::builtin::Rid] for our snapshot."]
        pub(crate) fn discover_spatial_entities_full(&mut self, spatial_context: Rid, component_types: RefArg < PackedInt64Array >, next: CowArg < Option < Gd < crate::classes::OpenXrStructureBase > > >, user_callback: RefArg < Callable >,) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
            type CallRet = Option < Gd < crate::classes::OpenXrFutureResult > >;
            type CallParams < 'a0, 'a1, 'a2, > = (Rid, RefArg < 'a0, PackedInt64Array >, CowArg < 'a1, Option < Gd < crate::classes::OpenXrStructureBase > > >, RefArg < 'a2, Callable >,);
            let args = (spatial_context, component_types, next, user_callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5749usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "discover_spatial_entities", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`discover_spatial_entities_ex`][Self::discover_spatial_entities_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Starts a new discovery query, this will gather all objects tracked by the `spatial_context` that have at least one of the component types specified in `component_types`.\n\n`next` is an optional parameter that can contain additional information for executing the discovery query.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the discovery process. On success `user_callback` will be called if specified. The result data for this function is the [`RID`][crate::builtin::Rid] for our snapshot."]
        #[inline]
        pub fn discover_spatial_entities(&mut self, spatial_context: Rid, component_types: &PackedInt64Array,) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
            self.discover_spatial_entities_ex(spatial_context, component_types,) . done()
        }
        #[doc = "Starts a new discovery query, this will gather all objects tracked by the `spatial_context` that have at least one of the component types specified in `component_types`.\n\n`next` is an optional parameter that can contain additional information for executing the discovery query.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the discovery process. On success `user_callback` will be called if specified. The result data for this function is the [`RID`][crate::builtin::Rid] for our snapshot."]
        #[inline]
        pub fn discover_spatial_entities_ex < 'ex > (&'ex mut self, spatial_context: Rid, component_types: &'ex PackedInt64Array,) -> ExDiscoverSpatialEntities < 'ex > {
            ExDiscoverSpatialEntities::new(self, spatial_context, component_types,)
        }
        #[doc = "Performs a snapshot for a limited number of entities. This is NOT an asynchronous method and will return the snapshot immediately."]
        pub(crate) fn update_spatial_entities_full(&mut self, spatial_context: Rid, entities: RefArg < Array < Rid > >, component_types: RefArg < PackedInt64Array >, next: CowArg < Option < Gd < crate::classes::OpenXrStructureBase > > >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, 'a1, 'a2, > = (Rid, RefArg < 'a0, Array < Rid > >, RefArg < 'a1, PackedInt64Array >, CowArg < 'a2, Option < Gd < crate::classes::OpenXrStructureBase > > >,);
            let args = (spatial_context, entities, component_types, next,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5750usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "update_spatial_entities", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`update_spatial_entities_ex`][Self::update_spatial_entities_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Performs a snapshot for a limited number of entities. This is NOT an asynchronous method and will return the snapshot immediately."]
        #[inline]
        pub fn update_spatial_entities(&mut self, spatial_context: Rid, entities: &Array < Rid >, component_types: &PackedInt64Array,) -> Rid {
            self.update_spatial_entities_ex(spatial_context, entities, component_types,) . done()
        }
        #[doc = "Performs a snapshot for a limited number of entities. This is NOT an asynchronous method and will return the snapshot immediately."]
        #[inline]
        pub fn update_spatial_entities_ex < 'ex > (&'ex mut self, spatial_context: Rid, entities: &'ex Array < Rid >, component_types: &'ex PackedInt64Array,) -> ExUpdateSpatialEntities < 'ex > {
            ExUpdateSpatialEntities::new(self, spatial_context, entities, component_types,)
        }
        #[doc = "Frees a spatial snapshot previously created when calling [`discover_spatial_entities`][`crate::classes::OpenXrSpatialEntityExtension::discover_spatial_entities`]. If the spatial snapshot creation is still ongoing, the asynchronous process is cancelled."]
        pub fn free_spatial_snapshot(&mut self, spatial_snapshot: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (spatial_snapshot,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5751usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "free_spatial_snapshot", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the OpenXR spatial snapshot handle for this snapshot.\n\n**Note:** This method is intended to be used from GDExtensions that implement spatial entity capability handlers."]
        pub fn get_spatial_snapshot_handle(&self, spatial_snapshot: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (spatial_snapshot,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5752usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_spatial_snapshot_handle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the spatial context related to this spatial snapshot."]
        pub fn get_spatial_snapshot_context(&self, spatial_snapshot: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (spatial_snapshot,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_spatial_snapshot_context", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Queries the snapshot data. This will find all entities in the snapshot that contain all requested components in `component_data`. The objects held within `component_data` will then be populated with the queried data. `component_data` must always have an object of [`OpenXRSpatialQueryResultData`][crate::classes::OpenXrSpatialQueryResultData] as the first entry.\n\n`next` is an optional parameter that can contain additional information passed when setting our query conditions."]
        pub(crate) fn query_snapshot_full(&mut self, spatial_snapshot: Rid, component_data: RefArg < Array < Gd < crate::classes::OpenXrSpatialComponentData > > >, next: CowArg < Option < Gd < crate::classes::OpenXrStructureBase > > >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, Array < Gd < crate::classes::OpenXrSpatialComponentData > > >, CowArg < 'a1, Option < Gd < crate::classes::OpenXrStructureBase > > >,);
            let args = (spatial_snapshot, component_data, next,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "query_snapshot", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`query_snapshot_ex`][Self::query_snapshot_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Queries the snapshot data. This will find all entities in the snapshot that contain all requested components in `component_data`. The objects held within `component_data` will then be populated with the queried data. `component_data` must always have an object of [`OpenXRSpatialQueryResultData`][crate::classes::OpenXrSpatialQueryResultData] as the first entry.\n\n`next` is an optional parameter that can contain additional information passed when setting our query conditions."]
        #[inline]
        pub fn query_snapshot(&mut self, spatial_snapshot: Rid, component_data: &Array < Gd < crate::classes::OpenXrSpatialComponentData > >,) -> bool {
            self.query_snapshot_ex(spatial_snapshot, component_data,) . done()
        }
        #[doc = "Queries the snapshot data. This will find all entities in the snapshot that contain all requested components in `component_data`. The objects held within `component_data` will then be populated with the queried data. `component_data` must always have an object of [`OpenXRSpatialQueryResultData`][crate::classes::OpenXrSpatialQueryResultData] as the first entry.\n\n`next` is an optional parameter that can contain additional information passed when setting our query conditions."]
        #[inline]
        pub fn query_snapshot_ex < 'ex > (&'ex mut self, spatial_snapshot: Rid, component_data: &'ex Array < Gd < crate::classes::OpenXrSpatialComponentData > >,) -> ExQuerySnapshot < 'ex > {
            ExQuerySnapshot::new(self, spatial_snapshot, component_data,)
        }
        #[doc = "Returns a string from a buffer that was retrieved when taking a snapshot."]
        pub fn get_string(&self, spatial_snapshot: Rid, buffer_id: u64,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid, u64,);
            let args = (spatial_snapshot, buffer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a buffer with 8 bit ints from a buffer that was retrieved when taking a snapshot."]
        pub fn get_uint8_buffer(&self, spatial_snapshot: Rid, buffer_id: u64,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (Rid, u64,);
            let args = (spatial_snapshot, buffer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_uint8_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a buffer with 16 bit ints from a buffer that was retrieved when taking a snapshot."]
        pub fn get_uint16_buffer(&self, spatial_snapshot: Rid, buffer_id: u64,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid, u64,);
            let args = (spatial_snapshot, buffer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_uint16_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a buffer with 32 bit ints from a buffer that was retrieved when taking a snapshot."]
        pub fn get_uint32_buffer(&self, spatial_snapshot: Rid, buffer_id: u64,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid, u64,);
            let args = (spatial_snapshot, buffer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_uint32_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a buffer with floats from a buffer that was retrieved when taking a snapshot."]
        pub fn get_float_buffer(&self, spatial_snapshot: Rid, buffer_id: u64,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = (Rid, u64,);
            let args = (spatial_snapshot, buffer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_float_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a buffer with [`Vector2`][crate::builtin::Vector2] entries from a buffer that was retrieved when taking a snapshot."]
        pub fn get_vector2_buffer(&self, spatial_snapshot: Rid, buffer_id: u64,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = (Rid, u64,);
            let args = (spatial_snapshot, buffer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_vector2_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a buffer with [`Vector3`][crate::builtin::Vector3] entries from a buffer that was retrieved when taking a snapshot."]
        pub fn get_vector3_buffer(&self, spatial_snapshot: Rid, buffer_id: u64,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = (Rid, u64,);
            let args = (spatial_snapshot, buffer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_vector3_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] for the specified spatial entity ID."]
        pub fn find_spatial_entity(&mut self, entity_id: u64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (u64,);
            let args = (entity_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "find_spatial_entity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers an entity that was created directly on the OpenXR runtime."]
        pub fn add_spatial_entity(&mut self, spatial_context: Rid, entity_id: u64, entity: u64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, u64, u64,);
            let args = (spatial_context, entity_id, entity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "add_spatial_entity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new entity for this `entity_id`. The `spatial_context` should match the context that discovered the entity."]
        pub fn make_spatial_entity(&mut self, spatial_context: Rid, entity_id: u64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, u64,);
            let args = (spatial_context, entity_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "make_spatial_entity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the internal `XrSpatialEntityIdEXT` associated with the entity."]
        pub fn get_spatial_entity_id(&self, entity: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (entity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_spatial_entity_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the spatial context for this entity."]
        pub fn get_spatial_entity_context(&self, entity: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (entity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "get_spatial_entity_context", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Frees an entity previously created when calling [`add_spatial_entity`][`crate::classes::OpenXrSpatialEntityExtension::add_spatial_entity`] or [`make_spatial_entity`][`crate::classes::OpenXrSpatialEntityExtension::make_spatial_entity`]."]
        pub fn free_spatial_entity(&mut self, entity: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (entity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityExtension", "free_spatial_entity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" Creates a validated object for FFI boundary crossing."]
        #[doc = r""]
        #[doc = r" Low-level internal method. Validation (liveness/type checks) depend on safeguard level."]
        fn __validated_obj(&self) -> crate::obj::ValidatedObject {
            let raw_gd = unsafe {
                std::mem::transmute::< &Self, &crate::obj::RawGd < Self >> (self)
            };
            raw_gd.validated_object()
        }
        #[doc(hidden)]
        pub fn __object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
    }
    impl crate::obj::GodotClass for OpenXrSpatialEntityExtension {
        type Base = crate::classes::OpenXrExtensionWrapper;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRSpatialEntityExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrSpatialEntityExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::OpenXrExtensionWrapper > for OpenXrSpatialEntityExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrSpatialEntityExtension {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrSpatialEntityExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrSpatialEntityExtension {
        type Target = crate::classes::OpenXrExtensionWrapper;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrSpatialEntityExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrSpatialEntityExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrSpatialEntityExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrSpatialEntityExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrExtensionWrapper > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`OpenXrSpatialEntityExtension::create_spatial_context_ex`][super::OpenXrSpatialEntityExtension::create_spatial_context_ex]."]
#[must_use]
pub struct ExCreateSpatialContext < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::OpenXrSpatialEntityExtension, capability_configurations: CowArg < 'ex, Array < Gd < crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader > > >, next: CowArg < 'ex, Option < Gd < crate::classes::OpenXrStructureBase > > >, user_callback: CowArg < 'ex, Callable >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateSpatialContext < 'ex > {
    fn new(surround_object: &'ex mut re_export::OpenXrSpatialEntityExtension, capability_configurations: &'ex Array < Gd < crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader > >,) -> Self {
        let next = Gd::null_arg();
        let user_callback = Callable::invalid();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, capability_configurations: CowArg::Borrowed(capability_configurations), next: next.into_arg(), user_callback: CowArg::Owned(user_callback),
        }
    }
    #[inline]
    pub fn next(self, next: impl AsArg < Option < Gd < crate::classes::OpenXrStructureBase >> > + 'ex) -> Self {
        Self {
            next: next.into_arg(), .. self
        }
    }
    #[inline]
    pub fn user_callback(self, user_callback: &'ex Callable) -> Self {
        Self {
            user_callback: CowArg::Borrowed(user_callback), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
        let Self {
            _phantom, surround_object, capability_configurations, next, user_callback,
        }
        = self;
        re_export::OpenXrSpatialEntityExtension::create_spatial_context_full(surround_object, capability_configurations.cow_as_arg(), next, user_callback.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`OpenXrSpatialEntityExtension::discover_spatial_entities_ex`][super::OpenXrSpatialEntityExtension::discover_spatial_entities_ex]."]
#[must_use]
pub struct ExDiscoverSpatialEntities < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::OpenXrSpatialEntityExtension, spatial_context: Rid, component_types: CowArg < 'ex, PackedInt64Array >, next: CowArg < 'ex, Option < Gd < crate::classes::OpenXrStructureBase > > >, user_callback: CowArg < 'ex, Callable >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDiscoverSpatialEntities < 'ex > {
    fn new(surround_object: &'ex mut re_export::OpenXrSpatialEntityExtension, spatial_context: Rid, component_types: &'ex PackedInt64Array,) -> Self {
        let next = Gd::null_arg();
        let user_callback = Callable::invalid();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, spatial_context: spatial_context, component_types: CowArg::Borrowed(component_types), next: next.into_arg(), user_callback: CowArg::Owned(user_callback),
        }
    }
    #[inline]
    pub fn next(self, next: impl AsArg < Option < Gd < crate::classes::OpenXrStructureBase >> > + 'ex) -> Self {
        Self {
            next: next.into_arg(), .. self
        }
    }
    #[inline]
    pub fn user_callback(self, user_callback: &'ex Callable) -> Self {
        Self {
            user_callback: CowArg::Borrowed(user_callback), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
        let Self {
            _phantom, surround_object, spatial_context, component_types, next, user_callback,
        }
        = self;
        re_export::OpenXrSpatialEntityExtension::discover_spatial_entities_full(surround_object, spatial_context, component_types.cow_as_arg(), next, user_callback.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`OpenXrSpatialEntityExtension::update_spatial_entities_ex`][super::OpenXrSpatialEntityExtension::update_spatial_entities_ex]."]
#[must_use]
pub struct ExUpdateSpatialEntities < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::OpenXrSpatialEntityExtension, spatial_context: Rid, entities: CowArg < 'ex, Array < Rid > >, component_types: CowArg < 'ex, PackedInt64Array >, next: CowArg < 'ex, Option < Gd < crate::classes::OpenXrStructureBase > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExUpdateSpatialEntities < 'ex > {
    fn new(surround_object: &'ex mut re_export::OpenXrSpatialEntityExtension, spatial_context: Rid, entities: &'ex Array < Rid >, component_types: &'ex PackedInt64Array,) -> Self {
        let next = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, spatial_context: spatial_context, entities: CowArg::Borrowed(entities), component_types: CowArg::Borrowed(component_types), next: next.into_arg(),
        }
    }
    #[inline]
    pub fn next(self, next: impl AsArg < Option < Gd < crate::classes::OpenXrStructureBase >> > + 'ex) -> Self {
        Self {
            next: next.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, spatial_context, entities, component_types, next,
        }
        = self;
        re_export::OpenXrSpatialEntityExtension::update_spatial_entities_full(surround_object, spatial_context, entities.cow_as_arg(), component_types.cow_as_arg(), next,)
    }
}
#[doc = "Default-param extender for [`OpenXrSpatialEntityExtension::query_snapshot_ex`][super::OpenXrSpatialEntityExtension::query_snapshot_ex]."]
#[must_use]
pub struct ExQuerySnapshot < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::OpenXrSpatialEntityExtension, spatial_snapshot: Rid, component_data: CowArg < 'ex, Array < Gd < crate::classes::OpenXrSpatialComponentData > > >, next: CowArg < 'ex, Option < Gd < crate::classes::OpenXrStructureBase > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExQuerySnapshot < 'ex > {
    fn new(surround_object: &'ex mut re_export::OpenXrSpatialEntityExtension, spatial_snapshot: Rid, component_data: &'ex Array < Gd < crate::classes::OpenXrSpatialComponentData > >,) -> Self {
        let next = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, spatial_snapshot: spatial_snapshot, component_data: CowArg::Borrowed(component_data), next: next.into_arg(),
        }
    }
    #[inline]
    pub fn next(self, next: impl AsArg < Option < Gd < crate::classes::OpenXrStructureBase >> > + 'ex) -> Self {
        Self {
            next: next.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, spatial_snapshot, component_data, next,
        }
        = self;
        re_export::OpenXrSpatialEntityExtension::query_snapshot_full(surround_object, spatial_snapshot, component_data.cow_as_arg(), next,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Capability {
    ord: i32
}
impl Capability {
    #[doc(alias = "CAPABILITY_PLANE_TRACKING")]
    #[doc = "Godot enumerator name: `CAPABILITY_PLANE_TRACKING`"]
    pub const PLANE_TRACKING: Capability = Capability {
        ord: 1000741000i32
    };
    #[doc(alias = "CAPABILITY_MARKER_TRACKING_QR_CODE")]
    #[doc = "Godot enumerator name: `CAPABILITY_MARKER_TRACKING_QR_CODE`"]
    pub const MARKER_TRACKING_QR_CODE: Capability = Capability {
        ord: 1000743000i32
    };
    #[doc(alias = "CAPABILITY_MARKER_TRACKING_MICRO_QR_CODE")]
    #[doc = "Godot enumerator name: `CAPABILITY_MARKER_TRACKING_MICRO_QR_CODE`"]
    pub const MARKER_TRACKING_MICRO_QR_CODE: Capability = Capability {
        ord: 1000743001i32
    };
    #[doc(alias = "CAPABILITY_MARKER_TRACKING_ARUCO_MARKER")]
    #[doc = "Godot enumerator name: `CAPABILITY_MARKER_TRACKING_ARUCO_MARKER`"]
    pub const MARKER_TRACKING_ARUCO_MARKER: Capability = Capability {
        ord: 1000743002i32
    };
    #[doc(alias = "CAPABILITY_MARKER_TRACKING_APRIL_TAG")]
    #[doc = "Godot enumerator name: `CAPABILITY_MARKER_TRACKING_APRIL_TAG`"]
    pub const MARKER_TRACKING_APRIL_TAG: Capability = Capability {
        ord: 1000743003i32
    };
    #[doc(alias = "CAPABILITY_ANCHOR")]
    #[doc = "Godot enumerator name: `CAPABILITY_ANCHOR`"]
    pub const ANCHOR: Capability = Capability {
        ord: 1000762000i32
    };
    
}
impl std::fmt::Debug for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Capability") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Capability {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1000741000i32 | ord @ 1000743000i32 | ord @ 1000743001i32 | ord @ 1000743002i32 | ord @ 1000743003i32 | ord @ 1000762000i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::PLANE_TRACKING => "PLANE_TRACKING", Self::MARKER_TRACKING_QR_CODE => "MARKER_TRACKING_QR_CODE", Self::MARKER_TRACKING_MICRO_QR_CODE => "MARKER_TRACKING_MICRO_QR_CODE", Self::MARKER_TRACKING_ARUCO_MARKER => "MARKER_TRACKING_ARUCO_MARKER", Self::MARKER_TRACKING_APRIL_TAG => "MARKER_TRACKING_APRIL_TAG", Self::ANCHOR => "ANCHOR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Capability::PLANE_TRACKING, Capability::MARKER_TRACKING_QR_CODE, Capability::MARKER_TRACKING_MICRO_QR_CODE, Capability::MARKER_TRACKING_ARUCO_MARKER, Capability::MARKER_TRACKING_APRIL_TAG, Capability::ANCHOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Capability >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PLANE_TRACKING", "CAPABILITY_PLANE_TRACKING", Capability::PLANE_TRACKING), crate::meta::inspect::EnumConstant::new("MARKER_TRACKING_QR_CODE", "CAPABILITY_MARKER_TRACKING_QR_CODE", Capability::MARKER_TRACKING_QR_CODE), crate::meta::inspect::EnumConstant::new("MARKER_TRACKING_MICRO_QR_CODE", "CAPABILITY_MARKER_TRACKING_MICRO_QR_CODE", Capability::MARKER_TRACKING_MICRO_QR_CODE), crate::meta::inspect::EnumConstant::new("MARKER_TRACKING_ARUCO_MARKER", "CAPABILITY_MARKER_TRACKING_ARUCO_MARKER", Capability::MARKER_TRACKING_ARUCO_MARKER), crate::meta::inspect::EnumConstant::new("MARKER_TRACKING_APRIL_TAG", "CAPABILITY_MARKER_TRACKING_APRIL_TAG", Capability::MARKER_TRACKING_APRIL_TAG), crate::meta::inspect::EnumConstant::new("ANCHOR", "CAPABILITY_ANCHOR", Capability::ANCHOR)]
        }
    }
}
impl crate::meta::GodotConvert for Capability {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Capability Plane Tracking", 1000741000i64), EnumeratorShape::new_int("Capability Marker Tracking Qr Code", 1000743000i64), EnumeratorShape::new_int("Capability Marker Tracking Micro Qr Code", 1000743001i64), EnumeratorShape::new_int("Capability Marker Tracking Aruco Marker", 1000743002i64), EnumeratorShape::new_int("Capability Marker Tracking April Tag", 1000743003i64), EnumeratorShape::new_int("Capability Anchor", 1000762000i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("OpenXRSpatialEntityExtension.Capability")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Capability {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Capability {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Capability {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for Capability {
    
}
impl crate::meta::Element for Capability {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ComponentType {
    ord: i32
}
impl ComponentType {
    #[doc(alias = "COMPONENT_TYPE_BOUNDED_2D")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_BOUNDED_2D`"]
    pub const BOUNDED_2D: ComponentType = ComponentType {
        ord: 1i32
    };
    #[doc(alias = "COMPONENT_TYPE_BOUNDED_3D")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_BOUNDED_3D`"]
    pub const BOUNDED_3D: ComponentType = ComponentType {
        ord: 2i32
    };
    #[doc(alias = "COMPONENT_TYPE_PARENT")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_PARENT`"]
    pub const PARENT: ComponentType = ComponentType {
        ord: 3i32
    };
    #[doc(alias = "COMPONENT_TYPE_MESH_3D")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_MESH_3D`"]
    pub const MESH_3D: ComponentType = ComponentType {
        ord: 4i32
    };
    #[doc(alias = "COMPONENT_TYPE_PLANE_ALIGNMENT")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_PLANE_ALIGNMENT`"]
    pub const PLANE_ALIGNMENT: ComponentType = ComponentType {
        ord: 1000741000i32
    };
    #[doc(alias = "COMPONENT_TYPE_MESH_2D")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_MESH_2D`"]
    pub const MESH_2D: ComponentType = ComponentType {
        ord: 1000741001i32
    };
    #[doc(alias = "COMPONENT_TYPE_POLYGON_2D")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_POLYGON_2D`"]
    pub const POLYGON_2D: ComponentType = ComponentType {
        ord: 1000741002i32
    };
    #[doc(alias = "COMPONENT_TYPE_PLANE_SEMANTIC_LABEL")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_PLANE_SEMANTIC_LABEL`"]
    pub const PLANE_SEMANTIC_LABEL: ComponentType = ComponentType {
        ord: 1000741003i32
    };
    #[doc(alias = "COMPONENT_TYPE_MARKER")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_MARKER`"]
    pub const MARKER: ComponentType = ComponentType {
        ord: 1000743000i32
    };
    #[doc(alias = "COMPONENT_TYPE_ANCHOR")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_ANCHOR`"]
    pub const ANCHOR: ComponentType = ComponentType {
        ord: 1000762000i32
    };
    #[doc(alias = "COMPONENT_TYPE_PERSISTENCE")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_PERSISTENCE`"]
    pub const PERSISTENCE: ComponentType = ComponentType {
        ord: 1000763000i32
    };
    
}
impl std::fmt::Debug for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ComponentType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ComponentType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 1000741000i32 | ord @ 1000741001i32 | ord @ 1000741002i32 | ord @ 1000741003i32 | ord @ 1000743000i32 | ord @ 1000762000i32 | ord @ 1000763000i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BOUNDED_2D => "BOUNDED_2D", Self::BOUNDED_3D => "BOUNDED_3D", Self::PARENT => "PARENT", Self::MESH_3D => "MESH_3D", Self::PLANE_ALIGNMENT => "PLANE_ALIGNMENT", Self::MESH_2D => "MESH_2D", Self::POLYGON_2D => "POLYGON_2D", Self::PLANE_SEMANTIC_LABEL => "PLANE_SEMANTIC_LABEL", Self::MARKER => "MARKER", Self::ANCHOR => "ANCHOR", Self::PERSISTENCE => "PERSISTENCE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ComponentType::BOUNDED_2D, ComponentType::BOUNDED_3D, ComponentType::PARENT, ComponentType::MESH_3D, ComponentType::PLANE_ALIGNMENT, ComponentType::MESH_2D, ComponentType::POLYGON_2D, ComponentType::PLANE_SEMANTIC_LABEL, ComponentType::MARKER, ComponentType::ANCHOR, ComponentType::PERSISTENCE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ComponentType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BOUNDED_2D", "COMPONENT_TYPE_BOUNDED_2D", ComponentType::BOUNDED_2D), crate::meta::inspect::EnumConstant::new("BOUNDED_3D", "COMPONENT_TYPE_BOUNDED_3D", ComponentType::BOUNDED_3D), crate::meta::inspect::EnumConstant::new("PARENT", "COMPONENT_TYPE_PARENT", ComponentType::PARENT), crate::meta::inspect::EnumConstant::new("MESH_3D", "COMPONENT_TYPE_MESH_3D", ComponentType::MESH_3D), crate::meta::inspect::EnumConstant::new("PLANE_ALIGNMENT", "COMPONENT_TYPE_PLANE_ALIGNMENT", ComponentType::PLANE_ALIGNMENT), crate::meta::inspect::EnumConstant::new("MESH_2D", "COMPONENT_TYPE_MESH_2D", ComponentType::MESH_2D), crate::meta::inspect::EnumConstant::new("POLYGON_2D", "COMPONENT_TYPE_POLYGON_2D", ComponentType::POLYGON_2D), crate::meta::inspect::EnumConstant::new("PLANE_SEMANTIC_LABEL", "COMPONENT_TYPE_PLANE_SEMANTIC_LABEL", ComponentType::PLANE_SEMANTIC_LABEL), crate::meta::inspect::EnumConstant::new("MARKER", "COMPONENT_TYPE_MARKER", ComponentType::MARKER), crate::meta::inspect::EnumConstant::new("ANCHOR", "COMPONENT_TYPE_ANCHOR", ComponentType::ANCHOR), crate::meta::inspect::EnumConstant::new("PERSISTENCE", "COMPONENT_TYPE_PERSISTENCE", ComponentType::PERSISTENCE)]
        }
    }
}
impl crate::meta::GodotConvert for ComponentType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Component Type Bounded 2d", 1i64), EnumeratorShape::new_int("Component Type Bounded 3d", 2i64), EnumeratorShape::new_int("Component Type Parent", 3i64), EnumeratorShape::new_int("Component Type Mesh 3d", 4i64), EnumeratorShape::new_int("Component Type Plane Alignment", 1000741000i64), EnumeratorShape::new_int("Component Type Mesh 2d", 1000741001i64), EnumeratorShape::new_int("Component Type Polygon 2d", 1000741002i64), EnumeratorShape::new_int("Component Type Plane Semantic Label", 1000741003i64), EnumeratorShape::new_int("Component Type Marker", 1000743000i64), EnumeratorShape::new_int("Component Type Anchor", 1000762000i64), EnumeratorShape::new_int("Component Type Persistence", 1000763000i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("OpenXRSpatialEntityExtension.ComponentType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ComponentType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ComponentType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ComponentType {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for ComponentType {
    
}
impl crate::meta::Element for ComponentType {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrSpatialEntityExtension;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`OpenXrSpatialEntityExtension`][crate::classes::OpenXrSpatialEntityExtension] class."]
    pub struct SignalsOfOpenXrSpatialEntityExtension < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfOpenXrSpatialEntityExtension < 'c, C > {
        #[doc = "Signature: `(spatial_context: Rid)`"]
        pub fn spatial_discovery_recommended(&mut self) -> SigSpatialDiscoveryRecommended < 'c, C > {
            SigSpatialDiscoveryRecommended {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "spatial_discovery_recommended")
            }
        }
    }
    type TypedSigSpatialDiscoveryRecommended < 'c, C > = TypedSignal < 'c, C, (Rid,) >;
    pub struct SigSpatialDiscoveryRecommended < 'c, C: WithSignals > {
        typed: TypedSigSpatialDiscoveryRecommended < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSpatialDiscoveryRecommended < 'c, C > {
        pub fn emit(&mut self, spatial_context: Rid,) {
            self.typed.emit_tuple((spatial_context,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSpatialDiscoveryRecommended < 'c, C > {
        type Target = TypedSigSpatialDiscoveryRecommended < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSpatialDiscoveryRecommended < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for OpenXrSpatialEntityExtension {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfOpenXrSpatialEntityExtension < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfOpenXrSpatialEntityExtension < 'c, C > {
        type Target = < < OpenXrSpatialEntityExtension as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = OpenXrSpatialEntityExtension;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfOpenXrSpatialEntityExtension < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = OpenXrSpatialEntityExtension;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
#![doc = "Sidecar module for class [`OpenXrSpatialAnchorCapability`][crate::classes::OpenXrSpatialAnchorCapability].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRSpatialAnchorCapability` enums](https://docs.godotengine.org/en/stable/classes/class_openxrspatialanchorcapability.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRSpatialAnchorCapability`.\n\nInherits [`OpenXrExtensionWrapper`][crate::classes::OpenXrExtensionWrapper].\n\nRelated symbols:\n\n* [`open_xr_spatial_anchor_capability`][crate::classes::open_xr_spatial_anchor_capability]: sidecar module with related enum/flag types\n* [`IOpenXrSpatialAnchorCapability`][crate::classes::IOpenXrSpatialAnchorCapability]: virtual methods\n\n\nSee also [Godot docs for `OpenXRSpatialAnchorCapability`](https://docs.godotengine.org/en/stable/classes/class_openxrspatialanchorcapability.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`OpenXrSpatialAnchorCapability::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nThis is an internal class that handles the OpenXR anchor spatial entity extension."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrSpatialAnchorCapability {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrSpatialAnchorCapability`][crate::classes::OpenXrSpatialAnchorCapability].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IOpenXrExtensionWrapper`][crate::classes::IOpenXrExtensionWrapper] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `OpenXRSpatialAnchorCapability` methods](https://docs.godotengine.org/en/stable/classes/class_openxrspatialanchorcapability.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrSpatialAnchorCapability: crate::obj::GodotClass < Base = OpenXrSpatialAnchorCapability > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OpenXrSpatialAnchorCapability {
        #[doc = "Returns `true` if spatial anchors are supported by the hardware. Only returns a valid value after OpenXR has been initialized."]
        pub fn is_spatial_anchor_supported(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialAnchorCapability", "is_spatial_anchor_supported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if persistent spatial anchors are supported by the hardware. Only returns a valid value after OpenXR has been initialized."]
        pub fn is_spatial_persistence_supported(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialAnchorCapability", "is_spatial_persistence_supported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this persistence scope is supported by our spatial anchor capability.\n\n**Note:** Only valid after an OpenXR instance has been created."]
        pub fn is_persistence_scope_supported(&self, scope: crate::classes::open_xr_spatial_anchor_capability::PersistenceScope,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::open_xr_spatial_anchor_capability::PersistenceScope,);
            let args = (scope,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialAnchorCapability", "is_persistence_scope_supported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new persistence context for storing persistent data.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the creation process. On success `user_callback` will be called if specified. The result value for this function is the [`RID`][crate::builtin::Rid] for our persistence context."]
        pub(crate) fn create_persistence_context_full(&mut self, scope: crate::classes::open_xr_spatial_anchor_capability::PersistenceScope, user_callback: RefArg < Callable >,) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
            type CallRet = Option < Gd < crate::classes::OpenXrFutureResult > >;
            type CallParams < 'a0, > = (crate::classes::open_xr_spatial_anchor_capability::PersistenceScope, RefArg < 'a0, Callable >,);
            let args = (scope, user_callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialAnchorCapability", "create_persistence_context", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_persistence_context_ex`][Self::create_persistence_context_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new persistence context for storing persistent data.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the creation process. On success `user_callback` will be called if specified. The result value for this function is the [`RID`][crate::builtin::Rid] for our persistence context."]
        #[inline]
        pub fn create_persistence_context(&mut self, scope: crate::classes::open_xr_spatial_anchor_capability::PersistenceScope,) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
            self.create_persistence_context_ex(scope,) . done()
        }
        #[doc = "Creates a new persistence context for storing persistent data.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the creation process. On success `user_callback` will be called if specified. The result value for this function is the [`RID`][crate::builtin::Rid] for our persistence context."]
        #[inline]
        pub fn create_persistence_context_ex < 'ex > (&'ex mut self, scope: crate::classes::open_xr_spatial_anchor_capability::PersistenceScope,) -> ExCreatePersistenceContext < 'ex > {
            ExCreatePersistenceContext::new(self, scope,)
        }
        #[doc = "Returns the internal handle for this persistence context.\n\n**Note:** For GDExtension implementations."]
        pub fn get_persistence_context_handle(&self, persistence_context: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (persistence_context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialAnchorCapability", "get_persistence_context_handle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Frees a persistence context previously created with [`create_persistence_context`][`crate::classes::OpenXrSpatialAnchorCapability::create_persistence_context`]."]
        pub fn free_persistence_context(&mut self, persistence_context: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (persistence_context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialAnchorCapability", "free_persistence_context", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new anchor that will be tracked by the XR runtime. The `transform` should be a transform in the local space of your [`XROrigin3D`][crate::classes::XrOrigin3D] node. If `spatial_context` is not specified the default will be used, this requires \\[member ProjectSettings.xr/openxr/extensions/spatial_entity/enable_builtin_anchor_detection] to be set. The returned tracker will track the location in case our reference space changes."]
        pub(crate) fn create_new_anchor_full(&mut self, transform: Transform3D, spatial_context: Rid,) -> Option < Gd < crate::classes::OpenXrAnchorTracker > > {
            type CallRet = Option < Gd < crate::classes::OpenXrAnchorTracker > >;
            type CallParams = (Transform3D, Rid,);
            let args = (transform, spatial_context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialAnchorCapability", "create_new_anchor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_new_anchor_ex`][Self::create_new_anchor_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new anchor that will be tracked by the XR runtime. The `transform` should be a transform in the local space of your [`XROrigin3D`][crate::classes::XrOrigin3D] node. If `spatial_context` is not specified the default will be used, this requires \\[member ProjectSettings.xr/openxr/extensions/spatial_entity/enable_builtin_anchor_detection] to be set. The returned tracker will track the location in case our reference space changes."]
        #[inline]
        pub fn create_new_anchor(&mut self, transform: Transform3D,) -> Option < Gd < crate::classes::OpenXrAnchorTracker > > {
            self.create_new_anchor_ex(transform,) . done()
        }
        #[doc = "Creates a new anchor that will be tracked by the XR runtime. The `transform` should be a transform in the local space of your [`XROrigin3D`][crate::classes::XrOrigin3D] node. If `spatial_context` is not specified the default will be used, this requires \\[member ProjectSettings.xr/openxr/extensions/spatial_entity/enable_builtin_anchor_detection] to be set. The returned tracker will track the location in case our reference space changes."]
        #[inline]
        pub fn create_new_anchor_ex < 'ex > (&'ex mut self, transform: Transform3D,) -> ExCreateNewAnchor < 'ex > {
            ExCreateNewAnchor::new(self, transform,)
        }
        #[doc = "Remove an anchor previously created with [`create_new_anchor`][`crate::classes::OpenXrSpatialAnchorCapability::create_new_anchor`]. If this anchor was persistent you must first call [`unpersist_anchor`][`crate::classes::OpenXrSpatialAnchorCapability::unpersist_anchor`] and await its callback."]
        pub fn remove_anchor(&mut self, anchor_tracker: impl AsArg < Option < Gd < crate::classes::OpenXrAnchorTracker >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrAnchorTracker > > >,);
            let args = (anchor_tracker.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialAnchorCapability", "remove_anchor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Changes this anchor into a persistent anchor. This means its location will be stored on the device and the anchor will be restored the next time your application starts. If `persistence_context` is not specified the default will be used, this requires \\[member ProjectSettings.xr/openxr/extensions/spatial_entity/enable_builtin_anchor_detection] to be set.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the creation process. On success `user_callback` will be called if specified. The result value for this function is a boolean which will be set to `true` on successful completion."]
        pub(crate) fn persist_anchor_full(&mut self, anchor_tracker: CowArg < Option < Gd < crate::classes::OpenXrAnchorTracker > > >, persistence_context: Rid, user_callback: RefArg < Callable >,) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
            type CallRet = Option < Gd < crate::classes::OpenXrFutureResult > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrAnchorTracker > > >, Rid, RefArg < 'a1, Callable >,);
            let args = (anchor_tracker, persistence_context, user_callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialAnchorCapability", "persist_anchor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`persist_anchor_ex`][Self::persist_anchor_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Changes this anchor into a persistent anchor. This means its location will be stored on the device and the anchor will be restored the next time your application starts. If `persistence_context` is not specified the default will be used, this requires \\[member ProjectSettings.xr/openxr/extensions/spatial_entity/enable_builtin_anchor_detection] to be set.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the creation process. On success `user_callback` will be called if specified. The result value for this function is a boolean which will be set to `true` on successful completion."]
        #[inline]
        pub fn persist_anchor(&mut self, anchor_tracker: impl AsArg < Option < Gd < crate::classes::OpenXrAnchorTracker >> >,) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
            self.persist_anchor_ex(anchor_tracker,) . done()
        }
        #[doc = "Changes this anchor into a persistent anchor. This means its location will be stored on the device and the anchor will be restored the next time your application starts. If `persistence_context` is not specified the default will be used, this requires \\[member ProjectSettings.xr/openxr/extensions/spatial_entity/enable_builtin_anchor_detection] to be set.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the creation process. On success `user_callback` will be called if specified. The result value for this function is a boolean which will be set to `true` on successful completion."]
        #[inline]
        pub fn persist_anchor_ex < 'ex > (&'ex mut self, anchor_tracker: impl AsArg < Option < Gd < crate::classes::OpenXrAnchorTracker >> > + 'ex,) -> ExPersistAnchor < 'ex > {
            ExPersistAnchor::new(self, anchor_tracker,)
        }
        #[doc = "Removes the persistent data from this anchor. The runtime will not recreate the anchor when your application restarts. If `persistence_context` is not specified the default will be used, this requires \\[member ProjectSettings.xr/openxr/extensions/spatial_entity/enabled] to be set.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the creation process. On success `user_callback` will be called if specified. The result value for this function is a boolean which will be set to `true` on successful completion."]
        pub(crate) fn unpersist_anchor_full(&mut self, anchor_tracker: CowArg < Option < Gd < crate::classes::OpenXrAnchorTracker > > >, persistence_context: Rid, user_callback: RefArg < Callable >,) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
            type CallRet = Option < Gd < crate::classes::OpenXrFutureResult > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrAnchorTracker > > >, Rid, RefArg < 'a1, Callable >,);
            let args = (anchor_tracker, persistence_context, user_callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialAnchorCapability", "unpersist_anchor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`unpersist_anchor_ex`][Self::unpersist_anchor_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Removes the persistent data from this anchor. The runtime will not recreate the anchor when your application restarts. If `persistence_context` is not specified the default will be used, this requires \\[member ProjectSettings.xr/openxr/extensions/spatial_entity/enabled] to be set.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the creation process. On success `user_callback` will be called if specified. The result value for this function is a boolean which will be set to `true` on successful completion."]
        #[inline]
        pub fn unpersist_anchor(&mut self, anchor_tracker: impl AsArg < Option < Gd < crate::classes::OpenXrAnchorTracker >> >,) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
            self.unpersist_anchor_ex(anchor_tracker,) . done()
        }
        #[doc = "Removes the persistent data from this anchor. The runtime will not recreate the anchor when your application restarts. If `persistence_context` is not specified the default will be used, this requires \\[member ProjectSettings.xr/openxr/extensions/spatial_entity/enabled] to be set.\n\n**Note:** This is an asynchronous method and returns an [`OpenXRFutureResult`][crate::classes::OpenXrFutureResult] object with which to track the status, discarding this object will not cancel the creation process. On success `user_callback` will be called if specified. The result value for this function is a boolean which will be set to `true` on successful completion."]
        #[inline]
        pub fn unpersist_anchor_ex < 'ex > (&'ex mut self, anchor_tracker: impl AsArg < Option < Gd < crate::classes::OpenXrAnchorTracker >> > + 'ex,) -> ExUnpersistAnchor < 'ex > {
            ExUnpersistAnchor::new(self, anchor_tracker,)
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
    impl crate::obj::GodotClass for OpenXrSpatialAnchorCapability {
        type Base = crate::classes::OpenXrExtensionWrapper;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRSpatialAnchorCapability"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrSpatialAnchorCapability {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::OpenXrExtensionWrapper > for OpenXrSpatialAnchorCapability {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrSpatialAnchorCapability {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrSpatialAnchorCapability {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrSpatialAnchorCapability {
        type Target = crate::classes::OpenXrExtensionWrapper;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrSpatialAnchorCapability {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrSpatialAnchorCapability`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrSpatialAnchorCapability__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrSpatialAnchorCapability > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrExtensionWrapper > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`OpenXrSpatialAnchorCapability::create_persistence_context_ex`][super::OpenXrSpatialAnchorCapability::create_persistence_context_ex]."]
#[must_use]
pub struct ExCreatePersistenceContext < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::OpenXrSpatialAnchorCapability, scope: crate::classes::open_xr_spatial_anchor_capability::PersistenceScope, user_callback: CowArg < 'ex, Callable >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreatePersistenceContext < 'ex > {
    fn new(surround_object: &'ex mut re_export::OpenXrSpatialAnchorCapability, scope: crate::classes::open_xr_spatial_anchor_capability::PersistenceScope,) -> Self {
        let user_callback = Callable::invalid();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, scope: scope, user_callback: CowArg::Owned(user_callback),
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
            _phantom, surround_object, scope, user_callback,
        }
        = self;
        re_export::OpenXrSpatialAnchorCapability::create_persistence_context_full(surround_object, scope, user_callback.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`OpenXrSpatialAnchorCapability::create_new_anchor_ex`][super::OpenXrSpatialAnchorCapability::create_new_anchor_ex]."]
#[must_use]
pub struct ExCreateNewAnchor < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::OpenXrSpatialAnchorCapability, transform: Transform3D, spatial_context: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateNewAnchor < 'ex > {
    fn new(surround_object: &'ex mut re_export::OpenXrSpatialAnchorCapability, transform: Transform3D,) -> Self {
        let spatial_context = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, transform: transform, spatial_context: spatial_context,
        }
    }
    #[inline]
    pub fn spatial_context(self, spatial_context: Rid) -> Self {
        Self {
            spatial_context: spatial_context, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::OpenXrAnchorTracker > > {
        let Self {
            _phantom, surround_object, transform, spatial_context,
        }
        = self;
        re_export::OpenXrSpatialAnchorCapability::create_new_anchor_full(surround_object, transform, spatial_context,)
    }
}
#[doc = "Default-param extender for [`OpenXrSpatialAnchorCapability::persist_anchor_ex`][super::OpenXrSpatialAnchorCapability::persist_anchor_ex]."]
#[must_use]
pub struct ExPersistAnchor < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::OpenXrSpatialAnchorCapability, anchor_tracker: CowArg < 'ex, Option < Gd < crate::classes::OpenXrAnchorTracker > > >, persistence_context: Rid, user_callback: CowArg < 'ex, Callable >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPersistAnchor < 'ex > {
    fn new(surround_object: &'ex mut re_export::OpenXrSpatialAnchorCapability, anchor_tracker: impl AsArg < Option < Gd < crate::classes::OpenXrAnchorTracker >> > + 'ex,) -> Self {
        let persistence_context = Rid::Invalid;
        let user_callback = Callable::invalid();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, anchor_tracker: anchor_tracker.into_arg(), persistence_context: persistence_context, user_callback: CowArg::Owned(user_callback),
        }
    }
    #[inline]
    pub fn persistence_context(self, persistence_context: Rid) -> Self {
        Self {
            persistence_context: persistence_context, .. self
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
            _phantom, surround_object, anchor_tracker, persistence_context, user_callback,
        }
        = self;
        re_export::OpenXrSpatialAnchorCapability::persist_anchor_full(surround_object, anchor_tracker, persistence_context, user_callback.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`OpenXrSpatialAnchorCapability::unpersist_anchor_ex`][super::OpenXrSpatialAnchorCapability::unpersist_anchor_ex]."]
#[must_use]
pub struct ExUnpersistAnchor < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::OpenXrSpatialAnchorCapability, anchor_tracker: CowArg < 'ex, Option < Gd < crate::classes::OpenXrAnchorTracker > > >, persistence_context: Rid, user_callback: CowArg < 'ex, Callable >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExUnpersistAnchor < 'ex > {
    fn new(surround_object: &'ex mut re_export::OpenXrSpatialAnchorCapability, anchor_tracker: impl AsArg < Option < Gd < crate::classes::OpenXrAnchorTracker >> > + 'ex,) -> Self {
        let persistence_context = Rid::Invalid;
        let user_callback = Callable::invalid();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, anchor_tracker: anchor_tracker.into_arg(), persistence_context: persistence_context, user_callback: CowArg::Owned(user_callback),
        }
    }
    #[inline]
    pub fn persistence_context(self, persistence_context: Rid) -> Self {
        Self {
            persistence_context: persistence_context, .. self
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
            _phantom, surround_object, anchor_tracker, persistence_context, user_callback,
        }
        = self;
        re_export::OpenXrSpatialAnchorCapability::unpersist_anchor_full(surround_object, anchor_tracker, persistence_context, user_callback.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PersistenceScope {
    ord: i32
}
impl PersistenceScope {
    #[doc(alias = "PERSISTENCE_SCOPE_SYSTEM_MANAGED")]
    #[doc = "Godot enumerator name: `PERSISTENCE_SCOPE_SYSTEM_MANAGED`"]
    pub const SYSTEM_MANAGED: PersistenceScope = PersistenceScope {
        ord: 1i32
    };
    #[doc(alias = "PERSISTENCE_SCOPE_LOCAL_ANCHORS")]
    #[doc = "Godot enumerator name: `PERSISTENCE_SCOPE_LOCAL_ANCHORS`"]
    pub const LOCAL_ANCHORS: PersistenceScope = PersistenceScope {
        ord: 1000781000i32
    };
    
}
impl std::fmt::Debug for PersistenceScope {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PersistenceScope") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PersistenceScope {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 1000781000i32 => Some(Self {
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
            Self::SYSTEM_MANAGED => "SYSTEM_MANAGED", Self::LOCAL_ANCHORS => "LOCAL_ANCHORS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PersistenceScope::SYSTEM_MANAGED, PersistenceScope::LOCAL_ANCHORS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PersistenceScope >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SYSTEM_MANAGED", "PERSISTENCE_SCOPE_SYSTEM_MANAGED", PersistenceScope::SYSTEM_MANAGED), crate::meta::inspect::EnumConstant::new("LOCAL_ANCHORS", "PERSISTENCE_SCOPE_LOCAL_ANCHORS", PersistenceScope::LOCAL_ANCHORS)]
        }
    }
}
impl crate::meta::GodotConvert for PersistenceScope {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Persistence Scope System Managed", 1i64), EnumeratorShape::new_int("Persistence Scope Local Anchors", 1000781000i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("OpenXRSpatialAnchorCapability.PersistenceScope")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PersistenceScope {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PersistenceScope {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PersistenceScope {
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
impl crate::registry::property::Export for PersistenceScope {
    
}
impl crate::meta::Element for PersistenceScope {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrSpatialAnchorCapability;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for OpenXrSpatialAnchorCapability {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
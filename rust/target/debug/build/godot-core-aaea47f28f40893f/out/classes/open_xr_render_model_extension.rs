#![doc = "Sidecar module for class [`OpenXrRenderModelExtension`][crate::classes::OpenXrRenderModelExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRRenderModelExtension` enums](https://docs.godotengine.org/en/stable/classes/class_openxrrendermodelextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRRenderModelExtension`.\n\nInherits [`OpenXrExtensionWrapper`][crate::classes::OpenXrExtensionWrapper].\n\nRelated symbols:\n\n* [`open_xr_render_model_extension`][crate::classes::open_xr_render_model_extension]: sidecar module with related enum/flag types\n* [`IOpenXrRenderModelExtension`][crate::classes::IOpenXrRenderModelExtension]: virtual methods\n* [`SignalsOfOpenXrRenderModelExtension`][crate::classes::open_xr_render_model_extension::SignalsOfOpenXrRenderModelExtension]: signal collection\n\n\nSee also [Godot docs for `OpenXRRenderModelExtension`](https://docs.godotengine.org/en/stable/classes/class_openxrrendermodelextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`OpenXrRenderModelExtension::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nThis class implements the OpenXR Render Model Extension, if enabled it will maintain a list of active render models and provides an interface to the render model data."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrRenderModelExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrRenderModelExtension`][crate::classes::OpenXrRenderModelExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IOpenXrExtensionWrapper`][crate::classes::IOpenXrExtensionWrapper] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `OpenXRRenderModelExtension` methods](https://docs.godotengine.org/en/stable/classes/class_openxrrendermodelextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrRenderModelExtension: crate::obj::GodotClass < Base = OpenXrRenderModelExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OpenXrRenderModelExtension {
        #[doc = "Returns `true` if OpenXR's render model extension is supported and enabled.\n\n**Note:** This only returns a valid value after OpenXR has been initialized."]
        pub fn is_active(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7043usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "is_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a render model object within OpenXR using a render model id.\n\n**Note:** This function is exposed for dependent OpenXR extensions that provide render model ids to be used with the render model extension."]
        pub fn render_model_create(&mut self, render_model_id: u64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (u64,);
            let args = (render_model_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7044usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "render_model_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Destroys a render model object within OpenXR that was previously created with [`render_model_create`][`crate::classes::OpenXrRenderModelExtension::render_model_create`].\n\n**Note:** This function is exposed for dependent OpenXR extensions that provide render model ids to be used with the render model extension."]
        pub fn render_model_destroy(&mut self, render_model: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (render_model,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7045usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "render_model_destroy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all currently active render models registered with this extension."]
        pub fn render_model_get_all(&mut self,) -> Array < Rid > {
            type CallRet = Array < Rid >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7046usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "render_model_get_all", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an instance of a subscene that contains all [`MeshInstance3D`][crate::classes::MeshInstance3D] nodes that allow you to visualize the render model."]
        pub fn render_model_new_scene_instance(&self, render_model: Rid,) -> Option < Gd < crate::classes::Node3D > > {
            type CallRet = Option < Gd < crate::classes::Node3D > >;
            type CallParams = (Rid,);
            let args = (render_model,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7047usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "render_model_new_scene_instance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of active subaction paths for this `render_model`.\n\n**Note:** If different devices are bound to your actions than available in suggested interaction bindings, this information shows paths related to the interaction bindings being mimicked by that device."]
        pub fn render_model_get_subaction_paths(&mut self, render_model: Rid,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (Rid,);
            let args = (render_model,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7048usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "render_model_get_subaction_paths", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the top level path associated with this `render_model`. If provided this identifies whether the render model is associated with the player's hands or other body part."]
        pub fn render_model_get_top_level_path(&self, render_model: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (render_model,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7049usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "render_model_get_top_level_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tracking confidence of the tracking data for the render model."]
        pub fn render_model_get_confidence(&self, render_model: Rid,) -> crate::classes::xr_pose::TrackingConfidence {
            type CallRet = crate::classes::xr_pose::TrackingConfidence;
            type CallParams = (Rid,);
            let args = (render_model,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7050usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "render_model_get_confidence", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the root transform of a render model. This is the tracked position relative to our [`XROrigin3D`][crate::classes::XrOrigin3D] node."]
        pub fn render_model_get_root_transform(&self, render_model: Rid,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (Rid,);
            let args = (render_model,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7051usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "render_model_get_root_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of animatable nodes this render model has."]
        pub fn render_model_get_animatable_node_count(&self, render_model: Rid,) -> u32 {
            type CallRet = u32;
            type CallParams = (Rid,);
            let args = (render_model,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7052usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "render_model_get_animatable_node_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the given animatable node."]
        pub fn render_model_get_animatable_node_name(&self, render_model: Rid, index: u32,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid, u32,);
            let args = (render_model, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7053usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "render_model_get_animatable_node_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this animatable node should be visible."]
        pub fn render_model_is_animatable_node_visible(&self, render_model: Rid, index: u32,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, u32,);
            let args = (render_model, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7054usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "render_model_is_animatable_node_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current local transform for an animatable node. This is updated every frame."]
        pub fn render_model_get_animatable_node_transform(&self, render_model: Rid, index: u32,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (Rid, u32,);
            let args = (render_model, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7055usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrRenderModelExtension", "render_model_get_animatable_node_transform", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrRenderModelExtension {
        type Base = crate::classes::OpenXrExtensionWrapper;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRRenderModelExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrRenderModelExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::OpenXrExtensionWrapper > for OpenXrRenderModelExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrRenderModelExtension {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrRenderModelExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrRenderModelExtension {
        type Target = crate::classes::OpenXrExtensionWrapper;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrRenderModelExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrRenderModelExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrRenderModelExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrRenderModelExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrExtensionWrapper > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrRenderModelExtension;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`OpenXrRenderModelExtension`][crate::classes::OpenXrRenderModelExtension] class."]
    pub struct SignalsOfOpenXrRenderModelExtension < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfOpenXrRenderModelExtension < 'c, C > {
        #[doc = "Signature: `(render_model: Rid)`"]
        pub fn render_model_added(&mut self) -> SigRenderModelAdded < 'c, C > {
            SigRenderModelAdded {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "render_model_added")
            }
        }
        #[doc = "Signature: `(render_model: Rid)`"]
        pub fn render_model_removed(&mut self) -> SigRenderModelRemoved < 'c, C > {
            SigRenderModelRemoved {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "render_model_removed")
            }
        }
        #[doc = "Signature: `(render_model: Rid)`"]
        pub fn render_model_top_level_path_changed(&mut self) -> SigRenderModelTopLevelPathChanged < 'c, C > {
            SigRenderModelTopLevelPathChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "render_model_top_level_path_changed")
            }
        }
    }
    type TypedSigRenderModelAdded < 'c, C > = TypedSignal < 'c, C, (Rid,) >;
    pub struct SigRenderModelAdded < 'c, C: WithSignals > {
        typed: TypedSigRenderModelAdded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRenderModelAdded < 'c, C > {
        pub fn emit(&mut self, render_model: Rid,) {
            self.typed.emit_tuple((render_model,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRenderModelAdded < 'c, C > {
        type Target = TypedSigRenderModelAdded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRenderModelAdded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigRenderModelRemoved < 'c, C > = TypedSignal < 'c, C, (Rid,) >;
    pub struct SigRenderModelRemoved < 'c, C: WithSignals > {
        typed: TypedSigRenderModelRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRenderModelRemoved < 'c, C > {
        pub fn emit(&mut self, render_model: Rid,) {
            self.typed.emit_tuple((render_model,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRenderModelRemoved < 'c, C > {
        type Target = TypedSigRenderModelRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRenderModelRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigRenderModelTopLevelPathChanged < 'c, C > = TypedSignal < 'c, C, (Rid,) >;
    pub struct SigRenderModelTopLevelPathChanged < 'c, C: WithSignals > {
        typed: TypedSigRenderModelTopLevelPathChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRenderModelTopLevelPathChanged < 'c, C > {
        pub fn emit(&mut self, render_model: Rid,) {
            self.typed.emit_tuple((render_model,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRenderModelTopLevelPathChanged < 'c, C > {
        type Target = TypedSigRenderModelTopLevelPathChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRenderModelTopLevelPathChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for OpenXrRenderModelExtension {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfOpenXrRenderModelExtension < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfOpenXrRenderModelExtension < 'c, C > {
        type Target = < < OpenXrRenderModelExtension as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = OpenXrRenderModelExtension;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfOpenXrRenderModelExtension < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = OpenXrRenderModelExtension;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
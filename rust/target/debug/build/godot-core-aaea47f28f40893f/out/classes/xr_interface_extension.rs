#![doc = "Sidecar module for class [`XrInterfaceExtension`][crate::classes::XrInterfaceExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRInterfaceExtension` enums](https://docs.godotengine.org/en/stable/classes/class_xrinterfaceextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRInterfaceExtension`.\n\nInherits [`XrInterface`][crate::classes::XrInterface].\n\nRelated symbols:\n\n* [`IXrInterfaceExtension`][crate::classes::IXrInterfaceExtension]: virtual methods\n\n\nSee also [Godot docs for `XRInterfaceExtension`](https://docs.godotengine.org/en/stable/classes/class_xrinterfaceextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`XrInterfaceExtension::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nExternal XR interface plugins should inherit from this class."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrInterfaceExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`XrInterfaceExtension`][crate::classes::XrInterfaceExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IXrInterface`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `XRInterfaceExtension` methods](https://docs.godotengine.org/en/stable/classes/class_xrinterfaceextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IXrInterfaceExtension: crate::obj::GodotClass < Base = XrInterfaceExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Called if this `XRInterfaceExtension` is active before our physics and game process is called. Most XR interfaces will update its [`XRPositionalTracker`][crate::classes::XrPositionalTracker]s at this point in time."]
        fn process(&mut self,) {
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
        #[doc = "Returns the name of this interface."]
        fn get_name(&self,) -> StringName {
            unimplemented !()
        }
        #[doc = "Returns the capabilities of this interface."]
        fn get_capabilities(&self,) -> u32 {
            unimplemented !()
        }
        #[doc = "Returns `true` if this interface has been initialized."]
        fn is_initialized(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Initializes the interface, returns `true` on success."]
        fn initialize(&mut self,) -> bool {
            unimplemented !()
        }
        #[doc = "Uninitialize the interface."]
        fn uninitialize(&mut self,) {
            unimplemented !()
        }
        #[doc = "Returns a [`Dictionary`][crate::builtin::Dictionary] with system information related to this interface."]
        fn get_system_info(&self,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "Returns `true` if this interface supports this play area mode."]
        fn supports_play_area_mode(&self, mode: crate::classes::xr_interface::PlayAreaMode,) -> bool {
            unimplemented !()
        }
        #[doc = "Returns the play area mode that sets up our play area."]
        fn get_play_area_mode(&self,) -> crate::classes::xr_interface::PlayAreaMode {
            unimplemented !()
        }
        #[doc = "Set the play area mode for this interface."]
        fn set_play_area_mode(&self, mode: crate::classes::xr_interface::PlayAreaMode,) -> bool {
            unimplemented !()
        }
        #[doc = "Returns a [`PackedVector3Array`][crate::builtin::PackedVector3Array] that represents the play areas boundaries (if applicable)."]
        fn get_play_area(&self,) -> PackedVector3Array {
            unimplemented !()
        }
        #[doc = "Returns the size of our render target for this interface, this overrides the size of the [`Viewport`][crate::classes::Viewport] marked as the xr viewport."]
        fn get_render_target_size(&mut self,) -> Vector2 {
            unimplemented !()
        }
        #[doc = "Returns the number of views this interface requires, 1 for mono, 2 for stereoscopic."]
        fn get_view_count(&mut self,) -> u32 {
            unimplemented !()
        }
        #[doc = "Returns the [`Transform3D`][crate::builtin::Transform3D] that positions the [`XRCamera3D`][crate::classes::XrCamera3D] in the world."]
        fn get_camera_transform(&mut self,) -> Transform3D {
            unimplemented !()
        }
        #[doc = "Returns a [`Transform3D`][crate::builtin::Transform3D] for a given view."]
        fn get_transform_for_view(&mut self, view: u32, cam_transform: Transform3D,) -> Transform3D {
            unimplemented !()
        }
        #[doc = "Returns the projection matrix for the given view as a [`PackedFloat64Array`][crate::builtin::PackedFloat64Array]."]
        fn get_projection_for_view(&mut self, view: u32, aspect: f64, z_near: f64, z_far: f64,) -> PackedFloat64Array {
            unimplemented !()
        }
        fn get_vrs_texture(&mut self,) -> Rid {
            unimplemented !()
        }
        #[doc = "Returns the format of the texture returned by [`get_vrs_texture`][`crate::classes::IXrInterfaceExtension::get_vrs_texture`]."]
        fn get_vrs_texture_format(&mut self,) -> crate::classes::xr_interface::VrsTextureFormat {
            unimplemented !()
        }
        #[doc = "Called if this `XRInterfaceExtension` is active before rendering starts. Most XR interfaces will sync tracking at this point in time."]
        fn pre_render(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called if this is our primary `XRInterfaceExtension` before we start processing a [`Viewport`][crate::classes::Viewport] for every active XR [`Viewport`][crate::classes::Viewport], returns `true` if that viewport should be rendered. An XR interface may return `false` if the user has taken off their headset and we can pause rendering."]
        fn pre_draw_viewport(&mut self, render_target: Rid,) -> bool {
            unimplemented !()
        }
        #[doc = "Called after the XR [`Viewport`][crate::classes::Viewport] draw logic has completed."]
        fn post_draw_viewport(&mut self, render_target: Rid, screen_rect: Rect2,) {
            unimplemented !()
        }
        #[doc = "Called if interface is active and queues have been submitted."]
        fn end_frame(&mut self,) {
            unimplemented !()
        }
        #[doc = "Returns a [`PackedStringArray`][crate::builtin::PackedStringArray] with tracker names configured by this interface. Note that user configuration can override this list."]
        fn get_suggested_tracker_names(&self,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Returns a [`PackedStringArray`][crate::builtin::PackedStringArray] with pose names configured by this interface. Note that user configuration can override this list."]
        fn get_suggested_pose_names(&self, tracker_name: StringName,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Returns the current status of our tracking."]
        fn get_tracking_status(&self,) -> crate::classes::xr_interface::TrackingStatus {
            unimplemented !()
        }
        #[doc = "Triggers a haptic pulse to be emitted on the specified tracker."]
        fn trigger_haptic_pulse(&mut self, action_name: GString, tracker_name: StringName, frequency: f64, amplitude: f64, duration_sec: f64, delay_sec: f64,) {
            unimplemented !()
        }
        #[doc = "Return `true` if anchor detection is enabled for this interface."]
        fn get_anchor_detection_is_enabled(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Enables anchor detection on this interface if supported."]
        fn set_anchor_detection_is_enabled(&mut self, enabled: bool,) {
            unimplemented !()
        }
        #[doc = "Returns the camera feed ID for the [`CameraFeed`][crate::classes::CameraFeed] registered with the [`CameraServer`][crate::classes::CameraServer] that should be presented as the background on an AR capable device (if applicable)."]
        fn get_camera_feed_id(&self,) -> i32 {
            unimplemented !()
        }
        #[doc = "Return color texture into which to render (if applicable)."]
        fn get_color_texture(&mut self,) -> Rid {
            unimplemented !()
        }
        #[doc = "Return depth texture into which to render (if applicable)."]
        fn get_depth_texture(&mut self,) -> Rid {
            unimplemented !()
        }
        #[doc = "Return velocity texture into which to render (if applicable)."]
        fn get_velocity_texture(&mut self,) -> Rid {
            unimplemented !()
        }
    }
    impl XrInterfaceExtension {
        pub fn get_color_texture(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterfaceExtension", "get_color_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_depth_texture(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterfaceExtension", "get_depth_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_velocity_texture(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterfaceExtension", "get_velocity_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Blits our render results to screen optionally applying lens distortion. This can only be called while processing `_commit_views`."]
        pub fn add_blit(&mut self, render_target: Rid, src_rect: Rect2, dst_rect: Rect2i, use_layer: bool, layer: u32, apply_lens_distortion: bool, eye_center: Vector2, k1: f64, k2: f64, upscale: f64, aspect_ratio: f64,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Rect2i, bool, u32, bool, Vector2, f64, f64, f64, f64,);
            let args = (render_target, src_rect, dst_rect, use_layer, layer, apply_lens_distortion, eye_center, k1, k2, upscale, aspect_ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterfaceExtension", "add_blit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a valid [`RID`][crate::builtin::Rid] for a texture to which we should render the current frame if supported by the interface."]
        pub fn get_render_target_texture(&self, render_target: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (render_target,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterfaceExtension", "get_render_target_texture", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for XrInterfaceExtension {
        type Base = crate::classes::XrInterface;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("XRInterfaceExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for XrInterfaceExtension {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrInterface > for XrInterfaceExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for XrInterfaceExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for XrInterfaceExtension {
        
    }
    impl crate::obj::cap::GodotDefault for XrInterfaceExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for XrInterfaceExtension {
        type Target = crate::classes::XrInterface;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrInterfaceExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`XrInterfaceExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_XrInterfaceExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrInterfaceExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrInterface > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
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
    use super::re_export::XrInterfaceExtension;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::xr_interface::SignalsOfXrInterface;
    impl WithSignals for XrInterfaceExtension {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfXrInterface < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
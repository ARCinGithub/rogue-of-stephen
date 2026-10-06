#![doc = "Sidecar module for class [`OpenXrMarkerTracker`][crate::classes::OpenXrMarkerTracker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRMarkerTracker` enums](https://docs.godotengine.org/en/stable/classes/class_openxrmarkertracker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRMarkerTracker`.\n\nInherits [`OpenXrSpatialEntityTracker`][crate::classes::OpenXrSpatialEntityTracker].\n\nRelated symbols:\n\n* [`IOpenXrMarkerTracker`][crate::classes::IOpenXrMarkerTracker]: virtual methods\n\n\nSee also [Godot docs for `OpenXRMarkerTracker`](https://docs.godotengine.org/en/stable/classes/class_openxrmarkertracker.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`OpenXrMarkerTracker::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nSpatial entity tracker for our OpenXR spatial entity marker tracking extension. These trackers identify entities in our real space detected by a visual marker such as a QRCode or Aruco code, and map their location to our virtual space."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrMarkerTracker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrMarkerTracker`][crate::classes::OpenXrMarkerTracker].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IOpenXrSpatialEntityTracker`][crate::classes::IOpenXrSpatialEntityTracker] > [`IXrPositionalTracker`][crate::classes::IXrPositionalTracker] > ~~`IXrTracker`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `OpenXRMarkerTracker` methods](https://docs.godotengine.org/en/stable/classes/class_openxrmarkertracker.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrMarkerTracker: crate::obj::GodotClass < Base = OpenXrMarkerTracker > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl OpenXrMarkerTracker {
        pub fn set_bounds_size(&mut self, bounds_size: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (bounds_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrMarkerTracker", "set_bounds_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bounds_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5718usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrMarkerTracker", "get_bounds_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_marker_type(&mut self, marker_type: crate::classes::open_xr_spatial_component_marker_list::MarkerType,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_spatial_component_marker_list::MarkerType,);
            let args = (marker_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrMarkerTracker", "set_marker_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_marker_type(&self,) -> crate::classes::open_xr_spatial_component_marker_list::MarkerType {
            type CallRet = crate::classes::open_xr_spatial_component_marker_list::MarkerType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5720usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrMarkerTracker", "get_marker_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_marker_id(&mut self, marker_id: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (marker_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrMarkerTracker", "set_marker_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_marker_id(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5722usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrMarkerTracker", "get_marker_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the marker data for this marker.\n\n**Note:** This should only be set by marker discovery logic."]
        pub fn set_marker_data(&mut self, marker_data: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (RefArg::new(marker_data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5723usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrMarkerTracker", "set_marker_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the marker data for this marker. This can return a [`String`][crate::builtin::GString] or [`PackedByteArray`][crate::builtin::PackedByteArray]. Only applicable to QR Code based markers."]
        pub fn get_marker_data(&self,) -> Variant {
            type CallRet = Variant;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrMarkerTracker", "get_marker_data", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrMarkerTracker {
        type Base = crate::classes::OpenXrSpatialEntityTracker;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRMarkerTracker"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrMarkerTracker {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::OpenXrSpatialEntityTracker > for OpenXrMarkerTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrPositionalTracker > for OpenXrMarkerTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrTracker > for OpenXrMarkerTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for OpenXrMarkerTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrMarkerTracker {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrMarkerTracker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrMarkerTracker {
        type Target = crate::classes::OpenXrSpatialEntityTracker;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrMarkerTracker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrMarkerTracker`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrMarkerTracker__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrMarkerTracker > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrSpatialEntityTracker > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrPositionalTracker > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrTracker > for $Class {
                
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
    use super::re_export::OpenXrMarkerTracker;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::open_xr_spatial_entity_tracker::SignalsOfOpenXrSpatialEntityTracker;
    impl WithSignals for OpenXrMarkerTracker {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfOpenXrSpatialEntityTracker < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
#![doc = "Sidecar module for class [`RenderSceneBuffersConfiguration`][crate::classes::RenderSceneBuffersConfiguration].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderSceneBuffersConfiguration` enums](https://docs.godotengine.org/en/stable/classes/class_renderscenebuffersconfiguration.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RenderSceneBuffersConfiguration`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`IRenderSceneBuffersConfiguration`][crate::classes::IRenderSceneBuffersConfiguration]: virtual methods\n\n\nSee also [Godot docs for `RenderSceneBuffersConfiguration`](https://docs.godotengine.org/en/stable/classes/class_renderscenebuffersconfiguration.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RenderSceneBuffersConfiguration::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThis configuration object is created and populated by the render engine on a viewport change and used to (re)configure a [`RenderSceneBuffers`][crate::classes::RenderSceneBuffers] object."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RenderSceneBuffersConfiguration {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`RenderSceneBuffersConfiguration`][crate::classes::RenderSceneBuffersConfiguration].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `RenderSceneBuffersConfiguration` methods](https://docs.godotengine.org/en/stable/classes/class_renderscenebuffersconfiguration.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRenderSceneBuffersConfiguration: crate::obj::GodotClass < Base = RenderSceneBuffersConfiguration > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RenderSceneBuffersConfiguration {
        pub fn get_render_target(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "get_render_target", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_render_target(&mut self, render_target: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (render_target,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "set_render_target", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_internal_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "get_internal_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_internal_size(&mut self, internal_size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (internal_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "set_internal_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_target_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "get_target_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_target_size(&mut self, target_size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (target_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "set_target_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_view_count(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6595usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "get_view_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_view_count(&mut self, view_count: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6596usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "set_view_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scaling_3d_mode(&self,) -> crate::classes::rendering_server::ViewportScaling3DMode {
            type CallRet = crate::classes::rendering_server::ViewportScaling3DMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6597usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "get_scaling_3d_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scaling_3d_mode(&mut self, scaling_3d_mode: crate::classes::rendering_server::ViewportScaling3DMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::ViewportScaling3DMode,);
            let args = (scaling_3d_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6598usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "set_scaling_3d_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_msaa_3d(&self,) -> crate::classes::rendering_server::ViewportMsaa {
            type CallRet = crate::classes::rendering_server::ViewportMsaa;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6599usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "get_msaa_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_msaa_3d(&mut self, msaa_3d: crate::classes::rendering_server::ViewportMsaa,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::ViewportMsaa,);
            let args = (msaa_3d,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6600usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "set_msaa_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_screen_space_aa(&self,) -> crate::classes::rendering_server::ViewportScreenSpaceAa {
            type CallRet = crate::classes::rendering_server::ViewportScreenSpaceAa;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6601usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "get_screen_space_aa", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_screen_space_aa(&mut self, screen_space_aa: crate::classes::rendering_server::ViewportScreenSpaceAa,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::ViewportScreenSpaceAa,);
            let args = (screen_space_aa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6602usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "set_screen_space_aa", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fsr_sharpness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6603usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "get_fsr_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fsr_sharpness(&mut self, fsr_sharpness: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (fsr_sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6604usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "set_fsr_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_mipmap_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6605usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "get_texture_mipmap_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_mipmap_bias(&mut self, texture_mipmap_bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (texture_mipmap_bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "set_texture_mipmap_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_anisotropic_filtering_level(&self,) -> crate::classes::rendering_server::ViewportAnisotropicFiltering {
            type CallRet = crate::classes::rendering_server::ViewportAnisotropicFiltering;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "get_anisotropic_filtering_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_anisotropic_filtering_level(&mut self, anisotropic_filtering_level: crate::classes::rendering_server::ViewportAnisotropicFiltering,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::ViewportAnisotropicFiltering,);
            let args = (anisotropic_filtering_level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersConfiguration", "set_anisotropic_filtering_level", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for RenderSceneBuffersConfiguration {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RenderSceneBuffersConfiguration"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RenderSceneBuffersConfiguration {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RenderSceneBuffersConfiguration {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RenderSceneBuffersConfiguration {
        
    }
    impl crate::obj::cap::GodotDefault for RenderSceneBuffersConfiguration {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RenderSceneBuffersConfiguration {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RenderSceneBuffersConfiguration {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RenderSceneBuffersConfiguration`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RenderSceneBuffersConfiguration__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RenderSceneBuffersConfiguration > for $Class {
                
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
    use super::re_export::RenderSceneBuffersConfiguration;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for RenderSceneBuffersConfiguration {
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
#![doc = "Sidecar module for class [`FogMaterial`][crate::classes::FogMaterial].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FogMaterial` enums](https://docs.godotengine.org/en/stable/classes/class_fogmaterial.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FogMaterial`.\n\nInherits [`Material`][crate::classes::Material].\n\nRelated symbols:\n\n* [`IFogMaterial`][crate::classes::IFogMaterial]: virtual methods\n\n\nSee also [Godot docs for `FogMaterial`](https://docs.godotengine.org/en/stable/classes/class_fogmaterial.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`FogMaterial::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA [`Material`][crate::classes::Material] resource that can be used by [`FogVolume`][crate::classes::FogVolume]s to draw volumetric effects.\n\nIf you need more advanced effects, use a custom [fog shader]($DOCS_URL/tutorials/shaders/shader_reference/fog_shader.html)."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FogMaterial {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`FogMaterial`][crate::classes::FogMaterial].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IMaterial`][crate::classes::IMaterial] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `FogMaterial` methods](https://docs.godotengine.org/en/stable/classes/class_fogmaterial.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFogMaterial: crate::obj::GodotClass < Base = FogMaterial > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Only exposed for the purpose of overriding. You cannot call this function directly. Used internally by various editor tools. Used to access the RID of the `Material`'s [`Shader`][crate::classes::Shader]."]
        fn get_shader_rid(&self,) -> Rid;
        #[doc = "Only exposed for the purpose of overriding. You cannot call this function directly. Used internally by various editor tools."]
        fn get_shader_mode(&self,) -> crate::classes::shader::Mode;
        #[doc = "Only exposed for the purpose of overriding. You cannot call this function directly. Used internally to determine if \\[member next_pass] should be shown in the editor or not."]
        fn can_do_next_pass(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Only exposed for the purpose of overriding. You cannot call this function directly. Used internally to determine if \\[member render_priority] should be shown in the editor or not."]
        fn can_use_render_priority(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Override this method to customize the newly duplicated resource created from [`instantiate`][`crate::classes::PackedScene::instantiate`], if the original's \\[member resource_local_to_scene] is set to `true`.\n\n**Example:** Set a random `damage` value to every local resource from an instantiated scene:\n\n```gdscript\nextends Resource\n\nvar damage = 0\n\nfunc _setup_local_to_scene():\n\tdamage = randi_range(10, 40)\n```"]
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to return a custom [`RID`][crate::builtin::Rid] when [`get_rid`][`crate::classes::Resource::get_rid`] is called."]
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        #[doc = "For resources that store state in non-exported properties, such as via [`on_validate_property`][`crate::classes::IObject::on_validate_property`] or [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`], this method must be implemented to clear them."]
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to execute additional logic after [`set_path_cache`][`crate::classes::Resource::set_path_cache`] is called on this object."]
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl FogMaterial {
        pub fn set_density(&mut self, density: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (density,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FogMaterial", "set_density", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_density(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FogMaterial", "get_density", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_albedo(&mut self, albedo: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (albedo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FogMaterial", "set_albedo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_albedo(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FogMaterial", "get_albedo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission(&mut self, emission: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (emission,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FogMaterial", "set_emission", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FogMaterial", "get_emission", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_height_falloff(&mut self, height_falloff: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (height_falloff,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FogMaterial", "set_height_falloff", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_height_falloff(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FogMaterial", "get_height_falloff", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_edge_fade(&mut self, edge_fade: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (edge_fade,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FogMaterial", "set_edge_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_edge_fade(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FogMaterial", "get_edge_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_density_texture(&mut self, density_texture: impl AsArg < Option < Gd < crate::classes::Texture3D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture3D > > >,);
            let args = (density_texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FogMaterial", "set_density_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_density_texture(&self,) -> Option < Gd < crate::classes::Texture3D > > {
            type CallRet = Option < Gd < crate::classes::Texture3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FogMaterial", "get_density_texture", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for FogMaterial {
        type Base = crate::classes::Material;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("FogMaterial"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FogMaterial {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Material > for FogMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for FogMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for FogMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FogMaterial {
        
    }
    impl crate::obj::cap::GodotDefault for FogMaterial {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for FogMaterial {
        type Target = crate::classes::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FogMaterial {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`FogMaterial`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_FogMaterial__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::FogMaterial > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Material > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
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
    use super::re_export::FogMaterial;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for FogMaterial {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
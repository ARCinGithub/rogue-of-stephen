#![doc = "Sidecar module for class [`NoiseTexture3D`][crate::classes::NoiseTexture3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `NoiseTexture3D` enums](https://docs.godotengine.org/en/stable/classes/class_noisetexture3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `NoiseTexture3D`.\n\nInherits [`Texture3D`][crate::classes::Texture3D].\n\nRelated symbols:\n\n* [`INoiseTexture3D`][crate::classes::INoiseTexture3D]: virtual methods\n\n\nSee also [Godot docs for `NoiseTexture3D`](https://docs.godotengine.org/en/stable/classes/class_noisetexture3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`NoiseTexture3D::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nUses the [`FastNoiseLite`][crate::classes::FastNoiseLite] library or other noise generators to fill the texture data of your desired size.\n\nThe class uses `Thread`s to generate the texture data internally, so [`get_data`][`crate::classes::Texture3D::get_data`] may return `null` if the generation process has not completed yet. In that case, you need to wait for the texture to be generated before accessing the image:\n\n```gdscript\nvar texture = NoiseTexture3D.new()\ntexture.noise = FastNoiseLite.new()\nawait texture.changed\nvar data = texture.get_data()\n```"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct NoiseTexture3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`NoiseTexture3D`][crate::classes::NoiseTexture3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ITexture3D`][crate::classes::ITexture3D] > [`ITexture`][crate::classes::ITexture] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `NoiseTexture3D` methods](https://docs.godotengine.org/en/stable/classes/class_noisetexture3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INoiseTexture3D: crate::obj::GodotClass < Base = NoiseTexture3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Called when the `Texture3D`'s format is queried."]
        fn get_format(&self,) -> crate::classes::image::Format;
        #[doc = "Called when the `Texture3D`'s width is queried."]
        fn get_width(&self,) -> i32;
        #[doc = "Called when the `Texture3D`'s height is queried."]
        fn get_height(&self,) -> i32;
        #[doc = "Called when the `Texture3D`'s depth is queried."]
        fn get_depth(&self,) -> i32;
        #[doc = "Called when the presence of mipmaps in the `Texture3D` is queried."]
        fn has_mipmaps(&self,) -> bool;
        #[doc = "Called when the `Texture3D`'s data is queried."]
        fn get_data(&self,) -> Array < Gd < crate::classes::Image > >;
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
    impl NoiseTexture3D {
        pub fn set_width(&mut self, width: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "set_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_height(&mut self, height: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "set_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_depth(&mut self, depth: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "set_depth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_noise(&mut self, noise: impl AsArg < Option < Gd < crate::classes::Noise >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Noise > > >,);
            let args = (noise.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "set_noise", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_noise(&self,) -> Option < Gd < crate::classes::Noise > > {
            type CallRet = Option < Gd < crate::classes::Noise > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "get_noise", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color_ramp(&mut self, gradient: impl AsArg < Option < Gd < crate::classes::Gradient >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Gradient > > >,);
            let args = (gradient.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "set_color_ramp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_ramp(&self,) -> Option < Gd < crate::classes::Gradient > > {
            type CallRet = Option < Gd < crate::classes::Gradient > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "get_color_ramp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_seamless(&mut self, seamless: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (seamless,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "set_seamless", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_seamless(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "get_seamless", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_invert(&mut self, invert: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (invert,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "set_invert", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_invert(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "get_invert", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_normalize(&mut self, normalize: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "set_normalize", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_normalized(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "is_normalized", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_seamless_blend_skirt(&mut self, seamless_blend_skirt: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (seamless_blend_skirt,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "set_seamless_blend_skirt", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_seamless_blend_skirt(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture3D", "get_seamless_blend_skirt", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for NoiseTexture3D {
        type Base = crate::classes::Texture3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("NoiseTexture3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for NoiseTexture3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture3D > for NoiseTexture3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for NoiseTexture3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for NoiseTexture3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for NoiseTexture3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for NoiseTexture3D {
        
    }
    impl crate::obj::cap::GodotDefault for NoiseTexture3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for NoiseTexture3D {
        type Target = crate::classes::Texture3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for NoiseTexture3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`NoiseTexture3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_NoiseTexture3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::NoiseTexture3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture > for $Class {
                
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
    use super::re_export::NoiseTexture3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for NoiseTexture3D {
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
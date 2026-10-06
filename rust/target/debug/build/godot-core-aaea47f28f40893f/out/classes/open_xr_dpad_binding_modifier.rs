#![doc = "Sidecar module for class [`OpenXrDpadBindingModifier`][crate::classes::OpenXrDpadBindingModifier].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRDpadBindingModifier` enums](https://docs.godotengine.org/en/stable/classes/class_openxrdpadbindingmodifier.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRDpadBindingModifier`.\n\nInherits [`OpenXripBindingModifier`][crate::classes::OpenXripBindingModifier].\n\nRelated symbols:\n\n* [`IOpenXrDpadBindingModifier`][crate::classes::IOpenXrDpadBindingModifier]: virtual methods\n\n\nSee also [Godot docs for `OpenXRDpadBindingModifier`](https://docs.godotengine.org/en/stable/classes/class_openxrdpadbindingmodifier.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`OpenXrDpadBindingModifier::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThe DPad binding modifier converts an axis input to a dpad output, emulating a DPad. New input paths for each dpad direction will be added to the interaction profile. When bound to actions the DPad emulation will be activated. You should **not** combine dpad inputs with normal inputs in the same action set for the same control, this will result in an error being returned when suggested bindings are submitted to OpenXR.\n\nSee [XR_EXT_dpad_binding](https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_dpad_binding) for in-depth details.\n\n**Note:** If the DPad binding modifier extension is enabled, all dpad binding paths will be available in the action map. Adding the modifier to an interaction profile allows you to further customize the behavior."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrDpadBindingModifier {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrDpadBindingModifier`][crate::classes::OpenXrDpadBindingModifier].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IOpenXripBindingModifier`][crate::classes::IOpenXripBindingModifier] > ~~`IOpenXrBindingModifier`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `OpenXRDpadBindingModifier` methods](https://docs.godotengine.org/en/stable/classes/class_openxrdpadbindingmodifier.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrDpadBindingModifier: crate::obj::GodotClass < Base = OpenXrDpadBindingModifier > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Return the description of this class that is used for the title bar of the binding modifier editor."]
        fn get_description(&self,) -> GString;
        #[doc = "Returns the data that is sent to OpenXR when submitting the suggested interacting bindings this modifier is a part of.\n\n**Note:** This must be data compatible with an `XrBindingModificationBaseHeaderKHR` structure."]
        fn get_ip_modification(&mut self,) -> PackedByteArray;
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
    impl OpenXrDpadBindingModifier {
        pub fn set_action_set(&mut self, action_set: impl AsArg < Option < Gd < crate::classes::OpenXrActionSet >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrActionSet > > >,);
            let args = (action_set.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "set_action_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_action_set(&self,) -> Option < Gd < crate::classes::OpenXrActionSet > > {
            type CallRet = Option < Gd < crate::classes::OpenXrActionSet > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "get_action_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_input_path(&mut self, input_path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (input_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "set_input_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_input_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "get_input_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_threshold(&mut self, threshold: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "set_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "get_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_threshold_released(&mut self, threshold_released: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (threshold_released,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "set_threshold_released", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_threshold_released(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "get_threshold_released", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_region(&mut self, center_region: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (center_region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "set_center_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_region(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "get_center_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_wedge_angle(&mut self, wedge_angle: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (wedge_angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "set_wedge_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_wedge_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "get_wedge_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_is_sticky(&mut self, is_sticky: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (is_sticky,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "set_is_sticky", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_is_sticky(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "get_is_sticky", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_on_haptic(&mut self, haptic: impl AsArg < Option < Gd < crate::classes::OpenXrHapticBase >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrHapticBase > > >,);
            let args = (haptic.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "set_on_haptic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_on_haptic(&self,) -> Option < Gd < crate::classes::OpenXrHapticBase > > {
            type CallRet = Option < Gd < crate::classes::OpenXrHapticBase > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "get_on_haptic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_off_haptic(&mut self, haptic: impl AsArg < Option < Gd < crate::classes::OpenXrHapticBase >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrHapticBase > > >,);
            let args = (haptic.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "set_off_haptic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_off_haptic(&self,) -> Option < Gd < crate::classes::OpenXrHapticBase > > {
            type CallRet = Option < Gd < crate::classes::OpenXrHapticBase > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrDpadBindingModifier", "get_off_haptic", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrDpadBindingModifier {
        type Base = crate::classes::OpenXripBindingModifier;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRDpadBindingModifier"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrDpadBindingModifier {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::OpenXripBindingModifier > for OpenXrDpadBindingModifier {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::OpenXrBindingModifier > for OpenXrDpadBindingModifier {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for OpenXrDpadBindingModifier {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for OpenXrDpadBindingModifier {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrDpadBindingModifier {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrDpadBindingModifier {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrDpadBindingModifier {
        type Target = crate::classes::OpenXripBindingModifier;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrDpadBindingModifier {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrDpadBindingModifier`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrDpadBindingModifier__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrDpadBindingModifier > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXripBindingModifier > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrBindingModifier > for $Class {
                
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
    use super::re_export::OpenXrDpadBindingModifier;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for OpenXrDpadBindingModifier {
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
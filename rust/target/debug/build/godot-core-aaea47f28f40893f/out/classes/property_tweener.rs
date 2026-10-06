#![doc = "Sidecar module for class [`PropertyTweener`][crate::classes::PropertyTweener].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PropertyTweener` enums](https://docs.godotengine.org/en/stable/classes/class_propertytweener.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PropertyTweener`.\n\nInherits [`Tweener`][crate::classes::Tweener].\n\nRelated symbols:\n\n* [`IPropertyTweener`][crate::classes::IPropertyTweener]: virtual methods\n\n\nSee also [Godot docs for `PropertyTweener`](https://docs.godotengine.org/en/stable/classes/class_propertytweener.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PropertyTweener>` instances via Godot APIs.\n# Godot docs\n`PropertyTweener` is used to interpolate a property in an object. See [`tween_property`][`crate::classes::Tween::tween_property`] for more usage information.\n\nThe tweener will finish automatically if the target object is freed.\n\n**Note:** [`tween_property`][`crate::classes::Tween::tween_property`] is the only correct way to create `PropertyTweener`. Any `PropertyTweener` created manually will not function correctly."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PropertyTweener {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PropertyTweener`][crate::classes::PropertyTweener].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`ITweener`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `PropertyTweener` methods](https://docs.godotengine.org/en/stable/classes/class_propertytweener.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPropertyTweener: crate::obj::GodotClass < Base = PropertyTweener > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PropertyTweener {
        #[doc = "Sets a custom initial value to the `PropertyTweener`.\n\n**Example:** Move the node from position `(100, 100)` to `(200, 100)`.\n\n\n```gdscript\nvar tween = get_tree().create_tween()\ntween.tween_property(self, \"position\", Vector2(200, 100), 1).from(Vector2(100, 100))\n```\n"]
        pub fn from(&mut self, value: &Variant,) -> Gd < crate::classes::PropertyTweener > {
            type CallRet = Gd < crate::classes::PropertyTweener >;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(53usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PropertyTweener", "from", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Makes the `PropertyTweener` use the current property value (i.e. at the time of creating this `PropertyTweener`) as a starting point. This is equivalent of using [`from`][`crate::classes::PropertyTweener::from`] with the current value. These two calls will do the same:\n\n\n```gdscript\ntween.tween_property(self, \"position\", Vector2(200, 100), 1).from(position)\ntween.tween_property(self, \"position\", Vector2(200, 100), 1).from_current()\n```\n"]
        pub fn from_current(&mut self,) -> Gd < crate::classes::PropertyTweener > {
            type CallRet = Gd < crate::classes::PropertyTweener >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(54usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PropertyTweener", "from_current", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "When called, the final value will be used as a relative value instead.\n\n**Example:** Move the node by `100` pixels to the right.\n\n\n```gdscript\nvar tween = get_tree().create_tween()\ntween.tween_property(self, \"position\", Vector2.RIGHT * 100, 1).as_relative()\n```\n"]
        pub fn as_relative(&mut self,) -> Gd < crate::classes::PropertyTweener > {
            type CallRet = Gd < crate::classes::PropertyTweener >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(55usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PropertyTweener", "as_relative", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the type of used transition from \\[enum Tween.TransitionType]. If not set, the default transition is used from the [`Tween`][crate::classes::Tween] that contains this Tweener."]
        pub fn set_trans(&mut self, trans: crate::classes::tween::TransitionType,) -> Gd < crate::classes::PropertyTweener > {
            type CallRet = Gd < crate::classes::PropertyTweener >;
            type CallParams = (crate::classes::tween::TransitionType,);
            let args = (trans,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(56usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PropertyTweener", "set_trans", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the type of used easing from \\[enum Tween.EaseType]. If not set, the default easing is used from the [`Tween`][crate::classes::Tween] that contains this Tweener."]
        pub fn set_ease(&mut self, ease: crate::classes::tween::EaseType,) -> Gd < crate::classes::PropertyTweener > {
            type CallRet = Gd < crate::classes::PropertyTweener >;
            type CallParams = (crate::classes::tween::EaseType,);
            let args = (ease,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(57usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PropertyTweener", "set_ease", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Allows interpolating the value with a custom easing function. The provided `interpolator_method` will be called with a value ranging from `0.0` to `1.0` and is expected to return a value within the same range (values outside the range can be used for overshoot). The return value of the method is then used for interpolation between initial and final value. Note that the parameter passed to the method is still subject to the tweener's own easing.\n\n\n```gdscript\n@export var curve: Curve\n\nfunc _ready():\n\tvar tween = create_tween()\n\t# Interpolate the value using a custom curve.\n\ttween.tween_property(self, \"position:x\", 300, 1).as_relative().set_custom_interpolator(tween_curve)\n\nfunc tween_curve(v):\n\treturn curve.sample_baked(v)\n```\n"]
        pub fn set_custom_interpolator(&mut self, interpolator_method: &Callable,) -> Gd < crate::classes::PropertyTweener > {
            type CallRet = Gd < crate::classes::PropertyTweener >;
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(interpolator_method),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(58usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PropertyTweener", "set_custom_interpolator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the time in seconds after which the `PropertyTweener` will start interpolating. By default there's no delay."]
        pub fn set_delay(&mut self, delay: f64,) -> Gd < crate::classes::PropertyTweener > {
            type CallRet = Gd < crate::classes::PropertyTweener >;
            type CallParams = (f64,);
            let args = (delay,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(59usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PropertyTweener", "set_delay", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PropertyTweener {
        type Base = crate::classes::Tweener;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PropertyTweener"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PropertyTweener {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Tweener > for PropertyTweener {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PropertyTweener {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PropertyTweener {
        
    }
    impl std::ops::Deref for PropertyTweener {
        type Target = crate::classes::Tweener;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PropertyTweener {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PropertyTweener`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PropertyTweener__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PropertyTweener > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Tweener > for $Class {
                
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
    use super::re_export::PropertyTweener;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::tweener::SignalsOfTweener;
    impl WithSignals for PropertyTweener {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTweener < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
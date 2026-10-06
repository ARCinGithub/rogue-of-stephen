#![doc = "Sidecar module for class [`VisualShaderNodeFloatFunc`][crate::classes::VisualShaderNodeFloatFunc].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeFloatFunc` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodefloatfunc.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeFloatFunc`.\n\nInherits [`VisualShaderNode`][crate::classes::VisualShaderNode].\n\nRelated symbols:\n\n* [`visual_shader_node_float_func`][crate::classes::visual_shader_node_float_func]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeFloatFunc`][crate::classes::IVisualShaderNodeFloatFunc]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeFloatFunc`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodefloatfunc.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VisualShaderNodeFloatFunc::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nAccept a floating-point scalar (`x`) to the input port and transform it according to \\[member function]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeFloatFunc {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VisualShaderNodeFloatFunc`][crate::classes::VisualShaderNodeFloatFunc].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IVisualShaderNode`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `VisualShaderNodeFloatFunc` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodefloatfunc.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeFloatFunc: crate::obj::GodotClass < Base = VisualShaderNodeFloatFunc > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeFloatFunc {
        pub fn set_function(&mut self, func: crate::classes::visual_shader_node_float_func::Function,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_float_func::Function,);
            let args = (func,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1960usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatFunc", "set_function", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_function(&self,) -> crate::classes::visual_shader_node_float_func::Function {
            type CallRet = crate::classes::visual_shader_node_float_func::Function;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1961usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeFloatFunc", "get_function", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeFloatFunc {
        type Base = crate::classes::VisualShaderNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisualShaderNodeFloatFunc"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeFloatFunc {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeFloatFunc {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeFloatFunc {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeFloatFunc {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeFloatFunc {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeFloatFunc {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeFloatFunc {
        type Target = crate::classes::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeFloatFunc {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShaderNodeFloatFunc`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeFloatFunc__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeFloatFunc > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNode > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Function {
    ord: i32
}
impl Function {
    #[doc(alias = "FUNC_SIN")]
    #[doc = "Godot enumerator name: `FUNC_SIN`"]
    pub const SIN: Function = Function {
        ord: 0i32
    };
    #[doc(alias = "FUNC_COS")]
    #[doc = "Godot enumerator name: `FUNC_COS`"]
    pub const COS: Function = Function {
        ord: 1i32
    };
    #[doc(alias = "FUNC_TAN")]
    #[doc = "Godot enumerator name: `FUNC_TAN`"]
    pub const TAN: Function = Function {
        ord: 2i32
    };
    #[doc(alias = "FUNC_ASIN")]
    #[doc = "Godot enumerator name: `FUNC_ASIN`"]
    pub const ASIN: Function = Function {
        ord: 3i32
    };
    #[doc(alias = "FUNC_ACOS")]
    #[doc = "Godot enumerator name: `FUNC_ACOS`"]
    pub const ACOS: Function = Function {
        ord: 4i32
    };
    #[doc(alias = "FUNC_ATAN")]
    #[doc = "Godot enumerator name: `FUNC_ATAN`"]
    pub const ATAN: Function = Function {
        ord: 5i32
    };
    #[doc(alias = "FUNC_SINH")]
    #[doc = "Godot enumerator name: `FUNC_SINH`"]
    pub const SINH: Function = Function {
        ord: 6i32
    };
    #[doc(alias = "FUNC_COSH")]
    #[doc = "Godot enumerator name: `FUNC_COSH`"]
    pub const COSH: Function = Function {
        ord: 7i32
    };
    #[doc(alias = "FUNC_TANH")]
    #[doc = "Godot enumerator name: `FUNC_TANH`"]
    pub const TANH: Function = Function {
        ord: 8i32
    };
    #[doc(alias = "FUNC_LOG")]
    #[doc = "Godot enumerator name: `FUNC_LOG`"]
    pub const LOG: Function = Function {
        ord: 9i32
    };
    #[doc(alias = "FUNC_EXP")]
    #[doc = "Godot enumerator name: `FUNC_EXP`"]
    pub const EXP: Function = Function {
        ord: 10i32
    };
    #[doc(alias = "FUNC_SQRT")]
    #[doc = "Godot enumerator name: `FUNC_SQRT`"]
    pub const SQRT: Function = Function {
        ord: 11i32
    };
    #[doc(alias = "FUNC_ABS")]
    #[doc = "Godot enumerator name: `FUNC_ABS`"]
    pub const ABS: Function = Function {
        ord: 12i32
    };
    #[doc(alias = "FUNC_SIGN")]
    #[doc = "Godot enumerator name: `FUNC_SIGN`"]
    pub const SIGN: Function = Function {
        ord: 13i32
    };
    #[doc(alias = "FUNC_FLOOR")]
    #[doc = "Godot enumerator name: `FUNC_FLOOR`"]
    pub const FLOOR: Function = Function {
        ord: 14i32
    };
    #[doc(alias = "FUNC_ROUND")]
    #[doc = "Godot enumerator name: `FUNC_ROUND`"]
    pub const ROUND: Function = Function {
        ord: 15i32
    };
    #[doc(alias = "FUNC_CEIL")]
    #[doc = "Godot enumerator name: `FUNC_CEIL`"]
    pub const CEIL: Function = Function {
        ord: 16i32
    };
    #[doc(alias = "FUNC_FRACT")]
    #[doc = "Godot enumerator name: `FUNC_FRACT`"]
    pub const FRACT: Function = Function {
        ord: 17i32
    };
    #[doc(alias = "FUNC_SATURATE")]
    #[doc = "Godot enumerator name: `FUNC_SATURATE`"]
    pub const SATURATE: Function = Function {
        ord: 18i32
    };
    #[doc(alias = "FUNC_NEGATE")]
    #[doc = "Godot enumerator name: `FUNC_NEGATE`"]
    pub const NEGATE: Function = Function {
        ord: 19i32
    };
    #[doc(alias = "FUNC_ACOSH")]
    #[doc = "Godot enumerator name: `FUNC_ACOSH`"]
    pub const ACOSH: Function = Function {
        ord: 20i32
    };
    #[doc(alias = "FUNC_ASINH")]
    #[doc = "Godot enumerator name: `FUNC_ASINH`"]
    pub const ASINH: Function = Function {
        ord: 21i32
    };
    #[doc(alias = "FUNC_ATANH")]
    #[doc = "Godot enumerator name: `FUNC_ATANH`"]
    pub const ATANH: Function = Function {
        ord: 22i32
    };
    #[doc(alias = "FUNC_DEGREES")]
    #[doc = "Godot enumerator name: `FUNC_DEGREES`"]
    pub const DEGREES: Function = Function {
        ord: 23i32
    };
    #[doc(alias = "FUNC_EXP2")]
    #[doc = "Godot enumerator name: `FUNC_EXP2`"]
    pub const EXP2: Function = Function {
        ord: 24i32
    };
    #[doc(alias = "FUNC_INVERSE_SQRT")]
    #[doc = "Godot enumerator name: `FUNC_INVERSE_SQRT`"]
    pub const INVERSE_SQRT: Function = Function {
        ord: 25i32
    };
    #[doc(alias = "FUNC_LOG2")]
    #[doc = "Godot enumerator name: `FUNC_LOG2`"]
    pub const LOG2: Function = Function {
        ord: 26i32
    };
    #[doc(alias = "FUNC_RADIANS")]
    #[doc = "Godot enumerator name: `FUNC_RADIANS`"]
    pub const RADIANS: Function = Function {
        ord: 27i32
    };
    #[doc(alias = "FUNC_RECIPROCAL")]
    #[doc = "Godot enumerator name: `FUNC_RECIPROCAL`"]
    pub const RECIPROCAL: Function = Function {
        ord: 28i32
    };
    #[doc(alias = "FUNC_ROUNDEVEN")]
    #[doc = "Godot enumerator name: `FUNC_ROUNDEVEN`"]
    pub const ROUNDEVEN: Function = Function {
        ord: 29i32
    };
    #[doc(alias = "FUNC_TRUNC")]
    #[doc = "Godot enumerator name: `FUNC_TRUNC`"]
    pub const TRUNC: Function = Function {
        ord: 30i32
    };
    #[doc(alias = "FUNC_ONEMINUS")]
    #[doc = "Godot enumerator name: `FUNC_ONEMINUS`"]
    pub const ONEMINUS: Function = Function {
        ord: 31i32
    };
    #[doc(alias = "FUNC_MAX")]
    #[doc = "Godot enumerator name: `FUNC_MAX`"]
    pub const MAX: Function = Function {
        ord: 32i32
    };
    
}
impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Function") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Function {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 => Some(Self {
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
            Self::SIN => "SIN", Self::COS => "COS", Self::TAN => "TAN", Self::ASIN => "ASIN", Self::ACOS => "ACOS", Self::ATAN => "ATAN", Self::SINH => "SINH", Self::COSH => "COSH", Self::TANH => "TANH", Self::LOG => "LOG", Self::EXP => "EXP", Self::SQRT => "SQRT", Self::ABS => "ABS", Self::SIGN => "SIGN", Self::FLOOR => "FLOOR", Self::ROUND => "ROUND", Self::CEIL => "CEIL", Self::FRACT => "FRACT", Self::SATURATE => "SATURATE", Self::NEGATE => "NEGATE", Self::ACOSH => "ACOSH", Self::ASINH => "ASINH", Self::ATANH => "ATANH", Self::DEGREES => "DEGREES", Self::EXP2 => "EXP2", Self::INVERSE_SQRT => "INVERSE_SQRT", Self::LOG2 => "LOG2", Self::RADIANS => "RADIANS", Self::RECIPROCAL => "RECIPROCAL", Self::ROUNDEVEN => "ROUNDEVEN", Self::TRUNC => "TRUNC", Self::ONEMINUS => "ONEMINUS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Function::SIN, Function::COS, Function::TAN, Function::ASIN, Function::ACOS, Function::ATAN, Function::SINH, Function::COSH, Function::TANH, Function::LOG, Function::EXP, Function::SQRT, Function::ABS, Function::SIGN, Function::FLOOR, Function::ROUND, Function::CEIL, Function::FRACT, Function::SATURATE, Function::NEGATE, Function::ACOSH, Function::ASINH, Function::ATANH, Function::DEGREES, Function::EXP2, Function::INVERSE_SQRT, Function::LOG2, Function::RADIANS, Function::RECIPROCAL, Function::ROUNDEVEN, Function::TRUNC, Function::ONEMINUS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Function >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SIN", "FUNC_SIN", Function::SIN), crate::meta::inspect::EnumConstant::new("COS", "FUNC_COS", Function::COS), crate::meta::inspect::EnumConstant::new("TAN", "FUNC_TAN", Function::TAN), crate::meta::inspect::EnumConstant::new("ASIN", "FUNC_ASIN", Function::ASIN), crate::meta::inspect::EnumConstant::new("ACOS", "FUNC_ACOS", Function::ACOS), crate::meta::inspect::EnumConstant::new("ATAN", "FUNC_ATAN", Function::ATAN), crate::meta::inspect::EnumConstant::new("SINH", "FUNC_SINH", Function::SINH), crate::meta::inspect::EnumConstant::new("COSH", "FUNC_COSH", Function::COSH), crate::meta::inspect::EnumConstant::new("TANH", "FUNC_TANH", Function::TANH), crate::meta::inspect::EnumConstant::new("LOG", "FUNC_LOG", Function::LOG), crate::meta::inspect::EnumConstant::new("EXP", "FUNC_EXP", Function::EXP), crate::meta::inspect::EnumConstant::new("SQRT", "FUNC_SQRT", Function::SQRT), crate::meta::inspect::EnumConstant::new("ABS", "FUNC_ABS", Function::ABS), crate::meta::inspect::EnumConstant::new("SIGN", "FUNC_SIGN", Function::SIGN), crate::meta::inspect::EnumConstant::new("FLOOR", "FUNC_FLOOR", Function::FLOOR), crate::meta::inspect::EnumConstant::new("ROUND", "FUNC_ROUND", Function::ROUND), crate::meta::inspect::EnumConstant::new("CEIL", "FUNC_CEIL", Function::CEIL), crate::meta::inspect::EnumConstant::new("FRACT", "FUNC_FRACT", Function::FRACT), crate::meta::inspect::EnumConstant::new("SATURATE", "FUNC_SATURATE", Function::SATURATE), crate::meta::inspect::EnumConstant::new("NEGATE", "FUNC_NEGATE", Function::NEGATE), crate::meta::inspect::EnumConstant::new("ACOSH", "FUNC_ACOSH", Function::ACOSH), crate::meta::inspect::EnumConstant::new("ASINH", "FUNC_ASINH", Function::ASINH), crate::meta::inspect::EnumConstant::new("ATANH", "FUNC_ATANH", Function::ATANH), crate::meta::inspect::EnumConstant::new("DEGREES", "FUNC_DEGREES", Function::DEGREES), crate::meta::inspect::EnumConstant::new("EXP2", "FUNC_EXP2", Function::EXP2), crate::meta::inspect::EnumConstant::new("INVERSE_SQRT", "FUNC_INVERSE_SQRT", Function::INVERSE_SQRT), crate::meta::inspect::EnumConstant::new("LOG2", "FUNC_LOG2", Function::LOG2), crate::meta::inspect::EnumConstant::new("RADIANS", "FUNC_RADIANS", Function::RADIANS), crate::meta::inspect::EnumConstant::new("RECIPROCAL", "FUNC_RECIPROCAL", Function::RECIPROCAL), crate::meta::inspect::EnumConstant::new("ROUNDEVEN", "FUNC_ROUNDEVEN", Function::ROUNDEVEN), crate::meta::inspect::EnumConstant::new("TRUNC", "FUNC_TRUNC", Function::TRUNC), crate::meta::inspect::EnumConstant::new("ONEMINUS", "FUNC_ONEMINUS", Function::ONEMINUS), crate::meta::inspect::EnumConstant::new("MAX", "FUNC_MAX", Function::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Function {
    const ENUMERATOR_COUNT: usize = 32usize;
    
}
impl crate::meta::GodotConvert for Function {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Func Sin", 0i64), EnumeratorShape::new_int("Func Cos", 1i64), EnumeratorShape::new_int("Func Tan", 2i64), EnumeratorShape::new_int("Func Asin", 3i64), EnumeratorShape::new_int("Func Acos", 4i64), EnumeratorShape::new_int("Func Atan", 5i64), EnumeratorShape::new_int("Func Sinh", 6i64), EnumeratorShape::new_int("Func Cosh", 7i64), EnumeratorShape::new_int("Func Tanh", 8i64), EnumeratorShape::new_int("Func Log", 9i64), EnumeratorShape::new_int("Func Exp", 10i64), EnumeratorShape::new_int("Func Sqrt", 11i64), EnumeratorShape::new_int("Func Abs", 12i64), EnumeratorShape::new_int("Func Sign", 13i64), EnumeratorShape::new_int("Func Floor", 14i64), EnumeratorShape::new_int("Func Round", 15i64), EnumeratorShape::new_int("Func Ceil", 16i64), EnumeratorShape::new_int("Func Fract", 17i64), EnumeratorShape::new_int("Func Saturate", 18i64), EnumeratorShape::new_int("Func Negate", 19i64), EnumeratorShape::new_int("Func Acosh", 20i64), EnumeratorShape::new_int("Func Asinh", 21i64), EnumeratorShape::new_int("Func Atanh", 22i64), EnumeratorShape::new_int("Func Degrees", 23i64), EnumeratorShape::new_int("Func Exp2", 24i64), EnumeratorShape::new_int("Func Inverse Sqrt", 25i64), EnumeratorShape::new_int("Func Log2", 26i64), EnumeratorShape::new_int("Func Radians", 27i64), EnumeratorShape::new_int("Func Reciprocal", 28i64), EnumeratorShape::new_int("Func Roundeven", 29i64), EnumeratorShape::new_int("Func Trunc", 30i64), EnumeratorShape::new_int("Func Oneminus", 31i64), EnumeratorShape::new_int("Func Max", 32i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("VisualShaderNodeFloatFunc.Function")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Function {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Function {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Function {
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
impl crate::registry::property::Export for Function {
    
}
impl crate::meta::Element for Function {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNodeFloatFunc;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeFloatFunc {
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
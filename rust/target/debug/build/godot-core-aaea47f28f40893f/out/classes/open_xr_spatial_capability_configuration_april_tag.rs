#![doc = "Sidecar module for class [`OpenXrSpatialCapabilityConfigurationAprilTag`][crate::classes::OpenXrSpatialCapabilityConfigurationAprilTag].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRSpatialCapabilityConfigurationAprilTag` enums](https://docs.godotengine.org/en/stable/classes/class_openxrspatialcapabilityconfigurationapriltag.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRSpatialCapabilityConfigurationAprilTag`.\n\nInherits [`OpenXrSpatialCapabilityConfigurationBaseHeader`][crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader].\n\nRelated symbols:\n\n* [`open_xr_spatial_capability_configuration_april_tag`][crate::classes::open_xr_spatial_capability_configuration_april_tag]: sidecar module with related enum/flag types\n* [`IOpenXrSpatialCapabilityConfigurationAprilTag`][crate::classes::IOpenXrSpatialCapabilityConfigurationAprilTag]: virtual methods\n\n\nSee also [Godot docs for `OpenXRSpatialCapabilityConfigurationAprilTag`](https://docs.godotengine.org/en/stable/classes/class_openxrspatialcapabilityconfigurationapriltag.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`OpenXrSpatialCapabilityConfigurationAprilTag::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nConfiguration header for April tag markers. Pass this to [`create_spatial_context`][`crate::classes::OpenXrSpatialEntityExtension::create_spatial_context`] to create a spatial context that can detect April tags."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrSpatialCapabilityConfigurationAprilTag {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrSpatialCapabilityConfigurationAprilTag`][crate::classes::OpenXrSpatialCapabilityConfigurationAprilTag].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IOpenXrSpatialCapabilityConfigurationBaseHeader`][crate::classes::IOpenXrSpatialCapabilityConfigurationBaseHeader] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `OpenXRSpatialCapabilityConfigurationAprilTag` methods](https://docs.godotengine.org/en/stable/classes/class_openxrspatialcapabilityconfigurationapriltag.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrSpatialCapabilityConfigurationAprilTag: crate::obj::GodotClass < Base = OpenXrSpatialCapabilityConfigurationAprilTag > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Return `true` if this object contains a valid configuration that can be retrieved when calling [`get_configuration`][`crate::classes::IOpenXrSpatialCapabilityConfigurationBaseHeader::get_configuration`]."]
        fn has_valid_configuration(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Return a pointer (encoded as an `int64_t`) to a struct holding the spatial capability configuration data. The memory for this struct should remain accessible as long as this object remains instantiated."]
        fn get_configuration(&mut self,) -> u64 {
            unimplemented !()
        }
    }
    impl OpenXrSpatialCapabilityConfigurationAprilTag {
        #[doc = "Returns the components enabled by this configuration.\n\n**Note:** Only valid after this configuration was used to create a spatial context."]
        pub fn get_enabled_components(&self,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialCapabilityConfigurationAprilTag", "get_enabled_components", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_april_dict(&mut self, april_dict: crate::classes::open_xr_spatial_capability_configuration_april_tag::AprilTagDict,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_spatial_capability_configuration_april_tag::AprilTagDict,);
            let args = (april_dict,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialCapabilityConfigurationAprilTag", "set_april_dict", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_april_dict(&self,) -> crate::classes::open_xr_spatial_capability_configuration_april_tag::AprilTagDict {
            type CallRet = crate::classes::open_xr_spatial_capability_configuration_april_tag::AprilTagDict;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialCapabilityConfigurationAprilTag", "get_april_dict", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrSpatialCapabilityConfigurationAprilTag {
        type Base = crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRSpatialCapabilityConfigurationAprilTag"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrSpatialCapabilityConfigurationAprilTag {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader > for OpenXrSpatialCapabilityConfigurationAprilTag {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for OpenXrSpatialCapabilityConfigurationAprilTag {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrSpatialCapabilityConfigurationAprilTag {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrSpatialCapabilityConfigurationAprilTag {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrSpatialCapabilityConfigurationAprilTag {
        type Target = crate::classes::OpenXrSpatialCapabilityConfigurationBaseHeader;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrSpatialCapabilityConfigurationAprilTag {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrSpatialCapabilityConfigurationAprilTag`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrSpatialCapabilityConfigurationAprilTag__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrSpatialCapabilityConfigurationAprilTag > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrSpatialCapabilityConfigurationBaseHeader > for $Class {
                
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
pub struct AprilTagDict {
    ord: i32
}
impl AprilTagDict {
    #[doc(alias = "APRIL_TAG_DICT_16H5")]
    #[doc = "Godot enumerator name: `APRIL_TAG_DICT_16H5`"]
    pub const DICT_16H5: AprilTagDict = AprilTagDict {
        ord: 1i32
    };
    #[doc(alias = "APRIL_TAG_DICT_25H9")]
    #[doc = "Godot enumerator name: `APRIL_TAG_DICT_25H9`"]
    pub const DICT_25H9: AprilTagDict = AprilTagDict {
        ord: 2i32
    };
    #[doc(alias = "APRIL_TAG_DICT_36H10")]
    #[doc = "Godot enumerator name: `APRIL_TAG_DICT_36H10`"]
    pub const DICT_36H10: AprilTagDict = AprilTagDict {
        ord: 3i32
    };
    #[doc(alias = "APRIL_TAG_DICT_36H11")]
    #[doc = "Godot enumerator name: `APRIL_TAG_DICT_36H11`"]
    pub const DICT_36H11: AprilTagDict = AprilTagDict {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for AprilTagDict {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AprilTagDict") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AprilTagDict {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::DICT_16H5 => "DICT_16H5", Self::DICT_25H9 => "DICT_25H9", Self::DICT_36H10 => "DICT_36H10", Self::DICT_36H11 => "DICT_36H11", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AprilTagDict::DICT_16H5, AprilTagDict::DICT_25H9, AprilTagDict::DICT_36H10, AprilTagDict::DICT_36H11]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AprilTagDict >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DICT_16H5", "APRIL_TAG_DICT_16H5", AprilTagDict::DICT_16H5), crate::meta::inspect::EnumConstant::new("DICT_25H9", "APRIL_TAG_DICT_25H9", AprilTagDict::DICT_25H9), crate::meta::inspect::EnumConstant::new("DICT_36H10", "APRIL_TAG_DICT_36H10", AprilTagDict::DICT_36H10), crate::meta::inspect::EnumConstant::new("DICT_36H11", "APRIL_TAG_DICT_36H11", AprilTagDict::DICT_36H11)]
        }
    }
}
impl crate::meta::GodotConvert for AprilTagDict {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("April Tag Dict 16h5", 1i64), EnumeratorShape::new_int("April Tag Dict 25h9", 2i64), EnumeratorShape::new_int("April Tag Dict 36h10", 3i64), EnumeratorShape::new_int("April Tag Dict 36h11", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("OpenXRSpatialCapabilityConfigurationAprilTag.AprilTagDict")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AprilTagDict {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AprilTagDict {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AprilTagDict {
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
impl crate::registry::property::Export for AprilTagDict {
    
}
impl crate::meta::Element for AprilTagDict {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrSpatialCapabilityConfigurationAprilTag;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for OpenXrSpatialCapabilityConfigurationAprilTag {
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
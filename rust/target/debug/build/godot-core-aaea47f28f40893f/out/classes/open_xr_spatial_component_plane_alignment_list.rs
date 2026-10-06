#![doc = "Sidecar module for class [`OpenXrSpatialComponentPlaneAlignmentList`][crate::classes::OpenXrSpatialComponentPlaneAlignmentList].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRSpatialComponentPlaneAlignmentList` enums](https://docs.godotengine.org/en/stable/classes/class_openxrspatialcomponentplanealignmentlist.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRSpatialComponentPlaneAlignmentList`.\n\nInherits [`OpenXrSpatialComponentData`][crate::classes::OpenXrSpatialComponentData].\n\nRelated symbols:\n\n* [`open_xr_spatial_component_plane_alignment_list`][crate::classes::open_xr_spatial_component_plane_alignment_list]: sidecar module with related enum/flag types\n* [`IOpenXrSpatialComponentPlaneAlignmentList`][crate::classes::IOpenXrSpatialComponentPlaneAlignmentList]: virtual methods\n\n\nSee also [Godot docs for `OpenXRSpatialComponentPlaneAlignmentList`](https://docs.godotengine.org/en/stable/classes/class_openxrspatialcomponentplanealignmentlist.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`OpenXrSpatialComponentPlaneAlignmentList::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nObject for storing the queries plane alignment result data when calling [`query_snapshot`][`crate::classes::OpenXrSpatialEntityExtension::query_snapshot`]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrSpatialComponentPlaneAlignmentList {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrSpatialComponentPlaneAlignmentList`][crate::classes::OpenXrSpatialComponentPlaneAlignmentList].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IOpenXrSpatialComponentData`][crate::classes::IOpenXrSpatialComponentData] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `OpenXRSpatialComponentPlaneAlignmentList` methods](https://docs.godotengine.org/en/stable/classes/class_openxrspatialcomponentplanealignmentlist.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrSpatialComponentPlaneAlignmentList: crate::obj::GodotClass < Base = OpenXrSpatialComponentPlaneAlignmentList > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Set the expected capacity as provided by the spatial entities query system. Buffers should be initialized with the correct storage."]
        fn set_capacity(&mut self, capacity: u32,) {
            unimplemented !()
        }
        #[doc = "Return the component type for the component we store data for."]
        fn get_component_type(&self,) -> u64 {
            unimplemented !()
        }
        #[doc = "Return a pointer to the structure data that will be submitted along with the snapshot query. This pointer must remain valid as long as this object is instantiated."]
        fn get_structure_data(&self, next: u64,) -> u64 {
            unimplemented !()
        }
    }
    impl OpenXrSpatialComponentPlaneAlignmentList {
        #[doc = "Returns the plane alignment for the parent entity at this `index`."]
        pub fn get_plane_alignment(&self, index: i64,) -> crate::classes::open_xr_spatial_component_plane_alignment_list::PlaneAlignment {
            type CallRet = crate::classes::open_xr_spatial_component_plane_alignment_list::PlaneAlignment;
            type CallParams = (i64,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialComponentPlaneAlignmentList", "get_plane_alignment", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrSpatialComponentPlaneAlignmentList {
        type Base = crate::classes::OpenXrSpatialComponentData;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRSpatialComponentPlaneAlignmentList"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrSpatialComponentPlaneAlignmentList {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::OpenXrSpatialComponentData > for OpenXrSpatialComponentPlaneAlignmentList {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for OpenXrSpatialComponentPlaneAlignmentList {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrSpatialComponentPlaneAlignmentList {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrSpatialComponentPlaneAlignmentList {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrSpatialComponentPlaneAlignmentList {
        type Target = crate::classes::OpenXrSpatialComponentData;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrSpatialComponentPlaneAlignmentList {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrSpatialComponentPlaneAlignmentList`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrSpatialComponentPlaneAlignmentList__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrSpatialComponentPlaneAlignmentList > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrSpatialComponentData > for $Class {
                
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
pub struct PlaneAlignment {
    ord: i32
}
impl PlaneAlignment {
    #[doc(alias = "PLANE_ALIGNMENT_HORIZONTAL_UPWARD")]
    #[doc = "Godot enumerator name: `PLANE_ALIGNMENT_HORIZONTAL_UPWARD`"]
    pub const HORIZONTAL_UPWARD: PlaneAlignment = PlaneAlignment {
        ord: 0i32
    };
    #[doc(alias = "PLANE_ALIGNMENT_HORIZONTAL_DOWNWARD")]
    #[doc = "Godot enumerator name: `PLANE_ALIGNMENT_HORIZONTAL_DOWNWARD`"]
    pub const HORIZONTAL_DOWNWARD: PlaneAlignment = PlaneAlignment {
        ord: 1i32
    };
    #[doc(alias = "PLANE_ALIGNMENT_VERTICAL")]
    #[doc = "Godot enumerator name: `PLANE_ALIGNMENT_VERTICAL`"]
    pub const VERTICAL: PlaneAlignment = PlaneAlignment {
        ord: 2i32
    };
    #[doc(alias = "PLANE_ALIGNMENT_ARBITRARY")]
    #[doc = "Godot enumerator name: `PLANE_ALIGNMENT_ARBITRARY`"]
    pub const ARBITRARY: PlaneAlignment = PlaneAlignment {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for PlaneAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PlaneAlignment") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PlaneAlignment {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::HORIZONTAL_UPWARD => "HORIZONTAL_UPWARD", Self::HORIZONTAL_DOWNWARD => "HORIZONTAL_DOWNWARD", Self::VERTICAL => "VERTICAL", Self::ARBITRARY => "ARBITRARY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PlaneAlignment::HORIZONTAL_UPWARD, PlaneAlignment::HORIZONTAL_DOWNWARD, PlaneAlignment::VERTICAL, PlaneAlignment::ARBITRARY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PlaneAlignment >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("HORIZONTAL_UPWARD", "PLANE_ALIGNMENT_HORIZONTAL_UPWARD", PlaneAlignment::HORIZONTAL_UPWARD), crate::meta::inspect::EnumConstant::new("HORIZONTAL_DOWNWARD", "PLANE_ALIGNMENT_HORIZONTAL_DOWNWARD", PlaneAlignment::HORIZONTAL_DOWNWARD), crate::meta::inspect::EnumConstant::new("VERTICAL", "PLANE_ALIGNMENT_VERTICAL", PlaneAlignment::VERTICAL), crate::meta::inspect::EnumConstant::new("ARBITRARY", "PLANE_ALIGNMENT_ARBITRARY", PlaneAlignment::ARBITRARY)]
        }
    }
}
impl crate::meta::GodotConvert for PlaneAlignment {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Plane Alignment Horizontal Upward", 0i64), EnumeratorShape::new_int("Plane Alignment Horizontal Downward", 1i64), EnumeratorShape::new_int("Plane Alignment Vertical", 2i64), EnumeratorShape::new_int("Plane Alignment Arbitrary", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("OpenXRSpatialComponentPlaneAlignmentList.PlaneAlignment")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PlaneAlignment {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PlaneAlignment {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PlaneAlignment {
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
impl crate::registry::property::Export for PlaneAlignment {
    
}
impl crate::meta::Element for PlaneAlignment {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrSpatialComponentPlaneAlignmentList;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for OpenXrSpatialComponentPlaneAlignmentList {
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
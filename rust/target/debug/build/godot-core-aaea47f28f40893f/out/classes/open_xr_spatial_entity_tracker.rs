#![doc = "Sidecar module for class [`OpenXrSpatialEntityTracker`][crate::classes::OpenXrSpatialEntityTracker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRSpatialEntityTracker` enums](https://docs.godotengine.org/en/stable/classes/class_openxrspatialentitytracker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRSpatialEntityTracker`.\n\nInherits [`XrPositionalTracker`][crate::classes::XrPositionalTracker].\n\nRelated symbols:\n\n* [`open_xr_spatial_entity_tracker`][crate::classes::open_xr_spatial_entity_tracker]: sidecar module with related enum/flag types\n* [`IOpenXrSpatialEntityTracker`][crate::classes::IOpenXrSpatialEntityTracker]: virtual methods\n* [`SignalsOfOpenXrSpatialEntityTracker`][crate::classes::open_xr_spatial_entity_tracker::SignalsOfOpenXrSpatialEntityTracker]: signal collection\n\n\nSee also [Godot docs for `OpenXRSpatialEntityTracker`](https://docs.godotengine.org/en/stable/classes/class_openxrspatialentitytracker.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`OpenXrSpatialEntityTracker::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThese are trackers created and managed by OpenXR's spatial entity extensions that give access to specific data related to OpenXR's spatial entities. They will always be of type `TRACKER_ANCHOR`."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrSpatialEntityTracker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrSpatialEntityTracker`][crate::classes::OpenXrSpatialEntityTracker].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IXrPositionalTracker`][crate::classes::IXrPositionalTracker] > ~~`IXrTracker`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `OpenXRSpatialEntityTracker` methods](https://docs.godotengine.org/en/stable/classes/class_openxrspatialentitytracker.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrSpatialEntityTracker: crate::obj::GodotClass < Base = OpenXrSpatialEntityTracker > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OpenXrSpatialEntityTracker {
        pub fn set_entity(&mut self, entity: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (entity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5739usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityTracker", "set_entity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_entity(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5740usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityTracker", "get_entity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_spatial_tracking_state(&mut self, spatial_tracking_state: crate::classes::open_xr_spatial_entity_tracker::EntityTrackingState,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_spatial_entity_tracker::EntityTrackingState,);
            let args = (spatial_tracking_state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityTracker", "set_spatial_tracking_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_spatial_tracking_state(&self,) -> crate::classes::open_xr_spatial_entity_tracker::EntityTrackingState {
            type CallRet = crate::classes::open_xr_spatial_entity_tracker::EntityTrackingState;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrSpatialEntityTracker", "get_spatial_tracking_state", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrSpatialEntityTracker {
        type Base = crate::classes::XrPositionalTracker;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRSpatialEntityTracker"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrSpatialEntityTracker {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrPositionalTracker > for OpenXrSpatialEntityTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrTracker > for OpenXrSpatialEntityTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for OpenXrSpatialEntityTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrSpatialEntityTracker {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrSpatialEntityTracker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrSpatialEntityTracker {
        type Target = crate::classes::XrPositionalTracker;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrSpatialEntityTracker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrSpatialEntityTracker`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrSpatialEntityTracker__ensure_class_exists {
        ($Class: ident) => {
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EntityTrackingState {
    ord: i32
}
impl EntityTrackingState {
    #[doc(alias = "ENTITY_TRACKING_STATE_STOPPED")]
    #[doc = "Godot enumerator name: `ENTITY_TRACKING_STATE_STOPPED`"]
    pub const STOPPED: EntityTrackingState = EntityTrackingState {
        ord: 1i32
    };
    #[doc(alias = "ENTITY_TRACKING_STATE_PAUSED")]
    #[doc = "Godot enumerator name: `ENTITY_TRACKING_STATE_PAUSED`"]
    pub const PAUSED: EntityTrackingState = EntityTrackingState {
        ord: 2i32
    };
    #[doc(alias = "ENTITY_TRACKING_STATE_TRACKING")]
    #[doc = "Godot enumerator name: `ENTITY_TRACKING_STATE_TRACKING`"]
    pub const TRACKING: EntityTrackingState = EntityTrackingState {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EntityTrackingState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EntityTrackingState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EntityTrackingState {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::STOPPED => "STOPPED", Self::PAUSED => "PAUSED", Self::TRACKING => "TRACKING", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EntityTrackingState::STOPPED, EntityTrackingState::PAUSED, EntityTrackingState::TRACKING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EntityTrackingState >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STOPPED", "ENTITY_TRACKING_STATE_STOPPED", EntityTrackingState::STOPPED), crate::meta::inspect::EnumConstant::new("PAUSED", "ENTITY_TRACKING_STATE_PAUSED", EntityTrackingState::PAUSED), crate::meta::inspect::EnumConstant::new("TRACKING", "ENTITY_TRACKING_STATE_TRACKING", EntityTrackingState::TRACKING)]
        }
    }
}
impl crate::meta::GodotConvert for EntityTrackingState {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Entity Tracking State Stopped", 1i64), EnumeratorShape::new_int("Entity Tracking State Paused", 2i64), EnumeratorShape::new_int("Entity Tracking State Tracking", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("OpenXRSpatialEntityTracker.EntityTrackingState")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EntityTrackingState {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EntityTrackingState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EntityTrackingState {
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
impl crate::registry::property::Export for EntityTrackingState {
    
}
impl crate::meta::Element for EntityTrackingState {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrSpatialEntityTracker;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`OpenXrSpatialEntityTracker`][crate::classes::OpenXrSpatialEntityTracker] class."]
    pub struct SignalsOfOpenXrSpatialEntityTracker < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfOpenXrSpatialEntityTracker < 'c, C > {
        #[doc = "Signature: `(spatial_tracking_state: i64)`"]
        pub fn spatial_tracking_state_changed(&mut self) -> SigSpatialTrackingStateChanged < 'c, C > {
            SigSpatialTrackingStateChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "spatial_tracking_state_changed")
            }
        }
    }
    type TypedSigSpatialTrackingStateChanged < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSpatialTrackingStateChanged < 'c, C: WithSignals > {
        typed: TypedSigSpatialTrackingStateChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSpatialTrackingStateChanged < 'c, C > {
        pub fn emit(&mut self, spatial_tracking_state: i64,) {
            self.typed.emit_tuple((spatial_tracking_state,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSpatialTrackingStateChanged < 'c, C > {
        type Target = TypedSigSpatialTrackingStateChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSpatialTrackingStateChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for OpenXrSpatialEntityTracker {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfOpenXrSpatialEntityTracker < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfOpenXrSpatialEntityTracker < 'c, C > {
        type Target = < < OpenXrSpatialEntityTracker as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = OpenXrSpatialEntityTracker;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfOpenXrSpatialEntityTracker < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = OpenXrSpatialEntityTracker;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
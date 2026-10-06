#![doc = "Sidecar module for class [`OpenXrPlaneTracker`][crate::classes::OpenXrPlaneTracker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRPlaneTracker` enums](https://docs.godotengine.org/en/stable/classes/class_openxrplanetracker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRPlaneTracker`.\n\nInherits [`OpenXrSpatialEntityTracker`][crate::classes::OpenXrSpatialEntityTracker].\n\nRelated symbols:\n\n* [`open_xr_plane_tracker`][crate::classes::open_xr_plane_tracker]: sidecar module with related enum/flag types\n* [`IOpenXrPlaneTracker`][crate::classes::IOpenXrPlaneTracker]: virtual methods\n* [`SignalsOfOpenXrPlaneTracker`][crate::classes::open_xr_plane_tracker::SignalsOfOpenXrPlaneTracker]: signal collection\n\n\nSee also [Godot docs for `OpenXRPlaneTracker`](https://docs.godotengine.org/en/stable/classes/class_openxrplanetracker.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`OpenXrPlaneTracker::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nSpatial entity tracker for our OpenXR spatial entity plane tracking extension. These trackers identify entities in our real space such as walls, floors, tables, etc. and map their location to our virtual space."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrPlaneTracker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrPlaneTracker`][crate::classes::OpenXrPlaneTracker].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IOpenXrSpatialEntityTracker`][crate::classes::IOpenXrSpatialEntityTracker] > [`IXrPositionalTracker`][crate::classes::IXrPositionalTracker] > ~~`IXrTracker`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `OpenXRPlaneTracker` methods](https://docs.godotengine.org/en/stable/classes/class_openxrplanetracker.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrPlaneTracker: crate::obj::GodotClass < Base = OpenXrPlaneTracker > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OpenXrPlaneTracker {
        pub fn set_bounds_size(&mut self, bounds_size: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (bounds_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrPlaneTracker", "set_bounds_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bounds_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5726usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrPlaneTracker", "get_bounds_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_plane_alignment(&mut self, plane_alignment: crate::classes::open_xr_spatial_component_plane_alignment_list::PlaneAlignment,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_spatial_component_plane_alignment_list::PlaneAlignment,);
            let args = (plane_alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5727usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrPlaneTracker", "set_plane_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_plane_alignment(&self,) -> crate::classes::open_xr_spatial_component_plane_alignment_list::PlaneAlignment {
            type CallRet = crate::classes::open_xr_spatial_component_plane_alignment_list::PlaneAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5728usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrPlaneTracker", "get_plane_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_plane_label(&mut self, plane_label: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (plane_label.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5729usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrPlaneTracker", "set_plane_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_plane_label(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5730usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrPlaneTracker", "get_plane_label", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the mesh data for this plane. You should only call this if you are handling your own discovery logic."]
        pub(crate) fn set_mesh_data_full(&mut self, origin: Transform3D, vertices: RefArg < PackedVector2Array >, indices: RefArg < PackedInt32Array >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Transform3D, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedInt32Array >,);
            let args = (origin, vertices, indices,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrPlaneTracker", "set_mesh_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_mesh_data_ex`][Self::set_mesh_data_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the mesh data for this plane. You should only call this if you are handling your own discovery logic."]
        #[inline]
        pub fn set_mesh_data(&mut self, origin: Transform3D, vertices: &PackedVector2Array,) {
            self.set_mesh_data_ex(origin, vertices,) . done()
        }
        #[doc = "Sets the mesh data for this plane. You should only call this if you are handling your own discovery logic."]
        #[inline]
        pub fn set_mesh_data_ex < 'ex > (&'ex mut self, origin: Transform3D, vertices: &'ex PackedVector2Array,) -> ExSetMeshData < 'ex > {
            ExSetMeshData::new(self, origin, vertices,)
        }
        #[doc = "Clears the mesh data for this tracker. You should only call this if you are handling your own discovery logic."]
        pub fn clear_mesh_data(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrPlaneTracker", "clear_mesh_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the transform by which to offset the mesh and collision shape from our pose to display these correctly."]
        pub fn get_mesh_offset(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrPlaneTracker", "get_mesh_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a mesh created from either the mesh data or from our bounding size for this plane."]
        pub fn get_mesh(&self,) -> Option < Gd < crate::classes::Mesh > > {
            type CallRet = Option < Gd < crate::classes::Mesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5734usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrPlaneTracker", "get_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a collision shape built either from the mesh data or from our bounding size for this plane."]
        pub(crate) fn get_shape_full(&self, thickness: f32,) -> Option < Gd < crate::classes::Shape3D > > {
            type CallRet = Option < Gd < crate::classes::Shape3D > >;
            type CallParams = (f32,);
            let args = (thickness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5735usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrPlaneTracker", "get_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_shape_ex`][Self::get_shape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Gets a collision shape built either from the mesh data or from our bounding size for this plane."]
        #[inline]
        pub fn get_shape(&self,) -> Option < Gd < crate::classes::Shape3D > > {
            self.get_shape_ex() . done()
        }
        #[doc = "Gets a collision shape built either from the mesh data or from our bounding size for this plane."]
        #[inline]
        pub fn get_shape_ex < 'ex > (&'ex self,) -> ExGetShape < 'ex > {
            ExGetShape::new(self,)
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
    impl crate::obj::GodotClass for OpenXrPlaneTracker {
        type Base = crate::classes::OpenXrSpatialEntityTracker;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRPlaneTracker"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrPlaneTracker {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::OpenXrSpatialEntityTracker > for OpenXrPlaneTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrPositionalTracker > for OpenXrPlaneTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrTracker > for OpenXrPlaneTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for OpenXrPlaneTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrPlaneTracker {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrPlaneTracker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrPlaneTracker {
        type Target = crate::classes::OpenXrSpatialEntityTracker;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrPlaneTracker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrPlaneTracker`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrPlaneTracker__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrPlaneTracker > for $Class {
                
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
#[doc = "Default-param extender for [`OpenXrPlaneTracker::set_mesh_data_ex`][super::OpenXrPlaneTracker::set_mesh_data_ex]."]
#[must_use]
pub struct ExSetMeshData < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::OpenXrPlaneTracker, origin: Transform3D, vertices: CowArg < 'ex, PackedVector2Array >, indices: CowArg < 'ex, PackedInt32Array >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetMeshData < 'ex > {
    fn new(surround_object: &'ex mut re_export::OpenXrPlaneTracker, origin: Transform3D, vertices: &'ex PackedVector2Array,) -> Self {
        let indices = PackedInt32Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, origin: origin, vertices: CowArg::Borrowed(vertices), indices: CowArg::Owned(indices),
        }
    }
    #[inline]
    pub fn indices(self, indices: &'ex PackedInt32Array) -> Self {
        Self {
            indices: CowArg::Borrowed(indices), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, origin, vertices, indices,
        }
        = self;
        re_export::OpenXrPlaneTracker::set_mesh_data_full(surround_object, origin, vertices.cow_as_arg(), indices.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`OpenXrPlaneTracker::get_shape_ex`][super::OpenXrPlaneTracker::get_shape_ex]."]
#[must_use]
pub struct ExGetShape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::OpenXrPlaneTracker, thickness: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetShape < 'ex > {
    fn new(surround_object: &'ex re_export::OpenXrPlaneTracker,) -> Self {
        let thickness = 0.01f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, thickness: thickness,
        }
    }
    #[inline]
    pub fn thickness(self, thickness: f32) -> Self {
        Self {
            thickness: thickness, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Shape3D > > {
        let Self {
            _phantom, surround_object, thickness,
        }
        = self;
        re_export::OpenXrPlaneTracker::get_shape_full(surround_object, thickness,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrPlaneTracker;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`OpenXrPlaneTracker`][crate::classes::OpenXrPlaneTracker] class."]
    pub struct SignalsOfOpenXrPlaneTracker < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfOpenXrPlaneTracker < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn mesh_changed(&mut self) -> SigMeshChanged < 'c, C > {
            SigMeshChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "mesh_changed")
            }
        }
    }
    type TypedSigMeshChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMeshChanged < 'c, C: WithSignals > {
        typed: TypedSigMeshChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMeshChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMeshChanged < 'c, C > {
        type Target = TypedSigMeshChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMeshChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for OpenXrPlaneTracker {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfOpenXrPlaneTracker < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfOpenXrPlaneTracker < 'c, C > {
        type Target = < < OpenXrPlaneTracker as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = OpenXrPlaneTracker;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfOpenXrPlaneTracker < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = OpenXrPlaneTracker;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
#![doc = "Sidecar module for class [`VoxelGiData`][crate::classes::VoxelGiData].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VoxelGIData` enums](https://docs.godotengine.org/en/stable/classes/class_voxelgidata.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VoxelGIData`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`IVoxelGiData`][crate::classes::IVoxelGiData]: virtual methods\n\n\nSee also [Godot docs for `VoxelGIData`](https://docs.godotengine.org/en/stable/classes/class_voxelgidata.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VoxelGiData::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`VoxelGIData` contains baked voxel global illumination for use in a [`VoxelGI`][crate::classes::VoxelGi] node. `VoxelGIData` also offers several properties to adjust the final appearance of the global illumination. These properties can be adjusted at run-time without having to bake the [`VoxelGI`][crate::classes::VoxelGi] node again.\n\n**Note:** To prevent text-based scene files (`.tscn`) from growing too much and becoming slow to load and save, always save `VoxelGIData` to an external binary resource file (`.res`) instead of embedding it within the scene. This can be done by clicking the dropdown arrow next to the `VoxelGIData` resource, choosing **Edit**, clicking the floppy disk icon at the top of the Inspector then choosing **Save As...**."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VoxelGiData {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VoxelGiData`][crate::classes::VoxelGiData].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `VoxelGIData` methods](https://docs.godotengine.org/en/stable/classes/class_voxelgidata.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVoxelGiData: crate::obj::GodotClass < Base = VoxelGiData > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VoxelGiData {
        pub fn allocate(&mut self, to_cell_xform: Transform3D, aabb: Aabb, octree_size: Vector3, octree_cells: &PackedByteArray, data_cells: &PackedByteArray, distance_field: &PackedByteArray, level_counts: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Transform3D, Aabb, Vector3, RefArg < 'a0, PackedByteArray >, RefArg < 'a1, PackedByteArray >, RefArg < 'a2, PackedByteArray >, RefArg < 'a3, PackedInt32Array >,);
            let args = (to_cell_xform, aabb, octree_size, RefArg::new(octree_cells), RefArg::new(data_cells), RefArg::new(distance_field), RefArg::new(level_counts),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "allocate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the bounds of the baked voxel data as an [`AABB`][crate::builtin::Aabb], which should match \\[member VoxelGI.size] after being baked (which only contains the size as a [`Vector3`][crate::builtin::Vector3]).\n\n**Note:** If the size was modified without baking the VoxelGI data, then the value of [`get_bounds`][`crate::classes::VoxelGiData::get_bounds`] and \\[member VoxelGI.size] will not match."]
        pub fn get_bounds(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_bounds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_octree_size(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_octree_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_to_cell_xform(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_to_cell_xform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_octree_cells(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_octree_cells", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_data_cells(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_data_cells", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_level_counts(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_level_counts", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_dynamic_range(&mut self, dynamic_range: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (dynamic_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_dynamic_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_dynamic_range(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_dynamic_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_energy(&mut self, energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_energy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bias(&mut self, bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_normal_bias(&mut self, bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_normal_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_normal_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_normal_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_propagation(&mut self, propagation: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (propagation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_propagation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_propagation(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "get_propagation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_interior(&mut self, interior: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (interior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_interior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_interior(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "is_interior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_two_bounces(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "set_use_two_bounces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_two_bounces(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGiData", "is_using_two_bounces", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VoxelGiData {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VoxelGIData"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VoxelGiData {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VoxelGiData {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VoxelGiData {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VoxelGiData {
        
    }
    impl crate::obj::cap::GodotDefault for VoxelGiData {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VoxelGiData {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VoxelGiData {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VoxelGiData`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VoxelGiData__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VoxelGiData > for $Class {
                
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
    use super::re_export::VoxelGiData;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VoxelGiData {
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
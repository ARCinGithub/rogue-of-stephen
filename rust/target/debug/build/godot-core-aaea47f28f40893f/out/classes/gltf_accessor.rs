#![doc = "Sidecar module for class [`GltfAccessor`][crate::classes::GltfAccessor].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFAccessor` enums](https://docs.godotengine.org/en/stable/classes/class_gltfaccessor.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFAccessor`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`gltf_accessor`][crate::classes::gltf_accessor]: sidecar module with related enum/flag types\n* [`IGltfAccessor`][crate::classes::IGltfAccessor]: virtual methods\n\n\nSee also [Godot docs for `GLTFAccessor`](https://docs.godotengine.org/en/stable/classes/class_gltfaccessor.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfAccessor::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nGLTFAccessor is a data structure representing a glTF `accessor` that would be found in the `\"accessors\"` array. A buffer is a blob of binary data. A buffer view is a slice of a buffer. An accessor is a typed interpretation of the data in a buffer view.\n\nMost custom data stored in glTF does not need accessors, only buffer views (see [`GLTFBufferView`][crate::classes::GltfBufferView]). Accessors are for more advanced use cases such as interleaved mesh data encoded for the GPU."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfAccessor {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GltfAccessor`][crate::classes::GltfAccessor].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GLTFAccessor` methods](https://docs.godotengine.org/en/stable/classes/class_gltfaccessor.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfAccessor: crate::obj::GodotClass < Base = GltfAccessor > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfAccessor {
        #[doc = "Creates a new GLTFAccessor instance by parsing the given [`Dictionary`][crate::builtin::Dictionary]."]
        pub fn from_dictionary(dictionary: &AnyDictionary,) -> Option < Gd < crate::classes::GltfAccessor > > {
            type CallRet = Option < Gd < crate::classes::GltfAccessor > >;
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(dictionary),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "from_dictionary", None, args,)
            }
        }
        #[doc = "Serializes this GLTFAccessor instance into a [`Dictionary`][crate::builtin::Dictionary]."]
        pub fn to_dictionary(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "to_dictionary", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_buffer_view(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_buffer_view", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_buffer_view(&mut self, buffer_view: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (buffer_view,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_buffer_view", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_byte_offset(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_byte_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_byte_offset(&mut self, byte_offset: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (byte_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_byte_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_component_type(&self,) -> crate::classes::gltf_accessor::GltfComponentType {
            type CallRet = crate::classes::gltf_accessor::GltfComponentType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_component_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_component_type(&mut self, component_type: crate::classes::gltf_accessor::GltfComponentType,) {
            type CallRet = ();
            type CallParams = (crate::classes::gltf_accessor::GltfComponentType,);
            let args = (component_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_component_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_normalized(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_normalized", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_normalized(&mut self, normalized: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (normalized,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_normalized", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_count(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_count(&mut self, count: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessor_type(&self,) -> crate::classes::gltf_accessor::GltfAccessorType {
            type CallRet = crate::classes::gltf_accessor::GltfAccessorType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_accessor_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accessor_type(&mut self, accessor_type: crate::classes::gltf_accessor::GltfAccessorType,) {
            type CallRet = ();
            type CallParams = (crate::classes::gltf_accessor::GltfAccessorType,);
            let args = (accessor_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_accessor_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_type(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_type(&mut self, type_: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_min(&self,) -> PackedFloat64Array {
            type CallRet = PackedFloat64Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_min", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_min(&mut self, min: &PackedFloat64Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat64Array >,);
            let args = (RefArg::new(min),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_min", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max(&self,) -> PackedFloat64Array {
            type CallRet = PackedFloat64Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_max", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max(&mut self, max: &PackedFloat64Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat64Array >,);
            let args = (RefArg::new(max),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_max", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sparse_count(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_sparse_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sparse_count(&mut self, sparse_count: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (sparse_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_sparse_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sparse_indices_buffer_view(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_sparse_indices_buffer_view", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sparse_indices_buffer_view(&mut self, sparse_indices_buffer_view: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (sparse_indices_buffer_view,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_sparse_indices_buffer_view", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sparse_indices_byte_offset(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_sparse_indices_byte_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sparse_indices_byte_offset(&mut self, sparse_indices_byte_offset: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (sparse_indices_byte_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_sparse_indices_byte_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sparse_indices_component_type(&self,) -> crate::classes::gltf_accessor::GltfComponentType {
            type CallRet = crate::classes::gltf_accessor::GltfComponentType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_sparse_indices_component_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sparse_indices_component_type(&mut self, sparse_indices_component_type: crate::classes::gltf_accessor::GltfComponentType,) {
            type CallRet = ();
            type CallParams = (crate::classes::gltf_accessor::GltfComponentType,);
            let args = (sparse_indices_component_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_sparse_indices_component_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sparse_values_buffer_view(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_sparse_values_buffer_view", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sparse_values_buffer_view(&mut self, sparse_values_buffer_view: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (sparse_values_buffer_view,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_sparse_values_buffer_view", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sparse_values_byte_offset(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "get_sparse_values_byte_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sparse_values_byte_offset(&mut self, sparse_values_byte_offset: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (sparse_values_byte_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfAccessor", "set_sparse_values_byte_offset", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for GltfAccessor {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GLTFAccessor"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfAccessor {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfAccessor {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfAccessor {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfAccessor {
        
    }
    impl crate::obj::cap::GodotDefault for GltfAccessor {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfAccessor {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfAccessor {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfAccessor`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GltfAccessor__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfAccessor > for $Class {
                
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
#[doc = "Godot enum name: `GLTFAccessorType`."]
pub struct GltfAccessorType {
    ord: i32
}
impl GltfAccessorType {
    #[doc(alias = "TYPE_SCALAR")]
    #[doc = "Godot enumerator name: `TYPE_SCALAR`"]
    pub const SCALAR: GltfAccessorType = GltfAccessorType {
        ord: 0i32
    };
    #[doc(alias = "TYPE_VEC2")]
    #[doc = "Godot enumerator name: `TYPE_VEC2`"]
    pub const VEC2: GltfAccessorType = GltfAccessorType {
        ord: 1i32
    };
    #[doc(alias = "TYPE_VEC3")]
    #[doc = "Godot enumerator name: `TYPE_VEC3`"]
    pub const VEC3: GltfAccessorType = GltfAccessorType {
        ord: 2i32
    };
    #[doc(alias = "TYPE_VEC4")]
    #[doc = "Godot enumerator name: `TYPE_VEC4`"]
    pub const VEC4: GltfAccessorType = GltfAccessorType {
        ord: 3i32
    };
    #[doc(alias = "TYPE_MAT2")]
    #[doc = "Godot enumerator name: `TYPE_MAT2`"]
    pub const MAT2: GltfAccessorType = GltfAccessorType {
        ord: 4i32
    };
    #[doc(alias = "TYPE_MAT3")]
    #[doc = "Godot enumerator name: `TYPE_MAT3`"]
    pub const MAT3: GltfAccessorType = GltfAccessorType {
        ord: 5i32
    };
    #[doc(alias = "TYPE_MAT4")]
    #[doc = "Godot enumerator name: `TYPE_MAT4`"]
    pub const MAT4: GltfAccessorType = GltfAccessorType {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for GltfAccessorType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GltfAccessorType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GltfAccessorType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::SCALAR => "SCALAR", Self::VEC2 => "VEC2", Self::VEC3 => "VEC3", Self::VEC4 => "VEC4", Self::MAT2 => "MAT2", Self::MAT3 => "MAT3", Self::MAT4 => "MAT4", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GltfAccessorType::SCALAR, GltfAccessorType::VEC2, GltfAccessorType::VEC3, GltfAccessorType::VEC4, GltfAccessorType::MAT2, GltfAccessorType::MAT3, GltfAccessorType::MAT4]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GltfAccessorType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALAR", "TYPE_SCALAR", GltfAccessorType::SCALAR), crate::meta::inspect::EnumConstant::new("VEC2", "TYPE_VEC2", GltfAccessorType::VEC2), crate::meta::inspect::EnumConstant::new("VEC3", "TYPE_VEC3", GltfAccessorType::VEC3), crate::meta::inspect::EnumConstant::new("VEC4", "TYPE_VEC4", GltfAccessorType::VEC4), crate::meta::inspect::EnumConstant::new("MAT2", "TYPE_MAT2", GltfAccessorType::MAT2), crate::meta::inspect::EnumConstant::new("MAT3", "TYPE_MAT3", GltfAccessorType::MAT3), crate::meta::inspect::EnumConstant::new("MAT4", "TYPE_MAT4", GltfAccessorType::MAT4)]
        }
    }
}
impl crate::meta::GodotConvert for GltfAccessorType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Type Scalar", 0i64), EnumeratorShape::new_int("Type Vec2", 1i64), EnumeratorShape::new_int("Type Vec3", 2i64), EnumeratorShape::new_int("Type Vec4", 3i64), EnumeratorShape::new_int("Type Mat2", 4i64), EnumeratorShape::new_int("Type Mat3", 5i64), EnumeratorShape::new_int("Type Mat4", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("GLTFAccessor.GLTFAccessorType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for GltfAccessorType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GltfAccessorType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for GltfAccessorType {
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
impl crate::registry::property::Export for GltfAccessorType {
    
}
impl crate::meta::Element for GltfAccessorType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `GLTFComponentType`."]
pub struct GltfComponentType {
    ord: i32
}
impl GltfComponentType {
    #[doc(alias = "COMPONENT_TYPE_NONE")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_NONE`"]
    pub const NONE: GltfComponentType = GltfComponentType {
        ord: 0i32
    };
    #[doc(alias = "COMPONENT_TYPE_SIGNED_BYTE")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_SIGNED_BYTE`"]
    pub const SIGNED_BYTE: GltfComponentType = GltfComponentType {
        ord: 5120i32
    };
    #[doc(alias = "COMPONENT_TYPE_UNSIGNED_BYTE")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_UNSIGNED_BYTE`"]
    pub const UNSIGNED_BYTE: GltfComponentType = GltfComponentType {
        ord: 5121i32
    };
    #[doc(alias = "COMPONENT_TYPE_SIGNED_SHORT")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_SIGNED_SHORT`"]
    pub const SIGNED_SHORT: GltfComponentType = GltfComponentType {
        ord: 5122i32
    };
    #[doc(alias = "COMPONENT_TYPE_UNSIGNED_SHORT")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_UNSIGNED_SHORT`"]
    pub const UNSIGNED_SHORT: GltfComponentType = GltfComponentType {
        ord: 5123i32
    };
    #[doc(alias = "COMPONENT_TYPE_SIGNED_INT")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_SIGNED_INT`"]
    pub const SIGNED_INT: GltfComponentType = GltfComponentType {
        ord: 5124i32
    };
    #[doc(alias = "COMPONENT_TYPE_UNSIGNED_INT")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_UNSIGNED_INT`"]
    pub const UNSIGNED_INT: GltfComponentType = GltfComponentType {
        ord: 5125i32
    };
    #[doc(alias = "COMPONENT_TYPE_SINGLE_FLOAT")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_SINGLE_FLOAT`"]
    pub const SINGLE_FLOAT: GltfComponentType = GltfComponentType {
        ord: 5126i32
    };
    #[doc(alias = "COMPONENT_TYPE_DOUBLE_FLOAT")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_DOUBLE_FLOAT`"]
    pub const DOUBLE_FLOAT: GltfComponentType = GltfComponentType {
        ord: 5130i32
    };
    #[doc(alias = "COMPONENT_TYPE_HALF_FLOAT")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_HALF_FLOAT`"]
    pub const HALF_FLOAT: GltfComponentType = GltfComponentType {
        ord: 5131i32
    };
    #[doc(alias = "COMPONENT_TYPE_SIGNED_LONG")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_SIGNED_LONG`"]
    pub const SIGNED_LONG: GltfComponentType = GltfComponentType {
        ord: 5134i32
    };
    #[doc(alias = "COMPONENT_TYPE_UNSIGNED_LONG")]
    #[doc = "Godot enumerator name: `COMPONENT_TYPE_UNSIGNED_LONG`"]
    pub const UNSIGNED_LONG: GltfComponentType = GltfComponentType {
        ord: 5135i32
    };
    
}
impl std::fmt::Debug for GltfComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GltfComponentType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GltfComponentType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 5120i32 | ord @ 5121i32 | ord @ 5122i32 | ord @ 5123i32 | ord @ 5124i32 | ord @ 5125i32 | ord @ 5126i32 | ord @ 5130i32 | ord @ 5131i32 | ord @ 5134i32 | ord @ 5135i32 => Some(Self {
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
            Self::NONE => "NONE", Self::SIGNED_BYTE => "SIGNED_BYTE", Self::UNSIGNED_BYTE => "UNSIGNED_BYTE", Self::SIGNED_SHORT => "SIGNED_SHORT", Self::UNSIGNED_SHORT => "UNSIGNED_SHORT", Self::SIGNED_INT => "SIGNED_INT", Self::UNSIGNED_INT => "UNSIGNED_INT", Self::SINGLE_FLOAT => "SINGLE_FLOAT", Self::DOUBLE_FLOAT => "DOUBLE_FLOAT", Self::HALF_FLOAT => "HALF_FLOAT", Self::SIGNED_LONG => "SIGNED_LONG", Self::UNSIGNED_LONG => "UNSIGNED_LONG", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GltfComponentType::NONE, GltfComponentType::SIGNED_BYTE, GltfComponentType::UNSIGNED_BYTE, GltfComponentType::SIGNED_SHORT, GltfComponentType::UNSIGNED_SHORT, GltfComponentType::SIGNED_INT, GltfComponentType::UNSIGNED_INT, GltfComponentType::SINGLE_FLOAT, GltfComponentType::DOUBLE_FLOAT, GltfComponentType::HALF_FLOAT, GltfComponentType::SIGNED_LONG, GltfComponentType::UNSIGNED_LONG]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GltfComponentType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "COMPONENT_TYPE_NONE", GltfComponentType::NONE), crate::meta::inspect::EnumConstant::new("SIGNED_BYTE", "COMPONENT_TYPE_SIGNED_BYTE", GltfComponentType::SIGNED_BYTE), crate::meta::inspect::EnumConstant::new("UNSIGNED_BYTE", "COMPONENT_TYPE_UNSIGNED_BYTE", GltfComponentType::UNSIGNED_BYTE), crate::meta::inspect::EnumConstant::new("SIGNED_SHORT", "COMPONENT_TYPE_SIGNED_SHORT", GltfComponentType::SIGNED_SHORT), crate::meta::inspect::EnumConstant::new("UNSIGNED_SHORT", "COMPONENT_TYPE_UNSIGNED_SHORT", GltfComponentType::UNSIGNED_SHORT), crate::meta::inspect::EnumConstant::new("SIGNED_INT", "COMPONENT_TYPE_SIGNED_INT", GltfComponentType::SIGNED_INT), crate::meta::inspect::EnumConstant::new("UNSIGNED_INT", "COMPONENT_TYPE_UNSIGNED_INT", GltfComponentType::UNSIGNED_INT), crate::meta::inspect::EnumConstant::new("SINGLE_FLOAT", "COMPONENT_TYPE_SINGLE_FLOAT", GltfComponentType::SINGLE_FLOAT), crate::meta::inspect::EnumConstant::new("DOUBLE_FLOAT", "COMPONENT_TYPE_DOUBLE_FLOAT", GltfComponentType::DOUBLE_FLOAT), crate::meta::inspect::EnumConstant::new("HALF_FLOAT", "COMPONENT_TYPE_HALF_FLOAT", GltfComponentType::HALF_FLOAT), crate::meta::inspect::EnumConstant::new("SIGNED_LONG", "COMPONENT_TYPE_SIGNED_LONG", GltfComponentType::SIGNED_LONG), crate::meta::inspect::EnumConstant::new("UNSIGNED_LONG", "COMPONENT_TYPE_UNSIGNED_LONG", GltfComponentType::UNSIGNED_LONG)]
        }
    }
}
impl crate::meta::GodotConvert for GltfComponentType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Component Type None", 0i64), EnumeratorShape::new_int("Component Type Signed Byte", 5120i64), EnumeratorShape::new_int("Component Type Unsigned Byte", 5121i64), EnumeratorShape::new_int("Component Type Signed Short", 5122i64), EnumeratorShape::new_int("Component Type Unsigned Short", 5123i64), EnumeratorShape::new_int("Component Type Signed Int", 5124i64), EnumeratorShape::new_int("Component Type Unsigned Int", 5125i64), EnumeratorShape::new_int("Component Type Single Float", 5126i64), EnumeratorShape::new_int("Component Type Double Float", 5130i64), EnumeratorShape::new_int("Component Type Half Float", 5131i64), EnumeratorShape::new_int("Component Type Signed Long", 5134i64), EnumeratorShape::new_int("Component Type Unsigned Long", 5135i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("GLTFAccessor.GLTFComponentType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for GltfComponentType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GltfComponentType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for GltfComponentType {
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
impl crate::registry::property::Export for GltfComponentType {
    
}
impl crate::meta::Element for GltfComponentType {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GltfAccessor;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for GltfAccessor {
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
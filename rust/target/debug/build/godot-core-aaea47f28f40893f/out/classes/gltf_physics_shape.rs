#![doc = "Sidecar module for class [`GltfPhysicsShape`][crate::classes::GltfPhysicsShape].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFPhysicsShape` enums](https://docs.godotengine.org/en/stable/classes/class_gltfphysicsshape.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFPhysicsShape`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`gltf_physics_shape`][crate::classes::gltf_physics_shape]: sidecar module with related enum/flag types\n* [`IGltfPhysicsShape`][crate::classes::IGltfPhysicsShape]: virtual methods\n\n\nSee also [Godot docs for `GLTFPhysicsShape`](https://docs.godotengine.org/en/stable/classes/class_gltfphysicsshape.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfPhysicsShape::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nRepresents a physics shape as defined by the `OMI_physics_shape` or `OMI_collider` glTF extensions. This class is an intermediary between the glTF data and Godot's nodes, and it's abstracted in a way that allows adding support for different glTF physics extensions in the future."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfPhysicsShape {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GltfPhysicsShape`][crate::classes::GltfPhysicsShape].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GLTFPhysicsShape` methods](https://docs.godotengine.org/en/stable/classes/class_gltfphysicsshape.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfPhysicsShape: crate::obj::GodotClass < Base = GltfPhysicsShape > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfPhysicsShape {
        #[doc = "Creates a new GLTFPhysicsShape instance from the given Godot [`CollisionShape3D`][crate::classes::CollisionShape3D] node."]
        pub fn from_node(shape_node: impl AsArg < Option < Gd < crate::classes::CollisionShape3D >> >,) -> Option < Gd < crate::classes::GltfPhysicsShape > > {
            type CallRet = Option < Gd < crate::classes::GltfPhysicsShape > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::CollisionShape3D > > >,);
            let args = (shape_node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "from_node", None, args,)
            }
        }
        #[doc = "Converts this GLTFPhysicsShape instance into a Godot [`CollisionShape3D`][crate::classes::CollisionShape3D] node."]
        pub(crate) fn to_node_full(&mut self, cache_shapes: bool,) -> Option < Gd < crate::classes::CollisionShape3D > > {
            type CallRet = Option < Gd < crate::classes::CollisionShape3D > >;
            type CallParams = (bool,);
            let args = (cache_shapes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4013usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "to_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`to_node_ex`][Self::to_node_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Converts this GLTFPhysicsShape instance into a Godot [`CollisionShape3D`][crate::classes::CollisionShape3D] node."]
        #[inline]
        pub fn to_node(&mut self,) -> Option < Gd < crate::classes::CollisionShape3D > > {
            self.to_node_ex() . done()
        }
        #[doc = "Converts this GLTFPhysicsShape instance into a Godot [`CollisionShape3D`][crate::classes::CollisionShape3D] node."]
        #[inline]
        pub fn to_node_ex < 'ex > (&'ex mut self,) -> ExToNode < 'ex > {
            ExToNode::new(self,)
        }
        #[doc = "Creates a new GLTFPhysicsShape instance from the given Godot [`Shape3D`][crate::classes::Shape3D] resource."]
        pub fn from_resource(shape_resource: impl AsArg < Option < Gd < crate::classes::Shape3D >> >,) -> Option < Gd < crate::classes::GltfPhysicsShape > > {
            type CallRet = Option < Gd < crate::classes::GltfPhysicsShape > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Shape3D > > >,);
            let args = (shape_resource.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4014usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "from_resource", None, args,)
            }
        }
        #[doc = "Converts this GLTFPhysicsShape instance into a Godot [`Shape3D`][crate::classes::Shape3D] resource."]
        pub(crate) fn to_resource_full(&mut self, cache_shapes: bool,) -> Option < Gd < crate::classes::Shape3D > > {
            type CallRet = Option < Gd < crate::classes::Shape3D > >;
            type CallParams = (bool,);
            let args = (cache_shapes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4015usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "to_resource", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`to_resource_ex`][Self::to_resource_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Converts this GLTFPhysicsShape instance into a Godot [`Shape3D`][crate::classes::Shape3D] resource."]
        #[inline]
        pub fn to_resource(&mut self,) -> Option < Gd < crate::classes::Shape3D > > {
            self.to_resource_ex() . done()
        }
        #[doc = "Converts this GLTFPhysicsShape instance into a Godot [`Shape3D`][crate::classes::Shape3D] resource."]
        #[inline]
        pub fn to_resource_ex < 'ex > (&'ex mut self,) -> ExToResource < 'ex > {
            ExToResource::new(self,)
        }
        #[doc = "Creates a new GLTFPhysicsShape instance by parsing the given [`Dictionary`][crate::builtin::Dictionary]."]
        pub fn from_dictionary(dictionary: &AnyDictionary,) -> Option < Gd < crate::classes::GltfPhysicsShape > > {
            type CallRet = Option < Gd < crate::classes::GltfPhysicsShape > >;
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(dictionary),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "from_dictionary", None, args,)
            }
        }
        #[doc = "Serializes this GLTFPhysicsShape instance into a [`Dictionary`][crate::builtin::Dictionary] in the format defined by `OMI_physics_shape`."]
        pub fn to_dictionary(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "to_dictionary", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shape_type(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4018usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_shape_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shape_type(&mut self, shape_type: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (shape_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4019usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_shape_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4020usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4021usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4022usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_radius(&mut self, radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4023usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_height(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4024usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_height(&mut self, height: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_is_trigger(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_is_trigger", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_is_trigger(&mut self, is_trigger: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (is_trigger,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_is_trigger", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mesh_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_mesh_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mesh_index(&mut self, mesh_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (mesh_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_mesh_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_importer_mesh(&self,) -> Option < Gd < crate::classes::ImporterMesh > > {
            type CallRet = Option < Gd < crate::classes::ImporterMesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "get_importer_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_importer_mesh(&mut self, importer_mesh: impl AsArg < Option < Gd < crate::classes::ImporterMesh >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ImporterMesh > > >,);
            let args = (importer_mesh.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfPhysicsShape", "set_importer_mesh", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for GltfPhysicsShape {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GLTFPhysicsShape"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfPhysicsShape {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfPhysicsShape {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfPhysicsShape {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfPhysicsShape {
        
    }
    impl crate::obj::cap::GodotDefault for GltfPhysicsShape {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfPhysicsShape {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfPhysicsShape {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfPhysicsShape`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GltfPhysicsShape__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfPhysicsShape > for $Class {
                
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
#[doc = "Default-param extender for [`GltfPhysicsShape::to_node_ex`][super::GltfPhysicsShape::to_node_ex]."]
#[must_use]
pub struct ExToNode < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::GltfPhysicsShape, cache_shapes: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExToNode < 'ex > {
    fn new(surround_object: &'ex mut re_export::GltfPhysicsShape,) -> Self {
        let cache_shapes = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, cache_shapes: cache_shapes,
        }
    }
    #[inline]
    pub fn cache_shapes(self, cache_shapes: bool) -> Self {
        Self {
            cache_shapes: cache_shapes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::CollisionShape3D > > {
        let Self {
            _phantom, surround_object, cache_shapes,
        }
        = self;
        re_export::GltfPhysicsShape::to_node_full(surround_object, cache_shapes,)
    }
}
#[doc = "Default-param extender for [`GltfPhysicsShape::to_resource_ex`][super::GltfPhysicsShape::to_resource_ex]."]
#[must_use]
pub struct ExToResource < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::GltfPhysicsShape, cache_shapes: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExToResource < 'ex > {
    fn new(surround_object: &'ex mut re_export::GltfPhysicsShape,) -> Self {
        let cache_shapes = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, cache_shapes: cache_shapes,
        }
    }
    #[inline]
    pub fn cache_shapes(self, cache_shapes: bool) -> Self {
        Self {
            cache_shapes: cache_shapes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Shape3D > > {
        let Self {
            _phantom, surround_object, cache_shapes,
        }
        = self;
        re_export::GltfPhysicsShape::to_resource_full(surround_object, cache_shapes,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GltfPhysicsShape;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for GltfPhysicsShape {
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
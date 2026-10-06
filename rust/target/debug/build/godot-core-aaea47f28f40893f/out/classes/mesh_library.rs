#![doc = "Sidecar module for class [`MeshLibrary`][crate::classes::MeshLibrary].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MeshLibrary` enums](https://docs.godotengine.org/en/stable/classes/class_meshlibrary.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MeshLibrary`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`IMeshLibrary`][crate::classes::IMeshLibrary]: virtual methods\n\n\nSee also [Godot docs for `MeshLibrary`](https://docs.godotengine.org/en/stable/classes/class_meshlibrary.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`MeshLibrary::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA library of meshes. Contains a list of [`Mesh`][crate::classes::Mesh] resources, each with a name and ID. Each item can also include collision and navigation shapes. This resource is used in [`GridMap`][crate::classes::GridMap]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MeshLibrary {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MeshLibrary`][crate::classes::MeshLibrary].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `MeshLibrary` methods](https://docs.godotengine.org/en/stable/classes/class_meshlibrary.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMeshLibrary: crate::obj::GodotClass < Base = MeshLibrary > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MeshLibrary {
        #[doc = "Creates a new item in the library with the given ID.\n\nYou can get an unused ID from [`get_last_unused_item_id`][`crate::classes::MeshLibrary::get_last_unused_item_id`]."]
        pub fn create_item(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "create_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the item's name.\n\nThis name is shown in the editor. It can also be used to look up the item later using [`find_item_by_name`][`crate::classes::MeshLibrary::find_item_by_name`]."]
        pub fn set_item_name(&mut self, id: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (id, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "set_item_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the item's mesh."]
        pub fn set_item_mesh(&mut self, id: i32, mesh: impl AsArg < Option < Gd < crate::classes::Mesh >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Mesh > > >,);
            let args = (id, mesh.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5018usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "set_item_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the transform to apply to the item's mesh."]
        pub fn set_item_mesh_transform(&mut self, id: i32, mesh_transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (i32, Transform3D,);
            let args = (id, mesh_transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5019usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "set_item_mesh_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the item's shadow casting mode to `shadow_casting_setting`."]
        pub fn set_item_mesh_cast_shadow(&mut self, id: i32, shadow_casting_setting: crate::classes::rendering_server::ShadowCastingSetting,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::rendering_server::ShadowCastingSetting,);
            let args = (id, shadow_casting_setting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5020usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "set_item_mesh_cast_shadow", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the transform to apply to the item's navigation mesh."]
        pub fn set_item_navigation_mesh_transform(&mut self, id: i32, navigation_mesh: Transform3D,) {
            type CallRet = ();
            type CallParams = (i32, Transform3D,);
            let args = (id, navigation_mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5021usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "set_item_navigation_mesh_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the item's navigation layers bitmask."]
        pub fn set_item_navigation_layers(&mut self, id: i32, navigation_layers: u32,) {
            type CallRet = ();
            type CallParams = (i32, u32,);
            let args = (id, navigation_layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5022usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "set_item_navigation_layers", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets an item's collision shapes.\n\nThe array should consist of [`Shape3D`][crate::classes::Shape3D] objects, each followed by a [`Transform3D`][crate::builtin::Transform3D] that will be applied to it. For shapes that should not have a transform, use `Transform3D.IDENTITY`."]
        pub fn set_item_shapes(&mut self, id: i32, shapes: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, AnyArray >,);
            let args = (id, RefArg::new(shapes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5023usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "set_item_shapes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a texture to use as the item's preview icon in the editor."]
        pub fn set_item_preview(&mut self, id: i32, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (id, texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5024usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "set_item_preview", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the item's name."]
        pub fn get_item_name(&self, id: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "get_item_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the item's mesh."]
        pub fn get_item_mesh(&self, id: i32,) -> Option < Gd < crate::classes::Mesh > > {
            type CallRet = Option < Gd < crate::classes::Mesh > >;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "get_item_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transform applied to the item's mesh."]
        pub fn get_item_mesh_transform(&self, id: i32,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "get_item_mesh_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the item's shadow casting mode."]
        pub fn get_item_mesh_cast_shadow(&self, id: i32,) -> crate::classes::rendering_server::ShadowCastingSetting {
            type CallRet = crate::classes::rendering_server::ShadowCastingSetting;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "get_item_mesh_cast_shadow", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transform applied to the item's navigation mesh."]
        pub fn get_item_navigation_mesh_transform(&self, id: i32,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "get_item_navigation_mesh_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the item's navigation layers bitmask."]
        pub fn get_item_navigation_layers(&self, id: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "get_item_navigation_layers", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an item's collision shapes.\n\nThe array consists of each [`Shape3D`][crate::classes::Shape3D] followed by its [`Transform3D`][crate::builtin::Transform3D]."]
        pub fn get_item_shapes(&self, id: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "get_item_shapes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "When running in the editor, returns a generated item preview (a 3D rendering in isometric perspective). When used in a running project, returns the manually-defined item preview which can be set using [`set_item_preview`][`crate::classes::MeshLibrary::set_item_preview`]. Returns an empty [`Texture2D`][crate::classes::Texture2D] if no preview was manually set in a running project."]
        pub fn get_item_preview(&self, id: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "get_item_preview", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the item."]
        pub fn remove_item(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "remove_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the first item with the given name, or `-1` if no item is found."]
        pub fn find_item_by_name(&self, name: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "find_item_by_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears the library."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of item IDs in use."]
        pub fn get_item_list(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "get_item_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets an unused ID for a new item."]
        pub fn get_last_unused_item_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshLibrary", "get_last_unused_item_id", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for MeshLibrary {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MeshLibrary"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MeshLibrary {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for MeshLibrary {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MeshLibrary {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MeshLibrary {
        
    }
    impl crate::obj::cap::GodotDefault for MeshLibrary {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MeshLibrary {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MeshLibrary {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MeshLibrary`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MeshLibrary__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MeshLibrary > for $Class {
                
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
    use super::re_export::MeshLibrary;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for MeshLibrary {
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
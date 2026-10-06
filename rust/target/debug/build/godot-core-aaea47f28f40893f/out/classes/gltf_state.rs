#![doc = "Sidecar module for class [`GltfState`][crate::classes::GltfState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFState` enums](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFState`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`gltf_state`][crate::classes::gltf_state]: sidecar module with related enum/flag types\n* [`IGltfState`][crate::classes::IGltfState]: virtual methods\n\n\nSee also [Godot docs for `GLTFState`](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfState::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nContains all nodes and resources of a glTF file. This is used by [`GLTFDocument`][crate::classes::GltfDocument] as data storage, which allows [`GLTFDocument`][crate::classes::GltfDocument] and all [`GLTFDocumentExtension`][crate::classes::GltfDocumentExtension] classes to remain stateless.\n\nGLTFState can be populated by [`GLTFDocument`][crate::classes::GltfDocument] reading a file or by converting a Godot scene. Then the data can either be used to create a Godot scene or save to a glTF file. The code that converts to/from a Godot scene can be intercepted at arbitrary points by [`GLTFDocumentExtension`][crate::classes::GltfDocumentExtension] classes. This allows for custom data to be stored in the glTF file or for custom data to be converted to/from Godot nodes."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfState {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GltfState`][crate::classes::GltfState].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GLTFState` methods](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfState: crate::obj::GodotClass < Base = GltfState > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfState {
        #[doc = "Appends an extension to the list of extensions used by this glTF file during serialization. If `required` is `true`, the extension will also be added to the list of required extensions. Do not run this in [`export_post`][`crate::classes::IGltfDocumentExtension::export_post`], as that stage is too late to add extensions. The final list is sorted alphabetically."]
        pub fn add_used_extension(&mut self, extension_name: impl AsArg < GString >, required: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (extension_name.into_arg(), required,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "add_used_extension", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Appends the given byte array `data` to the buffers and creates a [`GLTFBufferView`][crate::classes::GltfBufferView] for it. The index of the destination [`GLTFBufferView`][crate::classes::GltfBufferView] is returned. If `deduplication` is `true`, the buffers are first searched for duplicate data, otherwise new bytes are always appended."]
        pub fn append_data_to_buffers(&mut self, data: &PackedByteArray, deduplication: bool,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >, bool,);
            let args = (RefArg::new(data), deduplication,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "append_data_to_buffers", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Appends the given [`GLTFNode`][crate::classes::GltfNode] to the state, and returns its new index. This can be used to export one Godot node as multiple glTF nodes, or inject new glTF nodes at import time. On import, this must be called before [`generate_scene_node`][`crate::classes::IGltfDocumentExtension::generate_scene_node`] finishes for the parent node. On export, this must be called before [`export_node`][`crate::classes::IGltfDocumentExtension::export_node`] runs for the parent node.\n\nThe `godot_scene_node` parameter is the Godot scene node that corresponds to this glTF node. This is highly recommended to be set to a valid node, but may be `null` if there is no corresponding Godot scene node. One Godot scene node may be used for multiple glTF nodes, so if exporting multiple glTF nodes for one Godot scene node, use the same Godot scene node for each.\n\nThe `parent_node_index` parameter is the index of the parent [`GLTFNode`][crate::classes::GltfNode] in the state. If `-1`, the node will be a root node, otherwise the new node will be added to the parent's list of children. The index will also be written to the \\[member GLTFNode.parent] property of the new node."]
        pub fn append_gltf_node(&mut self, gltf_node: impl AsArg < Option < Gd < crate::classes::GltfNode >> >, godot_scene_node: impl AsArg < Option < Gd < crate::classes::Node >> >, parent_node_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::GltfNode > > >, CowArg < 'a1, Option < Gd < crate::classes::Node > > >, i32,);
            let args = (gltf_node.into_arg(), godot_scene_node.into_arg(), parent_node_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "append_gltf_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_json(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_json", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_json(&mut self, json: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(json),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_json", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_major_version(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_major_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_major_version(&mut self, major_version: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (major_version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_major_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_minor_version(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_minor_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_minor_version(&mut self, minor_version: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (minor_version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_minor_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_copyright(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_copyright", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_copyright(&mut self, copyright: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (copyright.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_copyright", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glb_data(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_glb_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glb_data(&mut self, glb_data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(glb_data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_glb_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_named_skin_binds(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_use_named_skin_binds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_named_skin_binds(&mut self, use_named_skin_binds: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_named_skin_binds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_use_named_skin_binds", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all [`GLTFNode`][crate::classes::GltfNode]s in the glTF file. These are the nodes that \\[member GLTFNode.children] and \\[member root_nodes] refer to. This includes nodes that may not be generated in the Godot scene, or nodes that may generate multiple Godot scene nodes."]
        pub fn get_nodes(&self,) -> Array < Gd < crate::classes::GltfNode > > {
            type CallRet = Array < Gd < crate::classes::GltfNode > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_nodes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`GLTFNode`][crate::classes::GltfNode]s in the state. These are the nodes that \\[member GLTFNode.children] and \\[member root_nodes] refer to. Some of the nodes set here may not be generated in the Godot scene, or may generate multiple Godot scene nodes."]
        pub fn set_nodes(&mut self, nodes: &Array < Gd < crate::classes::GltfNode > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfNode > > >,);
            let args = (RefArg::new(nodes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_buffers(&self,) -> Array < PackedByteArray > {
            type CallRet = Array < PackedByteArray >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_buffers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_buffers(&mut self, buffers: &Array < PackedByteArray >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < PackedByteArray > >,);
            let args = (RefArg::new(buffers),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_buffers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_buffer_views(&self,) -> Array < Gd < crate::classes::GltfBufferView > > {
            type CallRet = Array < Gd < crate::classes::GltfBufferView > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_buffer_views", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_buffer_views(&mut self, buffer_views: &Array < Gd < crate::classes::GltfBufferView > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfBufferView > > >,);
            let args = (RefArg::new(buffer_views),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_buffer_views", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessors(&self,) -> Array < Gd < crate::classes::GltfAccessor > > {
            type CallRet = Array < Gd < crate::classes::GltfAccessor > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_accessors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accessors(&mut self, accessors: &Array < Gd < crate::classes::GltfAccessor > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfAccessor > > >,);
            let args = (RefArg::new(accessors),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_accessors", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all [`GLTFMesh`][crate::classes::GltfMesh]es in the glTF file. These are the meshes that the \\[member GLTFNode.mesh] index refers to."]
        pub fn get_meshes(&self,) -> Array < Gd < crate::classes::GltfMesh > > {
            type CallRet = Array < Gd < crate::classes::GltfMesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_meshes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`GLTFMesh`][crate::classes::GltfMesh]es in the state. These are the meshes that the \\[member GLTFNode.mesh] index refers to."]
        pub fn set_meshes(&mut self, meshes: &Array < Gd < crate::classes::GltfMesh > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfMesh > > >,);
            let args = (RefArg::new(meshes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_meshes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of [`AnimationPlayer`][crate::classes::AnimationPlayer] nodes in this `GLTFState`. These nodes are only used during the export process when converting Godot [`AnimationPlayer`][crate::classes::AnimationPlayer] nodes to glTF animations."]
        pub fn get_animation_players_count(&self, anim_player_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (anim_player_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_animation_players_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`AnimationPlayer`][crate::classes::AnimationPlayer] node with the given index. These nodes are only used during the export process when converting Godot [`AnimationPlayer`][crate::classes::AnimationPlayer] nodes to glTF animations."]
        pub fn get_animation_player(&self, anim_player_index: i32,) -> Option < Gd < crate::classes::AnimationPlayer > > {
            type CallRet = Option < Gd < crate::classes::AnimationPlayer > >;
            type CallParams = (i32,);
            let args = (anim_player_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_animation_player", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_materials(&self,) -> Array < Gd < crate::classes::Material > > {
            type CallRet = Array < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_materials", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_materials(&mut self, materials: &Array < Gd < crate::classes::Material > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::Material > > >,);
            let args = (RefArg::new(materials),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_materials", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scene_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_scene_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scene_name(&mut self, scene_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (scene_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_scene_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_base_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_base_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_base_path(&mut self, base_path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (base_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_base_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_filename(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_filename", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_filename(&mut self, filename: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (filename.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_filename", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_nodes(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_root_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_root_nodes(&mut self, root_nodes: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedInt32Array >,);
            let args = (RefArg::new(root_nodes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_root_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_textures(&self,) -> Array < Gd < crate::classes::GltfTexture > > {
            type CallRet = Array < Gd < crate::classes::GltfTexture > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_textures", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_textures(&mut self, textures: &Array < Gd < crate::classes::GltfTexture > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfTexture > > >,);
            let args = (RefArg::new(textures),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_textures", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Retrieves the array of texture samplers that are used by the textures contained in the glTF."]
        pub fn get_texture_samplers(&self,) -> Array < Gd < crate::classes::GltfTextureSampler > > {
            type CallRet = Array < Gd < crate::classes::GltfTextureSampler > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_texture_samplers", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the array of texture samplers that are used by the textures contained in the glTF."]
        pub fn set_texture_samplers(&mut self, texture_samplers: &Array < Gd < crate::classes::GltfTextureSampler > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfTextureSampler > > >,);
            let args = (RefArg::new(texture_samplers),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_texture_samplers", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the images of the glTF file as an array of [`Texture2D`][crate::classes::Texture2D]s. These are the images that the \\[member GLTFTexture.src_image] index refers to."]
        pub fn get_images(&self,) -> Array < Gd < crate::classes::Texture2D > > {
            type CallRet = Array < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_images", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the images in the state stored as an array of [`Texture2D`][crate::classes::Texture2D]s. This can be used during export. These are the images that the \\[member GLTFTexture.src_image] index refers to."]
        pub fn set_images(&mut self, images: &Array < Gd < crate::classes::Texture2D > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::Texture2D > > >,);
            let args = (RefArg::new(images),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_images", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all [`GLTFSkin`][crate::classes::GltfSkin]s in the glTF file. These are the skins that the \\[member GLTFNode.skin] index refers to."]
        pub fn get_skins(&self,) -> Array < Gd < crate::classes::GltfSkin > > {
            type CallRet = Array < Gd < crate::classes::GltfSkin > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_skins", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`GLTFSkin`][crate::classes::GltfSkin]s in the state. These are the skins that the \\[member GLTFNode.skin] index refers to."]
        pub fn set_skins(&mut self, skins: &Array < Gd < crate::classes::GltfSkin > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfSkin > > >,);
            let args = (RefArg::new(skins),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_skins", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all [`GLTFCamera`][crate::classes::GltfCamera]s in the glTF file. These are the cameras that the \\[member GLTFNode.camera] index refers to."]
        pub fn get_cameras(&self,) -> Array < Gd < crate::classes::GltfCamera > > {
            type CallRet = Array < Gd < crate::classes::GltfCamera > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_cameras", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`GLTFCamera`][crate::classes::GltfCamera]s in the state. These are the cameras that the \\[member GLTFNode.camera] index refers to."]
        pub fn set_cameras(&mut self, cameras: &Array < Gd < crate::classes::GltfCamera > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfCamera > > >,);
            let args = (RefArg::new(cameras),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_cameras", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all [`GLTFLight`][crate::classes::GltfLight]s in the glTF file. These are the lights that the \\[member GLTFNode.light] index refers to."]
        pub fn get_lights(&self,) -> Array < Gd < crate::classes::GltfLight > > {
            type CallRet = Array < Gd < crate::classes::GltfLight > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_lights", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`GLTFLight`][crate::classes::GltfLight]s in the state. These are the lights that the \\[member GLTFNode.light] index refers to."]
        pub fn set_lights(&mut self, lights: &Array < Gd < crate::classes::GltfLight > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfLight > > >,);
            let args = (RefArg::new(lights),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_lights", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of unique node names. This is used in both the import process and export process."]
        pub fn get_unique_names(&self,) -> Array < GString > {
            type CallRet = Array < GString >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_unique_names", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the unique node names in the state. This is used in both the import process and export process."]
        pub fn set_unique_names(&mut self, unique_names: &Array < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < GString > >,);
            let args = (RefArg::new(unique_names),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_unique_names", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of unique animation names. This is only used during the import process."]
        pub fn get_unique_animation_names(&self,) -> Array < GString > {
            type CallRet = Array < GString >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_unique_animation_names", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the unique animation names in the state. This is only used during the import process."]
        pub fn set_unique_animation_names(&mut self, unique_animation_names: &Array < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < GString > >,);
            let args = (RefArg::new(unique_animation_names),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_unique_animation_names", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all [`GLTFSkeleton`][crate::classes::GltfSkeleton]s in the glTF file. These are the skeletons that the \\[member GLTFNode.skeleton] index refers to."]
        pub fn get_skeletons(&self,) -> Array < Gd < crate::classes::GltfSkeleton > > {
            type CallRet = Array < Gd < crate::classes::GltfSkeleton > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_skeletons", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`GLTFSkeleton`][crate::classes::GltfSkeleton]s in the state. These are the skeletons that the \\[member GLTFNode.skeleton] index refers to."]
        pub fn set_skeletons(&mut self, skeletons: &Array < Gd < crate::classes::GltfSkeleton > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfSkeleton > > >,);
            let args = (RefArg::new(skeletons),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_skeletons", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_create_animations(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_create_animations", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_create_animations(&mut self, create_animations: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (create_animations,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_create_animations", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_import_as_skeleton_bones(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_import_as_skeleton_bones", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_import_as_skeleton_bones(&mut self, import_as_skeleton_bones: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (import_as_skeleton_bones,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_import_as_skeleton_bones", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all [`GLTFAnimation`][crate::classes::GltfAnimation]s in the glTF file. When importing, these will be generated as animations in an [`AnimationPlayer`][crate::classes::AnimationPlayer] node. When exporting, these will be generated from Godot [`AnimationPlayer`][crate::classes::AnimationPlayer] nodes."]
        pub fn get_animations(&self,) -> Array < Gd < crate::classes::GltfAnimation > > {
            type CallRet = Array < Gd < crate::classes::GltfAnimation > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_animations", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`GLTFAnimation`][crate::classes::GltfAnimation]s in the state. When importing, these will be generated as animations in an [`AnimationPlayer`][crate::classes::AnimationPlayer] node. When exporting, these will be generated from Godot [`AnimationPlayer`][crate::classes::AnimationPlayer] nodes."]
        pub fn set_animations(&mut self, animations: &Array < Gd < crate::classes::GltfAnimation > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfAnimation > > >,);
            let args = (RefArg::new(animations),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_animations", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the Godot scene node that corresponds to the same index as the [`GLTFNode`][crate::classes::GltfNode] it was generated from. This is the inverse of [`get_node_index`][`crate::classes::GltfState::get_node_index`]. Useful during the import process.\n\n**Note:** Not every [`GLTFNode`][crate::classes::GltfNode] will have a scene node generated, and not every generated scene node will have a corresponding [`GLTFNode`][crate::classes::GltfNode]. If there is no scene node for this [`GLTFNode`][crate::classes::GltfNode] index, `null` is returned."]
        pub fn get_scene_node(&self, gltf_node_index: i32,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = (i32,);
            let args = (gltf_node_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_scene_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the [`GLTFNode`][crate::classes::GltfNode] corresponding to this Godot scene node. This is the inverse of [`get_scene_node`][`crate::classes::GltfState::get_scene_node`]. Useful during the export process.\n\n**Note:** Not every Godot scene node will have a corresponding [`GLTFNode`][crate::classes::GltfNode], and not every [`GLTFNode`][crate::classes::GltfNode] will have a scene node generated. If there is no [`GLTFNode`][crate::classes::GltfNode] index for this scene node, `-1` is returned."]
        pub fn get_node_index(&self, scene_node: impl AsArg < Option < Gd < crate::classes::Node >> >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (scene_node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_node_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets additional arbitrary data in this `GLTFState` instance. This can be used to keep per-file state data in [`GLTFDocumentExtension`][crate::classes::GltfDocumentExtension] classes, which is important because they are stateless.\n\nThe argument should be the [`GLTFDocumentExtension`][crate::classes::GltfDocumentExtension] name (does not have to match the extension name in the glTF file), and the return value can be anything you set. If nothing was set, the return value is `null`."]
        pub fn get_additional_data(&self, extension_name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (extension_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_additional_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets additional arbitrary data in this `GLTFState` instance. This can be used to keep per-file state data in [`GLTFDocumentExtension`][crate::classes::GltfDocumentExtension] classes, which is important because they are stateless.\n\nThe first argument should be the [`GLTFDocumentExtension`][crate::classes::GltfDocumentExtension] name (does not have to match the extension name in the glTF file), and the second argument can be anything you want."]
        pub fn set_additional_data(&mut self, extension_name: impl AsArg < StringName >, additional_data: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (extension_name.into_arg(), RefArg::new(additional_data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_additional_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_handle_binary_image_mode(&self,) -> crate::classes::gltf_state::HandleBinaryImageMode {
            type CallRet = crate::classes::gltf_state::HandleBinaryImageMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_handle_binary_image_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_handle_binary_image_mode(&mut self, method: crate::classes::gltf_state::HandleBinaryImageMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::gltf_state::HandleBinaryImageMode,);
            let args = (method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_handle_binary_image_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bake_fps(&mut self, value: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_bake_fps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bake_fps(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_bake_fps", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deprecated untyped alias for \\[member handle_binary_image_mode]. When importing a glTF file with unimported raw binary images embedded inside of binary blob buffers, in data URIs, or separate files not imported by Godot, this controls how the images are handled."]
        pub fn get_handle_binary_image(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_handle_binary_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deprecated untyped alias for \\[member handle_binary_image_mode]. When importing a glTF file with unimported raw binary images embedded inside of binary blob buffers, in data URIs, or separate files not imported by Godot, this controls how the images are handled."]
        pub fn set_handle_binary_image(&mut self, method: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_handle_binary_image", Some(self.__validated_obj()), args,)
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
        pub const HANDLE_BINARY_DISCARD_TEXTURES: i32 = 0i32;
        pub const HANDLE_BINARY_EXTRACT_TEXTURES: i32 = 1i32;
        pub const HANDLE_BINARY_EMBED_AS_BASISU: i32 = 2i32;
        pub const HANDLE_BINARY_EMBED_AS_UNCOMPRESSED: i32 = 3i32;
        
    }
    impl crate::obj::GodotClass for GltfState {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GLTFState"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfState {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfState {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfState {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfState {
        
    }
    impl crate::obj::cap::GodotDefault for GltfState {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfState {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfState`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GltfState__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfState > for $Class {
                
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
pub struct HandleBinaryImageMode {
    ord: i32
}
impl HandleBinaryImageMode {
    #[doc(alias = "HANDLE_BINARY_IMAGE_MODE_DISCARD_TEXTURES")]
    #[doc = "Godot enumerator name: `HANDLE_BINARY_IMAGE_MODE_DISCARD_TEXTURES`"]
    pub const DISCARD_TEXTURES: HandleBinaryImageMode = HandleBinaryImageMode {
        ord: 0i32
    };
    #[doc(alias = "HANDLE_BINARY_IMAGE_MODE_EXTRACT_TEXTURES")]
    #[doc = "Godot enumerator name: `HANDLE_BINARY_IMAGE_MODE_EXTRACT_TEXTURES`"]
    pub const EXTRACT_TEXTURES: HandleBinaryImageMode = HandleBinaryImageMode {
        ord: 1i32
    };
    #[doc(alias = "HANDLE_BINARY_IMAGE_MODE_EMBED_AS_BASISU")]
    #[doc = "Godot enumerator name: `HANDLE_BINARY_IMAGE_MODE_EMBED_AS_BASISU`"]
    pub const EMBED_AS_BASISU: HandleBinaryImageMode = HandleBinaryImageMode {
        ord: 2i32
    };
    #[doc(alias = "HANDLE_BINARY_IMAGE_MODE_EMBED_AS_UNCOMPRESSED")]
    #[doc = "Godot enumerator name: `HANDLE_BINARY_IMAGE_MODE_EMBED_AS_UNCOMPRESSED`"]
    pub const EMBED_AS_UNCOMPRESSED: HandleBinaryImageMode = HandleBinaryImageMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for HandleBinaryImageMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HandleBinaryImageMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HandleBinaryImageMode {
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
            Self::DISCARD_TEXTURES => "DISCARD_TEXTURES", Self::EXTRACT_TEXTURES => "EXTRACT_TEXTURES", Self::EMBED_AS_BASISU => "EMBED_AS_BASISU", Self::EMBED_AS_UNCOMPRESSED => "EMBED_AS_UNCOMPRESSED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[HandleBinaryImageMode::DISCARD_TEXTURES, HandleBinaryImageMode::EXTRACT_TEXTURES, HandleBinaryImageMode::EMBED_AS_BASISU, HandleBinaryImageMode::EMBED_AS_UNCOMPRESSED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < HandleBinaryImageMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISCARD_TEXTURES", "HANDLE_BINARY_IMAGE_MODE_DISCARD_TEXTURES", HandleBinaryImageMode::DISCARD_TEXTURES), crate::meta::inspect::EnumConstant::new("EXTRACT_TEXTURES", "HANDLE_BINARY_IMAGE_MODE_EXTRACT_TEXTURES", HandleBinaryImageMode::EXTRACT_TEXTURES), crate::meta::inspect::EnumConstant::new("EMBED_AS_BASISU", "HANDLE_BINARY_IMAGE_MODE_EMBED_AS_BASISU", HandleBinaryImageMode::EMBED_AS_BASISU), crate::meta::inspect::EnumConstant::new("EMBED_AS_UNCOMPRESSED", "HANDLE_BINARY_IMAGE_MODE_EMBED_AS_UNCOMPRESSED", HandleBinaryImageMode::EMBED_AS_UNCOMPRESSED)]
        }
    }
}
impl crate::meta::GodotConvert for HandleBinaryImageMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Handle Binary Image Mode Discard Textures", 0i64), EnumeratorShape::new_int("Handle Binary Image Mode Extract Textures", 1i64), EnumeratorShape::new_int("Handle Binary Image Mode Embed As Basisu", 2i64), EnumeratorShape::new_int("Handle Binary Image Mode Embed As Uncompressed", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("GLTFState.HandleBinaryImageMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for HandleBinaryImageMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandleBinaryImageMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for HandleBinaryImageMode {
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
impl crate::registry::property::Export for HandleBinaryImageMode {
    
}
impl crate::meta::Element for HandleBinaryImageMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GltfState;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for GltfState {
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
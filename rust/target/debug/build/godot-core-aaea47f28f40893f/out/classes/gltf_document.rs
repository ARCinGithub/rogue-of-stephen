#![doc = "Sidecar module for class [`GltfDocument`][crate::classes::GltfDocument].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFDocument` enums](https://docs.godotengine.org/en/stable/classes/class_gltfdocument.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFDocument`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`gltf_document`][crate::classes::gltf_document]: sidecar module with related enum/flag types\n* [`IGltfDocument`][crate::classes::IGltfDocument]: virtual methods\n\n\nSee also [Godot docs for `GLTFDocument`](https://docs.godotengine.org/en/stable/classes/class_gltfdocument.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfDocument::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nGLTFDocument supports reading data from a glTF file, buffer, or Godot scene. This data can then be written to the filesystem, buffer, or used to create a Godot scene.\n\nAll of the data in a glTF scene is stored in the [`GLTFState`][crate::classes::GltfState] class. GLTFDocument processes state objects, but does not contain any scene data itself. GLTFDocument has member variables to store export configuration settings such as the image format, but is otherwise stateless. Multiple scenes can be processed with the same settings using the same GLTFDocument object and different [`GLTFState`][crate::classes::GltfState] objects.\n\nGLTFDocument can be extended with arbitrary functionality by extending the [`GLTFDocumentExtension`][crate::classes::GltfDocumentExtension] class and registering it with GLTFDocument via [`register_gltf_document_extension`][`crate::classes::GltfDocument::register_gltf_document_extension`]. This allows for custom data to be imported and exported."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfDocument {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GltfDocument`][crate::classes::GltfDocument].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GLTFDocument` methods](https://docs.godotengine.org/en/stable/classes/class_gltfdocument.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfDocument: crate::obj::GodotClass < Base = GltfDocument > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfDocument {
        pub fn set_image_format(&mut self, image_format: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (image_format.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "set_image_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_image_format(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "get_image_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lossy_quality(&mut self, lossy_quality: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (lossy_quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "set_lossy_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lossy_quality(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "get_lossy_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fallback_image_format(&mut self, fallback_image_format: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (fallback_image_format.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "set_fallback_image_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fallback_image_format(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "get_fallback_image_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fallback_image_quality(&mut self, fallback_image_quality: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (fallback_image_quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "set_fallback_image_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fallback_image_quality(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "get_fallback_image_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_root_node_mode(&mut self, root_node_mode: crate::classes::gltf_document::RootNodeMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::gltf_document::RootNodeMode,);
            let args = (root_node_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "set_root_node_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_node_mode(&self,) -> crate::classes::gltf_document::RootNodeMode {
            type CallRet = crate::classes::gltf_document::RootNodeMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "get_root_node_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_mode(&mut self, visibility_mode: crate::classes::gltf_document::VisibilityMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::gltf_document::VisibilityMode,);
            let args = (visibility_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "set_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_mode(&self,) -> crate::classes::gltf_document::VisibilityMode {
            type CallRet = crate::classes::gltf_document::VisibilityMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "get_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Takes a path to a glTF file and imports the data at that file path to the given [`GLTFState`][crate::classes::GltfState] object through the `state` parameter.\n\n**Note:** The `base_path` tells [`append_from_file`][`crate::classes::GltfDocument::append_from_file`] where to find dependencies and can be empty."]
        pub(crate) fn append_from_file_full(&mut self, path: CowArg < GString >, state: CowArg < Option < Gd < crate::classes::GltfState > > >, flags: u32, base_path: CowArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::GltfState > > >, u32, CowArg < 'a2, GString >,);
            let args = (path, state, flags, base_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "append_from_file", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`append_from_file_ex`][Self::append_from_file_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Takes a path to a glTF file and imports the data at that file path to the given [`GLTFState`][crate::classes::GltfState] object through the `state` parameter.\n\n**Note:** The `base_path` tells [`append_from_file`][`crate::classes::GltfDocument::append_from_file`] where to find dependencies and can be empty."]
        #[inline]
        pub fn append_from_file(&mut self, path: impl AsArg < GString >, state: impl AsArg < Option < Gd < crate::classes::GltfState >> >,) -> crate::global::Error {
            self.append_from_file_ex(path, state,) . done()
        }
        #[doc = "Takes a path to a glTF file and imports the data at that file path to the given [`GLTFState`][crate::classes::GltfState] object through the `state` parameter.\n\n**Note:** The `base_path` tells [`append_from_file`][`crate::classes::GltfDocument::append_from_file`] where to find dependencies and can be empty."]
        #[inline]
        pub fn append_from_file_ex < 'ex > (&'ex mut self, path: impl AsArg < GString > + 'ex, state: impl AsArg < Option < Gd < crate::classes::GltfState >> > + 'ex,) -> ExAppendFromFile < 'ex > {
            ExAppendFromFile::new(self, path, state,)
        }
        #[doc = "Takes a [`PackedByteArray`][crate::builtin::PackedByteArray] defining a glTF and imports the data to the given [`GLTFState`][crate::classes::GltfState] object through the `state` parameter.\n\n**Note:** The `base_path` tells [`append_from_buffer`][`crate::classes::GltfDocument::append_from_buffer`] where to find dependencies and can be empty."]
        pub(crate) fn append_from_buffer_full(&mut self, bytes: RefArg < PackedByteArray >, base_path: CowArg < GString >, state: CowArg < Option < Gd < crate::classes::GltfState > > >, flags: u32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, PackedByteArray >, CowArg < 'a1, GString >, CowArg < 'a2, Option < Gd < crate::classes::GltfState > > >, u32,);
            let args = (bytes, base_path, state, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "append_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`append_from_buffer_ex`][Self::append_from_buffer_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Takes a [`PackedByteArray`][crate::builtin::PackedByteArray] defining a glTF and imports the data to the given [`GLTFState`][crate::classes::GltfState] object through the `state` parameter.\n\n**Note:** The `base_path` tells [`append_from_buffer`][`crate::classes::GltfDocument::append_from_buffer`] where to find dependencies and can be empty."]
        #[inline]
        pub fn append_from_buffer(&mut self, bytes: &PackedByteArray, base_path: impl AsArg < GString >, state: impl AsArg < Option < Gd < crate::classes::GltfState >> >,) -> crate::global::Error {
            self.append_from_buffer_ex(bytes, base_path, state,) . done()
        }
        #[doc = "Takes a [`PackedByteArray`][crate::builtin::PackedByteArray] defining a glTF and imports the data to the given [`GLTFState`][crate::classes::GltfState] object through the `state` parameter.\n\n**Note:** The `base_path` tells [`append_from_buffer`][`crate::classes::GltfDocument::append_from_buffer`] where to find dependencies and can be empty."]
        #[inline]
        pub fn append_from_buffer_ex < 'ex > (&'ex mut self, bytes: &'ex PackedByteArray, base_path: impl AsArg < GString > + 'ex, state: impl AsArg < Option < Gd < crate::classes::GltfState >> > + 'ex,) -> ExAppendFromBuffer < 'ex > {
            ExAppendFromBuffer::new(self, bytes, base_path, state,)
        }
        #[doc = "Takes a Godot Engine scene node and exports it and its descendants to the given [`GLTFState`][crate::classes::GltfState] object through the `state` parameter."]
        pub(crate) fn append_from_scene_full(&mut self, node: CowArg < Option < Gd < crate::classes::Node > > >, state: CowArg < Option < Gd < crate::classes::GltfState > > >, flags: u32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, CowArg < 'a1, Option < Gd < crate::classes::GltfState > > >, u32,);
            let args = (node, state, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "append_from_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`append_from_scene_ex`][Self::append_from_scene_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Takes a Godot Engine scene node and exports it and its descendants to the given [`GLTFState`][crate::classes::GltfState] object through the `state` parameter."]
        #[inline]
        pub fn append_from_scene(&mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> >, state: impl AsArg < Option < Gd < crate::classes::GltfState >> >,) -> crate::global::Error {
            self.append_from_scene_ex(node, state,) . done()
        }
        #[doc = "Takes a Godot Engine scene node and exports it and its descendants to the given [`GLTFState`][crate::classes::GltfState] object through the `state` parameter."]
        #[inline]
        pub fn append_from_scene_ex < 'ex > (&'ex mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'ex, state: impl AsArg < Option < Gd < crate::classes::GltfState >> > + 'ex,) -> ExAppendFromScene < 'ex > {
            ExAppendFromScene::new(self, node, state,)
        }
        #[doc = "Takes a [`GLTFState`][crate::classes::GltfState] object through the `state` parameter and returns a Godot Engine scene node.\n\nThe `bake_fps` parameter overrides the bake_fps in `state`."]
        pub(crate) fn generate_scene_full(&mut self, state: CowArg < Option < Gd < crate::classes::GltfState > > >, bake_fps: f32, trimming: bool, remove_immutable_tracks: bool,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::GltfState > > >, f32, bool, bool,);
            let args = (state, bake_fps, trimming, remove_immutable_tracks,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "generate_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`generate_scene_ex`][Self::generate_scene_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Takes a [`GLTFState`][crate::classes::GltfState] object through the `state` parameter and returns a Godot Engine scene node.\n\nThe `bake_fps` parameter overrides the bake_fps in `state`."]
        #[inline]
        pub fn generate_scene(&mut self, state: impl AsArg < Option < Gd < crate::classes::GltfState >> >,) -> Option < Gd < crate::classes::Node > > {
            self.generate_scene_ex(state,) . done()
        }
        #[doc = "Takes a [`GLTFState`][crate::classes::GltfState] object through the `state` parameter and returns a Godot Engine scene node.\n\nThe `bake_fps` parameter overrides the bake_fps in `state`."]
        #[inline]
        pub fn generate_scene_ex < 'ex > (&'ex mut self, state: impl AsArg < Option < Gd < crate::classes::GltfState >> > + 'ex,) -> ExGenerateScene < 'ex > {
            ExGenerateScene::new(self, state,)
        }
        #[doc = "Takes a [`GLTFState`][crate::classes::GltfState] object through the `state` parameter and returns a glTF [`PackedByteArray`][crate::builtin::PackedByteArray]."]
        pub fn generate_buffer(&mut self, state: impl AsArg < Option < Gd < crate::classes::GltfState >> >,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::GltfState > > >,);
            let args = (state.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "generate_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Takes a [`GLTFState`][crate::classes::GltfState] object through the `state` parameter and writes a glTF file to the filesystem.\n\n**Note:** The extension of the glTF file determines if it is a .glb binary file or a .gltf text file."]
        pub fn write_to_filesystem(&mut self, state: impl AsArg < Option < Gd < crate::classes::GltfState >> >, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::GltfState > > >, CowArg < 'a1, GString >,);
            let args = (state.into_arg(), path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "write_to_filesystem", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Determines a mapping between the given glTF Object Model `json_pointer` and the corresponding Godot node path(s) in the generated Godot scene. The details of this mapping are returned in a [`GLTFObjectModelProperty`][crate::classes::GltfObjectModelProperty] object. Additional mappings can be supplied via the [`export_object_model_property`][`crate::classes::IGltfDocumentExtension::export_object_model_property`] callback method."]
        pub fn import_object_model_property(state: impl AsArg < Option < Gd < crate::classes::GltfState >> >, json_pointer: impl AsArg < GString >,) -> Option < Gd < crate::classes::GltfObjectModelProperty > > {
            type CallRet = Option < Gd < crate::classes::GltfObjectModelProperty > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::GltfState > > >, CowArg < 'a1, GString >,);
            let args = (state.into_arg(), json_pointer.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "import_object_model_property", None, args,)
            }
        }
        #[doc = "Determines a mapping between the given Godot `node_path` and the corresponding glTF Object Model JSON pointer(s) in the generated glTF file. The details of this mapping are returned in a [`GLTFObjectModelProperty`][crate::classes::GltfObjectModelProperty] object. Additional mappings can be supplied via the [`import_object_model_property`][`crate::classes::IGltfDocumentExtension::import_object_model_property`] callback method."]
        pub fn export_object_model_property(state: impl AsArg < Option < Gd < crate::classes::GltfState >> >, node_path: impl AsArg < NodePath >, godot_node: impl AsArg < Option < Gd < crate::classes::Node >> >, gltf_node_index: i32,) -> Option < Gd < crate::classes::GltfObjectModelProperty > > {
            type CallRet = Option < Gd < crate::classes::GltfObjectModelProperty > >;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::GltfState > > >, CowArg < 'a1, NodePath >, CowArg < 'a2, Option < Gd < crate::classes::Node > > >, i32,);
            let args = (state.into_arg(), node_path.into_arg(), godot_node.into_arg(), gltf_node_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "export_object_model_property", None, args,)
            }
        }
        #[doc = "Registers the given [`GLTFDocumentExtension`][crate::classes::GltfDocumentExtension] instance with GLTFDocument. If `first_priority` is `true`, this extension will be run first. Otherwise, it will be run last.\n\n**Note:** Like GLTFDocument itself, all GLTFDocumentExtension classes must be stateless in order to function properly. If you need to store data, use the `set_additional_data` and `get_additional_data` methods in [`GLTFState`][crate::classes::GltfState] or [`GLTFNode`][crate::classes::GltfNode]."]
        pub(crate) fn register_gltf_document_extension_full(extension: CowArg < Option < Gd < crate::classes::GltfDocumentExtension > > >, first_priority: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::GltfDocumentExtension > > >, bool,);
            let args = (extension, first_priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "register_gltf_document_extension", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`register_gltf_document_extension_ex`][Self::register_gltf_document_extension_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Registers the given [`GLTFDocumentExtension`][crate::classes::GltfDocumentExtension] instance with GLTFDocument. If `first_priority` is `true`, this extension will be run first. Otherwise, it will be run last.\n\n**Note:** Like GLTFDocument itself, all GLTFDocumentExtension classes must be stateless in order to function properly. If you need to store data, use the `set_additional_data` and `get_additional_data` methods in [`GLTFState`][crate::classes::GltfState] or [`GLTFNode`][crate::classes::GltfNode]."]
        #[inline]
        pub fn register_gltf_document_extension(extension: impl AsArg < Option < Gd < crate::classes::GltfDocumentExtension >> >,) {
            Self::register_gltf_document_extension_ex(extension,) . done()
        }
        #[doc = "Registers the given [`GLTFDocumentExtension`][crate::classes::GltfDocumentExtension] instance with GLTFDocument. If `first_priority` is `true`, this extension will be run first. Otherwise, it will be run last.\n\n**Note:** Like GLTFDocument itself, all GLTFDocumentExtension classes must be stateless in order to function properly. If you need to store data, use the `set_additional_data` and `get_additional_data` methods in [`GLTFState`][crate::classes::GltfState] or [`GLTFNode`][crate::classes::GltfNode]."]
        #[inline]
        pub fn register_gltf_document_extension_ex < 'ex > (extension: impl AsArg < Option < Gd < crate::classes::GltfDocumentExtension >> > + 'ex,) -> ExRegisterGltfDocumentExtension < 'ex > {
            ExRegisterGltfDocumentExtension::new(extension,)
        }
        #[doc = "Unregisters the given [`GLTFDocumentExtension`][crate::classes::GltfDocumentExtension] instance."]
        pub fn unregister_gltf_document_extension(extension: impl AsArg < Option < Gd < crate::classes::GltfDocumentExtension >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::GltfDocumentExtension > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "unregister_gltf_document_extension", None, args,)
            }
        }
        #[doc = "Returns a list of all support glTF extensions, including extensions supported directly by the engine, and extensions supported by user plugins registering [`GLTFDocumentExtension`][crate::classes::GltfDocumentExtension] classes.\n\n**Note:** If this method is run before a GLTFDocumentExtension is registered, its extensions won't be included in the list. Be sure to only run this method after all extensions are registered. If you run this when the engine starts, consider waiting a frame before calling this method to ensure all extensions are registered."]
        pub fn get_supported_gltf_extensions() -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9894usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfDocument", "get_supported_gltf_extensions", None, args,)
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
    impl crate::obj::GodotClass for GltfDocument {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GLTFDocument"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfDocument {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfDocument {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfDocument {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfDocument {
        
    }
    impl crate::obj::cap::GodotDefault for GltfDocument {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfDocument {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfDocument {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfDocument`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GltfDocument__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfDocument > for $Class {
                
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
#[doc = "Default-param extender for [`GltfDocument::append_from_file_ex`][super::GltfDocument::append_from_file_ex]."]
#[must_use]
pub struct ExAppendFromFile < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::GltfDocument, path: CowArg < 'ex, GString >, state: CowArg < 'ex, Option < Gd < crate::classes::GltfState > > >, flags: u32, base_path: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAppendFromFile < 'ex > {
    fn new(surround_object: &'ex mut re_export::GltfDocument, path: impl AsArg < GString > + 'ex, state: impl AsArg < Option < Gd < crate::classes::GltfState >> > + 'ex,) -> Self {
        let flags = 0u32;
        let base_path = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), state: state.into_arg(), flags: flags, base_path: CowArg::Owned(base_path),
        }
    }
    #[inline]
    pub fn flags(self, flags: u32) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn base_path(self, base_path: impl AsArg < GString > + 'ex) -> Self {
        Self {
            base_path: base_path.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, state, flags, base_path,
        }
        = self;
        re_export::GltfDocument::append_from_file_full(surround_object, path, state, flags, base_path,)
    }
}
#[doc = "Default-param extender for [`GltfDocument::append_from_buffer_ex`][super::GltfDocument::append_from_buffer_ex]."]
#[must_use]
pub struct ExAppendFromBuffer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::GltfDocument, bytes: CowArg < 'ex, PackedByteArray >, base_path: CowArg < 'ex, GString >, state: CowArg < 'ex, Option < Gd < crate::classes::GltfState > > >, flags: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAppendFromBuffer < 'ex > {
    fn new(surround_object: &'ex mut re_export::GltfDocument, bytes: &'ex PackedByteArray, base_path: impl AsArg < GString > + 'ex, state: impl AsArg < Option < Gd < crate::classes::GltfState >> > + 'ex,) -> Self {
        let flags = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bytes: CowArg::Borrowed(bytes), base_path: base_path.into_arg(), state: state.into_arg(), flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: u32) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, bytes, base_path, state, flags,
        }
        = self;
        re_export::GltfDocument::append_from_buffer_full(surround_object, bytes.cow_as_arg(), base_path, state, flags,)
    }
}
#[doc = "Default-param extender for [`GltfDocument::append_from_scene_ex`][super::GltfDocument::append_from_scene_ex]."]
#[must_use]
pub struct ExAppendFromScene < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::GltfDocument, node: CowArg < 'ex, Option < Gd < crate::classes::Node > > >, state: CowArg < 'ex, Option < Gd < crate::classes::GltfState > > >, flags: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAppendFromScene < 'ex > {
    fn new(surround_object: &'ex mut re_export::GltfDocument, node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'ex, state: impl AsArg < Option < Gd < crate::classes::GltfState >> > + 'ex,) -> Self {
        let flags = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.into_arg(), state: state.into_arg(), flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: u32) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, node, state, flags,
        }
        = self;
        re_export::GltfDocument::append_from_scene_full(surround_object, node, state, flags,)
    }
}
#[doc = "Default-param extender for [`GltfDocument::generate_scene_ex`][super::GltfDocument::generate_scene_ex]."]
#[must_use]
pub struct ExGenerateScene < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::GltfDocument, state: CowArg < 'ex, Option < Gd < crate::classes::GltfState > > >, bake_fps: f32, trimming: bool, remove_immutable_tracks: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGenerateScene < 'ex > {
    fn new(surround_object: &'ex mut re_export::GltfDocument, state: impl AsArg < Option < Gd < crate::classes::GltfState >> > + 'ex,) -> Self {
        let bake_fps = 30f32;
        let trimming = false;
        let remove_immutable_tracks = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, state: state.into_arg(), bake_fps: bake_fps, trimming: trimming, remove_immutable_tracks: remove_immutable_tracks,
        }
    }
    #[inline]
    pub fn bake_fps(self, bake_fps: f32) -> Self {
        Self {
            bake_fps: bake_fps, .. self
        }
    }
    #[inline]
    pub fn trimming(self, trimming: bool) -> Self {
        Self {
            trimming: trimming, .. self
        }
    }
    #[inline]
    pub fn remove_immutable_tracks(self, remove_immutable_tracks: bool) -> Self {
        Self {
            remove_immutable_tracks: remove_immutable_tracks, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, state, bake_fps, trimming, remove_immutable_tracks,
        }
        = self;
        re_export::GltfDocument::generate_scene_full(surround_object, state, bake_fps, trimming, remove_immutable_tracks,)
    }
}
#[doc = "Default-param extender for [`GltfDocument::register_gltf_document_extension_ex`][super::GltfDocument::register_gltf_document_extension_ex]."]
#[must_use]
pub struct ExRegisterGltfDocumentExtension < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, extension: CowArg < 'ex, Option < Gd < crate::classes::GltfDocumentExtension > > >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExRegisterGltfDocumentExtension < 'ex > {
    fn new(extension: impl AsArg < Option < Gd < crate::classes::GltfDocumentExtension >> > + 'ex,) -> Self {
        let first_priority = false;
        Self {
            _phantom: std::marker::PhantomData, extension: extension.into_arg(), first_priority: first_priority,
        }
    }
    #[inline]
    pub fn first_priority(self, first_priority: bool) -> Self {
        Self {
            first_priority: first_priority, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, extension, first_priority,
        }
        = self;
        re_export::GltfDocument::register_gltf_document_extension_full(extension, first_priority,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RootNodeMode {
    ord: i32
}
impl RootNodeMode {
    #[doc(alias = "ROOT_NODE_MODE_SINGLE_ROOT")]
    #[doc = "Godot enumerator name: `ROOT_NODE_MODE_SINGLE_ROOT`"]
    pub const SINGLE_ROOT: RootNodeMode = RootNodeMode {
        ord: 0i32
    };
    #[doc(alias = "ROOT_NODE_MODE_KEEP_ROOT")]
    #[doc = "Godot enumerator name: `ROOT_NODE_MODE_KEEP_ROOT`"]
    pub const KEEP_ROOT: RootNodeMode = RootNodeMode {
        ord: 1i32
    };
    #[doc(alias = "ROOT_NODE_MODE_MULTI_ROOT")]
    #[doc = "Godot enumerator name: `ROOT_NODE_MODE_MULTI_ROOT`"]
    pub const MULTI_ROOT: RootNodeMode = RootNodeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for RootNodeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RootNodeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RootNodeMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::SINGLE_ROOT => "SINGLE_ROOT", Self::KEEP_ROOT => "KEEP_ROOT", Self::MULTI_ROOT => "MULTI_ROOT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RootNodeMode::SINGLE_ROOT, RootNodeMode::KEEP_ROOT, RootNodeMode::MULTI_ROOT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RootNodeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SINGLE_ROOT", "ROOT_NODE_MODE_SINGLE_ROOT", RootNodeMode::SINGLE_ROOT), crate::meta::inspect::EnumConstant::new("KEEP_ROOT", "ROOT_NODE_MODE_KEEP_ROOT", RootNodeMode::KEEP_ROOT), crate::meta::inspect::EnumConstant::new("MULTI_ROOT", "ROOT_NODE_MODE_MULTI_ROOT", RootNodeMode::MULTI_ROOT)]
        }
    }
}
impl crate::meta::GodotConvert for RootNodeMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Root Node Mode Single Root", 0i64), EnumeratorShape::new_int("Root Node Mode Keep Root", 1i64), EnumeratorShape::new_int("Root Node Mode Multi Root", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("GLTFDocument.RootNodeMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for RootNodeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RootNodeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for RootNodeMode {
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
impl crate::registry::property::Export for RootNodeMode {
    
}
impl crate::meta::Element for RootNodeMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibilityMode {
    ord: i32
}
impl VisibilityMode {
    #[doc(alias = "VISIBILITY_MODE_INCLUDE_REQUIRED")]
    #[doc = "Godot enumerator name: `VISIBILITY_MODE_INCLUDE_REQUIRED`"]
    pub const INCLUDE_REQUIRED: VisibilityMode = VisibilityMode {
        ord: 0i32
    };
    #[doc(alias = "VISIBILITY_MODE_INCLUDE_OPTIONAL")]
    #[doc = "Godot enumerator name: `VISIBILITY_MODE_INCLUDE_OPTIONAL`"]
    pub const INCLUDE_OPTIONAL: VisibilityMode = VisibilityMode {
        ord: 1i32
    };
    #[doc(alias = "VISIBILITY_MODE_EXCLUDE")]
    #[doc = "Godot enumerator name: `VISIBILITY_MODE_EXCLUDE`"]
    pub const EXCLUDE: VisibilityMode = VisibilityMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for VisibilityMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibilityMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibilityMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::INCLUDE_REQUIRED => "INCLUDE_REQUIRED", Self::INCLUDE_OPTIONAL => "INCLUDE_OPTIONAL", Self::EXCLUDE => "EXCLUDE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VisibilityMode::INCLUDE_REQUIRED, VisibilityMode::INCLUDE_OPTIONAL, VisibilityMode::EXCLUDE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VisibilityMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INCLUDE_REQUIRED", "VISIBILITY_MODE_INCLUDE_REQUIRED", VisibilityMode::INCLUDE_REQUIRED), crate::meta::inspect::EnumConstant::new("INCLUDE_OPTIONAL", "VISIBILITY_MODE_INCLUDE_OPTIONAL", VisibilityMode::INCLUDE_OPTIONAL), crate::meta::inspect::EnumConstant::new("EXCLUDE", "VISIBILITY_MODE_EXCLUDE", VisibilityMode::EXCLUDE)]
        }
    }
}
impl crate::meta::GodotConvert for VisibilityMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Visibility Mode Include Required", 0i64), EnumeratorShape::new_int("Visibility Mode Include Optional", 1i64), EnumeratorShape::new_int("Visibility Mode Exclude", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("GLTFDocument.VisibilityMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for VisibilityMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibilityMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for VisibilityMode {
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
impl crate::registry::property::Export for VisibilityMode {
    
}
impl crate::meta::Element for VisibilityMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GltfDocument;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for GltfDocument {
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
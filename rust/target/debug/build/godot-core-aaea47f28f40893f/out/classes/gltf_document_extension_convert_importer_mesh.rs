#![doc = "Sidecar module for class [`GltfDocumentExtensionConvertImporterMesh`][crate::classes::GltfDocumentExtensionConvertImporterMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFDocumentExtensionConvertImporterMesh` enums](https://docs.godotengine.org/en/stable/classes/class_gltfdocumentextensionconvertimportermesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFDocumentExtensionConvertImporterMesh`.\n\nInherits [`GltfDocumentExtension`][crate::classes::GltfDocumentExtension].\n\nRelated symbols:\n\n* [`IGltfDocumentExtensionConvertImporterMesh`][crate::classes::IGltfDocumentExtensionConvertImporterMesh]: virtual methods\n\n\nSee also [Godot docs for `GLTFDocumentExtensionConvertImporterMesh`](https://docs.godotengine.org/en/stable/classes/class_gltfdocumentextensionconvertimportermesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfDocumentExtensionConvertImporterMesh::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfDocumentExtensionConvertImporterMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GltfDocumentExtensionConvertImporterMesh`][crate::classes::GltfDocumentExtensionConvertImporterMesh].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IGltfDocumentExtension`][crate::classes::IGltfDocumentExtension] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GLTFDocumentExtensionConvertImporterMesh` methods](https://docs.godotengine.org/en/stable/classes/class_gltfdocumentextensionconvertimportermesh.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfDocumentExtensionConvertImporterMesh: crate::obj::GodotClass < Base = GltfDocumentExtensionConvertImporterMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Part of the import process. This method is run first, before all other parts of the import process.\n\nThe return value is used to determine if this `GLTFDocumentExtension` instance should be used for importing a given glTF file. If [`Error::OK`][`crate::global::Error::OK`], the import will use this `GLTFDocumentExtension` instance. If not overridden, [`Error::OK`][`crate::global::Error::OK`] is returned."]
        fn import_preflight(&mut self, state: Option < Gd < crate::classes::GltfState > >, extensions: PackedStringArray,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the import process. This method is run after [`import_preflight`][`crate::classes::IGltfDocumentExtension::import_preflight`] and before [`parse_node_extensions`][`crate::classes::IGltfDocumentExtension::parse_node_extensions`].\n\nReturns an array of the glTF extensions supported by this GLTFDocumentExtension class. This is used to validate if a glTF file with required extensions can be loaded."]
        fn get_supported_extensions(&mut self,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Part of the import process. This method is run after [`get_supported_extensions`][`crate::classes::IGltfDocumentExtension::get_supported_extensions`] and before [`import_post_parse`][`crate::classes::IGltfDocumentExtension::import_post_parse`].\n\nRuns when parsing the node extensions of a GLTFNode. This method can be used to process the extension JSON data into a format that can be used by [`generate_scene_node`][`crate::classes::IGltfDocumentExtension::generate_scene_node`]. The return value should be a member of the \\[enum Error] enum."]
        fn parse_node_extensions(&mut self, state: Option < Gd < crate::classes::GltfState > >, gltf_node: Option < Gd < crate::classes::GltfNode > >, extensions: VarDictionary,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the import process. This method is run after [`parse_node_extensions`][`crate::classes::IGltfDocumentExtension::parse_node_extensions`] and before [`parse_texture_json`][`crate::classes::IGltfDocumentExtension::parse_texture_json`].\n\nRuns when parsing image data from a glTF file. The data could be sourced from a separate file, a URI, or a buffer, and then is passed as a byte array."]
        fn parse_image_data(&mut self, state: Option < Gd < crate::classes::GltfState > >, image_data: PackedByteArray, mime_type: GString, ret_image: Option < Gd < crate::classes::Image > >,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Returns the file extension to use for saving image data into, for example, `\".png\"`. If defined, when this extension is used to handle images, and the images are saved to a separate file, the image bytes will be copied to a file with this extension. If this is set, there should be a [`ResourceImporter`][crate::classes::ResourceImporter] class able to import the file. If not defined or empty, Godot will save the image into a PNG file."]
        fn get_image_file_extension(&mut self,) -> GString {
            unimplemented !()
        }
        #[doc = "Part of the import process. This method is run after [`parse_image_data`][`crate::classes::IGltfDocumentExtension::parse_image_data`] and before [`generate_scene_node`][`crate::classes::IGltfDocumentExtension::generate_scene_node`].\n\nRuns when parsing the texture JSON from the glTF textures array. This can be used to set the source image index to use as the texture."]
        fn parse_texture_json(&mut self, state: Option < Gd < crate::classes::GltfState > >, texture_json: VarDictionary, ret_gltf_texture: Option < Gd < crate::classes::GltfTexture > >,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the import process. Allows GLTFDocumentExtension classes to provide mappings for JSON pointers to glTF properties, as defined by the glTF object model, to properties of nodes in the Godot scene tree.\n\nReturns a [`GLTFObjectModelProperty`][crate::classes::GltfObjectModelProperty] instance that defines how the property should be mapped. If your extension can't handle the property, return `null` or an instance without any NodePaths (see [`has_node_paths`][`crate::classes::GltfObjectModelProperty::has_node_paths`]). You should use [`set_types`][`crate::classes::GltfObjectModelProperty::set_types`] to set the types, and [`append_path_to_property`][`crate::classes::GltfObjectModelProperty::append_path_to_property`] function is useful for most simple cases.\n\nIn many cases, `partial_paths` will contain the start of a path, allowing the extension to complete the path. For example, for `/nodes/3/extensions/MY_ext/prop`, Godot will pass you a NodePath that leads to node 3, so the GLTFDocumentExtension class only needs to resolve the last `MY_ext/prop` part of the path. In this example, the extension should check `split.size() > 4 and split[0] == \"nodes\" and split[2] == \"extensions\" and split[3] == \"MY_ext\"` at the start of the function to check if this JSON pointer applies to it, then it can use `partial_paths` and handle `split[4]`."]
        fn import_object_model_property(&mut self, state: Option < Gd < crate::classes::GltfState > >, split_json_pointer: PackedStringArray, partial_paths: Array < NodePath >,) -> Option < Gd < crate::classes::GltfObjectModelProperty > > {
            unimplemented !()
        }
        #[doc = "Part of the import process. This method is run after [`parse_node_extensions`][`crate::classes::IGltfDocumentExtension::parse_node_extensions`] and before [`import_pre_generate`][`crate::classes::IGltfDocumentExtension::import_pre_generate`].\n\nThis method can be used to modify any of the data imported so far after parsing each node, but before generating the scene or any of its nodes."]
        fn import_post_parse(&mut self, state: Option < Gd < crate::classes::GltfState > >,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the import process. This method is run after [`import_post_parse`][`crate::classes::IGltfDocumentExtension::import_post_parse`] and before [`generate_scene_node`][`crate::classes::IGltfDocumentExtension::generate_scene_node`].\n\nThis method can be used to modify or read from any of the processed data structures, before generating the nodes and then running the final per-node import step."]
        fn import_pre_generate(&mut self, state: Option < Gd < crate::classes::GltfState > >,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the import process. This method is run after [`import_pre_generate`][`crate::classes::IGltfDocumentExtension::import_pre_generate`] and before [`import_node`][`crate::classes::IGltfDocumentExtension::import_node`].\n\nRuns when generating a Godot scene node from a GLTFNode. The returned node will be added to the scene tree. Multiple nodes can be generated in this step if they are added as a child of the returned node.\n\n**Note:** The `scene_parent` parameter may be `null` if this is the single root node."]
        fn generate_scene_node(&mut self, state: Option < Gd < crate::classes::GltfState > >, gltf_node: Option < Gd < crate::classes::GltfNode > >, scene_parent: Option < Gd < crate::classes::Node > >,) -> Option < Gd < crate::classes::Node3D > > {
            unimplemented !()
        }
        #[doc = "Part of the import process. This method is run after [`generate_scene_node`][`crate::classes::IGltfDocumentExtension::generate_scene_node`] and before [`import_post`][`crate::classes::IGltfDocumentExtension::import_post`].\n\nThis method can be used to make modifications to each of the generated Godot scene nodes."]
        fn import_node(&mut self, state: Option < Gd < crate::classes::GltfState > >, gltf_node: Option < Gd < crate::classes::GltfNode > >, json: VarDictionary, node: Option < Gd < crate::classes::Node > >,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the import process. This method is run last, after all other parts of the import process.\n\nThis method can be used to modify the final Godot scene generated by the import process."]
        fn import_post(&mut self, state: Option < Gd < crate::classes::GltfState > >, root: Option < Gd < crate::classes::Node > >,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the export process. This method is run first, before all other parts of the export process.\n\nThe return value is used to determine if this `GLTFDocumentExtension` instance should be used for exporting a given glTF file. If [`Error::OK`][`crate::global::Error::OK`], the export will use this `GLTFDocumentExtension` instance. If not overridden, [`Error::OK`][`crate::global::Error::OK`] is returned."]
        fn export_preflight(&mut self, state: Option < Gd < crate::classes::GltfState > >, root: Option < Gd < crate::classes::Node > >,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the export process. This method is run after [`export_preflight`][`crate::classes::IGltfDocumentExtension::export_preflight`] and before [`export_post_convert`][`crate::classes::IGltfDocumentExtension::export_post_convert`].\n\nRuns when converting the data from a Godot scene node. This method can be used to process the Godot scene node data into a format that can be used by [`export_node`][`crate::classes::IGltfDocumentExtension::export_node`]."]
        fn convert_scene_node(&mut self, state: Option < Gd < crate::classes::GltfState > >, gltf_node: Option < Gd < crate::classes::GltfNode > >, scene_node: Option < Gd < crate::classes::Node > >,) {
            unimplemented !()
        }
        #[doc = "Part of the export process. This method is run after [`convert_scene_node`][`crate::classes::IGltfDocumentExtension::convert_scene_node`] and before [`export_preserialize`][`crate::classes::IGltfDocumentExtension::export_preserialize`].\n\nThis method can be used to modify the converted node data structures before serialization with any additional data from the scene tree."]
        fn export_post_convert(&mut self, state: Option < Gd < crate::classes::GltfState > >, root: Option < Gd < crate::classes::Node > >,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the export process. This method is run after [`export_post_convert`][`crate::classes::IGltfDocumentExtension::export_post_convert`] and before [`get_saveable_image_formats`][`crate::classes::IGltfDocumentExtension::get_saveable_image_formats`].\n\nThis method can be used to alter the state before performing serialization. It runs every time when generating a buffer with [`generate_buffer`][`crate::classes::GltfDocument::generate_buffer`] or writing to the file system with [`write_to_filesystem`][`crate::classes::GltfDocument::write_to_filesystem`]."]
        fn export_preserialize(&mut self, state: Option < Gd < crate::classes::GltfState > >,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the export process. Allows GLTFDocumentExtension classes to provide mappings for properties of nodes in the Godot scene tree, to JSON pointers to glTF properties, as defined by the glTF object model.\n\nReturns a [`GLTFObjectModelProperty`][crate::classes::GltfObjectModelProperty] instance that defines how the property should be mapped. If your extension can't handle the property, return `null` or an instance without any JSON pointers (see [`has_json_pointers`][`crate::classes::GltfObjectModelProperty::has_json_pointers`]). You should use [`set_types`][`crate::classes::GltfObjectModelProperty::set_types`] to set the types, and set the JSON pointer(s) using the \\[member GLTFObjectModelProperty.json_pointers] property.\n\nThe parameters provide context for the property, including the NodePath, the Godot node, the GLTF node index, and the target object. The `target_object` will be equal to `godot_node` if no sub-object can be found, otherwise it will point to a sub-object. For example, if the path is `^\"A/B/C/MeshInstance3D:mesh:surface_0/material:emission_intensity\"`, it will get the node, then the mesh, and then the material, so `target_object` will be the [`Material`][crate::classes::Material] resource, and `target_depth` will be 2 because 2 levels were traversed to get to the target."]
        fn export_object_model_property(&mut self, state: Option < Gd < crate::classes::GltfState > >, node_path: NodePath, godot_node: Option < Gd < crate::classes::Node > >, gltf_node_index: i32, target_object: Option < Gd < crate::classes::Object > >, target_depth: i32,) -> Option < Gd < crate::classes::GltfObjectModelProperty > > {
            unimplemented !()
        }
        #[doc = "Part of the export process. This method is run after [`convert_scene_node`][`crate::classes::IGltfDocumentExtension::convert_scene_node`] and before [`export_node`][`crate::classes::IGltfDocumentExtension::export_node`].\n\nReturns an array of the image formats that can be saved/exported by this extension. This extension will only be selected as the image exporter if the [`GLTFDocument`][crate::classes::GltfDocument]'s \\[member GLTFDocument.image_format] is in this array. If this `GLTFDocumentExtension` is selected as the image exporter, one of the [`save_image_at_path`][`crate::classes::IGltfDocumentExtension::save_image_at_path`] or [`serialize_image_to_bytes`][`crate::classes::IGltfDocumentExtension::serialize_image_to_bytes`] methods will run next, otherwise [`export_node`][`crate::classes::IGltfDocumentExtension::export_node`] will run next. If the format name contains `\"Lossy\"`, the lossy quality slider will be displayed."]
        fn get_saveable_image_formats(&mut self,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Part of the export process. This method is run after [`get_saveable_image_formats`][`crate::classes::IGltfDocumentExtension::get_saveable_image_formats`] and before [`serialize_texture_json`][`crate::classes::IGltfDocumentExtension::serialize_texture_json`].\n\nThis method is run when embedding images in the glTF file. When images are saved separately, [`save_image_at_path`][`crate::classes::IGltfDocumentExtension::save_image_at_path`] runs instead. Note that these methods only run when this `GLTFDocumentExtension` is selected as the image exporter.\n\nThis method must set the image MIME type in the `image_dict` with the `\"mimeType\"` key. For example, for a PNG image, it would be set to `\"image/png\"`. The return value must be a [`PackedByteArray`][crate::builtin::PackedByteArray] containing the image data."]
        fn serialize_image_to_bytes(&mut self, state: Option < Gd < crate::classes::GltfState > >, image: Option < Gd < crate::classes::Image > >, image_dict: VarDictionary, image_format: GString, lossy_quality: f32,) -> PackedByteArray {
            unimplemented !()
        }
        #[doc = "Part of the export process. This method is run after [`get_saveable_image_formats`][`crate::classes::IGltfDocumentExtension::get_saveable_image_formats`] and before [`serialize_texture_json`][`crate::classes::IGltfDocumentExtension::serialize_texture_json`].\n\nThis method is run when saving images separately from the glTF file. When images are embedded, [`serialize_image_to_bytes`][`crate::classes::IGltfDocumentExtension::serialize_image_to_bytes`] runs instead. Note that these methods only run when this `GLTFDocumentExtension` is selected as the image exporter."]
        fn save_image_at_path(&mut self, state: Option < Gd < crate::classes::GltfState > >, image: Option < Gd < crate::classes::Image > >, file_path: GString, image_format: GString, lossy_quality: f32,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the export process. This method is run after [`save_image_at_path`][`crate::classes::IGltfDocumentExtension::save_image_at_path`] or [`serialize_image_to_bytes`][`crate::classes::IGltfDocumentExtension::serialize_image_to_bytes`], and before [`export_node`][`crate::classes::IGltfDocumentExtension::export_node`]. Note that this method only runs when this `GLTFDocumentExtension` is selected as the image exporter.\n\nThis method can be used to set up the extensions for the texture JSON by editing `texture_json`. The extension must also be added as used extension with [`add_used_extension`][`crate::classes::GltfState::add_used_extension`], be sure to set `required` to `true` if you are not providing a fallback."]
        fn serialize_texture_json(&mut self, state: Option < Gd < crate::classes::GltfState > >, texture_json: VarDictionary, gltf_texture: Option < Gd < crate::classes::GltfTexture > >, image_format: GString,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the export process. This method is run after [`get_saveable_image_formats`][`crate::classes::IGltfDocumentExtension::get_saveable_image_formats`] and before [`export_post`][`crate::classes::IGltfDocumentExtension::export_post`]. If this `GLTFDocumentExtension` is used for exporting images, this runs after [`serialize_texture_json`][`crate::classes::IGltfDocumentExtension::serialize_texture_json`].\n\nThis method can be used to modify the final JSON of each node. Data should be primarily stored in `gltf_node` prior to serializing the JSON, but the original Godot [`Node`][crate::classes::Node] is also provided if available. `node` may be `null` if not available, such as when exporting glTF data not generated from a Godot scene."]
        fn export_node(&mut self, state: Option < Gd < crate::classes::GltfState > >, gltf_node: Option < Gd < crate::classes::GltfNode > >, json: VarDictionary, node: Option < Gd < crate::classes::Node > >,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Part of the export process. This method is run last, after all other parts of the export process.\n\nThis method can be used to modify the final JSON of the generated glTF file."]
        fn export_post(&mut self, state: Option < Gd < crate::classes::GltfState > >,) -> crate::global::Error {
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
    impl GltfDocumentExtensionConvertImporterMesh {
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
    impl crate::obj::GodotClass for GltfDocumentExtensionConvertImporterMesh {
        type Base = crate::classes::GltfDocumentExtension;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GLTFDocumentExtensionConvertImporterMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfDocumentExtensionConvertImporterMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GltfDocumentExtension > for GltfDocumentExtensionConvertImporterMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfDocumentExtensionConvertImporterMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfDocumentExtensionConvertImporterMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfDocumentExtensionConvertImporterMesh {
        
    }
    impl crate::obj::cap::GodotDefault for GltfDocumentExtensionConvertImporterMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfDocumentExtensionConvertImporterMesh {
        type Target = crate::classes::GltfDocumentExtension;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfDocumentExtensionConvertImporterMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfDocumentExtensionConvertImporterMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GltfDocumentExtensionConvertImporterMesh__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfDocumentExtensionConvertImporterMesh > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfDocumentExtension > for $Class {
                
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
    use super::re_export::GltfDocumentExtensionConvertImporterMesh;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for GltfDocumentExtensionConvertImporterMesh {
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
#![doc = "Sidecar module for class [`EditorExportPlugin`][crate::classes::EditorExportPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorExportPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorexportplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorExportPlugin`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`IEditorExportPlugin`][crate::classes::IEditorExportPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorExportPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorexportplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorExportPlugin::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`EditorExportPlugin`s are automatically invoked whenever the user exports the project. Their most common use is to determine what files are being included in the exported project. For each plugin, [`export_begin`][`crate::classes::IEditorExportPlugin::export_begin`] is called at the beginning of the export process and then [`export_file`][`crate::classes::IEditorExportPlugin::export_file`] is called for each exported file.\n\nTo use `EditorExportPlugin`, register it using the [`add_export_plugin`][`crate::classes::EditorPlugin::add_export_plugin`] method first."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorExportPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorExportPlugin`][crate::classes::EditorExportPlugin].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `EditorExportPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorexportplugin.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorExportPlugin: crate::obj::GodotClass < Base = EditorExportPlugin > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Virtual method to be overridden by the user. Called for each exported file before [`customize_resource`][`crate::classes::IEditorExportPlugin::customize_resource`] and [`customize_scene`][`crate::classes::IEditorExportPlugin::customize_scene`]. The arguments can be used to identify the file. `path` is the path of the file, `type` is the [`Resource`][crate::classes::Resource] represented by the file (e.g. [`PackedScene`][crate::classes::PackedScene]), and `features` is the list of features for the export.\n\nCalling [`skip`][`crate::classes::EditorExportPlugin::skip`] inside this callback will make the file not included in the export."]
        fn export_file(&mut self, path: GString, type_: GString, features: PackedStringArray,) {
            unimplemented !()
        }
        #[doc = "Virtual method to be overridden by the user. It is called when the export starts and provides all information about the export. `features` is the list of features for the export, `is_debug` is `true` for debug builds, `path` is the target path for the exported project. `flags` is only used when running a runnable profile, e.g. when using native run on Android."]
        fn export_begin(&mut self, features: PackedStringArray, is_debug: bool, path: GString, flags: u32,) {
            unimplemented !()
        }
        #[doc = "Virtual method to be overridden by the user. Called when the export is finished."]
        fn export_end(&mut self,) {
            unimplemented !()
        }
        #[doc = "Return `true` if this plugin will customize resources based on the platform and features used.\n\nWhen enabled, [`get_customization_configuration_hash`][`crate::classes::IEditorExportPlugin::get_customization_configuration_hash`] and [`customize_resource`][`crate::classes::IEditorExportPlugin::customize_resource`] will be called and must be implemented."]
        fn begin_customize_resources(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, features: PackedStringArray,) -> bool {
            unimplemented !()
        }
        #[doc = "Customize a resource. If changes are made to it, return the same or a new resource. Otherwise, return `null`. When a new resource is returned, `resource` will be replaced by a copy of the new resource.\n\nThe `path` argument is only used when customizing an actual file, otherwise this means that this resource is part of another one and it will be empty.\n\nImplementing this method is required if [`begin_customize_resources`][`crate::classes::IEditorExportPlugin::begin_customize_resources`] returns `true`.\n\n**Note:** When customizing any of the following types and returning another resource, the other resource should not be skipped using [`skip`][`crate::classes::EditorExportPlugin::skip`] in [`export_file`][`crate::classes::IEditorExportPlugin::export_file`]:\n\n- [`AtlasTexture`][crate::classes::AtlasTexture]\n\n- [`CompressedCubemap`][crate::classes::CompressedCubemap]\n\n- [`CompressedCubemapArray`][crate::classes::CompressedCubemapArray]\n\n- [`CompressedTexture2D`][crate::classes::CompressedTexture2D]\n\n- [`CompressedTexture2DArray`][crate::classes::CompressedTexture2DArray]\n\n- [`CompressedTexture3D`][crate::classes::CompressedTexture3D]"]
        fn customize_resource(&mut self, resource: Gd < crate::classes::Resource >, path: GString,) -> Option < Gd < crate::classes::Resource > >;
        #[doc = "Return `true` if this plugin will customize scenes based on the platform and features used.\n\nWhen enabled, [`get_customization_configuration_hash`][`crate::classes::IEditorExportPlugin::get_customization_configuration_hash`] and [`customize_scene`][`crate::classes::IEditorExportPlugin::customize_scene`] will be called and must be implemented.\n\n**Note:** [`customize_scene`][`crate::classes::IEditorExportPlugin::customize_scene`] will only be called for scenes that have been modified since the last export."]
        fn begin_customize_scenes(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, features: PackedStringArray,) -> bool {
            unimplemented !()
        }
        #[doc = "Customize a scene. If changes are made to it, return the same or a new scene. Otherwise, return `null`. If a new scene is returned, it is up to you to dispose of the old one.\n\nImplementing this method is required if [`begin_customize_scenes`][`crate::classes::IEditorExportPlugin::begin_customize_scenes`] returns `true`."]
        fn customize_scene(&mut self, scene: Gd < crate::classes::Node >, path: GString,) -> Option < Gd < crate::classes::Node > >;
        #[doc = "Return a hash based on the configuration passed (for both scenes and resources). This helps keep separate caches for separate export configurations.\n\nImplementing this method is required if [`begin_customize_resources`][`crate::classes::IEditorExportPlugin::begin_customize_resources`] returns `true`."]
        fn get_customization_configuration_hash(&self,) -> u64;
        #[doc = "This is called when the customization process for scenes ends."]
        fn end_customize_scenes(&mut self,) {
            unimplemented !()
        }
        #[doc = "This is called when the customization process for resources ends."]
        fn end_customize_resources(&mut self,) {
            unimplemented !()
        }
        #[doc = "Return a list of export options that can be configured for this export plugin.\n\nEach element in the return value is a [`Dictionary`][crate::builtin::Dictionary] with the following keys:\n\n- `option`: A dictionary with the structure documented by [`get_property_list`][`crate::classes::Object::get_property_list`], but all keys are optional.\n\n- `default_value`: The default value for this option.\n\n- `update_visibility`: An optional boolean value. If set to `true`, the preset will emit `Object.property_list_changed` when the option is changed."]
        fn get_export_options(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >,) -> Array < AnyDictionary > {
            unimplemented !()
        }
        #[doc = "Return a [`Dictionary`][crate::builtin::Dictionary] of override values for export options, that will be used instead of user-provided values. Overridden options will be hidden from the user interface.\n\n```gdscript\nclass MyExportPlugin extends EditorExportPlugin:\n\tfunc _get_name() -> String:\n\t\treturn \"MyExportPlugin\"\n\n\tfunc _supports_platform(platform) -> bool:\n\t\tif platform is EditorExportPlatformPC:\n\t\t\t# Run on all desktop platforms including Windows, MacOS and Linux.\n\t\t\treturn true\n\t\treturn false\n\n\tfunc _get_export_options_overrides(platform) -> Dictionary:\n\t\t# Override \"Embed PCK\" to always be enabled.\n\t\treturn {\n\t\t\t\"binary_format/embed_pck\": true,\n\t\t}\n```"]
        fn get_export_options_overrides(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "Return `true` if the result of [`get_export_options`][`crate::classes::IEditorExportPlugin::get_export_options`] has changed and the export options of the preset corresponding to `platform` should be updated."]
        fn should_update_export_options(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >,) -> bool {
            unimplemented !()
        }
        #[doc = "Validates `option` and returns the visibility for the specified `platform`. The default implementation returns `true` for all options."]
        fn get_export_option_visibility(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, option: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Check the requirements for the given `option` and return a non-empty warning string if they are not met.\n\n**Note:** Use [`get_option`][`crate::classes::EditorExportPlugin::get_option`] to check the value of the export options."]
        fn get_export_option_warning(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, option: GString,) -> GString {
            unimplemented !()
        }
        #[doc = "Return a [`PackedStringArray`][crate::builtin::PackedStringArray] of additional features this preset, for the given `platform`, should have."]
        fn get_export_features(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Return the name identifier of this plugin (for future identification by the exporter). The plugins are sorted by name before exporting.\n\nImplementing this method is required."]
        fn get_name(&self,) -> GString;
        #[doc = "Return `true` if the plugin supports the given `platform`."]
        fn supports_platform(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >,) -> bool {
            unimplemented !()
        }
        #[doc = "Virtual method to be overridden by the user. This is called to retrieve the set of Android dependencies provided by this plugin. Each returned Android dependency should have the format of an Android remote binary dependency: `org.godot.example:my-plugin:0.0.0`\n\nFor more information see [Android documentation on dependencies](https://developer.android.com/build/dependencies?agpversion=4.1#dependency-types).\n\n**Note:** Only supported on Android and requires \\[member EditorExportPlatformAndroid.gradle_build/use_gradle_build] to be enabled."]
        fn get_android_dependencies(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Virtual method to be overridden by the user. This is called to retrieve the URLs of Maven repositories for the set of Android dependencies provided by this plugin.\n\nFor more information see [Gradle documentation on dependency management](https://docs.gradle.org/current/userguide/dependency_management.html#sec:maven_repo).\n\n**Note:** Google's Maven repo and the Maven Central repo are already included by default.\n\n**Note:** Only supported on Android and requires \\[member EditorExportPlatformAndroid.gradle_build/use_gradle_build] to be enabled."]
        fn get_android_dependencies_maven_repos(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Virtual method to be overridden by the user. This is called to retrieve the local paths of the Android libraries archive (AAR) files provided by this plugin.\n\n**Note:** Relative paths **must** be relative to Godot's `res://addons/` directory. For example, an AAR file located under `res://addons/hello_world_plugin/HelloWorld.release.aar` can be returned as an absolute path using `res://addons/hello_world_plugin/HelloWorld.release.aar` or a relative path using `hello_world_plugin/HelloWorld.release.aar`.\n\n**Note:** Only supported on Android and requires \\[member EditorExportPlatformAndroid.gradle_build/use_gradle_build] to be enabled."]
        fn get_android_libraries(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Virtual method to be overridden by the user. This is used at export time to update the contents of the `activity` element in the generated Android manifest.\n\n**Note:** Only supported on Android and requires \\[member EditorExportPlatformAndroid.gradle_build/use_gradle_build] to be enabled."]
        fn get_android_manifest_activity_element_contents(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> GString {
            unimplemented !()
        }
        #[doc = "Virtual method to be overridden by the user. This is used at export time to update the contents of the `application` element in the generated Android manifest.\n\n**Note:** Only supported on Android and requires \\[member EditorExportPlatformAndroid.gradle_build/use_gradle_build] to be enabled."]
        fn get_android_manifest_application_element_contents(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> GString {
            unimplemented !()
        }
        #[doc = "Virtual method to be overridden by the user. This is used at export time to update the contents of the `manifest` element in the generated Android manifest.\n\n**Note:** Only supported on Android and requires \\[member EditorExportPlatformAndroid.gradle_build/use_gradle_build] to be enabled."]
        fn get_android_manifest_element_contents(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, debug: bool,) -> GString {
            unimplemented !()
        }
        #[doc = "Provide access to the Android prebuilt manifest and allows the plugin to modify it if needed.\n\nImplementers of this virtual method should take the binary manifest data from `manifest_data`, copy it, modify it, and then return it with the modifications.\n\nIf no modifications are needed, then an empty [`PackedByteArray`][crate::builtin::PackedByteArray] should be returned."]
        fn update_android_prebuilt_manifest(&self, platform: Option < Gd < crate::classes::EditorExportPlatform > >, manifest_data: PackedByteArray,) -> PackedByteArray {
            unimplemented !()
        }
    }
    impl EditorExportPlugin {
        #[doc = "Adds a shared object or a directory containing only shared objects with the given `tags` and destination `path`.\n\n**Note:** In case of macOS exports, those shared objects will be added to `Frameworks` directory of app bundle.\n\nIn case of a directory code-sign will error if you place non code object in directory."]
        pub fn add_shared_object(&mut self, path: impl AsArg < GString >, tags: &PackedStringArray, target: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, CowArg < 'a2, GString >,);
            let args = (path.into_arg(), RefArg::new(tags), target.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(98usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_shared_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a custom file to be exported. `path` is the virtual path that can be used to load the file, `file` is the binary data of the file.\n\nWhen called inside [`export_file`][`crate::classes::IEditorExportPlugin::export_file`] and `remap` is `true`, the current file will not be exported, but instead remapped to this custom file. `remap` is ignored when called in other places.\n\n`file` will not be imported, so consider using [`customize_resource`][`crate::classes::IEditorExportPlugin::customize_resource`] to remap imported resources."]
        pub fn add_file(&mut self, path: impl AsArg < GString >, file: &PackedByteArray, remap: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedByteArray >, bool,);
            let args = (path.into_arg(), RefArg::new(file), remap,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(99usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_file", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a static library from the given `path` to the Apple embedded platform project."]
        pub fn add_apple_embedded_platform_project_static_lib(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_apple_embedded_platform_project_static_lib", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a static library (*.a) or a dynamic library (*.dylib, *.framework) to the Linking Phase to the Apple embedded platform's Xcode project."]
        pub fn add_apple_embedded_platform_framework(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_apple_embedded_platform_framework", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a dynamic library (*.dylib, *.framework) to the Linking Phase in the Apple embedded platform's Xcode project and embeds it into the resulting binary.\n\n**Note:** For static libraries (*.a), this works in the same way as [`add_apple_embedded_platform_framework`][`crate::classes::EditorExportPlugin::add_apple_embedded_platform_framework`].\n\n**Note:** This method should not be used for System libraries as they are already present on the device."]
        pub fn add_apple_embedded_platform_embedded_framework(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_apple_embedded_platform_embedded_framework", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds additional fields to the Apple embedded platform's project Info.plist file."]
        pub fn add_apple_embedded_platform_plist_content(&mut self, plist_content: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (plist_content.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_apple_embedded_platform_plist_content", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds linker flags for the Apple embedded platform export."]
        pub fn add_apple_embedded_platform_linker_flags(&mut self, flags: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (flags.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_apple_embedded_platform_linker_flags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an Apple embedded platform bundle file from the given `path` to the exported project."]
        pub fn add_apple_embedded_platform_bundle_file(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_apple_embedded_platform_bundle_file", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds C++ code to the Apple embedded platform export. The final code is created from the code appended by each active export plugin."]
        pub fn add_apple_embedded_platform_cpp_code(&mut self, code: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (code.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_apple_embedded_platform_cpp_code", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a static library from the given `path` to the iOS project."]
        pub fn add_ios_project_static_lib(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_project_static_lib", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a static library (*.a) or a dynamic library (*.dylib, *.framework) to the Linking Phase to the iOS Xcode project."]
        pub fn add_ios_framework(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_framework", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a dynamic library (*.dylib, *.framework) to Linking Phase in iOS's Xcode project and embeds it into resulting binary.\n\n**Note:** For static libraries (*.a), this works the in same way as [`add_apple_embedded_platform_framework`][`crate::classes::EditorExportPlugin::add_apple_embedded_platform_framework`].\n\n**Note:** This method should not be used for System libraries as they are already present on the device."]
        pub fn add_ios_embedded_framework(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_embedded_framework", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds additional fields to the iOS project Info.plist file."]
        pub fn add_ios_plist_content(&mut self, plist_content: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (plist_content.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_plist_content", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds linker flags for the iOS export."]
        pub fn add_ios_linker_flags(&mut self, flags: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (flags.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_linker_flags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an iOS bundle file from the given `path` to the exported project."]
        pub fn add_ios_bundle_file(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_bundle_file", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds C++ code to the iOS export. The final code is created from the code appended by each active export plugin."]
        pub fn add_ios_cpp_code(&mut self, code: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (code.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_ios_cpp_code", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds file or directory matching `path` to `PlugIns` directory of macOS app bundle.\n\n**Note:** This is useful only for macOS exports."]
        pub fn add_macos_plugin_file(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "add_macos_plugin_file", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To be called inside [`export_file`][`crate::classes::IEditorExportPlugin::export_file`]. Skips the current file, so it's not included in the export."]
        pub fn skip(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "skip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current value of an export option supplied by [`get_export_options`][`crate::classes::IEditorExportPlugin::get_export_options`]."]
        pub fn get_option(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "get_option", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns currently used export preset."]
        pub fn get_export_preset(&self,) -> Option < Gd < crate::classes::EditorExportPreset > > {
            type CallRet = Option < Gd < crate::classes::EditorExportPreset > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "get_export_preset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns currently used export platform."]
        pub fn get_export_platform(&self,) -> Option < Gd < crate::classes::EditorExportPlatform > > {
            type CallRet = Option < Gd < crate::classes::EditorExportPlatform > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlugin", "get_export_platform", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorExportPlugin {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorExportPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorExportPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorExportPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorExportPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorExportPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorExportPlugin {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorExportPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorExportPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorExportPlugin__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorExportPlugin > for $Class {
                
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
    use super::re_export::EditorExportPlugin;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorExportPlugin {
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
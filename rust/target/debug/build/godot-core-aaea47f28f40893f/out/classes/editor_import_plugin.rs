#![doc = "Sidecar module for class [`EditorImportPlugin`][crate::classes::EditorImportPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorImportPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorimportplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorImportPlugin`.\n\nInherits [`ResourceImporter`][crate::classes::ResourceImporter].\n\nRelated symbols:\n\n* [`editor_import_plugin`][crate::classes::editor_import_plugin]: sidecar module with related enum/flag types\n* [`IEditorImportPlugin`][crate::classes::IEditorImportPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorImportPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorimportplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorImportPlugin::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`EditorImportPlugin`s provide a way to extend the editor's resource import functionality. Use them to import resources from custom files or to provide alternatives to the editor's existing importers.\n\nEditorImportPlugins work by associating with specific file extensions and a resource type. See [`get_recognized_extensions`][`crate::classes::IEditorImportPlugin::get_recognized_extensions`] and [`get_resource_type`][`crate::classes::IEditorImportPlugin::get_resource_type`]. They may optionally specify some import presets that affect the import process. EditorImportPlugins are responsible for creating the resources and saving them in the `.godot/imported` directory (see \\[member ProjectSettings.application/config/use_hidden_project_data_directory]).\n\nBelow is an example EditorImportPlugin that imports a [`Mesh`][crate::classes::Mesh] from a file with the extension \".special\" or \".spec\":\n\n\n```gdscript\n@tool\nextends EditorImportPlugin\n\nfunc _get_importer_name():\n\treturn \"my.special.plugin\"\n\nfunc _get_visible_name():\n\treturn \"Special Mesh\"\n\nfunc _get_recognized_extensions():\n\treturn [\"special\", \"spec\"]\n\nfunc _get_save_extension():\n\treturn \"mesh\"\n\nfunc _get_resource_type():\n\treturn \"Mesh\"\n\nfunc _get_preset_count():\n\treturn 1\n\nfunc _get_preset_name(preset_index):\n\treturn \"Default\"\n\nfunc _get_import_options(path, preset_index):\n\treturn [{\"name\": \"my_option\", \"default_value\": false}]\n\nfunc _import(source_file, save_path, options, platform_variants, gen_files):\n\tvar file = FileAccess.open(source_file, FileAccess.READ)\n\tif file == null:\n\t\treturn FAILED\n\tvar mesh = ArrayMesh.new()\n\t# Fill the Mesh with data read in \"file\", left as an exercise to the reader.\n\n\tvar filename = save_path + \".\" + _get_save_extension()\n\treturn ResourceSaver.save(mesh, filename)\n```\n\n\nTo use `EditorImportPlugin`, register it using the [`add_import_plugin`][`crate::classes::EditorPlugin::add_import_plugin`] method first."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorImportPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorImportPlugin`][crate::classes::EditorImportPlugin].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IResourceImporter`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `EditorImportPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorimportplugin.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorImportPlugin: crate::obj::GodotClass < Base = EditorImportPlugin > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Gets the unique name of the importer."]
        fn get_importer_name(&self,) -> GString;
        #[doc = "Gets the name to display in the import window. You should choose this name as a continuation to \"Import as\", e.g. \"Import as Special Mesh\"."]
        fn get_visible_name(&self,) -> GString;
        #[doc = "Gets the number of initial presets defined by the plugin. Use [`get_import_options`][`crate::classes::IEditorImportPlugin::get_import_options`] to get the default options for the preset and [`get_preset_name`][`crate::classes::IEditorImportPlugin::get_preset_name`] to get the name of the preset.\n\nBy default, there are no presets."]
        fn get_preset_count(&self,) -> i32 {
            unimplemented !()
        }
        #[doc = "Gets the name of the options preset at this index."]
        fn get_preset_name(&self, preset_index: i32,) -> GString;
        #[doc = "Gets the list of file extensions to associate with this loader (case-insensitive). e.g. `[\"obj\"]`."]
        fn get_recognized_extensions(&self,) -> PackedStringArray;
        #[doc = "Gets the options and default values for the preset at this index. Returns an Array of Dictionaries with the following keys: `name`, `default_value`, `property_hint` (optional), `hint_string` (optional), `usage` (optional)."]
        fn get_import_options(&self, path: GString, preset_index: i32,) -> Array < AnyDictionary >;
        #[doc = "Gets the extension used to save this resource in the `.godot/imported` directory (see \\[member ProjectSettings.application/config/use_hidden_project_data_directory])."]
        fn get_save_extension(&self,) -> GString;
        #[doc = "Gets the Godot resource type associated with this loader. e.g. `\"Mesh\"` or `\"Animation\"`."]
        fn get_resource_type(&self,) -> GString;
        #[doc = "Gets the priority of this plugin for the recognized extension. Higher priority plugins will be preferred. The default priority is `1.0`."]
        fn get_priority(&self,) -> f32 {
            unimplemented !()
        }
        #[doc = "Gets the order of this importer to be run when importing resources. Importers with _lower_ import orders will be called first, and higher values will be called later. Use this to ensure the importer runs after the dependencies are already imported. The default import order is `0` unless overridden by a specific importer. See \\[enum ResourceImporter.ImportOrder] for some predefined values."]
        fn get_import_order(&self,) -> i32 {
            unimplemented !()
        }
        #[doc = "Gets the format version of this importer. Increment this version when making incompatible changes to the format of the imported resources.\n\nIf not overridden, the format version is `0`."]
        fn get_format_version(&self,) -> i32 {
            unimplemented !()
        }
        #[doc = "Gets whether the import option specified by `option_name` should be visible in the Import dock. The default implementation always returns `true`, making all options visible. This is mainly useful for hiding options that depend on others if one of them is disabled.\n\n\n```gdscript\nfunc _get_option_visibility(path, option_name, options):\n\t# Only show the lossy quality setting if the compression mode is set to \"Lossy\".\n\tif option_name == \"compress/lossy_quality\" and options.has(\"compress/mode\"):\n\t\treturn int(options[\"compress/mode\"]) == COMPRESS_LOSSY # This is a constant that you set\n\n\treturn true\n```\n"]
        fn get_option_visibility(&self, path: GString, option_name: StringName, options: VarDictionary,) -> bool {
            unimplemented !()
        }
        #[doc = "Imports `source_file` with the import `options` specified. Should return `@GlobalScope.OK` if the import is successful, other values indicate failure.\n\nThe imported resource is expected to be saved to `save_path + \".\" + _get_save_extension()`. If a different variant is preferred for a [feature tag]($DOCS_URL/tutorials/export/feature_tags.html), save the variant to `save_path + \".\" + tag + \".\" + _get_save_extension()` and add the feature tag to `platform_variants`.\n\nIf additional resource files are generated in the resource filesystem (`res://`), add their full path to `gen_files` so that the editor knows they depend on `source_file`.\n\nThis method must be overridden to do the actual importing work. See this class' description for an example of overriding this method."]
        fn import(&mut self, source_file: GString, save_path: GString, options: VarDictionary, platform_variants: Array < GString >, gen_files: Array < GString >,) -> crate::global::Error;
        #[doc = "Tells whether this importer can be run in parallel on threads, or, on the contrary, it's only safe for the editor to call it from the main thread, for one file at a time.\n\nIf this importer's implementation is thread-safe and can be run in parallel, override this with `true` to optimize for concurrency.\n\nIf not overridden, returns `false`."]
        fn can_import_threaded(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Called when the engine compilation profile editor wants to check what build options an imported resource needs. For example, [`ResourceImporterDynamicFont`][crate::classes::ResourceImporterDynamicFont] has a property called \\[member ResourceImporterDynamicFont.multichannel_signed_distance_field], that depends on the engine to be build with the \"msdfgen\" module. If that resource happened to be a custom one, it would be handled like this:\n\n```gdscript\nfunc _get_build_dependencies(path):\n\tvar resource = load(path)\n\tvar dependencies = PackedStringArray()\n\n\tif resource.multichannel_signed_distance_field:\n\t\tdependencies.push_back(\"module_msdfgen_enabled\")\n\n\treturn dependencies\n```"]
        fn get_build_dependencies(&self, path: GString,) -> PackedStringArray {
            unimplemented !()
        }
    }
    impl EditorImportPlugin {
        #[doc = "This function can only be called during the [`import`][`crate::classes::IEditorImportPlugin::import`] callback and it allows manually importing resources from it. This is useful when the imported file generates external resources that require importing (as example, images). Custom parameters for the \".import\" file can be passed via the `custom_options`. Additionally, in cases where multiple importers can handle a file, the `custom_importer` can be specified to force a specific one. This function performs a resource import and returns immediately with a success or error code. `generator_parameters` defines optional extra metadata which will be stored as `generator_parameters` in the `remap` section of the `.import` file, for example to store a md5 hash of the source data."]
        pub(crate) fn append_import_external_resource_full(&mut self, path: CowArg < GString >, custom_options: RefArg < AnyDictionary >, custom_importer: CowArg < GString >, generator_parameters: RefArg < Variant >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, RefArg < 'a1, AnyDictionary >, CowArg < 'a2, GString >, RefArg < 'a3, Variant >,);
            let args = (path, custom_options, custom_importer, generator_parameters,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorImportPlugin", "append_import_external_resource", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`append_import_external_resource_ex`][Self::append_import_external_resource_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "This function can only be called during the [`import`][`crate::classes::IEditorImportPlugin::import`] callback and it allows manually importing resources from it. This is useful when the imported file generates external resources that require importing (as example, images). Custom parameters for the \".import\" file can be passed via the `custom_options`. Additionally, in cases where multiple importers can handle a file, the `custom_importer` can be specified to force a specific one. This function performs a resource import and returns immediately with a success or error code. `generator_parameters` defines optional extra metadata which will be stored as `generator_parameters` in the `remap` section of the `.import` file, for example to store a md5 hash of the source data."]
        #[inline]
        pub fn append_import_external_resource(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            self.append_import_external_resource_ex(path,) . done()
        }
        #[doc = "This function can only be called during the [`import`][`crate::classes::IEditorImportPlugin::import`] callback and it allows manually importing resources from it. This is useful when the imported file generates external resources that require importing (as example, images). Custom parameters for the \".import\" file can be passed via the `custom_options`. Additionally, in cases where multiple importers can handle a file, the `custom_importer` can be specified to force a specific one. This function performs a resource import and returns immediately with a success or error code. `generator_parameters` defines optional extra metadata which will be stored as `generator_parameters` in the `remap` section of the `.import` file, for example to store a md5 hash of the source data."]
        #[inline]
        pub fn append_import_external_resource_ex < 'ex > (&'ex mut self, path: impl AsArg < GString > + 'ex,) -> ExAppendImportExternalResource < 'ex > {
            ExAppendImportExternalResource::new(self, path,)
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
    impl crate::obj::GodotClass for EditorImportPlugin {
        type Base = crate::classes::ResourceImporter;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorImportPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorImportPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::ResourceImporter > for EditorImportPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorImportPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorImportPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorImportPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorImportPlugin {
        type Target = crate::classes::ResourceImporter;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorImportPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorImportPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorImportPlugin__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorImportPlugin > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::ResourceImporter > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorImportPlugin::append_import_external_resource_ex`][super::EditorImportPlugin::append_import_external_resource_ex]."]
#[must_use]
pub struct ExAppendImportExternalResource < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorImportPlugin, path: CowArg < 'ex, GString >, custom_options: CowArg < 'ex, AnyDictionary >, custom_importer: CowArg < 'ex, GString >, generator_parameters: CowArg < 'ex, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAppendImportExternalResource < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorImportPlugin, path: impl AsArg < GString > + 'ex,) -> Self {
        let custom_options = AnyDictionary::new_untyped();
        let custom_importer = GString::from("");
        let generator_parameters = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), custom_options: CowArg::Owned(custom_options), custom_importer: CowArg::Owned(custom_importer), generator_parameters: CowArg::Owned(generator_parameters),
        }
    }
    #[inline]
    pub fn custom_options(self, custom_options: &'ex AnyDictionary) -> Self {
        Self {
            custom_options: CowArg::Borrowed(custom_options), .. self
        }
    }
    #[inline]
    pub fn custom_importer(self, custom_importer: impl AsArg < GString > + 'ex) -> Self {
        Self {
            custom_importer: custom_importer.into_arg(), .. self
        }
    }
    #[inline]
    pub fn generator_parameters(self, generator_parameters: &'ex Variant) -> Self {
        Self {
            generator_parameters: CowArg::Borrowed(generator_parameters), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, custom_options, custom_importer, generator_parameters,
        }
        = self;
        re_export::EditorImportPlugin::append_import_external_resource_full(surround_object, path, custom_options.cow_as_arg(), custom_importer, generator_parameters.cow_as_arg(),)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorImportPlugin;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorImportPlugin {
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
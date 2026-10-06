#![doc = "Sidecar module for class [`EditorExportPlatformExtension`][crate::classes::EditorExportPlatformExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorExportPlatformExtension` enums](https://docs.godotengine.org/en/stable/classes/class_editorexportplatformextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorExportPlatformExtension`.\n\nInherits [`EditorExportPlatform`][crate::classes::EditorExportPlatform].\n\nRelated symbols:\n\n* [`IEditorExportPlatformExtension`][crate::classes::IEditorExportPlatformExtension]: virtual methods\n\n\nSee also [Godot docs for `EditorExportPlatformExtension`](https://docs.godotengine.org/en/stable/classes/class_editorexportplatformextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorExportPlatformExtension::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nExternal [`EditorExportPlatform`][crate::classes::EditorExportPlatform] implementations should inherit from this class.\n\nTo use [`EditorExportPlatform`][crate::classes::EditorExportPlatform], register it using the [`add_export_platform`][`crate::classes::EditorPlugin::add_export_platform`] method first."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorExportPlatformExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorExportPlatformExtension`][crate::classes::EditorExportPlatformExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IEditorExportPlatform`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `EditorExportPlatformExtension` methods](https://docs.godotengine.org/en/stable/classes/class_editorexportplatformextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorExportPlatformExtension: crate::obj::GodotClass < Base = EditorExportPlatformExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Returns array of platform specific features for the specified `preset`."]
        fn get_preset_features(&self, preset: Option < Gd < crate::classes::EditorExportPreset > >,) -> PackedStringArray;
        #[doc = "Returns `true` if specified file is a valid executable (native executable or script) for the target platform."]
        fn is_executable(&self, path: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Returns a property list, as an [`Array`][crate::builtin::Array] of dictionaries. Each [`Dictionary`][crate::builtin::Dictionary] must at least contain the `name: StringName` and `type: Variant.Type` entries.\n\nAdditionally, the following keys are supported:\n\n- `hint: PropertyHint`\n\n- `hint_string: String`\n\n- `usage: PropertyUsageFlags`\n\n- `class_name: StringName`\n\n- `default_value: Variant`, default value of the property.\n\n- `update_visibility: bool`, if set to `true`, [`get_export_option_visibility`][`crate::classes::IEditorExportPlatformExtension::get_export_option_visibility`] is called for each property when this property is changed.\n\n- `required: bool`, if set to `true`, this property warnings are critical, and should be resolved to make export possible. This value is a hint for the [`has_valid_export_configuration`][`crate::classes::IEditorExportPlatformExtension::has_valid_export_configuration`] implementation, and not used by the engine directly.\n\nSee also [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`]."]
        fn get_export_options(&self,) -> Array < AnyDictionary > {
            unimplemented !()
        }
        #[doc = "Returns `true` if export options list is changed and presets should be updated."]
        fn should_update_export_options(&mut self,) -> bool {
            unimplemented !()
        }
        #[doc = "Validates `option` and returns visibility for the specified `preset`. Default implementation return `true` for all options."]
        fn get_export_option_visibility(&self, preset: Option < Gd < crate::classes::EditorExportPreset > >, option: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Validates `option` and returns warning message for the specified `preset`. Default implementation return empty string for all options."]
        fn get_export_option_warning(&self, preset: Option < Gd < crate::classes::EditorExportPreset > >, option: StringName,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns target OS name."]
        fn get_os_name(&self,) -> GString;
        #[doc = "Returns export platform name."]
        fn get_name(&self,) -> GString;
        #[doc = "Returns the platform logo displayed in the export dialog. The logo should be 32×32 pixels, adjusted for the current editor scale (see [`get_editor_scale`][`crate::classes::EditorInterface::get_editor_scale`])."]
        fn get_logo(&self,) -> Option < Gd < crate::classes::Texture2D > >;
        #[doc = "Returns `true` if one-click deploy options are changed and editor interface should be updated."]
        fn poll_export(&mut self,) -> bool {
            unimplemented !()
        }
        #[doc = "Returns the number of devices (or other options) available in the one-click deploy menu."]
        fn get_options_count(&self,) -> i32 {
            unimplemented !()
        }
        #[doc = "Returns tooltip of the one-click deploy menu button."]
        fn get_options_tooltip(&self,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns the item icon for the specified `device` in the one-click deploy menu. The icon should be 16×16 pixels, adjusted for the current editor scale (see [`get_editor_scale`][`crate::classes::EditorInterface::get_editor_scale`])."]
        fn get_option_icon(&self, device: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            unimplemented !()
        }
        #[doc = "Returns one-click deploy menu item label for the specified `device`."]
        fn get_option_label(&self, device: i32,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns one-click deploy menu item tooltip for the specified `device`."]
        fn get_option_tooltip(&self, device: i32,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns device architecture for one-click deploy."]
        fn get_device_architecture(&self, device: i32,) -> GString {
            unimplemented !()
        }
        #[doc = "Called by the editor before platform is unregistered."]
        fn cleanup(&mut self,) {
            unimplemented !()
        }
        #[doc = "This method is called when `device` one-click deploy menu option is selected.\n\nImplementation should export project to a temporary location, upload and run it on the specific `device`, or perform another action associated with the menu item."]
        fn run(&mut self, preset: Option < Gd < crate::classes::EditorExportPreset > >, device: i32, debug_flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Returns the icon of the one-click deploy menu button. The icon should be 16×16 pixels, adjusted for the current editor scale (see [`get_editor_scale`][`crate::classes::EditorInterface::get_editor_scale`])."]
        fn get_run_icon(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            unimplemented !()
        }
        #[doc = "Returns `true` if the specified `preset` is valid and can be exported. Use [`set_config_error`][`crate::classes::EditorExportPlatformExtension::set_config_error`] and [`set_config_missing_templates`][`crate::classes::EditorExportPlatformExtension::set_config_missing_templates`] to set error details.\n\nUsual implementations call [`has_valid_export_configuration`][`crate::classes::IEditorExportPlatformExtension::has_valid_export_configuration`] and [`has_valid_project_configuration`][`crate::classes::IEditorExportPlatformExtension::has_valid_project_configuration`] to determine if exporting is possible."]
        fn can_export(&self, preset: Option < Gd < crate::classes::EditorExportPreset > >, debug: bool,) -> bool {
            unimplemented !()
        }
        #[doc = "Returns `true` if export configuration is valid."]
        fn has_valid_export_configuration(&self, preset: Option < Gd < crate::classes::EditorExportPreset > >, debug: bool,) -> bool;
        #[doc = "Returns `true` if project configuration is valid."]
        fn has_valid_project_configuration(&self, preset: Option < Gd < crate::classes::EditorExportPreset > >,) -> bool;
        #[doc = "Returns array of supported binary extensions for the full project export."]
        fn get_binary_extensions(&self, preset: Option < Gd < crate::classes::EditorExportPreset > >,) -> PackedStringArray;
        #[doc = "Creates a full project at `path` for the specified `preset`.\n\nThis method is called when \"Export\" button is pressed in the export dialog.\n\nThis method implementation can call [`save_pack`][`crate::classes::EditorExportPlatform::save_pack`] or [`save_zip`][`crate::classes::EditorExportPlatform::save_zip`] to use default PCK/ZIP export process, or calls [`export_project_files`][`crate::classes::EditorExportPlatform::export_project_files`] and implement custom callback for processing each exported file."]
        fn export_project(&mut self, preset: Option < Gd < crate::classes::EditorExportPreset > >, debug: bool, path: GString, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error;
        #[doc = "Creates a PCK archive at `path` for the specified `preset`.\n\nThis method is called when \"Export PCK/ZIP\" button is pressed in the export dialog, with \"Export as Patch\" disabled, and PCK is selected as a file type."]
        fn export_pack(&mut self, preset: Option < Gd < crate::classes::EditorExportPreset > >, debug: bool, path: GString, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Create a ZIP archive at `path` for the specified `preset`.\n\nThis method is called when \"Export PCK/ZIP\" button is pressed in the export dialog, with \"Export as Patch\" disabled, and ZIP is selected as a file type."]
        fn export_zip(&mut self, preset: Option < Gd < crate::classes::EditorExportPreset > >, debug: bool, path: GString, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Creates a patch PCK archive at `path` for the specified `preset`, containing only the files that have changed since the last patch.\n\nThis method is called when \"Export PCK/ZIP\" button is pressed in the export dialog, with \"Export as Patch\" enabled, and PCK is selected as a file type.\n\n**Note:** The patches provided in `patches` have already been loaded when this method is called and are merely provided as context. When empty the patches defined in the export preset have been loaded instead."]
        fn export_pack_patch(&mut self, preset: Option < Gd < crate::classes::EditorExportPreset > >, debug: bool, path: GString, patches: PackedStringArray, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Create a ZIP archive at `path` for the specified `preset`, containing only the files that have changed since the last patch.\n\nThis method is called when \"Export PCK/ZIP\" button is pressed in the export dialog, with \"Export as Patch\" enabled, and ZIP is selected as a file type.\n\n**Note:** The patches provided in `patches` have already been loaded when this method is called and are merely provided as context. When empty the patches defined in the export preset have been loaded instead."]
        fn export_zip_patch(&mut self, preset: Option < Gd < crate::classes::EditorExportPreset > >, debug: bool, path: GString, patches: PackedStringArray, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Returns array of platform specific features."]
        fn get_platform_features(&self,) -> PackedStringArray;
        #[doc = "Returns protocol used for remote debugging. Default implementation return `tcp://`."]
        fn get_debug_protocol(&self,) -> GString {
            unimplemented !()
        }
        #[doc = "Initializes the plugin. Called by the editor when platform is registered."]
        fn initialize(&mut self,) {
            unimplemented !()
        }
    }
    impl EditorExportPlatformExtension {
        #[doc = "Sets current configuration error message text. This method should be called only from the [`can_export`][`crate::classes::IEditorExportPlatformExtension::can_export`], [`has_valid_export_configuration`][`crate::classes::IEditorExportPlatformExtension::has_valid_export_configuration`], or [`has_valid_project_configuration`][`crate::classes::IEditorExportPlatformExtension::has_valid_project_configuration`] implementations."]
        pub fn set_config_error(&self, error_text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (error_text.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(94usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatformExtension", "set_config_error", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns current configuration error message text. This method should be called only from the [`can_export`][`crate::classes::IEditorExportPlatformExtension::can_export`], [`has_valid_export_configuration`][`crate::classes::IEditorExportPlatformExtension::has_valid_export_configuration`], or [`has_valid_project_configuration`][`crate::classes::IEditorExportPlatformExtension::has_valid_project_configuration`] implementations."]
        pub fn get_config_error(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(95usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatformExtension", "get_config_error", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set to `true` is export templates are missing from the current configuration. This method should be called only from the [`can_export`][`crate::classes::IEditorExportPlatformExtension::can_export`], [`has_valid_export_configuration`][`crate::classes::IEditorExportPlatformExtension::has_valid_export_configuration`], or [`has_valid_project_configuration`][`crate::classes::IEditorExportPlatformExtension::has_valid_project_configuration`] implementations."]
        pub fn set_config_missing_templates(&self, missing_templates: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (missing_templates,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(96usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatformExtension", "set_config_missing_templates", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` is export templates are missing from the current configuration. This method should be called only from the [`can_export`][`crate::classes::IEditorExportPlatformExtension::can_export`], [`has_valid_export_configuration`][`crate::classes::IEditorExportPlatformExtension::has_valid_export_configuration`], or [`has_valid_project_configuration`][`crate::classes::IEditorExportPlatformExtension::has_valid_project_configuration`] implementations."]
        pub fn get_config_missing_templates(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(97usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatformExtension", "get_config_missing_templates", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorExportPlatformExtension {
        type Base = crate::classes::EditorExportPlatform;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorExportPlatformExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorExportPlatformExtension {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::EditorExportPlatform > for EditorExportPlatformExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorExportPlatformExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorExportPlatformExtension {
        
    }
    impl crate::obj::cap::GodotDefault for EditorExportPlatformExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorExportPlatformExtension {
        type Target = crate::classes::EditorExportPlatform;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorExportPlatformExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorExportPlatformExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorExportPlatformExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorExportPlatformExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorExportPlatform > for $Class {
                
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
    use super::re_export::EditorExportPlatformExtension;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorExportPlatformExtension {
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
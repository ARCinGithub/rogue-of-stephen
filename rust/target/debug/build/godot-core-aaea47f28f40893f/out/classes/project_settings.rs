#![doc = "Sidecar module for class [`ProjectSettings`][crate::classes::ProjectSettings].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ProjectSettings` enums](https://docs.godotengine.org/en/stable/classes/class_projectsettings.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ProjectSettings`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`project_settings`][crate::classes::project_settings]: sidecar module with related enum/flag types\n* [`SignalsOfProjectSettings`][crate::classes::project_settings::SignalsOfProjectSettings]: signal collection\n\n\nSee also [Godot docs for `ProjectSettings`](https://docs.godotengine.org/en/stable/classes/class_projectsettings.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nStores variables that can be accessed from everywhere. Use [`get_setting`][`crate::classes::ProjectSettings::get_setting`], [`set_setting`][`crate::classes::ProjectSettings::set_setting`] or [`has_setting`][`crate::classes::ProjectSettings::has_setting`] to access them. Variables stored in `project.godot` are also loaded into `ProjectSettings`, making this object very useful for reading custom game configuration options.\n\nWhen naming a Project Settings property, use the full path to the setting including the category. For example, `\"application/config/name\"` for the project name. Category and property names can be viewed in the Project Settings dialog.\n\n**Feature tags:** Project settings can be overridden for specific platforms and configurations (debug, release, ...) using [feature tags]($DOCS_URL/tutorials/export/feature_tags.html).\n\n**Overriding:** Any project setting can be overridden by creating a file named `override.cfg` in the project's root directory. This can also be used in exported projects by placing this file in the same directory as the project binary. Overriding will still take the base project settings' [feature tags]($DOCS_URL/tutorials/export/feature_tags.html) in account. Therefore, make sure to _also_ override the setting with the desired feature tags if you want them to override base project settings on all platforms and configurations."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ProjectSettings {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ProjectSettings {
        #[doc = "Returns `true` if a configuration value is present.\n\n**Note:** In order to be be detected, custom settings have to be either defined with [`set_setting`][`crate::classes::ProjectSettings::set_setting`], or exist in the `project.godot` file. This is especially relevant when using [`set_initial_value`][`crate::classes::ProjectSettings::set_initial_value`]."]
        pub fn has_setting(&self, name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(73usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "has_setting", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of a setting.\n\n\n```gdscript\nProjectSettings.set_setting(\"application/config/name\", \"Example\")\n```\n\n\nThis can also be used to erase custom project settings. To do this change the setting value to `null`."]
        pub fn set_setting(&mut self, name: impl AsArg < GString >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(74usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_setting", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the setting identified by `name`. If the setting doesn't exist and `default_value` is specified, the value of `default_value` is returned. Otherwise, `null` is returned.\n\n\n```gdscript\nprint(ProjectSettings.get_setting(\"application/config/name\"))\nprint(ProjectSettings.get_setting(\"application/config/custom_description\", \"No description specified.\"))\n```\n\n\n**Note:** This method doesn't take potential feature overrides into account automatically. Use [`get_setting_with_override`][`crate::classes::ProjectSettings::get_setting_with_override`] to handle seamlessly.\n\nSee also [`has_setting`][`crate::classes::ProjectSettings::has_setting`] to check whether a setting exists."]
        pub(crate) fn get_setting_full(&self, name: CowArg < GString >, default_value: RefArg < Variant >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (name, default_value,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(75usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_setting", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_setting_ex`][Self::get_setting_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the value of the setting identified by `name`. If the setting doesn't exist and `default_value` is specified, the value of `default_value` is returned. Otherwise, `null` is returned.\n\n\n```gdscript\nprint(ProjectSettings.get_setting(\"application/config/name\"))\nprint(ProjectSettings.get_setting(\"application/config/custom_description\", \"No description specified.\"))\n```\n\n\n**Note:** This method doesn't take potential feature overrides into account automatically. Use [`get_setting_with_override`][`crate::classes::ProjectSettings::get_setting_with_override`] to handle seamlessly.\n\nSee also [`has_setting`][`crate::classes::ProjectSettings::has_setting`] to check whether a setting exists."]
        #[inline]
        pub fn get_setting(&self, name: impl AsArg < GString >,) -> Variant {
            self.get_setting_ex(name,) . done()
        }
        #[doc = "Returns the value of the setting identified by `name`. If the setting doesn't exist and `default_value` is specified, the value of `default_value` is returned. Otherwise, `null` is returned.\n\n\n```gdscript\nprint(ProjectSettings.get_setting(\"application/config/name\"))\nprint(ProjectSettings.get_setting(\"application/config/custom_description\", \"No description specified.\"))\n```\n\n\n**Note:** This method doesn't take potential feature overrides into account automatically. Use [`get_setting_with_override`][`crate::classes::ProjectSettings::get_setting_with_override`] to handle seamlessly.\n\nSee also [`has_setting`][`crate::classes::ProjectSettings::has_setting`] to check whether a setting exists."]
        #[inline]
        pub fn get_setting_ex < 'ex > (&'ex self, name: impl AsArg < GString > + 'ex,) -> ExGetSetting < 'ex > {
            ExGetSetting::new(self, name,)
        }
        #[doc = "Similar to [`get_setting`][`crate::classes::ProjectSettings::get_setting`], but applies feature tag overrides if any exists and is valid.\n\n**Example:** If the setting override `\"application/config/name.windows\"` exists, and the following code is executed on a _Windows_ operating system, the overridden setting is printed instead:\n\n\n```gdscript\nprint(ProjectSettings.get_setting_with_override(\"application/config/name\"))\n```\n"]
        pub fn get_setting_with_override(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(76usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_setting_with_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of registered global classes. Each global class is represented as a [`Dictionary`][crate::builtin::Dictionary] that contains the following entries:\n\n- `base` is a name of the base class;\n\n- `class` is a name of the registered global class;\n\n- `icon` is a path to a custom icon of the global class, if it has any;\n\n- `language` is a name of a programming language in which the global class is written;\n\n- `path` is a path to a file containing the global class.\n\n**Note:** Both the script and the icon paths are local to the project filesystem, i.e. they start with `res://`."]
        pub fn get_global_class_list(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(77usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_global_class_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Similar to [`get_setting_with_override`][`crate::classes::ProjectSettings::get_setting_with_override`], but applies feature tag overrides instead of current OS features."]
        pub fn get_setting_with_override_and_custom_features(&self, name: impl AsArg < StringName >, features: &PackedStringArray,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, PackedStringArray >,);
            let args = (name.into_arg(), RefArg::new(features),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(78usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_setting_with_override_and_custom_features", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the order of a configuration value (influences when saved to the config file)."]
        pub fn set_order(&mut self, name: impl AsArg < GString >, position: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (name.into_arg(), position,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(79usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_order", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the order of a configuration value (influences when saved to the config file)."]
        pub fn get_order(&self, name: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(80usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_order", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the specified setting's initial value. This is the value the setting reverts to. The setting should already exist before calling this method. Note that project settings equal to their default value are not saved, so your code needs to account for that.\n\n```gdscript\nextends EditorPlugin\n\nconst SETTING_NAME = \"addons/my_setting\"\nconst SETTING_DEFAULT = 10.0\n\nfunc _enter_tree():\n\tif not ProjectSettings.has_setting(SETTING_NAME):\n\t\tProjectSettings.set_setting(SETTING_NAME, SETTING_DEFAULT)\n\n\tProjectSettings.set_initial_value(SETTING_NAME, SETTING_DEFAULT)\n```\n\nIf you have a project setting defined by an [`EditorPlugin`][crate::classes::EditorPlugin], but want to use it in a running project, you will need a similar code at runtime."]
        pub fn set_initial_value(&mut self, name: impl AsArg < GString >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(81usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_initial_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Defines if the specified setting is considered basic or advanced. Basic settings will always be shown in the project settings. Advanced settings will only be shown if the user enables the \"Advanced Settings\" option."]
        pub fn set_as_basic(&mut self, name: impl AsArg < GString >, basic: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (name.into_arg(), basic,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(82usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_as_basic", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Defines if the specified setting is considered internal. An internal setting won't show up in the Project Settings dialog. This is mostly useful for addons that need to store their own internal settings without exposing them directly to the user."]
        pub fn set_as_internal(&mut self, name: impl AsArg < GString >, internal: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (name.into_arg(), internal,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(83usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_as_internal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a custom property info to a property. The dictionary must contain:\n\n- `\"name\"`: [`String`][crate::builtin::GString] (the property's name)\n\n- `\"type\"`: `int` (see \\[enum Variant.Type])\n\n- optionally `\"hint\"`: `int` (see \\[enum PropertyHint]) and `\"hint_string\"`: [`String`][crate::builtin::GString]\n\n\n```gdscript\nProjectSettings.set(\"category/property_name\", 0)\n\nvar property_info = {\n\t\"name\": \"category/property_name\",\n\t\"type\": TYPE_INT,\n\t\"hint\": PROPERTY_HINT_ENUM,\n\t\"hint_string\": \"one,two,three\"\n}\n\nProjectSettings.add_property_info(property_info)\n```\n\n\n**Note:** Setting `\"usage\"` for the property is not supported. Use [`set_as_basic`][`crate::classes::ProjectSettings::set_as_basic`], [`set_restart_if_changed`][`crate::classes::ProjectSettings::set_restart_if_changed`], and [`set_as_internal`][`crate::classes::ProjectSettings::set_as_internal`] to modify usage flags."]
        pub fn add_property_info(&mut self, hint: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(hint),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(84usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "add_property_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether a setting requires restarting the editor to properly take effect.\n\n**Note:** This is just a hint to display to the user that the editor must be restarted for changes to take effect. Enabling [`set_restart_if_changed`][`crate::classes::ProjectSettings::set_restart_if_changed`] does _not_ delay the setting being set when changed."]
        pub fn set_restart_if_changed(&mut self, name: impl AsArg < GString >, restart: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (name.into_arg(), restart,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(85usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_restart_if_changed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears the whole configuration (not recommended, may break things)."]
        pub fn clear(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(86usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the localized path (starting with `res://`) corresponding to the absolute, native OS `path`. See also [`globalize_path`][`crate::classes::ProjectSettings::globalize_path`]."]
        pub fn localize_path(&self, path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(87usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "localize_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the absolute, native OS path corresponding to the localized `path` (starting with `res://` or `user://`). The returned path will vary depending on the operating system and user preferences. See [File paths in Godot projects]($DOCS_URL/tutorials/io/data_paths.html) to see what those paths convert to. See also [`localize_path`][`crate::classes::ProjectSettings::localize_path`].\n\n**Note:** [`globalize_path`][`crate::classes::ProjectSettings::globalize_path`] with `res://` will not work in an exported project. Instead, prepend the executable's base directory to the path when running from an exported project:\n\n```gdscript\nvar path = \"\"\nif OS.has_feature(\"editor\"):\n\t# Running from an editor binary.\n\t# `path` will contain the absolute path to `hello.txt` located in the project root.\n\tpath = ProjectSettings.globalize_path(\"res://hello.txt\")\nelse:\n\t# Running from an exported project.\n\t# `path` will contain the absolute path to `hello.txt` next to the executable.\n\t# This is *not* identical to using `ProjectSettings.globalize_path()` with a `res://` path,\n\t# but is close enough in spirit.\n\tpath = OS.get_executable_path().get_base_dir().path_join(\"hello.txt\")\n```"]
        pub fn globalize_path(&self, path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(88usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "globalize_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves the configuration to the `project.godot` file.\n\n**Note:** This method is intended to be used by editor plugins, as modified `ProjectSettings` can't be loaded back in the running app. If you want to change project settings in exported projects, use [`save_custom`][`crate::classes::ProjectSettings::save_custom`] to save `override.cfg` file."]
        pub fn save(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(89usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "save", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads the contents of the .pck or .zip file specified by `pack` into the resource filesystem (`res://`). Returns `true` on success.\n\n**Note:** If a file from `pack` shares the same path as a file already in the resource filesystem, any attempts to load that file will use the file from `pack` unless `replace_files` is set to `false`.\n\n**Note:** The optional `offset` parameter can be used to specify the offset in bytes to the start of the resource pack. This is only supported for .pck files.\n\n**Note:** [`DirAccess`][crate::classes::DirAccess] will not show changes made to the contents of `res://` after calling this function."]
        pub(crate) fn load_resource_pack_full(&mut self, pack: CowArg < GString >, replace_files: bool, offset: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool, i32,);
            let args = (pack, replace_files, offset,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(90usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "load_resource_pack", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`load_resource_pack_ex`][Self::load_resource_pack_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Loads the contents of the .pck or .zip file specified by `pack` into the resource filesystem (`res://`). Returns `true` on success.\n\n**Note:** If a file from `pack` shares the same path as a file already in the resource filesystem, any attempts to load that file will use the file from `pack` unless `replace_files` is set to `false`.\n\n**Note:** The optional `offset` parameter can be used to specify the offset in bytes to the start of the resource pack. This is only supported for .pck files.\n\n**Note:** [`DirAccess`][crate::classes::DirAccess] will not show changes made to the contents of `res://` after calling this function."]
        #[inline]
        pub fn load_resource_pack(&mut self, pack: impl AsArg < GString >,) -> bool {
            self.load_resource_pack_ex(pack,) . done()
        }
        #[doc = "Loads the contents of the .pck or .zip file specified by `pack` into the resource filesystem (`res://`). Returns `true` on success.\n\n**Note:** If a file from `pack` shares the same path as a file already in the resource filesystem, any attempts to load that file will use the file from `pack` unless `replace_files` is set to `false`.\n\n**Note:** The optional `offset` parameter can be used to specify the offset in bytes to the start of the resource pack. This is only supported for .pck files.\n\n**Note:** [`DirAccess`][crate::classes::DirAccess] will not show changes made to the contents of `res://` after calling this function."]
        #[inline]
        pub fn load_resource_pack_ex < 'ex > (&'ex mut self, pack: impl AsArg < GString > + 'ex,) -> ExLoadResourcePack < 'ex > {
            ExLoadResourcePack::new(self, pack,)
        }
        #[doc = "Saves the configuration to a custom file. The file extension must be `.godot` (to save in text-based [`ConfigFile`][crate::classes::ConfigFile] format) or `.binary` (to save in binary format). You can also save `override.cfg` file, which is also text, but can be used in exported projects unlike other formats."]
        pub fn save_custom(&mut self, file: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(91usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "save_custom", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets an array of the settings which have been changed since the last save. Note that internally `changed_settings` is cleared after a successful save, so generally the most appropriate place to use this method is when processing `settings_changed`."]
        pub fn get_changed_settings(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(92usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_changed_settings", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Checks if any settings with the prefix `setting_prefix` exist in the set of changed settings. See also [`get_changed_settings`][`crate::classes::ProjectSettings::get_changed_settings`]."]
        pub fn check_changed_settings_in_group(&self, setting_prefix: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (setting_prefix.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(93usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "check_changed_settings_in_group", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ProjectSettings {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ProjectSettings"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Core;
        
    }
    unsafe impl crate::obj::Bounds for ProjectSettings {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ProjectSettings {
        
    }
    impl crate::obj::Singleton for ProjectSettings {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"ProjectSettings"))
            }
        }
    }
    impl std::ops::Deref for ProjectSettings {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ProjectSettings {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ProjectSettings__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ProjectSettings` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`ProjectSettings::get_setting_ex`][super::ProjectSettings::get_setting_ex]."]
#[must_use]
pub struct ExGetSetting < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::ProjectSettings, name: CowArg < 'ex, GString >, default_value: CowArg < 'ex, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSetting < 'ex > {
    fn new(surround_object: &'ex re_export::ProjectSettings, name: impl AsArg < GString > + 'ex,) -> Self {
        let default_value = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), default_value: CowArg::Owned(default_value),
        }
    }
    #[inline]
    pub fn default_value(self, default_value: &'ex Variant) -> Self {
        Self {
            default_value: CowArg::Borrowed(default_value), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, name, default_value,
        }
        = self;
        re_export::ProjectSettings::get_setting_full(surround_object, name, default_value.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`ProjectSettings::load_resource_pack_ex`][super::ProjectSettings::load_resource_pack_ex]."]
#[must_use]
pub struct ExLoadResourcePack < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ProjectSettings, pack: CowArg < 'ex, GString >, replace_files: bool, offset: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExLoadResourcePack < 'ex > {
    fn new(surround_object: &'ex mut re_export::ProjectSettings, pack: impl AsArg < GString > + 'ex,) -> Self {
        let replace_files = true;
        let offset = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pack: pack.into_arg(), replace_files: replace_files, offset: offset,
        }
    }
    #[inline]
    pub fn replace_files(self, replace_files: bool) -> Self {
        Self {
            replace_files: replace_files, .. self
        }
    }
    #[inline]
    pub fn offset(self, offset: i32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, pack, replace_files, offset,
        }
        = self;
        re_export::ProjectSettings::load_resource_pack_full(surround_object, pack, replace_files, offset,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ProjectSettings;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`ProjectSettings`][crate::classes::ProjectSettings] class."]
    pub struct SignalsOfProjectSettings < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfProjectSettings < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn settings_changed(&mut self) -> SigSettingsChanged < 'c, C > {
            SigSettingsChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "settings_changed")
            }
        }
    }
    type TypedSigSettingsChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSettingsChanged < 'c, C: WithSignals > {
        typed: TypedSigSettingsChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSettingsChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSettingsChanged < 'c, C > {
        type Target = TypedSigSettingsChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSettingsChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for ProjectSettings {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfProjectSettings < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfProjectSettings < 'c, C > {
        type Target = < < ProjectSettings as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = ProjectSettings;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfProjectSettings < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = ProjectSettings;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
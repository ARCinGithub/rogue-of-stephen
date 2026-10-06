#![doc = "Sidecar module for class [`Engine`][crate::classes::Engine].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Engine` enums](https://docs.godotengine.org/en/stable/classes/class_engine.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Engine`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`engine`][crate::classes::engine]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Engine`](https://docs.godotengine.org/en/stable/classes/class_engine.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThe `Engine` singleton allows you to query and modify the project's run-time parameters, such as frames per second, time scale, and others. It also stores information about the current build of Godot, such as the current version."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Engine {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Engine {
        pub fn set_physics_ticks_per_second(&mut self, physics_ticks_per_second: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (physics_ticks_per_second,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(32usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_physics_ticks_per_second", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_ticks_per_second(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(33usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_physics_ticks_per_second", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_physics_steps_per_frame(&mut self, max_physics_steps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_physics_steps,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(34usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_max_physics_steps_per_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_physics_steps_per_frame(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(35usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_max_physics_steps_per_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_jitter_fix(&mut self, physics_jitter_fix: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (physics_jitter_fix,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(36usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_physics_jitter_fix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_jitter_fix(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(37usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_physics_jitter_fix", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the fraction through the current physics tick we are at the time of rendering the frame. This can be used to implement fixed timestep interpolation."]
        pub fn get_physics_interpolation_fraction(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(38usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_physics_interpolation_fraction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_fps(&mut self, max_fps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_fps,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(39usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_max_fps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_fps(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(40usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_max_fps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_time_scale(&mut self, time_scale: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (time_scale,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(41usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_time_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_time_scale(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(42usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_time_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total number of frames drawn since the engine started.\n\n**Note:** On headless platforms, or if rendering is disabled with `--disable-render-loop` via command line, this method always returns `0`. See also [`get_process_frames`][`crate::classes::Engine::get_process_frames`]."]
        pub fn get_frames_drawn(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(43usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_frames_drawn", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the average frames rendered every second (FPS), also known as the framerate."]
        pub fn get_frames_per_second(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(44usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_frames_per_second", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total number of frames passed since the engine started. This number is increased every **physics frame**. See also [`get_process_frames`][`crate::classes::Engine::get_process_frames`].\n\nThis method can be used to run expensive logic less often without relying on a [`Timer`][crate::classes::Timer]:\n\n\n```gdscript\nfunc _physics_process(_delta):\n\tif Engine.get_physics_frames() % 2 == 0:\n\t\tpass # Run expensive logic only once every 2 physics frames here.\n```\n"]
        pub fn get_physics_frames(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(45usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_physics_frames", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total number of frames passed since the engine started. This number is increased every **process frame**, regardless of whether the render loop is enabled. See also [`get_frames_drawn`][`crate::classes::Engine::get_frames_drawn`] and [`get_physics_frames`][`crate::classes::Engine::get_physics_frames`].\n\nThis method can be used to run expensive logic less often without relying on a [`Timer`][crate::classes::Timer]:\n\n\n```gdscript\nfunc _process(_delta):\n\tif Engine.get_process_frames() % 5 == 0:\n\t\tpass # Run expensive logic only once every 5 process (render) frames here.\n```\n"]
        pub fn get_process_frames(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(46usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_process_frames", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the instance of the [`MainLoop`][crate::classes::MainLoop]. This is usually the main [`SceneTree`][crate::classes::SceneTree] and is the same as [`get_tree`][`crate::classes::Node::get_tree`].\n\n**Note:** The type instantiated as the main loop can changed with \\[member ProjectSettings.application/run/main_loop_type]."]
        pub fn get_main_loop(&self,) -> Option < Gd < crate::classes::MainLoop > > {
            type CallRet = Option < Gd < crate::classes::MainLoop > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(47usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_main_loop", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current engine version information as a [`Dictionary`][crate::builtin::Dictionary] containing the following entries:\n\n- `major` - Major version number as an int;\n\n- `minor` - Minor version number as an int;\n\n- `patch` - Patch version number as an int;\n\n- `hex` - Full version encoded as a hexadecimal int with one byte (2 hex digits) per number (see example below);\n\n- `status` - Status (such as \"beta\", \"rc1\", \"rc2\", \"stable\", etc.) as a String;\n\n- `build` - Build name (e.g. \"custom_build\") as a String;\n\n- `hash` - Full Git commit hash as a String;\n\n- `timestamp` - Holds the Git commit date UNIX timestamp in seconds as an int, or `0` if unavailable;\n\n- `string` - `major`, `minor`, `patch`, `status`, and `build` in a single String.\n\nThe `hex` value is encoded as follows, from left to right: one byte for the major, one byte for the minor, one byte for the patch version. For example, \"3.1.12\" would be `0x03010C`.\n\n**Note:** The `hex` value is still an `int` internally, and printing it will give you its decimal representation, which is not particularly meaningful. Use hexadecimal literals for quick version comparisons from code:\n\n\n```gdscript\nif Engine.get_version_info().hex >= 0x040100:\n\tpass # Do things specific to version 4.1 or later.\nelse:\n\tpass # Do things specific to versions before 4.1.\n```\n"]
        pub fn get_version_info(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(48usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_version_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the engine author information as a [`Dictionary`][crate::builtin::Dictionary], where each entry is an [`Array`][crate::builtin::Array] of strings with the names of notable contributors to the Godot Engine: `lead_developers`, `founders`, `project_managers`, and `developers`."]
        pub fn get_author_info(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(49usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_author_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of dictionaries with copyright information for every component of Godot's source code.\n\nEvery [`Dictionary`][crate::builtin::Dictionary] contains a `name` identifier, and a `parts` array of dictionaries. It describes the component in detail with the following entries:\n\n- `files` - [`Array`][crate::builtin::Array] of file paths from the source code affected by this component;\n\n- `copyright` - [`Array`][crate::builtin::Array] of owners of this component;\n\n- `license` - The license applied to this component (such as \"[Expat](https://en.wikipedia.org/wiki/MIT_License#Ambiguity_and_variants)\" or \"[CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/)\")."]
        pub fn get_copyright_info(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(50usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_copyright_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Dictionary`][crate::builtin::Dictionary] of categorized donor names. Each entry is an [`Array`][crate::builtin::Array] of strings:\n\n{`platinum_sponsors`, `gold_sponsors`, `silver_sponsors`, `bronze_sponsors`, `mini_sponsors`, `gold_donors`, `silver_donors`, `bronze_donors`}"]
        pub fn get_donor_info(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(51usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_donor_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Dictionary`][crate::builtin::Dictionary] of licenses used by Godot and included third party components. Each entry is a license name (such as \"[Expat](https://en.wikipedia.org/wiki/MIT_License#Ambiguity_and_variants)\") and its associated text."]
        pub fn get_license_info(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(52usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_license_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the full Godot license text."]
        pub fn get_license_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(53usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_license_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the CPU architecture the Godot binary was built for. Possible return values include `\"x86_64\"`, `\"x86_32\"`, `\"arm64\"`, `\"arm32\"`, `\"rv64\"`, `\"ppc64\"`, `\"loongarch64\"`, `\"wasm64\"`, and `\"wasm32\"`.\n\nTo detect whether the current build is 64-bit, or the type of architecture, don't use the architecture name. Instead, use [`has_feature`][`crate::classes::Os::has_feature`] to check for the `\"64\"` feature tag, or tags such as `\"x86\"` or `\"arm\"`. See the [Feature Tags]($DOCS_URL/tutorials/export/feature_tags.html) documentation for more details.\n\n**Note:** This method does _not_ return the name of the system's CPU architecture (like [`get_processor_name`][`crate::classes::Os::get_processor_name`]). For example, when running an `x86_32` Godot binary on an `x86_64` system, the returned value will still be `\"x86_32\"`."]
        pub fn get_architecture_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(54usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_architecture_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the engine is inside the fixed physics process step of the main loop.\n\n```gdscript\nfunc _enter_tree():\n\t# Depending on when the node is added to the tree,\n\t# prints either \"true\" or \"false\".\n\tprint(Engine.is_in_physics_frame())\n\nfunc _process(delta):\n\tprint(Engine.is_in_physics_frame()) # Prints false\n\nfunc _physics_process(delta):\n\tprint(Engine.is_in_physics_frame()) # Prints true\n```"]
        pub fn is_in_physics_frame(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(55usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_in_physics_frame", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a singleton with the given `name` exists in the global scope. See also [`get_singleton`][`crate::classes::Engine::get_singleton`].\n\n\n```gdscript\nprint(Engine.has_singleton(\"OS\"))          # Prints true\nprint(Engine.has_singleton(\"Engine\"))      # Prints true\nprint(Engine.has_singleton(\"AudioServer\")) # Prints true\nprint(Engine.has_singleton(\"Unknown\"))     # Prints false\n```\n\n\n**Note:** Global singletons are not the same as autoloaded nodes, which are configurable in the project settings."]
        pub fn has_singleton(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(56usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "has_singleton", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the global singleton with the given `name`, or `null` if it does not exist. Often used for plugins. See also [`has_singleton`][`crate::classes::Engine::has_singleton`] and [`get_singleton_list`][`crate::classes::Engine::get_singleton_list`].\n\n**Note:** Global singletons are not the same as autoloaded nodes, which are configurable in the project settings."]
        pub fn get_singleton(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Object > > {
            type CallRet = Option < Gd < crate::classes::Object > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(57usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_singleton", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers the given [`Object`][crate::classes::Object] `instance` as a singleton, available globally under `name`. Useful for plugins."]
        pub fn register_singleton(&mut self, name: impl AsArg < StringName >, instance: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::Object > > >,);
            let args = (name.into_arg(), instance.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(58usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "register_singleton", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the singleton registered under `name`. The singleton object is _not_ freed. Only works with user-defined singletons registered with [`register_singleton`][`crate::classes::Engine::register_singleton`]."]
        pub fn unregister_singleton(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(59usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "unregister_singleton", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of names of all available global singletons. See also [`get_singleton`][`crate::classes::Engine::get_singleton`]."]
        pub fn get_singleton_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(60usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_singleton_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers a [`ScriptLanguage`][crate::classes::ScriptLanguage] instance to be available with `ScriptServer`.\n\nReturns:\n\n- [`Error::OK`][`crate::global::Error::OK`] on success;\n\n- [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`] if `ScriptServer` has reached the limit and cannot register any new language;\n\n- [`Error::ERR_ALREADY_EXISTS`][`crate::global::Error::ERR_ALREADY_EXISTS`] if `ScriptServer` already contains a language with similar extension/name/type."]
        pub fn register_script_language(&mut self, language: impl AsArg < Option < Gd < crate::classes::ScriptLanguage >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ScriptLanguage > > >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(61usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "register_script_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unregisters the [`ScriptLanguage`][crate::classes::ScriptLanguage] instance from `ScriptServer`.\n\nReturns:\n\n- [`Error::OK`][`crate::global::Error::OK`] on success;\n\n- [`Error::ERR_DOES_NOT_EXIST`][`crate::global::Error::ERR_DOES_NOT_EXIST`] if the language is not registered in `ScriptServer`."]
        pub fn unregister_script_language(&mut self, language: impl AsArg < Option < Gd < crate::classes::ScriptLanguage >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ScriptLanguage > > >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(62usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "unregister_script_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of available script languages. Use with [`get_script_language`][`crate::classes::Engine::get_script_language`]."]
        pub fn get_script_language_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(63usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_script_language_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an instance of a [`ScriptLanguage`][crate::classes::ScriptLanguage] with the given `index`."]
        pub fn get_script_language(&self, index: i32,) -> Option < Gd < crate::classes::ScriptLanguage > > {
            type CallRet = Option < Gd < crate::classes::ScriptLanguage > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(64usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_script_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Captures and returns backtraces from all registered script languages.\n\nBy default, the returned [`ScriptBacktrace`][crate::classes::ScriptBacktrace] will only contain stack frames in editor builds and debug builds. To enable them for release builds as well, you need to enable \\[member ProjectSettings.debug/settings/gdscript/always_track_call_stacks].\n\nIf `include_variables` is `true`, the backtrace will also include the names and values of any global variables (e.g. autoload singletons) at the point of the capture, as well as local variables and class member variables at each stack frame. This will however will only be respected when running the game with a debugger attached, like when running the game from the editor. To enable it for export builds as well, you need to enable \\[member ProjectSettings.debug/settings/gdscript/always_track_local_variables].\n\n**Warning:** When `include_variables` is `true`, any captured variables can potentially (e.g. with GDScript backtraces) be their actual values, including any object references. This means that storing such a [`ScriptBacktrace`][crate::classes::ScriptBacktrace] will prevent those objects from being deallocated, so it's generally recommended not to do so."]
        pub(crate) fn capture_script_backtraces_full(&self, include_variables: bool,) -> Array < Gd < crate::classes::ScriptBacktrace > > {
            type CallRet = Array < Gd < crate::classes::ScriptBacktrace > >;
            type CallParams = (bool,);
            let args = (include_variables,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(65usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "capture_script_backtraces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`capture_script_backtraces_ex`][Self::capture_script_backtraces_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Captures and returns backtraces from all registered script languages.\n\nBy default, the returned [`ScriptBacktrace`][crate::classes::ScriptBacktrace] will only contain stack frames in editor builds and debug builds. To enable them for release builds as well, you need to enable \\[member ProjectSettings.debug/settings/gdscript/always_track_call_stacks].\n\nIf `include_variables` is `true`, the backtrace will also include the names and values of any global variables (e.g. autoload singletons) at the point of the capture, as well as local variables and class member variables at each stack frame. This will however will only be respected when running the game with a debugger attached, like when running the game from the editor. To enable it for export builds as well, you need to enable \\[member ProjectSettings.debug/settings/gdscript/always_track_local_variables].\n\n**Warning:** When `include_variables` is `true`, any captured variables can potentially (e.g. with GDScript backtraces) be their actual values, including any object references. This means that storing such a [`ScriptBacktrace`][crate::classes::ScriptBacktrace] will prevent those objects from being deallocated, so it's generally recommended not to do so."]
        #[inline]
        pub fn capture_script_backtraces(&self,) -> Array < Gd < crate::classes::ScriptBacktrace > > {
            self.capture_script_backtraces_ex() . done()
        }
        #[doc = "Captures and returns backtraces from all registered script languages.\n\nBy default, the returned [`ScriptBacktrace`][crate::classes::ScriptBacktrace] will only contain stack frames in editor builds and debug builds. To enable them for release builds as well, you need to enable \\[member ProjectSettings.debug/settings/gdscript/always_track_call_stacks].\n\nIf `include_variables` is `true`, the backtrace will also include the names and values of any global variables (e.g. autoload singletons) at the point of the capture, as well as local variables and class member variables at each stack frame. This will however will only be respected when running the game with a debugger attached, like when running the game from the editor. To enable it for export builds as well, you need to enable \\[member ProjectSettings.debug/settings/gdscript/always_track_local_variables].\n\n**Warning:** When `include_variables` is `true`, any captured variables can potentially (e.g. with GDScript backtraces) be their actual values, including any object references. This means that storing such a [`ScriptBacktrace`][crate::classes::ScriptBacktrace] will prevent those objects from being deallocated, so it's generally recommended not to do so."]
        #[inline]
        pub fn capture_script_backtraces_ex < 'ex > (&'ex self,) -> ExCaptureScriptBacktraces < 'ex > {
            ExCaptureScriptBacktraces::new(self,)
        }
        #[doc = "Returns `true` if the script is currently running inside the editor, otherwise returns `false`. This is useful for `@tool` scripts to conditionally draw editor helpers, or prevent accidentally running \"game\" code that would affect the scene state while in the editor:\n\n\n```gdscript\nif Engine.is_editor_hint():\n\tdraw_gizmos()\nelse:\n\tsimulate_physics()\n```\n\n\nSee [Running code in the editor]($DOCS_URL/tutorials/plugins/running_code_in_the_editor.html) in the documentation for more information.\n\n**Note:** To detect whether the script is running on an editor _build_ (such as when pressing `F5`), use [`has_feature`][`crate::classes::Os::has_feature`] with the `\"editor\"` argument instead. `OS.has_feature(\"editor\")` evaluate to `true` both when the script is running in the editor and when running the project from the editor, but returns `false` when run from an exported project."]
        pub fn is_editor_hint(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(66usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_editor_hint", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the engine is running embedded in the editor. This is useful to prevent attempting to update window mode or window flags that are not supported when running the project embedded in the editor."]
        pub fn is_embedded_in_editor(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(67usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_embedded_in_editor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the path to the [`MovieWriter`][crate::classes::MovieWriter]'s output file, or an empty string if the engine wasn't started in Movie Maker mode. The default path can be changed in \\[member ProjectSettings.editor/movie_writer/movie_file]."]
        pub fn get_write_movie_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(68usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_write_movie_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_print_to_stdout(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(69usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_print_to_stdout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_printing_to_stdout(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(70usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_printing_to_stdout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_print_error_messages(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(71usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_print_error_messages", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_printing_error_messages(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(72usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_printing_error_messages", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Engine {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Engine"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Core;
        
    }
    unsafe impl crate::obj::Bounds for Engine {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Engine {
        
    }
    impl crate::obj::Singleton for Engine {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"Engine"))
            }
        }
    }
    impl std::ops::Deref for Engine {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Engine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Engine__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Engine` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Engine::capture_script_backtraces_ex`][super::Engine::capture_script_backtraces_ex]."]
#[must_use]
pub struct ExCaptureScriptBacktraces < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Engine, include_variables: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCaptureScriptBacktraces < 'ex > {
    fn new(surround_object: &'ex re_export::Engine,) -> Self {
        let include_variables = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_variables: include_variables,
        }
    }
    #[inline]
    pub fn include_variables(self, include_variables: bool) -> Self {
        Self {
            include_variables: include_variables, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::ScriptBacktrace > > {
        let Self {
            _phantom, surround_object, include_variables,
        }
        = self;
        re_export::Engine::capture_script_backtraces_full(surround_object, include_variables,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Engine;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Engine {
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
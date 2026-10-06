#![doc = "Sidecar module for class [`EditorExportPlatform`][crate::classes::EditorExportPlatform].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorExportPlatform` enums](https://docs.godotengine.org/en/stable/classes/class_editorexportplatform.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorExportPlatform`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`editor_export_platform`][crate::classes::editor_export_platform]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `EditorExportPlatform`](https://docs.godotengine.org/en/stable/classes/class_editorexportplatform.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<EditorExportPlatform>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nBase resource that provides the functionality of exporting a release build of a project to a platform, from the editor. Stores platform-specific metadata such as the name and supported features of the platform, and performs the exporting of projects, PCK files, and ZIP files. Uses an export template for the platform provided at the time of project exporting.\n\nUsed in scripting by [`EditorExportPlugin`][crate::classes::EditorExportPlugin] to configure platform-specific customization of scenes and resources. See [`begin_customize_scenes`][`crate::classes::IEditorExportPlugin::begin_customize_scenes`] and [`begin_customize_resources`][`crate::classes::IEditorExportPlugin::begin_customize_resources`] for more details."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorExportPlatform {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl EditorExportPlatform {
        #[doc = "Returns the name of the export operating system handled by this `EditorExportPlatform` class, as a friendly string. Possible return values are `Windows`, `Linux`, `macOS`, `Android`, `iOS`, and `Web`."]
        pub fn get_os_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_os_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Create a new preset for this platform."]
        pub fn create_preset(&mut self,) -> Option < Gd < crate::classes::EditorExportPreset > > {
            type CallRet = Option < Gd < crate::classes::EditorExportPreset > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "create_preset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Locates export template for the platform, and returns [`Dictionary`][crate::builtin::Dictionary] with the following keys: `path: String` and `error: String`. This method is provided for convenience and custom export platforms aren't required to use it or keep export templates stored in the same way official templates are."]
        pub fn find_export_template(&self, template_file_name: impl AsArg < GString >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (template_file_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "find_export_template", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns array of [`EditorExportPreset`][crate::classes::EditorExportPreset]s for this platform."]
        pub fn get_current_presets(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_current_presets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves PCK archive and returns [`Dictionary`][crate::builtin::Dictionary] with the following keys: `result: Error`, `so_files: Array` (array of the shared/static objects which contains dictionaries with the following keys: `path: String`, `tags: PackedStringArray`, and `target_folder: String`).\n\nIf `embed` is `true`, PCK content is appended to the end of `path` file and return [`Dictionary`][crate::builtin::Dictionary] additionally include following keys: `embedded_start: int` (embedded PCK offset) and `embedded_size: int` (embedded PCK size)."]
        pub(crate) fn save_pack_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < GString >, embed: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >, bool,);
            let args = (preset, debug, path, embed,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "save_pack", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`save_pack_ex`][Self::save_pack_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Saves PCK archive and returns [`Dictionary`][crate::builtin::Dictionary] with the following keys: `result: Error`, `so_files: Array` (array of the shared/static objects which contains dictionaries with the following keys: `path: String`, `tags: PackedStringArray`, and `target_folder: String`).\n\nIf `embed` is `true`, PCK content is appended to the end of `path` file and return [`Dictionary`][crate::builtin::Dictionary] additionally include following keys: `embedded_start: int` (embedded PCK offset) and `embedded_size: int` (embedded PCK size)."]
        #[inline]
        pub fn save_pack(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> VarDictionary {
            self.save_pack_ex(preset, debug, path,) . done()
        }
        #[doc = "Saves PCK archive and returns [`Dictionary`][crate::builtin::Dictionary] with the following keys: `result: Error`, `so_files: Array` (array of the shared/static objects which contains dictionaries with the following keys: `path: String`, `tags: PackedStringArray`, and `target_folder: String`).\n\nIf `embed` is `true`, PCK content is appended to the end of `path` file and return [`Dictionary`][crate::builtin::Dictionary] additionally include following keys: `embedded_start: int` (embedded PCK offset) and `embedded_size: int` (embedded PCK size)."]
        #[inline]
        pub fn save_pack_ex < 'ex > (&'ex mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, path: impl AsArg < GString > + 'ex,) -> ExSavePack < 'ex > {
            ExSavePack::new(self, preset, debug, path,)
        }
        #[doc = "Saves ZIP archive and returns [`Dictionary`][crate::builtin::Dictionary] with the following keys: `result: Error`, `so_files: Array` (array of the shared/static objects which contains dictionaries with the following keys: `path: String`, `tags: PackedStringArray`, and `target_folder: String`)."]
        pub fn save_zip(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >,);
            let args = (preset.into_arg(), debug, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "save_zip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves patch PCK archive and returns [`Dictionary`][crate::builtin::Dictionary] with the following keys: `result: Error`, `so_files: Array` (array of the shared/static objects which contains dictionaries with the following keys: `path: String`, `tags: PackedStringArray`, and `target_folder: String`)."]
        pub fn save_pack_patch(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >,);
            let args = (preset.into_arg(), debug, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "save_pack_patch", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves patch ZIP archive and returns [`Dictionary`][crate::builtin::Dictionary] with the following keys: `result: Error`, `so_files: Array` (array of the shared/static objects which contains dictionaries with the following keys: `path: String`, `tags: PackedStringArray`, and `target_folder: String`)."]
        pub fn save_zip_patch(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >,);
            let args = (preset.into_arg(), debug, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "save_zip_patch", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Generates array of command line arguments for the default export templates for the debug flags and editor settings."]
        pub fn gen_export_flags(&mut self, flags: crate::classes::editor_export_platform::DebugFlags,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (crate::classes::editor_export_platform::DebugFlags,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "gen_export_flags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Exports project files for the specified preset. This method can be used to implement custom export format, other than PCK and ZIP. One of the callbacks is called for each exported file.\n\n`save_cb` is called for all exported files and have the following arguments: `file_path: String`, `file_data: PackedByteArray`, `file_index: int`, `file_count: int`, `encryption_include_filters: PackedStringArray`, `encryption_exclude_filters: PackedStringArray`, `encryption_key: PackedByteArray`.\n\n`shared_cb` is called for exported native shared/static libraries and have the following arguments: `file_path: String`, `tags: PackedStringArray`, `target_folder: String`.\n\n**Note:** `file_index` and `file_count` are intended for progress tracking only and aren't necessarily unique and precise."]
        pub(crate) fn export_project_files_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, save_cb: RefArg < Callable >, shared_cb: RefArg < Callable >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >,);
            let args = (preset, debug, save_cb, shared_cb,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "export_project_files", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`export_project_files_ex`][Self::export_project_files_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Exports project files for the specified preset. This method can be used to implement custom export format, other than PCK and ZIP. One of the callbacks is called for each exported file.\n\n`save_cb` is called for all exported files and have the following arguments: `file_path: String`, `file_data: PackedByteArray`, `file_index: int`, `file_count: int`, `encryption_include_filters: PackedStringArray`, `encryption_exclude_filters: PackedStringArray`, `encryption_key: PackedByteArray`.\n\n`shared_cb` is called for exported native shared/static libraries and have the following arguments: `file_path: String`, `tags: PackedStringArray`, `target_folder: String`.\n\n**Note:** `file_index` and `file_count` are intended for progress tracking only and aren't necessarily unique and precise."]
        #[inline]
        pub fn export_project_files(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, save_cb: &Callable,) -> crate::global::Error {
            self.export_project_files_ex(preset, debug, save_cb,) . done()
        }
        #[doc = "Exports project files for the specified preset. This method can be used to implement custom export format, other than PCK and ZIP. One of the callbacks is called for each exported file.\n\n`save_cb` is called for all exported files and have the following arguments: `file_path: String`, `file_data: PackedByteArray`, `file_index: int`, `file_count: int`, `encryption_include_filters: PackedStringArray`, `encryption_exclude_filters: PackedStringArray`, `encryption_key: PackedByteArray`.\n\n`shared_cb` is called for exported native shared/static libraries and have the following arguments: `file_path: String`, `tags: PackedStringArray`, `target_folder: String`.\n\n**Note:** `file_index` and `file_count` are intended for progress tracking only and aren't necessarily unique and precise."]
        #[inline]
        pub fn export_project_files_ex < 'ex > (&'ex mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, save_cb: &'ex Callable,) -> ExExportProjectFiles < 'ex > {
            ExExportProjectFiles::new(self, preset, debug, save_cb,)
        }
        #[doc = "Creates a full project at `path` for the specified `preset`."]
        pub(crate) fn export_project_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < GString >, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >, crate::classes::editor_export_platform::DebugFlags,);
            let args = (preset, debug, path, flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "export_project", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`export_project_ex`][Self::export_project_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a full project at `path` for the specified `preset`."]
        #[inline]
        pub fn export_project(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> crate::global::Error {
            self.export_project_ex(preset, debug, path,) . done()
        }
        #[doc = "Creates a full project at `path` for the specified `preset`."]
        #[inline]
        pub fn export_project_ex < 'ex > (&'ex mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, path: impl AsArg < GString > + 'ex,) -> ExExportProject < 'ex > {
            ExExportProject::new(self, preset, debug, path,)
        }
        #[doc = "Creates a PCK archive at `path` for the specified `preset`."]
        pub(crate) fn export_pack_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < GString >, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >, crate::classes::editor_export_platform::DebugFlags,);
            let args = (preset, debug, path, flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "export_pack", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`export_pack_ex`][Self::export_pack_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a PCK archive at `path` for the specified `preset`."]
        #[inline]
        pub fn export_pack(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> crate::global::Error {
            self.export_pack_ex(preset, debug, path,) . done()
        }
        #[doc = "Creates a PCK archive at `path` for the specified `preset`."]
        #[inline]
        pub fn export_pack_ex < 'ex > (&'ex mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, path: impl AsArg < GString > + 'ex,) -> ExExportPack < 'ex > {
            ExExportPack::new(self, preset, debug, path,)
        }
        #[doc = "Create a ZIP archive at `path` for the specified `preset`."]
        pub(crate) fn export_zip_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < GString >, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >, crate::classes::editor_export_platform::DebugFlags,);
            let args = (preset, debug, path, flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "export_zip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`export_zip_ex`][Self::export_zip_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Create a ZIP archive at `path` for the specified `preset`."]
        #[inline]
        pub fn export_zip(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> crate::global::Error {
            self.export_zip_ex(preset, debug, path,) . done()
        }
        #[doc = "Create a ZIP archive at `path` for the specified `preset`."]
        #[inline]
        pub fn export_zip_ex < 'ex > (&'ex mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, path: impl AsArg < GString > + 'ex,) -> ExExportZip < 'ex > {
            ExExportZip::new(self, preset, debug, path,)
        }
        #[doc = "Creates a patch PCK archive at `path` for the specified `preset`, containing only the files that have changed since the last patch.\n\n**Note:** `patches` is an optional override of the set of patches defined in the export preset. When empty the patches defined in the export preset will be used instead."]
        pub(crate) fn export_pack_patch_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < GString >, patches: RefArg < PackedStringArray >, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, crate::classes::editor_export_platform::DebugFlags,);
            let args = (preset, debug, path, patches, flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "export_pack_patch", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`export_pack_patch_ex`][Self::export_pack_patch_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a patch PCK archive at `path` for the specified `preset`, containing only the files that have changed since the last patch.\n\n**Note:** `patches` is an optional override of the set of patches defined in the export preset. When empty the patches defined in the export preset will be used instead."]
        #[inline]
        pub fn export_pack_patch(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> crate::global::Error {
            self.export_pack_patch_ex(preset, debug, path,) . done()
        }
        #[doc = "Creates a patch PCK archive at `path` for the specified `preset`, containing only the files that have changed since the last patch.\n\n**Note:** `patches` is an optional override of the set of patches defined in the export preset. When empty the patches defined in the export preset will be used instead."]
        #[inline]
        pub fn export_pack_patch_ex < 'ex > (&'ex mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, path: impl AsArg < GString > + 'ex,) -> ExExportPackPatch < 'ex > {
            ExExportPackPatch::new(self, preset, debug, path,)
        }
        #[doc = "Create a patch ZIP archive at `path` for the specified `preset`, containing only the files that have changed since the last patch.\n\n**Note:** `patches` is an optional override of the set of patches defined in the export preset. When empty the patches defined in the export preset will be used instead."]
        pub(crate) fn export_zip_patch_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < GString >, patches: RefArg < PackedStringArray >, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, crate::classes::editor_export_platform::DebugFlags,);
            let args = (preset, debug, path, patches, flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "export_zip_patch", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`export_zip_patch_ex`][Self::export_zip_patch_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Create a patch ZIP archive at `path` for the specified `preset`, containing only the files that have changed since the last patch.\n\n**Note:** `patches` is an optional override of the set of patches defined in the export preset. When empty the patches defined in the export preset will be used instead."]
        #[inline]
        pub fn export_zip_patch(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> crate::global::Error {
            self.export_zip_patch_ex(preset, debug, path,) . done()
        }
        #[doc = "Create a patch ZIP archive at `path` for the specified `preset`, containing only the files that have changed since the last patch.\n\n**Note:** `patches` is an optional override of the set of patches defined in the export preset. When empty the patches defined in the export preset will be used instead."]
        #[inline]
        pub fn export_zip_patch_ex < 'ex > (&'ex mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, path: impl AsArg < GString > + 'ex,) -> ExExportZipPatch < 'ex > {
            ExExportZipPatch::new(self, preset, debug, path,)
        }
        #[doc = "Clears the export log."]
        pub fn clear_messages(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "clear_messages", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a message to the export log that will be displayed when exporting ends."]
        pub fn add_message(&mut self, type_: crate::classes::editor_export_platform::ExportMessageType, category: impl AsArg < GString >, message: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (crate::classes::editor_export_platform::ExportMessageType, CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (type_, category.into_arg(), message.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "add_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of messages in the export log."]
        pub fn get_message_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_message_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the type for the message with the given `index`."]
        pub fn get_message_type(&self, index: i32,) -> crate::classes::editor_export_platform::ExportMessageType {
            type CallRet = crate::classes::editor_export_platform::ExportMessageType;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_message_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the message category for the message with the given `index`."]
        pub fn get_message_category(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_message_category", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text for the message with the given `index`."]
        pub fn get_message_text(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_message_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns most severe message type currently present in the export log."]
        pub fn get_worst_message_type(&self,) -> crate::classes::editor_export_platform::ExportMessageType {
            type CallRet = crate::classes::editor_export_platform::ExportMessageType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_worst_message_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Executes specified command on the remote host via SSH protocol and returns command output in the `output`."]
        pub(crate) fn ssh_run_on_remote_full(&self, host: CowArg < GString >, port: CowArg < GString >, ssh_arg: RefArg < PackedStringArray >, cmd_args: CowArg < GString >, output: RefArg < AnyArray >, port_fwd: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, CowArg < 'a3, GString >, RefArg < 'a4, AnyArray >, i32,);
            let args = (host, port, ssh_arg, cmd_args, output, port_fwd,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "ssh_run_on_remote", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`ssh_run_on_remote_ex`][Self::ssh_run_on_remote_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Executes specified command on the remote host via SSH protocol and returns command output in the `output`."]
        #[inline]
        pub fn ssh_run_on_remote(&self, host: impl AsArg < GString >, port: impl AsArg < GString >, ssh_arg: &PackedStringArray, cmd_args: impl AsArg < GString >,) -> crate::global::Error {
            self.ssh_run_on_remote_ex(host, port, ssh_arg, cmd_args,) . done()
        }
        #[doc = "Executes specified command on the remote host via SSH protocol and returns command output in the `output`."]
        #[inline]
        pub fn ssh_run_on_remote_ex < 'ex > (&'ex self, host: impl AsArg < GString > + 'ex, port: impl AsArg < GString > + 'ex, ssh_arg: &'ex PackedStringArray, cmd_args: impl AsArg < GString > + 'ex,) -> ExSshRunOnRemote < 'ex > {
            ExSshRunOnRemote::new(self, host, port, ssh_arg, cmd_args,)
        }
        #[doc = "Executes specified command on the remote host via SSH protocol and returns process ID (on the remote host) without waiting for command to finish."]
        pub(crate) fn ssh_run_on_remote_no_wait_full(&self, host: CowArg < GString >, port: CowArg < GString >, ssh_args: RefArg < PackedStringArray >, cmd_args: CowArg < GString >, port_fwd: i32,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, CowArg < 'a3, GString >, i32,);
            let args = (host, port, ssh_args, cmd_args, port_fwd,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "ssh_run_on_remote_no_wait", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`ssh_run_on_remote_no_wait_ex`][Self::ssh_run_on_remote_no_wait_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Executes specified command on the remote host via SSH protocol and returns process ID (on the remote host) without waiting for command to finish."]
        #[inline]
        pub fn ssh_run_on_remote_no_wait(&self, host: impl AsArg < GString >, port: impl AsArg < GString >, ssh_args: &PackedStringArray, cmd_args: impl AsArg < GString >,) -> i64 {
            self.ssh_run_on_remote_no_wait_ex(host, port, ssh_args, cmd_args,) . done()
        }
        #[doc = "Executes specified command on the remote host via SSH protocol and returns process ID (on the remote host) without waiting for command to finish."]
        #[inline]
        pub fn ssh_run_on_remote_no_wait_ex < 'ex > (&'ex self, host: impl AsArg < GString > + 'ex, port: impl AsArg < GString > + 'ex, ssh_args: &'ex PackedStringArray, cmd_args: impl AsArg < GString > + 'ex,) -> ExSshRunOnRemoteNoWait < 'ex > {
            ExSshRunOnRemoteNoWait::new(self, host, port, ssh_args, cmd_args,)
        }
        #[doc = "Uploads specified file over SCP protocol to the remote host."]
        pub fn ssh_push_to_remote(&self, host: impl AsArg < GString >, port: impl AsArg < GString >, scp_args: &PackedStringArray, src_file: impl AsArg < GString >, dst_file: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, CowArg < 'a3, GString >, CowArg < 'a4, GString >,);
            let args = (host.into_arg(), port.into_arg(), RefArg::new(scp_args), src_file.into_arg(), dst_file.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "ssh_push_to_remote", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns additional files that should always be exported regardless of preset configuration, and are not part of the project source. The returned [`Dictionary`][crate::builtin::Dictionary] contains filename keys ([`String`][crate::builtin::GString]) and their corresponding raw data ([`PackedByteArray`][crate::builtin::PackedByteArray])."]
        pub fn get_internal_export_files(&self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool,);
            let args = (preset.into_arg(), debug,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_internal_export_files", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns array of core file names that always should be exported regardless of preset config."]
        pub(crate) fn get_forced_export_files_full(preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >,);
            let args = (preset,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_forced_export_files", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_forced_export_files_ex`][Self::get_forced_export_files_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns array of core file names that always should be exported regardless of preset config."]
        #[inline]
        pub fn get_forced_export_files() -> PackedStringArray {
            Self::get_forced_export_files_ex() . done()
        }
        #[doc = "Returns array of core file names that always should be exported regardless of preset config."]
        #[inline]
        pub fn get_forced_export_files_ex < 'ex > () -> ExGetForcedExportFiles < 'ex > {
            ExGetForcedExportFiles::new()
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
    impl crate::obj::GodotClass for EditorExportPlatform {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorExportPlatform"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorExportPlatform {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorExportPlatform {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorExportPlatform {
        
    }
    impl std::ops::Deref for EditorExportPlatform {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorExportPlatform {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorExportPlatform__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `EditorExportPlatform` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::save_pack_ex`][super::EditorExportPlatform::save_pack_ex]."]
#[must_use]
pub struct ExSavePack < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorExportPlatform, preset: CowArg < 'ex, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < 'ex, GString >, embed: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSavePack < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, path: impl AsArg < GString > + 'ex,) -> Self {
        let embed = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, path: path.into_arg(), embed: embed,
        }
    }
    #[inline]
    pub fn embed(self, embed: bool) -> Self {
        Self {
            embed: embed, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarDictionary {
        let Self {
            _phantom, surround_object, preset, debug, path, embed,
        }
        = self;
        re_export::EditorExportPlatform::save_pack_full(surround_object, preset, debug, path, embed,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::export_project_files_ex`][super::EditorExportPlatform::export_project_files_ex]."]
#[must_use]
pub struct ExExportProjectFiles < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorExportPlatform, preset: CowArg < 'ex, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, save_cb: CowArg < 'ex, Callable >, shared_cb: CowArg < 'ex, Callable >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExExportProjectFiles < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, save_cb: &'ex Callable,) -> Self {
        let shared_cb = Callable::invalid();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, save_cb: CowArg::Borrowed(save_cb), shared_cb: CowArg::Owned(shared_cb),
        }
    }
    #[inline]
    pub fn shared_cb(self, shared_cb: &'ex Callable) -> Self {
        Self {
            shared_cb: CowArg::Borrowed(shared_cb), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, preset, debug, save_cb, shared_cb,
        }
        = self;
        re_export::EditorExportPlatform::export_project_files_full(surround_object, preset, debug, save_cb.cow_as_arg(), shared_cb.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::export_project_ex`][super::EditorExportPlatform::export_project_ex]."]
#[must_use]
pub struct ExExportProject < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorExportPlatform, preset: CowArg < 'ex, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < 'ex, GString >, flags: crate::classes::editor_export_platform::DebugFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExExportProject < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, path: impl AsArg < GString > + 'ex,) -> Self {
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, path: path.into_arg(), flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::editor_export_platform::DebugFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, preset, debug, path, flags,
        }
        = self;
        re_export::EditorExportPlatform::export_project_full(surround_object, preset, debug, path, flags,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::export_pack_ex`][super::EditorExportPlatform::export_pack_ex]."]
#[must_use]
pub struct ExExportPack < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorExportPlatform, preset: CowArg < 'ex, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < 'ex, GString >, flags: crate::classes::editor_export_platform::DebugFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExExportPack < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, path: impl AsArg < GString > + 'ex,) -> Self {
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, path: path.into_arg(), flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::editor_export_platform::DebugFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, preset, debug, path, flags,
        }
        = self;
        re_export::EditorExportPlatform::export_pack_full(surround_object, preset, debug, path, flags,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::export_zip_ex`][super::EditorExportPlatform::export_zip_ex]."]
#[must_use]
pub struct ExExportZip < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorExportPlatform, preset: CowArg < 'ex, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < 'ex, GString >, flags: crate::classes::editor_export_platform::DebugFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExExportZip < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, path: impl AsArg < GString > + 'ex,) -> Self {
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, path: path.into_arg(), flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::editor_export_platform::DebugFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, preset, debug, path, flags,
        }
        = self;
        re_export::EditorExportPlatform::export_zip_full(surround_object, preset, debug, path, flags,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::export_pack_patch_ex`][super::EditorExportPlatform::export_pack_patch_ex]."]
#[must_use]
pub struct ExExportPackPatch < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorExportPlatform, preset: CowArg < 'ex, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < 'ex, GString >, patches: CowArg < 'ex, PackedStringArray >, flags: crate::classes::editor_export_platform::DebugFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExExportPackPatch < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, path: impl AsArg < GString > + 'ex,) -> Self {
        let patches = PackedStringArray::new();
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, path: path.into_arg(), patches: CowArg::Owned(patches), flags: flags,
        }
    }
    #[inline]
    pub fn patches(self, patches: &'ex PackedStringArray) -> Self {
        Self {
            patches: CowArg::Borrowed(patches), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::editor_export_platform::DebugFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, preset, debug, path, patches, flags,
        }
        = self;
        re_export::EditorExportPlatform::export_pack_patch_full(surround_object, preset, debug, path, patches.cow_as_arg(), flags,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::export_zip_patch_ex`][super::EditorExportPlatform::export_zip_patch_ex]."]
#[must_use]
pub struct ExExportZipPatch < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorExportPlatform, preset: CowArg < 'ex, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < 'ex, GString >, patches: CowArg < 'ex, PackedStringArray >, flags: crate::classes::editor_export_platform::DebugFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExExportZipPatch < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex, debug: bool, path: impl AsArg < GString > + 'ex,) -> Self {
        let patches = PackedStringArray::new();
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, path: path.into_arg(), patches: CowArg::Owned(patches), flags: flags,
        }
    }
    #[inline]
    pub fn patches(self, patches: &'ex PackedStringArray) -> Self {
        Self {
            patches: CowArg::Borrowed(patches), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::editor_export_platform::DebugFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, preset, debug, path, patches, flags,
        }
        = self;
        re_export::EditorExportPlatform::export_zip_patch_full(surround_object, preset, debug, path, patches.cow_as_arg(), flags,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::ssh_run_on_remote_ex`][super::EditorExportPlatform::ssh_run_on_remote_ex]."]
#[must_use]
pub struct ExSshRunOnRemote < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::EditorExportPlatform, host: CowArg < 'ex, GString >, port: CowArg < 'ex, GString >, ssh_arg: CowArg < 'ex, PackedStringArray >, cmd_args: CowArg < 'ex, GString >, output: CowArg < 'ex, AnyArray >, port_fwd: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSshRunOnRemote < 'ex > {
    fn new(surround_object: &'ex re_export::EditorExportPlatform, host: impl AsArg < GString > + 'ex, port: impl AsArg < GString > + 'ex, ssh_arg: &'ex PackedStringArray, cmd_args: impl AsArg < GString > + 'ex,) -> Self {
        let output = AnyArray::new_untyped();
        let port_fwd = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), port: port.into_arg(), ssh_arg: CowArg::Borrowed(ssh_arg), cmd_args: cmd_args.into_arg(), output: CowArg::Owned(output), port_fwd: port_fwd,
        }
    }
    #[inline]
    pub fn output(self, output: &'ex AnyArray) -> Self {
        Self {
            output: CowArg::Borrowed(output), .. self
        }
    }
    #[inline]
    pub fn port_fwd(self, port_fwd: i32) -> Self {
        Self {
            port_fwd: port_fwd, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, host, port, ssh_arg, cmd_args, output, port_fwd,
        }
        = self;
        re_export::EditorExportPlatform::ssh_run_on_remote_full(surround_object, host, port, ssh_arg.cow_as_arg(), cmd_args, output.cow_as_arg(), port_fwd,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::ssh_run_on_remote_no_wait_ex`][super::EditorExportPlatform::ssh_run_on_remote_no_wait_ex]."]
#[must_use]
pub struct ExSshRunOnRemoteNoWait < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::EditorExportPlatform, host: CowArg < 'ex, GString >, port: CowArg < 'ex, GString >, ssh_args: CowArg < 'ex, PackedStringArray >, cmd_args: CowArg < 'ex, GString >, port_fwd: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSshRunOnRemoteNoWait < 'ex > {
    fn new(surround_object: &'ex re_export::EditorExportPlatform, host: impl AsArg < GString > + 'ex, port: impl AsArg < GString > + 'ex, ssh_args: &'ex PackedStringArray, cmd_args: impl AsArg < GString > + 'ex,) -> Self {
        let port_fwd = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), port: port.into_arg(), ssh_args: CowArg::Borrowed(ssh_args), cmd_args: cmd_args.into_arg(), port_fwd: port_fwd,
        }
    }
    #[inline]
    pub fn port_fwd(self, port_fwd: i32) -> Self {
        Self {
            port_fwd: port_fwd, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, host, port, ssh_args, cmd_args, port_fwd,
        }
        = self;
        re_export::EditorExportPlatform::ssh_run_on_remote_no_wait_full(surround_object, host, port, ssh_args.cow_as_arg(), cmd_args, port_fwd,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::get_forced_export_files_ex`][super::EditorExportPlatform::get_forced_export_files_ex]."]
#[must_use]
pub struct ExGetForcedExportFiles < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, preset: CowArg < 'ex, Option < Gd < crate::classes::EditorExportPreset > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetForcedExportFiles < 'ex > {
    fn new() -> Self {
        let preset = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, preset: preset.into_arg(),
        }
    }
    #[inline]
    pub fn preset(self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'ex) -> Self {
        Self {
            preset: preset.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, preset,
        }
        = self;
        re_export::EditorExportPlatform::get_forced_export_files_full(preset,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ExportMessageType {
    ord: i32
}
impl ExportMessageType {
    #[doc(alias = "EXPORT_MESSAGE_NONE")]
    #[doc = "Godot enumerator name: `EXPORT_MESSAGE_NONE`"]
    pub const NONE: ExportMessageType = ExportMessageType {
        ord: 0i32
    };
    #[doc(alias = "EXPORT_MESSAGE_INFO")]
    #[doc = "Godot enumerator name: `EXPORT_MESSAGE_INFO`"]
    pub const INFO: ExportMessageType = ExportMessageType {
        ord: 1i32
    };
    #[doc(alias = "EXPORT_MESSAGE_WARNING")]
    #[doc = "Godot enumerator name: `EXPORT_MESSAGE_WARNING`"]
    pub const WARNING: ExportMessageType = ExportMessageType {
        ord: 2i32
    };
    #[doc(alias = "EXPORT_MESSAGE_ERROR")]
    #[doc = "Godot enumerator name: `EXPORT_MESSAGE_ERROR`"]
    pub const ERROR: ExportMessageType = ExportMessageType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ExportMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ExportMessageType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ExportMessageType {
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
            Self::NONE => "NONE", Self::INFO => "INFO", Self::WARNING => "WARNING", Self::ERROR => "ERROR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ExportMessageType::NONE, ExportMessageType::INFO, ExportMessageType::WARNING, ExportMessageType::ERROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ExportMessageType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "EXPORT_MESSAGE_NONE", ExportMessageType::NONE), crate::meta::inspect::EnumConstant::new("INFO", "EXPORT_MESSAGE_INFO", ExportMessageType::INFO), crate::meta::inspect::EnumConstant::new("WARNING", "EXPORT_MESSAGE_WARNING", ExportMessageType::WARNING), crate::meta::inspect::EnumConstant::new("ERROR", "EXPORT_MESSAGE_ERROR", ExportMessageType::ERROR)]
        }
    }
}
impl crate::meta::GodotConvert for ExportMessageType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Export Message None", 0i64), EnumeratorShape::new_int("Export Message Info", 1i64), EnumeratorShape::new_int("Export Message Warning", 2i64), EnumeratorShape::new_int("Export Message Error", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("EditorExportPlatform.ExportMessageType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ExportMessageType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ExportMessageType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ExportMessageType {
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
impl crate::registry::property::Export for ExportMessageType {
    
}
impl crate::meta::Element for ExportMessageType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct DebugFlags {
    ord: u64
}
impl DebugFlags {
    #[doc(alias = "DEBUG_FLAG_DUMB_CLIENT")]
    #[doc = "Godot enumerator name: `DEBUG_FLAG_DUMB_CLIENT`"]
    pub const DUMB_CLIENT: DebugFlags = DebugFlags {
        ord: 1u64
    };
    #[doc(alias = "DEBUG_FLAG_REMOTE_DEBUG")]
    #[doc = "Godot enumerator name: `DEBUG_FLAG_REMOTE_DEBUG`"]
    pub const REMOTE_DEBUG: DebugFlags = DebugFlags {
        ord: 2u64
    };
    #[doc(alias = "DEBUG_FLAG_REMOTE_DEBUG_LOCALHOST")]
    #[doc = "Godot enumerator name: `DEBUG_FLAG_REMOTE_DEBUG_LOCALHOST`"]
    pub const REMOTE_DEBUG_LOCALHOST: DebugFlags = DebugFlags {
        ord: 4u64
    };
    #[doc(alias = "DEBUG_FLAG_VIEW_COLLISIONS")]
    #[doc = "Godot enumerator name: `DEBUG_FLAG_VIEW_COLLISIONS`"]
    pub const VIEW_COLLISIONS: DebugFlags = DebugFlags {
        ord: 8u64
    };
    #[doc(alias = "DEBUG_FLAG_VIEW_NAVIGATION")]
    #[doc = "Godot enumerator name: `DEBUG_FLAG_VIEW_NAVIGATION`"]
    pub const VIEW_NAVIGATION: DebugFlags = DebugFlags {
        ord: 16u64
    };
    
}
impl std::fmt::Debug for DebugFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for DebugFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DebugFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DUMB_CLIENT", "DEBUG_FLAG_DUMB_CLIENT", DebugFlags::DUMB_CLIENT), crate::meta::inspect::EnumConstant::new("REMOTE_DEBUG", "DEBUG_FLAG_REMOTE_DEBUG", DebugFlags::REMOTE_DEBUG), crate::meta::inspect::EnumConstant::new("REMOTE_DEBUG_LOCALHOST", "DEBUG_FLAG_REMOTE_DEBUG_LOCALHOST", DebugFlags::REMOTE_DEBUG_LOCALHOST), crate::meta::inspect::EnumConstant::new("VIEW_COLLISIONS", "DEBUG_FLAG_VIEW_COLLISIONS", DebugFlags::VIEW_COLLISIONS), crate::meta::inspect::EnumConstant::new("VIEW_NAVIGATION", "DEBUG_FLAG_VIEW_NAVIGATION", DebugFlags::VIEW_NAVIGATION)]
        }
    }
}
impl std::ops::BitOr for DebugFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for DebugFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for DebugFlags {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Debug Flag Dumb Client", 1i64), EnumeratorShape::new_int("Debug Flag Remote Debug", 2i64), EnumeratorShape::new_int("Debug Flag Remote Debug Localhost", 4i64), EnumeratorShape::new_int("Debug Flag View Collisions", 8i64), EnumeratorShape::new_int("Debug Flag View Navigation", 16i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("EditorExportPlatform.DebugFlags")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for DebugFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DebugFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DebugFlags {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* field)
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
impl crate::registry::property::Export for DebugFlags {
    
}
impl crate::meta::Element for DebugFlags {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorExportPlatform;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorExportPlatform {
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
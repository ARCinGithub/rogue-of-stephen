#![doc = "Sidecar module for class [`DirAccess`][crate::classes::DirAccess].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `DirAccess` enums](https://docs.godotengine.org/en/stable/classes/class_diraccess.html#enumerations).\n\n"]
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
    #[doc = "Godot class `DirAccess`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`dir_access`][crate::classes::dir_access]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `DirAccess`](https://docs.godotengine.org/en/stable/classes/class_diraccess.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<DirAccess>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThis class is used to manage directories and their content, even outside of the project folder.\n\n`DirAccess` can't be instantiated directly. Instead it is created with a static method that takes a path for which it will be opened.\n\nMost of the methods have a static alternative that can be used without creating a `DirAccess`. Static methods only support absolute paths (including `res://` and `user://`).\n\n```gdscript\n# Standard\nvar dir = DirAccess.open(\"user://levels\")\ndir.make_dir(\"world1\")\n# Static\nDirAccess.make_dir_absolute(\"user://levels/world1\")\n```\n\n**Note:** Accessing project (\"res://\") directories once exported may behave unexpectedly as some files are converted to engine-specific formats and their original source files may not be present in the expected PCK package. Because of this, to access resources in an exported project, it is recommended to use [`ResourceLoader`][crate::classes::ResourceLoader] instead of [`FileAccess`][crate::classes::FileAccess].\n\nHere is an example on how to iterate through the files of a directory:\n\n\n```gdscript\nfunc dir_contents(path):\n\tvar dir = DirAccess.open(path)\n\tif dir:\n\t\tdir.list_dir_begin()\n\t\tvar file_name = dir.get_next()\n\t\twhile file_name != \"\":\n\t\t\tif dir.current_is_dir():\n\t\t\t\tprint(\"Found directory: \" + file_name)\n\t\t\telse:\n\t\t\t\tprint(\"Found file: \" + file_name)\n\t\t\tfile_name = dir.get_next()\n\telse:\n\t\tprint(\"An error occurred when trying to access the path.\")\n```\n\n\nKeep in mind that file names may change or be remapped after export. If you want to see the actual resource file list as it appears in the editor, use [`list_directory`][`crate::classes::ResourceLoader::list_directory`] instead."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct DirAccess {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl DirAccess {
        #[doc = "Creates a new `DirAccess` object and opens an existing directory of the filesystem. The `path` argument can be within the project tree (`res://folder`), the user directory (`user://folder`) or an absolute path of the user filesystem (e.g. `/tmp/folder` or `C:\\tmp\\folder`).\n\nReturns `null` if opening the directory failed. You can use [`get_open_error`][`crate::classes::DirAccess::get_open_error`] to check the error that occurred."]
        pub fn open(path: impl AsArg < GString >,) -> Option < Gd < crate::classes::DirAccess > > {
            type CallRet = Option < Gd < crate::classes::DirAccess > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "open", None, args,)
            }
        }
        #[doc = "Returns the result of the last [`open`][`crate::classes::DirAccess::open`] call in the current thread."]
        pub fn get_open_error() -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_open_error", None, args,)
            }
        }
        #[doc = "Creates a temporary directory. This directory will be freed when the returned `DirAccess` is freed.\n\nIf `prefix` is not empty, it will be prefixed to the directory name, separated by a `-`.\n\nIf `keep` is `true`, the directory is not deleted when the returned `DirAccess` is freed.\n\nReturns `null` if opening the directory failed. You can use [`get_open_error`][`crate::classes::DirAccess::get_open_error`] to check the error that occurred."]
        pub(crate) fn create_temp_full(prefix: CowArg < GString >, keep: bool,) -> Option < Gd < crate::classes::DirAccess > > {
            type CallRet = Option < Gd < crate::classes::DirAccess > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (prefix, keep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "create_temp", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_temp_ex`][Self::create_temp_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a temporary directory. This directory will be freed when the returned `DirAccess` is freed.\n\nIf `prefix` is not empty, it will be prefixed to the directory name, separated by a `-`.\n\nIf `keep` is `true`, the directory is not deleted when the returned `DirAccess` is freed.\n\nReturns `null` if opening the directory failed. You can use [`get_open_error`][`crate::classes::DirAccess::get_open_error`] to check the error that occurred."]
        #[inline]
        pub fn create_temp() -> Option < Gd < crate::classes::DirAccess > > {
            Self::create_temp_ex() . done()
        }
        #[doc = "Creates a temporary directory. This directory will be freed when the returned `DirAccess` is freed.\n\nIf `prefix` is not empty, it will be prefixed to the directory name, separated by a `-`.\n\nIf `keep` is `true`, the directory is not deleted when the returned `DirAccess` is freed.\n\nReturns `null` if opening the directory failed. You can use [`get_open_error`][`crate::classes::DirAccess::get_open_error`] to check the error that occurred."]
        #[inline]
        pub fn create_temp_ex < 'ex > () -> ExCreateTemp < 'ex > {
            ExCreateTemp::new()
        }
        #[doc = "Initializes the stream used to list all files and directories using the [`get_next`][`crate::classes::DirAccess::get_next`] function, closing the currently opened stream if needed. Once the stream has been processed, it should typically be closed with [`list_dir_end`][`crate::classes::DirAccess::list_dir_end`].\n\nAffected by \\[member include_hidden] and \\[member include_navigational].\n\n**Note:** The order of files and directories returned by this method is not deterministic, and can vary between operating systems. If you want a list of all files or folders sorted alphabetically, use [`get_files`][`crate::classes::DirAccess::get_files`] or [`get_directories`][`crate::classes::DirAccess::get_directories`]."]
        pub fn list_dir_begin(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "list_dir_begin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next element (file or directory) in the current directory.\n\nThe name of the file or directory is returned (and not its full path). Once the stream has been fully processed, the method returns an empty [`String`][crate::builtin::GString] and closes the stream automatically (i.e. [`list_dir_end`][`crate::classes::DirAccess::list_dir_end`] would not be mandatory in such a case)."]
        pub fn get_next(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_next", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether the current item processed with the last [`get_next`][`crate::classes::DirAccess::get_next`] call is a directory (`.` and `..` are considered directories)."]
        pub fn current_is_dir(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "current_is_dir", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Closes the current stream opened with [`list_dir_begin`][`crate::classes::DirAccess::list_dir_begin`] (whether it has been fully processed with [`get_next`][`crate::classes::DirAccess::get_next`] does not matter)."]
        pub fn list_dir_end(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "list_dir_end", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`PackedStringArray`][crate::builtin::PackedStringArray] containing filenames of the directory contents, excluding directories. The array is sorted alphabetically.\n\nAffected by \\[member include_hidden].\n\n**Note:** When used on a `res://` path in an exported project, only the files actually included in the PCK at the given folder level are returned. In practice, this means that since imported resources are stored in a top-level `.godot/` folder, only paths to `*.gd` and `*.import` files are returned (plus a few files such as `project.godot` or `project.binary` and the project icon). In an exported project, the list of returned files will also vary depending on whether \\[member ProjectSettings.editor/export/convert_text_resources_to_binary] is `true`."]
        pub fn get_files(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_files", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`PackedStringArray`][crate::builtin::PackedStringArray] containing filenames of the directory contents, excluding directories, at the given `path`. The array is sorted alphabetically.\n\nUse [`get_files`][`crate::classes::DirAccess::get_files`] if you want more control of what gets included.\n\n**Note:** When used on a `res://` path in an exported project, only the files included in the PCK at the given folder level are returned. In practice, this means that since imported resources are stored in a top-level `.godot/` folder, only paths to `.gd` and `.import` files are returned (plus a few other files, such as `project.godot` or `project.binary` and the project icon). In an exported project, the list of returned files will also vary depending on \\[member ProjectSettings.editor/export/convert_text_resources_to_binary]."]
        pub fn get_files_at(path: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_files_at", None, args,)
            }
        }
        #[doc = "Returns a [`PackedStringArray`][crate::builtin::PackedStringArray] containing filenames of the directory contents, excluding files. The array is sorted alphabetically.\n\nAffected by \\[member include_hidden] and \\[member include_navigational].\n\n**Note:** The returned directories in the editor and after exporting in the `res://` directory may differ as some files are converted to engine-specific formats when exported."]
        pub fn get_directories(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_directories", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`PackedStringArray`][crate::builtin::PackedStringArray] containing filenames of the directory contents, excluding files, at the given `path`. The array is sorted alphabetically.\n\nUse [`get_directories`][`crate::classes::DirAccess::get_directories`] if you want more control of what gets included.\n\n**Note:** The returned directories in the editor and after exporting in the `res://` directory may differ as some files are converted to engine-specific formats when exported."]
        pub fn get_directories_at(path: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11040usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_directories_at", None, args,)
            }
        }
        #[doc = "On Windows, returns the number of drives (partitions) mounted on the current filesystem.\n\nOn macOS and Android, returns the number of mounted volumes.\n\nOn Linux, returns the number of mounted volumes and GTK 3 bookmarks.\n\nOn other platforms, the method returns 0."]
        pub fn get_drive_count() -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11041usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_drive_count", None, args,)
            }
        }
        #[doc = "On Windows, returns the name of the drive (partition) passed as an argument (e.g. `C:`).\n\nOn macOS, returns the path to the mounted volume passed as an argument.\n\nOn Linux, returns the path to the mounted volume or GTK 3 bookmark passed as an argument.\n\nOn Android (API level 30+), returns the path to the mounted volume as an argument.\n\nOn other platforms, or if the requested drive does not exist, the method returns an empty String."]
        pub fn get_drive_name(idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11042usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_drive_name", None, args,)
            }
        }
        #[doc = "Returns the currently opened directory's drive index. See [`get_drive_name`][`crate::classes::DirAccess::get_drive_name`] to convert returned index to the name of the drive."]
        pub fn get_current_drive(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11043usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_current_drive", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Changes the currently opened directory to the one passed as an argument. The argument can be relative to the current directory (e.g. `newdir` or `../newdir`), or an absolute path (e.g. `/tmp/newdir` or `res://somedir/newdir`).\n\nReturns one of the \\[enum Error] code constants ([`Error::OK`][`crate::global::Error::OK`] on success).\n\n**Note:** The new directory must be within the same scope, e.g. when you had opened a directory inside `res://`, you can't change it to `user://` directory. If you need to open a directory in another access scope, use [`open`][`crate::classes::DirAccess::open`] to create a new instance instead."]
        pub fn change_dir(&mut self, to_dir: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (to_dir.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11044usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "change_dir", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the absolute path to the currently opened directory (e.g. `res://folder` or `C:\\tmp\\folder`)."]
        pub(crate) fn get_current_dir_full(&self, include_drive: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (bool,);
            let args = (include_drive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11045usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_current_dir", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_current_dir_ex`][Self::get_current_dir_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the absolute path to the currently opened directory (e.g. `res://folder` or `C:\\tmp\\folder`)."]
        #[inline]
        pub fn get_current_dir(&self,) -> GString {
            self.get_current_dir_ex() . done()
        }
        #[doc = "Returns the absolute path to the currently opened directory (e.g. `res://folder` or `C:\\tmp\\folder`)."]
        #[inline]
        pub fn get_current_dir_ex < 'ex > (&'ex self,) -> ExGetCurrentDir < 'ex > {
            ExGetCurrentDir::new(self,)
        }
        #[doc = "Creates a directory. The argument can be relative to the current directory, or an absolute path. The target directory should be placed in an already existing directory (to create the full path recursively, see [`make_dir_recursive`][`crate::classes::DirAccess::make_dir_recursive`]).\n\nReturns one of the \\[enum Error] code constants ([`Error::OK`][`crate::global::Error::OK`] on success)."]
        pub fn make_dir(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11046usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "make_dir", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Static version of [`make_dir`][`crate::classes::DirAccess::make_dir`]. Supports only absolute paths."]
        pub fn make_dir_absolute(path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11047usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "make_dir_absolute", None, args,)
            }
        }
        #[doc = "Creates a target directory and all necessary intermediate directories in its path, by calling [`make_dir`][`crate::classes::DirAccess::make_dir`] recursively. The argument can be relative to the current directory, or an absolute path.\n\nReturns one of the \\[enum Error] code constants ([`Error::OK`][`crate::global::Error::OK`] on success)."]
        pub fn make_dir_recursive(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11048usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "make_dir_recursive", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Static version of [`make_dir_recursive`][`crate::classes::DirAccess::make_dir_recursive`]. Supports only absolute paths."]
        pub fn make_dir_recursive_absolute(path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11049usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "make_dir_recursive_absolute", None, args,)
            }
        }
        #[doc = "Returns whether the target file exists. The argument can be relative to the current directory, or an absolute path.\n\nFor a static equivalent, use [`file_exists`][`crate::classes::FileAccess::file_exists`].\n\n**Note:** Many resources types are imported (e.g. textures or sound files), and their source asset will not be included in the exported game, as only the imported version is used. See [`exists`][`crate::classes::ResourceLoader::exists`] for an alternative approach that takes resource remapping into account."]
        pub fn file_exists(&mut self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11050usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "file_exists", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether the target directory exists. The argument can be relative to the current directory, or an absolute path.\n\n**Note:** The returned `bool` in the editor and after exporting when used on a path in the `res://` directory may be different. Some files are converted to engine-specific formats when exported, potentially changing the directory structure."]
        pub fn dir_exists(&mut self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11051usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "dir_exists", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Static version of [`dir_exists`][`crate::classes::DirAccess::dir_exists`]. Supports only absolute paths.\n\n**Note:** The returned `bool` in the editor and after exporting when used on a path in the `res://` directory may be different. Some files are converted to engine-specific formats when exported, potentially changing the directory structure."]
        pub fn dir_exists_absolute(path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11052usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "dir_exists_absolute", None, args,)
            }
        }
        #[doc = "Returns the available space on the current directory's disk, in bytes. Returns `0` if the platform-specific method to query the available space fails."]
        pub fn get_space_left(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11053usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_space_left", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Copies the `from` file to the `to` destination. Both arguments should be paths to files, either relative or absolute. If the destination file exists and is not access-protected, it will be overwritten.\n\nIf `chmod_flags` is different than `-1`, the Unix permissions for the destination path will be set to the provided value, if available on the current operating system.\n\nReturns one of the \\[enum Error] code constants ([`Error::OK`][`crate::global::Error::OK`] on success)."]
        pub(crate) fn copy_full(&mut self, from: CowArg < GString >, to: CowArg < GString >, chmod_flags: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32,);
            let args = (from, to, chmod_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11054usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "copy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`copy_ex`][Self::copy_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Copies the `from` file to the `to` destination. Both arguments should be paths to files, either relative or absolute. If the destination file exists and is not access-protected, it will be overwritten.\n\nIf `chmod_flags` is different than `-1`, the Unix permissions for the destination path will be set to the provided value, if available on the current operating system.\n\nReturns one of the \\[enum Error] code constants ([`Error::OK`][`crate::global::Error::OK`] on success)."]
        #[inline]
        pub fn copy(&mut self, from: impl AsArg < GString >, to: impl AsArg < GString >,) -> crate::global::Error {
            self.copy_ex(from, to,) . done()
        }
        #[doc = "Copies the `from` file to the `to` destination. Both arguments should be paths to files, either relative or absolute. If the destination file exists and is not access-protected, it will be overwritten.\n\nIf `chmod_flags` is different than `-1`, the Unix permissions for the destination path will be set to the provided value, if available on the current operating system.\n\nReturns one of the \\[enum Error] code constants ([`Error::OK`][`crate::global::Error::OK`] on success)."]
        #[inline]
        pub fn copy_ex < 'ex > (&'ex mut self, from: impl AsArg < GString > + 'ex, to: impl AsArg < GString > + 'ex,) -> ExCopy < 'ex > {
            ExCopy::new(self, from, to,)
        }
        #[doc = "Static version of [`copy`][`crate::classes::DirAccess::copy`]. Supports only absolute paths."]
        pub(crate) fn copy_absolute_full(from: CowArg < GString >, to: CowArg < GString >, chmod_flags: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32,);
            let args = (from, to, chmod_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11055usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "copy_absolute", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`copy_absolute_ex`][Self::copy_absolute_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Static version of [`copy`][`crate::classes::DirAccess::copy`]. Supports only absolute paths."]
        #[inline]
        pub fn copy_absolute(from: impl AsArg < GString >, to: impl AsArg < GString >,) -> crate::global::Error {
            Self::copy_absolute_ex(from, to,) . done()
        }
        #[doc = "Static version of [`copy`][`crate::classes::DirAccess::copy`]. Supports only absolute paths."]
        #[inline]
        pub fn copy_absolute_ex < 'ex > (from: impl AsArg < GString > + 'ex, to: impl AsArg < GString > + 'ex,) -> ExCopyAbsolute < 'ex > {
            ExCopyAbsolute::new(from, to,)
        }
        #[doc = "Renames (move) the `from` file or directory to the `to` destination. Both arguments should be paths to files or directories, either relative or absolute. If the destination file or directory exists and is not access-protected, it will be overwritten.\n\nReturns one of the \\[enum Error] code constants ([`Error::OK`][`crate::global::Error::OK`] on success)."]
        pub fn rename(&mut self, from: impl AsArg < GString >, to: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (from.into_arg(), to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11056usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "rename", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Static version of [`rename`][`crate::classes::DirAccess::rename`]. Supports only absolute paths."]
        pub fn rename_absolute(from: impl AsArg < GString >, to: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (from.into_arg(), to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11057usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "rename_absolute", None, args,)
            }
        }
        #[doc = "Permanently deletes the target file or an empty directory. The argument can be relative to the current directory, or an absolute path. If the target directory is not empty, the operation will fail.\n\nIf you don't want to delete the file/directory permanently, use [`move_to_trash`][`crate::classes::Os::move_to_trash`] instead.\n\nReturns one of the \\[enum Error] code constants ([`Error::OK`][`crate::global::Error::OK`] on success)."]
        pub fn remove(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11058usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "remove", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Static version of [`remove`][`crate::classes::DirAccess::remove`]. Supports only absolute paths."]
        pub fn remove_absolute(path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11059usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "remove_absolute", None, args,)
            }
        }
        #[doc = "Returns `true` if the file or directory is a symbolic link, directory junction, or other reparse point.\n\n**Note:** This method is implemented on macOS, Linux, and Windows."]
        pub fn is_link(&self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11060usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "is_link", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns target of the symbolic link.\n\n**Note:** This method is implemented on macOS, Linux, and Windows."]
        pub fn read_link(&mut self, path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11061usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "read_link", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates symbolic link between files or folders.\n\n**Note:** On Windows, this method works only if the application is running with elevated privileges or Developer Mode is enabled.\n\n**Note:** This method is implemented on macOS, Linux, and Windows."]
        pub fn create_link(&mut self, source: impl AsArg < GString >, target: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (source.into_arg(), target.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "create_link", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the directory is a macOS bundle.\n\n**Note:** This method is implemented on macOS."]
        pub fn is_bundle(&self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11063usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "is_bundle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_include_navigational(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11064usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "set_include_navigational", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_include_navigational(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_include_navigational", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_include_hidden(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11066usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "set_include_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_include_hidden(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11067usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_include_hidden", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns file system type name of the current directory's disk. Returned values are uppercase strings like `NTFS`, `FAT32`, `EXFAT`, `APFS`, `EXT4`, `BTRFS`, and so on.\n\n**Note:** This method is implemented on macOS, Linux, Windows and for PCK virtual file system."]
        pub fn get_filesystem_type(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_filesystem_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the file system or directory use case sensitive file names.\n\n**Note:** This method is implemented on macOS, Linux (for EXT4 and F2FS filesystems only) and Windows. On other platforms, it always returns `true`."]
        pub fn is_case_sensitive(&self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "is_case_sensitive", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if paths `path_a` and `path_b` resolve to the same file system object. Returns `false` otherwise, even if the files are bit-for-bit identical (e.g., identical copies of the file that are not symbolic links)."]
        pub fn is_equivalent(&self, path_a: impl AsArg < GString >, path_b: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (path_a.into_arg(), path_b.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "is_equivalent", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for DirAccess {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("DirAccess"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for DirAccess {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for DirAccess {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for DirAccess {
        
    }
    impl std::ops::Deref for DirAccess {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for DirAccess {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_DirAccess__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `DirAccess` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`DirAccess::create_temp_ex`][super::DirAccess::create_temp_ex]."]
#[must_use]
pub struct ExCreateTemp < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, prefix: CowArg < 'ex, GString >, keep: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateTemp < 'ex > {
    fn new() -> Self {
        let prefix = GString::from("");
        let keep = false;
        Self {
            _phantom: std::marker::PhantomData, prefix: CowArg::Owned(prefix), keep: keep,
        }
    }
    #[inline]
    pub fn prefix(self, prefix: impl AsArg < GString > + 'ex) -> Self {
        Self {
            prefix: prefix.into_arg(), .. self
        }
    }
    #[inline]
    pub fn keep(self, keep: bool) -> Self {
        Self {
            keep: keep, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::DirAccess > > {
        let Self {
            _phantom, prefix, keep,
        }
        = self;
        re_export::DirAccess::create_temp_full(prefix, keep,)
    }
}
#[doc = "Default-param extender for [`DirAccess::get_current_dir_ex`][super::DirAccess::get_current_dir_ex]."]
#[must_use]
pub struct ExGetCurrentDir < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DirAccess, include_drive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetCurrentDir < 'ex > {
    fn new(surround_object: &'ex re_export::DirAccess,) -> Self {
        let include_drive = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_drive: include_drive,
        }
    }
    #[inline]
    pub fn include_drive(self, include_drive: bool) -> Self {
        Self {
            include_drive: include_drive, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, include_drive,
        }
        = self;
        re_export::DirAccess::get_current_dir_full(surround_object, include_drive,)
    }
}
#[doc = "Default-param extender for [`DirAccess::copy_ex`][super::DirAccess::copy_ex]."]
#[must_use]
pub struct ExCopy < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DirAccess, from: CowArg < 'ex, GString >, to: CowArg < 'ex, GString >, chmod_flags: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCopy < 'ex > {
    fn new(surround_object: &'ex mut re_export::DirAccess, from: impl AsArg < GString > + 'ex, to: impl AsArg < GString > + 'ex,) -> Self {
        let chmod_flags = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from.into_arg(), to: to.into_arg(), chmod_flags: chmod_flags,
        }
    }
    #[inline]
    pub fn chmod_flags(self, chmod_flags: i32) -> Self {
        Self {
            chmod_flags: chmod_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, from, to, chmod_flags,
        }
        = self;
        re_export::DirAccess::copy_full(surround_object, from, to, chmod_flags,)
    }
}
#[doc = "Default-param extender for [`DirAccess::copy_absolute_ex`][super::DirAccess::copy_absolute_ex]."]
#[must_use]
pub struct ExCopyAbsolute < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, from: CowArg < 'ex, GString >, to: CowArg < 'ex, GString >, chmod_flags: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCopyAbsolute < 'ex > {
    fn new(from: impl AsArg < GString > + 'ex, to: impl AsArg < GString > + 'ex,) -> Self {
        let chmod_flags = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, from: from.into_arg(), to: to.into_arg(), chmod_flags: chmod_flags,
        }
    }
    #[inline]
    pub fn chmod_flags(self, chmod_flags: i32) -> Self {
        Self {
            chmod_flags: chmod_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, from, to, chmod_flags,
        }
        = self;
        re_export::DirAccess::copy_absolute_full(from, to, chmod_flags,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::DirAccess;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for DirAccess {
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
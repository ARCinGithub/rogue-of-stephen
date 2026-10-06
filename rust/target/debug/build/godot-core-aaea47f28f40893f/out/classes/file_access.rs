#![doc = "Sidecar module for class [`FileAccess`][crate::classes::FileAccess].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FileAccess` enums](https://docs.godotengine.org/en/stable/classes/class_fileaccess.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FileAccess`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`file_access`][crate::classes::file_access]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `FileAccess`](https://docs.godotengine.org/en/stable/classes/class_fileaccess.html).\n\n# Specific notes for this class\n\nThe godot-rust library provides a higher-level abstraction, which should be preferred: [`GFile`][crate::tools::GFile]."]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<FileAccess>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThis class can be used to permanently store data in the user device's file system and to read from it. This is useful for storing game save data or player configuration files.\n\n**Example:** How to write and read from a file. The file named `\"save_game.dat\"` will be stored in the user data folder, as specified in the [Data paths]($DOCS_URL/tutorials/io/data_paths.html) documentation:\n\n\n```gdscript\nfunc save_to_file(content):\n\tvar file = FileAccess.open(\"user://save_game.dat\", FileAccess.WRITE)\n\tfile.store_string(content)\n\nfunc load_from_file():\n\tvar file = FileAccess.open(\"user://save_game.dat\", FileAccess.READ)\n\tvar content = file.get_as_text()\n\treturn content\n```\n\n\nA `FileAccess` instance has its own file cursor, which is the position in bytes in the file where the next read/write operation will occur. Functions such as [`get_8`][`crate::classes::FileAccess::get_8`], [`get_16`][`crate::classes::FileAccess::get_16`], [`store_8`][`crate::classes::FileAccess::store_8`], and [`store_16`][`crate::classes::FileAccess::store_16`] will move the file cursor forward by the number of bytes read/written. The file cursor can be moved to a specific position using [`seek`][`crate::classes::FileAccess::seek`] or [`seek_end`][`crate::classes::FileAccess::seek_end`], and its position can be retrieved using [`get_position`][`crate::classes::FileAccess::get_position`].\n\nA `FileAccess` instance will close its file when the instance is freed. Since it inherits [`RefCounted`][crate::classes::RefCounted], this happens automatically when it is no longer in use. [`close`][`crate::classes::FileAccess::close`] can be called to close it earlier. In C#, the reference must be disposed manually, which can be done with the `using` statement or by calling the `Dispose` method directly.\n\n**Note:** To access project resources once exported, it is recommended to use [`ResourceLoader`][crate::classes::ResourceLoader] instead of `FileAccess`, as some files are converted to engine-specific formats and their original source files might not be present in the exported PCK package. If using `FileAccess`, make sure the file is included in the export by changing its import mode to **Keep File (exported as is)** in the Import dock, or, for files where this option is not available, change the non-resource export filter in the Export dialog to include the file's extension (e.g. `*.txt`).\n\n**Note:** Files are automatically closed only if the process exits \"normally\" (such as by clicking the window manager's close button or pressing `Alt + F4`). If you stop the project execution by pressing `F8` while the project is running, the file won't be closed as the game process will be killed. You can work around this by calling [`flush`][`crate::classes::FileAccess::flush`] at regular intervals."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FileAccess {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl FileAccess {
        #[doc = "Creates a new `FileAccess` object and opens the file for writing or reading, depending on the flags.\n\nReturns `null` if opening the file failed. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        pub fn open(path: impl AsArg < GString >, flags: crate::classes::file_access::ModeFlags,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallRet = Option < Gd < crate::classes::FileAccess > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::file_access::ModeFlags,);
            let args = (path.into_arg(), flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "open", None, args,)
            }
        }
        #[doc = "Creates a new `FileAccess` object and opens an encrypted file in write or read mode. You need to pass a binary key to encrypt/decrypt it.\n\n**Note:** The provided key must be 32 bytes long.\n\nReturns `null` if opening the file failed. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        pub(crate) fn open_encrypted_full(path: CowArg < GString >, mode_flags: crate::classes::file_access::ModeFlags, key: RefArg < PackedByteArray >, iv: RefArg < PackedByteArray >,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallRet = Option < Gd < crate::classes::FileAccess > >;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, crate::classes::file_access::ModeFlags, RefArg < 'a1, PackedByteArray >, RefArg < 'a2, PackedByteArray >,);
            let args = (path, mode_flags, key, iv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "open_encrypted", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`open_encrypted_ex`][Self::open_encrypted_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new `FileAccess` object and opens an encrypted file in write or read mode. You need to pass a binary key to encrypt/decrypt it.\n\n**Note:** The provided key must be 32 bytes long.\n\nReturns `null` if opening the file failed. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        #[inline]
        pub fn open_encrypted(path: impl AsArg < GString >, mode_flags: crate::classes::file_access::ModeFlags, key: &PackedByteArray,) -> Option < Gd < crate::classes::FileAccess > > {
            Self::open_encrypted_ex(path, mode_flags, key,) . done()
        }
        #[doc = "Creates a new `FileAccess` object and opens an encrypted file in write or read mode. You need to pass a binary key to encrypt/decrypt it.\n\n**Note:** The provided key must be 32 bytes long.\n\nReturns `null` if opening the file failed. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        #[inline]
        pub fn open_encrypted_ex < 'ex > (path: impl AsArg < GString > + 'ex, mode_flags: crate::classes::file_access::ModeFlags, key: &'ex PackedByteArray,) -> ExOpenEncrypted < 'ex > {
            ExOpenEncrypted::new(path, mode_flags, key,)
        }
        #[doc = "Creates a new `FileAccess` object and opens an encrypted file in write or read mode. You need to pass a password to encrypt/decrypt it.\n\nReturns `null` if opening the file failed. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        pub fn open_encrypted_with_pass(path: impl AsArg < GString >, mode_flags: crate::classes::file_access::ModeFlags, pass: impl AsArg < GString >,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallRet = Option < Gd < crate::classes::FileAccess > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, crate::classes::file_access::ModeFlags, CowArg < 'a1, GString >,);
            let args = (path.into_arg(), mode_flags, pass.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "open_encrypted_with_pass", None, args,)
            }
        }
        #[doc = "Creates a new `FileAccess` object and opens a compressed file for reading or writing.\n\n**Note:** [`open_compressed`][`crate::classes::FileAccess::open_compressed`] can only read files that were saved by Godot, not third-party compression formats. See [GitHub issue #28999](https://github.com/godotengine/godot/issues/28999) for a workaround.\n\nReturns `null` if opening the file failed. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        pub(crate) fn open_compressed_full(path: CowArg < GString >, mode_flags: crate::classes::file_access::ModeFlags, compression_mode: crate::classes::file_access::CompressionMode,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallRet = Option < Gd < crate::classes::FileAccess > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::file_access::ModeFlags, crate::classes::file_access::CompressionMode,);
            let args = (path, mode_flags, compression_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "open_compressed", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`open_compressed_ex`][Self::open_compressed_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new `FileAccess` object and opens a compressed file for reading or writing.\n\n**Note:** [`open_compressed`][`crate::classes::FileAccess::open_compressed`] can only read files that were saved by Godot, not third-party compression formats. See [GitHub issue #28999](https://github.com/godotengine/godot/issues/28999) for a workaround.\n\nReturns `null` if opening the file failed. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        #[inline]
        pub fn open_compressed(path: impl AsArg < GString >, mode_flags: crate::classes::file_access::ModeFlags,) -> Option < Gd < crate::classes::FileAccess > > {
            Self::open_compressed_ex(path, mode_flags,) . done()
        }
        #[doc = "Creates a new `FileAccess` object and opens a compressed file for reading or writing.\n\n**Note:** [`open_compressed`][`crate::classes::FileAccess::open_compressed`] can only read files that were saved by Godot, not third-party compression formats. See [GitHub issue #28999](https://github.com/godotengine/godot/issues/28999) for a workaround.\n\nReturns `null` if opening the file failed. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        #[inline]
        pub fn open_compressed_ex < 'ex > (path: impl AsArg < GString > + 'ex, mode_flags: crate::classes::file_access::ModeFlags,) -> ExOpenCompressed < 'ex > {
            ExOpenCompressed::new(path, mode_flags,)
        }
        #[doc = "Returns the result of the last [`open`][`crate::classes::FileAccess::open`] call in the current thread."]
        pub fn get_open_error() -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_open_error", None, args,)
            }
        }
        #[doc = "Creates a temporary file. This file will be freed when the returned `FileAccess` is freed.\n\nIf `prefix` is not empty, it will be prefixed to the file name, separated by a `-`.\n\nIf `extension` is not empty, it will be appended to the temporary file name.\n\nIf `keep` is `true`, the file is not deleted when the returned `FileAccess` is freed.\n\nReturns `null` if opening the file failed. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        pub(crate) fn create_temp_full(mode_flags: crate::classes::file_access::ModeFlags, prefix: CowArg < GString >, extension: CowArg < GString >, keep: bool,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallRet = Option < Gd < crate::classes::FileAccess > >;
            type CallParams < 'a0, 'a1, > = (crate::classes::file_access::ModeFlags, CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool,);
            let args = (mode_flags, prefix, extension, keep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11076usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "create_temp", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_temp_ex`][Self::create_temp_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a temporary file. This file will be freed when the returned `FileAccess` is freed.\n\nIf `prefix` is not empty, it will be prefixed to the file name, separated by a `-`.\n\nIf `extension` is not empty, it will be appended to the temporary file name.\n\nIf `keep` is `true`, the file is not deleted when the returned `FileAccess` is freed.\n\nReturns `null` if opening the file failed. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        #[inline]
        pub fn create_temp(mode_flags: crate::classes::file_access::ModeFlags,) -> Option < Gd < crate::classes::FileAccess > > {
            Self::create_temp_ex(mode_flags,) . done()
        }
        #[doc = "Creates a temporary file. This file will be freed when the returned `FileAccess` is freed.\n\nIf `prefix` is not empty, it will be prefixed to the file name, separated by a `-`.\n\nIf `extension` is not empty, it will be appended to the temporary file name.\n\nIf `keep` is `true`, the file is not deleted when the returned `FileAccess` is freed.\n\nReturns `null` if opening the file failed. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        #[inline]
        pub fn create_temp_ex < 'ex > (mode_flags: crate::classes::file_access::ModeFlags,) -> ExCreateTemp < 'ex > {
            ExCreateTemp::new(mode_flags,)
        }
        #[doc = "Returns the whole `path` file contents as a [`PackedByteArray`][crate::builtin::PackedByteArray] without any decoding.\n\nReturns an empty [`PackedByteArray`][crate::builtin::PackedByteArray] if an error occurred while opening the file. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        pub fn get_file_as_bytes(path: impl AsArg < GString >,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11077usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_file_as_bytes", None, args,)
            }
        }
        #[doc = "Returns the whole `path` file contents as a [`String`][crate::builtin::GString]. Text is interpreted as being UTF-8 encoded.\n\nReturns an empty [`String`][crate::builtin::GString] if an error occurred while opening the file. You can use [`get_open_error`][`crate::classes::FileAccess::get_open_error`] to check the error that occurred."]
        pub fn get_file_as_string(path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11078usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_file_as_string", None, args,)
            }
        }
        #[doc = "Resizes the file to a specified length. The file must be open in a mode that permits writing. If the file is extended, NUL characters are appended. If the file is truncated, all data from the end file to the original length of the file is lost."]
        pub fn resize(&mut self, length: i64,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (i64,);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11079usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "resize", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Writes the file's buffer to disk. Flushing is automatically performed when the file is closed. This means you don't need to call [`flush`][`crate::classes::FileAccess::flush`] manually before closing a file. Still, calling [`flush`][`crate::classes::FileAccess::flush`] can be used to ensure the data is safe even if the project crashes instead of being closed gracefully.\n\n**Note:** Only call [`flush`][`crate::classes::FileAccess::flush`] when you actually need it. Otherwise, it will decrease performance due to constant disk writes."]
        pub fn flush(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11080usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "flush", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the path as a [`String`][crate::builtin::GString] for the current open file."]
        pub fn get_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11081usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the absolute path as a [`String`][crate::builtin::GString] for the current open file."]
        pub fn get_path_absolute(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11082usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_path_absolute", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the file is currently opened."]
        pub fn is_open(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11083usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "is_open", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the file cursor to the specified position in bytes, from the beginning of the file. This changes the value returned by [`get_position`][`crate::classes::FileAccess::get_position`]."]
        pub fn seek(&mut self, position: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11084usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "seek", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the file cursor to the specified position in bytes, from the end of the file. This changes the value returned by [`get_position`][`crate::classes::FileAccess::get_position`].\n\n**Note:** This is an offset, so you should use negative numbers otherwise the file cursor will be at the end of the file."]
        pub(crate) fn seek_end_full(&mut self, position: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11085usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "seek_end", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`seek_end_ex`][Self::seek_end_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the file cursor to the specified position in bytes, from the end of the file. This changes the value returned by [`get_position`][`crate::classes::FileAccess::get_position`].\n\n**Note:** This is an offset, so you should use negative numbers otherwise the file cursor will be at the end of the file."]
        #[inline]
        pub fn seek_end(&mut self,) {
            self.seek_end_ex() . done()
        }
        #[doc = "Sets the file cursor to the specified position in bytes, from the end of the file. This changes the value returned by [`get_position`][`crate::classes::FileAccess::get_position`].\n\n**Note:** This is an offset, so you should use negative numbers otherwise the file cursor will be at the end of the file."]
        #[inline]
        pub fn seek_end_ex < 'ex > (&'ex mut self,) -> ExSeekEnd < 'ex > {
            ExSeekEnd::new(self,)
        }
        #[doc = "Returns the file cursor's position in bytes from the beginning of the file. This is the file reading/writing cursor set by [`seek`][`crate::classes::FileAccess::seek`] or [`seek_end`][`crate::classes::FileAccess::seek_end`] and advanced by read/write operations."]
        pub fn get_position(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11086usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the size of the file in bytes. For a pipe, returns the number of bytes available for reading from the pipe."]
        pub fn get_length(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11087usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_length", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the file cursor has already read past the end of the file.\n\n**Note:** `eof_reached() == false` cannot be used to check whether there is more data available. To loop while there is more data available, use:\n\n\n```gdscript\nwhile file.get_position() < file.get_length():\n\t# Read data\n```\n"]
        pub fn eof_reached(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11088usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "eof_reached", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next 8 bits from the file as an integer. This advances the file cursor by 1 byte. See [`store_8`][`crate::classes::FileAccess::store_8`] for details on what values can be stored and retrieved this way."]
        pub fn get_8(&mut self,) -> u8 {
            type CallRet = u8;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11089usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_8", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next 16 bits from the file as an integer. This advances the file cursor by 2 bytes. See [`store_16`][`crate::classes::FileAccess::store_16`] for details on what values can be stored and retrieved this way."]
        pub fn get_16(&mut self,) -> u16 {
            type CallRet = u16;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_16", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next 32 bits from the file as an integer. This advances the file cursor by 4 bytes. See [`store_32`][`crate::classes::FileAccess::store_32`] for details on what values can be stored and retrieved this way."]
        pub fn get_32(&mut self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_32", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next 64 bits from the file as an integer. This advances the file cursor by 8 bytes. See [`store_64`][`crate::classes::FileAccess::store_64`] for details on what values can be stored and retrieved this way."]
        pub fn get_64(&mut self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_64", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next 16 bits from the file as a half-precision floating-point number. This advances the file cursor by 2 bytes."]
        pub fn get_half(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_half", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next 32 bits from the file as a floating-point number. This advances the file cursor by 4 bytes."]
        pub fn get_float(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_float", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next 64 bits from the file as a floating-point number. This advances the file cursor by 8 bytes."]
        pub fn get_double(&mut self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_double", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next bits from the file as a floating-point number. This advances the file cursor by either 4 or 8 bytes, depending on the precision used by the Godot build that saved the file.\n\nIf the file was saved by a Godot build compiled with the `precision=single` option (the default), the number of read bits for that file is 32. Otherwise, if compiled with the `precision=double` option, the number of read bits is 64."]
        pub fn get_real(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_real", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns next `length` bytes of the file as a [`PackedByteArray`][crate::builtin::PackedByteArray]. This advances the file cursor by `length` bytes."]
        pub fn get_buffer(&mut self, length: i64,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (i64,);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11097usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next line of the file as a [`String`][crate::builtin::GString]. The returned string doesn't include newline (`\\n`) or carriage return (`\\r`) characters, but does include any other leading or trailing whitespace. This advances the file cursor to after the newline character at the end of the line.\n\nText is interpreted as being UTF-8 encoded."]
        pub fn get_line(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11098usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next value of the file in CSV (Comma-Separated Values) format. You can pass a different delimiter `delim` to use other than the default `\",\"` (comma). This delimiter must be one-character long, and cannot be a double quotation mark.\n\nText is interpreted as being UTF-8 encoded. Text values must be enclosed in double quotes if they include the delimiter character. Double quotes within a text value can be escaped by doubling their occurrence. This advances the file cursor to after the newline character at the end of the line.\n\nFor example, the following CSV lines are valid and will be properly parsed as two strings each:\n\n```text\nAlice,\"Hello, Bob!\"\nBob,Alice! What a surprise!\nAlice,\"I thought you'd reply with \"\"Hello, world\"\".\"\n```\n\nNote how the second line can omit the enclosing quotes as it does not include the delimiter. However it _could_ very well use quotes, it was only written without for demonstration purposes. The third line must use `\"\"` for each quotation mark that needs to be interpreted as such instead of the end of a text value."]
        pub(crate) fn get_csv_line_full(&mut self, delim: CowArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (delim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11099usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_csv_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_csv_line_ex`][Self::get_csv_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the next value of the file in CSV (Comma-Separated Values) format. You can pass a different delimiter `delim` to use other than the default `\",\"` (comma). This delimiter must be one-character long, and cannot be a double quotation mark.\n\nText is interpreted as being UTF-8 encoded. Text values must be enclosed in double quotes if they include the delimiter character. Double quotes within a text value can be escaped by doubling their occurrence. This advances the file cursor to after the newline character at the end of the line.\n\nFor example, the following CSV lines are valid and will be properly parsed as two strings each:\n\n```text\nAlice,\"Hello, Bob!\"\nBob,Alice! What a surprise!\nAlice,\"I thought you'd reply with \"\"Hello, world\"\".\"\n```\n\nNote how the second line can omit the enclosing quotes as it does not include the delimiter. However it _could_ very well use quotes, it was only written without for demonstration purposes. The third line must use `\"\"` for each quotation mark that needs to be interpreted as such instead of the end of a text value."]
        #[inline]
        pub fn get_csv_line(&mut self,) -> PackedStringArray {
            self.get_csv_line_ex() . done()
        }
        #[doc = "Returns the next value of the file in CSV (Comma-Separated Values) format. You can pass a different delimiter `delim` to use other than the default `\",\"` (comma). This delimiter must be one-character long, and cannot be a double quotation mark.\n\nText is interpreted as being UTF-8 encoded. Text values must be enclosed in double quotes if they include the delimiter character. Double quotes within a text value can be escaped by doubling their occurrence. This advances the file cursor to after the newline character at the end of the line.\n\nFor example, the following CSV lines are valid and will be properly parsed as two strings each:\n\n```text\nAlice,\"Hello, Bob!\"\nBob,Alice! What a surprise!\nAlice,\"I thought you'd reply with \"\"Hello, world\"\".\"\n```\n\nNote how the second line can omit the enclosing quotes as it does not include the delimiter. However it _could_ very well use quotes, it was only written without for demonstration purposes. The third line must use `\"\"` for each quotation mark that needs to be interpreted as such instead of the end of a text value."]
        #[inline]
        pub fn get_csv_line_ex < 'ex > (&'ex mut self,) -> ExGetCsvLine < 'ex > {
            ExGetCsvLine::new(self,)
        }
        #[doc = "Returns the whole file as a [`String`][crate::builtin::GString]. Text is interpreted as being UTF-8 encoded. This ignores the file cursor and does not affect it."]
        pub fn get_as_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_as_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an MD5 String representing the file at the given path or an empty [`String`][crate::builtin::GString] on failure."]
        pub fn get_md5(path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_md5", None, args,)
            }
        }
        #[doc = "Returns an SHA-256 [`String`][crate::builtin::GString] representing the file at the given path or an empty [`String`][crate::builtin::GString] on failure."]
        pub fn get_sha256(path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_sha256", None, args,)
            }
        }
        pub fn is_big_endian(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "is_big_endian", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_big_endian(&mut self, big_endian: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (big_endian,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "set_big_endian", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the last error that happened when trying to perform operations. Compare with the `ERR_FILE_*` constants from \\[enum Error]."]
        pub fn get_error(&self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_error", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next [`Variant`][crate::builtin::Variant] value from the file. If `allow_objects` is `true`, decoding objects is allowed. This advances the file cursor by the number of bytes read.\n\nInternally, this uses the same decoding mechanism as the [`bytes_to_var`][`crate::global::bytes_to_var`] method, as described in the [Binary serialization API]($DOCS_URL/tutorials/io/binary_serialization_api.html) documentation.\n\n**Warning:** Deserialized objects can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats such as remote code execution."]
        pub(crate) fn get_var_full(&mut self, allow_objects: bool,) -> Variant {
            type CallRet = Variant;
            type CallParams = (bool,);
            let args = (allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_var", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_var_ex`][Self::get_var_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the next [`Variant`][crate::builtin::Variant] value from the file. If `allow_objects` is `true`, decoding objects is allowed. This advances the file cursor by the number of bytes read.\n\nInternally, this uses the same decoding mechanism as the [`bytes_to_var`][`crate::global::bytes_to_var`] method, as described in the [Binary serialization API]($DOCS_URL/tutorials/io/binary_serialization_api.html) documentation.\n\n**Warning:** Deserialized objects can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats such as remote code execution."]
        #[inline]
        pub fn get_var(&mut self,) -> Variant {
            self.get_var_ex() . done()
        }
        #[doc = "Returns the next [`Variant`][crate::builtin::Variant] value from the file. If `allow_objects` is `true`, decoding objects is allowed. This advances the file cursor by the number of bytes read.\n\nInternally, this uses the same decoding mechanism as the [`bytes_to_var`][`crate::global::bytes_to_var`] method, as described in the [Binary serialization API]($DOCS_URL/tutorials/io/binary_serialization_api.html) documentation.\n\n**Warning:** Deserialized objects can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats such as remote code execution."]
        #[inline]
        pub fn get_var_ex < 'ex > (&'ex mut self,) -> ExGetVar < 'ex > {
            ExGetVar::new(self,)
        }
        #[doc = "Stores an integer as 8 bits in the file. This advances the file cursor by 1 byte. Returns `true` if the operation is successful.\n\n**Note:** The `value` should lie in the interval `[0, 255]`. Any other value will overflow and wrap around.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate.\n\nTo store a signed integer, use [`store_64`][`crate::classes::FileAccess::store_64`], or convert it manually (see [`store_16`][`crate::classes::FileAccess::store_16`] for an example)."]
        pub fn store_8(&mut self, value: u8,) -> bool {
            type CallRet = bool;
            type CallParams = (u8,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_8", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stores an integer as 16 bits in the file. This advances the file cursor by 2 bytes. Returns `true` if the operation is successful.\n\n**Note:** The `value` should lie in the interval `[0, 2^16 - 1]`. Any other value will overflow and wrap around.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate.\n\nTo store a signed integer, use [`store_64`][`crate::classes::FileAccess::store_64`] or store a signed integer from the interval `[-2^15, 2^15 - 1]` (i.e. keeping one bit for the signedness) and compute its sign manually when reading. For example:\n\n\n```gdscript\nconst MAX_15B = 1 << 15\nconst MAX_16B = 1 << 16\n\nfunc unsigned16_to_signed(unsigned):\n\treturn (unsigned + MAX_15B) % MAX_16B - MAX_15B\n\nfunc _ready():\n\tvar f = FileAccess.open(\"user://file.dat\", FileAccess.WRITE_READ)\n\tf.store_16(-42) # This wraps around and stores 65494 (2^16 - 42).\n\tf.store_16(121) # In bounds, will store 121.\n\tf.seek(0) # Go back to start to read the stored value.\n\tvar read1 = f.get_16() # 65494\n\tvar read2 = f.get_16() # 121\n\tvar converted1 = unsigned16_to_signed(read1) # -42\n\tvar converted2 = unsigned16_to_signed(read2) # 121\n```\n"]
        pub fn store_16(&mut self, value: u16,) -> bool {
            type CallRet = bool;
            type CallParams = (u16,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_16", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stores an integer as 32 bits in the file. This advances the file cursor by 4 bytes. Returns `true` if the operation is successful.\n\n**Note:** The `value` should lie in the interval `[0, 2^32 - 1]`. Any other value will overflow and wrap around.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate.\n\nTo store a signed integer, use [`store_64`][`crate::classes::FileAccess::store_64`], or convert it manually (see [`store_16`][`crate::classes::FileAccess::store_16`] for an example)."]
        pub fn store_32(&mut self, value: u32,) -> bool {
            type CallRet = bool;
            type CallParams = (u32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_32", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stores an integer as 64 bits in the file. This advances the file cursor by 8 bytes. Returns `true` if the operation is successful.\n\n**Note:** The `value` must lie in the interval `[-2^63, 2^63 - 1]` (i.e. be a valid `int` value).\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        pub fn store_64(&mut self, value: u64,) -> bool {
            type CallRet = bool;
            type CallParams = (u64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_64", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stores a half-precision floating-point number as 16 bits in the file. This advances the file cursor by 2 bytes. Returns `true` if the operation is successful.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        pub fn store_half(&mut self, value: f32,) -> bool {
            type CallRet = bool;
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_half", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stores a floating-point number as 32 bits in the file. This advances the file cursor by 4 bytes. Returns `true` if the operation is successful.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        pub fn store_float(&mut self, value: f32,) -> bool {
            type CallRet = bool;
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_float", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stores a floating-point number as 64 bits in the file. This advances the file cursor by 8 bytes. Returns `true` if the operation is successful.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        pub fn store_double(&mut self, value: f64,) -> bool {
            type CallRet = bool;
            type CallParams = (f64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_double", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stores a floating-point number in the file. This advances the file cursor by either 4 or 8 bytes, depending on the precision used by the current Godot build.\n\nIf using a Godot build compiled with the `precision=single` option (the default), this method will save a 32-bit float. Otherwise, if compiled with the `precision=double` option, this will save a 64-bit float. Returns `true` if the operation is successful.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        pub fn store_real(&mut self, value: f32,) -> bool {
            type CallRet = bool;
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_real", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stores the given array of bytes in the file. This advances the file cursor by the number of bytes written. Returns `true` if the operation is successful.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        pub fn store_buffer(&mut self, buffer: &PackedByteArray,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stores `line` in the file followed by a newline character (`\\n`), encoding the text as UTF-8. This advances the file cursor by the length of the line, after the newline character. The amount of bytes written depends on the UTF-8 encoded bytes, which may be different from [`len`][`crate::builtin::GString::len`] which counts the number of UTF-32 codepoints. Returns `true` if the operation is successful.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        pub fn store_line(&mut self, line: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (line.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stores the given [`PackedStringArray`][crate::builtin::PackedStringArray] in the file as a line formatted in the CSV (Comma-Separated Values) format. You can pass a different delimiter `delim` to use other than the default `\",\"` (comma). This delimiter must be one-character long.\n\nText will be encoded as UTF-8. Returns `true` if the operation is successful.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        pub(crate) fn store_csv_line_full(&mut self, values: RefArg < PackedStringArray >, delim: CowArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedStringArray >, CowArg < 'a1, GString >,);
            let args = (values, delim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_csv_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`store_csv_line_ex`][Self::store_csv_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Stores the given [`PackedStringArray`][crate::builtin::PackedStringArray] in the file as a line formatted in the CSV (Comma-Separated Values) format. You can pass a different delimiter `delim` to use other than the default `\",\"` (comma). This delimiter must be one-character long.\n\nText will be encoded as UTF-8. Returns `true` if the operation is successful.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        #[inline]
        pub fn store_csv_line(&mut self, values: &PackedStringArray,) -> bool {
            self.store_csv_line_ex(values,) . done()
        }
        #[doc = "Stores the given [`PackedStringArray`][crate::builtin::PackedStringArray] in the file as a line formatted in the CSV (Comma-Separated Values) format. You can pass a different delimiter `delim` to use other than the default `\",\"` (comma). This delimiter must be one-character long.\n\nText will be encoded as UTF-8. Returns `true` if the operation is successful.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        #[inline]
        pub fn store_csv_line_ex < 'ex > (&'ex mut self, values: &'ex PackedStringArray,) -> ExStoreCsvLine < 'ex > {
            ExStoreCsvLine::new(self, values,)
        }
        #[doc = "Stores `string` in the file without a newline character (`\\n`), encoding the text as UTF-8. This advances the file cursor by the length of the string in UTF-8 encoded bytes, which may be different from [`len`][`crate::builtin::GString::len`] which counts the number of UTF-32 codepoints. Returns `true` if the operation is successful.\n\n**Note:** This method is intended to be used to write text files. The string is stored as a UTF-8 encoded buffer without string length or terminating zero, which means that it can't be loaded back easily. If you want to store a retrievable string in a binary file, consider using [`store_pascal_string`][`crate::classes::FileAccess::store_pascal_string`] instead. For retrieving strings from a text file, you can use `get_buffer(length).get_string_from_utf8()` (if you know the length) or [`get_as_text`][`crate::classes::FileAccess::get_as_text`].\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        pub fn store_string(&mut self, string: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stores any Variant value in the file. If `full_objects` is `true`, encoding objects is allowed (and can potentially include code). This advances the file cursor by the number of bytes written. Returns `true` if the operation is successful.\n\nInternally, this uses the same encoding mechanism as the [`var_to_bytes`][`crate::global::var_to_bytes`] method, as described in the [Binary serialization API]($DOCS_URL/tutorials/io/binary_serialization_api.html) documentation.\n\n**Note:** Not all properties are included. Only properties that are configured with the [`PropertyUsageFlags::STORAGE`][`crate::registry::info::PropertyUsageFlags::STORAGE`] flag set will be serialized. You can add a new usage flag to a property by overriding the [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`] method in your class. You can also check how property usage is configured by calling [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`]. See \\[enum PropertyUsageFlags] for the possible usage flags.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        pub(crate) fn store_var_full(&mut self, value: RefArg < Variant >, full_objects: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, bool,);
            let args = (value, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_var", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`store_var_ex`][Self::store_var_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Stores any Variant value in the file. If `full_objects` is `true`, encoding objects is allowed (and can potentially include code). This advances the file cursor by the number of bytes written. Returns `true` if the operation is successful.\n\nInternally, this uses the same encoding mechanism as the [`var_to_bytes`][`crate::global::var_to_bytes`] method, as described in the [Binary serialization API]($DOCS_URL/tutorials/io/binary_serialization_api.html) documentation.\n\n**Note:** Not all properties are included. Only properties that are configured with the [`PropertyUsageFlags::STORAGE`][`crate::registry::info::PropertyUsageFlags::STORAGE`] flag set will be serialized. You can add a new usage flag to a property by overriding the [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`] method in your class. You can also check how property usage is configured by calling [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`]. See \\[enum PropertyUsageFlags] for the possible usage flags.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        #[inline]
        pub fn store_var(&mut self, value: &Variant,) -> bool {
            self.store_var_ex(value,) . done()
        }
        #[doc = "Stores any Variant value in the file. If `full_objects` is `true`, encoding objects is allowed (and can potentially include code). This advances the file cursor by the number of bytes written. Returns `true` if the operation is successful.\n\nInternally, this uses the same encoding mechanism as the [`var_to_bytes`][`crate::global::var_to_bytes`] method, as described in the [Binary serialization API]($DOCS_URL/tutorials/io/binary_serialization_api.html) documentation.\n\n**Note:** Not all properties are included. Only properties that are configured with the [`PropertyUsageFlags::STORAGE`][`crate::registry::info::PropertyUsageFlags::STORAGE`] flag set will be serialized. You can add a new usage flag to a property by overriding the [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`] method in your class. You can also check how property usage is configured by calling [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`]. See \\[enum PropertyUsageFlags] for the possible usage flags.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        #[inline]
        pub fn store_var_ex < 'ex > (&'ex mut self, value: &'ex Variant,) -> ExStoreVar < 'ex > {
            ExStoreVar::new(self, value,)
        }
        #[doc = "Stores the given [`String`][crate::builtin::GString] as a line in the file in Pascal format (i.e. also store the length of the string). Text will be encoded as UTF-8. This advances the file cursor by the number of bytes written depending on the UTF-8 encoded bytes, which may be different from [`len`][`crate::builtin::GString::len`] which counts the number of UTF-32 codepoints. Returns `true` if the operation is successful.\n\n**Note:** If an error occurs, the resulting value of the file position indicator is indeterminate."]
        pub fn store_pascal_string(&mut self, string: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_pascal_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`String`][crate::builtin::GString] saved in Pascal format from the file, meaning that the length of the string is explicitly stored at the start. See [`store_pascal_string`][`crate::classes::FileAccess::store_pascal_string`]. This may include newline characters. The file cursor is advanced after the bytes read.\n\nText is interpreted as being UTF-8 encoded."]
        pub fn get_pascal_string(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_pascal_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Closes the currently opened file and prevents subsequent read/write operations. Use [`flush`][`crate::classes::FileAccess::flush`] to persist the data to disk without closing the file.\n\n**Note:** `FileAccess` will automatically close when it's freed, which happens when it goes out of scope or when it gets assigned with `null`. In C# the reference must be disposed after we are done using it, this can be done with the `using` statement or calling the `Dispose` method directly."]
        pub fn close(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "close", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the file exists in the given path.\n\n**Note:** Many resources types are imported (e.g. textures or sound files), and their source asset will not be included in the exported game, as only the imported version is used. See [`exists`][`crate::classes::ResourceLoader::exists`] for an alternative approach that takes resource remapping into account.\n\nFor a non-static, relative equivalent, use [`file_exists`][`crate::classes::DirAccess::file_exists`]."]
        pub fn file_exists(path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "file_exists", None, args,)
            }
        }
        #[doc = "Returns the last time the `file` was modified in Unix timestamp format, or `0` on error. This Unix timestamp can be converted to another format using the [`Time`][crate::classes::Time] singleton."]
        pub fn get_modified_time(file: impl AsArg < GString >,) -> u64 {
            type CallRet = u64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_modified_time", None, args,)
            }
        }
        #[doc = "Returns the last time the `file` was accessed in Unix timestamp format, or `0` on error. This Unix timestamp can be converted to another format using the [`Time`][crate::classes::Time] singleton."]
        pub fn get_access_time(file: impl AsArg < GString >,) -> u64 {
            type CallRet = u64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_access_time", None, args,)
            }
        }
        #[doc = "Returns the size of the file at the given path, in bytes, or `-1` on error."]
        pub fn get_size(file: impl AsArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_size", None, args,)
            }
        }
        #[doc = "Returns the UNIX permissions of the file at the given path.\n\n**Note:** This method is implemented on iOS, Linux/BSD, and macOS."]
        pub fn get_unix_permissions(file: impl AsArg < GString >,) -> crate::classes::file_access::UnixPermissionFlags {
            type CallRet = crate::classes::file_access::UnixPermissionFlags;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_unix_permissions", None, args,)
            }
        }
        #[doc = "Sets file UNIX permissions.\n\n**Note:** This method is implemented on iOS, Linux/BSD, and macOS."]
        pub fn set_unix_permissions(file: impl AsArg < GString >, permissions: crate::classes::file_access::UnixPermissionFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::file_access::UnixPermissionFlags,);
            let args = (file.into_arg(), permissions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "set_unix_permissions", None, args,)
            }
        }
        #[doc = "Returns `true` if the **hidden** attribute is set on the file at the given path.\n\n**Note:** This method is implemented on iOS, BSD, macOS, and Windows."]
        pub fn get_hidden_attribute(file: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_hidden_attribute", None, args,)
            }
        }
        #[doc = "Sets file **hidden** attribute.\n\n**Note:** This method is implemented on iOS, BSD, macOS, and Windows."]
        pub fn set_hidden_attribute(file: impl AsArg < GString >, hidden: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (file.into_arg(), hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "set_hidden_attribute", None, args,)
            }
        }
        #[doc = "Sets file **read only** attribute.\n\n**Note:** This method is implemented on iOS, BSD, macOS, and Windows."]
        pub fn set_read_only_attribute(file: impl AsArg < GString >, ro: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (file.into_arg(), ro,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "set_read_only_attribute", None, args,)
            }
        }
        #[doc = "Returns `true` if the **read only** attribute is set on the file at the given path.\n\n**Note:** This method is implemented on iOS, BSD, macOS, and Windows."]
        pub fn get_read_only_attribute(file: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_read_only_attribute", None, args,)
            }
        }
        #[doc = "Reads the file extended attribute with name `attribute_name` as a byte array.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** Extended attributes support depends on the file system. Attributes will be lost when the file is moved between incompatible file systems.\n\n**Note:** On Linux, only \"user\" namespace attributes are accessible, namespace prefix should not be included.\n\n**Note:** On Windows, alternate data streams are used to store extended attributes."]
        pub fn get_extended_attribute(file: impl AsArg < GString >, attribute_name: impl AsArg < GString >,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (file.into_arg(), attribute_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_extended_attribute", None, args,)
            }
        }
        #[doc = "Reads the file extended attribute with name `attribute_name` as a UTF-8 encoded string.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** Extended attributes support depends on the file system. Attributes will be lost when the file is moved between incompatible file systems.\n\n**Note:** On Linux, only \"user\" namespace attributes are accessible, namespace prefix should not be included.\n\n**Note:** On Windows, alternate data streams are used to store extended attributes."]
        pub fn get_extended_attribute_string(file: impl AsArg < GString >, attribute_name: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (file.into_arg(), attribute_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_extended_attribute_string", None, args,)
            }
        }
        #[doc = "Writes file extended attribute with name `attribute_name` as a byte array.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** Extended attributes support depends on the file system. Attributes will be lost when the file is moved between incompatible file systems.\n\n**Note:** On Linux, only \"user\" namespace attributes are accessible, namespace prefix should not be included.\n\n**Note:** On Windows, alternate data streams are used to store extended attributes."]
        pub fn set_extended_attribute(file: impl AsArg < GString >, attribute_name: impl AsArg < GString >, data: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, PackedByteArray >,);
            let args = (file.into_arg(), attribute_name.into_arg(), RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "set_extended_attribute", None, args,)
            }
        }
        #[doc = "Writes file extended attribute with name `attribute_name` as a UTF-8 encoded string.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** Extended attributes support depends on the file system. Attributes will be lost when the file is moved between incompatible file systems.\n\n**Note:** On Linux, only \"user\" namespace attributes are accessible, namespace prefix should not be included.\n\n**Note:** On Windows, alternate data streams are used to store extended attributes."]
        pub fn set_extended_attribute_string(file: impl AsArg < GString >, attribute_name: impl AsArg < GString >, data: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >,);
            let args = (file.into_arg(), attribute_name.into_arg(), data.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "set_extended_attribute_string", None, args,)
            }
        }
        #[doc = "Removes file extended attribute with name `attribute_name`.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** Extended attributes support depends on the file system. Attributes will be lost when the file is moved between incompatible file systems.\n\n**Note:** On Linux, only \"user\" namespace attributes are accessible, namespace prefix should not be included.\n\n**Note:** On Windows, alternate data streams are used to store extended attributes."]
        pub fn remove_extended_attribute(file: impl AsArg < GString >, attribute_name: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (file.into_arg(), attribute_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "remove_extended_attribute", None, args,)
            }
        }
        #[doc = "Returns a list of file extended attributes.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** Extended attributes support depends on the file system. Attributes will be lost when the file is moved between incompatible file systems.\n\n**Note:** On Linux, only \"user\" namespace attributes are accessible, namespace prefix should not be included.\n\n**Note:** On Windows, alternate data streams are used to store extended attributes."]
        pub fn get_extended_attributes_list(file: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_extended_attributes_list", None, args,)
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
    impl crate::obj::GodotClass for FileAccess {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("FileAccess"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FileAccess {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for FileAccess {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FileAccess {
        
    }
    impl std::ops::Deref for FileAccess {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FileAccess {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_FileAccess__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `FileAccess` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`FileAccess::open_encrypted_ex`][super::FileAccess::open_encrypted_ex]."]
#[must_use]
pub struct ExOpenEncrypted < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, path: CowArg < 'ex, GString >, mode_flags: crate::classes::file_access::ModeFlags, key: CowArg < 'ex, PackedByteArray >, iv: CowArg < 'ex, PackedByteArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExOpenEncrypted < 'ex > {
    fn new(path: impl AsArg < GString > + 'ex, mode_flags: crate::classes::file_access::ModeFlags, key: &'ex PackedByteArray,) -> Self {
        let iv = PackedByteArray::new();
        Self {
            _phantom: std::marker::PhantomData, path: path.into_arg(), mode_flags: mode_flags, key: CowArg::Borrowed(key), iv: CowArg::Owned(iv),
        }
    }
    #[inline]
    pub fn iv(self, iv: &'ex PackedByteArray) -> Self {
        Self {
            iv: CowArg::Borrowed(iv), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::FileAccess > > {
        let Self {
            _phantom, path, mode_flags, key, iv,
        }
        = self;
        re_export::FileAccess::open_encrypted_full(path, mode_flags, key.cow_as_arg(), iv.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`FileAccess::open_compressed_ex`][super::FileAccess::open_compressed_ex]."]
#[must_use]
pub struct ExOpenCompressed < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, path: CowArg < 'ex, GString >, mode_flags: crate::classes::file_access::ModeFlags, compression_mode: crate::classes::file_access::CompressionMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExOpenCompressed < 'ex > {
    fn new(path: impl AsArg < GString > + 'ex, mode_flags: crate::classes::file_access::ModeFlags,) -> Self {
        let compression_mode = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, path: path.into_arg(), mode_flags: mode_flags, compression_mode: compression_mode,
        }
    }
    #[inline]
    pub fn compression_mode(self, compression_mode: crate::classes::file_access::CompressionMode) -> Self {
        Self {
            compression_mode: compression_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::FileAccess > > {
        let Self {
            _phantom, path, mode_flags, compression_mode,
        }
        = self;
        re_export::FileAccess::open_compressed_full(path, mode_flags, compression_mode,)
    }
}
#[doc = "Default-param extender for [`FileAccess::create_temp_ex`][super::FileAccess::create_temp_ex]."]
#[must_use]
pub struct ExCreateTemp < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, mode_flags: crate::classes::file_access::ModeFlags, prefix: CowArg < 'ex, GString >, extension: CowArg < 'ex, GString >, keep: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateTemp < 'ex > {
    fn new(mode_flags: crate::classes::file_access::ModeFlags,) -> Self {
        let prefix = GString::from("");
        let extension = GString::from("");
        let keep = false;
        Self {
            _phantom: std::marker::PhantomData, mode_flags: mode_flags, prefix: CowArg::Owned(prefix), extension: CowArg::Owned(extension), keep: keep,
        }
    }
    #[inline]
    pub fn prefix(self, prefix: impl AsArg < GString > + 'ex) -> Self {
        Self {
            prefix: prefix.into_arg(), .. self
        }
    }
    #[inline]
    pub fn extension(self, extension: impl AsArg < GString > + 'ex) -> Self {
        Self {
            extension: extension.into_arg(), .. self
        }
    }
    #[inline]
    pub fn keep(self, keep: bool) -> Self {
        Self {
            keep: keep, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::FileAccess > > {
        let Self {
            _phantom, mode_flags, prefix, extension, keep,
        }
        = self;
        re_export::FileAccess::create_temp_full(mode_flags, prefix, extension, keep,)
    }
}
#[doc = "Default-param extender for [`FileAccess::seek_end_ex`][super::FileAccess::seek_end_ex]."]
#[must_use]
pub struct ExSeekEnd < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::FileAccess, position: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSeekEnd < 'ex > {
    fn new(surround_object: &'ex mut re_export::FileAccess,) -> Self {
        let position = 0i64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: i64) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position,
        }
        = self;
        re_export::FileAccess::seek_end_full(surround_object, position,)
    }
}
#[doc = "Default-param extender for [`FileAccess::get_csv_line_ex`][super::FileAccess::get_csv_line_ex]."]
#[must_use]
pub struct ExGetCsvLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::FileAccess, delim: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetCsvLine < 'ex > {
    fn new(surround_object: &'ex mut re_export::FileAccess,) -> Self {
        let delim = GString::from(",");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, delim: CowArg::Owned(delim),
        }
    }
    #[inline]
    pub fn delim(self, delim: impl AsArg < GString > + 'ex) -> Self {
        Self {
            delim: delim.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, delim,
        }
        = self;
        re_export::FileAccess::get_csv_line_full(surround_object, delim,)
    }
}
#[doc = "Default-param extender for [`FileAccess::get_var_ex`][super::FileAccess::get_var_ex]."]
#[must_use]
pub struct ExGetVar < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::FileAccess, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetVar < 'ex > {
    fn new(surround_object: &'ex mut re_export::FileAccess,) -> Self {
        let allow_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, allow_objects: allow_objects,
        }
    }
    #[inline]
    pub fn allow_objects(self, allow_objects: bool) -> Self {
        Self {
            allow_objects: allow_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, allow_objects,
        }
        = self;
        re_export::FileAccess::get_var_full(surround_object, allow_objects,)
    }
}
#[doc = "Default-param extender for [`FileAccess::store_csv_line_ex`][super::FileAccess::store_csv_line_ex]."]
#[must_use]
pub struct ExStoreCsvLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::FileAccess, values: CowArg < 'ex, PackedStringArray >, delim: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExStoreCsvLine < 'ex > {
    fn new(surround_object: &'ex mut re_export::FileAccess, values: &'ex PackedStringArray,) -> Self {
        let delim = GString::from(",");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, values: CowArg::Borrowed(values), delim: CowArg::Owned(delim),
        }
    }
    #[inline]
    pub fn delim(self, delim: impl AsArg < GString > + 'ex) -> Self {
        Self {
            delim: delim.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, values, delim,
        }
        = self;
        re_export::FileAccess::store_csv_line_full(surround_object, values.cow_as_arg(), delim,)
    }
}
#[doc = "Default-param extender for [`FileAccess::store_var_ex`][super::FileAccess::store_var_ex]."]
#[must_use]
pub struct ExStoreVar < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::FileAccess, value: CowArg < 'ex, Variant >, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExStoreVar < 'ex > {
    fn new(surround_object: &'ex mut re_export::FileAccess, value: &'ex Variant,) -> Self {
        let full_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, value: CowArg::Borrowed(value), full_objects: full_objects,
        }
    }
    #[inline]
    pub fn full_objects(self, full_objects: bool) -> Self {
        Self {
            full_objects: full_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, value, full_objects,
        }
        = self;
        re_export::FileAccess::store_var_full(surround_object, value.cow_as_arg(), full_objects,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ModeFlags {
    ord: u64
}
impl ModeFlags {
    pub const READ: ModeFlags = ModeFlags {
        ord: 1u64
    };
    pub const WRITE: ModeFlags = ModeFlags {
        ord: 2u64
    };
    pub const READ_WRITE: ModeFlags = ModeFlags {
        ord: 3u64
    };
    pub const WRITE_READ: ModeFlags = ModeFlags {
        ord: 7u64
    };
    
}
impl std::fmt::Debug for ModeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for ModeFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ModeFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("READ", "READ", ModeFlags::READ), crate::meta::inspect::EnumConstant::new("WRITE", "WRITE", ModeFlags::WRITE), crate::meta::inspect::EnumConstant::new("READ_WRITE", "READ_WRITE", ModeFlags::READ_WRITE), crate::meta::inspect::EnumConstant::new("WRITE_READ", "WRITE_READ", ModeFlags::WRITE_READ)]
        }
    }
}
impl std::ops::BitOr for ModeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for ModeFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for ModeFlags {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Read", 1i64), EnumeratorShape::new_int("Write", 2i64), EnumeratorShape::new_int("Read Write", 3i64), EnumeratorShape::new_int("Write Read", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("FileAccess.ModeFlags")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for ModeFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ModeFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ModeFlags {
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
impl crate::registry::property::Export for ModeFlags {
    
}
impl crate::meta::Element for ModeFlags {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompressionMode {
    ord: i32
}
impl CompressionMode {
    #[doc(alias = "COMPRESSION_FASTLZ")]
    #[doc = "Godot enumerator name: `COMPRESSION_FASTLZ`"]
    pub const FASTLZ: CompressionMode = CompressionMode {
        ord: 0i32
    };
    #[doc(alias = "COMPRESSION_DEFLATE")]
    #[doc = "Godot enumerator name: `COMPRESSION_DEFLATE`"]
    pub const DEFLATE: CompressionMode = CompressionMode {
        ord: 1i32
    };
    #[doc(alias = "COMPRESSION_ZSTD")]
    #[doc = "Godot enumerator name: `COMPRESSION_ZSTD`"]
    pub const ZSTD: CompressionMode = CompressionMode {
        ord: 2i32
    };
    #[doc(alias = "COMPRESSION_GZIP")]
    #[doc = "Godot enumerator name: `COMPRESSION_GZIP`"]
    pub const GZIP: CompressionMode = CompressionMode {
        ord: 3i32
    };
    #[doc(alias = "COMPRESSION_BROTLI")]
    #[doc = "Godot enumerator name: `COMPRESSION_BROTLI`"]
    pub const BROTLI: CompressionMode = CompressionMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for CompressionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompressionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompressionMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::FASTLZ => "FASTLZ", Self::DEFLATE => "DEFLATE", Self::ZSTD => "ZSTD", Self::GZIP => "GZIP", Self::BROTLI => "BROTLI", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompressionMode::FASTLZ, CompressionMode::DEFLATE, CompressionMode::ZSTD, CompressionMode::GZIP, CompressionMode::BROTLI]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompressionMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FASTLZ", "COMPRESSION_FASTLZ", CompressionMode::FASTLZ), crate::meta::inspect::EnumConstant::new("DEFLATE", "COMPRESSION_DEFLATE", CompressionMode::DEFLATE), crate::meta::inspect::EnumConstant::new("ZSTD", "COMPRESSION_ZSTD", CompressionMode::ZSTD), crate::meta::inspect::EnumConstant::new("GZIP", "COMPRESSION_GZIP", CompressionMode::GZIP), crate::meta::inspect::EnumConstant::new("BROTLI", "COMPRESSION_BROTLI", CompressionMode::BROTLI)]
        }
    }
}
impl crate::meta::GodotConvert for CompressionMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Compression Fastlz", 0i64), EnumeratorShape::new_int("Compression Deflate", 1i64), EnumeratorShape::new_int("Compression Zstd", 2i64), EnumeratorShape::new_int("Compression Gzip", 3i64), EnumeratorShape::new_int("Compression Brotli", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("FileAccess.CompressionMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CompressionMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompressionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CompressionMode {
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
impl crate::registry::property::Export for CompressionMode {
    
}
impl crate::meta::Element for CompressionMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct UnixPermissionFlags {
    ord: u64
}
impl UnixPermissionFlags {
    #[doc(alias = "UNIX_READ_OWNER")]
    #[doc = "Godot enumerator name: `UNIX_READ_OWNER`"]
    pub const READ_OWNER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 256u64
    };
    #[doc(alias = "UNIX_WRITE_OWNER")]
    #[doc = "Godot enumerator name: `UNIX_WRITE_OWNER`"]
    pub const WRITE_OWNER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 128u64
    };
    #[doc(alias = "UNIX_EXECUTE_OWNER")]
    #[doc = "Godot enumerator name: `UNIX_EXECUTE_OWNER`"]
    pub const EXECUTE_OWNER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 64u64
    };
    #[doc(alias = "UNIX_READ_GROUP")]
    #[doc = "Godot enumerator name: `UNIX_READ_GROUP`"]
    pub const READ_GROUP: UnixPermissionFlags = UnixPermissionFlags {
        ord: 32u64
    };
    #[doc(alias = "UNIX_WRITE_GROUP")]
    #[doc = "Godot enumerator name: `UNIX_WRITE_GROUP`"]
    pub const WRITE_GROUP: UnixPermissionFlags = UnixPermissionFlags {
        ord: 16u64
    };
    #[doc(alias = "UNIX_EXECUTE_GROUP")]
    #[doc = "Godot enumerator name: `UNIX_EXECUTE_GROUP`"]
    pub const EXECUTE_GROUP: UnixPermissionFlags = UnixPermissionFlags {
        ord: 8u64
    };
    #[doc(alias = "UNIX_READ_OTHER")]
    #[doc = "Godot enumerator name: `UNIX_READ_OTHER`"]
    pub const READ_OTHER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 4u64
    };
    #[doc(alias = "UNIX_WRITE_OTHER")]
    #[doc = "Godot enumerator name: `UNIX_WRITE_OTHER`"]
    pub const WRITE_OTHER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 2u64
    };
    #[doc(alias = "UNIX_EXECUTE_OTHER")]
    #[doc = "Godot enumerator name: `UNIX_EXECUTE_OTHER`"]
    pub const EXECUTE_OTHER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 1u64
    };
    #[doc(alias = "UNIX_SET_USER_ID")]
    #[doc = "Godot enumerator name: `UNIX_SET_USER_ID`"]
    pub const SET_USER_ID: UnixPermissionFlags = UnixPermissionFlags {
        ord: 2048u64
    };
    #[doc(alias = "UNIX_SET_GROUP_ID")]
    #[doc = "Godot enumerator name: `UNIX_SET_GROUP_ID`"]
    pub const SET_GROUP_ID: UnixPermissionFlags = UnixPermissionFlags {
        ord: 1024u64
    };
    #[doc(alias = "UNIX_RESTRICTED_DELETE")]
    #[doc = "Godot enumerator name: `UNIX_RESTRICTED_DELETE`"]
    pub const RESTRICTED_DELETE: UnixPermissionFlags = UnixPermissionFlags {
        ord: 512u64
    };
    
}
impl std::fmt::Debug for UnixPermissionFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for UnixPermissionFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < UnixPermissionFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("READ_OWNER", "UNIX_READ_OWNER", UnixPermissionFlags::READ_OWNER), crate::meta::inspect::EnumConstant::new("WRITE_OWNER", "UNIX_WRITE_OWNER", UnixPermissionFlags::WRITE_OWNER), crate::meta::inspect::EnumConstant::new("EXECUTE_OWNER", "UNIX_EXECUTE_OWNER", UnixPermissionFlags::EXECUTE_OWNER), crate::meta::inspect::EnumConstant::new("READ_GROUP", "UNIX_READ_GROUP", UnixPermissionFlags::READ_GROUP), crate::meta::inspect::EnumConstant::new("WRITE_GROUP", "UNIX_WRITE_GROUP", UnixPermissionFlags::WRITE_GROUP), crate::meta::inspect::EnumConstant::new("EXECUTE_GROUP", "UNIX_EXECUTE_GROUP", UnixPermissionFlags::EXECUTE_GROUP), crate::meta::inspect::EnumConstant::new("READ_OTHER", "UNIX_READ_OTHER", UnixPermissionFlags::READ_OTHER), crate::meta::inspect::EnumConstant::new("WRITE_OTHER", "UNIX_WRITE_OTHER", UnixPermissionFlags::WRITE_OTHER), crate::meta::inspect::EnumConstant::new("EXECUTE_OTHER", "UNIX_EXECUTE_OTHER", UnixPermissionFlags::EXECUTE_OTHER), crate::meta::inspect::EnumConstant::new("SET_USER_ID", "UNIX_SET_USER_ID", UnixPermissionFlags::SET_USER_ID), crate::meta::inspect::EnumConstant::new("SET_GROUP_ID", "UNIX_SET_GROUP_ID", UnixPermissionFlags::SET_GROUP_ID), crate::meta::inspect::EnumConstant::new("RESTRICTED_DELETE", "UNIX_RESTRICTED_DELETE", UnixPermissionFlags::RESTRICTED_DELETE)]
        }
    }
}
impl std::ops::BitOr for UnixPermissionFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for UnixPermissionFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for UnixPermissionFlags {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Unix Read Owner", 256i64), EnumeratorShape::new_int("Unix Write Owner", 128i64), EnumeratorShape::new_int("Unix Execute Owner", 64i64), EnumeratorShape::new_int("Unix Read Group", 32i64), EnumeratorShape::new_int("Unix Write Group", 16i64), EnumeratorShape::new_int("Unix Execute Group", 8i64), EnumeratorShape::new_int("Unix Read Other", 4i64), EnumeratorShape::new_int("Unix Write Other", 2i64), EnumeratorShape::new_int("Unix Execute Other", 1i64), EnumeratorShape::new_int("Unix Set User Id", 2048i64), EnumeratorShape::new_int("Unix Set Group Id", 1024i64), EnumeratorShape::new_int("Unix Restricted Delete", 512i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("FileAccess.UnixPermissionFlags")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for UnixPermissionFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UnixPermissionFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for UnixPermissionFlags {
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
impl crate::registry::property::Export for UnixPermissionFlags {
    
}
impl crate::meta::Element for UnixPermissionFlags {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::FileAccess;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for FileAccess {
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
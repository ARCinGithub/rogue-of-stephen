#![doc = "Sidecar module for class [`Os`][crate::classes::Os].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OS` enums](https://docs.godotengine.org/en/stable/classes/class_os.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OS`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`os`][crate::classes::os]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `OS`](https://docs.godotengine.org/en/stable/classes/class_os.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThe `OS` class wraps the most common functionalities for communicating with the host operating system, such as the video driver, delays, environment variables, execution of binaries, command line, etc.\n\n**Note:** In Godot 4, `OS` functions related to window management, clipboard, and TTS were moved to the [`DisplayServer`][crate::classes::DisplayServer] singleton (and the [`Window`][crate::classes::Window] class). Functions related to time were removed and are only available in the [`Time`][crate::classes::Time] class."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Os {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Os {
        #[doc = "Generates a [`PackedByteArray`][crate::builtin::PackedByteArray] of cryptographically secure random bytes with given `size`.\n\n**Note:** Generating large quantities of bytes using this method can result in locking and entropy of lower quality on most platforms. Using [`generate_random_bytes`][`crate::classes::Crypto::generate_random_bytes`] is preferred in most cases."]
        pub fn get_entropy(&self, size: i32,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(94usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_entropy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of certification authorities trusted by the operating system as a string of concatenated certificates in PEM format."]
        pub fn get_system_ca_certificates(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(95usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_system_ca_certificates", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of connected MIDI device names, if they exist. Returns an empty array if the system MIDI driver has not previously been initialized with [`open_midi_inputs`][`crate::classes::Os::open_midi_inputs`]. See also [`close_midi_inputs`][`crate::classes::Os::close_midi_inputs`].\n\n**Note:** This method is implemented on Linux, macOS, Windows, and Web.\n\n**Note:** On the Web platform, Web MIDI needs to be supported by the browser. [For the time being](https://caniuse.com/midi), it is currently supported by all major browsers, except Safari.\n\n**Note:** On the Web platform, using MIDI input requires a browser permission to be granted first. This permission request is performed when calling [`open_midi_inputs`][`crate::classes::Os::open_midi_inputs`]. The browser will refrain from processing MIDI input until the user accepts the permission request."]
        pub fn get_connected_midi_inputs(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(96usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_connected_midi_inputs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Initializes the singleton for the system MIDI driver, allowing Godot to receive [`InputEventMIDI`][crate::classes::InputEventMidi]. See also [`get_connected_midi_inputs`][`crate::classes::Os::get_connected_midi_inputs`] and [`close_midi_inputs`][`crate::classes::Os::close_midi_inputs`].\n\n**Note:** This method is implemented on Linux, macOS, Windows, and Web.\n\n**Note:** On the Web platform, Web MIDI needs to be supported by the browser. [For the time being](https://caniuse.com/midi), it is currently supported by all major browsers, except Safari.\n\n**Note:** On the Web platform, using MIDI input requires a browser permission to be granted first. This permission request is performed when calling [`open_midi_inputs`][`crate::classes::Os::open_midi_inputs`]. The browser will refrain from processing MIDI input until the user accepts the permission request."]
        pub fn open_midi_inputs(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(97usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "open_midi_inputs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Shuts down the system MIDI driver. Godot will no longer receive [`InputEventMIDI`][crate::classes::InputEventMidi]. See also [`open_midi_inputs`][`crate::classes::Os::open_midi_inputs`] and [`get_connected_midi_inputs`][`crate::classes::Os::get_connected_midi_inputs`].\n\n**Note:** This method is implemented on Linux, macOS, Windows, and Web."]
        pub fn close_midi_inputs(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(98usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "close_midi_inputs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Displays a modal dialog box using the host platform's implementation. The engine execution is blocked until the dialog is closed."]
        pub(crate) fn alert_full(&mut self, text: CowArg < GString >, title: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (text, title,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(99usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "alert", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`alert_ex`][Self::alert_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Displays a modal dialog box using the host platform's implementation. The engine execution is blocked until the dialog is closed."]
        #[inline]
        pub fn alert(&mut self, text: impl AsArg < GString >,) {
            self.alert_ex(text,) . done()
        }
        #[doc = "Displays a modal dialog box using the host platform's implementation. The engine execution is blocked until the dialog is closed."]
        #[inline]
        pub fn alert_ex < 'ex > (&'ex mut self, text: impl AsArg < GString > + 'ex,) -> ExAlert < 'ex > {
            ExAlert::new(self, text,)
        }
        #[doc = "Crashes the engine (or the editor if called within a `@tool` script). See also [`kill`][`crate::classes::Os::kill`].\n\n**Note:** This method should _only_ be used for testing the system's crash handler, not for any other purpose. For general error reporting, use (in order of preference) \\[method @GDScript.assert], [`push_error`][`crate::global::push_error`], or [`alert`][`crate::classes::Os::alert`]."]
        pub fn crash(&mut self, message: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (message.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "crash", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_low_processor_usage_mode(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_low_processor_usage_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_in_low_processor_usage_mode(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_in_low_processor_usage_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_low_processor_usage_mode_sleep_usec(&mut self, usec: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (usec,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_low_processor_usage_mode_sleep_usec", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_low_processor_usage_mode_sleep_usec(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_low_processor_usage_mode_sleep_usec", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_delta_smoothing(&mut self, delta_smoothing_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (delta_smoothing_enabled,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_delta_smoothing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_delta_smoothing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_delta_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of _logical_ CPU cores available on the host machine. On CPUs with HyperThreading enabled, this number will be greater than the number of _physical_ CPU cores."]
        pub fn get_processor_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_processor_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the full name of the CPU model on the host machine (e.g. `\"Intel(R) Core(TM) i7-6700K CPU @ 4.00GHz\"`).\n\n**Note:** This method is only implemented on Windows, macOS, Linux and iOS. On Android and Web, [`get_processor_name`][`crate::classes::Os::get_processor_name`] returns an empty string."]
        pub fn get_processor_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_processor_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of font family names available.\n\n**Note:** This method is implemented on Android, iOS, Linux, macOS and Windows."]
        pub fn get_system_fonts(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_system_fonts", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the path to the system font file with `font_name` and style. Returns an empty string if no matching fonts found.\n\nThe following aliases can be used to request default fonts: \"sans-serif\", \"serif\", \"monospace\", \"cursive\", and \"fantasy\".\n\n**Note:** Returned font might have different style if the requested style is not available.\n\n**Note:** This method is implemented on Android, iOS, Linux, macOS and Windows."]
        pub(crate) fn get_system_font_path_full(&self, font_name: CowArg < GString >, weight: i32, stretch: i32, italic: bool,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32, bool,);
            let args = (font_name, weight, stretch, italic,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_system_font_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_system_font_path_ex`][Self::get_system_font_path_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the path to the system font file with `font_name` and style. Returns an empty string if no matching fonts found.\n\nThe following aliases can be used to request default fonts: \"sans-serif\", \"serif\", \"monospace\", \"cursive\", and \"fantasy\".\n\n**Note:** Returned font might have different style if the requested style is not available.\n\n**Note:** This method is implemented on Android, iOS, Linux, macOS and Windows."]
        #[inline]
        pub fn get_system_font_path(&self, font_name: impl AsArg < GString >,) -> GString {
            self.get_system_font_path_ex(font_name,) . done()
        }
        #[doc = "Returns the path to the system font file with `font_name` and style. Returns an empty string if no matching fonts found.\n\nThe following aliases can be used to request default fonts: \"sans-serif\", \"serif\", \"monospace\", \"cursive\", and \"fantasy\".\n\n**Note:** Returned font might have different style if the requested style is not available.\n\n**Note:** This method is implemented on Android, iOS, Linux, macOS and Windows."]
        #[inline]
        pub fn get_system_font_path_ex < 'ex > (&'ex self, font_name: impl AsArg < GString > + 'ex,) -> ExGetSystemFontPath < 'ex > {
            ExGetSystemFontPath::new(self, font_name,)
        }
        #[doc = "Returns an array of the system substitute font file paths, which are similar to the font with `font_name` and style for the specified text, locale, and script. Returns an empty array if no matching fonts found.\n\nThe following aliases can be used to request default fonts: \"sans-serif\", \"serif\", \"monospace\", \"cursive\", and \"fantasy\".\n\n**Note:** Depending on OS, it's not guaranteed that any of the returned fonts will be suitable for rendering specified text. Fonts should be loaded and checked in the order they are returned, and the first suitable one used.\n\n**Note:** Returned fonts might have different style if the requested style is not available or belong to a different font family.\n\n**Note:** This method is implemented on Android, iOS, Linux, macOS and Windows."]
        pub(crate) fn get_system_font_path_for_text_full(&self, font_name: CowArg < GString >, text: CowArg < GString >, locale: CowArg < GString >, script: CowArg < GString >, weight: i32, stretch: i32, italic: bool,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, CowArg < 'a3, GString >, i32, i32, bool,);
            let args = (font_name, text, locale, script, weight, stretch, italic,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_system_font_path_for_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_system_font_path_for_text_ex`][Self::get_system_font_path_for_text_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an array of the system substitute font file paths, which are similar to the font with `font_name` and style for the specified text, locale, and script. Returns an empty array if no matching fonts found.\n\nThe following aliases can be used to request default fonts: \"sans-serif\", \"serif\", \"monospace\", \"cursive\", and \"fantasy\".\n\n**Note:** Depending on OS, it's not guaranteed that any of the returned fonts will be suitable for rendering specified text. Fonts should be loaded and checked in the order they are returned, and the first suitable one used.\n\n**Note:** Returned fonts might have different style if the requested style is not available or belong to a different font family.\n\n**Note:** This method is implemented on Android, iOS, Linux, macOS and Windows."]
        #[inline]
        pub fn get_system_font_path_for_text(&self, font_name: impl AsArg < GString >, text: impl AsArg < GString >,) -> PackedStringArray {
            self.get_system_font_path_for_text_ex(font_name, text,) . done()
        }
        #[doc = "Returns an array of the system substitute font file paths, which are similar to the font with `font_name` and style for the specified text, locale, and script. Returns an empty array if no matching fonts found.\n\nThe following aliases can be used to request default fonts: \"sans-serif\", \"serif\", \"monospace\", \"cursive\", and \"fantasy\".\n\n**Note:** Depending on OS, it's not guaranteed that any of the returned fonts will be suitable for rendering specified text. Fonts should be loaded and checked in the order they are returned, and the first suitable one used.\n\n**Note:** Returned fonts might have different style if the requested style is not available or belong to a different font family.\n\n**Note:** This method is implemented on Android, iOS, Linux, macOS and Windows."]
        #[inline]
        pub fn get_system_font_path_for_text_ex < 'ex > (&'ex self, font_name: impl AsArg < GString > + 'ex, text: impl AsArg < GString > + 'ex,) -> ExGetSystemFontPathForText < 'ex > {
            ExGetSystemFontPathForText::new(self, font_name, text,)
        }
        #[doc = "Returns the file path to the current engine executable.\n\n**Note:** On macOS, if you want to launch another instance of Godot, always use [`create_instance`][`crate::classes::Os::create_instance`] instead of relying on the executable path."]
        pub fn get_executable_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_executable_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Reads a user input as a UTF-8 encoded string from the standard input. This operation can be _blocking_, which causes the window to freeze if [`read_string_from_stdin`][`crate::classes::Os::read_string_from_stdin`] is called on the main thread.\n\n- If standard input is console, this method will block until the program receives a line break in standard input (usually by the user pressing `Enter`).\n\n- If standard input is pipe, this method will block until a specific amount of data is read or pipe is closed.\n\n- If standard input is a file, this method will read a specific amount of data (or less if end-of-file is reached) and return immediately.\n\n**Note:** This method automatically replaces `\\r\\n` line breaks with `\\n` and removes them from the end of the string. Use [`read_buffer_from_stdin`][`crate::classes::Os::read_buffer_from_stdin`] to read the unprocessed data.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** On exported Windows builds, run the console wrapper executable to access the terminal. If standard input is console, calling this method without console wrapped will freeze permanently. If standard input is pipe or file, it can be used without console wrapper. If you need a single executable with full console support, use a custom build compiled with the `windows_subsystem=console` flag."]
        pub(crate) fn read_string_from_stdin_full(&mut self, buffer_size: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "read_string_from_stdin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`read_string_from_stdin_ex`][Self::read_string_from_stdin_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Reads a user input as a UTF-8 encoded string from the standard input. This operation can be _blocking_, which causes the window to freeze if [`read_string_from_stdin`][`crate::classes::Os::read_string_from_stdin`] is called on the main thread.\n\n- If standard input is console, this method will block until the program receives a line break in standard input (usually by the user pressing `Enter`).\n\n- If standard input is pipe, this method will block until a specific amount of data is read or pipe is closed.\n\n- If standard input is a file, this method will read a specific amount of data (or less if end-of-file is reached) and return immediately.\n\n**Note:** This method automatically replaces `\\r\\n` line breaks with `\\n` and removes them from the end of the string. Use [`read_buffer_from_stdin`][`crate::classes::Os::read_buffer_from_stdin`] to read the unprocessed data.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** On exported Windows builds, run the console wrapper executable to access the terminal. If standard input is console, calling this method without console wrapped will freeze permanently. If standard input is pipe or file, it can be used without console wrapper. If you need a single executable with full console support, use a custom build compiled with the `windows_subsystem=console` flag."]
        #[inline]
        pub fn read_string_from_stdin(&mut self,) -> GString {
            self.read_string_from_stdin_ex() . done()
        }
        #[doc = "Reads a user input as a UTF-8 encoded string from the standard input. This operation can be _blocking_, which causes the window to freeze if [`read_string_from_stdin`][`crate::classes::Os::read_string_from_stdin`] is called on the main thread.\n\n- If standard input is console, this method will block until the program receives a line break in standard input (usually by the user pressing `Enter`).\n\n- If standard input is pipe, this method will block until a specific amount of data is read or pipe is closed.\n\n- If standard input is a file, this method will read a specific amount of data (or less if end-of-file is reached) and return immediately.\n\n**Note:** This method automatically replaces `\\r\\n` line breaks with `\\n` and removes them from the end of the string. Use [`read_buffer_from_stdin`][`crate::classes::Os::read_buffer_from_stdin`] to read the unprocessed data.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** On exported Windows builds, run the console wrapper executable to access the terminal. If standard input is console, calling this method without console wrapped will freeze permanently. If standard input is pipe or file, it can be used without console wrapper. If you need a single executable with full console support, use a custom build compiled with the `windows_subsystem=console` flag."]
        #[inline]
        pub fn read_string_from_stdin_ex < 'ex > (&'ex mut self,) -> ExReadStringFromStdin < 'ex > {
            ExReadStringFromStdin::new(self,)
        }
        #[doc = "Reads a user input as raw data from the standard input. This operation can be _blocking_, which causes the window to freeze if [`read_buffer_from_stdin`][`crate::classes::Os::read_buffer_from_stdin`] is called on the main thread.\n\n- If standard input is console, this method will block until the program receives a line break in standard input (usually by the user pressing `Enter`).\n\n- If standard input is pipe, this method will block until a specific amount of data is read or pipe is closed.\n\n- If standard input is a file, this method will read a specific amount of data (or less if end-of-file is reached) and return immediately.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** On exported Windows builds, run the console wrapper executable to access the terminal. If standard input is console, calling this method without console wrapped will freeze permanently. If standard input is pipe or file, it can be used without console wrapper. If you need a single executable with full console support, use a custom build compiled with the `windows_subsystem=console` flag."]
        pub(crate) fn read_buffer_from_stdin_full(&mut self, buffer_size: i64,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (i64,);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "read_buffer_from_stdin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`read_buffer_from_stdin_ex`][Self::read_buffer_from_stdin_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Reads a user input as raw data from the standard input. This operation can be _blocking_, which causes the window to freeze if [`read_buffer_from_stdin`][`crate::classes::Os::read_buffer_from_stdin`] is called on the main thread.\n\n- If standard input is console, this method will block until the program receives a line break in standard input (usually by the user pressing `Enter`).\n\n- If standard input is pipe, this method will block until a specific amount of data is read or pipe is closed.\n\n- If standard input is a file, this method will read a specific amount of data (or less if end-of-file is reached) and return immediately.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** On exported Windows builds, run the console wrapper executable to access the terminal. If standard input is console, calling this method without console wrapped will freeze permanently. If standard input is pipe or file, it can be used without console wrapper. If you need a single executable with full console support, use a custom build compiled with the `windows_subsystem=console` flag."]
        #[inline]
        pub fn read_buffer_from_stdin(&mut self,) -> PackedByteArray {
            self.read_buffer_from_stdin_ex() . done()
        }
        #[doc = "Reads a user input as raw data from the standard input. This operation can be _blocking_, which causes the window to freeze if [`read_buffer_from_stdin`][`crate::classes::Os::read_buffer_from_stdin`] is called on the main thread.\n\n- If standard input is console, this method will block until the program receives a line break in standard input (usually by the user pressing `Enter`).\n\n- If standard input is pipe, this method will block until a specific amount of data is read or pipe is closed.\n\n- If standard input is a file, this method will read a specific amount of data (or less if end-of-file is reached) and return immediately.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** On exported Windows builds, run the console wrapper executable to access the terminal. If standard input is console, calling this method without console wrapped will freeze permanently. If standard input is pipe or file, it can be used without console wrapper. If you need a single executable with full console support, use a custom build compiled with the `windows_subsystem=console` flag."]
        #[inline]
        pub fn read_buffer_from_stdin_ex < 'ex > (&'ex mut self,) -> ExReadBufferFromStdin < 'ex > {
            ExReadBufferFromStdin::new(self,)
        }
        #[doc = "Returns the type of the standard input device.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** On exported Windows builds, run the console wrapper executable to access the standard input. If you need a single executable with full console support, use a custom build compiled with the `windows_subsystem=console` flag."]
        pub fn get_stdin_type(&self,) -> crate::classes::os::StdHandleType {
            type CallRet = crate::classes::os::StdHandleType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_stdin_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the type of the standard output device.\n\n**Note:** This method is implemented on Linux, macOS, and Windows."]
        pub fn get_stdout_type(&self,) -> crate::classes::os::StdHandleType {
            type CallRet = crate::classes::os::StdHandleType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_stdout_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the type of the standard error device.\n\n**Note:** This method is implemented on Linux, macOS, and Windows."]
        pub fn get_stderr_type(&self,) -> crate::classes::os::StdHandleType {
            type CallRet = crate::classes::os::StdHandleType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_stderr_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Executes the given process in a _blocking_ way. The file specified in `path` must exist and be executable. The system path resolution will be used. The `arguments` are used in the given order, separated by spaces, and wrapped in quotes.\n\nIf an `output` array is provided, the complete shell output of the process is appended to `output` as a single [`String`][crate::builtin::GString] element. If `read_stderr` is `true`, the output to the standard error stream is also appended to the array.\n\nOn Windows, if `open_console` is `true` and the process is a console app, a new terminal window is opened.\n\nThis method returns the exit code of the command, or `-1` if the process fails to execute.\n\n**Note:** The main thread will be blocked until the executed command terminates. Use `Thread` to create a separate thread that will not block the main thread, or use [`create_process`][`crate::classes::Os::create_process`] to create a completely independent process.\n\nFor example, to retrieve a list of the working directory's contents:\n\n\n```gdscript\nvar output = []\nvar exit_code = OS.execute(\"ls\", [\"-l\", \"/tmp\"], output)\n```\n\n\nIf you wish to access a shell built-in or execute a composite command, a platform-specific shell can be invoked. For example:\n\n\n```gdscript\nvar output = []\nOS.execute(\"CMD.exe\", [\"/C\", \"cd %TEMP% && dir\"], output)\n```\n\n\n**Note:** This method is implemented on Android, Linux, macOS, and Windows.\n\n**Note:** To execute a Windows command interpreter built-in command, specify `cmd.exe` in `path`, `/c` as the first argument, and the desired command as the second argument.\n\n**Note:** To execute a PowerShell built-in command, specify `powershell.exe` in `path`, `-Command` as the first argument, and the desired command as the second argument.\n\n**Note:** To execute a Unix shell built-in command, specify shell executable name in `path`, `-c` as the first argument, and the desired command as the second argument.\n\n**Note:** On macOS, sandboxed applications are limited to run only embedded helper executables, specified during export.\n\n**Note:** On Android, system commands such as `dumpsys` can only be run on a rooted device."]
        pub(crate) fn execute_full(&mut self, path: CowArg < GString >, arguments: RefArg < PackedStringArray >, output: RefArg < AnyArray >, read_stderr: bool, open_console: bool,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, RefArg < 'a2, AnyArray >, bool, bool,);
            let args = (path, arguments, output, read_stderr, open_console,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "execute", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`execute_ex`][Self::execute_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Executes the given process in a _blocking_ way. The file specified in `path` must exist and be executable. The system path resolution will be used. The `arguments` are used in the given order, separated by spaces, and wrapped in quotes.\n\nIf an `output` array is provided, the complete shell output of the process is appended to `output` as a single [`String`][crate::builtin::GString] element. If `read_stderr` is `true`, the output to the standard error stream is also appended to the array.\n\nOn Windows, if `open_console` is `true` and the process is a console app, a new terminal window is opened.\n\nThis method returns the exit code of the command, or `-1` if the process fails to execute.\n\n**Note:** The main thread will be blocked until the executed command terminates. Use `Thread` to create a separate thread that will not block the main thread, or use [`create_process`][`crate::classes::Os::create_process`] to create a completely independent process.\n\nFor example, to retrieve a list of the working directory's contents:\n\n\n```gdscript\nvar output = []\nvar exit_code = OS.execute(\"ls\", [\"-l\", \"/tmp\"], output)\n```\n\n\nIf you wish to access a shell built-in or execute a composite command, a platform-specific shell can be invoked. For example:\n\n\n```gdscript\nvar output = []\nOS.execute(\"CMD.exe\", [\"/C\", \"cd %TEMP% && dir\"], output)\n```\n\n\n**Note:** This method is implemented on Android, Linux, macOS, and Windows.\n\n**Note:** To execute a Windows command interpreter built-in command, specify `cmd.exe` in `path`, `/c` as the first argument, and the desired command as the second argument.\n\n**Note:** To execute a PowerShell built-in command, specify `powershell.exe` in `path`, `-Command` as the first argument, and the desired command as the second argument.\n\n**Note:** To execute a Unix shell built-in command, specify shell executable name in `path`, `-c` as the first argument, and the desired command as the second argument.\n\n**Note:** On macOS, sandboxed applications are limited to run only embedded helper executables, specified during export.\n\n**Note:** On Android, system commands such as `dumpsys` can only be run on a rooted device."]
        #[inline]
        pub fn execute(&mut self, path: impl AsArg < GString >, arguments: &PackedStringArray,) -> i32 {
            self.execute_ex(path, arguments,) . done()
        }
        #[doc = "Executes the given process in a _blocking_ way. The file specified in `path` must exist and be executable. The system path resolution will be used. The `arguments` are used in the given order, separated by spaces, and wrapped in quotes.\n\nIf an `output` array is provided, the complete shell output of the process is appended to `output` as a single [`String`][crate::builtin::GString] element. If `read_stderr` is `true`, the output to the standard error stream is also appended to the array.\n\nOn Windows, if `open_console` is `true` and the process is a console app, a new terminal window is opened.\n\nThis method returns the exit code of the command, or `-1` if the process fails to execute.\n\n**Note:** The main thread will be blocked until the executed command terminates. Use `Thread` to create a separate thread that will not block the main thread, or use [`create_process`][`crate::classes::Os::create_process`] to create a completely independent process.\n\nFor example, to retrieve a list of the working directory's contents:\n\n\n```gdscript\nvar output = []\nvar exit_code = OS.execute(\"ls\", [\"-l\", \"/tmp\"], output)\n```\n\n\nIf you wish to access a shell built-in or execute a composite command, a platform-specific shell can be invoked. For example:\n\n\n```gdscript\nvar output = []\nOS.execute(\"CMD.exe\", [\"/C\", \"cd %TEMP% && dir\"], output)\n```\n\n\n**Note:** This method is implemented on Android, Linux, macOS, and Windows.\n\n**Note:** To execute a Windows command interpreter built-in command, specify `cmd.exe` in `path`, `/c` as the first argument, and the desired command as the second argument.\n\n**Note:** To execute a PowerShell built-in command, specify `powershell.exe` in `path`, `-Command` as the first argument, and the desired command as the second argument.\n\n**Note:** To execute a Unix shell built-in command, specify shell executable name in `path`, `-c` as the first argument, and the desired command as the second argument.\n\n**Note:** On macOS, sandboxed applications are limited to run only embedded helper executables, specified during export.\n\n**Note:** On Android, system commands such as `dumpsys` can only be run on a rooted device."]
        #[inline]
        pub fn execute_ex < 'ex > (&'ex mut self, path: impl AsArg < GString > + 'ex, arguments: &'ex PackedStringArray,) -> ExExecute < 'ex > {
            ExExecute::new(self, path, arguments,)
        }
        #[doc = "Creates a new process that runs independently of Godot with redirected IO. It will not terminate when Godot terminates. The path specified in `path` must exist and be an executable file or macOS `.app` bundle. The path is resolved based on the current platform. The `arguments` are used in the given order and separated by a space.\n\nIf `blocking` is `false`, created pipes work in non-blocking mode, i.e. read and write operations will return immediately. Use [`get_error`][`crate::classes::FileAccess::get_error`] to check if the last read/write operation was successful.\n\nIf the process cannot be created, this method returns an empty [`Dictionary`][crate::builtin::Dictionary]. Otherwise, this method returns a [`Dictionary`][crate::builtin::Dictionary] with the following keys:\n\n- `\"stdio\"` - [`FileAccess`][crate::classes::FileAccess] to access the process stdin and stdout pipes (read/write).\n\n- `\"stderr\"` - [`FileAccess`][crate::classes::FileAccess] to access the process stderr pipe (read only).\n\n- `\"pid\"` - Process ID as an `int`, which you can use to monitor the process (and potentially terminate it with [`kill`][`crate::classes::Os::kill`]).\n\n**Note:** This method is implemented on Android, Linux, macOS, and Windows.\n\n**Note:** To execute a Windows command interpreter built-in command, specify `cmd.exe` in `path`, `/c` as the first argument, and the desired command as the second argument.\n\n**Note:** To execute a PowerShell built-in command, specify `powershell.exe` in `path`, `-Command` as the first argument, and the desired command as the second argument.\n\n**Note:** To execute a Unix shell built-in command, specify shell executable name in `path`, `-c` as the first argument, and the desired command as the second argument.\n\n**Note:** On macOS, sandboxed applications are limited to run only embedded helper executables, specified during export or system .app bundle, system .app bundles will ignore arguments."]
        pub(crate) fn execute_with_pipe_full(&mut self, path: CowArg < GString >, arguments: RefArg < PackedStringArray >, blocking: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, bool,);
            let args = (path, arguments, blocking,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "execute_with_pipe", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`execute_with_pipe_ex`][Self::execute_with_pipe_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new process that runs independently of Godot with redirected IO. It will not terminate when Godot terminates. The path specified in `path` must exist and be an executable file or macOS `.app` bundle. The path is resolved based on the current platform. The `arguments` are used in the given order and separated by a space.\n\nIf `blocking` is `false`, created pipes work in non-blocking mode, i.e. read and write operations will return immediately. Use [`get_error`][`crate::classes::FileAccess::get_error`] to check if the last read/write operation was successful.\n\nIf the process cannot be created, this method returns an empty [`Dictionary`][crate::builtin::Dictionary]. Otherwise, this method returns a [`Dictionary`][crate::builtin::Dictionary] with the following keys:\n\n- `\"stdio\"` - [`FileAccess`][crate::classes::FileAccess] to access the process stdin and stdout pipes (read/write).\n\n- `\"stderr\"` - [`FileAccess`][crate::classes::FileAccess] to access the process stderr pipe (read only).\n\n- `\"pid\"` - Process ID as an `int`, which you can use to monitor the process (and potentially terminate it with [`kill`][`crate::classes::Os::kill`]).\n\n**Note:** This method is implemented on Android, Linux, macOS, and Windows.\n\n**Note:** To execute a Windows command interpreter built-in command, specify `cmd.exe` in `path`, `/c` as the first argument, and the desired command as the second argument.\n\n**Note:** To execute a PowerShell built-in command, specify `powershell.exe` in `path`, `-Command` as the first argument, and the desired command as the second argument.\n\n**Note:** To execute a Unix shell built-in command, specify shell executable name in `path`, `-c` as the first argument, and the desired command as the second argument.\n\n**Note:** On macOS, sandboxed applications are limited to run only embedded helper executables, specified during export or system .app bundle, system .app bundles will ignore arguments."]
        #[inline]
        pub fn execute_with_pipe(&mut self, path: impl AsArg < GString >, arguments: &PackedStringArray,) -> VarDictionary {
            self.execute_with_pipe_ex(path, arguments,) . done()
        }
        #[doc = "Creates a new process that runs independently of Godot with redirected IO. It will not terminate when Godot terminates. The path specified in `path` must exist and be an executable file or macOS `.app` bundle. The path is resolved based on the current platform. The `arguments` are used in the given order and separated by a space.\n\nIf `blocking` is `false`, created pipes work in non-blocking mode, i.e. read and write operations will return immediately. Use [`get_error`][`crate::classes::FileAccess::get_error`] to check if the last read/write operation was successful.\n\nIf the process cannot be created, this method returns an empty [`Dictionary`][crate::builtin::Dictionary]. Otherwise, this method returns a [`Dictionary`][crate::builtin::Dictionary] with the following keys:\n\n- `\"stdio\"` - [`FileAccess`][crate::classes::FileAccess] to access the process stdin and stdout pipes (read/write).\n\n- `\"stderr\"` - [`FileAccess`][crate::classes::FileAccess] to access the process stderr pipe (read only).\n\n- `\"pid\"` - Process ID as an `int`, which you can use to monitor the process (and potentially terminate it with [`kill`][`crate::classes::Os::kill`]).\n\n**Note:** This method is implemented on Android, Linux, macOS, and Windows.\n\n**Note:** To execute a Windows command interpreter built-in command, specify `cmd.exe` in `path`, `/c` as the first argument, and the desired command as the second argument.\n\n**Note:** To execute a PowerShell built-in command, specify `powershell.exe` in `path`, `-Command` as the first argument, and the desired command as the second argument.\n\n**Note:** To execute a Unix shell built-in command, specify shell executable name in `path`, `-c` as the first argument, and the desired command as the second argument.\n\n**Note:** On macOS, sandboxed applications are limited to run only embedded helper executables, specified during export or system .app bundle, system .app bundles will ignore arguments."]
        #[inline]
        pub fn execute_with_pipe_ex < 'ex > (&'ex mut self, path: impl AsArg < GString > + 'ex, arguments: &'ex PackedStringArray,) -> ExExecuteWithPipe < 'ex > {
            ExExecuteWithPipe::new(self, path, arguments,)
        }
        #[doc = "Creates a new process that runs independently of Godot. It will not terminate when Godot terminates. The path specified in `path` must exist and be an executable file or macOS `.app` bundle. The path is resolved based on the current platform. The `arguments` are used in the given order and separated by a space.\n\nOn Windows, if `open_console` is `true` and the process is a console app, a new terminal window will be opened.\n\nIf the process is successfully created, this method returns its process ID, which you can use to monitor the process (and potentially terminate it with [`kill`][`crate::classes::Os::kill`]). Otherwise, this method returns `-1`.\n\n**Example:** Run another instance of the project:\n\n\n```gdscript\nvar pid = OS.create_process(OS.get_executable_path(), [])\n```\n\n\nSee [`execute`][`crate::classes::Os::execute`] if you wish to run an external command and retrieve the results.\n\n**Note:** This method is implemented on Android, Linux, macOS, and Windows.\n\n**Note:** On macOS, sandboxed applications are limited to run only embedded helper executables, specified during export or system .app bundle, system .app bundles will ignore arguments."]
        pub(crate) fn create_process_full(&mut self, path: CowArg < GString >, arguments: RefArg < PackedStringArray >, open_console: bool,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, bool,);
            let args = (path, arguments, open_console,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "create_process", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_process_ex`][Self::create_process_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new process that runs independently of Godot. It will not terminate when Godot terminates. The path specified in `path` must exist and be an executable file or macOS `.app` bundle. The path is resolved based on the current platform. The `arguments` are used in the given order and separated by a space.\n\nOn Windows, if `open_console` is `true` and the process is a console app, a new terminal window will be opened.\n\nIf the process is successfully created, this method returns its process ID, which you can use to monitor the process (and potentially terminate it with [`kill`][`crate::classes::Os::kill`]). Otherwise, this method returns `-1`.\n\n**Example:** Run another instance of the project:\n\n\n```gdscript\nvar pid = OS.create_process(OS.get_executable_path(), [])\n```\n\n\nSee [`execute`][`crate::classes::Os::execute`] if you wish to run an external command and retrieve the results.\n\n**Note:** This method is implemented on Android, Linux, macOS, and Windows.\n\n**Note:** On macOS, sandboxed applications are limited to run only embedded helper executables, specified during export or system .app bundle, system .app bundles will ignore arguments."]
        #[inline]
        pub fn create_process(&mut self, path: impl AsArg < GString >, arguments: &PackedStringArray,) -> i32 {
            self.create_process_ex(path, arguments,) . done()
        }
        #[doc = "Creates a new process that runs independently of Godot. It will not terminate when Godot terminates. The path specified in `path` must exist and be an executable file or macOS `.app` bundle. The path is resolved based on the current platform. The `arguments` are used in the given order and separated by a space.\n\nOn Windows, if `open_console` is `true` and the process is a console app, a new terminal window will be opened.\n\nIf the process is successfully created, this method returns its process ID, which you can use to monitor the process (and potentially terminate it with [`kill`][`crate::classes::Os::kill`]). Otherwise, this method returns `-1`.\n\n**Example:** Run another instance of the project:\n\n\n```gdscript\nvar pid = OS.create_process(OS.get_executable_path(), [])\n```\n\n\nSee [`execute`][`crate::classes::Os::execute`] if you wish to run an external command and retrieve the results.\n\n**Note:** This method is implemented on Android, Linux, macOS, and Windows.\n\n**Note:** On macOS, sandboxed applications are limited to run only embedded helper executables, specified during export or system .app bundle, system .app bundles will ignore arguments."]
        #[inline]
        pub fn create_process_ex < 'ex > (&'ex mut self, path: impl AsArg < GString > + 'ex, arguments: &'ex PackedStringArray,) -> ExCreateProcess < 'ex > {
            ExCreateProcess::new(self, path, arguments,)
        }
        #[doc = "Creates a new instance of Godot that runs independently. The `arguments` are used in the given order and separated by a space.\n\nIf the process is successfully created, this method returns the new process' ID, which you can use to monitor the process (and potentially terminate it with [`kill`][`crate::classes::Os::kill`]). If the process cannot be created, this method returns `-1`.\n\nSee [`create_process`][`crate::classes::Os::create_process`] if you wish to run a different process.\n\n**Note:** This method is implemented on Android, Linux, macOS and Windows."]
        pub fn create_instance(&mut self, arguments: &PackedStringArray,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(arguments),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "create_instance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Opens one or more files/directories with the specified application. The `program_path` specifies the path to the application to use for opening the files, and `paths` contains an array of file/directory paths to open.\n\n**Note:** This method is mostly only relevant for macOS, where opening files using [`create_process`][`crate::classes::Os::create_process`] might fail. On other platforms, this falls back to using [`create_process`][`crate::classes::Os::create_process`].\n\n**Note:** On macOS, `program_path` should ideally be the path to a `.app` bundle."]
        pub fn open_with_program(&mut self, program_path: impl AsArg < GString >, paths: &PackedStringArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >,);
            let args = (program_path.into_arg(), RefArg::new(paths),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "open_with_program", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Kill (terminate) the process identified by the given process ID (`pid`), such as the ID returned by [`execute`][`crate::classes::Os::execute`] in non-blocking mode. See also [`crash`][`crate::classes::Os::crash`].\n\n**Note:** This method can also be used to kill processes that were not spawned by the engine.\n\n**Note:** This method is implemented on Android, iOS, Linux, macOS and Windows."]
        pub fn kill(&mut self, pid: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (i32,);
            let args = (pid,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "kill", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Requests the OS to open a resource identified by `uri` with the most appropriate program. For example:\n\n- `OS.shell_open(\"C:\\\\Users\\\\name\\\\Downloads\")` on Windows opens the file explorer at the user's Downloads folder.\n\n- `OS.shell_open(\"C:/Users/name/Downloads\")` also works on Windows and opens the file explorer at the user's Downloads folder.\n\n- `OS.shell_open(\"https://godotengine.org\")` opens the default web browser on the official Godot website.\n\n- `OS.shell_open(\"mailto:example@example.com\")` opens the default email client with the \"To\" field set to `example@example.com`. See [RFC 2368 - The `mailto` URL scheme](https://datatracker.ietf.org/doc/html/rfc2368) for a list of fields that can be added.\n\nUse [`globalize_path`][`crate::classes::ProjectSettings::globalize_path`] to convert a `res://` or `user://` project path into a system path for use with this method.\n\n**Note:** Use [`uri_encode`][`crate::builtin::GString::uri_encode`] to encode characters within URLs in a URL-safe, portable way. This is especially required for line breaks. Otherwise, [`shell_open`][`crate::classes::Os::shell_open`] may not work correctly in a project exported to the Web platform.\n\n**Note:** This method is implemented on Android, iOS, Web, Linux, macOS and Windows."]
        pub fn shell_open(&mut self, uri: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (uri.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "shell_open", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Requests the OS to open the file manager, navigate to the given `file_or_dir_path` and select the target file or folder.\n\nIf `open_folder` is `true` and `file_or_dir_path` is a valid directory path, the OS will open the file manager and navigate to the target folder without selecting anything.\n\nUse [`globalize_path`][`crate::classes::ProjectSettings::globalize_path`] to convert a `res://` or `user://` project path into a system path to use with this method.\n\n**Note:** This method is currently only implemented on Windows and macOS. On other platforms, it will fallback to [`shell_open`][`crate::classes::Os::shell_open`] with a directory path of `file_or_dir_path` prefixed with `file://`."]
        pub(crate) fn shell_show_in_file_manager_full(&mut self, file_or_dir_path: CowArg < GString >, open_folder: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (file_or_dir_path, open_folder,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "shell_show_in_file_manager", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shell_show_in_file_manager_ex`][Self::shell_show_in_file_manager_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Requests the OS to open the file manager, navigate to the given `file_or_dir_path` and select the target file or folder.\n\nIf `open_folder` is `true` and `file_or_dir_path` is a valid directory path, the OS will open the file manager and navigate to the target folder without selecting anything.\n\nUse [`globalize_path`][`crate::classes::ProjectSettings::globalize_path`] to convert a `res://` or `user://` project path into a system path to use with this method.\n\n**Note:** This method is currently only implemented on Windows and macOS. On other platforms, it will fallback to [`shell_open`][`crate::classes::Os::shell_open`] with a directory path of `file_or_dir_path` prefixed with `file://`."]
        #[inline]
        pub fn shell_show_in_file_manager(&mut self, file_or_dir_path: impl AsArg < GString >,) -> crate::global::Error {
            self.shell_show_in_file_manager_ex(file_or_dir_path,) . done()
        }
        #[doc = "Requests the OS to open the file manager, navigate to the given `file_or_dir_path` and select the target file or folder.\n\nIf `open_folder` is `true` and `file_or_dir_path` is a valid directory path, the OS will open the file manager and navigate to the target folder without selecting anything.\n\nUse [`globalize_path`][`crate::classes::ProjectSettings::globalize_path`] to convert a `res://` or `user://` project path into a system path to use with this method.\n\n**Note:** This method is currently only implemented on Windows and macOS. On other platforms, it will fallback to [`shell_open`][`crate::classes::Os::shell_open`] with a directory path of `file_or_dir_path` prefixed with `file://`."]
        #[inline]
        pub fn shell_show_in_file_manager_ex < 'ex > (&'ex mut self, file_or_dir_path: impl AsArg < GString > + 'ex,) -> ExShellShowInFileManager < 'ex > {
            ExShellShowInFileManager::new(self, file_or_dir_path,)
        }
        #[doc = "Returns `true` if the child process ID (`pid`) is still running or `false` if it has terminated. `pid` must be a valid ID generated from [`create_process`][`crate::classes::Os::create_process`].\n\n**Note:** This method is implemented on Android, iOS, Linux, macOS, and Windows."]
        pub fn is_process_running(&self, pid: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (pid,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_process_running", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the exit code of a spawned process once it has finished running (see [`is_process_running`][`crate::classes::Os::is_process_running`]).\n\nReturns `-1` if the `pid` is not a PID of a spawned child process, the process is still running, or the method is not implemented for the current platform.\n\n**Note:** Returns `-1` if the `pid` is a macOS bundled app process.\n\n**Note:** This method is implemented on Android, Linux, macOS and Windows."]
        pub fn get_process_exit_code(&self, pid: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (pid,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_process_exit_code", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number used by the host machine to uniquely identify this application.\n\n**Note:** On Web, this method always returns `0`."]
        pub fn get_process_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_process_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the environment variable with the name `variable` exists.\n\n**Note:** Double-check the casing of `variable`. Environment variable names are case-sensitive on all platforms except Windows."]
        pub fn has_environment(&self, variable: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (variable.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "has_environment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the given environment variable, or an empty string if `variable` doesn't exist.\n\n**Note:** Double-check the casing of `variable`. Environment variable names are case-sensitive on all platforms except Windows.\n\n**Note:** On macOS, applications do not have access to shell environment variables."]
        pub fn get_environment(&self, variable: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (variable.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_environment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of the environment variable `variable` to `value`. The environment variable will be set for the Godot process and any process executed with [`execute`][`crate::classes::Os::execute`] after running [`set_environment`][`crate::classes::Os::set_environment`]. The environment variable will _not_ persist to processes run after the Godot process was terminated.\n\n**Note:** Environment variable names are case-sensitive on all platforms except Windows. The `variable` name cannot be empty or include the `=` character. On Windows, there is a 32767 characters limit for the combined length of `variable`, `value`, and the `=` and null terminator characters that will be registered in the environment block."]
        pub fn set_environment(&self, variable: impl AsArg < GString >, value: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (variable.into_arg(), value.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_environment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the given environment variable from the current environment, if it exists. The `variable` name cannot be empty or include the `=` character. The environment variable will be removed for the Godot process and any process executed with [`execute`][`crate::classes::Os::execute`] after running [`unset_environment`][`crate::classes::Os::unset_environment`]. The removal of the environment variable will _not_ persist to processes run after the Godot process was terminated.\n\n**Note:** Environment variable names are case-sensitive on all platforms except Windows."]
        pub fn unset_environment(&self, variable: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (variable.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "unset_environment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the host platform.\n\n- On Windows, this is `\"Windows\"`.\n\n- On macOS, this is `\"macOS\"`.\n\n- On Linux-based operating systems, this is `\"Linux\"`.\n\n- On BSD-based operating systems, this is `\"FreeBSD\"`, `\"NetBSD\"`, `\"OpenBSD\"`, or `\"BSD\"` as a fallback.\n\n- On Android, this is `\"Android\"`.\n\n- On iOS, this is `\"iOS\"`.\n\n- On Web, this is `\"Web\"`.\n\n**Note:** Custom builds of the engine may support additional platforms, such as consoles, possibly returning other names.\n\n\n```gdscript\nmatch OS.get_name():\n\t\"Windows\":\n\t\tprint(\"Welcome to Windows!\")\n\t\"macOS\":\n\t\tprint(\"Welcome to macOS!\")\n\t\"Linux\", \"FreeBSD\", \"NetBSD\", \"OpenBSD\", \"BSD\":\n\t\tprint(\"Welcome to Linux/BSD!\")\n\t\"Android\":\n\t\tprint(\"Welcome to Android!\")\n\t\"iOS\":\n\t\tprint(\"Welcome to iOS!\")\n\t\"Web\":\n\t\tprint(\"Welcome to the Web!\")\n```\n\n\n**Note:** On Web platforms, it is still possible to determine the host platform's OS with feature tags. See [`has_feature`][`crate::classes::Os::has_feature`]."]
        pub fn get_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the distribution for Linux and BSD platforms (e.g. \"Ubuntu\", \"Manjaro\", \"OpenBSD\", etc.).\n\nReturns the same value as [`get_name`][`crate::classes::Os::get_name`] for stock Android ROMs, but attempts to return the custom ROM name for popular Android derivatives such as \"LineageOS\".\n\nReturns the same value as [`get_name`][`crate::classes::Os::get_name`] for other platforms.\n\n**Note:** This method is not supported on the Web platform. It returns an empty string."]
        pub fn get_distribution_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_distribution_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the exact production and build version of the operating system. This is different from the branded version used in marketing. This helps to distinguish between different releases of operating systems, including minor versions, and insider and custom builds.\n\n- For Windows, the major and minor version are returned, as well as the build number. For example, the returned string may look like `10.0.9926` for a build of Windows 10.\n\n- For rolling distributions, such as Arch Linux, an empty string is returned.\n\n- For macOS and iOS, the major and minor version are returned, as well as the patch number.\n\n- For Android, the SDK version and the incremental build number are returned. If it's a custom ROM, it attempts to return its version instead.\n\n**Note:** This method is not supported on the Web platform. It returns an empty string."]
        pub fn get_version(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_version", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the branded version used in marketing, followed by the build number (on Windows), the version number (on macOS), or the SDK version and incremental build number (on Android). Examples include `11 (build 22000)`, `Sequoia (15.0.0)`, and `15 (SDK 35 build abc528-11988f)`.\n\nThis value can then be appended to [`get_name`][`crate::classes::Os::get_name`] to get a full, human-readable operating system name and version combination for the operating system. Windows feature updates such as 24H2 are not contained in the resulting string, but Windows Server is recognized as such (e.g. `2025 (build 26100)` for Windows Server 2025).\n\n**Note:** This method is only supported on Windows, macOS, and Android. On other operating systems, it returns the same value as [`get_version`][`crate::classes::Os::get_version`]."]
        pub fn get_version_alias(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_version_alias", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the command-line arguments passed to the engine, excluding arguments processed by the engine, such as `--headless` and `--fullscreen`.\n\n```gdscript\n# Godot has been executed with the following command:\n# godot --headless --verbose --scene my_scene.tscn --custom\nOS.get_cmdline_args() # Returns [\"--scene\", \"my_scene.tscn\", \"--custom\"]\n```\n\nCommand-line arguments can be written in any form, including both `--key value` and `--key=value` forms so they can be properly parsed, as long as custom command-line arguments do not conflict with engine arguments.\n\nYou can also incorporate environment variables using the [`get_environment`][`crate::classes::Os::get_environment`] method.\n\nYou can set \\[member ProjectSettings.editor/run/main_run_args] to define command-line arguments to be passed by the editor when running the project.\n\n**Example:** Parse command-line arguments into a [`Dictionary`][crate::builtin::Dictionary] using the `--key=value` form for arguments:\n\n\n```gdscript\nvar arguments = {}\nfor argument in OS.get_cmdline_args():\n\tif argument.contains(\"=\"):\n\t\tvar key_value = argument.split(\"=\")\n\t\targuments[key_value[0].trim_prefix(\"--\")] = key_value[1]\n\telse:\n\t\t# Options without an argument will be present in the dictionary,\n\t\t# with the value set to an empty string.\n\t\targuments[argument.trim_prefix(\"--\")] = \"\"\n```\n\n\n**Note:** Passing custom user arguments directly is not recommended, as the engine may discard or modify them. Instead, pass the standard UNIX double dash (`--`) and then the custom arguments, which the engine will ignore by design. These can be read via [`get_cmdline_user_args`][`crate::classes::Os::get_cmdline_user_args`]."]
        pub fn get_cmdline_args(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_cmdline_args", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the command-line user arguments passed to the engine. User arguments are ignored by the engine and reserved for the user. They are passed after the double dash `--` argument. `++` may be used when `--` is intercepted by another program (such as `startx`).\n\n```gdscript\n# Godot has been executed with the following command:\n# godot --fullscreen --custom -- --level=2 --hardcore\n\nOS.get_cmdline_args()      # Returns [\"--custom\"]\nOS.get_cmdline_user_args() # Returns [\"--level=2\", \"--hardcore\"]\n```\n\nTo get arguments passed before `--` or `++`, use [`get_cmdline_args`][`crate::classes::Os::get_cmdline_args`]."]
        pub fn get_cmdline_user_args(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_cmdline_user_args", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the video adapter driver name and version for the user's currently active graphics card, as a [`PackedStringArray`][crate::builtin::PackedStringArray]. See also [`get_video_adapter_api_version`][`crate::classes::RenderingServer::get_video_adapter_api_version`].\n\nThe first element holds the driver name, such as `nvidia`, `amdgpu`, etc.\n\nThe second element holds the driver version. For example, on the `nvidia` driver on a Linux/BSD platform, the version is in the format `510.85.02`. For Windows, the driver's format is `31.0.15.1659`.\n\n**Note:** This method is only supported on Linux/BSD and Windows when not running in headless mode. On other platforms, it returns an empty array.\n\n**Note:** This method will run slowly the first time it is called in a session; it can take several seconds depending on the operating system and hardware. It is blocking if called on the main thread, so it's recommended to call it on a separate thread using `Thread`. This allows the engine to keep running while the information is being retrieved. However, [`get_video_adapter_driver_info`][`crate::classes::Os::get_video_adapter_driver_info`] is _not_ thread-safe, so it should not be called from multiple threads at the same time.\n\n\n```gdscript\nvar thread = Thread.new()\n\nfunc _ready():\n\tthread.start(\n\t\tfunc():\n\t\t\tvar driver_info = OS.get_video_adapter_driver_info()\n\t\t\tif not driver_info.is_empty():\n\t\t\t\tprint(\"Driver: %s %s\" % [driver_info[0], driver_info[1]])\n\t\t\telse:\n\t\t\t\tprint(\"Driver: (unknown)\")\n\t)\n\nfunc _exit_tree():\n\tthread.wait_to_finish()\n```\n"]
        pub fn get_video_adapter_driver_info(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_video_adapter_driver_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `restart` is `true`, restarts the project automatically when it is exited with [`quit`][`crate::classes::SceneTree::quit`] or [`NodeNotification::WM_CLOSE_REQUEST`][`crate::classes::notify::NodeNotification::WM_CLOSE_REQUEST`]. Command-line `arguments` can be supplied. To restart the project with the same command line arguments as originally used to run the project, pass [`get_cmdline_args`][`crate::classes::Os::get_cmdline_args`] as the value for `arguments`.\n\nThis method can be used to apply setting changes that require a restart. See also [`is_restart_on_exit_set`][`crate::classes::Os::is_restart_on_exit_set`] and [`get_restart_on_exit_arguments`][`crate::classes::Os::get_restart_on_exit_arguments`].\n\n**Note:** This method is only effective on desktop platforms, and only when the project isn't started from the editor. It will have no effect on mobile and Web platforms, or when the project is started from the editor.\n\n**Note:** If the project process crashes or is _killed_ by the user (by sending `SIGKILL` instead of the usual `SIGTERM`), the project won't restart automatically."]
        pub(crate) fn set_restart_on_exit_full(&mut self, restart: bool, arguments: RefArg < PackedStringArray >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (bool, RefArg < 'a0, PackedStringArray >,);
            let args = (restart, arguments,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_restart_on_exit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_restart_on_exit_ex`][Self::set_restart_on_exit_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "If `restart` is `true`, restarts the project automatically when it is exited with [`quit`][`crate::classes::SceneTree::quit`] or [`NodeNotification::WM_CLOSE_REQUEST`][`crate::classes::notify::NodeNotification::WM_CLOSE_REQUEST`]. Command-line `arguments` can be supplied. To restart the project with the same command line arguments as originally used to run the project, pass [`get_cmdline_args`][`crate::classes::Os::get_cmdline_args`] as the value for `arguments`.\n\nThis method can be used to apply setting changes that require a restart. See also [`is_restart_on_exit_set`][`crate::classes::Os::is_restart_on_exit_set`] and [`get_restart_on_exit_arguments`][`crate::classes::Os::get_restart_on_exit_arguments`].\n\n**Note:** This method is only effective on desktop platforms, and only when the project isn't started from the editor. It will have no effect on mobile and Web platforms, or when the project is started from the editor.\n\n**Note:** If the project process crashes or is _killed_ by the user (by sending `SIGKILL` instead of the usual `SIGTERM`), the project won't restart automatically."]
        #[inline]
        pub fn set_restart_on_exit(&mut self, restart: bool,) {
            self.set_restart_on_exit_ex(restart,) . done()
        }
        #[doc = "If `restart` is `true`, restarts the project automatically when it is exited with [`quit`][`crate::classes::SceneTree::quit`] or [`NodeNotification::WM_CLOSE_REQUEST`][`crate::classes::notify::NodeNotification::WM_CLOSE_REQUEST`]. Command-line `arguments` can be supplied. To restart the project with the same command line arguments as originally used to run the project, pass [`get_cmdline_args`][`crate::classes::Os::get_cmdline_args`] as the value for `arguments`.\n\nThis method can be used to apply setting changes that require a restart. See also [`is_restart_on_exit_set`][`crate::classes::Os::is_restart_on_exit_set`] and [`get_restart_on_exit_arguments`][`crate::classes::Os::get_restart_on_exit_arguments`].\n\n**Note:** This method is only effective on desktop platforms, and only when the project isn't started from the editor. It will have no effect on mobile and Web platforms, or when the project is started from the editor.\n\n**Note:** If the project process crashes or is _killed_ by the user (by sending `SIGKILL` instead of the usual `SIGTERM`), the project won't restart automatically."]
        #[inline]
        pub fn set_restart_on_exit_ex < 'ex > (&'ex mut self, restart: bool,) -> ExSetRestartOnExit < 'ex > {
            ExSetRestartOnExit::new(self, restart,)
        }
        #[doc = "Returns `true` if the project will automatically restart when it exits for any reason, `false` otherwise. See also [`set_restart_on_exit`][`crate::classes::Os::set_restart_on_exit`] and [`get_restart_on_exit_arguments`][`crate::classes::Os::get_restart_on_exit_arguments`]."]
        pub fn is_restart_on_exit_set(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_restart_on_exit_set", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of command line arguments that will be used when the project automatically restarts using [`set_restart_on_exit`][`crate::classes::Os::set_restart_on_exit`]. See also [`is_restart_on_exit_set`][`crate::classes::Os::is_restart_on_exit_set`]."]
        pub fn get_restart_on_exit_arguments(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_restart_on_exit_arguments", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Delays execution of the current thread by `usec` microseconds. `usec` must be greater than or equal to `0`. Otherwise, [`delay_usec`][`crate::classes::Os::delay_usec`] does nothing and prints an error message.\n\n**Note:** [`delay_usec`][`crate::classes::Os::delay_usec`] is a _blocking_ way to delay code execution. To delay code execution in a non-blocking way, you may use [`create_timer`][`crate::classes::SceneTree::create_timer`]. Awaiting with a [`SceneTreeTimer`][crate::classes::SceneTreeTimer] delays the execution of code placed below the `await` without affecting the rest of the project (or editor, for [`EditorPlugin`][crate::classes::EditorPlugin]s and [`EditorScript`][crate::classes::EditorScript]s).\n\n**Note:** When [`delay_usec`][`crate::classes::Os::delay_usec`] is called on the main thread, it will freeze the project and will prevent it from redrawing and registering input until the delay has passed. When using [`delay_usec`][`crate::classes::Os::delay_usec`] as part of an [`EditorPlugin`][crate::classes::EditorPlugin] or [`EditorScript`][crate::classes::EditorScript], it will freeze the editor but won't freeze the project if it is currently running (since the project is an independent child process)."]
        pub fn delay_usec(&self, usec: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (usec,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "delay_usec", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Delays execution of the current thread by `msec` milliseconds. `msec` must be greater than or equal to `0`. Otherwise, [`delay_msec`][`crate::classes::Os::delay_msec`] does nothing and prints an error message.\n\n**Note:** [`delay_msec`][`crate::classes::Os::delay_msec`] is a _blocking_ way to delay code execution. To delay code execution in a non-blocking way, you may use [`create_timer`][`crate::classes::SceneTree::create_timer`]. Awaiting with [`SceneTreeTimer`][crate::classes::SceneTreeTimer] delays the execution of code placed below the `await` without affecting the rest of the project (or editor, for [`EditorPlugin`][crate::classes::EditorPlugin]s and [`EditorScript`][crate::classes::EditorScript]s).\n\n**Note:** When [`delay_msec`][`crate::classes::Os::delay_msec`] is called on the main thread, it will freeze the project and will prevent it from redrawing and registering input until the delay has passed. When using [`delay_msec`][`crate::classes::Os::delay_msec`] as part of an [`EditorPlugin`][crate::classes::EditorPlugin] or [`EditorScript`][crate::classes::EditorScript], it will freeze the editor but won't freeze the project if it is currently running (since the project is an independent child process)."]
        pub fn delay_msec(&self, msec: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (msec,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "delay_msec", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the host OS locale as a [`String`][crate::builtin::GString] of the form `language_Script_COUNTRY_VARIANT@extra`. Every substring after `language` is optional and may not exist.\n\n- `language` - 2 or 3-letter [language code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes), in lower case.\n\n- `Script` - 4-letter [script code](https://en.wikipedia.org/wiki/ISO_15924), in title case.\n\n- `COUNTRY` - 2 or 3-letter [country code](https://en.wikipedia.org/wiki/ISO_3166-1), in upper case.\n\n- `VARIANT` - language variant, region and sort order. The variant can have any number of underscored keywords.\n\n- `extra` - semicolon separated list of additional key words. This may include currency, calendar, sort order and numbering system information.\n\nIf you want only the language code and not the fully specified locale from the OS, you can use [`get_locale_language`][`crate::classes::Os::get_locale_language`]."]
        pub fn get_locale(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_locale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the host OS locale's 2 or 3-letter [language code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) as a string which should be consistent on all platforms. This is equivalent to extracting the `language` part of the [`get_locale`][`crate::classes::Os::get_locale`] string.\n\nThis can be used to narrow down fully specified locale strings to only the \"common\" language code, when you don't need the additional information about country code or variants. For example, for a French Canadian user with `fr_CA` locale, this would return `fr`."]
        pub fn get_locale_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_locale_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the model name of the current device.\n\n**Note:** This method is implemented on Android, iOS, macOS, and Windows. Returns `\"GenericDevice\"` on unsupported platforms."]
        pub fn get_model_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_model_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the `user://` file system is persistent, that is, its state is the same after a player quits and starts the game again. Relevant to the Web platform, where this persistence may be unavailable."]
        pub fn is_userfs_persistent(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_userfs_persistent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the engine was executed with the `--verbose` or `-v` command line argument, or if \\[member ProjectSettings.debug/settings/stdout/verbose_stdout] is `true`. See also [`print_verbose`][`crate::global::print_verbose`]."]
        pub fn is_stdout_verbose(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_stdout_verbose", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the Godot binary used to run the project is a _debug_ export template, or when running in the editor.\n\nReturns `false` if the Godot binary used to run the project is a _release_ export template.\n\n**Note:** To check whether the Godot binary used to run the project is an export template (debug or release), use `OS.has_feature(\"template\")` instead."]
        pub fn is_debug_build(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_debug_build", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the amount of static memory being used by the program in bytes. Only works in debug builds."]
        pub fn get_static_memory_usage(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_static_memory_usage", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the maximum amount of static memory used. Only works in debug builds."]
        pub fn get_static_memory_peak_usage(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_static_memory_peak_usage", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Dictionary`][crate::builtin::Dictionary] containing information about the current memory with the following entries:\n\n- `\"physical\"` - total amount of usable physical memory in bytes. This value can be slightly less than the actual physical memory amount, since it does not include memory reserved by the kernel and devices.\n\n- `\"free\"` - amount of physical memory, that can be immediately allocated without disk access or other costly operations, in bytes. The process might be able to allocate more physical memory, but this action will require moving inactive pages to disk, which can be expensive.\n\n- `\"available\"` - amount of memory that can be allocated without extending the swap file(s), in bytes. This value includes both physical memory and swap.\n\n- `\"stack\"` - size of the current thread stack in bytes.\n\n**Note:** Each entry's value may be `-1` if it is unknown."]
        pub fn get_memory_info(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_memory_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves the file or directory at the given `path` to the system's recycle bin. See also [`remove`][`crate::classes::DirAccess::remove`].\n\nThe method takes only global paths, so you may need to use [`globalize_path`][`crate::classes::ProjectSettings::globalize_path`]. Do not use it for files in `res://` as it will not work in exported projects.\n\nReturns [`Error::FAILED`][`crate::global::Error::FAILED`] if the file or directory cannot be found, or the system does not support this method.\n\n\n```gdscript\nvar file_to_remove = \"user://slot1.save\"\nOS.move_to_trash(ProjectSettings.globalize_path(file_to_remove))\n```\n\n\n**Note:** This method is implemented on Android, Linux, macOS and Windows.\n\n**Note:** If the user has disabled the recycle bin on their system, the file will be permanently deleted instead."]
        pub fn move_to_trash(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "move_to_trash", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the absolute directory path where user data is written (the `user://` directory in Godot). The path depends on the project name and \\[member ProjectSettings.application/config/use_custom_user_dir].\n\n- On Windows, this is `%AppData%\\Godot\\app_userdata\\[project_name]`, or `%AppData%\\[custom_name]` if `use_custom_user_dir` is set. `%AppData%` expands to `%UserProfile%\\AppData\\Roaming`.\n\n- On macOS, this is `~/Library/Application Support/Godot/app_userdata/[project_name]`, or `~/Library/Application Support/[custom_name]` if `use_custom_user_dir` is set.\n\n- On Linux and BSD, this is `~/.local/share/godot/app_userdata/[project_name]`, or `~/.local/share/[custom_name]` if `use_custom_user_dir` is set.\n\n- On Android and iOS, this is a sandboxed directory in either internal or external storage, depending on the user's configuration.\n\n- On Web, this is a virtual directory managed by the browser.\n\nIf the project name is empty, `[project_name]` falls back to `[unnamed project]`.\n\nNot to be confused with [`get_data_dir`][`crate::classes::Os::get_data_dir`], which returns the _global_ (non-project-specific) user home directory."]
        pub fn get_user_data_dir(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_user_data_dir", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the path to commonly used folders across different platforms, as defined by `dir`. See the \\[enum SystemDir] constants for available locations.\n\n**Note:** This method is implemented on Android, Linux, macOS and Windows.\n\n**Note:** Shared storage is implemented on Android and allows to differentiate between app specific and shared directories, if `shared_storage` is `true`. Shared directories have additional restrictions on Android."]
        pub(crate) fn get_system_dir_full(&self, dir: crate::classes::os::SystemDir, shared_storage: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (crate::classes::os::SystemDir, bool,);
            let args = (dir, shared_storage,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_system_dir", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_system_dir_ex`][Self::get_system_dir_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the path to commonly used folders across different platforms, as defined by `dir`. See the \\[enum SystemDir] constants for available locations.\n\n**Note:** This method is implemented on Android, Linux, macOS and Windows.\n\n**Note:** Shared storage is implemented on Android and allows to differentiate between app specific and shared directories, if `shared_storage` is `true`. Shared directories have additional restrictions on Android."]
        #[inline]
        pub fn get_system_dir(&self, dir: crate::classes::os::SystemDir,) -> GString {
            self.get_system_dir_ex(dir,) . done()
        }
        #[doc = "Returns the path to commonly used folders across different platforms, as defined by `dir`. See the \\[enum SystemDir] constants for available locations.\n\n**Note:** This method is implemented on Android, Linux, macOS and Windows.\n\n**Note:** Shared storage is implemented on Android and allows to differentiate between app specific and shared directories, if `shared_storage` is `true`. Shared directories have additional restrictions on Android."]
        #[inline]
        pub fn get_system_dir_ex < 'ex > (&'ex self, dir: crate::classes::os::SystemDir,) -> ExGetSystemDir < 'ex > {
            ExGetSystemDir::new(self, dir,)
        }
        #[doc = "Returns the _global_ user configuration directory according to the operating system's standards.\n\nOn the Linux/BSD platform, this path can be overridden by setting the `XDG_CONFIG_HOME` environment variable before starting the project. See [File paths in Godot projects]($DOCS_URL/tutorials/io/data_paths.html) in the documentation for more information. See also [`get_cache_dir`][`crate::classes::Os::get_cache_dir`] and [`get_data_dir`][`crate::classes::Os::get_data_dir`].\n\nNot to be confused with [`get_user_data_dir`][`crate::classes::Os::get_user_data_dir`], which returns the _project-specific_ user data path."]
        pub fn get_config_dir(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_config_dir", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the _global_ user data directory according to the operating system's standards.\n\nOn the Linux/BSD platform, this path can be overridden by setting the `XDG_DATA_HOME` environment variable before starting the project. See [File paths in Godot projects]($DOCS_URL/tutorials/io/data_paths.html) in the documentation for more information. See also [`get_cache_dir`][`crate::classes::Os::get_cache_dir`] and [`get_config_dir`][`crate::classes::Os::get_config_dir`].\n\nNot to be confused with [`get_user_data_dir`][`crate::classes::Os::get_user_data_dir`], which returns the _project-specific_ user data path."]
        pub fn get_data_dir(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_data_dir", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the _global_ cache data directory according to the operating system's standards.\n\nOn the Linux/BSD platform, this path can be overridden by setting the `XDG_CACHE_HOME` environment variable before starting the project. See [File paths in Godot projects]($DOCS_URL/tutorials/io/data_paths.html) in the documentation for more information. See also [`get_config_dir`][`crate::classes::Os::get_config_dir`] and [`get_data_dir`][`crate::classes::Os::get_data_dir`].\n\nNot to be confused with [`get_user_data_dir`][`crate::classes::Os::get_user_data_dir`], which returns the _project-specific_ user data path."]
        pub fn get_cache_dir(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_cache_dir", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the _global_ temporary data directory according to the operating system's standards."]
        pub fn get_temp_dir(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_temp_dir", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a string that is unique to the device.\n\n**Note:** This string may change without notice if the user reinstalls their operating system, upgrades it, or modifies their hardware. This means it should generally not be used to encrypt persistent data, as the data saved before an unexpected ID change would become inaccessible. The returned string may also be falsified using external programs, so do not rely on the string returned by this method for security purposes.\n\n**Note:** On Web, returns an empty string and generates an error, as this method cannot be implemented for security reasons."]
        pub fn get_unique_id(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_unique_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given keycode as a [`String`][crate::builtin::GString].\n\n\n```gdscript\nprint(OS.get_keycode_string(KEY_C))                    # Prints \"C\"\nprint(OS.get_keycode_string(KEY_ESCAPE))               # Prints \"Escape\"\nprint(OS.get_keycode_string(KEY_MASK_SHIFT | KEY_TAB)) # Prints \"Shift+Tab\"\n```\n\n\nSee also [`find_keycode_from_string`][`crate::classes::Os::find_keycode_from_string`], \\[member InputEventKey.keycode], and [`get_keycode_with_modifiers`][`crate::classes::InputEventKey::get_keycode_with_modifiers`]."]
        pub fn get_keycode_string(&self, code: crate::global::Key,) -> GString {
            type CallRet = GString;
            type CallParams = (crate::global::Key,);
            let args = (code,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_keycode_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the input keycode corresponds to a Unicode character. For a list of codes, see the \\[enum Key] constants.\n\n\n```gdscript\nprint(OS.is_keycode_unicode(KEY_G))      # Prints true\nprint(OS.is_keycode_unicode(KEY_KP_4))   # Prints true\nprint(OS.is_keycode_unicode(KEY_TAB))    # Prints false\nprint(OS.is_keycode_unicode(KEY_ESCAPE)) # Prints false\n```\n"]
        pub fn is_keycode_unicode(&self, code: u32,) -> bool {
            type CallRet = bool;
            type CallParams = (u32,);
            let args = (code,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_keycode_unicode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Finds the keycode for the given string. The returned values are equivalent to the \\[enum Key] constants.\n\n\n```gdscript\nprint(OS.find_keycode_from_string(\"C\"))         # Prints 67 (KEY_C)\nprint(OS.find_keycode_from_string(\"Escape\"))    # Prints 4194305 (KEY_ESCAPE)\nprint(OS.find_keycode_from_string(\"Shift+Tab\")) # Prints 37748738 (KEY_MASK_SHIFT | KEY_TAB)\nprint(OS.find_keycode_from_string(\"Unknown\"))   # Prints 0 (KEY_NONE)\n```\n\n\nSee also [`get_keycode_string`][`crate::classes::Os::get_keycode_string`]."]
        pub fn find_keycode_from_string(&self, string: impl AsArg < GString >,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "find_keycode_from_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `enabled` is `true`, when opening a file for writing, a temporary file is used in its place. When closed, it is automatically applied to the target file.\n\nThis can useful when files may be opened by other applications, such as antiviruses, text editors, or even the Godot editor itself."]
        pub fn set_use_file_access_save_and_swap(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_use_file_access_save_and_swap", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns the given name to the current thread. Returns [`Error::ERR_UNAVAILABLE`][`crate::global::Error::ERR_UNAVAILABLE`] if unavailable on the current platform."]
        pub fn set_thread_name(&mut self, name: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "set_thread_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID of the current thread. This can be used in logs to ease debugging of multi-threaded applications.\n\n**Note:** Thread IDs are not deterministic and may be reused across application restarts."]
        pub fn get_thread_caller_id(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_thread_caller_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID of the main thread. See [`get_thread_caller_id`][`crate::classes::Os::get_thread_caller_id`].\n\n**Note:** Thread IDs are not deterministic and may be reused across application restarts."]
        pub fn get_main_thread_id(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_main_thread_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the feature for the given feature tag is supported in the currently running instance, depending on the platform, build, etc. Can be used to check whether you're currently running a debug build, on a certain platform or arch, etc. Refer to the [Feature Tags]($DOCS_URL/tutorials/export/feature_tags.html) documentation for more details.\n\n**Note:** Tag names are case-sensitive.\n\n**Note:** On the Web platform, one of the following additional tags is defined to indicate the host platform: `web_android`, `web_ios`, `web_linuxbsd`, `web_macos`, or `web_windows`."]
        pub fn has_feature(&self, tag_name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (tag_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "has_feature", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the application is running in the sandbox.\n\n**Note:** This method is only implemented on macOS and Linux."]
        pub fn is_sandboxed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "is_sandboxed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Requests permission from the OS for the given `name`. Returns `true` if the permission has already been granted. See also `MainLoop.on_request_permissions_result`.\n\nThe `name` must be the full permission name. For example:\n\n- `OS.request_permission(\"android.permission.READ_EXTERNAL_STORAGE\")`\n\n- `OS.request_permission(\"android.permission.POST_NOTIFICATIONS\")`\n\n- `OS.request_permission(\"macos.permission.RECORD_SCREEN\")`\n\n- `OS.request_permission(\"appleembedded.permission.AUDIO_RECORD\")`\n\n**Note:** On Android, permission must be checked during export.\n\n**Note:** This method is implemented on Android, macOS, and visionOS platforms."]
        pub fn request_permission(&mut self, name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "request_permission", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Requests _dangerous_ permissions from the OS. Returns `true` if permissions have already been granted. See also `MainLoop.on_request_permissions_result`.\n\n**Note:** Permissions must be checked during export.\n\n**Note:** This method is only implemented on Android. Normal permissions are automatically granted at install time in Android applications."]
        pub fn request_permissions(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "request_permissions", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "On Android devices: Returns the list of dangerous permissions that have been granted.\n\nOn macOS: Returns the list of granted permissions and user selected folders accessible to the application (sandboxed applications only). Use the native file dialog to request folder access permission.\n\nOn iOS, visionOS: Returns the list of granted permissions."]
        pub fn get_granted_permissions(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "get_granted_permissions", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "On macOS (sandboxed applications only), this function clears list of user selected folders accessible to the application."]
        pub fn revoke_granted_permissions(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "revoke_granted_permissions", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Add a custom logger to intercept the internal message stream."]
        pub fn add_logger(&mut self, logger: impl AsArg < Option < Gd < crate::classes::Logger >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Logger > > >,);
            let args = (logger.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "add_logger", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Remove a custom logger added by [`add_logger`][`crate::classes::Os::add_logger`]."]
        pub fn remove_logger(&mut self, logger: impl AsArg < Option < Gd < crate::classes::Logger >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Logger > > >,);
            let args = (logger.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Os", "remove_logger", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Os {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OS"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Core;
        
    }
    unsafe impl crate::obj::Bounds for Os {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Os {
        
    }
    impl crate::obj::Singleton for Os {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"OS"))
            }
        }
    }
    impl std::ops::Deref for Os {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Os {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Os__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Os` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Os::alert_ex`][super::Os::alert_ex]."]
#[must_use]
pub struct ExAlert < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Os, text: CowArg < 'ex, GString >, title: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAlert < 'ex > {
    fn new(surround_object: &'ex mut re_export::Os, text: impl AsArg < GString > + 'ex,) -> Self {
        let title = GString::from("Alert!");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), title: CowArg::Owned(title),
        }
    }
    #[inline]
    pub fn title(self, title: impl AsArg < GString > + 'ex) -> Self {
        Self {
            title: title.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text, title,
        }
        = self;
        re_export::Os::alert_full(surround_object, text, title,)
    }
}
#[doc = "Default-param extender for [`Os::get_system_font_path_ex`][super::Os::get_system_font_path_ex]."]
#[must_use]
pub struct ExGetSystemFontPath < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Os, font_name: CowArg < 'ex, GString >, weight: i32, stretch: i32, italic: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSystemFontPath < 'ex > {
    fn new(surround_object: &'ex re_export::Os, font_name: impl AsArg < GString > + 'ex,) -> Self {
        let weight = 400i32;
        let stretch = 100i32;
        let italic = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_name: font_name.into_arg(), weight: weight, stretch: stretch, italic: italic,
        }
    }
    #[inline]
    pub fn weight(self, weight: i32) -> Self {
        Self {
            weight: weight, .. self
        }
    }
    #[inline]
    pub fn stretch(self, stretch: i32) -> Self {
        Self {
            stretch: stretch, .. self
        }
    }
    #[inline]
    pub fn italic(self, italic: bool) -> Self {
        Self {
            italic: italic, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, font_name, weight, stretch, italic,
        }
        = self;
        re_export::Os::get_system_font_path_full(surround_object, font_name, weight, stretch, italic,)
    }
}
#[doc = "Default-param extender for [`Os::get_system_font_path_for_text_ex`][super::Os::get_system_font_path_for_text_ex]."]
#[must_use]
pub struct ExGetSystemFontPathForText < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Os, font_name: CowArg < 'ex, GString >, text: CowArg < 'ex, GString >, locale: CowArg < 'ex, GString >, script: CowArg < 'ex, GString >, weight: i32, stretch: i32, italic: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSystemFontPathForText < 'ex > {
    fn new(surround_object: &'ex re_export::Os, font_name: impl AsArg < GString > + 'ex, text: impl AsArg < GString > + 'ex,) -> Self {
        let locale = GString::from("");
        let script = GString::from("");
        let weight = 400i32;
        let stretch = 100i32;
        let italic = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_name: font_name.into_arg(), text: text.into_arg(), locale: CowArg::Owned(locale), script: CowArg::Owned(script), weight: weight, stretch: stretch, italic: italic,
        }
    }
    #[inline]
    pub fn locale(self, locale: impl AsArg < GString > + 'ex) -> Self {
        Self {
            locale: locale.into_arg(), .. self
        }
    }
    #[inline]
    pub fn script(self, script: impl AsArg < GString > + 'ex) -> Self {
        Self {
            script: script.into_arg(), .. self
        }
    }
    #[inline]
    pub fn weight(self, weight: i32) -> Self {
        Self {
            weight: weight, .. self
        }
    }
    #[inline]
    pub fn stretch(self, stretch: i32) -> Self {
        Self {
            stretch: stretch, .. self
        }
    }
    #[inline]
    pub fn italic(self, italic: bool) -> Self {
        Self {
            italic: italic, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, font_name, text, locale, script, weight, stretch, italic,
        }
        = self;
        re_export::Os::get_system_font_path_for_text_full(surround_object, font_name, text, locale, script, weight, stretch, italic,)
    }
}
#[doc = "Default-param extender for [`Os::read_string_from_stdin_ex`][super::Os::read_string_from_stdin_ex]."]
#[must_use]
pub struct ExReadStringFromStdin < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Os, buffer_size: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExReadStringFromStdin < 'ex > {
    fn new(surround_object: &'ex mut re_export::Os,) -> Self {
        let buffer_size = 1024i64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, buffer_size: buffer_size,
        }
    }
    #[inline]
    pub fn buffer_size(self, buffer_size: i64) -> Self {
        Self {
            buffer_size: buffer_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, buffer_size,
        }
        = self;
        re_export::Os::read_string_from_stdin_full(surround_object, buffer_size,)
    }
}
#[doc = "Default-param extender for [`Os::read_buffer_from_stdin_ex`][super::Os::read_buffer_from_stdin_ex]."]
#[must_use]
pub struct ExReadBufferFromStdin < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Os, buffer_size: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExReadBufferFromStdin < 'ex > {
    fn new(surround_object: &'ex mut re_export::Os,) -> Self {
        let buffer_size = 1024i64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, buffer_size: buffer_size,
        }
    }
    #[inline]
    pub fn buffer_size(self, buffer_size: i64) -> Self {
        Self {
            buffer_size: buffer_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, buffer_size,
        }
        = self;
        re_export::Os::read_buffer_from_stdin_full(surround_object, buffer_size,)
    }
}
#[doc = "Default-param extender for [`Os::execute_ex`][super::Os::execute_ex]."]
#[must_use]
pub struct ExExecute < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Os, path: CowArg < 'ex, GString >, arguments: CowArg < 'ex, PackedStringArray >, output: CowArg < 'ex, AnyArray >, read_stderr: bool, open_console: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExExecute < 'ex > {
    fn new(surround_object: &'ex mut re_export::Os, path: impl AsArg < GString > + 'ex, arguments: &'ex PackedStringArray,) -> Self {
        let output = AnyArray::new_untyped();
        let read_stderr = false;
        let open_console = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), arguments: CowArg::Borrowed(arguments), output: CowArg::Owned(output), read_stderr: read_stderr, open_console: open_console,
        }
    }
    #[inline]
    pub fn output(self, output: &'ex AnyArray) -> Self {
        Self {
            output: CowArg::Borrowed(output), .. self
        }
    }
    #[inline]
    pub fn read_stderr(self, read_stderr: bool) -> Self {
        Self {
            read_stderr: read_stderr, .. self
        }
    }
    #[inline]
    pub fn open_console(self, open_console: bool) -> Self {
        Self {
            open_console: open_console, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, path, arguments, output, read_stderr, open_console,
        }
        = self;
        re_export::Os::execute_full(surround_object, path, arguments.cow_as_arg(), output.cow_as_arg(), read_stderr, open_console,)
    }
}
#[doc = "Default-param extender for [`Os::execute_with_pipe_ex`][super::Os::execute_with_pipe_ex]."]
#[must_use]
pub struct ExExecuteWithPipe < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Os, path: CowArg < 'ex, GString >, arguments: CowArg < 'ex, PackedStringArray >, blocking: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExExecuteWithPipe < 'ex > {
    fn new(surround_object: &'ex mut re_export::Os, path: impl AsArg < GString > + 'ex, arguments: &'ex PackedStringArray,) -> Self {
        let blocking = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), arguments: CowArg::Borrowed(arguments), blocking: blocking,
        }
    }
    #[inline]
    pub fn blocking(self, blocking: bool) -> Self {
        Self {
            blocking: blocking, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarDictionary {
        let Self {
            _phantom, surround_object, path, arguments, blocking,
        }
        = self;
        re_export::Os::execute_with_pipe_full(surround_object, path, arguments.cow_as_arg(), blocking,)
    }
}
#[doc = "Default-param extender for [`Os::create_process_ex`][super::Os::create_process_ex]."]
#[must_use]
pub struct ExCreateProcess < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Os, path: CowArg < 'ex, GString >, arguments: CowArg < 'ex, PackedStringArray >, open_console: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateProcess < 'ex > {
    fn new(surround_object: &'ex mut re_export::Os, path: impl AsArg < GString > + 'ex, arguments: &'ex PackedStringArray,) -> Self {
        let open_console = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), arguments: CowArg::Borrowed(arguments), open_console: open_console,
        }
    }
    #[inline]
    pub fn open_console(self, open_console: bool) -> Self {
        Self {
            open_console: open_console, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, path, arguments, open_console,
        }
        = self;
        re_export::Os::create_process_full(surround_object, path, arguments.cow_as_arg(), open_console,)
    }
}
#[doc = "Default-param extender for [`Os::shell_show_in_file_manager_ex`][super::Os::shell_show_in_file_manager_ex]."]
#[must_use]
pub struct ExShellShowInFileManager < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Os, file_or_dir_path: CowArg < 'ex, GString >, open_folder: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShellShowInFileManager < 'ex > {
    fn new(surround_object: &'ex mut re_export::Os, file_or_dir_path: impl AsArg < GString > + 'ex,) -> Self {
        let open_folder = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, file_or_dir_path: file_or_dir_path.into_arg(), open_folder: open_folder,
        }
    }
    #[inline]
    pub fn open_folder(self, open_folder: bool) -> Self {
        Self {
            open_folder: open_folder, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, file_or_dir_path, open_folder,
        }
        = self;
        re_export::Os::shell_show_in_file_manager_full(surround_object, file_or_dir_path, open_folder,)
    }
}
#[doc = "Default-param extender for [`Os::set_restart_on_exit_ex`][super::Os::set_restart_on_exit_ex]."]
#[must_use]
pub struct ExSetRestartOnExit < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Os, restart: bool, arguments: CowArg < 'ex, PackedStringArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetRestartOnExit < 'ex > {
    fn new(surround_object: &'ex mut re_export::Os, restart: bool,) -> Self {
        let arguments = PackedStringArray::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, restart: restart, arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'ex PackedStringArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, restart, arguments,
        }
        = self;
        re_export::Os::set_restart_on_exit_full(surround_object, restart, arguments.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`Os::get_system_dir_ex`][super::Os::get_system_dir_ex]."]
#[must_use]
pub struct ExGetSystemDir < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Os, dir: crate::classes::os::SystemDir, shared_storage: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSystemDir < 'ex > {
    fn new(surround_object: &'ex re_export::Os, dir: crate::classes::os::SystemDir,) -> Self {
        let shared_storage = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dir: dir, shared_storage: shared_storage,
        }
    }
    #[inline]
    pub fn shared_storage(self, shared_storage: bool) -> Self {
        Self {
            shared_storage: shared_storage, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, dir, shared_storage,
        }
        = self;
        re_export::Os::get_system_dir_full(surround_object, dir, shared_storage,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderingDriver {
    ord: i32
}
impl RenderingDriver {
    #[doc(alias = "RENDERING_DRIVER_VULKAN")]
    #[doc = "Godot enumerator name: `RENDERING_DRIVER_VULKAN`"]
    pub const VULKAN: RenderingDriver = RenderingDriver {
        ord: 0i32
    };
    #[doc(alias = "RENDERING_DRIVER_OPENGL3")]
    #[doc = "Godot enumerator name: `RENDERING_DRIVER_OPENGL3`"]
    pub const OPENGL3: RenderingDriver = RenderingDriver {
        ord: 1i32
    };
    #[doc(alias = "RENDERING_DRIVER_D3D12")]
    #[doc = "Godot enumerator name: `RENDERING_DRIVER_D3D12`"]
    pub const D3D12: RenderingDriver = RenderingDriver {
        ord: 2i32
    };
    #[doc(alias = "RENDERING_DRIVER_METAL")]
    #[doc = "Godot enumerator name: `RENDERING_DRIVER_METAL`"]
    pub const METAL: RenderingDriver = RenderingDriver {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for RenderingDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RenderingDriver") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RenderingDriver {
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
            Self::VULKAN => "VULKAN", Self::OPENGL3 => "OPENGL3", Self::D3D12 => "D3D12", Self::METAL => "METAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RenderingDriver::VULKAN, RenderingDriver::OPENGL3, RenderingDriver::D3D12, RenderingDriver::METAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RenderingDriver >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VULKAN", "RENDERING_DRIVER_VULKAN", RenderingDriver::VULKAN), crate::meta::inspect::EnumConstant::new("OPENGL3", "RENDERING_DRIVER_OPENGL3", RenderingDriver::OPENGL3), crate::meta::inspect::EnumConstant::new("D3D12", "RENDERING_DRIVER_D3D12", RenderingDriver::D3D12), crate::meta::inspect::EnumConstant::new("METAL", "RENDERING_DRIVER_METAL", RenderingDriver::METAL)]
        }
    }
}
impl crate::meta::GodotConvert for RenderingDriver {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Rendering Driver Vulkan", 0i64), EnumeratorShape::new_int("Rendering Driver Opengl3", 1i64), EnumeratorShape::new_int("Rendering Driver D3d12", 2i64), EnumeratorShape::new_int("Rendering Driver Metal", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("OS.RenderingDriver")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for RenderingDriver {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RenderingDriver {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for RenderingDriver {
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
impl crate::registry::property::Export for RenderingDriver {
    
}
impl crate::meta::Element for RenderingDriver {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SystemDir {
    ord: i32
}
impl SystemDir {
    #[doc(alias = "SYSTEM_DIR_DESKTOP")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_DESKTOP`"]
    pub const DESKTOP: SystemDir = SystemDir {
        ord: 0i32
    };
    #[doc(alias = "SYSTEM_DIR_DCIM")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_DCIM`"]
    pub const DCIM: SystemDir = SystemDir {
        ord: 1i32
    };
    #[doc(alias = "SYSTEM_DIR_DOCUMENTS")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_DOCUMENTS`"]
    pub const DOCUMENTS: SystemDir = SystemDir {
        ord: 2i32
    };
    #[doc(alias = "SYSTEM_DIR_DOWNLOADS")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_DOWNLOADS`"]
    pub const DOWNLOADS: SystemDir = SystemDir {
        ord: 3i32
    };
    #[doc(alias = "SYSTEM_DIR_MOVIES")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_MOVIES`"]
    pub const MOVIES: SystemDir = SystemDir {
        ord: 4i32
    };
    #[doc(alias = "SYSTEM_DIR_MUSIC")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_MUSIC`"]
    pub const MUSIC: SystemDir = SystemDir {
        ord: 5i32
    };
    #[doc(alias = "SYSTEM_DIR_PICTURES")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_PICTURES`"]
    pub const PICTURES: SystemDir = SystemDir {
        ord: 6i32
    };
    #[doc(alias = "SYSTEM_DIR_RINGTONES")]
    #[doc = "Godot enumerator name: `SYSTEM_DIR_RINGTONES`"]
    pub const RINGTONES: SystemDir = SystemDir {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for SystemDir {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SystemDir") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SystemDir {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::DESKTOP => "DESKTOP", Self::DCIM => "DCIM", Self::DOCUMENTS => "DOCUMENTS", Self::DOWNLOADS => "DOWNLOADS", Self::MOVIES => "MOVIES", Self::MUSIC => "MUSIC", Self::PICTURES => "PICTURES", Self::RINGTONES => "RINGTONES", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SystemDir::DESKTOP, SystemDir::DCIM, SystemDir::DOCUMENTS, SystemDir::DOWNLOADS, SystemDir::MOVIES, SystemDir::MUSIC, SystemDir::PICTURES, SystemDir::RINGTONES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SystemDir >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DESKTOP", "SYSTEM_DIR_DESKTOP", SystemDir::DESKTOP), crate::meta::inspect::EnumConstant::new("DCIM", "SYSTEM_DIR_DCIM", SystemDir::DCIM), crate::meta::inspect::EnumConstant::new("DOCUMENTS", "SYSTEM_DIR_DOCUMENTS", SystemDir::DOCUMENTS), crate::meta::inspect::EnumConstant::new("DOWNLOADS", "SYSTEM_DIR_DOWNLOADS", SystemDir::DOWNLOADS), crate::meta::inspect::EnumConstant::new("MOVIES", "SYSTEM_DIR_MOVIES", SystemDir::MOVIES), crate::meta::inspect::EnumConstant::new("MUSIC", "SYSTEM_DIR_MUSIC", SystemDir::MUSIC), crate::meta::inspect::EnumConstant::new("PICTURES", "SYSTEM_DIR_PICTURES", SystemDir::PICTURES), crate::meta::inspect::EnumConstant::new("RINGTONES", "SYSTEM_DIR_RINGTONES", SystemDir::RINGTONES)]
        }
    }
}
impl crate::meta::GodotConvert for SystemDir {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("System Dir Desktop", 0i64), EnumeratorShape::new_int("System Dir Dcim", 1i64), EnumeratorShape::new_int("System Dir Documents", 2i64), EnumeratorShape::new_int("System Dir Downloads", 3i64), EnumeratorShape::new_int("System Dir Movies", 4i64), EnumeratorShape::new_int("System Dir Music", 5i64), EnumeratorShape::new_int("System Dir Pictures", 6i64), EnumeratorShape::new_int("System Dir Ringtones", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("OS.SystemDir")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SystemDir {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SystemDir {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SystemDir {
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
impl crate::registry::property::Export for SystemDir {
    
}
impl crate::meta::Element for SystemDir {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StdHandleType {
    ord: i32
}
impl StdHandleType {
    #[doc(alias = "STD_HANDLE_INVALID")]
    #[doc = "Godot enumerator name: `STD_HANDLE_INVALID`"]
    pub const INVALID: StdHandleType = StdHandleType {
        ord: 0i32
    };
    #[doc(alias = "STD_HANDLE_CONSOLE")]
    #[doc = "Godot enumerator name: `STD_HANDLE_CONSOLE`"]
    pub const CONSOLE: StdHandleType = StdHandleType {
        ord: 1i32
    };
    #[doc(alias = "STD_HANDLE_FILE")]
    #[doc = "Godot enumerator name: `STD_HANDLE_FILE`"]
    pub const FILE: StdHandleType = StdHandleType {
        ord: 2i32
    };
    #[doc(alias = "STD_HANDLE_PIPE")]
    #[doc = "Godot enumerator name: `STD_HANDLE_PIPE`"]
    pub const PIPE: StdHandleType = StdHandleType {
        ord: 3i32
    };
    #[doc(alias = "STD_HANDLE_UNKNOWN")]
    #[doc = "Godot enumerator name: `STD_HANDLE_UNKNOWN`"]
    pub const UNKNOWN: StdHandleType = StdHandleType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for StdHandleType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StdHandleType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StdHandleType {
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
            Self::INVALID => "INVALID", Self::CONSOLE => "CONSOLE", Self::FILE => "FILE", Self::PIPE => "PIPE", Self::UNKNOWN => "UNKNOWN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[StdHandleType::INVALID, StdHandleType::CONSOLE, StdHandleType::FILE, StdHandleType::PIPE, StdHandleType::UNKNOWN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < StdHandleType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INVALID", "STD_HANDLE_INVALID", StdHandleType::INVALID), crate::meta::inspect::EnumConstant::new("CONSOLE", "STD_HANDLE_CONSOLE", StdHandleType::CONSOLE), crate::meta::inspect::EnumConstant::new("FILE", "STD_HANDLE_FILE", StdHandleType::FILE), crate::meta::inspect::EnumConstant::new("PIPE", "STD_HANDLE_PIPE", StdHandleType::PIPE), crate::meta::inspect::EnumConstant::new("UNKNOWN", "STD_HANDLE_UNKNOWN", StdHandleType::UNKNOWN)]
        }
    }
}
impl crate::meta::GodotConvert for StdHandleType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Std Handle Invalid", 0i64), EnumeratorShape::new_int("Std Handle Console", 1i64), EnumeratorShape::new_int("Std Handle File", 2i64), EnumeratorShape::new_int("Std Handle Pipe", 3i64), EnumeratorShape::new_int("Std Handle Unknown", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("OS.StdHandleType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for StdHandleType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StdHandleType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for StdHandleType {
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
impl crate::registry::property::Export for StdHandleType {
    
}
impl crate::meta::Element for StdHandleType {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Os;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Os {
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
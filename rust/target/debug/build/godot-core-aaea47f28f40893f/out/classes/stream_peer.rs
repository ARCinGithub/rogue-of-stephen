#![doc = "Sidecar module for class [`StreamPeer`][crate::classes::StreamPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StreamPeer` enums](https://docs.godotengine.org/en/stable/classes/class_streampeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `StreamPeer`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`stream_peer`][crate::classes::stream_peer]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `StreamPeer`](https://docs.godotengine.org/en/stable/classes/class_streampeer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<StreamPeer>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nStreamPeer is an abstract base class mostly used for stream-based protocols (such as TCP). It provides an API for sending and receiving data through streams as raw data or strings.\n\n**Note:** When exporting to Android, make sure to enable the `INTERNET` permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StreamPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl StreamPeer {
        #[doc = "Sends a chunk of data through the connection, blocking if necessary until the data is done sending. This function returns an \\[enum Error] code."]
        pub fn put_data(&mut self, data: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sends a chunk of data through the connection. If all the data could not be sent at once, only part of it will. This function returns two values, an \\[enum Error] code and an integer, describing how much data was actually sent."]
        pub fn put_partial_data(&mut self, data: &PackedByteArray,) -> VarArray {
            type CallRet = VarArray;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_partial_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a chunk data with the received bytes, as an [`Array`][crate::builtin::Array] containing two elements: an \\[enum Error] constant and a [`PackedByteArray`][crate::builtin::PackedByteArray]. `bytes` is the number of bytes to be received. If not enough bytes are available, the function will block until the desired amount is received."]
        pub fn get_data(&mut self, bytes: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a chunk data with the received bytes, as an [`Array`][crate::builtin::Array] containing two elements: an \\[enum Error] constant and a [`PackedByteArray`][crate::builtin::PackedByteArray]. `bytes` is the number of bytes to be received. If not enough bytes are available, the function will return how many were actually received."]
        pub fn get_partial_data(&mut self, bytes: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_partial_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of bytes this `StreamPeer` has available."]
        pub fn get_available_bytes(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_available_bytes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_big_endian(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "set_big_endian", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_big_endian_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "is_big_endian_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts a signed byte into the stream."]
        pub fn put_8(&mut self, value: i8,) {
            type CallRet = ();
            type CallParams = (i8,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_8", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts an unsigned byte into the stream."]
        pub fn put_u8(&mut self, value: u8,) {
            type CallRet = ();
            type CallParams = (u8,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_u8", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts a signed 16-bit value into the stream."]
        pub fn put_16(&mut self, value: i16,) {
            type CallRet = ();
            type CallParams = (i16,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_16", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts an unsigned 16-bit value into the stream."]
        pub fn put_u16(&mut self, value: u16,) {
            type CallRet = ();
            type CallParams = (u16,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_u16", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts a signed 32-bit value into the stream."]
        pub fn put_32(&mut self, value: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_32", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts an unsigned 32-bit value into the stream."]
        pub fn put_u32(&mut self, value: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11369usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_u32", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts a signed 64-bit value into the stream."]
        pub fn put_64(&mut self, value: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11370usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_64", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts an unsigned 64-bit value into the stream."]
        pub fn put_u64(&mut self, value: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11371usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_u64", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts a half-precision float into the stream."]
        pub fn put_half(&mut self, value: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11372usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_half", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts a single-precision float into the stream."]
        pub fn put_float(&mut self, value: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11373usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_float", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts a double-precision float into the stream."]
        pub fn put_double(&mut self, value: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11374usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_double", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts a zero-terminated ASCII string into the stream prepended by a 32-bit unsigned integer representing its size.\n\n**Note:** To put an ASCII string without prepending its size, you can use [`put_data`][`crate::classes::StreamPeer::put_data`]:\n\n\n```gdscript\nput_data(\"Hello world\".to_ascii_buffer())\n```\n"]
        pub fn put_string(&mut self, value: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (value.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11375usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts a zero-terminated UTF-8 string into the stream prepended by a 32 bits unsigned integer representing its size.\n\n**Note:** To put a UTF-8 string without prepending its size, you can use [`put_data`][`crate::classes::StreamPeer::put_data`]:\n\n\n```gdscript\nput_data(\"Hello world\".to_utf8_buffer())\n```\n"]
        pub fn put_utf8_string(&mut self, value: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (value.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_utf8_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Puts a Variant into the stream. If `full_objects` is `true` encoding objects is allowed (and can potentially include code).\n\nInternally, this uses the same encoding mechanism as the [`var_to_bytes`][`crate::global::var_to_bytes`] method."]
        pub(crate) fn put_var_full(&mut self, value: RefArg < Variant >, full_objects: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, bool,);
            let args = (value, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "put_var", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`put_var_ex`][Self::put_var_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Puts a Variant into the stream. If `full_objects` is `true` encoding objects is allowed (and can potentially include code).\n\nInternally, this uses the same encoding mechanism as the [`var_to_bytes`][`crate::global::var_to_bytes`] method."]
        #[inline]
        pub fn put_var(&mut self, value: &Variant,) {
            self.put_var_ex(value,) . done()
        }
        #[doc = "Puts a Variant into the stream. If `full_objects` is `true` encoding objects is allowed (and can potentially include code).\n\nInternally, this uses the same encoding mechanism as the [`var_to_bytes`][`crate::global::var_to_bytes`] method."]
        #[inline]
        pub fn put_var_ex < 'ex > (&'ex mut self, value: &'ex Variant,) -> ExPutVar < 'ex > {
            ExPutVar::new(self, value,)
        }
        #[doc = "Gets a signed byte from the stream."]
        pub fn get_8(&mut self,) -> i8 {
            type CallRet = i8;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_8", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets an unsigned byte from the stream."]
        pub fn get_u8(&mut self,) -> u8 {
            type CallRet = u8;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_u8", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a signed 16-bit value from the stream."]
        pub fn get_16(&mut self,) -> i16 {
            type CallRet = i16;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_16", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets an unsigned 16-bit value from the stream."]
        pub fn get_u16(&mut self,) -> u16 {
            type CallRet = u16;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_u16", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a signed 32-bit value from the stream."]
        pub fn get_32(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_32", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets an unsigned 32-bit value from the stream."]
        pub fn get_u32(&mut self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_u32", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a signed 64-bit value from the stream."]
        pub fn get_64(&mut self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_64", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets an unsigned 64-bit value from the stream."]
        pub fn get_u64(&mut self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_u64", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a half-precision float from the stream."]
        pub fn get_half(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_half", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a single-precision float from the stream."]
        pub fn get_float(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_float", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a double-precision float from the stream."]
        pub fn get_double(&mut self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_double", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets an ASCII string with byte-length `bytes` from the stream. If `bytes` is negative (default) the length will be read from the stream using the reverse process of [`put_string`][`crate::classes::StreamPeer::put_string`]."]
        pub(crate) fn get_string_full(&mut self, bytes: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_string_ex`][Self::get_string_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Gets an ASCII string with byte-length `bytes` from the stream. If `bytes` is negative (default) the length will be read from the stream using the reverse process of [`put_string`][`crate::classes::StreamPeer::put_string`]."]
        #[inline]
        pub fn get_string(&mut self,) -> GString {
            self.get_string_ex() . done()
        }
        #[doc = "Gets an ASCII string with byte-length `bytes` from the stream. If `bytes` is negative (default) the length will be read from the stream using the reverse process of [`put_string`][`crate::classes::StreamPeer::put_string`]."]
        #[inline]
        pub fn get_string_ex < 'ex > (&'ex mut self,) -> ExGetString < 'ex > {
            ExGetString::new(self,)
        }
        #[doc = "Gets a UTF-8 string with byte-length `bytes` from the stream (this decodes the string sent as UTF-8). If `bytes` is negative (default) the length will be read from the stream using the reverse process of [`put_utf8_string`][`crate::classes::StreamPeer::put_utf8_string`]."]
        pub(crate) fn get_utf8_string_full(&mut self, bytes: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_utf8_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_utf8_string_ex`][Self::get_utf8_string_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Gets a UTF-8 string with byte-length `bytes` from the stream (this decodes the string sent as UTF-8). If `bytes` is negative (default) the length will be read from the stream using the reverse process of [`put_utf8_string`][`crate::classes::StreamPeer::put_utf8_string`]."]
        #[inline]
        pub fn get_utf8_string(&mut self,) -> GString {
            self.get_utf8_string_ex() . done()
        }
        #[doc = "Gets a UTF-8 string with byte-length `bytes` from the stream (this decodes the string sent as UTF-8). If `bytes` is negative (default) the length will be read from the stream using the reverse process of [`put_utf8_string`][`crate::classes::StreamPeer::put_utf8_string`]."]
        #[inline]
        pub fn get_utf8_string_ex < 'ex > (&'ex mut self,) -> ExGetUtf8String < 'ex > {
            ExGetUtf8String::new(self,)
        }
        #[doc = "Gets a Variant from the stream. If `allow_objects` is `true`, decoding objects is allowed.\n\nInternally, this uses the same decoding mechanism as the [`bytes_to_var`][`crate::global::bytes_to_var`] method.\n\n**Warning:** Deserialized objects can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats such as remote code execution."]
        pub(crate) fn get_var_full(&mut self, allow_objects: bool,) -> Variant {
            type CallRet = Variant;
            type CallParams = (bool,);
            let args = (allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeer", "get_var", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_var_ex`][Self::get_var_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Gets a Variant from the stream. If `allow_objects` is `true`, decoding objects is allowed.\n\nInternally, this uses the same decoding mechanism as the [`bytes_to_var`][`crate::global::bytes_to_var`] method.\n\n**Warning:** Deserialized objects can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats such as remote code execution."]
        #[inline]
        pub fn get_var(&mut self,) -> Variant {
            self.get_var_ex() . done()
        }
        #[doc = "Gets a Variant from the stream. If `allow_objects` is `true`, decoding objects is allowed.\n\nInternally, this uses the same decoding mechanism as the [`bytes_to_var`][`crate::global::bytes_to_var`] method.\n\n**Warning:** Deserialized objects can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats such as remote code execution."]
        #[inline]
        pub fn get_var_ex < 'ex > (&'ex mut self,) -> ExGetVar < 'ex > {
            ExGetVar::new(self,)
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
    impl crate::obj::GodotClass for StreamPeer {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("StreamPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StreamPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for StreamPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for StreamPeer {
        
    }
    impl std::ops::Deref for StreamPeer {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StreamPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_StreamPeer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `StreamPeer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`StreamPeer::put_var_ex`][super::StreamPeer::put_var_ex]."]
#[must_use]
pub struct ExPutVar < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::StreamPeer, value: CowArg < 'ex, Variant >, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPutVar < 'ex > {
    fn new(surround_object: &'ex mut re_export::StreamPeer, value: &'ex Variant,) -> Self {
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
    pub fn done(self) {
        let Self {
            _phantom, surround_object, value, full_objects,
        }
        = self;
        re_export::StreamPeer::put_var_full(surround_object, value.cow_as_arg(), full_objects,)
    }
}
#[doc = "Default-param extender for [`StreamPeer::get_string_ex`][super::StreamPeer::get_string_ex]."]
#[must_use]
pub struct ExGetString < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::StreamPeer, bytes: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetString < 'ex > {
    fn new(surround_object: &'ex mut re_export::StreamPeer,) -> Self {
        let bytes = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bytes: bytes,
        }
    }
    #[inline]
    pub fn bytes(self, bytes: i32) -> Self {
        Self {
            bytes: bytes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, bytes,
        }
        = self;
        re_export::StreamPeer::get_string_full(surround_object, bytes,)
    }
}
#[doc = "Default-param extender for [`StreamPeer::get_utf8_string_ex`][super::StreamPeer::get_utf8_string_ex]."]
#[must_use]
pub struct ExGetUtf8String < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::StreamPeer, bytes: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetUtf8String < 'ex > {
    fn new(surround_object: &'ex mut re_export::StreamPeer,) -> Self {
        let bytes = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bytes: bytes,
        }
    }
    #[inline]
    pub fn bytes(self, bytes: i32) -> Self {
        Self {
            bytes: bytes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, bytes,
        }
        = self;
        re_export::StreamPeer::get_utf8_string_full(surround_object, bytes,)
    }
}
#[doc = "Default-param extender for [`StreamPeer::get_var_ex`][super::StreamPeer::get_var_ex]."]
#[must_use]
pub struct ExGetVar < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::StreamPeer, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetVar < 'ex > {
    fn new(surround_object: &'ex mut re_export::StreamPeer,) -> Self {
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
        re_export::StreamPeer::get_var_full(surround_object, allow_objects,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::StreamPeer;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for StreamPeer {
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
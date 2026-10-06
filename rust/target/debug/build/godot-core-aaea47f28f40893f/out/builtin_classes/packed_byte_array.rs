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
pub(super) mod re_export {
    use super::*;
    #[doc(hidden)]
    #[repr(transparent)]
    pub struct InnerPackedByteArray < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerPackedByteArray < 'inner > {
    pub fn from_outer(outer: &PackedByteArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the byte at the given `index` in the array. If `index` is out-of-bounds or negative, this method fails and returns `0`.\n\nThis method is similar (but not identical) to the `[]` operator. Most notably, when this method fails, it doesn't pause project execution if run from the editor."]
    pub fn get(&self, index: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(722usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get", self.sys_ptr, args)
        }
    }
    #[doc = "Changes the byte at the given index."]
    pub fn set(&mut self, index: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(723usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "set", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of elements in the array."]
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(724usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "size", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the array is empty."]
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(725usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "is_empty", self.sys_ptr, args)
        }
    }
    #[doc = "Appends an element at the end of the array."]
    pub fn push_back(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(726usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "push_back", self.sys_ptr, args)
        }
    }
    #[doc = "Appends an element at the end of the array (alias of \\[method push_back])."]
    pub fn append(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(727usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "append", self.sys_ptr, args)
        }
    }
    #[doc = "Appends a [`PackedByteArray`][crate::builtin::PackedByteArray] at the end of this array."]
    pub fn append_array(&mut self, array: &PackedByteArray,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(728usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "append_array", self.sys_ptr, args)
        }
    }
    #[doc = "Removes an element from the array by index."]
    pub fn remove_at(&mut self, index: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(729usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "remove_at", self.sys_ptr, args)
        }
    }
    #[doc = "Inserts a new element at a given position in the array. The position must be valid, or at the end of the array (`idx == size()`)."]
    pub fn insert(&mut self, at_index: i64, value: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(730usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "insert", self.sys_ptr, args)
        }
    }
    #[doc = "Assigns the given value to all elements in the array. This can typically be used together with \\[method resize] to create an array with a given size and initialized elements."]
    pub fn fill(&mut self, value: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(731usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "fill", self.sys_ptr, args)
        }
    }
    #[doc = "Sets the size of the array. If the array is grown, reserves elements at the end of the array. If the array is shrunk, truncates the array to the new size. Calling \\[method resize] once and assigning the new values is faster than adding new elements one by one.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the following \\[enum Error] constants if this method fails: [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if the size is negative, or [`Error::ERR_OUT_OF_MEMORY`][`crate::global::Error::ERR_OUT_OF_MEMORY`] if allocations fail. Use \\[method size] to find the actual size of the array after resize."]
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(732usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "resize", self.sys_ptr, args)
        }
    }
    #[doc = "Clears the array. This is equivalent to using \\[method resize] with a size of `0`."]
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(733usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "clear", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the array contains `value`."]
    pub fn has(&self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(734usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "has", self.sys_ptr, args)
        }
    }
    #[doc = "Reverses the order of the elements in the array."]
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(735usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "reverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the slice of the [`PackedByteArray`][crate::builtin::PackedByteArray], from `begin` (inclusive) to `end` (exclusive), as a new [`PackedByteArray`][crate::builtin::PackedByteArray].\n\nThe absolute value of `begin` and `end` will be clamped to the array size, so the default value for `end` makes it slice to the size of the array by default (i.e. `arr.slice(1)` is a shorthand for `arr.slice(1, arr.size())`).\n\nIf either `begin` or `end` are negative, they will be relative to the end of the array (i.e. `arr.slice(0, -2)` is a shorthand for `arr.slice(0, arr.size() - 2)`)."]
    pub fn slice(&self, begin: i64, end: i64,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(736usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "slice", self.sys_ptr, args)
        }
    }
    #[doc = "Sorts the elements of the array in ascending order."]
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(737usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "sort", self.sys_ptr, args)
        }
    }
    #[doc = "Finds the index of an existing value (or the insertion index that maintains sorting order, if the value is not yet present in the array) using binary search. Optionally, a `before` specifier can be passed. If `false`, the returned index comes after all existing entries of the value in the array.\n\n**Note:** Calling \\[method bsearch] on an unsorted array results in unexpected behavior."]
    pub fn bsearch(&self, value: i64, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, bool,);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(738usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "bsearch", self.sys_ptr, args)
        }
    }
    #[doc = "Creates a copy of the array, and returns it."]
    pub fn duplicate(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(739usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "duplicate", self.sys_ptr, args)
        }
    }
    #[doc = "Searches the array for a value and returns its index or `-1` if not found. Optionally, the initial search index can be passed."]
    pub fn find(&self, value: i64, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(740usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "find", self.sys_ptr, args)
        }
    }
    #[doc = "Searches the array in reverse order. Optionally, a start search index can be passed. If negative, the start index is considered relative to the end of the array."]
    pub fn rfind(&self, value: i64, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(741usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "rfind", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of times an element is in the array."]
    pub fn count(&self, value: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(742usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "count", self.sys_ptr, args)
        }
    }
    #[doc = "Removes the first occurrence of a value from the array and returns `true`. If the value does not exist in the array, nothing happens and `false` is returned. To remove an element by index, use \\[method remove_at] instead."]
    pub fn erase(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(743usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "erase", self.sys_ptr, args)
        }
    }
    #[doc = "Converts system multibyte code page encoded array to [`String`][crate::builtin::GString]. If conversion fails, empty string is returned. This is the inverse of [`to_multibyte_char_buffer`][`crate::builtin::GString::to_multibyte_char_buffer`].\n\nThe values permitted for `encoding` are system dependent. If `encoding` is empty string, system default encoding is used.\n\n- For Windows, see [Code Page Identifiers](https://learn.microsoft.com/en-us/windows/win32/Intl/code-page-identifiers) .NET names.\n\n- For macOS and Linux/BSD, see `libiconv` library documentation and `iconv --list` for a list of supported encodings."]
    pub fn get_string_from_multibyte_char(&self, encoding: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (encoding.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(749usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_multibyte_char", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new [`PackedByteArray`][crate::builtin::PackedByteArray] with the data compressed. Set the compression mode using one of \\[enum FileAccess.CompressionMode]'s constants."]
    pub fn compress(&self, compression_mode: i64,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = (i64,);
        let args = (compression_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(751usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "compress", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new [`PackedByteArray`][crate::builtin::PackedByteArray] with the data decompressed. Set `buffer_size` to the size of the uncompressed data. Set the compression mode using one of \\[enum FileAccess.CompressionMode]'s constants.\n\n**Note:** Decompression is not guaranteed to work with data not compressed by Godot, for example if data compressed with the deflate compression mode lacks a checksum or header."]
    pub fn decompress(&self, buffer_size: i64, compression_mode: i64,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = (i64, i64,);
        let args = (buffer_size, compression_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(752usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decompress", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new [`PackedByteArray`][crate::builtin::PackedByteArray] with the data decompressed. Set the compression mode using one of \\[enum FileAccess.CompressionMode]'s constants. **This method only accepts brotli, gzip, and deflate compression modes.**\n\nThis method is potentially slower than \\[method decompress], as it may have to re-allocate its output buffer multiple times while decompressing, whereas \\[method decompress] knows it's output buffer size from the beginning.\n\nGZIP has a maximal compression ratio of 1032:1, meaning it's very possible for a small compressed payload to decompress to a potentially very large output. To guard against this, you may provide a maximum size this function is allowed to allocate in bytes via `max_output_size`. Passing -1 will allow for unbounded output. If any positive value is passed, and the decompression exceeds that amount in bytes, then an error will be returned.\n\n**Note:** Decompression is not guaranteed to work with data not compressed by Godot, for example if data compressed with the deflate compression mode lacks a checksum or header."]
    pub fn decompress_dynamic(&self, max_output_size: i64, compression_mode: i64,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = (i64, i64,);
        let args = (max_output_size, compression_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(753usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decompress_dynamic", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a 8-bit unsigned integer number from the bytes starting at `byte_offset`. Fails if the byte count is insufficient. Returns `0` if a valid number can't be decoded."]
    pub fn decode_u8(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(754usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_u8", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a 8-bit signed integer number from the bytes starting at `byte_offset`. Fails if the byte count is insufficient. Returns `0` if a valid number can't be decoded."]
    pub fn decode_s8(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(755usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_s8", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a 16-bit unsigned integer number from the bytes starting at `byte_offset`. Fails if the byte count is insufficient. Returns `0` if a valid number can't be decoded."]
    pub fn decode_u16(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(756usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_u16", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a 16-bit signed integer number from the bytes starting at `byte_offset`. Fails if the byte count is insufficient. Returns `0` if a valid number can't be decoded."]
    pub fn decode_s16(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(757usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_s16", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a 32-bit unsigned integer number from the bytes starting at `byte_offset`. Fails if the byte count is insufficient. Returns `0` if a valid number can't be decoded."]
    pub fn decode_u32(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(758usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_u32", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a 32-bit signed integer number from the bytes starting at `byte_offset`. Fails if the byte count is insufficient. Returns `0` if a valid number can't be decoded."]
    pub fn decode_s32(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(759usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_s32", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a 64-bit unsigned integer number from the bytes starting at `byte_offset`. Fails if the byte count is insufficient. Returns `0` if a valid number can't be decoded."]
    pub fn decode_u64(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(760usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_u64", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a 64-bit signed integer number from the bytes starting at `byte_offset`. Fails if the byte count is insufficient. Returns `0` if a valid number can't be decoded."]
    pub fn decode_s64(&self, byte_offset: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(761usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_s64", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a 16-bit floating-point number from the bytes starting at `byte_offset`. Fails if the byte count is insufficient. Returns `0.0` if a valid number can't be decoded."]
    pub fn decode_half(&self, byte_offset: i64,) -> f64 {
        type CallRet = f64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(762usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_half", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a 32-bit floating-point number from the bytes starting at `byte_offset`. Fails if the byte count is insufficient. Returns `0.0` if a valid number can't be decoded."]
    pub fn decode_float(&self, byte_offset: i64,) -> f64 {
        type CallRet = f64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(763usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_float", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a 64-bit floating-point number from the bytes starting at `byte_offset`. Fails if the byte count is insufficient. Returns `0.0` if a valid number can't be decoded."]
    pub fn decode_double(&self, byte_offset: i64,) -> f64 {
        type CallRet = f64;
        type CallParams = (i64,);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(764usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_double", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if a valid [`Variant`][crate::builtin::Variant] value can be decoded at the `byte_offset`. Returns `false` otherwise or when the value is [`Object`][crate::classes::Object]-derived and `allow_objects` is `false`."]
    pub fn has_encoded_var(&self, byte_offset: i64, allow_objects: bool,) -> bool {
        type CallRet = bool;
        type CallParams = (i64, bool,);
        let args = (byte_offset, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(765usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "has_encoded_var", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a [`Variant`][crate::builtin::Variant] from the bytes starting at `byte_offset`. Returns `null` if a valid variant can't be decoded or the value is [`Object`][crate::classes::Object]-derived and `allow_objects` is `false`."]
    pub fn decode_var(&self, byte_offset: i64, allow_objects: bool,) -> Variant {
        type CallRet = Variant;
        type CallParams = (i64, bool,);
        let args = (byte_offset, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(766usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_var", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes a size of a [`Variant`][crate::builtin::Variant] from the bytes starting at `byte_offset`. Requires at least 4 bytes of data starting at the offset, otherwise fails."]
    pub fn decode_var_size(&self, byte_offset: i64, allow_objects: bool,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, bool,);
        let args = (byte_offset, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(767usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "decode_var_size", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the data converted to a [`PackedInt32Array`][crate::builtin::PackedInt32Array], where each block of 4 bytes has been converted to a signed 32-bit integer (C++ `int32_t`).\n\nThe size of the input array must be a multiple of 4 (size of 32-bit integer). The size of the new array will be `byte_array.size() / 4`.\n\nIf the original data can't be converted to signed 32-bit integers, the resulting data is undefined."]
    pub fn to_int32_array(&self,) -> PackedInt32Array {
        type CallRet = PackedInt32Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(768usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_int32_array", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the data converted to a [`PackedInt64Array`][crate::builtin::PackedInt64Array], where each block of 8 bytes has been converted to a signed 64-bit integer (C++ `int64_t`, Godot `int`).\n\nThe size of the input array must be a multiple of 8 (size of 64-bit integer). The size of the new array will be `byte_array.size() / 8`.\n\nIf the original data can't be converted to signed 64-bit integers, the resulting data is undefined."]
    pub fn to_int64_array(&self,) -> PackedInt64Array {
        type CallRet = PackedInt64Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(769usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_int64_array", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the data converted to a [`PackedFloat32Array`][crate::builtin::PackedFloat32Array], where each block of 4 bytes has been converted to a 32-bit float (C++ `float`).\n\nThe size of the input array must be a multiple of 4 (size of 32-bit float). The size of the new array will be `byte_array.size() / 4`.\n\nIf the original data can't be converted to 32-bit floats, the resulting data is undefined."]
    pub fn to_float32_array(&self,) -> PackedFloat32Array {
        type CallRet = PackedFloat32Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(770usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_float32_array", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the data converted to a [`PackedFloat64Array`][crate::builtin::PackedFloat64Array], where each block of 8 bytes has been converted to a 64-bit float (C++ `double`, Godot `float`).\n\nThe size of the input array must be a multiple of 8 (size of 64-bit double). The size of the new array will be `byte_array.size() / 8`.\n\nIf the original data can't be converted to 64-bit floats, the resulting data is undefined."]
    pub fn to_float64_array(&self,) -> PackedFloat64Array {
        type CallRet = PackedFloat64Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(771usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_float64_array", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the data converted to a [`PackedVector2Array`][crate::builtin::PackedVector2Array], where each block of 8 bytes or 16 bytes (32-bit or 64-bit) has been converted to a [`Vector2`][crate::builtin::Vector2] variant.\n\n**Note:** The size of the input array must be a multiple of 8 or 16 (depending on the build settings, see [`Vector2`][crate::builtin::Vector2] for more details). The size of the new array will be `byte_array.size() / (8 or 16)`. If the original data can't be converted to [`Vector2`][crate::builtin::Vector2] variants, the resulting data is undefined."]
    pub fn to_vector2_array(&self,) -> PackedVector2Array {
        type CallRet = PackedVector2Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(772usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_vector2_array", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the data converted to a [`PackedVector3Array`][crate::builtin::PackedVector3Array], where each block of 12 or 24 bytes (32-bit or 64-bit) has been converted to a [`Vector3`][crate::builtin::Vector3] variant.\n\n**Note:** The size of the input array must be a multiple of 12 or 24 (depending on the build settings, see [`Vector3`][crate::builtin::Vector3] for more details). The size of the new array will be `byte_array.size() / (12 or 24)`. If the original data can't be converted to [`Vector3`][crate::builtin::Vector3] variants, the resulting data is undefined."]
    pub fn to_vector3_array(&self,) -> PackedVector3Array {
        type CallRet = PackedVector3Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(773usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_vector3_array", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the data converted to a [`PackedVector4Array`][crate::builtin::PackedVector4Array], where each block of 16 or 32 bytes (32-bit or 64-bit) has been converted to a [`Vector4`][crate::builtin::Vector4] variant.\n\n**Note:** The size of the input array must be a multiple of 16 or 32 (depending on the build settings, see [`Vector4`][crate::builtin::Vector4] for more details). The size of the new array will be `byte_array.size() / (16 or 32)`. If the original data can't be converted to [`Vector4`][crate::builtin::Vector4] variants, the resulting data is undefined."]
    pub fn to_vector4_array(&self,) -> PackedVector4Array {
        type CallRet = PackedVector4Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(774usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_vector4_array", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the data converted to a [`PackedColorArray`][crate::builtin::PackedColorArray], where each block of 16 bytes has been converted to a [`Color`][crate::builtin::Color] variant.\n\n**Note:** The size of the input array must be a multiple of 16 (size of four 32-bit float variables). The size of the new array will be `byte_array.size() / 16`. If the original data can't be converted to [`Color`][crate::builtin::Color] variants, the resulting data is undefined."]
    pub fn to_color_array(&self,) -> PackedColorArray {
        type CallRet = PackedColorArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(775usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "to_color_array", self.sys_ptr, args)
        }
    }
    #[doc = "Swaps the byte order of `count` 16-bit segments of the array starting at `offset`. Swap is done in-place. If `count` is less than zero, all segments to the end of array are processed, if processed data size is not a multiple of 2, the byte after the last processed 16-bit segment is not modified."]
    pub fn bswap16(&mut self, offset: i64, count: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (offset, count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(776usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "bswap16", self.sys_ptr, args)
        }
    }
    #[doc = "Swaps the byte order of `count` 32-bit segments of the array starting at `offset`. Swap is done in-place. If `count` is less than zero, all segments to the end of array are processed, if processed data size is not a multiple of 4, bytes after the last processed 32-bit segment are not modified."]
    pub fn bswap32(&mut self, offset: i64, count: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (offset, count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(777usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "bswap32", self.sys_ptr, args)
        }
    }
    #[doc = "Swaps the byte order of `count` 64-bit segments of the array starting at `offset`. Swap is done in-place. If `count` is less than zero, all segments to the end of array are processed, if processed data size is not a multiple of 8, bytes after the last processed 64-bit segment are not modified."]
    pub fn bswap64(&mut self, offset: i64, count: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (offset, count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(778usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "bswap64", self.sys_ptr, args)
        }
    }
    #[doc = "Encodes a 8-bit unsigned integer number (byte) at the index of `byte_offset` bytes. The array must have at least 1 byte of space, starting at the offset."]
    pub fn encode_u8(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(779usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_u8", self.sys_ptr, args)
        }
    }
    #[doc = "Encodes a 8-bit signed integer number (signed byte) at the index of `byte_offset` bytes. The array must have at least 1 byte of space, starting at the offset."]
    pub fn encode_s8(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(780usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_s8", self.sys_ptr, args)
        }
    }
    #[doc = "Encodes a 16-bit unsigned integer number as bytes at the index of `byte_offset` bytes. The array must have at least 2 bytes of space, starting at the offset."]
    pub fn encode_u16(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(781usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_u16", self.sys_ptr, args)
        }
    }
    #[doc = "Encodes a 16-bit signed integer number as bytes at the index of `byte_offset` bytes. The array must have at least 2 bytes of space, starting at the offset."]
    pub fn encode_s16(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(782usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_s16", self.sys_ptr, args)
        }
    }
    #[doc = "Encodes a 32-bit unsigned integer number as bytes at the index of `byte_offset` bytes. The array must have at least 4 bytes of space, starting at the offset."]
    pub fn encode_u32(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(783usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_u32", self.sys_ptr, args)
        }
    }
    #[doc = "Encodes a 32-bit signed integer number as bytes at the index of `byte_offset` bytes. The array must have at least 4 bytes of space, starting at the offset."]
    pub fn encode_s32(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(784usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_s32", self.sys_ptr, args)
        }
    }
    #[doc = "Encodes a 64-bit unsigned integer number as bytes at the index of `byte_offset` bytes. The array must have at least 8 bytes of space, starting at the offset."]
    pub fn encode_u64(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(785usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_u64", self.sys_ptr, args)
        }
    }
    #[doc = "Encodes a 64-bit signed integer number as bytes at the index of `byte_offset` bytes. The array must have at least 8 bytes of space, starting at the offset."]
    pub fn encode_s64(&mut self, byte_offset: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(786usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_s64", self.sys_ptr, args)
        }
    }
    #[doc = "Encodes a 16-bit floating-point number as bytes at the index of `byte_offset` bytes. The array must have at least 2 bytes of space, starting at the offset."]
    pub fn encode_half(&mut self, byte_offset: i64, value: f64,) {
        type CallRet = ();
        type CallParams = (i64, f64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(787usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_half", self.sys_ptr, args)
        }
    }
    #[doc = "Encodes a 32-bit floating-point number as bytes at the index of `byte_offset` bytes. The array must have at least 4 bytes of space, starting at the offset."]
    pub fn encode_float(&mut self, byte_offset: i64, value: f64,) {
        type CallRet = ();
        type CallParams = (i64, f64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(788usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_float", self.sys_ptr, args)
        }
    }
    #[doc = "Encodes a 64-bit floating-point number as bytes at the index of `byte_offset` bytes. The array must have at least 8 bytes of allocated space, starting at the offset."]
    pub fn encode_double(&mut self, byte_offset: i64, value: f64,) {
        type CallRet = ();
        type CallParams = (i64, f64,);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(789usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_double", self.sys_ptr, args)
        }
    }
    #[doc = "Encodes a [`Variant`][crate::builtin::Variant] at the index of `byte_offset` bytes. A sufficient space must be allocated, depending on the encoded variant's size. If `allow_objects` is `false`, [`Object`][crate::classes::Object]-derived values are not permitted and will instead be serialized as ID-only."]
    pub fn encode_var(&mut self, byte_offset: i64, value: &Variant, allow_objects: bool,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (i64, RefArg < 'a0, Variant >, bool,);
        let args = (byte_offset, RefArg::new(value), allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(790usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "encode_var", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerPackedByteArray;
impl PackedByteArray {
    #[doc = "Converts ASCII/Latin-1 encoded array to [`String`][crate::builtin::GString]. Fast alternative to \\[method get_string_from_utf8] if the content is ASCII/Latin-1 only. Unlike the UTF-8 function this function maps every byte to a character in the array. Multibyte sequences will not be interpreted correctly. For parsing user input always use \\[method get_string_from_utf8]. This is the inverse of [`to_ascii_buffer`][`crate::builtin::GString::to_ascii_buffer`]."]
    pub fn get_string_from_ascii(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(744usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_ascii", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts UTF-8 encoded array to [`String`][crate::builtin::GString]. Slower than \\[method get_string_from_ascii] but supports UTF-8 encoded data. Use this function if you are unsure about the source of the data. For user input this function should always be preferred. Returns empty string if source array is not valid UTF-8 string. This is the inverse of [`to_utf8_buffer`][`crate::builtin::GString::to_utf8_buffer`]."]
    pub fn get_string_from_utf8(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(745usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_utf8", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts UTF-16 encoded array to [`String`][crate::builtin::GString]. If the BOM is missing, little-endianness is assumed. Returns empty string if source array is not valid UTF-16 string. This is the inverse of [`to_utf16_buffer`][`crate::builtin::GString::to_utf16_buffer`]."]
    pub fn get_string_from_utf16(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(746usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_utf16", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts UTF-32 encoded array to [`String`][crate::builtin::GString]. Returns empty string if source array is not valid UTF-32 string. This is the inverse of [`to_utf32_buffer`][`crate::builtin::GString::to_utf32_buffer`]."]
    pub fn get_string_from_utf32(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(747usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_utf32", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts wide character (`wchar_t`, UTF-16 on Windows, UTF-32 on other platforms) encoded array to [`String`][crate::builtin::GString]. Returns empty string if source array is not valid wide string. This is the inverse of [`to_wchar_buffer`][`crate::builtin::GString::to_wchar_buffer`]."]
    pub fn get_string_from_wchar(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(748usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "get_string_from_wchar", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns a hexadecimal representation of this array as a [`String`][crate::builtin::GString].\n\n\n```gdscript\nvar array = PackedByteArray([11, 46, 255])\nprint(array.hex_encode()) # Prints \"0b2eff\"\n```\n"]
    pub fn hex_encode(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(750usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedByteArray", "hex_encode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
}
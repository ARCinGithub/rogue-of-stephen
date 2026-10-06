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
    pub struct InnerPackedStringArray < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerPackedStringArray < 'inner > {
    pub fn from_outer(outer: &PackedStringArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the [`String`][crate::builtin::GString] at the given `index` in the array. If `index` is out-of-bounds or negative, this method fails and returns an empty string.\n\nThis method is similar (but not identical) to the `[]` operator. Most notably, when this method fails, it doesn't pause project execution if run from the editor."]
    pub fn get(&self, index: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(883usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "get", self.sys_ptr, args)
        }
    }
    #[doc = "Changes the [`String`][crate::builtin::GString] at the given index."]
    pub fn set(&mut self, index: i64, value: impl AsArg < GString >,) {
        type CallRet = ();
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (index, value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(884usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "set", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of elements in the array."]
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(885usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "size", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the array is empty."]
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(886usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "is_empty", self.sys_ptr, args)
        }
    }
    #[doc = "Appends a string element at end of the array."]
    pub fn push_back(&mut self, value: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(887usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "push_back", self.sys_ptr, args)
        }
    }
    #[doc = "Appends an element at the end of the array (alias of \\[method push_back])."]
    pub fn append(&mut self, value: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(888usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "append", self.sys_ptr, args)
        }
    }
    #[doc = "Appends a [`PackedStringArray`][crate::builtin::PackedStringArray] at the end of this array."]
    pub fn append_array(&mut self, array: &PackedStringArray,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(889usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "append_array", self.sys_ptr, args)
        }
    }
    #[doc = "Removes an element from the array by index."]
    pub fn remove_at(&mut self, index: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(890usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "remove_at", self.sys_ptr, args)
        }
    }
    #[doc = "Inserts a new element at a given position in the array. The position must be valid, or at the end of the array (`idx == size()`)."]
    pub fn insert(&mut self, at_index: i64, value: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (at_index, value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(891usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "insert", self.sys_ptr, args)
        }
    }
    #[doc = "Assigns the given value to all elements in the array. This can typically be used together with \\[method resize] to create an array with a given size and initialized elements."]
    pub fn fill(&mut self, value: impl AsArg < GString >,) {
        type CallRet = ();
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(892usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "fill", self.sys_ptr, args)
        }
    }
    #[doc = "Sets the size of the array. If the array is grown, reserves elements at the end of the array. If the array is shrunk, truncates the array to the new size. Calling \\[method resize] once and assigning the new values is faster than adding new elements one by one.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the following \\[enum Error] constants if this method fails: [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if the size is negative, or [`Error::ERR_OUT_OF_MEMORY`][`crate::global::Error::ERR_OUT_OF_MEMORY`] if allocations fail. Use \\[method size] to find the actual size of the array after resize."]
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(893usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "resize", self.sys_ptr, args)
        }
    }
    #[doc = "Clears the array. This is equivalent to using \\[method resize] with a size of `0`."]
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(894usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "clear", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the array contains `value`."]
    pub fn has(&self, value: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(895usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "has", self.sys_ptr, args)
        }
    }
    #[doc = "Reverses the order of the elements in the array."]
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(896usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "reverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the slice of the [`PackedStringArray`][crate::builtin::PackedStringArray], from `begin` (inclusive) to `end` (exclusive), as a new [`PackedStringArray`][crate::builtin::PackedStringArray].\n\nThe absolute value of `begin` and `end` will be clamped to the array size, so the default value for `end` makes it slice to the size of the array by default (i.e. `arr.slice(1)` is a shorthand for `arr.slice(1, arr.size())`).\n\nIf either `begin` or `end` are negative, they will be relative to the end of the array (i.e. `arr.slice(0, -2)` is a shorthand for `arr.slice(0, arr.size() - 2)`)."]
    pub fn slice(&self, begin: i64, end: i64,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(897usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "slice", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a [`PackedByteArray`][crate::builtin::PackedByteArray] with each string encoded as UTF-8. Strings are `null` terminated."]
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(898usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "to_byte_array", self.sys_ptr, args)
        }
    }
    #[doc = "Sorts the elements of the array in ascending order."]
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(899usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "sort", self.sys_ptr, args)
        }
    }
    #[doc = "Finds the index of an existing value (or the insertion index that maintains sorting order, if the value is not yet present in the array) using binary search. Optionally, a `before` specifier can be passed. If `false`, the returned index comes after all existing entries of the value in the array.\n\n**Note:** Calling \\[method bsearch] on an unsorted array results in unexpected behavior."]
    pub fn bsearch(&self, value: impl AsArg < GString >, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
        let args = (value.into_arg(), before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(900usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "bsearch", self.sys_ptr, args)
        }
    }
    #[doc = "Creates a copy of the array, and returns it."]
    pub fn duplicate(&self,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(901usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "duplicate", self.sys_ptr, args)
        }
    }
    #[doc = "Searches the array for a value and returns its index or `-1` if not found. Optionally, the initial search index can be passed."]
    pub fn find(&self, value: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (value.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(902usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "find", self.sys_ptr, args)
        }
    }
    #[doc = "Searches the array in reverse order. Optionally, a start search index can be passed. If negative, the start index is considered relative to the end of the array."]
    pub fn rfind(&self, value: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (value.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(903usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "rfind", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of times an element is in the array."]
    pub fn count(&self, value: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(904usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "count", self.sys_ptr, args)
        }
    }
    #[doc = "Removes the first occurrence of a value from the array and returns `true`. If the value does not exist in the array, nothing happens and `false` is returned. To remove an element by index, use \\[method remove_at] instead."]
    pub fn erase(&mut self, value: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(905usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "erase", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerPackedStringArray;
impl PackedStringArray {
    
}
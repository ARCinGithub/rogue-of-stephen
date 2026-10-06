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
    pub struct InnerPackedInt32Array < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerPackedInt32Array < 'inner > {
    pub fn from_outer(outer: &PackedInt32Array) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the 32-bit integer at the given `index` in the array. If `index` is out-of-bounds or negative, this method fails and returns `0`.\n\nThis method is similar (but not identical) to the `[]` operator. Most notably, when this method fails, it doesn't pause project execution if run from the editor."]
    pub fn get(&self, index: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(791usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "get", self.sys_ptr, args)
        }
    }
    #[doc = "Changes the integer at the given index."]
    pub fn set(&mut self, index: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(792usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "set", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of elements in the array."]
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(793usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "size", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the array is empty."]
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(794usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "is_empty", self.sys_ptr, args)
        }
    }
    #[doc = "Appends a value to the array."]
    pub fn push_back(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(795usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "push_back", self.sys_ptr, args)
        }
    }
    #[doc = "Appends an element at the end of the array (alias of \\[method push_back])."]
    pub fn append(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(796usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "append", self.sys_ptr, args)
        }
    }
    #[doc = "Appends a [`PackedInt32Array`][crate::builtin::PackedInt32Array] at the end of this array."]
    pub fn append_array(&mut self, array: &PackedInt32Array,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, PackedInt32Array >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(797usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "append_array", self.sys_ptr, args)
        }
    }
    #[doc = "Removes an element from the array by index."]
    pub fn remove_at(&mut self, index: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(798usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "remove_at", self.sys_ptr, args)
        }
    }
    #[doc = "Inserts a new integer at a given position in the array. The position must be valid, or at the end of the array (`idx == size()`)."]
    pub fn insert(&mut self, at_index: i64, value: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(799usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "insert", self.sys_ptr, args)
        }
    }
    #[doc = "Assigns the given value to all elements in the array. This can typically be used together with \\[method resize] to create an array with a given size and initialized elements."]
    pub fn fill(&mut self, value: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(800usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "fill", self.sys_ptr, args)
        }
    }
    #[doc = "Sets the size of the array. If the array is grown, reserves elements at the end of the array. If the array is shrunk, truncates the array to the new size. Calling \\[method resize] once and assigning the new values is faster than adding new elements one by one.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the following \\[enum Error] constants if this method fails: [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if the size is negative, or [`Error::ERR_OUT_OF_MEMORY`][`crate::global::Error::ERR_OUT_OF_MEMORY`] if allocations fail. Use \\[method size] to find the actual size of the array after resize."]
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(801usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "resize", self.sys_ptr, args)
        }
    }
    #[doc = "Clears the array. This is equivalent to using \\[method resize] with a size of `0`."]
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(802usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "clear", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the array contains `value`."]
    pub fn has(&self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(803usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "has", self.sys_ptr, args)
        }
    }
    #[doc = "Reverses the order of the elements in the array."]
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(804usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "reverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the slice of the [`PackedInt32Array`][crate::builtin::PackedInt32Array], from `begin` (inclusive) to `end` (exclusive), as a new [`PackedInt32Array`][crate::builtin::PackedInt32Array].\n\nThe absolute value of `begin` and `end` will be clamped to the array size, so the default value for `end` makes it slice to the size of the array by default (i.e. `arr.slice(1)` is a shorthand for `arr.slice(1, arr.size())`).\n\nIf either `begin` or `end` are negative, they will be relative to the end of the array (i.e. `arr.slice(0, -2)` is a shorthand for `arr.slice(0, arr.size() - 2)`)."]
    pub fn slice(&self, begin: i64, end: i64,) -> PackedInt32Array {
        type CallRet = PackedInt32Array;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(805usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "slice", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the data converted to a [`PackedByteArray`][crate::builtin::PackedByteArray], where each element has been encoded as 4 bytes.\n\nThe size of the new array will be `int32_array.size() * 4`."]
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(806usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "to_byte_array", self.sys_ptr, args)
        }
    }
    #[doc = "Sorts the elements of the array in ascending order."]
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(807usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "sort", self.sys_ptr, args)
        }
    }
    #[doc = "Finds the index of an existing value (or the insertion index that maintains sorting order, if the value is not yet present in the array) using binary search. Optionally, a `before` specifier can be passed. If `false`, the returned index comes after all existing entries of the value in the array.\n\n**Note:** Calling \\[method bsearch] on an unsorted array results in unexpected behavior."]
    pub fn bsearch(&self, value: i64, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, bool,);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(808usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "bsearch", self.sys_ptr, args)
        }
    }
    #[doc = "Creates a copy of the array, and returns it."]
    pub fn duplicate(&self,) -> PackedInt32Array {
        type CallRet = PackedInt32Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(809usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "duplicate", self.sys_ptr, args)
        }
    }
    #[doc = "Searches the array for a value and returns its index or `-1` if not found. Optionally, the initial search index can be passed."]
    pub fn find(&self, value: i64, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(810usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "find", self.sys_ptr, args)
        }
    }
    #[doc = "Searches the array in reverse order. Optionally, a start search index can be passed. If negative, the start index is considered relative to the end of the array."]
    pub fn rfind(&self, value: i64, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(811usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "rfind", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of times an element is in the array."]
    pub fn count(&self, value: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(812usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "count", self.sys_ptr, args)
        }
    }
    #[doc = "Removes the first occurrence of a value from the array and returns `true`. If the value does not exist in the array, nothing happens and `false` is returned. To remove an element by index, use \\[method remove_at] instead."]
    pub fn erase(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(813usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "erase", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerPackedInt32Array;
impl PackedInt32Array {
    
}
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
    pub struct InnerPackedVector2Array < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerPackedVector2Array < 'inner > {
    pub fn from_outer(outer: &PackedVector2Array) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the [`Vector2`][crate::builtin::Vector2] at the given `index` in the array. If `index` is out-of-bounds or negative, this method fails and returns `Vector2(0, 0)`.\n\nThis method is similar (but not identical) to the `[]` operator. Most notably, when this method fails, it doesn't pause project execution if run from the editor."]
    pub fn get(&self, index: i64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(906usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "get", self.sys_ptr, args)
        }
    }
    #[doc = "Changes the [`Vector2`][crate::builtin::Vector2] at the given index."]
    pub fn set(&mut self, index: i64, value: Vector2,) {
        type CallRet = ();
        type CallParams = (i64, Vector2,);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(907usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "set", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of elements in the array."]
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(908usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "size", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the array is empty."]
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(909usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "is_empty", self.sys_ptr, args)
        }
    }
    #[doc = "Inserts a [`Vector2`][crate::builtin::Vector2] at the end."]
    pub fn push_back(&mut self, value: Vector2,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(910usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "push_back", self.sys_ptr, args)
        }
    }
    #[doc = "Appends an element at the end of the array (alias of \\[method push_back])."]
    pub fn append(&mut self, value: Vector2,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(911usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "append", self.sys_ptr, args)
        }
    }
    #[doc = "Appends a [`PackedVector2Array`][crate::builtin::PackedVector2Array] at the end of this array."]
    pub fn append_array(&mut self, array: &PackedVector2Array,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(912usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "append_array", self.sys_ptr, args)
        }
    }
    #[doc = "Removes an element from the array by index."]
    pub fn remove_at(&mut self, index: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(913usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "remove_at", self.sys_ptr, args)
        }
    }
    #[doc = "Inserts a new element at a given position in the array. The position must be valid, or at the end of the array (`idx == size()`)."]
    pub fn insert(&mut self, at_index: i64, value: Vector2,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, Vector2,);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(914usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "insert", self.sys_ptr, args)
        }
    }
    #[doc = "Assigns the given value to all elements in the array. This can typically be used together with \\[method resize] to create an array with a given size and initialized elements."]
    pub fn fill(&mut self, value: Vector2,) {
        type CallRet = ();
        type CallParams = (Vector2,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(915usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "fill", self.sys_ptr, args)
        }
    }
    #[doc = "Sets the size of the array. If the array is grown, reserves elements at the end of the array. If the array is shrunk, truncates the array to the new size. Calling \\[method resize] once and assigning the new values is faster than adding new elements one by one.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the following \\[enum Error] constants if this method fails: [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if the size is negative, or [`Error::ERR_OUT_OF_MEMORY`][`crate::global::Error::ERR_OUT_OF_MEMORY`] if allocations fail. Use \\[method size] to find the actual size of the array after resize."]
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(916usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "resize", self.sys_ptr, args)
        }
    }
    #[doc = "Clears the array. This is equivalent to using \\[method resize] with a size of `0`."]
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(917usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "clear", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the array contains `value`.\n\n**Note:** Vectors with `@GDScript.NAN` elements don't behave the same as other vectors. Therefore, the results from this method may not be accurate if NaNs are included."]
    pub fn has(&self, value: Vector2,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(918usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "has", self.sys_ptr, args)
        }
    }
    #[doc = "Reverses the order of the elements in the array."]
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(919usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "reverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the slice of the [`PackedVector2Array`][crate::builtin::PackedVector2Array], from `begin` (inclusive) to `end` (exclusive), as a new [`PackedVector2Array`][crate::builtin::PackedVector2Array].\n\nThe absolute value of `begin` and `end` will be clamped to the array size, so the default value for `end` makes it slice to the size of the array by default (i.e. `arr.slice(1)` is a shorthand for `arr.slice(1, arr.size())`).\n\nIf either `begin` or `end` are negative, they will be relative to the end of the array (i.e. `arr.slice(0, -2)` is a shorthand for `arr.slice(0, arr.size() - 2)`)."]
    pub fn slice(&self, begin: i64, end: i64,) -> PackedVector2Array {
        type CallRet = PackedVector2Array;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(920usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "slice", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a [`PackedByteArray`][crate::builtin::PackedByteArray] with each vector encoded as bytes."]
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(921usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "to_byte_array", self.sys_ptr, args)
        }
    }
    #[doc = "Sorts the elements of the array in ascending order.\n\n**Note:** Vectors with `@GDScript.NAN` elements don't behave the same as other vectors. Therefore, the results from this method may not be accurate if NaNs are included."]
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(922usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "sort", self.sys_ptr, args)
        }
    }
    #[doc = "Finds the index of an existing value (or the insertion index that maintains sorting order, if the value is not yet present in the array) using binary search. Optionally, a `before` specifier can be passed. If `false`, the returned index comes after all existing entries of the value in the array.\n\n**Note:** Calling \\[method bsearch] on an unsorted array results in unexpected behavior.\n\n**Note:** Vectors with `@GDScript.NAN` elements don't behave the same as other vectors. Therefore, the results from this method may not be accurate if NaNs are included."]
    pub fn bsearch(&self, value: Vector2, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector2, bool,);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(923usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "bsearch", self.sys_ptr, args)
        }
    }
    #[doc = "Creates a copy of the array, and returns it."]
    pub fn duplicate(&self,) -> PackedVector2Array {
        type CallRet = PackedVector2Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(924usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "duplicate", self.sys_ptr, args)
        }
    }
    #[doc = "Searches the array for a value and returns its index or `-1` if not found. Optionally, the initial search index can be passed.\n\n**Note:** Vectors with `@GDScript.NAN` elements don't behave the same as other vectors. Therefore, the results from this method may not be accurate if NaNs are included."]
    pub fn find(&self, value: Vector2, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector2, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(925usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "find", self.sys_ptr, args)
        }
    }
    #[doc = "Searches the array in reverse order. Optionally, a start search index can be passed. If negative, the start index is considered relative to the end of the array.\n\n**Note:** Vectors with `@GDScript.NAN` elements don't behave the same as other vectors. Therefore, the results from this method may not be accurate if NaNs are included."]
    pub fn rfind(&self, value: Vector2, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector2, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(926usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "rfind", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of times an element is in the array.\n\n**Note:** Vectors with `@GDScript.NAN` elements don't behave the same as other vectors. Therefore, the results from this method may not be accurate if NaNs are included."]
    pub fn count(&self, value: Vector2,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector2,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(927usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "count", self.sys_ptr, args)
        }
    }
    #[doc = "Removes the first occurrence of a value from the array and returns `true`. If the value does not exist in the array, nothing happens and `false` is returned. To remove an element by index, use \\[method remove_at] instead.\n\n**Note:** Vectors with `@GDScript.NAN` elements don't behave the same as other vectors. Therefore, the results from this method may not be accurate if NaNs are included."]
    pub fn erase(&mut self, value: Vector2,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(928usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "erase", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerPackedVector2Array;
impl PackedVector2Array {
    
}
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
    pub struct InnerArray < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerArray < 'inner > {
    pub fn from_outer(outer: &VarArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn from_outer_typed < T > (outer: &Array < T >) -> Self where T: crate::meta::Element {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the number of elements in the array. Empty arrays (`[]`) always return `0`. See also \\[method is_empty]."]
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(671usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "size", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the array is empty (`[]`). See also \\[method size]."]
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(672usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "is_empty", self.sys_ptr, args)
        }
    }
    #[doc = "Removes all elements from the array. This is equivalent to using \\[method resize] with a size of `0`."]
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(673usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "clear", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a hashed 32-bit integer value representing the array and its contents.\n\n**Note:** Arrays with equal hash values are _not_ guaranteed to be the same, as a result of hash collisions. On the contrary, arrays with different hash values are guaranteed to be different."]
    pub fn hash(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(674usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "hash", self.sys_ptr, args)
        }
    }
    #[doc = "Assigns elements of another `array` into the array. Resizes the array to match `array`. Performs type conversions if the array is typed."]
    pub fn assign(&mut self, array: &AnyArray,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(675usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "assign", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the element at the given `index` in the array. If `index` is out-of-bounds or negative, this method fails and returns `null`.\n\nThis method is similar (but not identical) to the `[]` operator. Most notably, when this method fails, it doesn't pause project execution if run from the editor."]
    pub fn get(&self, index: i64,) -> Variant {
        type CallRet = Variant;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(676usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "get", self.sys_ptr, args)
        }
    }
    #[doc = "Sets the value of the element at the given `index` to the given `value`. This will not change the size of the array, it only changes the value at an index already in the array. This is the same as using the `[]` operator (`array[index] = value`)."]
    pub fn set(&mut self, index: i64, value: &Variant,) {
        type CallRet = ();
        type CallParams < 'a0, > = (i64, RefArg < 'a0, Variant >,);
        let args = (index, RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(677usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "set", self.sys_ptr, args)
        }
    }
    #[doc = "Appends an element at the end of the array. See also \\[method push_front]."]
    pub fn push_back(&mut self, value: &Variant,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(678usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "push_back", self.sys_ptr, args)
        }
    }
    #[doc = "Adds an element at the beginning of the array. See also \\[method push_back].\n\n**Note:** This method shifts every other element's index forward, which may have a noticeable performance cost, especially on larger arrays."]
    pub fn push_front(&mut self, value: &Variant,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(679usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "push_front", self.sys_ptr, args)
        }
    }
    #[doc = "Appends `value` at the end of the array (alias of \\[method push_back])."]
    pub fn append(&mut self, value: &Variant,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(680usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "append", self.sys_ptr, args)
        }
    }
    #[doc = "Appends another `array` at the end of this array.\n\n```gdscript\nvar numbers = [1, 2, 3]\nvar extra = [4, 5, 6]\nnumbers.append_array(extra)\nprint(numbers) # Prints [1, 2, 3, 4, 5, 6]\n```"]
    pub fn append_array(&mut self, array: &AnyArray,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(681usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "append_array", self.sys_ptr, args)
        }
    }
    #[doc = "Sets the array's number of elements to `size`. If `size` is smaller than the array's current size, the elements at the end are removed. If `size` is greater, new default elements (usually `null`) are added, depending on the array's type.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the following \\[enum Error] constants if this method fails: [`Error::ERR_LOCKED`][`crate::global::Error::ERR_LOCKED`] if the array is read-only, [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if the size is negative, or [`Error::ERR_OUT_OF_MEMORY`][`crate::global::Error::ERR_OUT_OF_MEMORY`] if allocations fail. Use \\[method size] to find the actual size of the array after resize.\n\n**Note:** Calling this method once and assigning the new values is faster than calling \\[method append] for every new element."]
    pub fn resize(&mut self, size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(682usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "resize", self.sys_ptr, args)
        }
    }
    #[doc = "Inserts a new element (`value`) at a given index (`position`) in the array. `position` should be between `0` and the array's \\[method size]. If negative, `position` is considered relative to the end of the array.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the other \\[enum Error] constants if this method fails.\n\n**Note:** Every element's index after `position` needs to be shifted forward, which may have a noticeable performance cost, especially on larger arrays."]
    pub fn insert(&mut self, position: i64, value: &Variant,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (i64, RefArg < 'a0, Variant >,);
        let args = (position, RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(683usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "insert", self.sys_ptr, args)
        }
    }
    #[doc = "Removes the element from the array at the given index (`position`). If the index is out of bounds, this method fails. If the index is negative, `position` is considered relative to the end of the array.\n\nIf you need to return the removed element, use \\[method pop_at]. To remove an element by value, use \\[method erase] instead.\n\n**Note:** This method shifts every element's index after `position` back, which may have a noticeable performance cost, especially on larger arrays."]
    pub fn remove_at(&mut self, position: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (position,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(684usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "remove_at", self.sys_ptr, args)
        }
    }
    #[doc = "Assigns the given `value` to all elements in the array.\n\nThis method can often be combined with \\[method resize] to create an array with a given size and initialized elements:\n\n\n```gdscript\nvar array = []\narray.resize(5)\narray.fill(2)\nprint(array) # Prints [2, 2, 2, 2, 2]\n```\n\n\n**Note:** If `value` is a [`Variant`][crate::builtin::Variant] passed by reference ([`Object`][crate::classes::Object]-derived, [`Array`][crate::builtin::Array], [`Dictionary`][crate::builtin::Dictionary], etc.), the array will be filled with references to the same `value`, which are not duplicates."]
    pub fn fill(&mut self, value: &Variant,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(685usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "fill", self.sys_ptr, args)
        }
    }
    #[doc = "Finds and removes the first occurrence of `value` from the array. If `value` does not exist in the array, nothing happens. To remove an element by index, use \\[method remove_at] instead.\n\n**Note:** This method shifts every element's index after the removed `value` back, which may have a noticeable performance cost, especially on larger arrays.\n\n**Note:** Erasing elements while iterating over arrays is **not** supported and will result in unpredictable behavior."]
    pub fn erase(&mut self, value: &Variant,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(686usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "erase", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the first element of the array. If the array is empty, fails and returns `null`. See also \\[method back].\n\n**Note:** Unlike with the `[]` operator (`array[0]`), an error is generated without stopping project execution."]
    pub fn front(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(687usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "front", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the last element of the array. If the array is empty, fails and returns `null`. See also \\[method front].\n\n**Note:** Unlike with the `[]` operator (`array[-1]`), an error is generated without stopping project execution."]
    pub fn back(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(688usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "back", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a random element from the array. Generates an error and returns `null` if the array is empty.\n\n\n```gdscript\n# May print 1, 2, 3.25, or \"Hi\".\nprint([1, 2, 3.25, \"Hi\"].pick_random())\n```\n\n\n**Note:** Like many similar functions in the engine (such as [`randi`][`crate::global::randi`] or \\[method shuffle]), this method uses a common, global random seed. To get a predictable outcome from this method, see [`seed`][`crate::global::seed`]."]
    pub fn pick_random(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(689usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "pick_random", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the index of the **first** occurrence of `what` in this array, or `-1` if there are none. The search's start can be specified with `from`, continuing to the end of the array.\n\n**Note:** If you just want to know whether the array contains `what`, use \\[method has] (`Contains` in C#). In GDScript, you may also use the `in` operator.\n\n**Note:** For performance reasons, the search is affected by `what`'s \\[enum Variant.Type]. For example, `7` (`int`) and `7.0` (`float`) are not considered equal for this method."]
    pub fn find(&self, what: &Variant, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >, i64,);
        let args = (RefArg::new(what), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(690usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "find", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the index of the **first** element in the array that causes `method` to return `true`, or `-1` if there are none. The search's start can be specified with `from`, continuing to the end of the array.\n\n`method` is a callable that takes an element of the array, and returns a `bool`.\n\n**Note:** If you just want to know whether the array contains _anything_ that satisfies `method`, use \\[method any].\n\n\n```gdscript\nfunc is_even(number):\n\treturn number % 2 == 0\n\nfunc _ready():\n\tprint([1, 3, 4, 7].find_custom(is_even.bind())) # Prints 2\n```\n"]
    pub fn find_custom(&self, method: &Callable, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i64,);
        let args = (RefArg::new(method), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(691usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "find_custom", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the index of the **last** occurrence of `what` in this array, or `-1` if there are none. The search's start can be specified with `from`, continuing to the beginning of the array. This method is the reverse of \\[method find]."]
    pub fn rfind(&self, what: &Variant, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >, i64,);
        let args = (RefArg::new(what), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(692usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "rfind", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the index of the **last** element of the array that causes `method` to return `true`, or `-1` if there are none. The search's start can be specified with `from`, continuing to the beginning of the array. This method is the reverse of \\[method find_custom]."]
    pub fn rfind_custom(&self, method: &Callable, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i64,);
        let args = (RefArg::new(method), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(693usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "rfind_custom", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of times an element is in the array.\n\nTo count how many elements in an array satisfy a condition, see \\[method reduce]."]
    pub fn count(&self, value: &Variant,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(694usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "count", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the array contains the given `value`.\n\n\n```gdscript\nprint([\"inside\", 7].has(\"inside\"))  # Prints true\nprint([\"inside\", 7].has(\"outside\")) # Prints false\nprint([\"inside\", 7].has(7))         # Prints true\nprint([\"inside\", 7].has(\"7\"))       # Prints false\n```\n\n\nIn GDScript, this is equivalent to the `in` operator:\n\n```gdscript\nif 4 in [2, 4, 6, 8]:\n\tprint(\"4 is here!\") # Will be printed.\n```\n\n**Note:** For performance reasons, the search is affected by the `value`'s \\[enum Variant.Type]. For example, `7` (`int`) and `7.0` (`float`) are not considered equal for this method."]
    pub fn has(&self, value: &Variant,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(695usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "has", self.sys_ptr, args)
        }
    }
    #[doc = "Removes and returns the last element of the array. Returns `null` if the array is empty, without generating an error. See also \\[method pop_front]."]
    pub fn pop_back(&mut self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(696usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "pop_back", self.sys_ptr, args)
        }
    }
    #[doc = "Removes and returns the first element of the array. Returns `null` if the array is empty, without generating an error. See also \\[method pop_back].\n\n**Note:** This method shifts every other element's index back, which may have a noticeable performance cost, especially on larger arrays."]
    pub fn pop_front(&mut self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(697usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "pop_front", self.sys_ptr, args)
        }
    }
    #[doc = "Removes and returns the element of the array at index `position`. If negative, `position` is considered relative to the end of the array. Returns `null` if the array is empty. If `position` is out of bounds, an error message is also generated.\n\n**Note:** This method shifts every element's index after `position` back, which may have a noticeable performance cost, especially on larger arrays."]
    pub fn pop_at(&mut self, position: i64,) -> Variant {
        type CallRet = Variant;
        type CallParams = (i64,);
        let args = (position,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(698usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "pop_at", self.sys_ptr, args)
        }
    }
    #[doc = "Sorts the array in ascending order. The final order is dependent on the \"less than\" (`<`) comparison between elements.\n\n\n```gdscript\nvar numbers = [10, 5, 2.5, 8]\nnumbers.sort()\nprint(numbers) # Prints [2.5, 5, 8, 10]\n```\n\n\n**Note:** The sorting algorithm used is not [stable](https://en.wikipedia.org/wiki/Sorting_algorithm#Stability). This means that equivalent elements (such as `2` and `2.0`) may have their order changed when calling \\[method sort]."]
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(699usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "sort", self.sys_ptr, args)
        }
    }
    #[doc = "Sorts the array using a custom [`Callable`][crate::builtin::Callable].\n\n`func` is called as many times as necessary, receiving two array elements as arguments. The function should return `true` if the first element should be moved _before_ the second one, otherwise it should return `false`.\n\n```gdscript\nfunc sort_ascending(a, b):\n\tif a[1] < b[1]:\n\t\treturn true\n\treturn false\n\nfunc _ready():\n\tvar my_items = [[\"Tomato\", 5], [\"Apple\", 9], [\"Rice\", 4]]\n\tmy_items.sort_custom(sort_ascending)\n\tprint(my_items) # Prints [[\"Rice\", 4], [\"Tomato\", 5], [\"Apple\", 9]]\n\n\t# Sort descending, using a lambda function.\n\tmy_items.sort_custom(func(a, b): return a[1] > b[1])\n\tprint(my_items) # Prints [[\"Apple\", 9], [\"Tomato\", 5], [\"Rice\", 4]]\n```\n\nIt may also be necessary to use this method to sort strings by natural order, with [`naturalnocasecmp_to`][`crate::builtin::GString::naturalnocasecmp_to`], as in the following example:\n\n```gdscript\nvar files = [\"newfile1\", \"newfile2\", \"newfile10\", \"newfile11\"]\nfiles.sort_custom(func(a, b): return a.naturalnocasecmp_to(b) < 0)\nprint(files) # Prints [\"newfile1\", \"newfile2\", \"newfile10\", \"newfile11\"]\n```\n\n**Note:** In C#, this method is not supported.\n\n**Note:** The sorting algorithm used is not [stable](https://en.wikipedia.org/wiki/Sorting_algorithm#Stability). This means that values considered equal may have their order changed when calling this method.\n\n**Note:** You should not randomize the return value of `func`, as the heapsort algorithm expects a consistent result. Randomizing the return value will result in unexpected behavior."]
    pub fn sort_custom(&mut self, func: &Callable,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(func),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(700usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "sort_custom", self.sys_ptr, args)
        }
    }
    #[doc = "Shuffles all elements of the array in a random order.\n\n**Note:** Like many similar functions in the engine (such as [`randi`][`crate::global::randi`] or \\[method pick_random]), this method uses a common, global random seed. To get a predictable outcome from this method, see [`seed`][`crate::global::seed`]."]
    pub fn shuffle(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(701usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "shuffle", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the index of `value` in the sorted array. If it cannot be found, returns where `value` should be inserted to keep the array sorted. The algorithm used is [binary search](https://en.wikipedia.org/wiki/Binary_search_algorithm).\n\nIf `before` is `true` (as by default), the returned index comes before all existing elements equal to `value` in the array.\n\n```gdscript\nvar numbers = [2, 4, 8, 10]\nvar idx = numbers.bsearch(7)\n\nnumbers.insert(idx, 7)\nprint(numbers) # Prints [2, 4, 7, 8, 10]\n\nvar fruits = [\"Apple\", \"Lemon\", \"Lemon\", \"Orange\"]\nprint(fruits.bsearch(\"Lemon\", true))  # Prints 1, points at the first \"Lemon\".\nprint(fruits.bsearch(\"Lemon\", false)) # Prints 3, points at \"Orange\".\n```\n\n**Note:** Calling \\[method bsearch] on an _unsorted_ array will result in unexpected behavior. Use \\[method sort] before calling this method."]
    pub fn bsearch(&self, value: &Variant, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >, bool,);
        let args = (RefArg::new(value), before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(702usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "bsearch", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the index of `value` in the sorted array. If it cannot be found, returns where `value` should be inserted to keep the array sorted (using `func` for the comparisons). The algorithm used is [binary search](https://en.wikipedia.org/wiki/Binary_search_algorithm).\n\nSimilar to \\[method sort_custom], `func` is called as many times as necessary, receiving one array element and `value` as arguments. The function should return `true` if the array element should be _behind_ `value`, otherwise it should return `false`.\n\nIf `before` is `true` (as by default), the returned index comes before all existing elements equal to `value` in the array.\n\n```gdscript\nfunc sort_by_amount(a, b):\n\tif a[1] < b[1]:\n\t\treturn true\n\treturn false\n\nfunc _ready():\n\tvar my_items = [[\"Tomato\", 2], [\"Kiwi\", 5], [\"Rice\", 9]]\n\n\tvar apple = [\"Apple\", 5]\n\t# \"Apple\" is inserted before \"Kiwi\".\n\tmy_items.insert(my_items.bsearch_custom(apple, sort_by_amount, true), apple)\n\n\tvar banana = [\"Banana\", 5]\n\t# \"Banana\" is inserted after \"Kiwi\".\n\tmy_items.insert(my_items.bsearch_custom(banana, sort_by_amount, false), banana)\n\n\t# Prints [[\"Tomato\", 2], [\"Apple\", 5], [\"Kiwi\", 5], [\"Banana\", 5], [\"Rice\", 9]]\n\tprint(my_items)\n```\n\n**Note:** Calling \\[method bsearch_custom] on an _unsorted_ array will result in unexpected behavior. Use \\[method sort_custom] with `func` before calling this method."]
    pub fn bsearch_custom(&self, value: &Variant, func: &Callable, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Callable >, bool,);
        let args = (RefArg::new(value), RefArg::new(func), before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(703usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "bsearch_custom", self.sys_ptr, args)
        }
    }
    #[doc = "Reverses the order of all elements in the array."]
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(704usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "reverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new copy of the array.\n\nBy default, a **shallow** copy is returned: all nested [`Array`][crate::builtin::Array], [`Dictionary`][crate::builtin::Dictionary], and [`Resource`][crate::classes::Resource] elements are shared with the original array. Modifying any of those in one array will also affect them in the other.\n\nIf `deep` is `true`, a **deep** copy is returned: all nested arrays and dictionaries are also duplicated (recursively). Any [`Resource`][crate::classes::Resource] is still shared with the original array, though."]
    pub fn duplicate(&self, deep: bool,) -> AnyArray {
        type CallRet = AnyArray;
        type CallParams = (bool,);
        let args = (deep,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(705usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "duplicate", self.sys_ptr, args)
        }
    }
    #[doc = "Duplicates this array, deeply, like \\[method duplicate] when passing `true`, with extra control over how subresources are handled.\n\n`deep_subresources_mode` must be one of the values from \\[enum Resource.DeepDuplicateMode]. By default, only internal resources will be duplicated (recursively)."]
    pub fn duplicate_deep(&self, deep_subresources_mode: i64,) -> AnyArray {
        type CallRet = AnyArray;
        type CallParams = (i64,);
        let args = (deep_subresources_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(706usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "duplicate_deep", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new [`Array`][crate::builtin::Array] containing this array's elements, from index `begin` (inclusive) to `end` (exclusive), every `step` elements.\n\nIf either `begin` or `end` are negative, their value is relative to the end of the array.\n\nIf `step` is negative, this method iterates through the array in reverse, returning a slice ordered backwards. For this to work, `begin` must be greater than `end`.\n\nIf `deep` is `true`, all nested [`Array`][crate::builtin::Array] and [`Dictionary`][crate::builtin::Dictionary] elements in the slice are duplicated from the original, recursively. See also \\[method duplicate].\n\n```gdscript\nvar letters = [\"A\", \"B\", \"C\", \"D\", \"E\", \"F\"]\n\nprint(letters.slice(0, 2))  # Prints [\"A\", \"B\"]\nprint(letters.slice(2, -2)) # Prints [\"C\", \"D\"]\nprint(letters.slice(-2, 6)) # Prints [\"E\", \"F\"]\n\nprint(letters.slice(0, 6, 2))  # Prints [\"A\", \"C\", \"E\"]\nprint(letters.slice(4, 1, -1)) # Prints [\"E\", \"D\", \"C\"]\n```"]
    pub fn slice(&self, begin: i64, end: i64, step: i64, deep: bool,) -> AnyArray {
        type CallRet = AnyArray;
        type CallParams = (i64, i64, i64, bool,);
        let args = (begin, end, step, deep,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(707usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "slice", self.sys_ptr, args)
        }
    }
    #[doc = "Calls the given [`Callable`][crate::builtin::Callable] on each element in the array and returns a new, filtered [`Array`][crate::builtin::Array].\n\nThe `method` receives one of the array elements as an argument, and should return `true` to add the element to the filtered array, or `false` to exclude it.\n\n```gdscript\nfunc is_even(number):\n\treturn number % 2 == 0\n\nfunc _ready():\n\tprint([1, 4, 5, 8].filter(is_even)) # Prints [4, 8]\n\n\t# Same as above, but using a lambda function.\n\tprint([1, 4, 5, 8].filter(func(number): return number % 2 == 0))\n```\n\nSee also \\[method any], \\[method all], \\[method map] and \\[method reduce]."]
    pub fn filter(&self, method: &Callable,) -> AnyArray {
        type CallRet = AnyArray;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(method),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(708usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "filter", self.sys_ptr, args)
        }
    }
    #[doc = "Calls the given [`Callable`][crate::builtin::Callable] for each element in the array and returns a new array filled with values returned by the `method`.\n\nThe `method` should take one [`Variant`][crate::builtin::Variant] parameter (the current array element) and can return any [`Variant`][crate::builtin::Variant].\n\n```gdscript\nfunc double(number):\n\treturn number * 2\n\nfunc _ready():\n\tprint([1, 2, 3].map(double)) # Prints [2, 4, 6]\n\n\t# Same as above, but using a lambda function.\n\tprint([1, 2, 3].map(func(element): return element * 2))\n```\n\nSee also \\[method filter], \\[method reduce], \\[method any] and \\[method all]."]
    pub fn map(&self, method: &Callable,) -> AnyArray {
        type CallRet = AnyArray;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(method),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(709usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "map", self.sys_ptr, args)
        }
    }
    #[doc = "Calls the given [`Callable`][crate::builtin::Callable] for each element in array, accumulates the result in `accum`, then returns it.\n\nThe `method` takes two arguments: the current value of `accum` and the current array element. If `accum` is `null` (as by default), the iteration will start from the second element, with the first one used as initial value of `accum`.\n\n```gdscript\nfunc sum(accum, number):\n\treturn accum + number\n\nfunc _ready():\n\tprint([1, 2, 3].reduce(sum, 0))  # Prints 6\n\tprint([1, 2, 3].reduce(sum, 10)) # Prints 16\n\n\t# Same as above, but using a lambda function.\n\tprint([1, 2, 3].reduce(func(accum, number): return accum + number, 10))\n```\n\nIf \\[method max] is not desirable, this method may also be used to implement a custom comparator:\n\n```gdscript\nfunc _ready():\n\tvar arr = [Vector2i(5, 0), Vector2i(3, 4), Vector2i(1, 2)]\n\n\tvar longest_vec = arr.reduce(func(max, vec): return vec if is_length_greater(vec, max) else max)\n\tprint(longest_vec) # Prints (3, 4)\n\nfunc is_length_greater(a, b):\n\treturn a.length() > b.length()\n```\n\nThis method can also be used to count how many elements in an array satisfy a certain condition, similar to \\[method count]:\n\n```gdscript\nfunc is_even(number):\n\treturn number % 2 == 0\n\nfunc _ready():\n\tvar arr = [1, 2, 3, 4, 5]\n\t# If the current element is even, increment count, otherwise leave count the same.\n\tvar even_count = arr.reduce(func(count, next): return count + 1 if is_even(next) else count, 0)\n\tprint(even_count) # Prints 2\n```\n\nSee also \\[method map], \\[method filter], \\[method any], and \\[method all]."]
    pub fn reduce(&self, method: &Callable, accum: &Variant,) -> Variant {
        type CallRet = Variant;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Callable >, RefArg < 'a1, Variant >,);
        let args = (RefArg::new(method), RefArg::new(accum),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(710usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "reduce", self.sys_ptr, args)
        }
    }
    #[doc = "Calls the given [`Callable`][crate::builtin::Callable] on each element in the array and returns `true` if the [`Callable`][crate::builtin::Callable] returns `true` for _one or more_ elements in the array. If the [`Callable`][crate::builtin::Callable] returns `false` for all elements in the array, this method returns `false`.\n\nThe `method` should take one [`Variant`][crate::builtin::Variant] parameter (the current array element) and return a `bool`.\n\n```gdscript\nfunc greater_than_5(number):\n\treturn number > 5\n\nfunc _ready():\n\tprint([6, 10, 6].any(greater_than_5)) # Prints true (3 elements evaluate to true).\n\tprint([4, 10, 4].any(greater_than_5)) # Prints true (1 elements evaluate to true).\n\tprint([4, 4, 4].any(greater_than_5))  # Prints false (0 elements evaluate to true).\n\tprint([].any(greater_than_5))         # Prints false (0 elements evaluate to true).\n\n\t# Same as the first line above, but using a lambda function.\n\tprint([6, 10, 6].any(func(number): return number > 5)) # Prints true\n```\n\nSee also \\[method all], \\[method filter], \\[method map] and \\[method reduce].\n\n**Note:** Unlike relying on the size of an array returned by \\[method filter], this method will return as early as possible to improve performance (especially with large arrays).\n\n**Note:** For an empty array, this method always returns `false`."]
    pub fn any(&self, method: &Callable,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(method),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(711usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "any", self.sys_ptr, args)
        }
    }
    #[doc = "Calls the given [`Callable`][crate::builtin::Callable] on each element in the array and returns `true` if the [`Callable`][crate::builtin::Callable] returns `true` for _all_ elements in the array. If the [`Callable`][crate::builtin::Callable] returns `false` for one array element or more, this method returns `false`.\n\nThe `method` should take one [`Variant`][crate::builtin::Variant] parameter (the current array element) and return a `bool`.\n\n\n```gdscript\nfunc greater_than_5(number):\n\treturn number > 5\n\nfunc _ready():\n\tprint([6, 10, 6].all(greater_than_5)) # Prints true (3/3 elements evaluate to true).\n\tprint([4, 10, 4].all(greater_than_5)) # Prints false (1/3 elements evaluate to true).\n\tprint([4, 4, 4].all(greater_than_5))  # Prints false (0/3 elements evaluate to true).\n\tprint([].all(greater_than_5))         # Prints true (0/0 elements evaluate to true).\n\n\t# Same as the first line above, but using a lambda function.\n\tprint([6, 10, 6].all(func(element): return element > 5)) # Prints true\n```\n\n\nSee also \\[method any], \\[method filter], \\[method map] and \\[method reduce].\n\n**Note:** Unlike relying on the size of an array returned by \\[method filter], this method will return as early as possible to improve performance (especially with large arrays).\n\n**Note:** For an empty array, this method [always](https://en.wikipedia.org/wiki/Vacuous_truth) returns `true`."]
    pub fn all(&self, method: &Callable,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(method),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(712usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "all", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the maximum value contained in the array, if all elements can be compared. Otherwise, returns `null`. See also \\[method min].\n\nTo find the maximum value using a custom comparator, you can use \\[method reduce]."]
    pub fn max(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(713usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "max", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the minimum value contained in the array, if all elements can be compared. Otherwise, returns `null`. See also \\[method max]."]
    pub fn min(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(714usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "min", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the array is typed. Typed arrays can only contain elements of a specific type, as defined by the typed array constructor. The methods of a typed array are still expected to return a generic [`Variant`][crate::builtin::Variant].\n\nIn GDScript, it is possible to define a typed array with static typing:\n\n```gdscript\nvar numbers: Array[float] = [0.2, 4.2, -2.0]\nprint(numbers.is_typed()) # Prints true\n```"]
    pub fn is_typed(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(715usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "is_typed", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this array is typed the same as the given `array`. See also \\[method is_typed]."]
    pub fn is_same_typed(&self, array: &AnyArray,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(716usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "is_same_typed", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the built-in [`Variant`][crate::builtin::Variant] type of the typed array as a \\[enum Variant.Type] constant. If the array is not typed, returns [`VariantType::NIL`][`crate::builtin::VariantType::NIL`]. See also \\[method is_typed]."]
    pub fn get_typed_builtin(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(717usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "get_typed_builtin", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the **built-in** class name of the typed array, if the built-in [`Variant`][crate::builtin::Variant] type [`VariantType::OBJECT`][`crate::builtin::VariantType::OBJECT`]. Otherwise, returns an empty [`StringName`][crate::builtin::StringName]. See also \\[method is_typed] and [`get_class`][`crate::classes::Object::get_class`]."]
    pub fn get_typed_class_name(&self,) -> StringName {
        type CallRet = StringName;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(718usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "get_typed_class_name", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the [`Script`][crate::classes::Script] instance associated with this typed array, or `null` if it does not exist. See also \\[method is_typed]."]
    pub fn get_typed_script(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(719usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "get_typed_script", self.sys_ptr, args)
        }
    }
    #[doc = "Makes the array read-only. The array's elements cannot be overridden with different values, and their order cannot change. Does not apply to nested elements, such as dictionaries.\n\nIn GDScript, arrays are automatically read-only if declared with the `const` keyword."]
    pub fn make_read_only(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(720usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "make_read_only", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the array is read-only. See \\[method make_read_only].\n\nIn GDScript, arrays are automatically read-only if declared with the `const` keyword."]
    pub fn is_read_only(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(721usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "is_read_only", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerArray;
impl VarArray {
    
}
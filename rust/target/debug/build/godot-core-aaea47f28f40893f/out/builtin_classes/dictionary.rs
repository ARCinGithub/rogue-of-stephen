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
    pub struct InnerDictionary < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerDictionary < 'inner > {
    pub fn from_outer(outer: &VarDictionary) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn from_outer_typed < K, V > (outer: &Dictionary < K, V >) -> Self where K: crate::meta::Element, V: crate::meta::Element, {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the number of entries in the dictionary. Empty dictionaries (`{ }`) always return `0`. See also \\[method is_empty]."]
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(637usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "size", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the dictionary is empty (its size is `0`). See also \\[method size]."]
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(638usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_empty", self.sys_ptr, args)
        }
    }
    #[doc = "Clears the dictionary, removing all entries from it."]
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(639usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "clear", self.sys_ptr, args)
        }
    }
    #[doc = "Assigns elements of another `dictionary` into the dictionary. Resizes the dictionary to match `dictionary`. Performs type conversions if the dictionary is typed."]
    pub fn assign(&mut self, dictionary: &AnyDictionary,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
        let args = (RefArg::new(dictionary),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(640usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "assign", self.sys_ptr, args)
        }
    }
    #[doc = "Sorts the dictionary in ascending order, by key. The final order is dependent on the \"less than\" (`<`) comparison between keys.\n\n\n```gdscript\nvar numbers = { \"c\": 2, \"a\": 0, \"b\": 1 }\nnumbers.sort()\nprint(numbers) # Prints { \"a\": 0, \"b\": 1, \"c\": 2 }\n```\n\n\nThis method ensures that the dictionary's entries are ordered consistently when \\[method keys] or \\[method values] are called, or when the dictionary needs to be converted to a string through [`str`][`crate::global::str`] or [`stringify`][`crate::classes::Json::stringify`]."]
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(641usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "sort", self.sys_ptr, args)
        }
    }
    #[doc = "Adds entries from `dictionary` to this dictionary. By default, duplicate keys are not copied over, unless `overwrite` is `true`.\n\n\n```gdscript\nvar dict = { \"item\": \"sword\", \"quantity\": 2 }\nvar other_dict = { \"quantity\": 15, \"color\": \"silver\" }\n\n# Overwriting of existing keys is disabled by default.\ndict.merge(other_dict)\nprint(dict)  # { \"item\": \"sword\", \"quantity\": 2, \"color\": \"silver\" }\n\n# With overwriting of existing keys enabled.\ndict.merge(other_dict, true)\nprint(dict)  # { \"item\": \"sword\", \"quantity\": 15, \"color\": \"silver\" }\n```\n\n\n**Note:** \\[method merge] is _not_ recursive. Nested dictionaries are considered as keys that can be overwritten or not depending on the value of `overwrite`, but they will never be merged together."]
    pub fn merge(&mut self, dictionary: &AnyDictionary, overwrite: bool,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >, bool,);
        let args = (RefArg::new(dictionary), overwrite,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(642usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "merge", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this dictionary merged with the other `dictionary`. By default, duplicate keys are not copied over, unless `overwrite` is `true`. See also \\[method merge].\n\nThis method is useful for quickly making dictionaries with default values:\n\n```gdscript\nvar base = { \"fruit\": \"apple\", \"vegetable\": \"potato\" }\nvar extra = { \"fruit\": \"orange\", \"dressing\": \"vinegar\" }\n# Prints { \"fruit\": \"orange\", \"vegetable\": \"potato\", \"dressing\": \"vinegar\" }\nprint(extra.merged(base))\n# Prints { \"fruit\": \"apple\", \"vegetable\": \"potato\", \"dressing\": \"vinegar\" }\nprint(extra.merged(base, true))\n```"]
    pub fn merged(&self, dictionary: &AnyDictionary, overwrite: bool,) -> AnyDictionary {
        type CallRet = AnyDictionary;
        type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >, bool,);
        let args = (RefArg::new(dictionary), overwrite,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(643usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "merged", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the dictionary contains an entry with the given `key`.\n\n\n```gdscript\nvar my_dict = {\n\t\"Godot\" : 4,\n\t210 : null,\n}\n\nprint(my_dict.has(\"Godot\")) # Prints true\nprint(my_dict.has(210))     # Prints true\nprint(my_dict.has(4))       # Prints false\n```\n\n\nIn GDScript, this is equivalent to the `in` operator:\n\n```gdscript\nif \"Godot\" in { \"Godot\": 4 }:\n\tprint(\"The key is here!\") # Will be printed.\n```\n\n**Note:** This method returns `true` as long as the `key` exists, even if its corresponding value is `null`."]
    pub fn has(&self, key: &Variant,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(key),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(644usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "has", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the dictionary contains all keys in the given `keys` array.\n\n```gdscript\nvar data = { \"width\": 10, \"height\": 20 }\ndata.has_all([\"height\", \"width\"]) # Returns true\n```"]
    pub fn has_all(&self, keys: &AnyArray,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
        let args = (RefArg::new(keys),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(645usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "has_all", self.sys_ptr, args)
        }
    }
    #[doc = "Finds and returns the first key whose associated value is equal to `value`, or `null` if it is not found.\n\n**Note:** `null` is also a valid key. If inside the dictionary, \\[method find_key] may give misleading results."]
    pub fn find_key(&self, value: &Variant,) -> Variant {
        type CallRet = Variant;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(646usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "find_key", self.sys_ptr, args)
        }
    }
    #[doc = "Removes the dictionary entry by key, if it exists. Returns `true` if the given `key` existed in the dictionary, otherwise `false`.\n\n**Note:** Do not erase entries while iterating over the dictionary. You can iterate over the \\[method keys] array instead."]
    pub fn erase(&mut self, key: &Variant,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(key),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(647usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "erase", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a hashed 32-bit integer value representing the dictionary contents.\n\n\n```gdscript\nvar dict1 = { \"A\": 10, \"B\": 2 }\nvar dict2 = { \"A\": 10, \"B\": 2 }\n\nprint(dict1.hash() == dict2.hash()) # Prints true\n```\n\n\n**Note:** Dictionaries with the same entries but in a different order will not have the same hash.\n\n**Note:** Dictionaries with equal hash values are _not_ guaranteed to be the same, because of hash collisions. On the contrary, dictionaries with different hash values are guaranteed to be different."]
    pub fn hash(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(648usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "hash", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the list of keys in the dictionary."]
    pub fn keys(&self,) -> AnyArray {
        type CallRet = AnyArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(649usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "keys", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the list of values in this dictionary."]
    pub fn values(&self,) -> AnyArray {
        type CallRet = AnyArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(650usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "values", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new copy of the dictionary.\n\nBy default, a **shallow** copy is returned: all nested [`Array`][crate::builtin::Array], [`Dictionary`][crate::builtin::Dictionary], and [`Resource`][crate::classes::Resource] keys and values are shared with the original dictionary. Modifying any of those in one dictionary will also affect them in the other.\n\nIf `deep` is `true`, a **deep** copy is returned: all nested arrays and dictionaries are also duplicated (recursively). Any [`Resource`][crate::classes::Resource] is still shared with the original dictionary, though."]
    pub fn duplicate(&self, deep: bool,) -> AnyDictionary {
        type CallRet = AnyDictionary;
        type CallParams = (bool,);
        let args = (deep,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(651usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "duplicate", self.sys_ptr, args)
        }
    }
    #[doc = "Duplicates this dictionary, deeply, like \\[method duplicate] when passing `true`, with extra control over how subresources are handled.\n\n`deep_subresources_mode` must be one of the values from \\[enum Resource.DeepDuplicateMode]. By default, only internal resources will be duplicated (recursively)."]
    pub fn duplicate_deep(&self, deep_subresources_mode: i64,) -> AnyDictionary {
        type CallRet = AnyDictionary;
        type CallParams = (i64,);
        let args = (deep_subresources_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(652usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "duplicate_deep", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the corresponding value for the given `key` in the dictionary. If the `key` does not exist, returns `default`, or `null` if the parameter is omitted."]
    pub fn get(&self, key: &Variant, default: &Variant,) -> Variant {
        type CallRet = Variant;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
        let args = (RefArg::new(key), RefArg::new(default),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(653usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get", self.sys_ptr, args)
        }
    }
    #[doc = "Gets a value and ensures the key is set. If the `key` exists in the dictionary, this behaves like \\[method get]. Otherwise, the `default` value is inserted into the dictionary and returned."]
    pub fn get_or_add(&mut self, key: &Variant, default: &Variant,) -> Variant {
        type CallRet = Variant;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
        let args = (RefArg::new(key), RefArg::new(default),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(654usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_or_add", self.sys_ptr, args)
        }
    }
    #[doc = "Sets the value of the element at the given `key` to the given `value`. Returns `true` if the value is set successfully. Fails and returns `false` if the dictionary is read-only, or if `key` and `value` don't match the dictionary's types. This is the same as using the `[]` operator (`dict[key] = value`)."]
    pub fn set(&mut self, key: &Variant, value: &Variant,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
        let args = (RefArg::new(key), RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(655usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "set", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the dictionary is typed. Typed dictionaries can only store keys/values of their associated type and provide type safety for the `[]` operator. Methods of typed dictionary still return [`Variant`][crate::builtin::Variant]."]
    pub fn is_typed(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(656usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_typed", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the dictionary's keys are typed."]
    pub fn is_typed_key(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(657usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_typed_key", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the dictionary's values are typed."]
    pub fn is_typed_value(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(658usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_typed_value", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the dictionary is typed the same as `dictionary`."]
    pub fn is_same_typed(&self, dictionary: &AnyDictionary,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
        let args = (RefArg::new(dictionary),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(659usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_same_typed", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the dictionary's keys are typed the same as `dictionary`'s keys."]
    pub fn is_same_typed_key(&self, dictionary: &AnyDictionary,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
        let args = (RefArg::new(dictionary),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(660usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_same_typed_key", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the dictionary's values are typed the same as `dictionary`'s values."]
    pub fn is_same_typed_value(&self, dictionary: &AnyDictionary,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
        let args = (RefArg::new(dictionary),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(661usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_same_typed_value", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the built-in [`Variant`][crate::builtin::Variant] type of the typed dictionary's keys as a \\[enum Variant.Type] constant. If the keys are not typed, returns [`VariantType::NIL`][`crate::builtin::VariantType::NIL`]. See also \\[method is_typed_key]."]
    pub fn get_typed_key_builtin(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(662usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_typed_key_builtin", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the built-in [`Variant`][crate::builtin::Variant] type of the typed dictionary's values as a \\[enum Variant.Type] constant. If the values are not typed, returns [`VariantType::NIL`][`crate::builtin::VariantType::NIL`]. See also \\[method is_typed_value]."]
    pub fn get_typed_value_builtin(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(663usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_typed_value_builtin", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the **built-in** class name of the typed dictionary's keys, if the built-in [`Variant`][crate::builtin::Variant] type is [`VariantType::OBJECT`][`crate::builtin::VariantType::OBJECT`]. Otherwise, returns an empty [`StringName`][crate::builtin::StringName]. See also \\[method is_typed_key] and [`get_class`][`crate::classes::Object::get_class`]."]
    pub fn get_typed_key_class_name(&self,) -> StringName {
        type CallRet = StringName;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(664usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_typed_key_class_name", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the **built-in** class name of the typed dictionary's values, if the built-in [`Variant`][crate::builtin::Variant] type is [`VariantType::OBJECT`][`crate::builtin::VariantType::OBJECT`]. Otherwise, returns an empty [`StringName`][crate::builtin::StringName]. See also \\[method is_typed_value] and [`get_class`][`crate::classes::Object::get_class`]."]
    pub fn get_typed_value_class_name(&self,) -> StringName {
        type CallRet = StringName;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(665usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_typed_value_class_name", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the [`Script`][crate::classes::Script] instance associated with this typed dictionary's keys, or `null` if it does not exist. See also \\[method is_typed_key]."]
    pub fn get_typed_key_script(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(666usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_typed_key_script", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the [`Script`][crate::classes::Script] instance associated with this typed dictionary's values, or `null` if it does not exist. See also \\[method is_typed_value]."]
    pub fn get_typed_value_script(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(667usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_typed_value_script", self.sys_ptr, args)
        }
    }
    #[doc = "Makes the dictionary read-only, i.e. disables modification of the dictionary's contents. Does not apply to nested content, e.g. content of nested dictionaries."]
    pub fn make_read_only(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(668usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "make_read_only", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the dictionary is read-only. See \\[method make_read_only]. Dictionaries are automatically read-only if declared with `const` keyword."]
    pub fn is_read_only(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(669usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_read_only", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the two dictionaries contain the same keys and values, inner [`Dictionary`][crate::builtin::Dictionary] and [`Array`][crate::builtin::Array] keys and values are compared recursively."]
    pub fn recursive_equal(&self, dictionary: &AnyDictionary, recursion_count: i64,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >, i64,);
        let args = (RefArg::new(dictionary), recursion_count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(670usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "recursive_equal", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerDictionary;
impl VarDictionary {
    
}
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
    pub struct InnerStringName < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerStringName < 'inner > {
    pub fn from_outer(outer: &StringName) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Performs a case-sensitive comparison to another string. Returns `-1` if less than, `1` if greater than, or `0` if equal. \"Less than\" and \"greater than\" are determined by the [Unicode code points](https://en.wikipedia.org/wiki/List_of_Unicode_characters) of each string, which roughly matches the alphabetical order.\n\nWith different string lengths, returns `1` if this string is longer than the `to` string, or `-1` if shorter. Note that the length of empty strings is _always_ `0`.\n\nTo get a `bool` result from a string comparison, use the `==` operator instead. See also \\[method nocasecmp_to], \\[method filecasecmp_to], and \\[method naturalcasecmp_to]."]
    pub fn casecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(483usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "casecmp_to", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a **case-insensitive** comparison to another string. Returns `-1` if less than, `1` if greater than, or `0` if equal. \"Less than\" or \"greater than\" are determined by the [Unicode code points](https://en.wikipedia.org/wiki/List_of_Unicode_characters) of each string, which roughly matches the alphabetical order. Internally, lowercase characters are converted to uppercase for the comparison.\n\nWith different string lengths, returns `1` if this string is longer than the `to` string, or `-1` if shorter. Note that the length of empty strings is _always_ `0`.\n\nTo get a `bool` result from a string comparison, use the `==` operator instead. See also \\[method casecmp_to], \\[method filenocasecmp_to], and \\[method naturalnocasecmp_to]."]
    pub fn nocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(484usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "nocasecmp_to", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a **case-sensitive**, _natural order_ comparison to another string. Returns `-1` if less than, `1` if greater than, or `0` if equal. \"Less than\" or \"greater than\" are determined by the [Unicode code points](https://en.wikipedia.org/wiki/List_of_Unicode_characters) of each string, which roughly matches the alphabetical order.\n\nWhen used for sorting, natural order comparison orders sequences of numbers by the combined value of each digit as is often expected, instead of the single digit's value. A sorted sequence of numbered strings will be `[\"1\", \"2\", \"3\", ...]`, not `[\"1\", \"10\", \"2\", \"3\", ...]`.\n\nWith different string lengths, returns `1` if this string is longer than the `to` string, or `-1` if shorter. Note that the length of empty strings is _always_ `0`.\n\nTo get a `bool` result from a string comparison, use the `==` operator instead. See also \\[method naturalnocasecmp_to], \\[method filecasecmp_to], and \\[method nocasecmp_to]."]
    pub fn naturalcasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(485usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "naturalcasecmp_to", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a **case-insensitive**, _natural order_ comparison to another string. Returns `-1` if less than, `1` if greater than, or `0` if equal. \"Less than\" or \"greater than\" are determined by the [Unicode code points](https://en.wikipedia.org/wiki/List_of_Unicode_characters) of each string, which roughly matches the alphabetical order. Internally, lowercase characters are converted to uppercase for the comparison.\n\nWhen used for sorting, natural order comparison orders sequences of numbers by the combined value of each digit as is often expected, instead of the single digit's value. A sorted sequence of numbered strings will be `[\"1\", \"2\", \"3\", ...]`, not `[\"1\", \"10\", \"2\", \"3\", ...]`.\n\nWith different string lengths, returns `1` if this string is longer than the `to` string, or `-1` if shorter. Note that the length of empty strings is _always_ `0`.\n\nTo get a `bool` result from a string comparison, use the `==` operator instead. See also \\[method naturalcasecmp_to], \\[method filenocasecmp_to], and \\[method casecmp_to]."]
    pub fn naturalnocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(486usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "naturalnocasecmp_to", self.sys_ptr, args)
        }
    }
    #[doc = "Like \\[method naturalcasecmp_to] but prioritizes strings that begin with periods (`.`) and underscores (`_`) before any other character. Useful when sorting folders or file names.\n\nTo get a `bool` result from a string comparison, use the `==` operator instead. See also \\[method filenocasecmp_to], \\[method naturalcasecmp_to], and \\[method casecmp_to]."]
    pub fn filecasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(487usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "filecasecmp_to", self.sys_ptr, args)
        }
    }
    #[doc = "Like \\[method naturalnocasecmp_to] but prioritizes strings that begin with periods (`.`) and underscores (`_`) before any other character. Useful when sorting folders or file names.\n\nTo get a `bool` result from a string comparison, use the `==` operator instead. See also \\[method filecasecmp_to], \\[method naturalnocasecmp_to], and \\[method nocasecmp_to]."]
    pub fn filenocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(488usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "filenocasecmp_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of characters in the string. Empty strings (`\"\"`) always return `0`. See also \\[method is_empty]."]
    pub fn length(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(489usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "length", self.sys_ptr, args)
        }
    }
    #[doc = "Returns part of the string from the position `from` with length `len`. If `len` is `-1` (as by default), returns the rest of the string starting from the given position."]
    pub fn substr(&self, from: i64, len: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64,);
        let args = (from, len,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(490usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "substr", self.sys_ptr, args)
        }
    }
    #[doc = "Splits the string using a `delimiter` and returns the substring at index `slice`. Returns the original string if `delimiter` does not occur in the string. Returns an empty string if the `slice` does not exist.\n\nThis is faster than \\[method split], if you only need one substring.\n\n```gdscript\nprint(\"i/am/example/hi\".get_slice(\"/\", 2)) # Prints \"example\"\n```"]
    pub fn get_slice(&self, delimiter: impl AsArg < GString >, slice: i64,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (delimiter.into_arg(), slice,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(491usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_slice", self.sys_ptr, args)
        }
    }
    #[doc = "Splits the string using a Unicode character with code `delimiter` and returns the substring at index `slice`. Returns an empty string if the `slice` does not exist.\n\nThis is faster than \\[method split], if you only need one substring."]
    pub fn get_slicec(&self, delimiter: i64, slice: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64,);
        let args = (delimiter, slice,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(492usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_slicec", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the total number of slices when the string is split with the given `delimiter` (see \\[method split])."]
    pub fn get_slice_count(&self, delimiter: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (delimiter.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(493usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_slice_count", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the index of the **first** occurrence of `what` in this string, or `-1` if there are none. The search's start can be specified with `from`, continuing to the end of the string.\n\n\n```gdscript\nprint(\"Team\".find(\"I\")) # Prints -1\n\nprint(\"Potato\".find(\"t\"))    # Prints 2\nprint(\"Potato\".find(\"t\", 3)) # Prints 4\nprint(\"Potato\".find(\"t\", 5)) # Prints -1\n```\n\n\n**Note:** If you just want to know whether the string contains `what`, use \\[method contains]. In GDScript, you may also use the `in` operator.\n\n**Note:** A negative value of `from` is converted to a starting index by counting back from the last possible index with enough space to find `what`."]
    pub fn find(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(494usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "find", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the index of the **first** **case-insensitive** occurrence of `what` in this string, or `-1` if there are none. The starting search index can be specified with `from`, continuing to the end of the string."]
    pub fn findn(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(495usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "findn", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of occurrences of the substring `what` between `from` and `to` positions. If `to` is 0, the search continues until the end of the string."]
    pub fn count(&self, what: impl AsArg < GString >, from: i64, to: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64, i64,);
        let args = (what.into_arg(), from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(496usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "count", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the number of occurrences of the substring `what` between `from` and `to` positions, **ignoring case**. If `to` is 0, the search continues until the end of the string."]
    pub fn countn(&self, what: impl AsArg < GString >, from: i64, to: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64, i64,);
        let args = (what.into_arg(), from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(497usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "countn", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the index of the **last** occurrence of `what` in this string, or `-1` if there are none. The search's start can be specified with `from`, continuing to the beginning of the string. This method is the reverse of \\[method find].\n\n**Note:** A negative value of `from` is converted to a starting index by counting back from the last possible index with enough space to find `what`.\n\n**Note:** A value of `from` that is greater than the last possible index with enough space to find `what` is considered out-of-bounds, and returns `-1`."]
    pub fn rfind(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(498usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "rfind", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the index of the **last** **case-insensitive** occurrence of `what` in this string, or `-1` if there are none. The starting search index can be specified with `from`, continuing to the beginning of the string. This method is the reverse of \\[method findn]."]
    pub fn rfindn(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(499usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "rfindn", self.sys_ptr, args)
        }
    }
    #[doc = "Does a simple expression match (also called \"glob\" or \"globbing\"), where `*` matches zero or more arbitrary characters and `?` matches any single character except a period (`.`). An empty string or empty expression always evaluates to `false`."]
    pub fn match_(&self, expr: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (expr.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(500usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "match", self.sys_ptr, args)
        }
    }
    #[doc = "Does a simple **case-insensitive** expression match, where `*` matches zero or more arbitrary characters and `?` matches any single character except a period (`.`). An empty string or empty expression always evaluates to `false`."]
    pub fn matchn(&self, expr: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (expr.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(501usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "matchn", self.sys_ptr, args)
        }
    }
    #[doc = "Formats the string by replacing all occurrences of `placeholder` with the elements of `values`.\n\n`values` can be a [`Dictionary`][crate::builtin::Dictionary], an [`Array`][crate::builtin::Array], or an [`Object`][crate::classes::Object]. Any underscores in `placeholder` will be replaced with the corresponding keys in advance. Array elements use their index as keys.\n\n```gdscript\n# Prints \"Waiting for Godot is a play by Samuel Beckett, and Godot Engine is named after it.\"\nvar use_array_values = \"Waiting for {0} is a play by {1}, and {0} Engine is named after it.\"\nprint(use_array_values.format([\"Godot\", \"Samuel Beckett\"]))\n\n# Prints \"User 42 is Godot.\"\nprint(\"User {id} is {name}.\".format({\"id\": 42, \"name\": \"Godot\"}))\n```\n\nSome additional handling is performed when `values` is an [`Array`][crate::builtin::Array]. If `placeholder` does not contain an underscore, the elements of the `values` array will be used to replace one occurrence of the placeholder in order; If an element of `values` is another 2-element array, it'll be interpreted as a key-value pair.\n\n```gdscript\n# Prints \"User 42 is Godot.\"\nprint(\"User {} is {}.\".format([42, \"Godot\"], \"{}\"))\nprint(\"User {id} is {name}.\".format([[\"id\", 42], [\"name\", \"Godot\"]]))\n```\n\nWhen passing an [`Object`][crate::classes::Object], the property names from [`get_property_list`][`crate::classes::Object::get_property_list`] are used as keys.\n\n```gdscript\n# Prints \"Visible true, position (0, 0)\"\nvar node = Node2D.new()\nprint(\"Visible {visible}, position {position}\".format(node))\n```\n\nSee also the [GDScript format string]($DOCS_URL/tutorials/scripting/gdscript/gdscript_format_string.html) tutorial.\n\n**Note:** Each replacement is done sequentially for each element of `values`, **not** all at once. This means that if any element is inserted and it contains another placeholder, it may be changed by the next replacement. While this can be very useful, it often causes unexpected results. If not necessary, make sure `values`'s elements do not contain placeholders.\n\n```gdscript\nprint(\"{0} {1}\".format([\"{1}\", \"x\"]))           # Prints \"x x\"\nprint(\"{0} {1}\".format([\"x\", \"{0}\"]))           # Prints \"x {0}\"\nprint(\"{a} {b}\".format({\"a\": \"{b}\", \"b\": \"c\"})) # Prints \"c c\"\nprint(\"{a} {b}\".format({\"b\": \"c\", \"a\": \"{b}\"})) # Prints \"{b} c\"\n```\n\n**Note:** In C#, it's recommended to [interpolate strings with \"$\"](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/tokens/interpolated), instead."]
    pub fn format(&self, values: &Variant, placeholder: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, CowArg < 'a1, GString >,);
        let args = (RefArg::new(values), placeholder.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(508usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "format", self.sys_ptr, args)
        }
    }
    #[doc = "Replaces all occurrences of the Unicode character with code `key` with the Unicode character with code `with`. Faster version of \\[method replace] when the key is only one character long. To get a single character use `\"X\".unicode_at(0)` (note that some strings, like compound letters and emoji, can be composed of multiple unicode codepoints, and will not work with this method, use \\[method length] to make sure)."]
    pub fn replace_char(&self, key: i64, with: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64,);
        let args = (key, with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(511usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "replace_char", self.sys_ptr, args)
        }
    }
    #[doc = "Replaces any occurrence of the characters in `keys` with the Unicode character with code `with`. See also \\[method replace_char]."]
    pub fn replace_chars(&self, keys: impl AsArg < GString >, with: i64,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (keys.into_arg(), with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(512usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "replace_chars", self.sys_ptr, args)
        }
    }
    #[doc = "Removes all occurrences of the Unicode character with code `what`. Faster version of \\[method replace] when the key is only one character long and the replacement is `\"\"`."]
    pub fn remove_char(&self, what: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (what,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(513usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "remove_char", self.sys_ptr, args)
        }
    }
    #[doc = "Removes all occurrences of the characters in `chars`. See also \\[method remove_char]."]
    pub fn remove_chars(&self, chars: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(514usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "remove_chars", self.sys_ptr, args)
        }
    }
    #[doc = "Inserts `what` at the given `position` in the string."]
    pub fn insert(&self, position: i64, what: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (position, what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(517usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "insert", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a string with `chars` characters erased starting from `position`. If `chars` goes beyond the string's length given the specified `position`, fewer characters will be erased from the returned string. Returns an empty string if either `position` or `chars` is negative. Returns the original string unmodified if `chars` is `0`."]
    pub fn erase(&self, position: i64, chars: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64,);
        let args = (position, chars,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(518usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "erase", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the string converted to `kebab-case`.\n\n**Note:** Numbers followed by a _single_ letter are not separated in the conversion to keep some words (such as \"2D\") together.\n\n\n```gdscript\n\"Node2D\".to_kebab_case()               # Returns \"node-2d\"\n\"2nd place\".to_kebab_case()            # Returns \"2-nd-place\"\n\"Texture3DAssetFolder\".to_kebab_case() # Returns \"texture-3d-asset-folder\"\n```\n"]
    pub fn to_kebab_case(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(523usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_kebab_case", self.sys_ptr, args)
        }
    }
    #[doc = "Splits the string using a `delimiter` and returns an array of the substrings. If `delimiter` is an empty string, each substring will be a single character. This method is the opposite of \\[method join].\n\nIf `allow_empty` is `false`, empty strings between adjacent delimiters are excluded from the array.\n\nIf `maxsplit` is greater than `0`, the number of splits may not exceed `maxsplit`. By default, the entire string is split.\n\n\n```gdscript\nvar some_array = \"One,Two,Three,Four\".split(\",\", true, 2)\n\nprint(some_array.size()) # Prints 3\nprint(some_array[0])     # Prints \"One\"\nprint(some_array[1])     # Prints \"Two\"\nprint(some_array[2])     # Prints \"Three,Four\"\n```\n\n\n**Note:** If you only need one substring from the array, consider using \\[method get_slice] which is faster. If you need to split strings with more complex rules, use the [`RegEx`][crate::classes::RegEx] class instead."]
    pub fn split(&self, delimiter: impl AsArg < GString >, allow_empty: bool, maxsplit: i64,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool, i64,);
        let args = (delimiter.into_arg(), allow_empty, maxsplit,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(524usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "split", self.sys_ptr, args)
        }
    }
    #[doc = "Splits the string using a `delimiter` and returns an array of the substrings, starting from the end of the string. The splits in the returned array appear in the same order as the original string. If `delimiter` is an empty string, each substring will be a single character.\n\nIf `allow_empty` is `false`, empty strings between adjacent delimiters are excluded from the array.\n\nIf `maxsplit` is greater than `0`, the number of splits may not exceed `maxsplit`. By default, the entire string is split, which is mostly identical to \\[method split].\n\n\n```gdscript\nvar some_string = \"One,Two,Three,Four\"\nvar some_array = some_string.rsplit(\",\", true, 1)\n\nprint(some_array.size()) # Prints 2\nprint(some_array[0])     # Prints \"One,Two,Three\"\nprint(some_array[1])     # Prints \"Four\"\n```\n"]
    pub fn rsplit(&self, delimiter: impl AsArg < GString >, allow_empty: bool, maxsplit: i64,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool, i64,);
        let args = (delimiter.into_arg(), allow_empty, maxsplit,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(525usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "rsplit", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the character code at position `at`.\n\nSee also [`chr`][`crate::builtin::GString::chr`], \\[method @GDScript.char], and \\[method @GDScript.ord]."]
    pub fn unicode_at(&self, at: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (at,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(539usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "unicode_at", self.sys_ptr, args)
        }
    }
    #[doc = "Decodes the file path from its URL-encoded format. Unlike \\[method uri_decode] this method leaves `+` as is."]
    pub fn uri_file_decode(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(560usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "uri_file_decode", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this string is a valid ASCII identifier. A valid ASCII identifier may contain only letters, digits, and underscores (`_`), and the first character may not be a digit.\n\n```gdscript\nprint(\"node_2d\".is_valid_ascii_identifier())    # Prints true\nprint(\"TYPE_FLOAT\".is_valid_ascii_identifier()) # Prints true\nprint(\"1st_method\".is_valid_ascii_identifier()) # Prints false\nprint(\"MyMethod#2\".is_valid_ascii_identifier()) # Prints false\n```\n\nSee also \\[method is_valid_unicode_identifier]."]
    pub fn is_valid_ascii_identifier(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(566usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_ascii_identifier", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this string is a valid Unicode identifier.\n\nA valid Unicode identifier must begin with a Unicode character of class `XID_Start` or `\"_\"`, and may contain Unicode characters of class `XID_Continue` in the other positions.\n\n```gdscript\nprint(\"node_2d\".is_valid_unicode_identifier())      # Prints true\nprint(\"1st_method\".is_valid_unicode_identifier())   # Prints false\nprint(\"MyMethod#2\".is_valid_unicode_identifier())   # Prints false\nprint(\"állóképesség\".is_valid_unicode_identifier()) # Prints true\nprint(\"выносливость\".is_valid_unicode_identifier()) # Prints true\nprint(\"体力\".is_valid_unicode_identifier())         # Prints true\n```\n\nSee also \\[method is_valid_ascii_identifier].\n\n**Note:** This method checks identifiers the same way as GDScript. See [`is_valid_identifier`][`crate::classes::TextServer::is_valid_identifier`] for more advanced checks."]
    pub fn is_valid_unicode_identifier(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(567usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_unicode_identifier", self.sys_ptr, args)
        }
    }
    #[doc = "Formats the string to be at least `min_length` long by adding `character`s to the left of the string, if necessary. See also \\[method rpad]."]
    pub fn lpad(&self, min_length: i64, character: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (min_length, character.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(579usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "lpad", self.sys_ptr, args)
        }
    }
    #[doc = "Formats the string to be at least `min_length` long, by adding `character`s to the right of the string, if necessary. See also \\[method lpad]."]
    pub fn rpad(&self, min_length: i64, character: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (min_length, character.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(580usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "rpad", self.sys_ptr, args)
        }
    }
    #[doc = "Formats the string representing a number to have an exact number of `digits` _after_ the decimal point."]
    pub fn pad_decimals(&self, digits: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (digits,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(581usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "pad_decimals", self.sys_ptr, args)
        }
    }
    #[doc = "Formats the string representing a number to have an exact number of `digits` _before_ the decimal point."]
    pub fn pad_zeros(&self, digits: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (digits,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(582usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "pad_zeros", self.sys_ptr, args)
        }
    }
    #[doc = "Converts the string to system multibyte code page encoded [`PackedByteArray`][crate::builtin::PackedByteArray]. If conversion fails, empty array is returned.\n\nThe values permitted for `encoding` are system dependent. If `encoding` is empty string, system default encoding is used.\n\n- For Windows, see [Code Page Identifiers](https://learn.microsoft.com/en-us/windows/win32/Intl/code-page-identifiers) .NET names.\n\n- For macOS and Linux/BSD, see `libiconv` library documentation and `iconv --list` for a list of supported encodings."]
    pub fn to_multibyte_char_buffer(&self, encoding: impl AsArg < GString >,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (encoding.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(590usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_multibyte_char_buffer", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the 32-bit hash value representing the string's contents.\n\n**Note:** Strings with equal hash values are _not_ guaranteed to be the same, as a result of hash collisions. On the contrary, strings with different hash values are guaranteed to be different."]
    pub fn hash(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(592usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "hash", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerStringName;
impl StringName {
    #[doc = "Returns `true` if the string begins with the given `text`. See also \\[method ends_with]."]
    pub fn begins_with(&self, text: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(502usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "begins_with", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if the string ends with the given `text`. See also \\[method begins_with]."]
    pub fn ends_with(&self, text: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(503usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "ends_with", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if all characters of this string can be found in `text` in their original order. This is not the same as \\[method contains].\n\n```gdscript\nvar text = \"Wow, incredible!\"\n\nprint(\"inedible\".is_subsequence_of(text)) # Prints true\nprint(\"Word!\".is_subsequence_of(text))    # Prints true\nprint(\"Window\".is_subsequence_of(text))   # Prints false\nprint(\"\".is_subsequence_of(text))         # Prints true\n```"]
    pub fn is_subsequence_of(&self, text: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(504usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_subsequence_of", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if all characters of this string can be found in `text` in their original order, **ignoring case**. This is not the same as \\[method containsn]."]
    pub fn is_subsequence_ofn(&self, text: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(505usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_subsequence_ofn", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns an array containing the bigrams (pairs of consecutive characters) of this string.\n\n```gdscript\nprint(\"Get up!\".bigrams()) # Prints [\"Ge\", \"et\", \"t \", \" u\", \"up\", \"p!\"]\n```"]
    pub fn bigrams(&self,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(506usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "bigrams", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the similarity index ([Sørensen-Dice coefficient](https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient)) of this string compared to another. A result of `1.0` means totally similar, while `0.0` means totally dissimilar.\n\n```gdscript\nprint(\"ABC123\".similarity(\"ABC123\")) # Prints 1.0\nprint(\"ABC123\".similarity(\"XYZ456\")) # Prints 0.0\nprint(\"ABC123\".similarity(\"123ABC\")) # Prints 0.8\nprint(\"ABC123\".similarity(\"abc123\")) # Prints 0.4\n```"]
    pub fn similarity(&self, text: impl AsArg < GString >,) -> f64 {
        type CallRet = f64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(507usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "similarity", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Replaces all occurrences of `what` inside the string with the given `forwhat`."]
    pub fn replace(&self, what: impl AsArg < GString >, forwhat: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
        let args = (what.into_arg(), forwhat.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(509usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "replace", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Replaces all **case-insensitive** occurrences of `what` inside the string with the given `forwhat`."]
    pub fn replacen(&self, what: impl AsArg < GString >, forwhat: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
        let args = (what.into_arg(), forwhat.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(510usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "replacen", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Repeats this string a number of times. `count` needs to be greater than `0`. Otherwise, returns an empty string."]
    pub fn repeat(&self, count: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(515usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "repeat", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the copy of this string in reverse order. This operation works on unicode codepoints, rather than sequences of codepoints, and may break things like compound letters or emojis."]
    pub fn reverse(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(516usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "reverse", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Changes the appearance of the string: replaces underscores (`_`) with spaces, adds spaces before uppercase letters in the middle of a word, converts all letters to lowercase, then converts the first one and each one following a space to uppercase.\n\n\n```gdscript\n\"move_local_x\".capitalize()   # Returns \"Move Local X\"\n\"sceneFile_path\".capitalize() # Returns \"Scene File Path\"\n\"2D, FPS, PNG\".capitalize()   # Returns \"2d, Fps, Png\"\n```\n"]
    pub fn capitalize(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(519usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "capitalize", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the string converted to `camelCase`."]
    pub fn to_camel_case(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(520usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_camel_case", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the string converted to `PascalCase`."]
    pub fn to_pascal_case(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(521usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_pascal_case", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the string converted to `snake_case`.\n\n**Note:** Numbers followed by a _single_ letter are not separated in the conversion to keep some words (such as \"2D\") together.\n\n\n```gdscript\n\"Node2D\".to_snake_case()               # Returns \"node_2d\"\n\"2nd place\".to_snake_case()            # Returns \"2_nd_place\"\n\"Texture3DAssetFolder\".to_snake_case() # Returns \"texture_3d_asset_folder\"\n```\n"]
    pub fn to_snake_case(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(522usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_snake_case", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Splits the string into floats by using a `delimiter` and returns a [`PackedFloat64Array`][crate::builtin::PackedFloat64Array].\n\nIf `allow_empty` is `false`, empty or invalid `float` conversions between adjacent delimiters are excluded.\n\n```gdscript\nvar a = \"1,2,4.5\".split_floats(\",\")         # a is [1.0, 2.0, 4.5]\nvar c = \"1| ||4.5\".split_floats(\"|\")        # c is [1.0, 0.0, 0.0, 4.5]\nvar b = \"1| ||4.5\".split_floats(\"|\", false) # b is [1.0, 4.5]\n```"]
    pub(crate) fn split_floats_full(&self, delimiter: CowArg < GString >, allow_empty: bool,) -> PackedFloat64Array {
        type CallRet = PackedFloat64Array;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
        let args = (delimiter, allow_empty,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(526usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "split_floats", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "To set the default parameters, use [`split_floats_ex`][Self::split_floats_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
    #[doc = "Splits the string into floats by using a `delimiter` and returns a [`PackedFloat64Array`][crate::builtin::PackedFloat64Array].\n\nIf `allow_empty` is `false`, empty or invalid `float` conversions between adjacent delimiters are excluded.\n\n```gdscript\nvar a = \"1,2,4.5\".split_floats(\",\")         # a is [1.0, 2.0, 4.5]\nvar c = \"1| ||4.5\".split_floats(\"|\")        # c is [1.0, 0.0, 0.0, 4.5]\nvar b = \"1| ||4.5\".split_floats(\"|\", false) # b is [1.0, 4.5]\n```"]
    #[inline]
    pub fn split_floats(&self, delimiter: impl AsArg < GString >,) -> PackedFloat64Array {
        self.split_floats_ex(delimiter,) . done()
    }
    #[doc = "Splits the string into floats by using a `delimiter` and returns a [`PackedFloat64Array`][crate::builtin::PackedFloat64Array].\n\nIf `allow_empty` is `false`, empty or invalid `float` conversions between adjacent delimiters are excluded.\n\n```gdscript\nvar a = \"1,2,4.5\".split_floats(\",\")         # a is [1.0, 2.0, 4.5]\nvar c = \"1| ||4.5\".split_floats(\"|\")        # c is [1.0, 0.0, 0.0, 4.5]\nvar b = \"1| ||4.5\".split_floats(\"|\", false) # b is [1.0, 4.5]\n```"]
    #[inline]
    pub fn split_floats_ex < 'ex > (&'ex self, delimiter: impl AsArg < GString > + 'ex,) -> ExSplitFloats < 'ex > {
        ExSplitFloats::new(self, delimiter,)
    }
    #[doc = "Returns the concatenation of `parts`' elements, with each element separated by the string calling this method. This method is the opposite of \\[method split].\n\n\n```gdscript\nvar fruits = [\"Apple\", \"Orange\", \"Pear\", \"Kiwi\"]\n\nprint(\", \".join(fruits))  # Prints \"Apple, Orange, Pear, Kiwi\"\nprint(\"---\".join(fruits)) # Prints \"Apple---Orange---Pear---Kiwi\"\n```\n"]
    pub fn join(&self, parts: &PackedStringArray,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
        let args = (RefArg::new(parts),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(527usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "join", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the string converted to `UPPERCASE`."]
    pub fn to_upper(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(528usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_upper", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the string converted to `lowercase`."]
    pub fn to_lower(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(529usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_lower", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the first `length` characters from the beginning of the string. If `length` is negative, strips the last `length` characters from the string's end.\n\n```gdscript\nprint(\"Hello World!\".left(3))  # Prints \"Hel\"\nprint(\"Hello World!\".left(-4)) # Prints \"Hello Wo\"\n```"]
    pub fn left(&self, length: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(530usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "left", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the last `length` characters from the end of the string. If `length` is negative, strips the first `length` characters from the string's beginning.\n\n```gdscript\nprint(\"Hello World!\".right(3))  # Prints \"ld!\"\nprint(\"Hello World!\".right(-4)) # Prints \"o World!\"\n```"]
    pub fn right(&self, length: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(531usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "right", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Strips all non-printable characters from the beginning and the end of the string. These include spaces, tabulations (`\\t`), and newlines (`\\n` `\\r`).\n\nIf `left` is `false`, ignores the string's beginning. Likewise, if `right` is `false`, ignores the string's end."]
    pub(crate) fn strip_edges_full(&self, left: bool, right: bool,) -> GString {
        type CallRet = GString;
        type CallParams = (bool, bool,);
        let args = (left, right,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(532usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "strip_edges", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "To set the default parameters, use [`strip_edges_ex`][Self::strip_edges_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
    #[doc = "Strips all non-printable characters from the beginning and the end of the string. These include spaces, tabulations (`\\t`), and newlines (`\\n` `\\r`).\n\nIf `left` is `false`, ignores the string's beginning. Likewise, if `right` is `false`, ignores the string's end."]
    #[inline]
    pub fn strip_edges(&self,) -> GString {
        self.strip_edges_ex() . done()
    }
    #[doc = "Strips all non-printable characters from the beginning and the end of the string. These include spaces, tabulations (`\\t`), and newlines (`\\n` `\\r`).\n\nIf `left` is `false`, ignores the string's beginning. Likewise, if `right` is `false`, ignores the string's end."]
    #[inline]
    pub fn strip_edges_ex < 'ex > (&'ex self,) -> ExStripEdges < 'ex > {
        ExStripEdges::new(self,)
    }
    #[doc = "Strips all escape characters from the string. These include all non-printable control characters of the first page of the ASCII table (values from 0 to 31), such as tabulation (`\\t`) and newline (`\\n`, `\\r`) characters, but _not_ spaces."]
    pub fn strip_escapes(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(533usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "strip_escapes", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Removes a set of characters defined in `chars` from the string's beginning. See also \\[method rstrip].\n\n**Note:** `chars` is not a prefix. Use \\[method trim_prefix] to remove a single prefix, rather than a set of characters."]
    pub fn lstrip(&self, chars: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(534usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "lstrip", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Removes a set of characters defined in `chars` from the string's end. See also \\[method lstrip].\n\n**Note:** `chars` is not a suffix. Use \\[method trim_suffix] to remove a single suffix, rather than a set of characters."]
    pub fn rstrip(&self, chars: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(535usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "rstrip", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "If the string is a valid file name or path, returns the file extension without the leading period (`.`). Otherwise, returns an empty string.\n\n```gdscript\nvar a = \"/path/to/file.txt\".get_extension() # a is \"txt\"\nvar b = \"cool.txt\".get_extension()          # b is \"txt\"\nvar c = \"cool.font.tres\".get_extension()    # c is \"tres\"\nvar d = \".pack1\".get_extension()            # d is \"pack1\"\n\nvar e = \"file.txt.\".get_extension()  # e is \"\"\nvar f = \"file.txt..\".get_extension() # f is \"\"\nvar g = \"txt\".get_extension()        # g is \"\"\nvar h = \"\".get_extension()           # h is \"\"\n```"]
    pub fn get_extension(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(536usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_extension", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "If the string is a valid file path, returns the full file path, without the extension.\n\n```gdscript\nvar base = \"/path/to/file.txt\".get_basename() # base is \"/path/to/file\"\n```"]
    pub fn get_basename(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(537usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_basename", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Concatenates `path` at the end of the string as a subpath, adding `/` if necessary.\n\n**Example:** `\"this/is\".path_join(\"path\") == \"this/is/path\"`."]
    pub fn path_join(&self, path: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (path.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(538usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "path_join", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Indents every line of the string with the given `prefix`. Empty lines are not indented. See also \\[method dedent] to remove indentation.\n\nFor example, the string can be indented with two tabulations using `\"\\t\\t\"`, or four spaces using `\"    \"`."]
    pub fn indent(&self, prefix: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (prefix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(540usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "indent", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns a copy of the string with indentation (leading tabs and spaces) removed. See also \\[method indent] to add indentation."]
    pub fn dedent(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(541usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "dedent", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the [MD5 hash](https://en.wikipedia.org/wiki/MD5) of the string as another [`String`][crate::builtin::GString]."]
    pub fn md5_text(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(542usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "md5_text", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the [SHA-1](https://en.wikipedia.org/wiki/SHA-1) hash of the string as another [`String`][crate::builtin::GString]."]
    pub fn sha1_text(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(543usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "sha1_text", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the [SHA-256](https://en.wikipedia.org/wiki/SHA-2) hash of the string as another [`String`][crate::builtin::GString]."]
    pub fn sha256_text(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(544usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "sha256_text", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the [MD5 hash](https://en.wikipedia.org/wiki/MD5) of the string as a [`PackedByteArray`][crate::builtin::PackedByteArray]."]
    pub fn md5_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(545usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "md5_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the [SHA-1](https://en.wikipedia.org/wiki/SHA-1) hash of the string as a [`PackedByteArray`][crate::builtin::PackedByteArray]."]
    pub fn sha1_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(546usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "sha1_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns the [SHA-256](https://en.wikipedia.org/wiki/SHA-2) hash of the string as a [`PackedByteArray`][crate::builtin::PackedByteArray]."]
    pub fn sha256_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(547usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "sha256_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if the string's length is `0` (`\"\"`). See also \\[method length]."]
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(548usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_empty", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if the string contains `what`. In GDScript, this corresponds to the `in` operator.\n\n\n```gdscript\nprint(\"Node\".contains(\"de\")) # Prints true\nprint(\"team\".contains(\"I\"))  # Prints false\nprint(\"I\" in \"team\")         # Prints false\n```\n\n\nIf you need to know where `what` is within the string, use \\[method find]. See also \\[method containsn]."]
    pub fn contains(&self, what: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(549usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "contains", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if the string contains `what`, **ignoring case**.\n\nIf you need to know where `what` is within the string, use \\[method findn]. See also \\[method contains]."]
    pub fn containsn(&self, what: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(550usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "containsn", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if the string is a path to a file or directory, and its starting point is explicitly defined. This method is the opposite of \\[method is_relative_path].\n\nThis includes all paths starting with `\"res://\"`, `\"user://\"`, `\"C:\\\"`, `\"/\"`, etc."]
    pub fn is_absolute_path(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(551usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_absolute_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if the string is a path, and its starting point is dependent on context. The path could begin from the current directory, or the current [`Node`][crate::classes::Node] (if the string is derived from a [`NodePath`][crate::builtin::NodePath]), and may sometimes be prefixed with `\"./\"`. This method is the opposite of \\[method is_absolute_path]."]
    pub fn is_relative_path(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(552usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_relative_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "If the string is a valid file path, converts the string into a canonical path. This is the shortest possible path, without `\"./\"`, and all the unnecessary `\"..\"` and `\"/\"`.\n\n```gdscript\nvar simple_path = \"./path/to///../file\".simplify_path()\nprint(simple_path) # Prints \"path/file\"\n```"]
    pub fn simplify_path(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(553usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "simplify_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "If the string is a valid file path, returns the base directory name.\n\n```gdscript\nvar dir_path = \"/path/to/file.txt\".get_base_dir() # dir_path is \"/path/to\"\n```"]
    pub fn get_base_dir(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(554usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_base_dir", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "If the string is a valid file path, returns the file name, including the extension.\n\n```gdscript\nvar file = \"/path/to/icon.png\".get_file() # file is \"icon.png\"\n```"]
    pub fn get_file(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(555usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_file", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns a copy of the string with special characters escaped using the XML standard. If `escape_quotes` is `true`, the single quote (`'`) and double quote (`\"`) characters are also escaped."]
    pub(crate) fn xml_escape_full(&self, escape_quotes: bool,) -> GString {
        type CallRet = GString;
        type CallParams = (bool,);
        let args = (escape_quotes,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(556usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "xml_escape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "To set the default parameters, use [`xml_escape_ex`][Self::xml_escape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
    #[doc = "Returns a copy of the string with special characters escaped using the XML standard. If `escape_quotes` is `true`, the single quote (`'`) and double quote (`\"`) characters are also escaped."]
    #[inline]
    pub fn xml_escape(&self,) -> GString {
        self.xml_escape_ex() . done()
    }
    #[doc = "Returns a copy of the string with special characters escaped using the XML standard. If `escape_quotes` is `true`, the single quote (`'`) and double quote (`\"`) characters are also escaped."]
    #[inline]
    pub fn xml_escape_ex < 'ex > (&'ex self,) -> ExXmlEscape < 'ex > {
        ExXmlEscape::new(self,)
    }
    #[doc = "Returns a copy of the string with escaped characters replaced by their meanings according to the XML standard."]
    pub fn xml_unescape(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(557usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "xml_unescape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Encodes the string to URL-friendly format. This method is meant to properly encode the parameters in a URL when sending an HTTP request. See also \\[method uri_decode].\n\n\n```gdscript\nvar prefix = \"$DOCS_URL/?highlight=\"\nvar url = prefix + \"Godot Engine:docs\".uri_encode()\n\nprint(url) # Prints \"$DOCS_URL/?highlight=Godot%20Engine%3%docs\"\n```\n"]
    pub fn uri_encode(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(558usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "uri_encode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Decodes the string from its URL-encoded format. This method is meant to properly decode the parameters in a URL when receiving an HTTP request. See also \\[method uri_encode].\n\n\n```gdscript\nvar url = \"$DOCS_URL/?highlight=Godot%20Engine%3%docs\"\nprint(url.uri_decode()) # Prints \"$DOCS_URL/?highlight=Godot Engine:docs\"\n```\n\n\n**Note:** This method decodes `+` as space."]
    pub fn uri_decode(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(559usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "uri_decode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns a copy of the string with special characters escaped using the C language standard."]
    pub fn c_escape(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(561usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "c_escape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns a copy of the string with escaped characters replaced by their meanings. Supported escape sequences are `\\'`, `\\\"`, `\\\\`, `\\a`, `\\b`, `\\f`, `\\n`, `\\r`, `\\t`, `\\v`.\n\n**Note:** Unlike the GDScript parser, this method doesn't support the `\\uXXXX` escape sequence."]
    pub fn c_unescape(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(562usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "c_unescape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns a copy of the string with special characters escaped using the JSON standard. Because it closely matches the C standard, it is possible to use \\[method c_unescape] to unescape the string, if necessary."]
    pub fn json_escape(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(563usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "json_escape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns a copy of the string with all characters that are not allowed in \\[member Node.name] (`.` `:` `@` `/` `\"` `%`) replaced with underscores."]
    pub fn validate_node_name(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(564usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "validate_node_name", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns a copy of the string with all characters that are not allowed in \\[method is_valid_filename] replaced with underscores."]
    pub fn validate_filename(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(565usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "validate_filename", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if this string is a valid identifier. A valid identifier may contain only letters, digits and underscores (`_`), and the first character may not be a digit.\n\n```gdscript\nprint(\"node_2d\".is_valid_identifier())    # Prints true\nprint(\"TYPE_FLOAT\".is_valid_identifier()) # Prints true\nprint(\"1st_method\".is_valid_identifier()) # Prints false\nprint(\"MyMethod#2\".is_valid_identifier()) # Prints false\n```"]
    pub fn is_valid_identifier(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(568usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_identifier", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if this string represents a valid integer. A valid integer only contains digits, and may be prefixed with a positive (`+`) or negative (`-`) sign. See also \\[method to_int].\n\n```gdscript\nprint(\"7\".is_valid_int())    # Prints true\nprint(\"1.65\".is_valid_int()) # Prints false\nprint(\"Hi\".is_valid_int())   # Prints false\nprint(\"+3\".is_valid_int())   # Prints true\nprint(\"-12\".is_valid_int())  # Prints true\n```"]
    pub fn is_valid_int(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(569usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if this string represents a valid floating-point number. A valid float may contain only digits, one decimal point (`.`), and the exponent letter (`e`). It may also be prefixed with a positive (`+`) or negative (`-`) sign. Any valid integer is also a valid float (see \\[method is_valid_int]). See also \\[method to_float].\n\n```gdscript\nprint(\"1.7\".is_valid_float())   # Prints true\nprint(\"24\".is_valid_float())    # Prints true\nprint(\"7e3\".is_valid_float())   # Prints true\nprint(\"Hello\".is_valid_float()) # Prints false\n```"]
    pub fn is_valid_float(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(570usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_float", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if this string is a valid hexadecimal number. A valid hexadecimal number only contains digits or letters `A` to `F` (either uppercase or lowercase), and may be prefixed with a positive (`+`) or negative (`-`) sign.\n\nIf `with_prefix` is `true`, the hexadecimal number needs to prefixed by `\"0x\"` to be considered valid.\n\n```gdscript\nprint(\"A08E\".is_valid_hex_number())    # Prints true\nprint(\"-AbCdEf\".is_valid_hex_number()) # Prints true\nprint(\"2.5\".is_valid_hex_number())     # Prints false\n\nprint(\"0xDEADC0DE\".is_valid_hex_number(true)) # Prints true\n```"]
    pub(crate) fn is_valid_hex_number_full(&self, with_prefix: bool,) -> bool {
        type CallRet = bool;
        type CallParams = (bool,);
        let args = (with_prefix,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(571usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_hex_number", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "To set the default parameters, use [`is_valid_hex_number_ex`][Self::is_valid_hex_number_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
    #[doc = "Returns `true` if this string is a valid hexadecimal number. A valid hexadecimal number only contains digits or letters `A` to `F` (either uppercase or lowercase), and may be prefixed with a positive (`+`) or negative (`-`) sign.\n\nIf `with_prefix` is `true`, the hexadecimal number needs to prefixed by `\"0x\"` to be considered valid.\n\n```gdscript\nprint(\"A08E\".is_valid_hex_number())    # Prints true\nprint(\"-AbCdEf\".is_valid_hex_number()) # Prints true\nprint(\"2.5\".is_valid_hex_number())     # Prints false\n\nprint(\"0xDEADC0DE\".is_valid_hex_number(true)) # Prints true\n```"]
    #[inline]
    pub fn is_valid_hex_number(&self,) -> bool {
        self.is_valid_hex_number_ex() . done()
    }
    #[doc = "Returns `true` if this string is a valid hexadecimal number. A valid hexadecimal number only contains digits or letters `A` to `F` (either uppercase or lowercase), and may be prefixed with a positive (`+`) or negative (`-`) sign.\n\nIf `with_prefix` is `true`, the hexadecimal number needs to prefixed by `\"0x\"` to be considered valid.\n\n```gdscript\nprint(\"A08E\".is_valid_hex_number())    # Prints true\nprint(\"-AbCdEf\".is_valid_hex_number()) # Prints true\nprint(\"2.5\".is_valid_hex_number())     # Prints false\n\nprint(\"0xDEADC0DE\".is_valid_hex_number(true)) # Prints true\n```"]
    #[inline]
    pub fn is_valid_hex_number_ex < 'ex > (&'ex self,) -> ExIsValidHexNumber < 'ex > {
        ExIsValidHexNumber::new(self,)
    }
    #[doc = "Returns `true` if this string is a valid color in hexadecimal HTML notation. The string must be a hexadecimal value (see \\[method is_valid_hex_number]) of either 3, 4, 6 or 8 digits, and may be prefixed by a hash sign (`#`). Other HTML notations for colors, such as names or `hsl()`, are not considered valid. See also [`html`][`crate::builtin::Color::html`]."]
    pub fn is_valid_html_color(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(572usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_html_color", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if this string represents a well-formatted IPv4 or IPv6 address. This method considers [reserved IP addresses](https://en.wikipedia.org/wiki/Reserved_IP_addresses) such as `\"0.0.0.0\"` and `\"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff\"` as valid."]
    pub fn is_valid_ip_address(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(573usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_ip_address", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Returns `true` if this string is a valid file name. A valid file name cannot be empty, begin or end with space characters, or contain characters that are not allowed (`:` `/` `\\` `?` `*` `\"` `|` `%` `<` `>`)."]
    pub fn is_valid_filename(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(574usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_filename", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts the string representing an integer number into an `int`. This method removes any non-number character and stops at the first decimal point (`.`). See also \\[method is_valid_int].\n\n```gdscript\nvar a = \"123\".to_int()    # a is 123\nvar b = \"x1y2z3\".to_int() # b is 123\nvar c = \"-1.2.3\".to_int() # c is -1\nvar d = \"Hello!\".to_int() # d is 0\n```"]
    pub fn to_int(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(575usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts the string representing a decimal number into a `float`. This method stops on the first non-number character, except the first decimal point (`.`) and the exponent letter (`e`). See also \\[method is_valid_float].\n\n```gdscript\nvar a = \"12.35\".to_float()  # a is 12.35\nvar b = \"1.2.3\".to_float()  # b is 1.2\nvar c = \"12xy3\".to_float()  # c is 12.0\nvar d = \"1e3\".to_float()    # d is 1000.0\nvar e = \"Hello!\".to_float() # e is 0.0\n```"]
    pub fn to_float(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(576usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_float", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts the string representing a hexadecimal number into an `int`. The string may be optionally prefixed with `\"0x\"`, and an additional `-` prefix for negative numbers.\n\n\n```gdscript\nprint(\"0xff\".hex_to_int()) # Prints 255\nprint(\"ab\".hex_to_int())   # Prints 171\n```\n"]
    pub fn hex_to_int(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(577usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "hex_to_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts the string representing a binary number into an `int`. The string may optionally be prefixed with `\"0b\"`, and an additional `-` prefix for negative numbers.\n\n\n```gdscript\nprint(\"101\".bin_to_int())   # Prints 5\nprint(\"0b101\".bin_to_int()) # Prints 5\nprint(\"-0b10\".bin_to_int()) # Prints -2\n```\n"]
    pub fn bin_to_int(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(578usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "bin_to_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Removes the given `prefix` from the start of the string, or returns the string unchanged."]
    pub fn trim_prefix(&self, prefix: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (prefix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(583usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "trim_prefix", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Removes the given `suffix` from the end of the string, or returns the string unchanged."]
    pub fn trim_suffix(&self, suffix: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (suffix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(584usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "trim_suffix", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts the string to an [ASCII](https://en.wikipedia.org/wiki/ASCII)/Latin-1 encoded [`PackedByteArray`][crate::builtin::PackedByteArray]. This method is slightly faster than \\[method to_utf8_buffer], but replaces all unsupported characters with spaces. This is the inverse of [`get_string_from_ascii`][`crate::builtin::PackedByteArray::get_string_from_ascii`]."]
    pub fn to_ascii_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(585usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_ascii_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts the string to a [UTF-8](https://en.wikipedia.org/wiki/UTF-8) encoded [`PackedByteArray`][crate::builtin::PackedByteArray]. This method is slightly slower than \\[method to_ascii_buffer], but supports all UTF-8 characters. For most cases, prefer using this method. This is the inverse of [`get_string_from_utf8`][`crate::builtin::PackedByteArray::get_string_from_utf8`]."]
    pub fn to_utf8_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(586usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_utf8_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts the string to a [UTF-16](https://en.wikipedia.org/wiki/UTF-16) encoded [`PackedByteArray`][crate::builtin::PackedByteArray]. This is the inverse of [`get_string_from_utf16`][`crate::builtin::PackedByteArray::get_string_from_utf16`]."]
    pub fn to_utf16_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(587usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_utf16_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts the string to a [UTF-32](https://en.wikipedia.org/wiki/UTF-32) encoded [`PackedByteArray`][crate::builtin::PackedByteArray]. This is the inverse of [`get_string_from_utf32`][`crate::builtin::PackedByteArray::get_string_from_utf32`]."]
    pub fn to_utf32_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(588usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_utf32_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Converts the string to a [wide character](https://en.wikipedia.org/wiki/Wide_character) (`wchar_t`, UTF-16 on Windows, UTF-32 on other platforms) encoded [`PackedByteArray`][crate::builtin::PackedByteArray]. This is the inverse of [`get_string_from_wchar`][`crate::builtin::PackedByteArray::get_string_from_wchar`]."]
    pub fn to_wchar_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(589usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_wchar_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Decodes a hexadecimal string as a [`PackedByteArray`][crate::builtin::PackedByteArray].\n\n\n```gdscript\nvar text = \"hello world\"\nvar encoded = text.to_utf8_buffer().hex_encode() # outputs \"68656c6c6f20776f726c64\"\nprint(encoded.hex_decode().get_string_from_utf8())\n```\n"]
    pub fn hex_decode(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(591usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "hex_decode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
}
#[doc = "Default-param extender for [`StringName::split_floats_ex`][crate::builtin::StringName::split_floats_ex]."]
#[must_use]
pub struct ExSplitFloats < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex StringName, delimiter: CowArg < 'ex, GString >, allow_empty: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSplitFloats < 'ex > {
    fn new(surround_object: &'ex StringName, delimiter: impl AsArg < GString > + 'ex,) -> Self {
        let allow_empty = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, delimiter: delimiter.into_arg(), allow_empty: allow_empty,
        }
    }
    #[inline]
    pub fn allow_empty(self, allow_empty: bool) -> Self {
        Self {
            allow_empty: allow_empty, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedFloat64Array {
        let Self {
            _phantom, surround_object, delimiter, allow_empty,
        }
        = self;
        StringName::split_floats_full(surround_object, delimiter, allow_empty,)
    }
}
#[doc = "Default-param extender for [`StringName::strip_edges_ex`][crate::builtin::StringName::strip_edges_ex]."]
#[must_use]
pub struct ExStripEdges < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex StringName, left: bool, right: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExStripEdges < 'ex > {
    fn new(surround_object: &'ex StringName,) -> Self {
        let left = true;
        let right = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, left: left, right: right,
        }
    }
    #[inline]
    pub fn left(self, left: bool) -> Self {
        Self {
            left: left, .. self
        }
    }
    #[inline]
    pub fn right(self, right: bool) -> Self {
        Self {
            right: right, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, left, right,
        }
        = self;
        StringName::strip_edges_full(surround_object, left, right,)
    }
}
#[doc = "Default-param extender for [`StringName::xml_escape_ex`][crate::builtin::StringName::xml_escape_ex]."]
#[must_use]
pub struct ExXmlEscape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex StringName, escape_quotes: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExXmlEscape < 'ex > {
    fn new(surround_object: &'ex StringName,) -> Self {
        let escape_quotes = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, escape_quotes: escape_quotes,
        }
    }
    #[inline]
    pub fn escape_quotes(self, escape_quotes: bool) -> Self {
        Self {
            escape_quotes: escape_quotes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, escape_quotes,
        }
        = self;
        StringName::xml_escape_full(surround_object, escape_quotes,)
    }
}
#[doc = "Default-param extender for [`StringName::is_valid_hex_number_ex`][crate::builtin::StringName::is_valid_hex_number_ex]."]
#[must_use]
pub struct ExIsValidHexNumber < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex StringName, with_prefix: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsValidHexNumber < 'ex > {
    fn new(surround_object: &'ex StringName,) -> Self {
        let with_prefix = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, with_prefix: with_prefix,
        }
    }
    #[inline]
    pub fn with_prefix(self, with_prefix: bool) -> Self {
        Self {
            with_prefix: with_prefix, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, with_prefix,
        }
        = self;
        StringName::is_valid_hex_number_full(surround_object, with_prefix,)
    }
}
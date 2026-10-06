#![doc = "Sidecar module for class [`CodeHighlighter`][crate::classes::CodeHighlighter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CodeHighlighter` enums](https://docs.godotengine.org/en/stable/classes/class_codehighlighter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CodeHighlighter`.\n\nInherits [`SyntaxHighlighter`][crate::classes::SyntaxHighlighter].\n\nRelated symbols:\n\n* [`code_highlighter`][crate::classes::code_highlighter]: sidecar module with related enum/flag types\n* [`ICodeHighlighter`][crate::classes::ICodeHighlighter]: virtual methods\n\n\nSee also [Godot docs for `CodeHighlighter`](https://docs.godotengine.org/en/stable/classes/class_codehighlighter.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`CodeHighlighter::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nBy adjusting various properties of this resource, you can change the colors of strings, comments, numbers, and other text patterns inside a [`TextEdit`][crate::classes::TextEdit] control."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CodeHighlighter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CodeHighlighter`][crate::classes::CodeHighlighter].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ISyntaxHighlighter`][crate::classes::ISyntaxHighlighter] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `CodeHighlighter` methods](https://docs.godotengine.org/en/stable/classes/class_codehighlighter.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICodeHighlighter: crate::obj::GodotClass < Base = CodeHighlighter > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn on_get(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn on_set(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `on_get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        fn on_validate_property(&self, property: &mut crate::registry::info::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        #[cfg_attr(published_docs, doc(cfg(since_api = "4.3")))]
        fn on_get_property_list(&mut self) -> Vec < crate::registry::info::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn on_property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = "Virtual method which can be overridden to return syntax highlighting data.\n\nSee [`get_line_syntax_highlighting`][`crate::classes::SyntaxHighlighter::get_line_syntax_highlighting`] for more details."]
        fn get_line_syntax_highlighting(&self, line: i32,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "Virtual method which can be overridden to clear any local caches."]
        fn clear_highlighting_cache(&mut self,) {
            unimplemented !()
        }
        #[doc = "Virtual method which can be overridden to update any local caches."]
        fn update_cache(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to customize the newly duplicated resource created from [`instantiate`][`crate::classes::PackedScene::instantiate`], if the original's \\[member resource_local_to_scene] is set to `true`.\n\n**Example:** Set a random `damage` value to every local resource from an instantiated scene:\n\n```gdscript\nextends Resource\n\nvar damage = 0\n\nfunc _setup_local_to_scene():\n\tdamage = randi_range(10, 40)\n```"]
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to return a custom [`RID`][crate::builtin::Rid] when [`get_rid`][`crate::classes::Resource::get_rid`] is called."]
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        #[doc = "For resources that store state in non-exported properties, such as via [`on_validate_property`][`crate::classes::IObject::on_validate_property`] or [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`], this method must be implemented to clear them."]
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to execute additional logic after [`set_path_cache`][`crate::classes::Resource::set_path_cache`] is called on this object."]
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl CodeHighlighter {
        #[doc = "Sets the color for a keyword.\n\nThe keyword cannot contain any symbols except '_'."]
        pub fn add_keyword_color(&mut self, keyword: impl AsArg < GString >, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, Color,);
            let args = (keyword.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "add_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the keyword."]
        pub fn remove_keyword_color(&mut self, keyword: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (keyword.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "remove_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the keyword exists, else `false`."]
        pub fn has_keyword_color(&self, keyword: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (keyword.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "has_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the color for a keyword."]
        pub fn get_keyword_color(&self, keyword: impl AsArg < GString >,) -> Color {
            type CallRet = Color;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (keyword.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keyword_colors(&mut self, keywords: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(keywords),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_keyword_colors", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all keywords."]
        pub fn clear_keyword_colors(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "clear_keyword_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keyword_colors(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_keyword_colors", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the color for a member keyword.\n\nThe member keyword cannot contain any symbols except '_'.\n\nIt will not be highlighted if preceded by a '.'."]
        pub fn add_member_keyword_color(&mut self, member_keyword: impl AsArg < GString >, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, Color,);
            let args = (member_keyword.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "add_member_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the member keyword."]
        pub fn remove_member_keyword_color(&mut self, member_keyword: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (member_keyword.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "remove_member_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the member keyword exists, else `false`."]
        pub fn has_member_keyword_color(&self, member_keyword: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (member_keyword.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "has_member_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the color for a member keyword."]
        pub fn get_member_keyword_color(&self, member_keyword: impl AsArg < GString >,) -> Color {
            type CallRet = Color;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (member_keyword.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_member_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_member_keyword_colors(&mut self, member_keyword: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(member_keyword),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_member_keyword_colors", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all member keywords."]
        pub fn clear_member_keyword_colors(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "clear_member_keyword_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_member_keyword_colors(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_member_keyword_colors", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a color region (such as for comments or strings) from `start_key` to `end_key`. Both keys should be symbols, and `start_key` must not be shared with other delimiters.\n\nIf `line_only` is `true` or `end_key` is an empty [`String`][crate::builtin::GString], the region does not carry over to the next line."]
        pub(crate) fn add_color_region_full(&mut self, start_key: CowArg < GString >, end_key: CowArg < GString >, color: Color, line_only: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, Color, bool,);
            let args = (start_key, end_key, color, line_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "add_color_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_color_region_ex`][Self::add_color_region_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a color region (such as for comments or strings) from `start_key` to `end_key`. Both keys should be symbols, and `start_key` must not be shared with other delimiters.\n\nIf `line_only` is `true` or `end_key` is an empty [`String`][crate::builtin::GString], the region does not carry over to the next line."]
        #[inline]
        pub fn add_color_region(&mut self, start_key: impl AsArg < GString >, end_key: impl AsArg < GString >, color: Color,) {
            self.add_color_region_ex(start_key, end_key, color,) . done()
        }
        #[doc = "Adds a color region (such as for comments or strings) from `start_key` to `end_key`. Both keys should be symbols, and `start_key` must not be shared with other delimiters.\n\nIf `line_only` is `true` or `end_key` is an empty [`String`][crate::builtin::GString], the region does not carry over to the next line."]
        #[inline]
        pub fn add_color_region_ex < 'ex > (&'ex mut self, start_key: impl AsArg < GString > + 'ex, end_key: impl AsArg < GString > + 'ex, color: Color,) -> ExAddColorRegion < 'ex > {
            ExAddColorRegion::new(self, start_key, end_key, color,)
        }
        #[doc = "Removes the color region that uses that start key."]
        pub fn remove_color_region(&mut self, start_key: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "remove_color_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the start key exists, else `false`."]
        pub fn has_color_region(&self, start_key: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "has_color_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color_regions(&mut self, color_regions: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(color_regions),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_color_regions", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all color regions."]
        pub fn clear_color_regions(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "clear_color_regions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_regions(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_color_regions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_function_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_function_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_function_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_function_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_number_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_number_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_number_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_number_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_symbol_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_symbol_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_symbol_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_symbol_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_member_variable_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_member_variable_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_member_variable_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_member_variable_color", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CodeHighlighter {
        type Base = crate::classes::SyntaxHighlighter;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CodeHighlighter"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CodeHighlighter {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::SyntaxHighlighter > for CodeHighlighter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for CodeHighlighter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for CodeHighlighter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CodeHighlighter {
        
    }
    impl crate::obj::cap::GodotDefault for CodeHighlighter {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CodeHighlighter {
        type Target = crate::classes::SyntaxHighlighter;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CodeHighlighter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CodeHighlighter`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CodeHighlighter__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CodeHighlighter > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::SyntaxHighlighter > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`CodeHighlighter::add_color_region_ex`][super::CodeHighlighter::add_color_region_ex]."]
#[must_use]
pub struct ExAddColorRegion < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CodeHighlighter, start_key: CowArg < 'ex, GString >, end_key: CowArg < 'ex, GString >, color: Color, line_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddColorRegion < 'ex > {
    fn new(surround_object: &'ex mut re_export::CodeHighlighter, start_key: impl AsArg < GString > + 'ex, end_key: impl AsArg < GString > + 'ex, color: Color,) -> Self {
        let line_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start_key: start_key.into_arg(), end_key: end_key.into_arg(), color: color, line_only: line_only,
        }
    }
    #[inline]
    pub fn line_only(self, line_only: bool) -> Self {
        Self {
            line_only: line_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start_key, end_key, color, line_only,
        }
        = self;
        re_export::CodeHighlighter::add_color_region_full(surround_object, start_key, end_key, color, line_only,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CodeHighlighter;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for CodeHighlighter {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
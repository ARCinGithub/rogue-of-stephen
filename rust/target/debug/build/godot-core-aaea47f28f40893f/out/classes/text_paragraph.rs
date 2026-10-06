#![doc = "Sidecar module for class [`TextParagraph`][crate::classes::TextParagraph].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextParagraph` enums](https://docs.godotengine.org/en/stable/classes/class_textparagraph.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextParagraph`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`text_paragraph`][crate::classes::text_paragraph]: sidecar module with related enum/flag types\n* [`ITextParagraph`][crate::classes::ITextParagraph]: virtual methods\n\n\nSee also [Godot docs for `TextParagraph`](https://docs.godotengine.org/en/stable/classes/class_textparagraph.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TextParagraph::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nAbstraction over [`TextServer`][crate::classes::TextServer] for handling a single paragraph of text."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextParagraph {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TextParagraph`][crate::classes::TextParagraph].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `TextParagraph` methods](https://docs.godotengine.org/en/stable/classes/class_textparagraph.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextParagraph: crate::obj::GodotClass < Base = TextParagraph > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl TextParagraph {
        #[doc = "Clears text paragraph (removes text and inline objects)."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8020usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Duplicates this `TextParagraph`."]
        pub fn duplicate(&self,) -> Option < Gd < crate::classes::TextParagraph > > {
            type CallRet = Option < Gd < crate::classes::TextParagraph > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8021usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "duplicate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_direction(&mut self, direction: crate::classes::text_server::Direction,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::Direction,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8022usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_direction(&self,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8023usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text writing direction inferred by the BiDi algorithm."]
        pub fn get_inferred_direction(&self,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8024usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_inferred_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_punctuation(&mut self, custom_punctuation: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (custom_punctuation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_custom_punctuation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_punctuation(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_custom_punctuation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_orientation(&mut self, orientation: crate::classes::text_server::Orientation,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::Orientation,);
            let args = (orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_orientation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_orientation(&self,) -> crate::classes::text_server::Orientation {
            type CallRet = crate::classes::text_server::Orientation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_orientation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_preserve_invalid(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_preserve_invalid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_preserve_invalid(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_preserve_invalid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_preserve_control(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_preserve_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_preserve_control(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_preserve_control", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Overrides BiDi for the structured text.\n\nOverride ranges should cover full source text without overlaps. BiDi algorithm will be used on each range separately."]
        pub fn set_bidi_override(&mut self, override_: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
            let args = (RefArg::new(override_),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets drop cap, overrides previously set drop cap. Drop cap (dropped capital) is a decorative element at the beginning of a paragraph that is larger than the rest of the text."]
        pub(crate) fn set_dropcap_full(&mut self, text: CowArg < GString >, font: CowArg < Option < Gd < crate::classes::Font > > >, font_size: i32, dropcap_margins: Rect2, language: CowArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Font > > >, i32, Rect2, CowArg < 'a2, GString >,);
            let args = (text, font, font_size, dropcap_margins, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_dropcap", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_dropcap_ex`][Self::set_dropcap_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets drop cap, overrides previously set drop cap. Drop cap (dropped capital) is a decorative element at the beginning of a paragraph that is larger than the rest of the text."]
        #[inline]
        pub fn set_dropcap(&mut self, text: impl AsArg < GString >, font: impl AsArg < Option < Gd < crate::classes::Font >> >, font_size: i32,) -> bool {
            self.set_dropcap_ex(text, font, font_size,) . done()
        }
        #[doc = "Sets drop cap, overrides previously set drop cap. Drop cap (dropped capital) is a decorative element at the beginning of a paragraph that is larger than the rest of the text."]
        #[inline]
        pub fn set_dropcap_ex < 'ex > (&'ex mut self, text: impl AsArg < GString > + 'ex, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'ex, font_size: i32,) -> ExSetDropcap < 'ex > {
            ExSetDropcap::new(self, text, font, font_size,)
        }
        #[doc = "Removes dropcap."]
        pub fn clear_dropcap(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "clear_dropcap", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds text span and font to draw it."]
        pub(crate) fn add_string_full(&mut self, text: CowArg < GString >, font: CowArg < Option < Gd < crate::classes::Font > > >, font_size: i32, language: CowArg < GString >, meta: RefArg < Variant >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Font > > >, i32, CowArg < 'a2, GString >, RefArg < 'a3, Variant >,);
            let args = (text, font, font_size, language, meta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "add_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_string_ex`][Self::add_string_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds text span and font to draw it."]
        #[inline]
        pub fn add_string(&mut self, text: impl AsArg < GString >, font: impl AsArg < Option < Gd < crate::classes::Font >> >, font_size: i32,) -> bool {
            self.add_string_ex(text, font, font_size,) . done()
        }
        #[doc = "Adds text span and font to draw it."]
        #[inline]
        pub fn add_string_ex < 'ex > (&'ex mut self, text: impl AsArg < GString > + 'ex, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'ex, font_size: i32,) -> ExAddString < 'ex > {
            ExAddString::new(self, text, font, font_size,)
        }
        #[doc = "Adds inline object to the text buffer, `key` must be unique. In the text, object is represented as `length` object replacement characters."]
        pub(crate) fn add_object_full(&mut self, key: RefArg < Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i32, baseline: f32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, Vector2, crate::global::InlineAlignment, i32, f32,);
            let args = (key, size, inline_align, length, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "add_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_object_ex`][Self::add_object_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds inline object to the text buffer, `key` must be unique. In the text, object is represented as `length` object replacement characters."]
        #[inline]
        pub fn add_object(&mut self, key: &Variant, size: Vector2,) -> bool {
            self.add_object_ex(key, size,) . done()
        }
        #[doc = "Adds inline object to the text buffer, `key` must be unique. In the text, object is represented as `length` object replacement characters."]
        #[inline]
        pub fn add_object_ex < 'ex > (&'ex mut self, key: &'ex Variant, size: Vector2,) -> ExAddObject < 'ex > {
            ExAddObject::new(self, key, size,)
        }
        #[doc = "Sets new size and alignment of embedded object."]
        pub(crate) fn resize_object_full(&mut self, key: RefArg < Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, Vector2, crate::global::InlineAlignment, f32,);
            let args = (key, size, inline_align, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "resize_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`resize_object_ex`][Self::resize_object_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets new size and alignment of embedded object."]
        #[inline]
        pub fn resize_object(&mut self, key: &Variant, size: Vector2,) -> bool {
            self.resize_object_ex(key, size,) . done()
        }
        #[doc = "Sets new size and alignment of embedded object."]
        #[inline]
        pub fn resize_object_ex < 'ex > (&'ex mut self, key: &'ex Variant, size: Vector2,) -> ExResizeObject < 'ex > {
            ExResizeObject::new(self, key, size,)
        }
        #[doc = "Returns `true` if an object with `key` is embedded in this shaped text buffer."]
        pub fn has_object(&self, key: &Variant,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "has_object", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::HorizontalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8040usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8041usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_alignment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Aligns paragraph to the given tab-stops."]
        pub fn tab_align(&mut self, tab_stops: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat32Array >,);
            let args = (RefArg::new(tab_stops),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8042usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "tab_align", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_break_flags(&mut self, flags: crate::classes::text_server::LineBreakFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::LineBreakFlag,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8043usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_break_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_break_flags(&self,) -> crate::classes::text_server::LineBreakFlag {
            type CallRet = crate::classes::text_server::LineBreakFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8044usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_break_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_justification_flags(&mut self, flags: crate::classes::text_server::JustificationFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::JustificationFlag,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8045usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_justification_flags(&self,) -> crate::classes::text_server::JustificationFlag {
            type CallRet = crate::classes::text_server::JustificationFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8046usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::OverrunBehavior,);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8047usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::classes::text_server::OverrunBehavior {
            type CallRet = crate::classes::text_server::OverrunBehavior;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8048usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ellipsis_char(&mut self, char: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (char.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8049usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_ellipsis_char", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ellipsis_char(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8050usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_ellipsis_char", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_width(&mut self, width: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8051usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_width(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8052usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the size of the bounding box of the paragraph, without line breaks."]
        pub fn get_non_wrapped_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8053usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_non_wrapped_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the size of the bounding box of the paragraph."]
        pub fn get_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8054usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns TextServer full string buffer RID."]
        pub fn get_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8055usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns TextServer line buffer RID."]
        pub fn get_line_rid(&self, line: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8056usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_line_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns drop cap text buffer RID."]
        pub fn get_dropcap_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8057usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_dropcap_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the character range of the paragraph."]
        pub fn get_range(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8058usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of lines in the paragraph."]
        pub fn get_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8059usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_line_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_lines_visible(&mut self, max_lines_visible: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_lines_visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8060usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_max_lines_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_lines_visible(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8061usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_max_lines_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_spacing(&mut self, line_spacing: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (line_spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "set_line_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_spacing(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8063usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_line_spacing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns array of inline objects in the line."]
        pub fn get_line_objects(&self, line: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8064usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_line_objects", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns bounding rectangle of the inline object."]
        pub fn get_line_object_rect(&self, line: i32, key: &Variant,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (line, RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_line_object_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns size of the bounding box of the line of text. Returned size is rounded up."]
        pub fn get_line_size(&self, line: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8066usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_line_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns character range of the line."]
        pub fn get_line_range(&self, line: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8067usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_line_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text line ascent (number of pixels above the baseline for horizontal layout or to the left of baseline for vertical)."]
        pub fn get_line_ascent(&self, line: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_line_ascent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text line descent (number of pixels below the baseline for horizontal layout or to the right of baseline for vertical)."]
        pub fn get_line_descent(&self, line: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_line_descent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns width (for horizontal layout) or height (for vertical) of the line of text."]
        pub fn get_line_width(&self, line: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_line_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns pixel offset of the underline below the baseline."]
        pub fn get_line_underline_position(&self, line: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_line_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns thickness of the underline."]
        pub fn get_line_underline_thickness(&self, line: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_line_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns drop cap bounding box size."]
        pub fn get_dropcap_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_dropcap_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of lines used by dropcap."]
        pub fn get_dropcap_lines(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "get_dropcap_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Draw all lines of the text and drop cap into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        pub(crate) fn draw_full(&self, canvas: Rid, pos: Vector2, color: Color, dc_color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Color, Color, f32,);
            let args = (canvas, pos, color, dc_color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_ex`][Self::draw_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw all lines of the text and drop cap into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw(&self, canvas: Rid, pos: Vector2,) {
            self.draw_ex(canvas, pos,) . done()
        }
        #[doc = "Draw all lines of the text and drop cap into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_ex < 'ex > (&'ex self, canvas: Rid, pos: Vector2,) -> ExDraw < 'ex > {
            ExDraw::new(self, canvas, pos,)
        }
        #[doc = "Draw outlines of all lines of the text and drop cap into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        pub(crate) fn draw_outline_full(&self, canvas: Rid, pos: Vector2, outline_size: i32, color: Color, dc_color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, i32, Color, Color, f32,);
            let args = (canvas, pos, outline_size, color, dc_color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8076usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "draw_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_outline_ex`][Self::draw_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw outlines of all lines of the text and drop cap into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_outline(&self, canvas: Rid, pos: Vector2,) {
            self.draw_outline_ex(canvas, pos,) . done()
        }
        #[doc = "Draw outlines of all lines of the text and drop cap into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_outline_ex < 'ex > (&'ex self, canvas: Rid, pos: Vector2,) -> ExDrawOutline < 'ex > {
            ExDrawOutline::new(self, canvas, pos,)
        }
        #[doc = "Draw single line of text into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        pub(crate) fn draw_line_full(&self, canvas: Rid, pos: Vector2, line: i32, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, i32, Color, f32,);
            let args = (canvas, pos, line, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8077usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "draw_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_line_ex`][Self::draw_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw single line of text into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_line(&self, canvas: Rid, pos: Vector2, line: i32,) {
            self.draw_line_ex(canvas, pos, line,) . done()
        }
        #[doc = "Draw single line of text into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_line_ex < 'ex > (&'ex self, canvas: Rid, pos: Vector2, line: i32,) -> ExDrawLine < 'ex > {
            ExDrawLine::new(self, canvas, pos, line,)
        }
        #[doc = "Draw outline of the single line of text into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        pub(crate) fn draw_line_outline_full(&self, canvas: Rid, pos: Vector2, line: i32, outline_size: i32, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, i32, i32, Color, f32,);
            let args = (canvas, pos, line, outline_size, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8078usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "draw_line_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_line_outline_ex`][Self::draw_line_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw outline of the single line of text into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_line_outline(&self, canvas: Rid, pos: Vector2, line: i32,) {
            self.draw_line_outline_ex(canvas, pos, line,) . done()
        }
        #[doc = "Draw outline of the single line of text into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_line_outline_ex < 'ex > (&'ex self, canvas: Rid, pos: Vector2, line: i32,) -> ExDrawLineOutline < 'ex > {
            ExDrawLineOutline::new(self, canvas, pos, line,)
        }
        #[doc = "Draw drop cap into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        pub(crate) fn draw_dropcap_full(&self, canvas: Rid, pos: Vector2, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Color, f32,);
            let args = (canvas, pos, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8079usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "draw_dropcap", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_dropcap_ex`][Self::draw_dropcap_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw drop cap into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_dropcap(&self, canvas: Rid, pos: Vector2,) {
            self.draw_dropcap_ex(canvas, pos,) . done()
        }
        #[doc = "Draw drop cap into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_dropcap_ex < 'ex > (&'ex self, canvas: Rid, pos: Vector2,) -> ExDrawDropcap < 'ex > {
            ExDrawDropcap::new(self, canvas, pos,)
        }
        #[doc = "Draw drop cap outline into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        pub(crate) fn draw_dropcap_outline_full(&self, canvas: Rid, pos: Vector2, outline_size: i32, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, i32, Color, f32,);
            let args = (canvas, pos, outline_size, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8080usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "draw_dropcap_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_dropcap_outline_ex`][Self::draw_dropcap_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw drop cap outline into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_dropcap_outline(&self, canvas: Rid, pos: Vector2,) {
            self.draw_dropcap_outline_ex(canvas, pos,) . done()
        }
        #[doc = "Draw drop cap outline into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_dropcap_outline_ex < 'ex > (&'ex self, canvas: Rid, pos: Vector2,) -> ExDrawDropcapOutline < 'ex > {
            ExDrawDropcapOutline::new(self, canvas, pos,)
        }
        #[doc = "Returns caret character offset at the specified coordinates. This function always returns a valid position."]
        pub fn hit_test(&self, coords: Vector2,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8081usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextParagraph", "hit_test", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TextParagraph {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextParagraph"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextParagraph {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TextParagraph {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextParagraph {
        
    }
    impl crate::obj::cap::GodotDefault for TextParagraph {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextParagraph {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextParagraph {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextParagraph`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextParagraph__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextParagraph > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TextParagraph::set_dropcap_ex`][super::TextParagraph::set_dropcap_ex]."]
#[must_use]
pub struct ExSetDropcap < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextParagraph, text: CowArg < 'ex, GString >, font: CowArg < 'ex, Option < Gd < crate::classes::Font > > >, font_size: i32, dropcap_margins: Rect2, language: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetDropcap < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextParagraph, text: impl AsArg < GString > + 'ex, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'ex, font_size: i32,) -> Self {
        let dropcap_margins = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), font: font.into_arg(), font_size: font_size, dropcap_margins: dropcap_margins, language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn dropcap_margins(self, dropcap_margins: Rect2) -> Self {
        Self {
            dropcap_margins: dropcap_margins, .. self
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'ex) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, text, font, font_size, dropcap_margins, language,
        }
        = self;
        re_export::TextParagraph::set_dropcap_full(surround_object, text, font, font_size, dropcap_margins, language,)
    }
}
#[doc = "Default-param extender for [`TextParagraph::add_string_ex`][super::TextParagraph::add_string_ex]."]
#[must_use]
pub struct ExAddString < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextParagraph, text: CowArg < 'ex, GString >, font: CowArg < 'ex, Option < Gd < crate::classes::Font > > >, font_size: i32, language: CowArg < 'ex, GString >, meta: CowArg < 'ex, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddString < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextParagraph, text: impl AsArg < GString > + 'ex, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'ex, font_size: i32,) -> Self {
        let language = GString::from("");
        let meta = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), font: font.into_arg(), font_size: font_size, language: CowArg::Owned(language), meta: CowArg::Owned(meta),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'ex) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn meta(self, meta: &'ex Variant) -> Self {
        Self {
            meta: CowArg::Borrowed(meta), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, text, font, font_size, language, meta,
        }
        = self;
        re_export::TextParagraph::add_string_full(surround_object, text, font, font_size, language, meta.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`TextParagraph::add_object_ex`][super::TextParagraph::add_object_ex]."]
#[must_use]
pub struct ExAddObject < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextParagraph, key: CowArg < 'ex, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i32, baseline: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddObject < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextParagraph, key: &'ex Variant, size: Vector2,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let length = 1i32;
        let baseline = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, key: CowArg::Borrowed(key), size: size, inline_align: inline_align, length: length, baseline: baseline,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn length(self, length: i32) -> Self {
        Self {
            length: length, .. self
        }
    }
    #[inline]
    pub fn baseline(self, baseline: f32) -> Self {
        Self {
            baseline: baseline, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, key, size, inline_align, length, baseline,
        }
        = self;
        re_export::TextParagraph::add_object_full(surround_object, key.cow_as_arg(), size, inline_align, length, baseline,)
    }
}
#[doc = "Default-param extender for [`TextParagraph::resize_object_ex`][super::TextParagraph::resize_object_ex]."]
#[must_use]
pub struct ExResizeObject < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextParagraph, key: CowArg < 'ex, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExResizeObject < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextParagraph, key: &'ex Variant, size: Vector2,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let baseline = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, key: CowArg::Borrowed(key), size: size, inline_align: inline_align, baseline: baseline,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn baseline(self, baseline: f32) -> Self {
        Self {
            baseline: baseline, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, key, size, inline_align, baseline,
        }
        = self;
        re_export::TextParagraph::resize_object_full(surround_object, key.cow_as_arg(), size, inline_align, baseline,)
    }
}
#[doc = "Default-param extender for [`TextParagraph::draw_ex`][super::TextParagraph::draw_ex]."]
#[must_use]
pub struct ExDraw < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextParagraph, canvas: Rid, pos: Vector2, color: Color, dc_color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDraw < 'ex > {
    fn new(surround_object: &'ex re_export::TextParagraph, canvas: Rid, pos: Vector2,) -> Self {
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let dc_color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas: canvas, pos: pos, color: color, dc_color: dc_color, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn dc_color(self, dc_color: Color) -> Self {
        Self {
            dc_color: dc_color, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas, pos, color, dc_color, oversampling,
        }
        = self;
        re_export::TextParagraph::draw_full(surround_object, canvas, pos, color, dc_color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextParagraph::draw_outline_ex`][super::TextParagraph::draw_outline_ex]."]
#[must_use]
pub struct ExDrawOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextParagraph, canvas: Rid, pos: Vector2, outline_size: i32, color: Color, dc_color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawOutline < 'ex > {
    fn new(surround_object: &'ex re_export::TextParagraph, canvas: Rid, pos: Vector2,) -> Self {
        let outline_size = 1i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let dc_color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas: canvas, pos: pos, outline_size: outline_size, color: color, dc_color: dc_color, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i32) -> Self {
        Self {
            outline_size: outline_size, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn dc_color(self, dc_color: Color) -> Self {
        Self {
            dc_color: dc_color, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas, pos, outline_size, color, dc_color, oversampling,
        }
        = self;
        re_export::TextParagraph::draw_outline_full(surround_object, canvas, pos, outline_size, color, dc_color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextParagraph::draw_line_ex`][super::TextParagraph::draw_line_ex]."]
#[must_use]
pub struct ExDrawLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextParagraph, canvas: Rid, pos: Vector2, line: i32, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawLine < 'ex > {
    fn new(surround_object: &'ex re_export::TextParagraph, canvas: Rid, pos: Vector2, line: i32,) -> Self {
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas: canvas, pos: pos, line: line, color: color, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas, pos, line, color, oversampling,
        }
        = self;
        re_export::TextParagraph::draw_line_full(surround_object, canvas, pos, line, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextParagraph::draw_line_outline_ex`][super::TextParagraph::draw_line_outline_ex]."]
#[must_use]
pub struct ExDrawLineOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextParagraph, canvas: Rid, pos: Vector2, line: i32, outline_size: i32, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawLineOutline < 'ex > {
    fn new(surround_object: &'ex re_export::TextParagraph, canvas: Rid, pos: Vector2, line: i32,) -> Self {
        let outline_size = 1i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas: canvas, pos: pos, line: line, outline_size: outline_size, color: color, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i32) -> Self {
        Self {
            outline_size: outline_size, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas, pos, line, outline_size, color, oversampling,
        }
        = self;
        re_export::TextParagraph::draw_line_outline_full(surround_object, canvas, pos, line, outline_size, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextParagraph::draw_dropcap_ex`][super::TextParagraph::draw_dropcap_ex]."]
#[must_use]
pub struct ExDrawDropcap < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextParagraph, canvas: Rid, pos: Vector2, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawDropcap < 'ex > {
    fn new(surround_object: &'ex re_export::TextParagraph, canvas: Rid, pos: Vector2,) -> Self {
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas: canvas, pos: pos, color: color, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas, pos, color, oversampling,
        }
        = self;
        re_export::TextParagraph::draw_dropcap_full(surround_object, canvas, pos, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextParagraph::draw_dropcap_outline_ex`][super::TextParagraph::draw_dropcap_outline_ex]."]
#[must_use]
pub struct ExDrawDropcapOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextParagraph, canvas: Rid, pos: Vector2, outline_size: i32, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawDropcapOutline < 'ex > {
    fn new(surround_object: &'ex re_export::TextParagraph, canvas: Rid, pos: Vector2,) -> Self {
        let outline_size = 1i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas: canvas, pos: pos, outline_size: outline_size, color: color, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i32) -> Self {
        Self {
            outline_size: outline_size, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas, pos, outline_size, color, oversampling,
        }
        = self;
        re_export::TextParagraph::draw_dropcap_outline_full(surround_object, canvas, pos, outline_size, color, oversampling,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TextParagraph;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TextParagraph {
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
#![doc = "Sidecar module for class [`TextLine`][crate::classes::TextLine].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextLine` enums](https://docs.godotengine.org/en/stable/classes/class_textline.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextLine`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`text_line`][crate::classes::text_line]: sidecar module with related enum/flag types\n* [`ITextLine`][crate::classes::ITextLine]: virtual methods\n\n\nSee also [Godot docs for `TextLine`](https://docs.godotengine.org/en/stable/classes/class_textline.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TextLine::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nAbstraction over [`TextServer`][crate::classes::TextServer] for handling a single line of text."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextLine {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TextLine`][crate::classes::TextLine].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `TextLine` methods](https://docs.godotengine.org/en/stable/classes/class_textline.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextLine: crate::obj::GodotClass < Base = TextLine > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TextLine {
        #[doc = "Clears text line (removes text and inline objects)."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8082usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Duplicates this `TextLine`."]
        pub fn duplicate(&self,) -> Option < Gd < crate::classes::TextLine > > {
            type CallRet = Option < Gd < crate::classes::TextLine > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8083usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "duplicate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_direction(&mut self, direction: crate::classes::text_server::Direction,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::Direction,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8084usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_direction(&self,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8085usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text writing direction inferred by the BiDi algorithm."]
        pub fn get_inferred_direction(&self,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8086usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_inferred_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_orientation(&mut self, orientation: crate::classes::text_server::Orientation,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::Orientation,);
            let args = (orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8087usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_orientation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_orientation(&self,) -> crate::classes::text_server::Orientation {
            type CallRet = crate::classes::text_server::Orientation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8088usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_orientation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_preserve_invalid(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8089usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_preserve_invalid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_preserve_invalid(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_preserve_invalid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_preserve_control(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_preserve_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_preserve_control(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_preserve_control", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Overrides BiDi for the structured text.\n\nOverride ranges should cover full source text without overlaps. BiDi algorithm will be used on each range separately."]
        pub fn set_bidi_override(&mut self, override_: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
            let args = (RefArg::new(override_),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds text span and font to draw it."]
        pub(crate) fn add_string_full(&mut self, text: CowArg < GString >, font: CowArg < Option < Gd < crate::classes::Font > > >, font_size: i32, language: CowArg < GString >, meta: RefArg < Variant >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Font > > >, i32, CowArg < 'a2, GString >, RefArg < 'a3, Variant >,);
            let args = (text, font, font_size, language, meta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "add_string", Some(self.__validated_obj()), args,)
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
                let method_bind = sys::class_scene_api() . fptr_by_index(8095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "add_object", Some(self.__validated_obj()), args,)
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
                let method_bind = sys::class_scene_api() . fptr_by_index(8096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "resize_object", Some(self.__validated_obj()), args,)
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
        #[doc = "Returns `true` if an object with `key` is embedded in this line."]
        pub fn has_object(&self, key: &Variant,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8097usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "has_object", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_width(&mut self, width: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8098usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_width(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8099usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::HorizontalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Aligns text to the given tab-stops."]
        pub fn tab_align(&mut self, tab_stops: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat32Array >,);
            let args = (RefArg::new(tab_stops),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "tab_align", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flags(&mut self, flags: crate::classes::text_server::JustificationFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::JustificationFlag,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flags(&self,) -> crate::classes::text_server::JustificationFlag {
            type CallRet = crate::classes::text_server::JustificationFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::OverrunBehavior,);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::classes::text_server::OverrunBehavior {
            type CallRet = crate::classes::text_server::OverrunBehavior;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ellipsis_char(&mut self, char: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (char.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_ellipsis_char", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ellipsis_char(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_ellipsis_char", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns array of inline objects."]
        pub fn get_objects(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_objects", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns bounding rectangle of the inline object."]
        pub fn get_object_rect(&self, key: &Variant,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_object_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns size of the bounding box of the text."]
        pub fn get_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns TextServer buffer RID."]
        pub fn get_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text ascent (number of pixels above the baseline for horizontal layout or to the left of baseline for vertical)."]
        pub fn get_line_ascent(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_line_ascent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text descent (number of pixels below the baseline for horizontal layout or to the right of baseline for vertical)."]
        pub fn get_line_descent(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_line_descent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns width (for horizontal layout) or height (for vertical) of the text."]
        pub fn get_line_width(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_line_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns pixel offset of the underline below the baseline."]
        pub fn get_line_underline_position(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_line_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns thickness of the underline."]
        pub fn get_line_underline_thickness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_line_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Draw text into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        pub(crate) fn draw_full(&self, canvas: Rid, pos: Vector2, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Color, f32,);
            let args = (canvas, pos, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_ex`][Self::draw_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw text into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw(&self, canvas: Rid, pos: Vector2,) {
            self.draw_ex(canvas, pos,) . done()
        }
        #[doc = "Draw text into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_ex < 'ex > (&'ex self, canvas: Rid, pos: Vector2,) -> ExDraw < 'ex > {
            ExDraw::new(self, canvas, pos,)
        }
        #[doc = "Draw text into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        pub(crate) fn draw_outline_full(&self, canvas: Rid, pos: Vector2, outline_size: i32, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, i32, Color, f32,);
            let args = (canvas, pos, outline_size, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "draw_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_outline_ex`][Self::draw_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw text into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_outline(&self, canvas: Rid, pos: Vector2,) {
            self.draw_outline_ex(canvas, pos,) . done()
        }
        #[doc = "Draw text into a canvas item at a given position, with `color`. `pos` specifies the top left corner of the bounding box. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_outline_ex < 'ex > (&'ex self, canvas: Rid, pos: Vector2,) -> ExDrawOutline < 'ex > {
            ExDrawOutline::new(self, canvas, pos,)
        }
        #[doc = "Returns caret character offset at the specified pixel offset at the baseline. This function always returns a valid position."]
        pub fn hit_test(&self, coords: f32,) -> i32 {
            type CallRet = i32;
            type CallParams = (f32,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "hit_test", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TextLine {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextLine"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextLine {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TextLine {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextLine {
        
    }
    impl crate::obj::cap::GodotDefault for TextLine {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextLine {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextLine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextLine`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextLine__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextLine > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TextLine::add_string_ex`][super::TextLine::add_string_ex]."]
#[must_use]
pub struct ExAddString < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextLine, text: CowArg < 'ex, GString >, font: CowArg < 'ex, Option < Gd < crate::classes::Font > > >, font_size: i32, language: CowArg < 'ex, GString >, meta: CowArg < 'ex, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddString < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextLine, text: impl AsArg < GString > + 'ex, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'ex, font_size: i32,) -> Self {
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
        re_export::TextLine::add_string_full(surround_object, text, font, font_size, language, meta.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`TextLine::add_object_ex`][super::TextLine::add_object_ex]."]
#[must_use]
pub struct ExAddObject < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextLine, key: CowArg < 'ex, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i32, baseline: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddObject < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextLine, key: &'ex Variant, size: Vector2,) -> Self {
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
        re_export::TextLine::add_object_full(surround_object, key.cow_as_arg(), size, inline_align, length, baseline,)
    }
}
#[doc = "Default-param extender for [`TextLine::resize_object_ex`][super::TextLine::resize_object_ex]."]
#[must_use]
pub struct ExResizeObject < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextLine, key: CowArg < 'ex, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExResizeObject < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextLine, key: &'ex Variant, size: Vector2,) -> Self {
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
        re_export::TextLine::resize_object_full(surround_object, key.cow_as_arg(), size, inline_align, baseline,)
    }
}
#[doc = "Default-param extender for [`TextLine::draw_ex`][super::TextLine::draw_ex]."]
#[must_use]
pub struct ExDraw < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextLine, canvas: Rid, pos: Vector2, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDraw < 'ex > {
    fn new(surround_object: &'ex re_export::TextLine, canvas: Rid, pos: Vector2,) -> Self {
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
        re_export::TextLine::draw_full(surround_object, canvas, pos, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextLine::draw_outline_ex`][super::TextLine::draw_outline_ex]."]
#[must_use]
pub struct ExDrawOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextLine, canvas: Rid, pos: Vector2, outline_size: i32, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawOutline < 'ex > {
    fn new(surround_object: &'ex re_export::TextLine, canvas: Rid, pos: Vector2,) -> Self {
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
        re_export::TextLine::draw_outline_full(surround_object, canvas, pos, outline_size, color, oversampling,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TextLine;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TextLine {
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
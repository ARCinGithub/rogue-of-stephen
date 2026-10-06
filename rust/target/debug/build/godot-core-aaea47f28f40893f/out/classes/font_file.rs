#![doc = "Sidecar module for class [`FontFile`][crate::classes::FontFile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FontFile` enums](https://docs.godotengine.org/en/stable/classes/class_fontfile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FontFile`.\n\nInherits [`Font`][crate::classes::Font].\n\nRelated symbols:\n\n* [`IFontFile`][crate::classes::IFontFile]: virtual methods\n\n\nSee also [Godot docs for `FontFile`](https://docs.godotengine.org/en/stable/classes/class_fontfile.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`FontFile::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`FontFile` contains a set of glyphs to represent Unicode characters imported from a font file, as well as a cache of rasterized glyphs, and a set of fallback [`Font`][crate::classes::Font]s to use.\n\nUse [`FontVariation`][crate::classes::FontVariation] to access specific OpenType variation of the font, create simulated bold / slanted version, and draw lines of text.\n\nFor more complex text processing, use [`FontVariation`][crate::classes::FontVariation] in conjunction with [`TextLine`][crate::classes::TextLine] or [`TextParagraph`][crate::classes::TextParagraph].\n\nSupported font formats:\n\n- Dynamic font importer: TrueType (.ttf), TrueType collection (.ttc), OpenType (.otf), OpenType collection (.otc), WOFF (.woff), WOFF2 (.woff2), Type 1 (.pfb, .pfm).\n\n- Bitmap font importer: AngelCode BMFont (.fnt, .font), text and binary (version 3) format variants.\n\n- Monospace image font importer: All supported image formats.\n\n**Note:** A character is a symbol that represents an item (letter, digit etc.) in an abstract way.\n\n**Note:** A glyph is a bitmap or a shape used to draw one or more characters in a context-dependent manner. Glyph indices are bound to the specific font data source.\n\n**Note:** If none of the font data sources contain glyphs for a character used in a string, the character in question will be replaced with a box displaying its hexadecimal code.\n\n\n```gdscript\nvar f = load(\"res://BarlowCondensed-Bold.ttf\")\n$Label.add_theme_font_override(\"font\", f)\n$Label.add_theme_font_size_override(\"font_size\", 64)\n```\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FontFile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`FontFile`][crate::classes::FontFile].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IFont`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `FontFile` methods](https://docs.godotengine.org/en/stable/classes/class_fontfile.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFontFile: crate::obj::GodotClass < Base = FontFile > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl FontFile {
        #[doc = "Loads an AngelCode BMFont (.fnt, .font) bitmap font from file `path`.\n\n**Warning:** This method should only be used in the editor or in cases when you need to load external fonts at run-time, such as fonts located at the `user://` directory."]
        pub fn load_bitmap_font(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "load_bitmap_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads a TrueType (.ttf), OpenType (.otf), WOFF (.woff), WOFF2 (.woff2) or Type 1 (.pfb, .pfm) dynamic font from file `path`.\n\n**Warning:** This method should only be used in the editor or in cases when you need to load external fonts at run-time, such as fonts located at the `user://` directory."]
        pub fn load_dynamic_font(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "load_dynamic_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_data(&mut self, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_data(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4734usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_name(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4735usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_font_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_style_name(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4736usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_font_style_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_style(&mut self, style: crate::classes::text_server::FontStyle,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::FontStyle,);
            let args = (style,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4737usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_font_style", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_weight(&mut self, weight: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4738usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_font_weight", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_stretch(&mut self, stretch: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (stretch,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4739usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_font_stretch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_antialiasing(&mut self, antialiasing: crate::classes::text_server::FontAntialiasing,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::FontAntialiasing,);
            let args = (antialiasing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4740usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_antialiasing(&self,) -> crate::classes::text_server::FontAntialiasing {
            type CallRet = crate::classes::text_server::FontAntialiasing;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_embedded_bitmaps(&mut self, disable_embedded_bitmaps: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disable_embedded_bitmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_disable_embedded_bitmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_disable_embedded_bitmaps(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4743usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_disable_embedded_bitmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_generate_mipmaps(&mut self, generate_mipmaps: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (generate_mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4744usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_generate_mipmaps(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4745usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_multichannel_signed_distance_field(&mut self, msdf: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (msdf,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4746usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_multichannel_signed_distance_field", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_multichannel_signed_distance_field(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4747usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "is_multichannel_signed_distance_field", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_msdf_pixel_range(&mut self, msdf_pixel_range: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (msdf_pixel_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4748usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_msdf_pixel_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_msdf_pixel_range(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4749usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_msdf_pixel_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_msdf_size(&mut self, msdf_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (msdf_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4750usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_msdf_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_msdf_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4751usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_msdf_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fixed_size(&mut self, fixed_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (fixed_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4752usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_fixed_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fixed_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_fixed_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fixed_size_scale_mode(&mut self, fixed_size_scale_mode: crate::classes::text_server::FixedSizeScaleMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::FixedSizeScaleMode,);
            let args = (fixed_size_scale_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_fixed_size_scale_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fixed_size_scale_mode(&self,) -> crate::classes::text_server::FixedSizeScaleMode {
            type CallRet = crate::classes::text_server::FixedSizeScaleMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_fixed_size_scale_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_system_fallback(&mut self, allow_system_fallback: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow_system_fallback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_allow_system_fallback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_allow_system_fallback(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "is_allow_system_fallback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_force_autohinter(&mut self, force_autohinter: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (force_autohinter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_force_autohinter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_force_autohinter(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "is_force_autohinter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_modulate_color_glyphs(&mut self, modulate: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_modulate_color_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_modulate_color_glyphs(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "is_modulate_color_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hinting(&mut self, hinting: crate::classes::text_server::Hinting,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::Hinting,);
            let args = (hinting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_hinting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hinting(&self,) -> crate::classes::text_server::Hinting {
            type CallRet = crate::classes::text_server::Hinting;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_hinting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_subpixel_positioning(&mut self, subpixel_positioning: crate::classes::text_server::SubpixelPositioning,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::SubpixelPositioning,);
            let args = (subpixel_positioning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_subpixel_positioning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_subpixel_positioning(&self,) -> crate::classes::text_server::SubpixelPositioning {
            type CallRet = crate::classes::text_server::SubpixelPositioning;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_subpixel_positioning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keep_rounding_remainders(&mut self, keep_rounding_remainders: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (keep_rounding_remainders,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_keep_rounding_remainders", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keep_rounding_remainders(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_keep_rounding_remainders", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_oversampling(&mut self, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_oversampling(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of the font cache entries."]
        pub fn get_cache_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all font cache entries."]
        pub fn clear_cache(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "clear_cache", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes specified font cache entry."]
        pub fn remove_cache(&mut self, cache_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_cache", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns list of the font sizes in the cache. Each size is [`Vector2i`][crate::builtin::Vector2i] with font size and outline size."]
        pub fn get_size_cache_list(&self, cache_index: i32,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_size_cache_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all font sizes from the cache entry."]
        pub fn clear_size_cache(&mut self, cache_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "clear_size_cache", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes specified font size from the cache entry."]
        pub fn remove_size_cache(&mut self, cache_index: i32, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_size_cache", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets variation coordinates for the specified font cache entry. See [`get_supported_variation_list`][`crate::classes::Font::get_supported_variation_list`] for more info."]
        pub fn set_variation_coordinates(&mut self, cache_index: i32, variation_coordinates: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, AnyDictionary >,);
            let args = (cache_index, RefArg::new(variation_coordinates),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_variation_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns variation coordinates for the specified font cache entry. See [`get_supported_variation_list`][`crate::classes::Font::get_supported_variation_list`] for more info."]
        pub fn get_variation_coordinates(&self, cache_index: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_variation_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets embolden strength, if is not equal to zero, emboldens the font outlines. Negative values reduce the outline thickness."]
        pub fn set_embolden(&mut self, cache_index: i32, strength: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (cache_index, strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_embolden", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns embolden strength, if is not equal to zero, emboldens the font outlines. Negative values reduce the outline thickness."]
        pub fn get_embolden(&self, cache_index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_embolden", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets 2D transform, applied to the font outlines, can be used for slanting, flipping, and rotating glyphs."]
        pub fn set_transform(&mut self, cache_index: i32, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (i32, Transform2D,);
            let args = (cache_index, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns 2D transform, applied to the font outlines, can be used for slanting, flipping and rotating glyphs."]
        pub fn get_transform(&self, cache_index: i32,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the spacing for `spacing` to `value` in pixels (not relative to the font size)."]
        pub fn set_extra_spacing(&mut self, cache_index: i32, spacing: crate::classes::text_server::SpacingType, value: i64,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::text_server::SpacingType, i64,);
            let args = (cache_index, spacing, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_extra_spacing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns spacing for `spacing` in pixels (not relative to the font size)."]
        pub fn get_extra_spacing(&self, cache_index: i32, spacing: crate::classes::text_server::SpacingType,) -> i64 {
            type CallRet = i64;
            type CallParams = (i32, crate::classes::text_server::SpacingType,);
            let args = (cache_index, spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_extra_spacing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets extra baseline offset (as a fraction of font height)."]
        pub fn set_extra_baseline_offset(&mut self, cache_index: i32, baseline_offset: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (cache_index, baseline_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_extra_baseline_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns extra baseline offset (as a fraction of font height)."]
        pub fn get_extra_baseline_offset(&self, cache_index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_extra_baseline_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets an active face index in the TrueType / OpenType collection."]
        pub fn set_face_index(&mut self, cache_index: i32, face_index: i64,) {
            type CallRet = ();
            type CallParams = (i32, i64,);
            let args = (cache_index, face_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_face_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an active face index in the TrueType / OpenType collection."]
        pub fn get_face_index(&self, cache_index: i32,) -> i64 {
            type CallRet = i64;
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_face_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the font ascent (number of pixels above the baseline)."]
        pub fn set_cache_ascent(&mut self, cache_index: i32, size: i32, ascent: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (cache_index, size, ascent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_ascent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the font ascent (number of pixels above the baseline)."]
        pub fn get_cache_ascent(&self, cache_index: i32, size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_ascent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the font descent (number of pixels below the baseline)."]
        pub fn set_cache_descent(&mut self, cache_index: i32, size: i32, descent: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (cache_index, size, descent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_descent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the font descent (number of pixels below the baseline)."]
        pub fn get_cache_descent(&self, cache_index: i32, size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_descent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets pixel offset of the underline below the baseline."]
        pub fn set_cache_underline_position(&mut self, cache_index: i32, size: i32, underline_position: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (cache_index, size, underline_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns pixel offset of the underline below the baseline."]
        pub fn get_cache_underline_position(&self, cache_index: i32, size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets thickness of the underline in pixels."]
        pub fn set_cache_underline_thickness(&mut self, cache_index: i32, size: i32, underline_thickness: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (cache_index, size, underline_thickness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns thickness of the underline in pixels."]
        pub fn get_cache_underline_thickness(&self, cache_index: i32, size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets scaling factor of the color bitmap font."]
        pub fn set_cache_scale(&mut self, cache_index: i32, size: i32, scale: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (cache_index, size, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns scaling factor of the color bitmap font."]
        pub fn get_cache_scale(&self, cache_index: i32, size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of textures used by font cache entry."]
        pub fn get_texture_count(&self, cache_index: i32, size: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, Vector2i,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_texture_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all textures from font cache entry.\n\n**Note:** This function will not remove glyphs associated with the texture, use [`remove_glyph`][`crate::classes::FontFile::remove_glyph`] to remove them manually."]
        pub fn clear_textures(&mut self, cache_index: i32, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "clear_textures", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes specified texture from the cache entry.\n\n**Note:** This function will not remove glyphs associated with the texture. Remove them manually using [`remove_glyph`][`crate::classes::FontFile::remove_glyph`]."]
        pub fn remove_texture(&mut self, cache_index: i32, size: Vector2i, texture_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets font cache texture image."]
        pub fn set_texture_image(&mut self, cache_index: i32, size: Vector2i, texture_index: i32, image: impl AsArg < Option < Gd < crate::classes::Image >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, Vector2i, i32, CowArg < 'a0, Option < Gd < crate::classes::Image > > >,);
            let args = (cache_index, size, texture_index, image.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_texture_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a copy of the font cache texture image."]
        pub fn get_texture_image(&self, cache_index: i32, size: Vector2i, texture_index: i32,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_texture_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets array containing glyph packing data."]
        pub fn set_texture_offsets(&mut self, cache_index: i32, size: Vector2i, texture_index: i32, offset: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, Vector2i, i32, RefArg < 'a0, PackedInt32Array >,);
            let args = (cache_index, size, texture_index, RefArg::new(offset),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_texture_offsets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a copy of the array containing glyph packing data."]
        pub fn get_texture_offsets(&self, cache_index: i32, size: Vector2i, texture_index: i32,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_texture_offsets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns list of rendered glyphs in the cache entry."]
        pub fn get_glyph_list(&self, cache_index: i32, size: Vector2i,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32, Vector2i,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all rendered glyph information from the cache entry.\n\n**Note:** This function will not remove textures associated with the glyphs, use [`remove_texture`][`crate::classes::FontFile::remove_texture`] to remove them manually."]
        pub fn clear_glyphs(&mut self, cache_index: i32, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4806usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "clear_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes specified rendered glyph information from the cache entry.\n\n**Note:** This function will not remove textures associated with the glyphs, use [`remove_texture`][`crate::classes::FontFile::remove_texture`] to remove them manually."]
        pub fn remove_glyph(&mut self, cache_index: i32, size: Vector2i, glyph: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4807usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_glyph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets glyph advance (offset of the next glyph).\n\n**Note:** Advance for glyphs outlines is the same as the base glyph advance and is not saved."]
        pub fn set_glyph_advance(&mut self, cache_index: i32, size: i32, glyph: i32, advance: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32, Vector2,);
            let args = (cache_index, size, glyph, advance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4808usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_advance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns glyph advance (offset of the next glyph).\n\n**Note:** Advance for glyphs outlines is the same as the base glyph advance and is not saved."]
        pub fn get_glyph_advance(&self, cache_index: i32, size: i32, glyph: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, i32, i32,);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4809usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_advance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets glyph offset from the baseline."]
        pub fn set_glyph_offset(&mut self, cache_index: i32, size: Vector2i, glyph: i32, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, Vector2,);
            let args = (cache_index, size, glyph, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns glyph offset from the baseline."]
        pub fn get_glyph_offset(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets glyph size."]
        pub fn set_glyph_size(&mut self, cache_index: i32, size: Vector2i, glyph: i32, gl_size: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, Vector2,);
            let args = (cache_index, size, glyph, gl_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4812usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns glyph size."]
        pub fn get_glyph_size(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4813usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets rectangle in the cache texture containing the glyph."]
        pub fn set_glyph_uv_rect(&mut self, cache_index: i32, size: Vector2i, glyph: i32, uv_rect: Rect2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, Rect2,);
            let args = (cache_index, size, glyph, uv_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4814usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_uv_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns rectangle in the cache texture containing the glyph."]
        pub fn get_glyph_uv_rect(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4815usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_uv_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets index of the cache texture containing the glyph."]
        pub fn set_glyph_texture_idx(&mut self, cache_index: i32, size: Vector2i, glyph: i32, texture_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, i32,);
            let args = (cache_index, size, glyph, texture_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4816usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_texture_idx", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns index of the cache texture containing the glyph."]
        pub fn get_glyph_texture_idx(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_texture_idx", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns list of the kerning overrides."]
        pub fn get_kerning_list(&self, cache_index: i32, size: i32,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_kerning_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all kerning overrides."]
        pub fn clear_kerning_map(&mut self, cache_index: i32, size: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "clear_kerning_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes kerning override for the pair of glyphs."]
        pub fn remove_kerning(&mut self, cache_index: i32, size: i32, glyph_pair: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, i32, Vector2i,);
            let args = (cache_index, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_kerning", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets kerning for the pair of glyphs."]
        pub fn set_kerning(&mut self, cache_index: i32, size: i32, glyph_pair: Vector2i, kerning: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, i32, Vector2i, Vector2,);
            let args = (cache_index, size, glyph_pair, kerning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_kerning", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns kerning for the pair of glyphs."]
        pub fn get_kerning(&self, cache_index: i32, size: i32, glyph_pair: Vector2i,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, i32, Vector2i,);
            let args = (cache_index, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_kerning", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renders the range of characters to the font cache texture."]
        pub fn render_range(&mut self, cache_index: i32, size: Vector2i, start: u32, end: u32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, u32, u32,);
            let args = (cache_index, size, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "render_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renders specified glyph to the font cache texture."]
        pub fn render_glyph(&mut self, cache_index: i32, size: Vector2i, index: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "render_glyph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds override for [`is_language_supported`][`crate::classes::Font::is_language_supported`]."]
        pub fn set_language_support_override(&mut self, language: impl AsArg < GString >, supported: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (language.into_arg(), supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_language_support_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if support override is enabled for the `language`."]
        pub fn get_language_support_override(&self, language: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_language_support_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Remove language support override."]
        pub fn remove_language_support_override(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_language_support_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns list of language support overrides."]
        pub fn get_language_support_overrides(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_language_support_overrides", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds override for [`is_script_supported`][`crate::classes::Font::is_script_supported`]."]
        pub fn set_script_support_override(&mut self, script: impl AsArg < GString >, supported: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (script.into_arg(), supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_script_support_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if support override is enabled for the `script`."]
        pub fn get_script_support_override(&self, script: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_script_support_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes script support override."]
        pub fn remove_script_support_override(&mut self, script: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_script_support_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns list of script support overrides."]
        pub fn get_script_support_overrides(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_script_support_overrides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_opentype_feature_overrides(&mut self, overrides: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(overrides),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_opentype_feature_overrides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_opentype_feature_overrides(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_opentype_feature_overrides", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the glyph index of a `char`, optionally modified by the `variation_selector`."]
        pub fn get_glyph_index(&self, size: i32, char: u32, variation_selector: u32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, u32, u32,);
            let args = (size, char, variation_selector,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns character code associated with `glyph_index`, or `0` if `glyph_index` is invalid. See [`get_glyph_index`][`crate::classes::FontFile::get_glyph_index`]."]
        pub fn get_char_from_glyph_index(&self, size: i32, glyph_index: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (i32, i32,);
            let args = (size, glyph_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_char_from_glyph_index", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for FontFile {
        type Base = crate::classes::Font;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("FontFile"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FontFile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Font > for FontFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for FontFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for FontFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FontFile {
        
    }
    impl crate::obj::cap::GodotDefault for FontFile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for FontFile {
        type Target = crate::classes::Font;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FontFile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`FontFile`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_FontFile__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::FontFile > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Font > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::FontFile;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for FontFile {
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
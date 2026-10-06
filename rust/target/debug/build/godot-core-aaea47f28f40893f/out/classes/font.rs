#![doc = "Sidecar module for class [`Font`][crate::classes::Font].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Font` enums](https://docs.godotengine.org/en/stable/classes/class_font.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Font`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`font`][crate::classes::font]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Font`](https://docs.godotengine.org/en/stable/classes/class_font.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Font>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nAbstract base class for different font types. It has methods for drawing text and font character introspection."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Font {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Font {
        pub fn set_fallbacks(&mut self, fallbacks: &Array < Gd < crate::classes::Font > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::Font > > >,);
            let args = (RefArg::new(fallbacks),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9895usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "set_fallbacks", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fallbacks(&self,) -> Array < Gd < crate::classes::Font > > {
            type CallRet = Array < Gd < crate::classes::Font > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9896usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_fallbacks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns [`TextServer`][crate::classes::TextServer] RID of the font cache for specific variation."]
        pub(crate) fn find_variation_full(&self, variation_coordinates: RefArg < AnyDictionary >, face_index: i32, strength: f32, transform: Transform2D, spacing_top: i32, spacing_bottom: i32, spacing_space: i32, spacing_glyph: i32, baseline_offset: f32,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >, i32, f32, Transform2D, i32, i32, i32, i32, f32,);
            let args = (variation_coordinates, face_index, strength, transform, spacing_top, spacing_bottom, spacing_space, spacing_glyph, baseline_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9897usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "find_variation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`find_variation_ex`][Self::find_variation_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns [`TextServer`][crate::classes::TextServer] RID of the font cache for specific variation."]
        #[inline]
        pub fn find_variation(&self, variation_coordinates: &AnyDictionary,) -> Rid {
            self.find_variation_ex(variation_coordinates,) . done()
        }
        #[doc = "Returns [`TextServer`][crate::classes::TextServer] RID of the font cache for specific variation."]
        #[inline]
        pub fn find_variation_ex < 'ex > (&'ex self, variation_coordinates: &'ex AnyDictionary,) -> ExFindVariation < 'ex > {
            ExFindVariation::new(self, variation_coordinates,)
        }
        #[doc = "Returns [`Array`][crate::builtin::Array] of valid `Font` [`RID`][crate::builtin::Rid]s, which can be passed to the [`TextServer`][crate::classes::TextServer] methods."]
        pub fn get_rids(&self,) -> Array < Rid > {
            type CallRet = Array < Rid >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9898usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_rids", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total average font height (ascent plus descent) in pixels.\n\n**Note:** Real height of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate (e.g. as the height of empty line)."]
        pub(crate) fn get_height_full(&self, font_size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9899usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_height_ex`][Self::get_height_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the total average font height (ascent plus descent) in pixels.\n\n**Note:** Real height of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate (e.g. as the height of empty line)."]
        #[inline]
        pub fn get_height(&self,) -> f32 {
            self.get_height_ex() . done()
        }
        #[doc = "Returns the total average font height (ascent plus descent) in pixels.\n\n**Note:** Real height of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate (e.g. as the height of empty line)."]
        #[inline]
        pub fn get_height_ex < 'ex > (&'ex self,) -> ExGetHeight < 'ex > {
            ExGetHeight::new(self,)
        }
        #[doc = "Returns the average font ascent (number of pixels above the baseline).\n\n**Note:** Real ascent of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate (e.g. as the ascent of empty line)."]
        pub(crate) fn get_ascent_full(&self, font_size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9900usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_ascent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_ascent_ex`][Self::get_ascent_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the average font ascent (number of pixels above the baseline).\n\n**Note:** Real ascent of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate (e.g. as the ascent of empty line)."]
        #[inline]
        pub fn get_ascent(&self,) -> f32 {
            self.get_ascent_ex() . done()
        }
        #[doc = "Returns the average font ascent (number of pixels above the baseline).\n\n**Note:** Real ascent of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate (e.g. as the ascent of empty line)."]
        #[inline]
        pub fn get_ascent_ex < 'ex > (&'ex self,) -> ExGetAscent < 'ex > {
            ExGetAscent::new(self,)
        }
        #[doc = "Returns the average font descent (number of pixels below the baseline).\n\n**Note:** Real descent of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate (e.g. as the descent of empty line)."]
        pub(crate) fn get_descent_full(&self, font_size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_descent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_descent_ex`][Self::get_descent_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the average font descent (number of pixels below the baseline).\n\n**Note:** Real descent of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate (e.g. as the descent of empty line)."]
        #[inline]
        pub fn get_descent(&self,) -> f32 {
            self.get_descent_ex() . done()
        }
        #[doc = "Returns the average font descent (number of pixels below the baseline).\n\n**Note:** Real descent of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate (e.g. as the descent of empty line)."]
        #[inline]
        pub fn get_descent_ex < 'ex > (&'ex self,) -> ExGetDescent < 'ex > {
            ExGetDescent::new(self,)
        }
        #[doc = "Returns average pixel offset of the underline below the baseline.\n\n**Note:** Real underline position of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate."]
        pub(crate) fn get_underline_position_full(&self, font_size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_underline_position_ex`][Self::get_underline_position_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns average pixel offset of the underline below the baseline.\n\n**Note:** Real underline position of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate."]
        #[inline]
        pub fn get_underline_position(&self,) -> f32 {
            self.get_underline_position_ex() . done()
        }
        #[doc = "Returns average pixel offset of the underline below the baseline.\n\n**Note:** Real underline position of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate."]
        #[inline]
        pub fn get_underline_position_ex < 'ex > (&'ex self,) -> ExGetUnderlinePosition < 'ex > {
            ExGetUnderlinePosition::new(self,)
        }
        #[doc = "Returns average thickness of the underline.\n\n**Note:** Real underline thickness of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate."]
        pub(crate) fn get_underline_thickness_full(&self, font_size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_underline_thickness_ex`][Self::get_underline_thickness_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns average thickness of the underline.\n\n**Note:** Real underline thickness of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate."]
        #[inline]
        pub fn get_underline_thickness(&self,) -> f32 {
            self.get_underline_thickness_ex() . done()
        }
        #[doc = "Returns average thickness of the underline.\n\n**Note:** Real underline thickness of the string is context-dependent and can be significantly different from the value returned by this function. Use it only as rough estimate."]
        #[inline]
        pub fn get_underline_thickness_ex < 'ex > (&'ex self,) -> ExGetUnderlineThickness < 'ex > {
            ExGetUnderlineThickness::new(self,)
        }
        #[doc = "Returns font family name."]
        pub fn get_font_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_font_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font style name."]
        pub fn get_font_style_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_font_style_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns [`Dictionary`][crate::builtin::Dictionary] with OpenType font name strings (localized font names, version, description, license information, sample text, etc.)."]
        pub fn get_ot_name_strings(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_ot_name_strings", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font style flags."]
        pub fn get_font_style(&self,) -> crate::classes::text_server::FontStyle {
            type CallRet = crate::classes::text_server::FontStyle;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_font_style", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns weight (boldness) of the font. A value in the `100...999` range, normal font weight is `400`, bold font weight is `700`."]
        pub fn get_font_weight(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_font_weight", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font stretch amount, compared to a normal width. A percentage value between `50%` and `200%`."]
        pub fn get_font_stretch(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_font_stretch", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the amount of spacing for the given `spacing` type."]
        pub fn get_spacing(&self, spacing: crate::classes::text_server::SpacingType,) -> i32 {
            type CallRet = i32;
            type CallParams = (crate::classes::text_server::SpacingType,);
            let args = (spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_spacing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a set of OpenType feature tags. More info: [OpenType feature tags](https://docs.microsoft.com/en-us/typography/opentype/spec/featuretags)."]
        pub fn get_opentype_features(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_opentype_features", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets LRU cache capacity for `draw_*` methods."]
        pub fn set_cache_capacity(&mut self, single_line: i32, multi_line: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (single_line, multi_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9912usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "set_cache_capacity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the size of a bounding box of a single-line string, taking kerning, advance and subpixel positioning into account. See also [`get_multiline_string_size`][`crate::classes::Font::get_multiline_string_size`] and [`draw_string`][`crate::classes::Font::draw_string`].\n\nFor example, to get the string size as displayed by a single-line Label, use:\n\n\n```gdscript\nvar string_size = $Label.get_theme_font(\"font\").get_string_size($Label.text, HORIZONTAL_ALIGNMENT_LEFT, -1, $Label.get_theme_font_size(\"font_size\"))\n```\n\n\n**Note:** Since kerning, advance and subpixel positioning are taken into account by [`get_string_size`][`crate::classes::Font::get_string_size`], using separate [`get_string_size`][`crate::classes::Font::get_string_size`] calls on substrings of a string then adding the results together will return a different result compared to using a single [`get_string_size`][`crate::classes::Font::get_string_size`] call on the full string.\n\n**Note:** Real height of the string is context-dependent and can be significantly different from the value returned by [`get_height`][`crate::classes::Font::get_height`]."]
        pub(crate) fn get_string_size_full(&self, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::global::HorizontalAlignment, f32, i32, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation,);
            let args = (text, alignment, width, font_size, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9913usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_string_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_string_size_ex`][Self::get_string_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the size of a bounding box of a single-line string, taking kerning, advance and subpixel positioning into account. See also [`get_multiline_string_size`][`crate::classes::Font::get_multiline_string_size`] and [`draw_string`][`crate::classes::Font::draw_string`].\n\nFor example, to get the string size as displayed by a single-line Label, use:\n\n\n```gdscript\nvar string_size = $Label.get_theme_font(\"font\").get_string_size($Label.text, HORIZONTAL_ALIGNMENT_LEFT, -1, $Label.get_theme_font_size(\"font_size\"))\n```\n\n\n**Note:** Since kerning, advance and subpixel positioning are taken into account by [`get_string_size`][`crate::classes::Font::get_string_size`], using separate [`get_string_size`][`crate::classes::Font::get_string_size`] calls on substrings of a string then adding the results together will return a different result compared to using a single [`get_string_size`][`crate::classes::Font::get_string_size`] call on the full string.\n\n**Note:** Real height of the string is context-dependent and can be significantly different from the value returned by [`get_height`][`crate::classes::Font::get_height`]."]
        #[inline]
        pub fn get_string_size(&self, text: impl AsArg < GString >,) -> Vector2 {
            self.get_string_size_ex(text,) . done()
        }
        #[doc = "Returns the size of a bounding box of a single-line string, taking kerning, advance and subpixel positioning into account. See also [`get_multiline_string_size`][`crate::classes::Font::get_multiline_string_size`] and [`draw_string`][`crate::classes::Font::draw_string`].\n\nFor example, to get the string size as displayed by a single-line Label, use:\n\n\n```gdscript\nvar string_size = $Label.get_theme_font(\"font\").get_string_size($Label.text, HORIZONTAL_ALIGNMENT_LEFT, -1, $Label.get_theme_font_size(\"font_size\"))\n```\n\n\n**Note:** Since kerning, advance and subpixel positioning are taken into account by [`get_string_size`][`crate::classes::Font::get_string_size`], using separate [`get_string_size`][`crate::classes::Font::get_string_size`] calls on substrings of a string then adding the results together will return a different result compared to using a single [`get_string_size`][`crate::classes::Font::get_string_size`] call on the full string.\n\n**Note:** Real height of the string is context-dependent and can be significantly different from the value returned by [`get_height`][`crate::classes::Font::get_height`]."]
        #[inline]
        pub fn get_string_size_ex < 'ex > (&'ex self, text: impl AsArg < GString > + 'ex,) -> ExGetStringSize < 'ex > {
            ExGetStringSize::new(self, text,)
        }
        #[doc = "Returns the size of a bounding box of a string broken into the lines, taking kerning and advance into account.\n\nSee also [`draw_multiline_string`][`crate::classes::Font::draw_multiline_string`]."]
        pub(crate) fn get_multiline_string_size_full(&self, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::global::HorizontalAlignment, f32, i32, i32, crate::classes::text_server::LineBreakFlag, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation,);
            let args = (text, alignment, width, font_size, max_lines, brk_flags, justification_flags, direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9914usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_multiline_string_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_multiline_string_size_ex`][Self::get_multiline_string_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the size of a bounding box of a string broken into the lines, taking kerning and advance into account.\n\nSee also [`draw_multiline_string`][`crate::classes::Font::draw_multiline_string`]."]
        #[inline]
        pub fn get_multiline_string_size(&self, text: impl AsArg < GString >,) -> Vector2 {
            self.get_multiline_string_size_ex(text,) . done()
        }
        #[doc = "Returns the size of a bounding box of a string broken into the lines, taking kerning and advance into account.\n\nSee also [`draw_multiline_string`][`crate::classes::Font::draw_multiline_string`]."]
        #[inline]
        pub fn get_multiline_string_size_ex < 'ex > (&'ex self, text: impl AsArg < GString > + 'ex,) -> ExGetMultilineStringSize < 'ex > {
            ExGetMultilineStringSize::new(self, text,)
        }
        #[doc = "Draw `text` into a canvas item using the font, at a given position, with `modulate` color, optionally clipping the width and aligning horizontally. `pos` specifies the baseline, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\nSee also [`draw_string`][`crate::classes::CanvasItem::draw_string`]."]
        pub(crate) fn draw_string_full(&self, canvas_item: Rid, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, Vector2, CowArg < 'a0, GString >, crate::global::HorizontalAlignment, f32, i32, Color, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation, f32,);
            let args = (canvas_item, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9915usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "draw_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_string_ex`][Self::draw_string_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw `text` into a canvas item using the font, at a given position, with `modulate` color, optionally clipping the width and aligning horizontally. `pos` specifies the baseline, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\nSee also [`draw_string`][`crate::classes::CanvasItem::draw_string`]."]
        #[inline]
        pub fn draw_string(&self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_string_ex(canvas_item, pos, text,) . done()
        }
        #[doc = "Draw `text` into a canvas item using the font, at a given position, with `modulate` color, optionally clipping the width and aligning horizontally. `pos` specifies the baseline, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\nSee also [`draw_string`][`crate::classes::CanvasItem::draw_string`]."]
        #[inline]
        pub fn draw_string_ex < 'ex > (&'ex self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> ExDrawString < 'ex > {
            ExDrawString::new(self, canvas_item, pos, text,)
        }
        #[doc = "Breaks `text` into lines using rules specified by `brk_flags` and draws it into a canvas item using the font, at a given position, with `modulate` color, optionally clipping the width and aligning horizontally. `pos` specifies the baseline of the first line, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\nSee also [`draw_multiline_string`][`crate::classes::CanvasItem::draw_multiline_string`]."]
        pub(crate) fn draw_multiline_string_full(&self, canvas_item: Rid, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, Vector2, CowArg < 'a0, GString >, crate::global::HorizontalAlignment, f32, i32, i32, Color, crate::classes::text_server::LineBreakFlag, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation, f32,);
            let args = (canvas_item, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9916usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "draw_multiline_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_multiline_string_ex`][Self::draw_multiline_string_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Breaks `text` into lines using rules specified by `brk_flags` and draws it into a canvas item using the font, at a given position, with `modulate` color, optionally clipping the width and aligning horizontally. `pos` specifies the baseline of the first line, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\nSee also [`draw_multiline_string`][`crate::classes::CanvasItem::draw_multiline_string`]."]
        #[inline]
        pub fn draw_multiline_string(&self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_multiline_string_ex(canvas_item, pos, text,) . done()
        }
        #[doc = "Breaks `text` into lines using rules specified by `brk_flags` and draws it into a canvas item using the font, at a given position, with `modulate` color, optionally clipping the width and aligning horizontally. `pos` specifies the baseline of the first line, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\nSee also [`draw_multiline_string`][`crate::classes::CanvasItem::draw_multiline_string`]."]
        #[inline]
        pub fn draw_multiline_string_ex < 'ex > (&'ex self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> ExDrawMultilineString < 'ex > {
            ExDrawMultilineString::new(self, canvas_item, pos, text,)
        }
        #[doc = "Draw `text` outline into a canvas item using the font, at a given position, with `modulate` color and `size` outline size, optionally clipping the width and aligning horizontally. `pos` specifies the baseline, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\nSee also [`draw_string_outline`][`crate::classes::CanvasItem::draw_string_outline`]."]
        pub(crate) fn draw_string_outline_full(&self, canvas_item: Rid, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, Vector2, CowArg < 'a0, GString >, crate::global::HorizontalAlignment, f32, i32, i32, Color, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation, f32,);
            let args = (canvas_item, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9917usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "draw_string_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_string_outline_ex`][Self::draw_string_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw `text` outline into a canvas item using the font, at a given position, with `modulate` color and `size` outline size, optionally clipping the width and aligning horizontally. `pos` specifies the baseline, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\nSee also [`draw_string_outline`][`crate::classes::CanvasItem::draw_string_outline`]."]
        #[inline]
        pub fn draw_string_outline(&self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_string_outline_ex(canvas_item, pos, text,) . done()
        }
        #[doc = "Draw `text` outline into a canvas item using the font, at a given position, with `modulate` color and `size` outline size, optionally clipping the width and aligning horizontally. `pos` specifies the baseline, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\nSee also [`draw_string_outline`][`crate::classes::CanvasItem::draw_string_outline`]."]
        #[inline]
        pub fn draw_string_outline_ex < 'ex > (&'ex self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> ExDrawStringOutline < 'ex > {
            ExDrawStringOutline::new(self, canvas_item, pos, text,)
        }
        #[doc = "Breaks `text` to the lines using rules specified by `brk_flags` and draws text outline into a canvas item using the font, at a given position, with `modulate` color and `size` outline size, optionally clipping the width and aligning horizontally. `pos` specifies the baseline of the first line, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\nSee also [`draw_multiline_string_outline`][`crate::classes::CanvasItem::draw_multiline_string_outline`]."]
        pub(crate) fn draw_multiline_string_outline_full(&self, canvas_item: Rid, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, size: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, Vector2, CowArg < 'a0, GString >, crate::global::HorizontalAlignment, f32, i32, i32, i32, Color, crate::classes::text_server::LineBreakFlag, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation, f32,);
            let args = (canvas_item, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9918usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "draw_multiline_string_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_multiline_string_outline_ex`][Self::draw_multiline_string_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Breaks `text` to the lines using rules specified by `brk_flags` and draws text outline into a canvas item using the font, at a given position, with `modulate` color and `size` outline size, optionally clipping the width and aligning horizontally. `pos` specifies the baseline of the first line, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\nSee also [`draw_multiline_string_outline`][`crate::classes::CanvasItem::draw_multiline_string_outline`]."]
        #[inline]
        pub fn draw_multiline_string_outline(&self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_multiline_string_outline_ex(canvas_item, pos, text,) . done()
        }
        #[doc = "Breaks `text` to the lines using rules specified by `brk_flags` and draws text outline into a canvas item using the font, at a given position, with `modulate` color and `size` outline size, optionally clipping the width and aligning horizontally. `pos` specifies the baseline of the first line, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\nSee also [`draw_multiline_string_outline`][`crate::classes::CanvasItem::draw_multiline_string_outline`]."]
        #[inline]
        pub fn draw_multiline_string_outline_ex < 'ex > (&'ex self, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> ExDrawMultilineStringOutline < 'ex > {
            ExDrawMultilineStringOutline::new(self, canvas_item, pos, text,)
        }
        #[doc = "Returns the size of a character. Does not take kerning into account.\n\n**Note:** Do not use this function to calculate width of the string character by character, use [`get_string_size`][`crate::classes::Font::get_string_size`] or [`TextLine`][crate::classes::TextLine] instead. The height returned is the font height (see also [`get_height`][`crate::classes::Font::get_height`]) and has no relation to the glyph height."]
        pub fn get_char_size(&self, char: u32, font_size: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (u32, i32,);
            let args = (char, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9919usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_char_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Draw a single Unicode character `char` into a canvas item using the font, at a given position, with `modulate` color. `pos` specifies the baseline, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Note:** Do not use this function to draw strings character by character, use [`draw_string`][`crate::classes::Font::draw_string`] or [`TextLine`][crate::classes::TextLine] instead."]
        pub(crate) fn draw_char_full(&self, canvas_item: Rid, pos: Vector2, char: u32, font_size: i32, modulate: Color, oversampling: f32,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid, Vector2, u32, i32, Color, f32,);
            let args = (canvas_item, pos, char, font_size, modulate, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9920usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "draw_char", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_char_ex`][Self::draw_char_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw a single Unicode character `char` into a canvas item using the font, at a given position, with `modulate` color. `pos` specifies the baseline, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Note:** Do not use this function to draw strings character by character, use [`draw_string`][`crate::classes::Font::draw_string`] or [`TextLine`][crate::classes::TextLine] instead."]
        #[inline]
        pub fn draw_char(&self, canvas_item: Rid, pos: Vector2, char: u32, font_size: i32,) -> f32 {
            self.draw_char_ex(canvas_item, pos, char, font_size,) . done()
        }
        #[doc = "Draw a single Unicode character `char` into a canvas item using the font, at a given position, with `modulate` color. `pos` specifies the baseline, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Note:** Do not use this function to draw strings character by character, use [`draw_string`][`crate::classes::Font::draw_string`] or [`TextLine`][crate::classes::TextLine] instead."]
        #[inline]
        pub fn draw_char_ex < 'ex > (&'ex self, canvas_item: Rid, pos: Vector2, char: u32, font_size: i32,) -> ExDrawChar < 'ex > {
            ExDrawChar::new(self, canvas_item, pos, char, font_size,)
        }
        #[doc = "Draw a single Unicode character `char` outline into a canvas item using the font, at a given position, with `modulate` color and `size` outline size. `pos` specifies the baseline, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Note:** Do not use this function to draw strings character by character, use [`draw_string`][`crate::classes::Font::draw_string`] or [`TextLine`][crate::classes::TextLine] instead."]
        pub(crate) fn draw_char_outline_full(&self, canvas_item: Rid, pos: Vector2, char: u32, font_size: i32, size: i32, modulate: Color, oversampling: f32,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid, Vector2, u32, i32, i32, Color, f32,);
            let args = (canvas_item, pos, char, font_size, size, modulate, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9921usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "draw_char_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_char_outline_ex`][Self::draw_char_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw a single Unicode character `char` outline into a canvas item using the font, at a given position, with `modulate` color and `size` outline size. `pos` specifies the baseline, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Note:** Do not use this function to draw strings character by character, use [`draw_string`][`crate::classes::Font::draw_string`] or [`TextLine`][crate::classes::TextLine] instead."]
        #[inline]
        pub fn draw_char_outline(&self, canvas_item: Rid, pos: Vector2, char: u32, font_size: i32,) -> f32 {
            self.draw_char_outline_ex(canvas_item, pos, char, font_size,) . done()
        }
        #[doc = "Draw a single Unicode character `char` outline into a canvas item using the font, at a given position, with `modulate` color and `size` outline size. `pos` specifies the baseline, not the top. To draw from the top, _ascent_ must be added to the Y axis. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Note:** Do not use this function to draw strings character by character, use [`draw_string`][`crate::classes::Font::draw_string`] or [`TextLine`][crate::classes::TextLine] instead."]
        #[inline]
        pub fn draw_char_outline_ex < 'ex > (&'ex self, canvas_item: Rid, pos: Vector2, char: u32, font_size: i32,) -> ExDrawCharOutline < 'ex > {
            ExDrawCharOutline::new(self, canvas_item, pos, char, font_size,)
        }
        #[doc = "Returns `true` if a Unicode `char` is available in the font."]
        pub fn has_char(&self, char: u32,) -> bool {
            type CallRet = bool;
            type CallParams = (u32,);
            let args = (char,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9922usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "has_char", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a string containing all the characters available in the font.\n\nIf a given character is included in more than one font data source, it appears only once in the returned string."]
        pub fn get_supported_chars(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9923usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_supported_chars", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the font supports the given language (as a [ISO 639](https://en.wikipedia.org/wiki/ISO_639-1) code)."]
        pub fn is_language_supported(&self, language: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9924usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "is_language_supported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the font supports the given script (as a [ISO 15924](https://en.wikipedia.org/wiki/ISO_15924) code)."]
        pub fn is_script_supported(&self, script: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9925usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "is_script_supported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns list of OpenType features supported by font."]
        pub fn get_supported_feature_list(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_supported_feature_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns list of supported [variation coordinates](https://docs.microsoft.com/en-us/typography/opentype/spec/dvaraxisreg), each coordinate is returned as `tag: Vector3i(min_value,max_value,default_value)`.\n\nFont variations allow for continuous change of glyph characteristics along some given design axis, such as weight, width or slant.\n\nTo print available variation axes of a variable font:\n\n```gdscript\nvar fv = FontVariation.new()\nfv.base_font = load(\"res://RobotoFlex.ttf\")\nvar variation_list = fv.get_supported_variation_list()\nfor tag in variation_list:\n\tvar name = TextServerManager.get_primary_interface().tag_to_name(tag)\n\tvar values = variation_list[tag]\n\tprint(\"variation axis: %s (%d)\\n\\tmin, max, default: %s\" % [name, tag, values])\n```\n\n**Note:** To set and get variation coordinates of a [`FontVariation`][crate::classes::FontVariation], use \\[member FontVariation.variation_opentype]."]
        pub fn get_supported_variation_list(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_supported_variation_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of faces in the TrueType / OpenType collection."]
        pub fn get_face_count(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9928usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Font", "get_face_count", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Font {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Font"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Font {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Font {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Font {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Font {
        
    }
    impl std::ops::Deref for Font {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Font {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Font__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Font` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Font::find_variation_ex`][super::Font::find_variation_ex]."]
#[must_use]
pub struct ExFindVariation < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, variation_coordinates: CowArg < 'ex, AnyDictionary >, face_index: i32, strength: f32, transform: Transform2D, spacing_top: i32, spacing_bottom: i32, spacing_space: i32, spacing_glyph: i32, baseline_offset: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExFindVariation < 'ex > {
    fn new(surround_object: &'ex re_export::Font, variation_coordinates: &'ex AnyDictionary,) -> Self {
        let face_index = 0i32;
        let strength = 0f32;
        let transform = Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _);
        let spacing_top = 0i32;
        let spacing_bottom = 0i32;
        let spacing_space = 0i32;
        let spacing_glyph = 0i32;
        let baseline_offset = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, variation_coordinates: CowArg::Borrowed(variation_coordinates), face_index: face_index, strength: strength, transform: transform, spacing_top: spacing_top, spacing_bottom: spacing_bottom, spacing_space: spacing_space, spacing_glyph: spacing_glyph, baseline_offset: baseline_offset,
        }
    }
    #[inline]
    pub fn face_index(self, face_index: i32) -> Self {
        Self {
            face_index: face_index, .. self
        }
    }
    #[inline]
    pub fn strength(self, strength: f32) -> Self {
        Self {
            strength: strength, .. self
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform2D) -> Self {
        Self {
            transform: transform, .. self
        }
    }
    #[inline]
    pub fn spacing_top(self, spacing_top: i32) -> Self {
        Self {
            spacing_top: spacing_top, .. self
        }
    }
    #[inline]
    pub fn spacing_bottom(self, spacing_bottom: i32) -> Self {
        Self {
            spacing_bottom: spacing_bottom, .. self
        }
    }
    #[inline]
    pub fn spacing_space(self, spacing_space: i32) -> Self {
        Self {
            spacing_space: spacing_space, .. self
        }
    }
    #[inline]
    pub fn spacing_glyph(self, spacing_glyph: i32) -> Self {
        Self {
            spacing_glyph: spacing_glyph, .. self
        }
    }
    #[inline]
    pub fn baseline_offset(self, baseline_offset: f32) -> Self {
        Self {
            baseline_offset: baseline_offset, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, variation_coordinates, face_index, strength, transform, spacing_top, spacing_bottom, spacing_space, spacing_glyph, baseline_offset,
        }
        = self;
        re_export::Font::find_variation_full(surround_object, variation_coordinates.cow_as_arg(), face_index, strength, transform, spacing_top, spacing_bottom, spacing_space, spacing_glyph, baseline_offset,)
    }
}
#[doc = "Default-param extender for [`Font::get_height_ex`][super::Font::get_height_ex]."]
#[must_use]
pub struct ExGetHeight < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetHeight < 'ex > {
    fn new(surround_object: &'ex re_export::Font,) -> Self {
        let font_size = 16i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, font_size,
        }
        = self;
        re_export::Font::get_height_full(surround_object, font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_ascent_ex`][super::Font::get_ascent_ex]."]
#[must_use]
pub struct ExGetAscent < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetAscent < 'ex > {
    fn new(surround_object: &'ex re_export::Font,) -> Self {
        let font_size = 16i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, font_size,
        }
        = self;
        re_export::Font::get_ascent_full(surround_object, font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_descent_ex`][super::Font::get_descent_ex]."]
#[must_use]
pub struct ExGetDescent < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetDescent < 'ex > {
    fn new(surround_object: &'ex re_export::Font,) -> Self {
        let font_size = 16i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, font_size,
        }
        = self;
        re_export::Font::get_descent_full(surround_object, font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_underline_position_ex`][super::Font::get_underline_position_ex]."]
#[must_use]
pub struct ExGetUnderlinePosition < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetUnderlinePosition < 'ex > {
    fn new(surround_object: &'ex re_export::Font,) -> Self {
        let font_size = 16i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, font_size,
        }
        = self;
        re_export::Font::get_underline_position_full(surround_object, font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_underline_thickness_ex`][super::Font::get_underline_thickness_ex]."]
#[must_use]
pub struct ExGetUnderlineThickness < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetUnderlineThickness < 'ex > {
    fn new(surround_object: &'ex re_export::Font,) -> Self {
        let font_size = 16i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, font_size,
        }
        = self;
        re_export::Font::get_underline_thickness_full(surround_object, font_size,)
    }
}
#[doc = "Default-param extender for [`Font::get_string_size_ex`][super::Font::get_string_size_ex]."]
#[must_use]
pub struct ExGetStringSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, text: CowArg < 'ex, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetStringSize < 'ex > {
    fn new(surround_object: &'ex re_export::Font, text: impl AsArg < GString > + 'ex,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, justification_flags: justification_flags, direction: direction, orientation: orientation,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        let Self {
            _phantom, surround_object, text, alignment, width, font_size, justification_flags, direction, orientation,
        }
        = self;
        re_export::Font::get_string_size_full(surround_object, text, alignment, width, font_size, justification_flags, direction, orientation,)
    }
}
#[doc = "Default-param extender for [`Font::get_multiline_string_size_ex`][super::Font::get_multiline_string_size_ex]."]
#[must_use]
pub struct ExGetMultilineStringSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, text: CowArg < 'ex, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetMultilineStringSize < 'ex > {
    fn new(surround_object: &'ex re_export::Font, text: impl AsArg < GString > + 'ex,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let max_lines = - 1i32;
        let brk_flags = crate::obj::EngineBitfield::from_ord(3);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, max_lines: max_lines, brk_flags: brk_flags, justification_flags: justification_flags, direction: direction, orientation: orientation,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, max_lines: i32) -> Self {
        Self {
            max_lines: max_lines, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, brk_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: brk_flags, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        let Self {
            _phantom, surround_object, text, alignment, width, font_size, max_lines, brk_flags, justification_flags, direction, orientation,
        }
        = self;
        re_export::Font::get_multiline_string_size_full(surround_object, text, alignment, width, font_size, max_lines, brk_flags, justification_flags, direction, orientation,)
    }
}
#[doc = "Default-param extender for [`Font::draw_string_ex`][super::Font::draw_string_ex]."]
#[must_use]
pub struct ExDrawString < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, canvas_item: Rid, pos: Vector2, text: CowArg < 'ex, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawString < 'ex > {
    fn new(surround_object: &'ex re_export::Font, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, modulate: modulate, justification_flags: justification_flags, direction: direction, orientation: orientation, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
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
            _phantom, surround_object, canvas_item, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation, oversampling,
        }
        = self;
        re_export::Font::draw_string_full(surround_object, canvas_item, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation, oversampling,)
    }
}
#[doc = "Default-param extender for [`Font::draw_multiline_string_ex`][super::Font::draw_multiline_string_ex]."]
#[must_use]
pub struct ExDrawMultilineString < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, canvas_item: Rid, pos: Vector2, text: CowArg < 'ex, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawMultilineString < 'ex > {
    fn new(surround_object: &'ex re_export::Font, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let max_lines = - 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let brk_flags = crate::obj::EngineBitfield::from_ord(3);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, max_lines: max_lines, modulate: modulate, brk_flags: brk_flags, justification_flags: justification_flags, direction: direction, orientation: orientation, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, max_lines: i32) -> Self {
        Self {
            max_lines: max_lines, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, brk_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: brk_flags, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
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
            _phantom, surround_object, canvas_item, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation, oversampling,
        }
        = self;
        re_export::Font::draw_multiline_string_full(surround_object, canvas_item, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation, oversampling,)
    }
}
#[doc = "Default-param extender for [`Font::draw_string_outline_ex`][super::Font::draw_string_outline_ex]."]
#[must_use]
pub struct ExDrawStringOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, canvas_item: Rid, pos: Vector2, text: CowArg < 'ex, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawStringOutline < 'ex > {
    fn new(surround_object: &'ex re_export::Font, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let size = 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, size: size, modulate: modulate, justification_flags: justification_flags, direction: direction, orientation: orientation, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
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
            _phantom, surround_object, canvas_item, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation, oversampling,
        }
        = self;
        re_export::Font::draw_string_outline_full(surround_object, canvas_item, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation, oversampling,)
    }
}
#[doc = "Default-param extender for [`Font::draw_multiline_string_outline_ex`][super::Font::draw_multiline_string_outline_ex]."]
#[must_use]
pub struct ExDrawMultilineStringOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, canvas_item: Rid, pos: Vector2, text: CowArg < 'ex, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, size: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawMultilineStringOutline < 'ex > {
    fn new(surround_object: &'ex re_export::Font, canvas_item: Rid, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let max_lines = - 1i32;
        let size = 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let brk_flags = crate::obj::EngineBitfield::from_ord(3);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, max_lines: max_lines, size: size, modulate: modulate, brk_flags: brk_flags, justification_flags: justification_flags, direction: direction, orientation: orientation, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, max_lines: i32) -> Self {
        Self {
            max_lines: max_lines, .. self
        }
    }
    #[inline]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, brk_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: brk_flags, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
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
            _phantom, surround_object, canvas_item, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation, oversampling,
        }
        = self;
        re_export::Font::draw_multiline_string_outline_full(surround_object, canvas_item, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation, oversampling,)
    }
}
#[doc = "Default-param extender for [`Font::draw_char_ex`][super::Font::draw_char_ex]."]
#[must_use]
pub struct ExDrawChar < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, canvas_item: Rid, pos: Vector2, char: u32, font_size: i32, modulate: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawChar < 'ex > {
    fn new(surround_object: &'ex re_export::Font, canvas_item: Rid, pos: Vector2, char: u32, font_size: i32,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, pos: pos, char: char, font_size: font_size, modulate: modulate, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, canvas_item, pos, char, font_size, modulate, oversampling,
        }
        = self;
        re_export::Font::draw_char_full(surround_object, canvas_item, pos, char, font_size, modulate, oversampling,)
    }
}
#[doc = "Default-param extender for [`Font::draw_char_outline_ex`][super::Font::draw_char_outline_ex]."]
#[must_use]
pub struct ExDrawCharOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Font, canvas_item: Rid, pos: Vector2, char: u32, font_size: i32, size: i32, modulate: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawCharOutline < 'ex > {
    fn new(surround_object: &'ex re_export::Font, canvas_item: Rid, pos: Vector2, char: u32, font_size: i32,) -> Self {
        let size = - 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, pos: pos, char: char, font_size: font_size, size: size, modulate: modulate, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, canvas_item, pos, char, font_size, size, modulate, oversampling,
        }
        = self;
        re_export::Font::draw_char_outline_full(surround_object, canvas_item, pos, char, font_size, size, modulate, oversampling,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Font;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Font {
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
#![doc = "Sidecar module for class [`TextServer`][crate::classes::TextServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextServer` enums](https://docs.godotengine.org/en/stable/classes/class_textserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextServer`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`text_server`][crate::classes::text_server]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `TextServer`](https://docs.godotengine.org/en/stable/classes/class_textserver.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<TextServer>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\n`TextServer` is the API backend for managing fonts and rendering text.\n\n**Note:** This is a low-level API, consider using [`TextLine`][crate::classes::TextLine], [`TextParagraph`][crate::classes::TextParagraph], and [`Font`][crate::classes::Font] classes instead.\n\nThis is an abstract class, so to get the currently active `TextServer` instance, use the following code:\n\n\n```gdscript\nvar ts = TextServerManager.get_primary_interface()\n```\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl TextServer {
        #[doc = "Returns `true` if the server supports a feature."]
        pub fn has_feature(&self, feature: crate::classes::text_server::Feature,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::text_server::Feature,);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "has_feature", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the server interface."]
        pub fn get_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns text server features, see \\[enum Feature]."]
        pub fn get_features(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "get_features", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads optional TextServer database (e.g. ICU break iterators and dictionaries).\n\n**Note:** This function should be called before any other TextServer functions used, otherwise it won't have any effect."]
        pub fn load_support_data(&mut self, filename: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (filename.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "load_support_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns default TextServer database (e.g. ICU break iterators and dictionaries) filename."]
        pub fn get_support_data_filename(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "get_support_data_filename", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns TextServer database (e.g. ICU break iterators and dictionaries) description."]
        pub fn get_support_data_info(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "get_support_data_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves optional TextServer database (e.g. ICU break iterators and dictionaries) to the file.\n\n**Note:** This function is used by during project export, to include TextServer database."]
        pub fn save_support_data(&self, filename: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (filename.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "save_support_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns default TextServer database (e.g. ICU break iterators and dictionaries)."]
        pub fn get_support_data(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "get_support_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the locale requires text server support data for line/word breaking."]
        pub fn is_locale_using_support_data(&self, locale: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "is_locale_using_support_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if locale is right-to-left."]
        pub fn is_locale_right_to_left(&self, locale: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "is_locale_right_to_left", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the given readable name of a feature, variation, script, or language to an OpenType tag."]
        pub fn name_to_tag(&self, name: impl AsArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "name_to_tag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the given OpenType tag to the readable name of a feature, variation, script, or language."]
        pub fn tag_to_name(&self, tag: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (tag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "tag_to_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if `rid` is valid resource owned by this text server."]
        pub fn has(&mut self, rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "has", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Frees an object created by this `TextServer`."]
        pub fn free_rid(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "free_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new, empty font cache entry resource. To free the resulting resource, use the [`free_rid`][`crate::classes::TextServer::free_rid`] method."]
        pub fn create_font(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "create_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new variation existing font which is reusing the same glyph cache and font data. To free the resulting resource, use the [`free_rid`][`crate::classes::TextServer::free_rid`] method."]
        pub fn create_font_linked_variation(&mut self, font_rid: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "create_font_linked_variation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets font source data, e.g contents of the dynamic font source file."]
        pub fn font_set_data(&mut self, font_rid: Rid, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, PackedByteArray >,);
            let args = (font_rid, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets an active face index in the TrueType / OpenType collection."]
        pub fn font_set_face_index(&mut self, font_rid: Rid, face_index: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, face_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_face_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an active face index in the TrueType / OpenType collection."]
        pub fn font_get_face_index(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_face_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of faces in the TrueType / OpenType collection."]
        pub fn font_get_face_count(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_face_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the font style flags.\n\n**Note:** This value is used for font matching only and will not affect font rendering. Use [`font_set_face_index`][`crate::classes::TextServer::font_set_face_index`], [`font_set_variation_coordinates`][`crate::classes::TextServer::font_set_variation_coordinates`], [`font_set_embolden`][`crate::classes::TextServer::font_set_embolden`], or [`font_set_transform`][`crate::classes::TextServer::font_set_transform`] instead."]
        pub fn font_set_style(&mut self, font_rid: Rid, style: crate::classes::text_server::FontStyle,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::FontStyle,);
            let args = (font_rid, style,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_style", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font style flags."]
        pub fn font_get_style(&self, font_rid: Rid,) -> crate::classes::text_server::FontStyle {
            type CallRet = crate::classes::text_server::FontStyle;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_style", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the font family name."]
        pub fn font_set_name(&mut self, font_rid: Rid, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7806usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font family name."]
        pub fn font_get_name(&self, font_rid: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7807usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns [`Dictionary`][crate::builtin::Dictionary] with OpenType font name strings (localized font names, version, description, license information, sample text, etc.)."]
        pub fn font_get_ot_name_strings(&self, font_rid: Rid,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7808usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_ot_name_strings", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the font style name."]
        pub fn font_set_style_name(&mut self, font_rid: Rid, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7809usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_style_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font style name."]
        pub fn font_get_style_name(&self, font_rid: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_style_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets weight (boldness) of the font. A value in the `100...999` range, normal font weight is `400`, bold font weight is `700`.\n\n**Note:** This value is used for font matching only and will not affect font rendering. Use [`font_set_face_index`][`crate::classes::TextServer::font_set_face_index`], [`font_set_variation_coordinates`][`crate::classes::TextServer::font_set_variation_coordinates`], or [`font_set_embolden`][`crate::classes::TextServer::font_set_embolden`] instead."]
        pub fn font_set_weight(&mut self, font_rid: Rid, weight: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_weight", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns weight (boldness) of the font. A value in the `100...999` range, normal font weight is `400`, bold font weight is `700`."]
        pub fn font_get_weight(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7812usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_weight", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets font stretch amount, compared to a normal width. A percentage value between `50%` and `200%`.\n\n**Note:** This value is used for font matching only and will not affect font rendering. Use [`font_set_face_index`][`crate::classes::TextServer::font_set_face_index`], [`font_set_variation_coordinates`][`crate::classes::TextServer::font_set_variation_coordinates`], or [`font_set_transform`][`crate::classes::TextServer::font_set_transform`] instead."]
        pub fn font_set_stretch(&mut self, font_rid: Rid, weight: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7813usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_stretch", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font stretch amount, compared to a normal width. A percentage value between `50%` and `200%`."]
        pub fn font_get_stretch(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7814usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_stretch", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets font anti-aliasing mode."]
        pub fn font_set_antialiasing(&mut self, font_rid: Rid, antialiasing: crate::classes::text_server::FontAntialiasing,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::FontAntialiasing,);
            let args = (font_rid, antialiasing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7815usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font anti-aliasing mode."]
        pub fn font_get_antialiasing(&self, font_rid: Rid,) -> crate::classes::text_server::FontAntialiasing {
            type CallRet = crate::classes::text_server::FontAntialiasing;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7816usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, embedded font bitmap loading is disabled (bitmap-only and color fonts ignore this property)."]
        pub fn font_set_disable_embedded_bitmaps(&mut self, font_rid: Rid, disable_embedded_bitmaps: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, disable_embedded_bitmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_disable_embedded_bitmaps", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether the font's embedded bitmap loading is disabled."]
        pub fn font_get_disable_embedded_bitmaps(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_disable_embedded_bitmaps", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true` font texture mipmap generation is enabled."]
        pub fn font_set_generate_mipmaps(&mut self, font_rid: Rid, generate_mipmaps: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, generate_mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if font texture mipmap generation is enabled."]
        pub fn font_get_generate_mipmaps(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, glyphs of all sizes are rendered using single multichannel signed distance field generated from the dynamic font vector data. MSDF rendering allows displaying the font at any scaling factor without blurriness, and without incurring a CPU cost when the font size changes (since the font no longer needs to be rasterized on the CPU). As a downside, font hinting is not available with MSDF. The lack of font hinting may result in less crisp and less readable fonts at small sizes.\n\n**Note:** MSDF font rendering does not render glyphs with overlapping shapes correctly. Overlapping shapes are not valid per the OpenType standard, but are still commonly found in many font files, especially those converted by Google Fonts. To avoid issues with overlapping glyphs, consider downloading the font file directly from the type foundry instead of relying on Google Fonts."]
        pub fn font_set_multichannel_signed_distance_field(&mut self, font_rid: Rid, msdf: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, msdf,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_multichannel_signed_distance_field", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if glyphs of all sizes are rendered using single multichannel signed distance field generated from the dynamic font vector data."]
        pub fn font_is_multichannel_signed_distance_field(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_is_multichannel_signed_distance_field", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the width of the range around the shape between the minimum and maximum representable signed distance."]
        pub fn font_set_msdf_pixel_range(&mut self, font_rid: Rid, msdf_pixel_range: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, msdf_pixel_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_msdf_pixel_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the width of the range around the shape between the minimum and maximum representable signed distance."]
        pub fn font_get_msdf_pixel_range(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_msdf_pixel_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets source font size used to generate MSDF textures."]
        pub fn font_set_msdf_size(&mut self, font_rid: Rid, msdf_size: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, msdf_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_msdf_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns source font size used to generate MSDF textures."]
        pub fn font_get_msdf_size(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_msdf_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets bitmap font fixed size. If set to value greater than zero, same cache entry will be used for all font sizes."]
        pub fn font_set_fixed_size(&mut self, font_rid: Rid, fixed_size: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, fixed_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_fixed_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns bitmap font fixed size."]
        pub fn font_get_fixed_size(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_fixed_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets bitmap font scaling mode. This property is used only if `fixed_size` is greater than zero."]
        pub fn font_set_fixed_size_scale_mode(&mut self, font_rid: Rid, fixed_size_scale_mode: crate::classes::text_server::FixedSizeScaleMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::FixedSizeScaleMode,);
            let args = (font_rid, fixed_size_scale_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_fixed_size_scale_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns bitmap font scaling mode."]
        pub fn font_get_fixed_size_scale_mode(&self, font_rid: Rid,) -> crate::classes::text_server::FixedSizeScaleMode {
            type CallRet = crate::classes::text_server::FixedSizeScaleMode;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_fixed_size_scale_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, system fonts can be automatically used as fallbacks."]
        pub fn font_set_allow_system_fallback(&mut self, font_rid: Rid, allow_system_fallback: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, allow_system_fallback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_allow_system_fallback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if system fonts can be automatically used as fallbacks."]
        pub fn font_is_allow_system_fallback(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_is_allow_system_fallback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Frees all automatically loaded system fonts."]
        pub fn font_clear_system_fallback_cache(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_system_fallback_cache", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true` auto-hinting is preferred over font built-in hinting."]
        pub fn font_set_force_autohinter(&mut self, font_rid: Rid, force_autohinter: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, force_autohinter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_force_autohinter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if auto-hinting is supported and preferred over font built-in hinting. Used by dynamic fonts only."]
        pub fn font_is_force_autohinter(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_is_force_autohinter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, color modulation is applied when drawing colored glyphs, otherwise it's applied to the monochrome glyphs only."]
        pub fn font_set_modulate_color_glyphs(&mut self, font_rid: Rid, force_autohinter: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, force_autohinter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_modulate_color_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if color modulation is applied when drawing the font's colored glyphs."]
        pub fn font_is_modulate_color_glyphs(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_is_modulate_color_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets font hinting mode. Used by dynamic fonts only."]
        pub fn font_set_hinting(&mut self, font_rid: Rid, hinting: crate::classes::text_server::Hinting,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::Hinting,);
            let args = (font_rid, hinting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_hinting", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the font hinting mode. Used by dynamic fonts only."]
        pub fn font_get_hinting(&self, font_rid: Rid,) -> crate::classes::text_server::Hinting {
            type CallRet = crate::classes::text_server::Hinting;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_hinting", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets font subpixel glyph positioning mode."]
        pub fn font_set_subpixel_positioning(&mut self, font_rid: Rid, subpixel_positioning: crate::classes::text_server::SubpixelPositioning,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::SubpixelPositioning,);
            let args = (font_rid, subpixel_positioning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_subpixel_positioning", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font subpixel glyph positioning mode."]
        pub fn font_get_subpixel_positioning(&self, font_rid: Rid,) -> crate::classes::text_server::SubpixelPositioning {
            type CallRet = crate::classes::text_server::SubpixelPositioning;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_subpixel_positioning", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets glyph position rounding behavior. If set to `true`, when aligning glyphs to the pixel boundaries rounding remainders are accumulated to ensure more uniform glyph distribution. This setting has no effect if subpixel positioning is enabled."]
        pub fn font_set_keep_rounding_remainders(&mut self, font_rid: Rid, keep_rounding_remainders: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, keep_rounding_remainders,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_keep_rounding_remainders", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns glyph position rounding behavior. If set to `true`, when aligning glyphs to the pixel boundaries rounding remainders are accumulated to ensure more uniform glyph distribution. This setting has no effect if subpixel positioning is enabled."]
        pub fn font_get_keep_rounding_remainders(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_keep_rounding_remainders", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets font embolden strength. If `strength` is not equal to zero, emboldens the font outlines. Negative values reduce the outline thickness."]
        pub fn font_set_embolden(&mut self, font_rid: Rid, strength: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (font_rid, strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_embolden", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font embolden strength."]
        pub fn font_get_embolden(&self, font_rid: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_embolden", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the spacing for `spacing` to `value` in pixels (not relative to the font size)."]
        pub fn font_set_spacing(&mut self, font_rid: Rid, spacing: crate::classes::text_server::SpacingType, value: i64,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::SpacingType, i64,);
            let args = (font_rid, spacing, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_spacing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the spacing for `spacing` in pixels (not relative to the font size)."]
        pub fn font_get_spacing(&self, font_rid: Rid, spacing: crate::classes::text_server::SpacingType,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, crate::classes::text_server::SpacingType,);
            let args = (font_rid, spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_spacing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets extra baseline offset (as a fraction of font height)."]
        pub fn font_set_baseline_offset(&mut self, font_rid: Rid, baseline_offset: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (font_rid, baseline_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_baseline_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns extra baseline offset (as a fraction of font height)."]
        pub fn font_get_baseline_offset(&self, font_rid: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_baseline_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets 2D transform, applied to the font outlines, can be used for slanting, flipping, and rotating glyphs.\n\nFor example, to simulate italic typeface by slanting, apply the following transform `Transform2D(1.0, slant, 0.0, 1.0, 0.0, 0.0)`."]
        pub fn font_set_transform(&mut self, font_rid: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (font_rid, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns 2D transform applied to the font outlines."]
        pub fn font_get_transform(&self, font_rid: Rid,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets variation coordinates for the specified font cache entry. See [`font_supported_variation_list`][`crate::classes::TextServer::font_supported_variation_list`] for more info."]
        pub fn font_set_variation_coordinates(&mut self, font_rid: Rid, variation_coordinates: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, AnyDictionary >,);
            let args = (font_rid, RefArg::new(variation_coordinates),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_variation_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns variation coordinates for the specified font cache entry. See [`font_supported_variation_list`][`crate::classes::TextServer::font_supported_variation_list`] for more info."]
        pub fn font_get_variation_coordinates(&self, font_rid: Rid,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_variation_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to a positive value, overrides the oversampling factor of the viewport this font is used in. See \\[member Viewport.oversampling]. This value doesn't override the `oversampling` parameter of `draw_*` methods. Used by dynamic fonts only."]
        pub fn font_set_oversampling(&mut self, font_rid: Rid, oversampling: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (font_rid, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns oversampling factor override. If set to a positive value, overrides the oversampling factor of the viewport this font is used in. See \\[member Viewport.oversampling]. This value doesn't override the `oversampling` parameter of `draw_*` methods. Used by dynamic fonts only."]
        pub fn font_get_oversampling(&self, font_rid: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns list of the font sizes in the cache. Each size is [`Vector2i`][crate::builtin::Vector2i] with font size and outline size."]
        pub fn font_get_size_cache_list(&self, font_rid: Rid,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_size_cache_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all font sizes from the cache entry."]
        pub fn font_clear_size_cache(&mut self, font_rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_size_cache", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes specified font size from the cache entry."]
        pub fn font_remove_size_cache(&mut self, font_rid: Rid, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_size_cache", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font cache information, each entry contains the following fields: `Vector2i size_px` - font size in pixels, `float viewport_oversampling` - viewport oversampling factor, `int glyphs` - number of rendered glyphs, `int textures` - number of used textures, `int textures_size` - size of texture data in bytes."]
        pub fn font_get_size_cache_info(&self, font_rid: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_size_cache_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the font ascent (number of pixels above the baseline)."]
        pub fn font_set_ascent(&mut self, font_rid: Rid, size: i64, ascent: f64,) {
            type CallRet = ();
            type CallParams = (Rid, i64, f64,);
            let args = (font_rid, size, ascent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_ascent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the font ascent (number of pixels above the baseline)."]
        pub fn font_get_ascent(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_ascent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the font descent (number of pixels below the baseline)."]
        pub fn font_set_descent(&mut self, font_rid: Rid, size: i64, descent: f64,) {
            type CallRet = ();
            type CallParams = (Rid, i64, f64,);
            let args = (font_rid, size, descent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_descent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the font descent (number of pixels below the baseline)."]
        pub fn font_get_descent(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_descent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets pixel offset of the underline below the baseline."]
        pub fn font_set_underline_position(&mut self, font_rid: Rid, size: i64, underline_position: f64,) {
            type CallRet = ();
            type CallParams = (Rid, i64, f64,);
            let args = (font_rid, size, underline_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns pixel offset of the underline below the baseline."]
        pub fn font_get_underline_position(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets thickness of the underline in pixels."]
        pub fn font_set_underline_thickness(&mut self, font_rid: Rid, size: i64, underline_thickness: f64,) {
            type CallRet = ();
            type CallParams = (Rid, i64, f64,);
            let args = (font_rid, size, underline_thickness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns thickness of the underline in pixels."]
        pub fn font_get_underline_thickness(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets scaling factor of the color bitmap font."]
        pub fn font_set_scale(&mut self, font_rid: Rid, size: i64, scale: f64,) {
            type CallRet = ();
            type CallParams = (Rid, i64, f64,);
            let args = (font_rid, size, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns scaling factor of the color bitmap font."]
        pub fn font_get_scale(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of textures used by font cache entry."]
        pub fn font_get_texture_count(&self, font_rid: Rid, size: Vector2i,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, Vector2i,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_texture_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all textures from font cache entry.\n\n**Note:** This function will not remove glyphs associated with the texture, use [`font_remove_glyph`][`crate::classes::TextServer::font_remove_glyph`] to remove them manually."]
        pub fn font_clear_textures(&mut self, font_rid: Rid, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_textures", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes specified texture from the cache entry.\n\n**Note:** This function will not remove glyphs associated with the texture, remove them manually, using [`font_remove_glyph`][`crate::classes::TextServer::font_remove_glyph`]."]
        pub fn font_remove_texture(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets font cache texture image data."]
        pub fn font_set_texture_image(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64, image: impl AsArg < Option < Gd < crate::classes::Image >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, Vector2i, i64, CowArg < 'a0, Option < Gd < crate::classes::Image > > >,);
            let args = (font_rid, size, texture_index, image.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_texture_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font cache texture image data."]
        pub fn font_get_texture_image(&self, font_rid: Rid, size: Vector2i, texture_index: i64,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_texture_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets array containing glyph packing data."]
        pub fn font_set_texture_offsets(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64, offset: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, Vector2i, i64, RefArg < 'a0, PackedInt32Array >,);
            let args = (font_rid, size, texture_index, RefArg::new(offset),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_texture_offsets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns array containing glyph packing data."]
        pub fn font_get_texture_offsets(&self, font_rid: Rid, size: Vector2i, texture_index: i64,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_texture_offsets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns list of rendered glyphs in the cache entry."]
        pub fn font_get_glyph_list(&self, font_rid: Rid, size: Vector2i,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid, Vector2i,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all rendered glyph information from the cache entry.\n\n**Note:** This function will not remove textures associated with the glyphs, use [`font_remove_texture`][`crate::classes::TextServer::font_remove_texture`] to remove them manually."]
        pub fn font_clear_glyphs(&mut self, font_rid: Rid, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes specified rendered glyph information from the cache entry.\n\n**Note:** This function will not remove textures associated with the glyphs, use [`font_remove_texture`][`crate::classes::TextServer::font_remove_texture`] to remove them manually."]
        pub fn font_remove_glyph(&mut self, font_rid: Rid, size: Vector2i, glyph: i64,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_glyph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns glyph advance (offset of the next glyph).\n\n**Note:** Advance for glyphs outlines is the same as the base glyph advance and is not saved."]
        pub fn font_get_glyph_advance(&self, font_rid: Rid, size: i64, glyph: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid, i64, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_advance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets glyph advance (offset of the next glyph).\n\n**Note:** Advance for glyphs outlines is the same as the base glyph advance and is not saved."]
        pub fn font_set_glyph_advance(&mut self, font_rid: Rid, size: i64, glyph: i64, advance: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, i64, i64, Vector2,);
            let args = (font_rid, size, glyph, advance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_advance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns glyph offset from the baseline."]
        pub fn font_get_glyph_offset(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets glyph offset from the baseline."]
        pub fn font_set_glyph_offset(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64, Vector2,);
            let args = (font_rid, size, glyph, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns size of the glyph."]
        pub fn font_get_glyph_size(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets size of the glyph."]
        pub fn font_set_glyph_size(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, gl_size: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64, Vector2,);
            let args = (font_rid, size, glyph, gl_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns rectangle in the cache texture containing the glyph."]
        pub fn font_get_glyph_uv_rect(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_uv_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets rectangle in the cache texture containing the glyph."]
        pub fn font_set_glyph_uv_rect(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, uv_rect: Rect2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64, Rect2,);
            let args = (font_rid, size, glyph, uv_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_uv_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns index of the cache texture containing the glyph."]
        pub fn font_get_glyph_texture_idx(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_texture_idx", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets index of the cache texture containing the glyph."]
        pub fn font_set_glyph_texture_idx(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, texture_idx: i64,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64, i64,);
            let args = (font_rid, size, glyph, texture_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_texture_idx", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns resource ID of the cache texture containing the glyph.\n\n**Note:** If there are pending glyphs to render, calling this function might trigger the texture cache update."]
        pub fn font_get_glyph_texture_rid(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_texture_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns size of the cache texture containing the glyph.\n\n**Note:** If there are pending glyphs to render, calling this function might trigger the texture cache update."]
        pub fn font_get_glyph_texture_size(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_texture_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns outline contours of the glyph as a [`Dictionary`][crate::builtin::Dictionary] with the following contents:\n\n`points`         - [`PackedVector3Array`][crate::builtin::PackedVector3Array], containing outline points. `x` and `y` are point coordinates. `z` is the type of the point, using the \\[enum ContourPointTag] values.\n\n`contours`       - [`PackedInt32Array`][crate::builtin::PackedInt32Array], containing indices the end points of each contour.\n\n`orientation`    - `bool`, contour orientation. If `true`, clockwise contours must be filled.\n\n- Two successive [`ContourPointTag::ON`][`crate::classes::text_server::ContourPointTag::ON`] points indicate a line segment.\n\n- One [`ContourPointTag::OFF_CONIC`][`crate::classes::text_server::ContourPointTag::OFF_CONIC`] point between two [`ContourPointTag::ON`][`crate::classes::text_server::ContourPointTag::ON`] points indicates a single conic (quadratic) Bézier arc.\n\n- Two [`ContourPointTag::OFF_CUBIC`][`crate::classes::text_server::ContourPointTag::OFF_CUBIC`] points between two [`ContourPointTag::ON`][`crate::classes::text_server::ContourPointTag::ON`] points indicate a single cubic Bézier arc.\n\n- Two successive [`ContourPointTag::OFF_CONIC`][`crate::classes::text_server::ContourPointTag::OFF_CONIC`] points indicate two successive conic (quadratic) Bézier arcs with a virtual [`ContourPointTag::ON`][`crate::classes::text_server::ContourPointTag::ON`] point at their middle.\n\n- Each contour is closed. The last point of a contour uses the first point of a contour as its next point, and vice versa. The first point can be [`ContourPointTag::OFF_CONIC`][`crate::classes::text_server::ContourPointTag::OFF_CONIC`] point."]
        pub fn font_get_glyph_contours(&self, font: Rid, size: i64, index: i64,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid, i64, i64,);
            let args = (font, size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_contours", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns list of the kerning overrides."]
        pub fn font_get_kerning_list(&self, font_rid: Rid, size: i64,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_kerning_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all kerning overrides."]
        pub fn font_clear_kerning_map(&mut self, font_rid: Rid, size: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7894usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_kerning_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes kerning override for the pair of glyphs."]
        pub fn font_remove_kerning(&mut self, font_rid: Rid, size: i64, glyph_pair: Vector2i,) {
            type CallRet = ();
            type CallParams = (Rid, i64, Vector2i,);
            let args = (font_rid, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7895usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_kerning", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets kerning for the pair of glyphs."]
        pub fn font_set_kerning(&mut self, font_rid: Rid, size: i64, glyph_pair: Vector2i, kerning: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, i64, Vector2i, Vector2,);
            let args = (font_rid, size, glyph_pair, kerning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7896usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_kerning", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns kerning for the pair of glyphs."]
        pub fn font_get_kerning(&self, font_rid: Rid, size: i64, glyph_pair: Vector2i,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid, i64, Vector2i,);
            let args = (font_rid, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7897usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_kerning", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the glyph index of a `char`, optionally modified by the `variation_selector`. See [`font_get_char_from_glyph_index`][`crate::classes::TextServer::font_get_char_from_glyph_index`]."]
        pub fn font_get_glyph_index(&self, font_rid: Rid, size: i64, char: i64, variation_selector: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64, i64, i64,);
            let args = (font_rid, size, char, variation_selector,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7898usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns character code associated with `glyph_index`, or `0` if `glyph_index` is invalid. See [`font_get_glyph_index`][`crate::classes::TextServer::font_get_glyph_index`]."]
        pub fn font_get_char_from_glyph_index(&self, font_rid: Rid, size: i64, glyph_index: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64, i64,);
            let args = (font_rid, size, glyph_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7899usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_char_from_glyph_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a Unicode `char` is available in the font."]
        pub fn font_has_char(&self, font_rid: Rid, char: i64,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i64,);
            let args = (font_rid, char,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7900usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_has_char", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a string containing all the characters available in the font."]
        pub fn font_get_supported_chars(&self, font_rid: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_supported_chars", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array containing all glyph indices in the font."]
        pub fn font_get_supported_glyphs(&self, font_rid: Rid,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_supported_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renders the range of characters to the font cache texture."]
        pub fn font_render_range(&mut self, font_rid: Rid, size: Vector2i, start: i64, end: i64,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64, i64,);
            let args = (font_rid, size, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_render_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renders specified glyph to the font cache texture."]
        pub fn font_render_glyph(&mut self, font_rid: Rid, size: Vector2i, index: i64,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_render_glyph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Draws single glyph into a canvas item at the position, using `font_rid` at the size `size`. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Note:** Glyph index is specific to the font, use glyphs indices returned by [`shaped_text_get_glyphs`][`crate::classes::TextServer::shaped_text_get_glyphs`] or [`font_get_glyph_index`][`crate::classes::TextServer::font_get_glyph_index`].\n\n**Note:** If there are pending glyphs to render, calling this function might trigger the texture cache update."]
        pub(crate) fn font_draw_glyph_full(&self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, i64, Vector2, i64, Color, f32,);
            let args = (font_rid, canvas, size, pos, index, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_draw_glyph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`font_draw_glyph_ex`][Self::font_draw_glyph_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws single glyph into a canvas item at the position, using `font_rid` at the size `size`. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Note:** Glyph index is specific to the font, use glyphs indices returned by [`shaped_text_get_glyphs`][`crate::classes::TextServer::shaped_text_get_glyphs`] or [`font_get_glyph_index`][`crate::classes::TextServer::font_get_glyph_index`].\n\n**Note:** If there are pending glyphs to render, calling this function might trigger the texture cache update."]
        #[inline]
        pub fn font_draw_glyph(&self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64,) {
            self.font_draw_glyph_ex(font_rid, canvas, size, pos, index,) . done()
        }
        #[doc = "Draws single glyph into a canvas item at the position, using `font_rid` at the size `size`. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Note:** Glyph index is specific to the font, use glyphs indices returned by [`shaped_text_get_glyphs`][`crate::classes::TextServer::shaped_text_get_glyphs`] or [`font_get_glyph_index`][`crate::classes::TextServer::font_get_glyph_index`].\n\n**Note:** If there are pending glyphs to render, calling this function might trigger the texture cache update."]
        #[inline]
        pub fn font_draw_glyph_ex < 'ex > (&'ex self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64,) -> ExFontDrawGlyph < 'ex > {
            ExFontDrawGlyph::new(self, font_rid, canvas, size, pos, index,)
        }
        #[doc = "Draws single glyph outline of size `outline_size` into a canvas item at the position, using `font_rid` at the size `size`. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Note:** Glyph index is specific to the font, use glyphs indices returned by [`shaped_text_get_glyphs`][`crate::classes::TextServer::shaped_text_get_glyphs`] or [`font_get_glyph_index`][`crate::classes::TextServer::font_get_glyph_index`].\n\n**Note:** If there are pending glyphs to render, calling this function might trigger the texture cache update."]
        pub(crate) fn font_draw_glyph_outline_full(&self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, i64, i64, Vector2, i64, Color, f32,);
            let args = (font_rid, canvas, size, outline_size, pos, index, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_draw_glyph_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`font_draw_glyph_outline_ex`][Self::font_draw_glyph_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws single glyph outline of size `outline_size` into a canvas item at the position, using `font_rid` at the size `size`. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Note:** Glyph index is specific to the font, use glyphs indices returned by [`shaped_text_get_glyphs`][`crate::classes::TextServer::shaped_text_get_glyphs`] or [`font_get_glyph_index`][`crate::classes::TextServer::font_get_glyph_index`].\n\n**Note:** If there are pending glyphs to render, calling this function might trigger the texture cache update."]
        #[inline]
        pub fn font_draw_glyph_outline(&self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64,) {
            self.font_draw_glyph_outline_ex(font_rid, canvas, size, outline_size, pos, index,) . done()
        }
        #[doc = "Draws single glyph outline of size `outline_size` into a canvas item at the position, using `font_rid` at the size `size`. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Note:** Glyph index is specific to the font, use glyphs indices returned by [`shaped_text_get_glyphs`][`crate::classes::TextServer::shaped_text_get_glyphs`] or [`font_get_glyph_index`][`crate::classes::TextServer::font_get_glyph_index`].\n\n**Note:** If there are pending glyphs to render, calling this function might trigger the texture cache update."]
        #[inline]
        pub fn font_draw_glyph_outline_ex < 'ex > (&'ex self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64,) -> ExFontDrawGlyphOutline < 'ex > {
            ExFontDrawGlyphOutline::new(self, font_rid, canvas, size, outline_size, pos, index,)
        }
        #[doc = "Returns `true` if the font supports the given language (as a [ISO 639](https://en.wikipedia.org/wiki/ISO_639-1) code)."]
        pub fn font_is_language_supported(&self, font_rid: Rid, language: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_is_language_supported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds override for [`font_is_language_supported`][`crate::classes::TextServer::font_is_language_supported`]."]
        pub fn font_set_language_support_override(&mut self, font_rid: Rid, language: impl AsArg < GString >, supported: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >, bool,);
            let args = (font_rid, language.into_arg(), supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_language_support_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if support override is enabled for the `language`."]
        pub fn font_get_language_support_override(&mut self, font_rid: Rid, language: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_language_support_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Remove language support override."]
        pub fn font_remove_language_support_override(&mut self, font_rid: Rid, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_language_support_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns list of language support overrides."]
        pub fn font_get_language_support_overrides(&mut self, font_rid: Rid,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_language_support_overrides", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the font supports the given script (as a [ISO 15924](https://en.wikipedia.org/wiki/ISO_15924) code)."]
        pub fn font_is_script_supported(&self, font_rid: Rid, script: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7912usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_is_script_supported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds override for [`font_is_script_supported`][`crate::classes::TextServer::font_is_script_supported`]."]
        pub fn font_set_script_support_override(&mut self, font_rid: Rid, script: impl AsArg < GString >, supported: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >, bool,);
            let args = (font_rid, script.into_arg(), supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7913usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_script_support_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if support override is enabled for the `script`."]
        pub fn font_get_script_support_override(&mut self, font_rid: Rid, script: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7914usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_script_support_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes script support override."]
        pub fn font_remove_script_support_override(&mut self, font_rid: Rid, script: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7915usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_script_support_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns list of script support overrides."]
        pub fn font_get_script_support_overrides(&mut self, font_rid: Rid,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7916usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_script_support_overrides", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets font OpenType feature set override."]
        pub fn font_set_opentype_feature_overrides(&mut self, font_rid: Rid, overrides: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, AnyDictionary >,);
            let args = (font_rid, RefArg::new(overrides),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7917usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_opentype_feature_overrides", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns font OpenType feature set override."]
        pub fn font_get_opentype_feature_overrides(&self, font_rid: Rid,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7918usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_opentype_feature_overrides", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the dictionary of the supported OpenType features."]
        pub fn font_supported_feature_list(&self, font_rid: Rid,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7919usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_supported_feature_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the dictionary of the supported OpenType variation coordinates."]
        pub fn font_supported_variation_list(&self, font_rid: Rid,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7920usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_supported_variation_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This method does nothing and always returns `1.0`."]
        pub fn font_get_global_oversampling(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7921usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_global_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This method does nothing."]
        pub fn font_set_global_oversampling(&mut self, oversampling: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7922usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_global_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns size of the replacement character (box with character hexadecimal code that is drawn in place of invalid characters)."]
        pub fn get_hex_code_box_size(&self, size: i64, index: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i64, i64,);
            let args = (size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7923usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "get_hex_code_box_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Draws box displaying character hexadecimal code. Used for replacing missing characters."]
        pub fn draw_hex_code_box(&self, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, i64, Vector2, i64, Color,);
            let args = (canvas, size, pos, index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7924usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "draw_hex_code_box", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new buffer for complex text layout, with the given `direction` and `orientation`. To free the resulting buffer, use [`free_rid`][`crate::classes::TextServer::free_rid`] method.\n\n**Note:** Direction is ignored if server does not support [`Feature::BIDI_LAYOUT`][`crate::classes::text_server::Feature::BIDI_LAYOUT`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced]).\n\n**Note:** Orientation is ignored if server does not support [`Feature::VERTICAL_LAYOUT`][`crate::classes::text_server::Feature::VERTICAL_LAYOUT`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced])."]
        pub(crate) fn create_shaped_text_full(&mut self, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,) -> Rid {
            type CallRet = Rid;
            type CallParams = (crate::classes::text_server::Direction, crate::classes::text_server::Orientation,);
            let args = (direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7925usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "create_shaped_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_shaped_text_ex`][Self::create_shaped_text_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new buffer for complex text layout, with the given `direction` and `orientation`. To free the resulting buffer, use [`free_rid`][`crate::classes::TextServer::free_rid`] method.\n\n**Note:** Direction is ignored if server does not support [`Feature::BIDI_LAYOUT`][`crate::classes::text_server::Feature::BIDI_LAYOUT`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced]).\n\n**Note:** Orientation is ignored if server does not support [`Feature::VERTICAL_LAYOUT`][`crate::classes::text_server::Feature::VERTICAL_LAYOUT`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced])."]
        #[inline]
        pub fn create_shaped_text(&mut self,) -> Rid {
            self.create_shaped_text_ex() . done()
        }
        #[doc = "Creates a new buffer for complex text layout, with the given `direction` and `orientation`. To free the resulting buffer, use [`free_rid`][`crate::classes::TextServer::free_rid`] method.\n\n**Note:** Direction is ignored if server does not support [`Feature::BIDI_LAYOUT`][`crate::classes::text_server::Feature::BIDI_LAYOUT`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced]).\n\n**Note:** Orientation is ignored if server does not support [`Feature::VERTICAL_LAYOUT`][`crate::classes::text_server::Feature::VERTICAL_LAYOUT`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced])."]
        #[inline]
        pub fn create_shaped_text_ex < 'ex > (&'ex mut self,) -> ExCreateShapedText < 'ex > {
            ExCreateShapedText::new(self,)
        }
        #[doc = "Clears text buffer (removes text and inline objects)."]
        pub fn shaped_text_clear(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Duplicates shaped text buffer."]
        pub fn shaped_text_duplicate(&mut self, rid: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_duplicate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets desired text direction. If set to [`Direction::AUTO`][`crate::classes::text_server::Direction::AUTO`], direction will be detected based on the buffer contents and current locale.\n\n**Note:** Direction is ignored if server does not support [`Feature::BIDI_LAYOUT`][`crate::classes::text_server::Feature::BIDI_LAYOUT`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced])."]
        pub(crate) fn shaped_text_set_direction_full(&mut self, shaped: Rid, direction: crate::classes::text_server::Direction,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::Direction,);
            let args = (shaped, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7928usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_text_set_direction_ex`][Self::shaped_text_set_direction_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets desired text direction. If set to [`Direction::AUTO`][`crate::classes::text_server::Direction::AUTO`], direction will be detected based on the buffer contents and current locale.\n\n**Note:** Direction is ignored if server does not support [`Feature::BIDI_LAYOUT`][`crate::classes::text_server::Feature::BIDI_LAYOUT`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced])."]
        #[inline]
        pub fn shaped_text_set_direction(&mut self, shaped: Rid,) {
            self.shaped_text_set_direction_ex(shaped,) . done()
        }
        #[doc = "Sets desired text direction. If set to [`Direction::AUTO`][`crate::classes::text_server::Direction::AUTO`], direction will be detected based on the buffer contents and current locale.\n\n**Note:** Direction is ignored if server does not support [`Feature::BIDI_LAYOUT`][`crate::classes::text_server::Feature::BIDI_LAYOUT`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced])."]
        #[inline]
        pub fn shaped_text_set_direction_ex < 'ex > (&'ex mut self, shaped: Rid,) -> ExShapedTextSetDirection < 'ex > {
            ExShapedTextSetDirection::new(self, shaped,)
        }
        #[doc = "Returns direction of the text."]
        pub fn shaped_text_get_direction(&self, shaped: Rid,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7929usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns direction of the text, inferred by the BiDi algorithm."]
        pub fn shaped_text_get_inferred_direction(&self, shaped: Rid,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7930usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_inferred_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Overrides BiDi for the structured text.\n\nOverride ranges should cover full source text without overlaps. BiDi algorithm will be used on each range separately."]
        pub fn shaped_text_set_bidi_override(&mut self, shaped: Rid, override_: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, AnyArray >,);
            let args = (shaped, RefArg::new(override_),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7931usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets custom punctuation character list, used for word breaking. If set to empty string, server defaults are used."]
        pub fn shaped_text_set_custom_punctuation(&mut self, shaped: Rid, punct: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (shaped, punct.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7932usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_custom_punctuation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns custom punctuation character list, used for word breaking. If set to empty string, server defaults are used."]
        pub fn shaped_text_get_custom_punctuation(&self, shaped: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7933usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_custom_punctuation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets ellipsis character used for text clipping."]
        pub fn shaped_text_set_custom_ellipsis(&mut self, shaped: Rid, char: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (shaped, char,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_custom_ellipsis", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns ellipsis character used for text clipping."]
        pub fn shaped_text_get_custom_ellipsis(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_custom_ellipsis", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets desired text orientation.\n\n**Note:** Orientation is ignored if server does not support [`Feature::VERTICAL_LAYOUT`][`crate::classes::text_server::Feature::VERTICAL_LAYOUT`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced])."]
        pub(crate) fn shaped_text_set_orientation_full(&mut self, shaped: Rid, orientation: crate::classes::text_server::Orientation,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::Orientation,);
            let args = (shaped, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_orientation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_text_set_orientation_ex`][Self::shaped_text_set_orientation_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets desired text orientation.\n\n**Note:** Orientation is ignored if server does not support [`Feature::VERTICAL_LAYOUT`][`crate::classes::text_server::Feature::VERTICAL_LAYOUT`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced])."]
        #[inline]
        pub fn shaped_text_set_orientation(&mut self, shaped: Rid,) {
            self.shaped_text_set_orientation_ex(shaped,) . done()
        }
        #[doc = "Sets desired text orientation.\n\n**Note:** Orientation is ignored if server does not support [`Feature::VERTICAL_LAYOUT`][`crate::classes::text_server::Feature::VERTICAL_LAYOUT`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced])."]
        #[inline]
        pub fn shaped_text_set_orientation_ex < 'ex > (&'ex mut self, shaped: Rid,) -> ExShapedTextSetOrientation < 'ex > {
            ExShapedTextSetOrientation::new(self, shaped,)
        }
        #[doc = "Returns text orientation."]
        pub fn shaped_text_get_orientation(&self, shaped: Rid,) -> crate::classes::text_server::Orientation {
            type CallRet = crate::classes::text_server::Orientation;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_orientation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true` text buffer will display invalid characters as hexadecimal codes, otherwise nothing is displayed."]
        pub fn shaped_text_set_preserve_invalid(&mut self, shaped: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (shaped, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_preserve_invalid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if text buffer is configured to display hexadecimal codes in place of invalid characters.\n\n**Note:** If set to `false`, nothing is displayed in place of invalid characters."]
        pub fn shaped_text_get_preserve_invalid(&self, shaped: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7939usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_preserve_invalid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true` text buffer will display control characters."]
        pub fn shaped_text_set_preserve_control(&mut self, shaped: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (shaped, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7940usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_preserve_control", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if text buffer is configured to display control characters."]
        pub fn shaped_text_get_preserve_control(&self, shaped: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_preserve_control", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets extra spacing added between glyphs or lines in pixels."]
        pub fn shaped_text_set_spacing(&mut self, shaped: Rid, spacing: crate::classes::text_server::SpacingType, value: i64,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::SpacingType, i64,);
            let args = (shaped, spacing, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_spacing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns extra spacing added between glyphs or lines in pixels."]
        pub fn shaped_text_get_spacing(&self, shaped: Rid, spacing: crate::classes::text_server::SpacingType,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, crate::classes::text_server::SpacingType,);
            let args = (shaped, spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_spacing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds text span and font to draw it to the text buffer."]
        pub(crate) fn shaped_text_add_string_full(&mut self, shaped: Rid, text: CowArg < GString >, fonts: RefArg < Array < Rid > >, size: i64, opentype_features: RefArg < AnyDictionary >, language: CowArg < GString >, meta: RefArg < Variant >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (Rid, CowArg < 'a0, GString >, RefArg < 'a1, Array < Rid > >, i64, RefArg < 'a2, AnyDictionary >, CowArg < 'a3, GString >, RefArg < 'a4, Variant >,);
            let args = (shaped, text, fonts, size, opentype_features, language, meta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_add_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_text_add_string_ex`][Self::shaped_text_add_string_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds text span and font to draw it to the text buffer."]
        #[inline]
        pub fn shaped_text_add_string(&mut self, shaped: Rid, text: impl AsArg < GString >, fonts: &Array < Rid >, size: i64,) -> bool {
            self.shaped_text_add_string_ex(shaped, text, fonts, size,) . done()
        }
        #[doc = "Adds text span and font to draw it to the text buffer."]
        #[inline]
        pub fn shaped_text_add_string_ex < 'ex > (&'ex mut self, shaped: Rid, text: impl AsArg < GString > + 'ex, fonts: &'ex Array < Rid >, size: i64,) -> ExShapedTextAddString < 'ex > {
            ExShapedTextAddString::new(self, shaped, text, fonts, size,)
        }
        #[doc = "Adds inline object to the text buffer, `key` must be unique. In the text, object is represented as `length` object replacement characters."]
        pub(crate) fn shaped_text_add_object_full(&mut self, shaped: Rid, key: RefArg < Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i64, baseline: f64,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >, Vector2, crate::global::InlineAlignment, i64, f64,);
            let args = (shaped, key, size, inline_align, length, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_add_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_text_add_object_ex`][Self::shaped_text_add_object_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds inline object to the text buffer, `key` must be unique. In the text, object is represented as `length` object replacement characters."]
        #[inline]
        pub fn shaped_text_add_object(&mut self, shaped: Rid, key: &Variant, size: Vector2,) -> bool {
            self.shaped_text_add_object_ex(shaped, key, size,) . done()
        }
        #[doc = "Adds inline object to the text buffer, `key` must be unique. In the text, object is represented as `length` object replacement characters."]
        #[inline]
        pub fn shaped_text_add_object_ex < 'ex > (&'ex mut self, shaped: Rid, key: &'ex Variant, size: Vector2,) -> ExShapedTextAddObject < 'ex > {
            ExShapedTextAddObject::new(self, shaped, key, size,)
        }
        #[doc = "Sets new size and alignment of embedded object."]
        pub(crate) fn shaped_text_resize_object_full(&mut self, shaped: Rid, key: RefArg < Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f64,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >, Vector2, crate::global::InlineAlignment, f64,);
            let args = (shaped, key, size, inline_align, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_resize_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_text_resize_object_ex`][Self::shaped_text_resize_object_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets new size and alignment of embedded object."]
        #[inline]
        pub fn shaped_text_resize_object(&mut self, shaped: Rid, key: &Variant, size: Vector2,) -> bool {
            self.shaped_text_resize_object_ex(shaped, key, size,) . done()
        }
        #[doc = "Sets new size and alignment of embedded object."]
        #[inline]
        pub fn shaped_text_resize_object_ex < 'ex > (&'ex mut self, shaped: Rid, key: &'ex Variant, size: Vector2,) -> ExShapedTextResizeObject < 'ex > {
            ExShapedTextResizeObject::new(self, shaped, key, size,)
        }
        #[doc = "Returns `true` if an object with `key` is embedded in this shaped text buffer."]
        pub fn shaped_text_has_object(&self, shaped: Rid, key: &Variant,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (shaped, RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_has_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text buffer source text, including object replacement characters."]
        pub fn shaped_get_text(&self, shaped: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of text spans added using [`shaped_text_add_string`][`crate::classes::TextServer::shaped_text_add_string`] or [`shaped_text_add_object`][`crate::classes::TextServer::shaped_text_add_object`]."]
        pub fn shaped_get_span_count(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_span_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns text span metadata."]
        pub fn shaped_get_span_meta(&self, shaped: Rid, index: i64,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7950usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_span_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns text embedded object key."]
        pub fn shaped_get_span_embedded_object(&self, shaped: Rid, index: i64,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7951usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_span_embedded_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text span source text."]
        pub fn shaped_get_span_text(&self, shaped: Rid, index: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7952usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_span_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text span embedded object key."]
        pub fn shaped_get_span_object(&self, shaped: Rid, index: i64,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7953usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_span_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Changes text span font, font size, and OpenType features, without changing the text."]
        pub(crate) fn shaped_set_span_update_font_full(&mut self, shaped: Rid, index: i64, fonts: RefArg < Array < Rid > >, size: i64, opentype_features: RefArg < AnyDictionary >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, i64, RefArg < 'a0, Array < Rid > >, i64, RefArg < 'a1, AnyDictionary >,);
            let args = (shaped, index, fonts, size, opentype_features,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7954usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_set_span_update_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_set_span_update_font_ex`][Self::shaped_set_span_update_font_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Changes text span font, font size, and OpenType features, without changing the text."]
        #[inline]
        pub fn shaped_set_span_update_font(&mut self, shaped: Rid, index: i64, fonts: &Array < Rid >, size: i64,) {
            self.shaped_set_span_update_font_ex(shaped, index, fonts, size,) . done()
        }
        #[doc = "Changes text span font, font size, and OpenType features, without changing the text."]
        #[inline]
        pub fn shaped_set_span_update_font_ex < 'ex > (&'ex mut self, shaped: Rid, index: i64, fonts: &'ex Array < Rid >, size: i64,) -> ExShapedSetSpanUpdateFont < 'ex > {
            ExShapedSetSpanUpdateFont::new(self, shaped, index, fonts, size,)
        }
        #[doc = "Returns the number of uniform text runs in the buffer."]
        pub fn shaped_get_run_count(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7955usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the source text of the `index` text run (in visual order)."]
        pub fn shaped_get_run_text(&self, shaped: Rid, index: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7956usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the source text range of the `index` text run (in visual order)."]
        pub fn shaped_get_run_range(&self, shaped: Rid, index: i64,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7957usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the font RID of the `index` text run (in visual order)."]
        pub fn shaped_get_run_font_rid(&self, shaped: Rid, index: i64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7958usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_font_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the font size of the `index` text run (in visual order)."]
        pub fn shaped_get_run_font_size(&self, shaped: Rid, index: i64,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7959usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the language of the `index` text run (in visual order)."]
        pub fn shaped_get_run_language(&self, shaped: Rid, index: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7960usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the direction of the `index` text run (in visual order)."]
        pub fn shaped_get_run_direction(&self, shaped: Rid, index: i64,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7961usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the embedded object of the `index` text run (in visual order)."]
        pub fn shaped_get_run_object(&self, shaped: Rid, index: i64,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7962usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns text buffer for the substring of the text in the `shaped` text buffer (including inline objects)."]
        pub fn shaped_text_substr(&self, shaped: Rid, start: i64, length: i64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, i64, i64,);
            let args = (shaped, start, length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7963usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_substr", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the parent buffer from which the substring originates."]
        pub fn shaped_text_get_parent(&self, shaped: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7964usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_parent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adjusts text width to fit to specified width, returns new text width."]
        pub(crate) fn shaped_text_fit_to_width_full(&mut self, shaped: Rid, width: f64, justification_flags: crate::classes::text_server::JustificationFlag,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid, f64, crate::classes::text_server::JustificationFlag,);
            let args = (shaped, width, justification_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7965usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_fit_to_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_text_fit_to_width_ex`][Self::shaped_text_fit_to_width_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adjusts text width to fit to specified width, returns new text width."]
        #[inline]
        pub fn shaped_text_fit_to_width(&mut self, shaped: Rid, width: f64,) -> f64 {
            self.shaped_text_fit_to_width_ex(shaped, width,) . done()
        }
        #[doc = "Adjusts text width to fit to specified width, returns new text width."]
        #[inline]
        pub fn shaped_text_fit_to_width_ex < 'ex > (&'ex mut self, shaped: Rid, width: f64,) -> ExShapedTextFitToWidth < 'ex > {
            ExShapedTextFitToWidth::new(self, shaped, width,)
        }
        #[doc = "Aligns shaped text to the given tab-stops."]
        pub fn shaped_text_tab_align(&mut self, shaped: Rid, tab_stops: &PackedFloat32Array,) -> f64 {
            type CallRet = f64;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, PackedFloat32Array >,);
            let args = (shaped, RefArg::new(tab_stops),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7966usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_tab_align", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Shapes buffer if it's not shaped. Returns `true` if the string is shaped successfully.\n\n**Note:** It is not necessary to call this function manually, buffer will be shaped automatically as soon as any of its output data is requested."]
        pub fn shaped_text_shape(&mut self, shaped: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7967usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if buffer is successfully shaped."]
        pub fn shaped_text_is_ready(&self, shaped: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7968usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_is_ready", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if text buffer contains any visible characters."]
        pub fn shaped_text_has_visible_chars(&self, shaped: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7969usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_has_visible_chars", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of glyphs in the visual order."]
        pub fn shaped_text_get_glyphs(&self, shaped: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7970usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns text glyphs in the logical order."]
        pub fn shaped_text_sort_logical(&mut self, shaped: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7971usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_sort_logical", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of glyphs in the buffer."]
        pub fn shaped_text_get_glyph_count(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7972usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_glyph_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns substring buffer character range in the parent buffer."]
        pub fn shaped_text_get_range(&self, shaped: Rid,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7973usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Breaks text to the lines and columns. Returns character ranges for each segment."]
        pub(crate) fn shaped_text_get_line_breaks_adv_full(&self, shaped: Rid, width: RefArg < PackedFloat32Array >, start: i64, once: bool, break_flags: crate::classes::text_server::LineBreakFlag,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, PackedFloat32Array >, i64, bool, crate::classes::text_server::LineBreakFlag,);
            let args = (shaped, width, start, once, break_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7974usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_line_breaks_adv", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_text_get_line_breaks_adv_ex`][Self::shaped_text_get_line_breaks_adv_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Breaks text to the lines and columns. Returns character ranges for each segment."]
        #[inline]
        pub fn shaped_text_get_line_breaks_adv(&self, shaped: Rid, width: &PackedFloat32Array,) -> PackedInt32Array {
            self.shaped_text_get_line_breaks_adv_ex(shaped, width,) . done()
        }
        #[doc = "Breaks text to the lines and columns. Returns character ranges for each segment."]
        #[inline]
        pub fn shaped_text_get_line_breaks_adv_ex < 'ex > (&'ex self, shaped: Rid, width: &'ex PackedFloat32Array,) -> ExShapedTextGetLineBreaksAdv < 'ex > {
            ExShapedTextGetLineBreaksAdv::new(self, shaped, width,)
        }
        #[doc = "Breaks text to the lines and returns character ranges for each line."]
        pub(crate) fn shaped_text_get_line_breaks_full(&self, shaped: Rid, width: f64, start: i64, break_flags: crate::classes::text_server::LineBreakFlag,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid, f64, i64, crate::classes::text_server::LineBreakFlag,);
            let args = (shaped, width, start, break_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7975usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_line_breaks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_text_get_line_breaks_ex`][Self::shaped_text_get_line_breaks_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Breaks text to the lines and returns character ranges for each line."]
        #[inline]
        pub fn shaped_text_get_line_breaks(&self, shaped: Rid, width: f64,) -> PackedInt32Array {
            self.shaped_text_get_line_breaks_ex(shaped, width,) . done()
        }
        #[doc = "Breaks text to the lines and returns character ranges for each line."]
        #[inline]
        pub fn shaped_text_get_line_breaks_ex < 'ex > (&'ex self, shaped: Rid, width: f64,) -> ExShapedTextGetLineBreaks < 'ex > {
            ExShapedTextGetLineBreaks::new(self, shaped, width,)
        }
        #[doc = "Breaks text into words and returns array of character ranges. Use `grapheme_flags` to set what characters are used for breaking."]
        pub(crate) fn shaped_text_get_word_breaks_full(&self, shaped: Rid, grapheme_flags: crate::classes::text_server::GraphemeFlag, skip_grapheme_flags: crate::classes::text_server::GraphemeFlag,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid, crate::classes::text_server::GraphemeFlag, crate::classes::text_server::GraphemeFlag,);
            let args = (shaped, grapheme_flags, skip_grapheme_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7976usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_word_breaks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_text_get_word_breaks_ex`][Self::shaped_text_get_word_breaks_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Breaks text into words and returns array of character ranges. Use `grapheme_flags` to set what characters are used for breaking."]
        #[inline]
        pub fn shaped_text_get_word_breaks(&self, shaped: Rid,) -> PackedInt32Array {
            self.shaped_text_get_word_breaks_ex(shaped,) . done()
        }
        #[doc = "Breaks text into words and returns array of character ranges. Use `grapheme_flags` to set what characters are used for breaking."]
        #[inline]
        pub fn shaped_text_get_word_breaks_ex < 'ex > (&'ex self, shaped: Rid,) -> ExShapedTextGetWordBreaks < 'ex > {
            ExShapedTextGetWordBreaks::new(self, shaped,)
        }
        #[doc = "Returns the position of the overrun trim."]
        pub fn shaped_text_get_trim_pos(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7977usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_trim_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns position of the ellipsis."]
        pub fn shaped_text_get_ellipsis_pos(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7978usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_ellipsis_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns array of the glyphs in the ellipsis."]
        pub fn shaped_text_get_ellipsis_glyphs(&self, shaped: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7979usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_ellipsis_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of glyphs in the ellipsis."]
        pub fn shaped_text_get_ellipsis_glyph_count(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7980usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_ellipsis_glyph_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Trims text if it exceeds the given width."]
        pub(crate) fn shaped_text_overrun_trim_to_width_full(&mut self, shaped: Rid, width: f64, overrun_trim_flags: crate::classes::text_server::TextOverrunFlag,) {
            type CallRet = ();
            type CallParams = (Rid, f64, crate::classes::text_server::TextOverrunFlag,);
            let args = (shaped, width, overrun_trim_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7981usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_overrun_trim_to_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_text_overrun_trim_to_width_ex`][Self::shaped_text_overrun_trim_to_width_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Trims text if it exceeds the given width."]
        #[inline]
        pub fn shaped_text_overrun_trim_to_width(&mut self, shaped: Rid,) {
            self.shaped_text_overrun_trim_to_width_ex(shaped,) . done()
        }
        #[doc = "Trims text if it exceeds the given width."]
        #[inline]
        pub fn shaped_text_overrun_trim_to_width_ex < 'ex > (&'ex mut self, shaped: Rid,) -> ExShapedTextOverrunTrimToWidth < 'ex > {
            ExShapedTextOverrunTrimToWidth::new(self, shaped,)
        }
        #[doc = "Returns array of inline objects."]
        pub fn shaped_text_get_objects(&self, shaped: Rid,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_objects", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns bounding rectangle of the inline object."]
        pub fn shaped_text_get_object_rect(&self, shaped: Rid, key: &Variant,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (shaped, RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_object_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the character range of the inline object."]
        pub fn shaped_text_get_object_range(&self, shaped: Rid, key: &Variant,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (shaped, RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_object_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the glyph index of the inline object."]
        pub fn shaped_text_get_object_glyph(&self, shaped: Rid, key: &Variant,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (shaped, RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_object_glyph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns size of the text."]
        pub fn shaped_text_get_size(&self, shaped: Rid,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text ascent (number of pixels above the baseline for horizontal layout or to the left of baseline for vertical).\n\n**Note:** Overall ascent can be higher than font ascent, if some glyphs are displaced from the baseline."]
        pub fn shaped_text_get_ascent(&self, shaped: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_ascent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text descent (number of pixels below the baseline for horizontal layout or to the right of baseline for vertical).\n\n**Note:** Overall descent can be higher than font descent, if some glyphs are displaced from the baseline."]
        pub fn shaped_text_get_descent(&self, shaped: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_descent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns width (for horizontal layout) or height (for vertical) of the text."]
        pub fn shaped_text_get_width(&self, shaped: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns pixel offset of the underline below the baseline."]
        pub fn shaped_text_get_underline_position(&self, shaped: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns thickness of the underline."]
        pub fn shaped_text_get_underline_thickness(&self, shaped: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns shapes of the carets corresponding to the character offset `position` in the text. Returned caret shape is 1 pixel wide rectangle."]
        pub fn shaped_text_get_carets(&self, shaped: Rid, position: i64,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid, i64,);
            let args = (shaped, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_carets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns selection rectangles for the specified character range."]
        pub fn shaped_text_get_selection(&self, shaped: Rid, start: i64, end: i64,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = (Rid, i64, i64,);
            let args = (shaped, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns grapheme index at the specified pixel offset at the baseline, or `-1` if none is found."]
        pub fn shaped_text_hit_test_grapheme(&self, shaped: Rid, coords: f64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, f64,);
            let args = (shaped, coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_hit_test_grapheme", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns caret character offset at the specified pixel offset at the baseline. This function always returns a valid position."]
        pub fn shaped_text_hit_test_position(&self, shaped: Rid, coords: f64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, f64,);
            let args = (shaped, coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_hit_test_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns composite character's bounds as offsets from the start of the line."]
        pub fn shaped_text_get_grapheme_bounds(&self, shaped: Rid, pos: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid, i64,);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_grapheme_bounds", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns grapheme end position closest to the `pos`."]
        pub fn shaped_text_next_grapheme_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64,);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_next_grapheme_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns grapheme start position closest to the `pos`."]
        pub fn shaped_text_prev_grapheme_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64,);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_prev_grapheme_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns array of the composite character boundaries."]
        pub fn shaped_text_get_character_breaks(&self, shaped: Rid,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_character_breaks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns composite character end position closest to the `pos`."]
        pub fn shaped_text_next_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64,);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_next_character_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns composite character start position closest to the `pos`."]
        pub fn shaped_text_prev_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64,);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_prev_character_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns composite character position closest to the `pos`."]
        pub fn shaped_text_closest_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64,);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_closest_character_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Draw shaped text into a canvas item at a given position, with `color`. `pos` specifies the leftmost point of the baseline (for horizontal layout) or topmost point of the baseline (for vertical layout). If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n`clip_l` and `clip_r` are offsets relative to `pos`, going to the right in horizontal layout and downward in vertical layout. If `clip_l` is not negative, glyphs starting before the offset are clipped. If `clip_r` is not negative, glyphs ending after the offset are clipped."]
        pub(crate) fn shaped_text_draw_full(&self, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Vector2, f64, f64, Color, f32,);
            let args = (shaped, canvas, pos, clip_l, clip_r, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_text_draw_ex`][Self::shaped_text_draw_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw shaped text into a canvas item at a given position, with `color`. `pos` specifies the leftmost point of the baseline (for horizontal layout) or topmost point of the baseline (for vertical layout). If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n`clip_l` and `clip_r` are offsets relative to `pos`, going to the right in horizontal layout and downward in vertical layout. If `clip_l` is not negative, glyphs starting before the offset are clipped. If `clip_r` is not negative, glyphs ending after the offset are clipped."]
        #[inline]
        pub fn shaped_text_draw(&self, shaped: Rid, canvas: Rid, pos: Vector2,) {
            self.shaped_text_draw_ex(shaped, canvas, pos,) . done()
        }
        #[doc = "Draw shaped text into a canvas item at a given position, with `color`. `pos` specifies the leftmost point of the baseline (for horizontal layout) or topmost point of the baseline (for vertical layout). If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n`clip_l` and `clip_r` are offsets relative to `pos`, going to the right in horizontal layout and downward in vertical layout. If `clip_l` is not negative, glyphs starting before the offset are clipped. If `clip_r` is not negative, glyphs ending after the offset are clipped."]
        #[inline]
        pub fn shaped_text_draw_ex < 'ex > (&'ex self, shaped: Rid, canvas: Rid, pos: Vector2,) -> ExShapedTextDraw < 'ex > {
            ExShapedTextDraw::new(self, shaped, canvas, pos,)
        }
        #[doc = "Draw the outline of the shaped text into a canvas item at a given position, with `color`. `pos` specifies the leftmost point of the baseline (for horizontal layout) or topmost point of the baseline (for vertical layout). If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n`clip_l` and `clip_r` are offsets relative to `pos`, going to the right in horizontal layout and downward in vertical layout. If `clip_l` is not negative, glyphs starting before the offset are clipped. If `clip_r` is not negative, glyphs ending after the offset are clipped."]
        pub(crate) fn shaped_text_draw_outline_full(&self, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, outline_size: i64, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Vector2, f64, f64, i64, Color, f32,);
            let args = (shaped, canvas, pos, clip_l, clip_r, outline_size, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_draw_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shaped_text_draw_outline_ex`][Self::shaped_text_draw_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draw the outline of the shaped text into a canvas item at a given position, with `color`. `pos` specifies the leftmost point of the baseline (for horizontal layout) or topmost point of the baseline (for vertical layout). If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n`clip_l` and `clip_r` are offsets relative to `pos`, going to the right in horizontal layout and downward in vertical layout. If `clip_l` is not negative, glyphs starting before the offset are clipped. If `clip_r` is not negative, glyphs ending after the offset are clipped."]
        #[inline]
        pub fn shaped_text_draw_outline(&self, shaped: Rid, canvas: Rid, pos: Vector2,) {
            self.shaped_text_draw_outline_ex(shaped, canvas, pos,) . done()
        }
        #[doc = "Draw the outline of the shaped text into a canvas item at a given position, with `color`. `pos` specifies the leftmost point of the baseline (for horizontal layout) or topmost point of the baseline (for vertical layout). If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n`clip_l` and `clip_r` are offsets relative to `pos`, going to the right in horizontal layout and downward in vertical layout. If `clip_l` is not negative, glyphs starting before the offset are clipped. If `clip_r` is not negative, glyphs ending after the offset are clipped."]
        #[inline]
        pub fn shaped_text_draw_outline_ex < 'ex > (&'ex self, shaped: Rid, canvas: Rid, pos: Vector2,) -> ExShapedTextDrawOutline < 'ex > {
            ExShapedTextDrawOutline::new(self, shaped, canvas, pos,)
        }
        #[doc = "Returns dominant direction of in the range of text."]
        pub fn shaped_text_get_dominant_direction_in_range(&self, shaped: Rid, start: i64, end: i64,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = (Rid, i64, i64,);
            let args = (shaped, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_dominant_direction_in_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts a number from Western Arabic (0..9) to the numeral system used in the given `language`.\n\nIf `language` is an empty string, the active locale will be used."]
        pub(crate) fn format_number_full(&self, number: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (number, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "format_number", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`format_number_ex`][Self::format_number_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Converts a number from Western Arabic (0..9) to the numeral system used in the given `language`.\n\nIf `language` is an empty string, the active locale will be used."]
        #[inline]
        pub fn format_number(&self, number: impl AsArg < GString >,) -> GString {
            self.format_number_ex(number,) . done()
        }
        #[doc = "Converts a number from Western Arabic (0..9) to the numeral system used in the given `language`.\n\nIf `language` is an empty string, the active locale will be used."]
        #[inline]
        pub fn format_number_ex < 'ex > (&'ex self, number: impl AsArg < GString > + 'ex,) -> ExFormatNumber < 'ex > {
            ExFormatNumber::new(self, number,)
        }
        #[doc = "Converts `number` from the numeral system used in the given `language` to Western Arabic (0..9).\n\nIf `language` is an empty string, the active locale will be used."]
        pub(crate) fn parse_number_full(&self, number: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (number, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "parse_number", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`parse_number_ex`][Self::parse_number_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Converts `number` from the numeral system used in the given `language` to Western Arabic (0..9).\n\nIf `language` is an empty string, the active locale will be used."]
        #[inline]
        pub fn parse_number(&self, number: impl AsArg < GString >,) -> GString {
            self.parse_number_ex(number,) . done()
        }
        #[doc = "Converts `number` from the numeral system used in the given `language` to Western Arabic (0..9).\n\nIf `language` is an empty string, the active locale will be used."]
        #[inline]
        pub fn parse_number_ex < 'ex > (&'ex self, number: impl AsArg < GString > + 'ex,) -> ExParseNumber < 'ex > {
            ExParseNumber::new(self, number,)
        }
        #[doc = "Returns the percent sign used in the given `language`.\n\nIf `language` is an empty string, the active locale will be used."]
        pub(crate) fn percent_sign_full(&self, language: CowArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "percent_sign", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`percent_sign_ex`][Self::percent_sign_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the percent sign used in the given `language`.\n\nIf `language` is an empty string, the active locale will be used."]
        #[inline]
        pub fn percent_sign(&self,) -> GString {
            self.percent_sign_ex() . done()
        }
        #[doc = "Returns the percent sign used in the given `language`.\n\nIf `language` is an empty string, the active locale will be used."]
        #[inline]
        pub fn percent_sign_ex < 'ex > (&'ex self,) -> ExPercentSign < 'ex > {
            ExPercentSign::new(self,)
        }
        #[doc = "Returns an array of the word break boundaries. Elements in the returned array are the offsets of the start and end of words. Therefore the length of the array is always even.\n\nWhen `chars_per_line` is greater than zero, line break boundaries are returned instead.\n\n```gdscript\nvar ts = TextServerManager.get_primary_interface()\n# Corresponds to the substrings \"The\", \"Godot\", \"Engine\", and \"4\".\nprint(ts.string_get_word_breaks(\"The Godot Engine, 4\")) # Prints [0, 3, 4, 9, 10, 16, 18, 19]\n# Corresponds to the substrings \"The\", \"Godot\", \"Engin\", and \"e, 4\".\nprint(ts.string_get_word_breaks(\"The Godot Engine, 4\", \"en\", 5)) # Prints [0, 3, 4, 9, 10, 15, 15, 19]\n# Corresponds to the substrings \"The Godot\" and \"Engine, 4\".\nprint(ts.string_get_word_breaks(\"The Godot Engine, 4\", \"en\", 10)) # Prints [0, 9, 10, 19]\n```"]
        pub(crate) fn string_get_word_breaks_full(&self, string: CowArg < GString >, language: CowArg < GString >, chars_per_line: i64,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, i64,);
            let args = (string, language, chars_per_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8009usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "string_get_word_breaks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`string_get_word_breaks_ex`][Self::string_get_word_breaks_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an array of the word break boundaries. Elements in the returned array are the offsets of the start and end of words. Therefore the length of the array is always even.\n\nWhen `chars_per_line` is greater than zero, line break boundaries are returned instead.\n\n```gdscript\nvar ts = TextServerManager.get_primary_interface()\n# Corresponds to the substrings \"The\", \"Godot\", \"Engine\", and \"4\".\nprint(ts.string_get_word_breaks(\"The Godot Engine, 4\")) # Prints [0, 3, 4, 9, 10, 16, 18, 19]\n# Corresponds to the substrings \"The\", \"Godot\", \"Engin\", and \"e, 4\".\nprint(ts.string_get_word_breaks(\"The Godot Engine, 4\", \"en\", 5)) # Prints [0, 3, 4, 9, 10, 15, 15, 19]\n# Corresponds to the substrings \"The Godot\" and \"Engine, 4\".\nprint(ts.string_get_word_breaks(\"The Godot Engine, 4\", \"en\", 10)) # Prints [0, 9, 10, 19]\n```"]
        #[inline]
        pub fn string_get_word_breaks(&self, string: impl AsArg < GString >,) -> PackedInt32Array {
            self.string_get_word_breaks_ex(string,) . done()
        }
        #[doc = "Returns an array of the word break boundaries. Elements in the returned array are the offsets of the start and end of words. Therefore the length of the array is always even.\n\nWhen `chars_per_line` is greater than zero, line break boundaries are returned instead.\n\n```gdscript\nvar ts = TextServerManager.get_primary_interface()\n# Corresponds to the substrings \"The\", \"Godot\", \"Engine\", and \"4\".\nprint(ts.string_get_word_breaks(\"The Godot Engine, 4\")) # Prints [0, 3, 4, 9, 10, 16, 18, 19]\n# Corresponds to the substrings \"The\", \"Godot\", \"Engin\", and \"e, 4\".\nprint(ts.string_get_word_breaks(\"The Godot Engine, 4\", \"en\", 5)) # Prints [0, 3, 4, 9, 10, 15, 15, 19]\n# Corresponds to the substrings \"The Godot\" and \"Engine, 4\".\nprint(ts.string_get_word_breaks(\"The Godot Engine, 4\", \"en\", 10)) # Prints [0, 9, 10, 19]\n```"]
        #[inline]
        pub fn string_get_word_breaks_ex < 'ex > (&'ex self, string: impl AsArg < GString > + 'ex,) -> ExStringGetWordBreaks < 'ex > {
            ExStringGetWordBreaks::new(self, string,)
        }
        #[doc = "Returns array of the composite character boundaries.\n\n```gdscript\nvar ts = TextServerManager.get_primary_interface()\nprint(ts.string_get_character_breaks(\"Test ❤\u{fe0f}\u{200d}🔥 Test\")) # Prints [1, 2, 3, 4, 5, 9, 10, 11, 12, 13, 14]\n```"]
        pub(crate) fn string_get_character_breaks_full(&self, string: CowArg < GString >, language: CowArg < GString >,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8010usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "string_get_character_breaks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`string_get_character_breaks_ex`][Self::string_get_character_breaks_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns array of the composite character boundaries.\n\n```gdscript\nvar ts = TextServerManager.get_primary_interface()\nprint(ts.string_get_character_breaks(\"Test ❤\u{fe0f}\u{200d}🔥 Test\")) # Prints [1, 2, 3, 4, 5, 9, 10, 11, 12, 13, 14]\n```"]
        #[inline]
        pub fn string_get_character_breaks(&self, string: impl AsArg < GString >,) -> PackedInt32Array {
            self.string_get_character_breaks_ex(string,) . done()
        }
        #[doc = "Returns array of the composite character boundaries.\n\n```gdscript\nvar ts = TextServerManager.get_primary_interface()\nprint(ts.string_get_character_breaks(\"Test ❤\u{fe0f}\u{200d}🔥 Test\")) # Prints [1, 2, 3, 4, 5, 9, 10, 11, 12, 13, 14]\n```"]
        #[inline]
        pub fn string_get_character_breaks_ex < 'ex > (&'ex self, string: impl AsArg < GString > + 'ex,) -> ExStringGetCharacterBreaks < 'ex > {
            ExStringGetCharacterBreaks::new(self, string,)
        }
        #[doc = "Returns index of the first string in `dict` which is visually confusable with the `string`, or `-1` if none is found.\n\n**Note:** This method doesn't detect invisible characters, for spoof detection use it in combination with [`spoof_check`][`crate::classes::TextServer::spoof_check`].\n\n**Note:** Always returns `-1` if the server does not support the [`Feature::UNICODE_SECURITY`][`crate::classes::text_server::Feature::UNICODE_SECURITY`] feature."]
        pub fn is_confusable(&self, string: impl AsArg < GString >, dict: &PackedStringArray,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >,);
            let args = (string.into_arg(), RefArg::new(dict),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8011usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "is_confusable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if `string` is likely to be an attempt at confusing the reader.\n\n**Note:** Always returns `false` if the server does not support the [`Feature::UNICODE_SECURITY`][`crate::classes::text_server::Feature::UNICODE_SECURITY`] feature."]
        pub fn spoof_check(&self, string: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "spoof_check", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Strips diacritics from the string.\n\n**Note:** The result may be longer or shorter than the original."]
        pub fn strip_diacritics(&self, string: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8013usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "strip_diacritics", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if `string` is a valid identifier.\n\nIf the text server supports the [`Feature::UNICODE_IDENTIFIERS`][`crate::classes::text_server::Feature::UNICODE_IDENTIFIERS`] feature, a valid identifier must:\n\n- Conform to normalization form C.\n\n- Begin with a Unicode character of class XID_Start or `\"_\"`.\n\n- May contain Unicode characters of class XID_Continue in the other positions.\n\n- Use UAX #31 recommended scripts only (mixed scripts are allowed).\n\nIf the [`Feature::UNICODE_IDENTIFIERS`][`crate::classes::text_server::Feature::UNICODE_IDENTIFIERS`] feature is not supported, a valid identifier must:\n\n- Begin with a Unicode character of class XID_Start or `\"_\"`.\n\n- May contain Unicode characters of class XID_Continue in the other positions."]
        pub fn is_valid_identifier(&self, string: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8014usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "is_valid_identifier", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given code point is a valid letter, i.e. it belongs to the Unicode category \"L\"."]
        pub fn is_valid_letter(&self, unicode: u64,) -> bool {
            type CallRet = bool;
            type CallParams = (u64,);
            let args = (unicode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8015usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "is_valid_letter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the string converted to `UPPERCASE`.\n\n**Note:** Casing is locale dependent and context sensitive if server support [`Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`][`crate::classes::text_server::Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced]).\n\n**Note:** The result may be longer or shorter than the original."]
        pub(crate) fn string_to_upper_full(&self, string: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "string_to_upper", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`string_to_upper_ex`][Self::string_to_upper_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the string converted to `UPPERCASE`.\n\n**Note:** Casing is locale dependent and context sensitive if server support [`Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`][`crate::classes::text_server::Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced]).\n\n**Note:** The result may be longer or shorter than the original."]
        #[inline]
        pub fn string_to_upper(&self, string: impl AsArg < GString >,) -> GString {
            self.string_to_upper_ex(string,) . done()
        }
        #[doc = "Returns the string converted to `UPPERCASE`.\n\n**Note:** Casing is locale dependent and context sensitive if server support [`Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`][`crate::classes::text_server::Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced]).\n\n**Note:** The result may be longer or shorter than the original."]
        #[inline]
        pub fn string_to_upper_ex < 'ex > (&'ex self, string: impl AsArg < GString > + 'ex,) -> ExStringToUpper < 'ex > {
            ExStringToUpper::new(self, string,)
        }
        #[doc = "Returns the string converted to `lowercase`.\n\n**Note:** Casing is locale dependent and context sensitive if server support [`Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`][`crate::classes::text_server::Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced]).\n\n**Note:** The result may be longer or shorter than the original."]
        pub(crate) fn string_to_lower_full(&self, string: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "string_to_lower", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`string_to_lower_ex`][Self::string_to_lower_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the string converted to `lowercase`.\n\n**Note:** Casing is locale dependent and context sensitive if server support [`Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`][`crate::classes::text_server::Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced]).\n\n**Note:** The result may be longer or shorter than the original."]
        #[inline]
        pub fn string_to_lower(&self, string: impl AsArg < GString >,) -> GString {
            self.string_to_lower_ex(string,) . done()
        }
        #[doc = "Returns the string converted to `lowercase`.\n\n**Note:** Casing is locale dependent and context sensitive if server support [`Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`][`crate::classes::text_server::Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced]).\n\n**Note:** The result may be longer or shorter than the original."]
        #[inline]
        pub fn string_to_lower_ex < 'ex > (&'ex self, string: impl AsArg < GString > + 'ex,) -> ExStringToLower < 'ex > {
            ExStringToLower::new(self, string,)
        }
        #[doc = "Returns the string converted to `Title Case`.\n\n**Note:** Casing is locale dependent and context sensitive if server support [`Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`][`crate::classes::text_server::Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced]).\n\n**Note:** The result may be longer or shorter than the original."]
        pub(crate) fn string_to_title_full(&self, string: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8018usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "string_to_title", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`string_to_title_ex`][Self::string_to_title_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the string converted to `Title Case`.\n\n**Note:** Casing is locale dependent and context sensitive if server support [`Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`][`crate::classes::text_server::Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced]).\n\n**Note:** The result may be longer or shorter than the original."]
        #[inline]
        pub fn string_to_title(&self, string: impl AsArg < GString >,) -> GString {
            self.string_to_title_ex(string,) . done()
        }
        #[doc = "Returns the string converted to `Title Case`.\n\n**Note:** Casing is locale dependent and context sensitive if server support [`Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`][`crate::classes::text_server::Feature::CONTEXT_SENSITIVE_CASE_CONVERSION`] feature (supported by [`TextServerAdvanced`][crate::classes::TextServerAdvanced]).\n\n**Note:** The result may be longer or shorter than the original."]
        #[inline]
        pub fn string_to_title_ex < 'ex > (&'ex self, string: impl AsArg < GString > + 'ex,) -> ExStringToTitle < 'ex > {
            ExStringToTitle::new(self, string,)
        }
        #[doc = "Default implementation of the BiDi algorithm override function."]
        pub fn parse_structured_text(&self, parser_type: crate::classes::text_server::StructuredTextParser, args: &AnyArray, text: impl AsArg < GString >,) -> Array < Vector3i > {
            type CallRet = Array < Vector3i >;
            type CallParams < 'a0, 'a1, > = (crate::classes::text_server::StructuredTextParser, RefArg < 'a0, AnyArray >, CowArg < 'a1, GString >,);
            let args = (parser_type, RefArg::new(args), text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8019usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "parse_structured_text", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TextServer {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextServer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TextServer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextServer {
        
    }
    impl std::ops::Deref for TextServer {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextServer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `TextServer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`TextServer::font_draw_glyph_ex`][super::TextServer::font_draw_glyph_ex]."]
#[must_use]
pub struct ExFontDrawGlyph < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExFontDrawGlyph < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64,) -> Self {
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_rid: font_rid, canvas: canvas, size: size, pos: pos, index: index, color: color, oversampling: oversampling,
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
            _phantom, surround_object, font_rid, canvas, size, pos, index, color, oversampling,
        }
        = self;
        re_export::TextServer::font_draw_glyph_full(surround_object, font_rid, canvas, size, pos, index, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextServer::font_draw_glyph_outline_ex`][super::TextServer::font_draw_glyph_outline_ex]."]
#[must_use]
pub struct ExFontDrawGlyphOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExFontDrawGlyphOutline < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64,) -> Self {
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_rid: font_rid, canvas: canvas, size: size, outline_size: outline_size, pos: pos, index: index, color: color, oversampling: oversampling,
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
            _phantom, surround_object, font_rid, canvas, size, outline_size, pos, index, color, oversampling,
        }
        = self;
        re_export::TextServer::font_draw_glyph_outline_full(surround_object, font_rid, canvas, size, outline_size, pos, index, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextServer::create_shaped_text_ex`][super::TextServer::create_shaped_text_ex]."]
#[must_use]
pub struct ExCreateShapedText < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextServer, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateShapedText < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextServer,) -> Self {
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, direction: direction, orientation: orientation,
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
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, direction, orientation,
        }
        = self;
        re_export::TextServer::create_shaped_text_full(surround_object, direction, orientation,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_set_direction_ex`][super::TextServer::shaped_text_set_direction_ex]."]
#[must_use]
pub struct ExShapedTextSetDirection < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextServer, shaped: Rid, direction: crate::classes::text_server::Direction,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedTextSetDirection < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextServer, shaped: Rid,) -> Self {
        let direction = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, direction: direction,
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shaped, direction,
        }
        = self;
        re_export::TextServer::shaped_text_set_direction_full(surround_object, shaped, direction,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_set_orientation_ex`][super::TextServer::shaped_text_set_orientation_ex]."]
#[must_use]
pub struct ExShapedTextSetOrientation < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextServer, shaped: Rid, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedTextSetOrientation < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextServer, shaped: Rid,) -> Self {
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, orientation: orientation,
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shaped, orientation,
        }
        = self;
        re_export::TextServer::shaped_text_set_orientation_full(surround_object, shaped, orientation,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_add_string_ex`][super::TextServer::shaped_text_add_string_ex]."]
#[must_use]
pub struct ExShapedTextAddString < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextServer, shaped: Rid, text: CowArg < 'ex, GString >, fonts: CowArg < 'ex, Array < Rid > >, size: i64, opentype_features: CowArg < 'ex, AnyDictionary >, language: CowArg < 'ex, GString >, meta: CowArg < 'ex, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedTextAddString < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextServer, shaped: Rid, text: impl AsArg < GString > + 'ex, fonts: &'ex Array < Rid >, size: i64,) -> Self {
        let opentype_features = AnyDictionary::new_untyped();
        let language = GString::from("");
        let meta = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, text: text.into_arg(), fonts: CowArg::Borrowed(fonts), size: size, opentype_features: CowArg::Owned(opentype_features), language: CowArg::Owned(language), meta: CowArg::Owned(meta),
        }
    }
    #[inline]
    pub fn opentype_features(self, opentype_features: &'ex AnyDictionary) -> Self {
        Self {
            opentype_features: CowArg::Borrowed(opentype_features), .. self
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
            _phantom, surround_object, shaped, text, fonts, size, opentype_features, language, meta,
        }
        = self;
        re_export::TextServer::shaped_text_add_string_full(surround_object, shaped, text, fonts.cow_as_arg(), size, opentype_features.cow_as_arg(), language, meta.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_add_object_ex`][super::TextServer::shaped_text_add_object_ex]."]
#[must_use]
pub struct ExShapedTextAddObject < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextServer, shaped: Rid, key: CowArg < 'ex, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i64, baseline: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedTextAddObject < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextServer, shaped: Rid, key: &'ex Variant, size: Vector2,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let length = 1i64;
        let baseline = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, key: CowArg::Borrowed(key), size: size, inline_align: inline_align, length: length, baseline: baseline,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn length(self, length: i64) -> Self {
        Self {
            length: length, .. self
        }
    }
    #[inline]
    pub fn baseline(self, baseline: f64) -> Self {
        Self {
            baseline: baseline, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, shaped, key, size, inline_align, length, baseline,
        }
        = self;
        re_export::TextServer::shaped_text_add_object_full(surround_object, shaped, key.cow_as_arg(), size, inline_align, length, baseline,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_resize_object_ex`][super::TextServer::shaped_text_resize_object_ex]."]
#[must_use]
pub struct ExShapedTextResizeObject < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextServer, shaped: Rid, key: CowArg < 'ex, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedTextResizeObject < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextServer, shaped: Rid, key: &'ex Variant, size: Vector2,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let baseline = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, key: CowArg::Borrowed(key), size: size, inline_align: inline_align, baseline: baseline,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn baseline(self, baseline: f64) -> Self {
        Self {
            baseline: baseline, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, shaped, key, size, inline_align, baseline,
        }
        = self;
        re_export::TextServer::shaped_text_resize_object_full(surround_object, shaped, key.cow_as_arg(), size, inline_align, baseline,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_set_span_update_font_ex`][super::TextServer::shaped_set_span_update_font_ex]."]
#[must_use]
pub struct ExShapedSetSpanUpdateFont < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextServer, shaped: Rid, index: i64, fonts: CowArg < 'ex, Array < Rid > >, size: i64, opentype_features: CowArg < 'ex, AnyDictionary >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedSetSpanUpdateFont < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextServer, shaped: Rid, index: i64, fonts: &'ex Array < Rid >, size: i64,) -> Self {
        let opentype_features = AnyDictionary::new_untyped();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, index: index, fonts: CowArg::Borrowed(fonts), size: size, opentype_features: CowArg::Owned(opentype_features),
        }
    }
    #[inline]
    pub fn opentype_features(self, opentype_features: &'ex AnyDictionary) -> Self {
        Self {
            opentype_features: CowArg::Borrowed(opentype_features), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shaped, index, fonts, size, opentype_features,
        }
        = self;
        re_export::TextServer::shaped_set_span_update_font_full(surround_object, shaped, index, fonts.cow_as_arg(), size, opentype_features.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_fit_to_width_ex`][super::TextServer::shaped_text_fit_to_width_ex]."]
#[must_use]
pub struct ExShapedTextFitToWidth < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextServer, shaped: Rid, width: f64, justification_flags: crate::classes::text_server::JustificationFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedTextFitToWidth < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextServer, shaped: Rid, width: f64,) -> Self {
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, width: width, justification_flags: justification_flags,
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f64 {
        let Self {
            _phantom, surround_object, shaped, width, justification_flags,
        }
        = self;
        re_export::TextServer::shaped_text_fit_to_width_full(surround_object, shaped, width, justification_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_get_line_breaks_adv_ex`][super::TextServer::shaped_text_get_line_breaks_adv_ex]."]
#[must_use]
pub struct ExShapedTextGetLineBreaksAdv < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, shaped: Rid, width: CowArg < 'ex, PackedFloat32Array >, start: i64, once: bool, break_flags: crate::classes::text_server::LineBreakFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedTextGetLineBreaksAdv < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, shaped: Rid, width: &'ex PackedFloat32Array,) -> Self {
        let start = 0i64;
        let once = true;
        let break_flags = crate::obj::EngineBitfield::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, width: CowArg::Borrowed(width), start: start, once: once, break_flags: break_flags,
        }
    }
    #[inline]
    pub fn start(self, start: i64) -> Self {
        Self {
            start: start, .. self
        }
    }
    #[inline]
    pub fn once(self, once: bool) -> Self {
        Self {
            once: once, .. self
        }
    }
    #[inline]
    pub fn break_flags(self, break_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            break_flags: break_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, shaped, width, start, once, break_flags,
        }
        = self;
        re_export::TextServer::shaped_text_get_line_breaks_adv_full(surround_object, shaped, width.cow_as_arg(), start, once, break_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_get_line_breaks_ex`][super::TextServer::shaped_text_get_line_breaks_ex]."]
#[must_use]
pub struct ExShapedTextGetLineBreaks < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, shaped: Rid, width: f64, start: i64, break_flags: crate::classes::text_server::LineBreakFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedTextGetLineBreaks < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, shaped: Rid, width: f64,) -> Self {
        let start = 0i64;
        let break_flags = crate::obj::EngineBitfield::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, width: width, start: start, break_flags: break_flags,
        }
    }
    #[inline]
    pub fn start(self, start: i64) -> Self {
        Self {
            start: start, .. self
        }
    }
    #[inline]
    pub fn break_flags(self, break_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            break_flags: break_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, shaped, width, start, break_flags,
        }
        = self;
        re_export::TextServer::shaped_text_get_line_breaks_full(surround_object, shaped, width, start, break_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_get_word_breaks_ex`][super::TextServer::shaped_text_get_word_breaks_ex]."]
#[must_use]
pub struct ExShapedTextGetWordBreaks < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, shaped: Rid, grapheme_flags: crate::classes::text_server::GraphemeFlag, skip_grapheme_flags: crate::classes::text_server::GraphemeFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedTextGetWordBreaks < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, shaped: Rid,) -> Self {
        let grapheme_flags = crate::obj::EngineBitfield::from_ord(264);
        let skip_grapheme_flags = crate::obj::EngineBitfield::from_ord(4);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, grapheme_flags: grapheme_flags, skip_grapheme_flags: skip_grapheme_flags,
        }
    }
    #[inline]
    pub fn grapheme_flags(self, grapheme_flags: crate::classes::text_server::GraphemeFlag) -> Self {
        Self {
            grapheme_flags: grapheme_flags, .. self
        }
    }
    #[inline]
    pub fn skip_grapheme_flags(self, skip_grapheme_flags: crate::classes::text_server::GraphemeFlag) -> Self {
        Self {
            skip_grapheme_flags: skip_grapheme_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, shaped, grapheme_flags, skip_grapheme_flags,
        }
        = self;
        re_export::TextServer::shaped_text_get_word_breaks_full(surround_object, shaped, grapheme_flags, skip_grapheme_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_overrun_trim_to_width_ex`][super::TextServer::shaped_text_overrun_trim_to_width_ex]."]
#[must_use]
pub struct ExShapedTextOverrunTrimToWidth < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextServer, shaped: Rid, width: f64, overrun_trim_flags: crate::classes::text_server::TextOverrunFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedTextOverrunTrimToWidth < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextServer, shaped: Rid,) -> Self {
        let width = 0f64;
        let overrun_trim_flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, width: width, overrun_trim_flags: overrun_trim_flags,
        }
    }
    #[inline]
    pub fn width(self, width: f64) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn overrun_trim_flags(self, overrun_trim_flags: crate::classes::text_server::TextOverrunFlag) -> Self {
        Self {
            overrun_trim_flags: overrun_trim_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shaped, width, overrun_trim_flags,
        }
        = self;
        re_export::TextServer::shaped_text_overrun_trim_to_width_full(surround_object, shaped, width, overrun_trim_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_draw_ex`][super::TextServer::shaped_text_draw_ex]."]
#[must_use]
pub struct ExShapedTextDraw < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedTextDraw < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2,) -> Self {
        let clip_l = - 1f64;
        let clip_r = - 1f64;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, canvas: canvas, pos: pos, clip_l: clip_l, clip_r: clip_r, color: color, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn clip_l(self, clip_l: f64) -> Self {
        Self {
            clip_l: clip_l, .. self
        }
    }
    #[inline]
    pub fn clip_r(self, clip_r: f64) -> Self {
        Self {
            clip_r: clip_r, .. self
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
            _phantom, surround_object, shaped, canvas, pos, clip_l, clip_r, color, oversampling,
        }
        = self;
        re_export::TextServer::shaped_text_draw_full(surround_object, shaped, canvas, pos, clip_l, clip_r, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_draw_outline_ex`][super::TextServer::shaped_text_draw_outline_ex]."]
#[must_use]
pub struct ExShapedTextDrawOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, outline_size: i64, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShapedTextDrawOutline < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2,) -> Self {
        let clip_l = - 1f64;
        let clip_r = - 1f64;
        let outline_size = 1i64;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, canvas: canvas, pos: pos, clip_l: clip_l, clip_r: clip_r, outline_size: outline_size, color: color, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn clip_l(self, clip_l: f64) -> Self {
        Self {
            clip_l: clip_l, .. self
        }
    }
    #[inline]
    pub fn clip_r(self, clip_r: f64) -> Self {
        Self {
            clip_r: clip_r, .. self
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i64) -> Self {
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
            _phantom, surround_object, shaped, canvas, pos, clip_l, clip_r, outline_size, color, oversampling,
        }
        = self;
        re_export::TextServer::shaped_text_draw_outline_full(surround_object, shaped, canvas, pos, clip_l, clip_r, outline_size, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextServer::format_number_ex`][super::TextServer::format_number_ex]."]
#[must_use]
pub struct ExFormatNumber < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, number: CowArg < 'ex, GString >, language: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExFormatNumber < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, number: impl AsArg < GString > + 'ex,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, number: number.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'ex) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, number, language,
        }
        = self;
        re_export::TextServer::format_number_full(surround_object, number, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::parse_number_ex`][super::TextServer::parse_number_ex]."]
#[must_use]
pub struct ExParseNumber < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, number: CowArg < 'ex, GString >, language: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExParseNumber < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, number: impl AsArg < GString > + 'ex,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, number: number.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'ex) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, number, language,
        }
        = self;
        re_export::TextServer::parse_number_full(surround_object, number, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::percent_sign_ex`][super::TextServer::percent_sign_ex]."]
#[must_use]
pub struct ExPercentSign < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, language: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPercentSign < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'ex) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, language,
        }
        = self;
        re_export::TextServer::percent_sign_full(surround_object, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_get_word_breaks_ex`][super::TextServer::string_get_word_breaks_ex]."]
#[must_use]
pub struct ExStringGetWordBreaks < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, string: CowArg < 'ex, GString >, language: CowArg < 'ex, GString >, chars_per_line: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExStringGetWordBreaks < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, string: impl AsArg < GString > + 'ex,) -> Self {
        let language = GString::from("");
        let chars_per_line = 0i64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language), chars_per_line: chars_per_line,
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'ex) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn chars_per_line(self, chars_per_line: i64) -> Self {
        Self {
            chars_per_line: chars_per_line, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, string, language, chars_per_line,
        }
        = self;
        re_export::TextServer::string_get_word_breaks_full(surround_object, string, language, chars_per_line,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_get_character_breaks_ex`][super::TextServer::string_get_character_breaks_ex]."]
#[must_use]
pub struct ExStringGetCharacterBreaks < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, string: CowArg < 'ex, GString >, language: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExStringGetCharacterBreaks < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, string: impl AsArg < GString > + 'ex,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'ex) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, string, language,
        }
        = self;
        re_export::TextServer::string_get_character_breaks_full(surround_object, string, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_to_upper_ex`][super::TextServer::string_to_upper_ex]."]
#[must_use]
pub struct ExStringToUpper < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, string: CowArg < 'ex, GString >, language: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExStringToUpper < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, string: impl AsArg < GString > + 'ex,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'ex) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, string, language,
        }
        = self;
        re_export::TextServer::string_to_upper_full(surround_object, string, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_to_lower_ex`][super::TextServer::string_to_lower_ex]."]
#[must_use]
pub struct ExStringToLower < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, string: CowArg < 'ex, GString >, language: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExStringToLower < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, string: impl AsArg < GString > + 'ex,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'ex) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, string, language,
        }
        = self;
        re_export::TextServer::string_to_lower_full(surround_object, string, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_to_title_ex`][super::TextServer::string_to_title_ex]."]
#[must_use]
pub struct ExStringToTitle < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextServer, string: CowArg < 'ex, GString >, language: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExStringToTitle < 'ex > {
    fn new(surround_object: &'ex re_export::TextServer, string: impl AsArg < GString > + 'ex,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'ex) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, string, language,
        }
        = self;
        re_export::TextServer::string_to_title_full(surround_object, string, language,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FontAntialiasing {
    ord: i32
}
impl FontAntialiasing {
    #[doc(alias = "FONT_ANTIALIASING_NONE")]
    #[doc = "Godot enumerator name: `FONT_ANTIALIASING_NONE`"]
    pub const NONE: FontAntialiasing = FontAntialiasing {
        ord: 0i32
    };
    #[doc(alias = "FONT_ANTIALIASING_GRAY")]
    #[doc = "Godot enumerator name: `FONT_ANTIALIASING_GRAY`"]
    pub const GRAY: FontAntialiasing = FontAntialiasing {
        ord: 1i32
    };
    #[doc(alias = "FONT_ANTIALIASING_LCD")]
    #[doc = "Godot enumerator name: `FONT_ANTIALIASING_LCD`"]
    pub const LCD: FontAntialiasing = FontAntialiasing {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for FontAntialiasing {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FontAntialiasing") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FontAntialiasing {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "NONE", Self::GRAY => "GRAY", Self::LCD => "LCD", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FontAntialiasing::NONE, FontAntialiasing::GRAY, FontAntialiasing::LCD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FontAntialiasing >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "FONT_ANTIALIASING_NONE", FontAntialiasing::NONE), crate::meta::inspect::EnumConstant::new("GRAY", "FONT_ANTIALIASING_GRAY", FontAntialiasing::GRAY), crate::meta::inspect::EnumConstant::new("LCD", "FONT_ANTIALIASING_LCD", FontAntialiasing::LCD)]
        }
    }
}
impl crate::meta::GodotConvert for FontAntialiasing {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Font Antialiasing None", 0i64), EnumeratorShape::new_int("Font Antialiasing Gray", 1i64), EnumeratorShape::new_int("Font Antialiasing Lcd", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.FontAntialiasing")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for FontAntialiasing {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FontAntialiasing {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for FontAntialiasing {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for FontAntialiasing {
    
}
impl crate::meta::Element for FontAntialiasing {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `FontLCDSubpixelLayout`."]
pub struct FontLcdSubpixelLayout {
    ord: i32
}
impl FontLcdSubpixelLayout {
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_NONE")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_NONE`"]
    pub const NONE: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 0i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_HRGB")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_HRGB`"]
    pub const HRGB: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 1i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_HBGR")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_HBGR`"]
    pub const HBGR: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 2i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_VRGB")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_VRGB`"]
    pub const VRGB: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 3i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_VBGR")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_VBGR`"]
    pub const VBGR: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 4i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_MAX")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_MAX`"]
    pub const MAX: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for FontLcdSubpixelLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FontLcdSubpixelLayout") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FontLcdSubpixelLayout {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "NONE", Self::HRGB => "HRGB", Self::HBGR => "HBGR", Self::VRGB => "VRGB", Self::VBGR => "VBGR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FontLcdSubpixelLayout::NONE, FontLcdSubpixelLayout::HRGB, FontLcdSubpixelLayout::HBGR, FontLcdSubpixelLayout::VRGB, FontLcdSubpixelLayout::VBGR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FontLcdSubpixelLayout >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "FONT_LCD_SUBPIXEL_LAYOUT_NONE", FontLcdSubpixelLayout::NONE), crate::meta::inspect::EnumConstant::new("HRGB", "FONT_LCD_SUBPIXEL_LAYOUT_HRGB", FontLcdSubpixelLayout::HRGB), crate::meta::inspect::EnumConstant::new("HBGR", "FONT_LCD_SUBPIXEL_LAYOUT_HBGR", FontLcdSubpixelLayout::HBGR), crate::meta::inspect::EnumConstant::new("VRGB", "FONT_LCD_SUBPIXEL_LAYOUT_VRGB", FontLcdSubpixelLayout::VRGB), crate::meta::inspect::EnumConstant::new("VBGR", "FONT_LCD_SUBPIXEL_LAYOUT_VBGR", FontLcdSubpixelLayout::VBGR), crate::meta::inspect::EnumConstant::new("MAX", "FONT_LCD_SUBPIXEL_LAYOUT_MAX", FontLcdSubpixelLayout::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for FontLcdSubpixelLayout {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for FontLcdSubpixelLayout {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Font Lcd Subpixel Layout None", 0i64), EnumeratorShape::new_int("Font Lcd Subpixel Layout Hrgb", 1i64), EnumeratorShape::new_int("Font Lcd Subpixel Layout Hbgr", 2i64), EnumeratorShape::new_int("Font Lcd Subpixel Layout Vrgb", 3i64), EnumeratorShape::new_int("Font Lcd Subpixel Layout Vbgr", 4i64), EnumeratorShape::new_int("Font Lcd Subpixel Layout Max", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.FontLCDSubpixelLayout")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for FontLcdSubpixelLayout {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FontLcdSubpixelLayout {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for FontLcdSubpixelLayout {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for FontLcdSubpixelLayout {
    
}
impl crate::meta::Element for FontLcdSubpixelLayout {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Direction {
    ord: i32
}
impl Direction {
    #[doc(alias = "DIRECTION_AUTO")]
    #[doc = "Godot enumerator name: `DIRECTION_AUTO`"]
    pub const AUTO: Direction = Direction {
        ord: 0i32
    };
    #[doc(alias = "DIRECTION_LTR")]
    #[doc = "Godot enumerator name: `DIRECTION_LTR`"]
    pub const LTR: Direction = Direction {
        ord: 1i32
    };
    #[doc(alias = "DIRECTION_RTL")]
    #[doc = "Godot enumerator name: `DIRECTION_RTL`"]
    pub const RTL: Direction = Direction {
        ord: 2i32
    };
    #[doc(alias = "DIRECTION_INHERITED")]
    #[doc = "Godot enumerator name: `DIRECTION_INHERITED`"]
    pub const INHERITED: Direction = Direction {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Direction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Direction {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::AUTO => "AUTO", Self::LTR => "LTR", Self::RTL => "RTL", Self::INHERITED => "INHERITED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Direction::AUTO, Direction::LTR, Direction::RTL, Direction::INHERITED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Direction >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("AUTO", "DIRECTION_AUTO", Direction::AUTO), crate::meta::inspect::EnumConstant::new("LTR", "DIRECTION_LTR", Direction::LTR), crate::meta::inspect::EnumConstant::new("RTL", "DIRECTION_RTL", Direction::RTL), crate::meta::inspect::EnumConstant::new("INHERITED", "DIRECTION_INHERITED", Direction::INHERITED)]
        }
    }
}
impl crate::meta::GodotConvert for Direction {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Direction Auto", 0i64), EnumeratorShape::new_int("Direction Ltr", 1i64), EnumeratorShape::new_int("Direction Rtl", 2i64), EnumeratorShape::new_int("Direction Inherited", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.Direction")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Direction {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Direction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Direction {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for Direction {
    
}
impl crate::meta::Element for Direction {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Orientation {
    ord: i32
}
impl Orientation {
    #[doc(alias = "ORIENTATION_HORIZONTAL")]
    #[doc = "Godot enumerator name: `ORIENTATION_HORIZONTAL`"]
    pub const HORIZONTAL: Orientation = Orientation {
        ord: 0i32
    };
    #[doc(alias = "ORIENTATION_VERTICAL")]
    #[doc = "Godot enumerator name: `ORIENTATION_VERTICAL`"]
    pub const VERTICAL: Orientation = Orientation {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Orientation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Orientation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::HORIZONTAL => "HORIZONTAL", Self::VERTICAL => "VERTICAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Orientation::HORIZONTAL, Orientation::VERTICAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Orientation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("HORIZONTAL", "ORIENTATION_HORIZONTAL", Orientation::HORIZONTAL), crate::meta::inspect::EnumConstant::new("VERTICAL", "ORIENTATION_VERTICAL", Orientation::VERTICAL)]
        }
    }
}
impl crate::meta::GodotConvert for Orientation {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Orientation Horizontal", 0i64), EnumeratorShape::new_int("Orientation Vertical", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.Orientation")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Orientation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Orientation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Orientation {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for Orientation {
    
}
impl crate::meta::Element for Orientation {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct JustificationFlag {
    ord: u64
}
impl JustificationFlag {
    #[doc(alias = "JUSTIFICATION_NONE")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_NONE`"]
    pub const NONE: JustificationFlag = JustificationFlag {
        ord: 0u64
    };
    #[doc(alias = "JUSTIFICATION_KASHIDA")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_KASHIDA`"]
    pub const KASHIDA: JustificationFlag = JustificationFlag {
        ord: 1u64
    };
    #[doc(alias = "JUSTIFICATION_WORD_BOUND")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_WORD_BOUND`"]
    pub const WORD_BOUND: JustificationFlag = JustificationFlag {
        ord: 2u64
    };
    #[doc(alias = "JUSTIFICATION_TRIM_EDGE_SPACES")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_TRIM_EDGE_SPACES`"]
    pub const TRIM_EDGE_SPACES: JustificationFlag = JustificationFlag {
        ord: 4u64
    };
    #[doc(alias = "JUSTIFICATION_AFTER_LAST_TAB")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_AFTER_LAST_TAB`"]
    pub const AFTER_LAST_TAB: JustificationFlag = JustificationFlag {
        ord: 8u64
    };
    #[doc(alias = "JUSTIFICATION_CONSTRAIN_ELLIPSIS")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_CONSTRAIN_ELLIPSIS`"]
    pub const CONSTRAIN_ELLIPSIS: JustificationFlag = JustificationFlag {
        ord: 16u64
    };
    #[doc(alias = "JUSTIFICATION_SKIP_LAST_LINE")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_SKIP_LAST_LINE`"]
    pub const SKIP_LAST_LINE: JustificationFlag = JustificationFlag {
        ord: 32u64
    };
    #[doc(alias = "JUSTIFICATION_SKIP_LAST_LINE_WITH_VISIBLE_CHARS")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_SKIP_LAST_LINE_WITH_VISIBLE_CHARS`"]
    pub const SKIP_LAST_LINE_WITH_VISIBLE_CHARS: JustificationFlag = JustificationFlag {
        ord: 64u64
    };
    #[doc(alias = "JUSTIFICATION_DO_NOT_SKIP_SINGLE_LINE")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_DO_NOT_SKIP_SINGLE_LINE`"]
    pub const DO_NOT_SKIP_SINGLE_LINE: JustificationFlag = JustificationFlag {
        ord: 128u64
    };
    
}
impl std::fmt::Debug for JustificationFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for JustificationFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < JustificationFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "JUSTIFICATION_NONE", JustificationFlag::NONE), crate::meta::inspect::EnumConstant::new("KASHIDA", "JUSTIFICATION_KASHIDA", JustificationFlag::KASHIDA), crate::meta::inspect::EnumConstant::new("WORD_BOUND", "JUSTIFICATION_WORD_BOUND", JustificationFlag::WORD_BOUND), crate::meta::inspect::EnumConstant::new("TRIM_EDGE_SPACES", "JUSTIFICATION_TRIM_EDGE_SPACES", JustificationFlag::TRIM_EDGE_SPACES), crate::meta::inspect::EnumConstant::new("AFTER_LAST_TAB", "JUSTIFICATION_AFTER_LAST_TAB", JustificationFlag::AFTER_LAST_TAB), crate::meta::inspect::EnumConstant::new("CONSTRAIN_ELLIPSIS", "JUSTIFICATION_CONSTRAIN_ELLIPSIS", JustificationFlag::CONSTRAIN_ELLIPSIS), crate::meta::inspect::EnumConstant::new("SKIP_LAST_LINE", "JUSTIFICATION_SKIP_LAST_LINE", JustificationFlag::SKIP_LAST_LINE), crate::meta::inspect::EnumConstant::new("SKIP_LAST_LINE_WITH_VISIBLE_CHARS", "JUSTIFICATION_SKIP_LAST_LINE_WITH_VISIBLE_CHARS", JustificationFlag::SKIP_LAST_LINE_WITH_VISIBLE_CHARS), crate::meta::inspect::EnumConstant::new("DO_NOT_SKIP_SINGLE_LINE", "JUSTIFICATION_DO_NOT_SKIP_SINGLE_LINE", JustificationFlag::DO_NOT_SKIP_SINGLE_LINE)]
        }
    }
}
impl std::ops::BitOr for JustificationFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for JustificationFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for JustificationFlag {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Justification None", 0i64), EnumeratorShape::new_int("Justification Kashida", 1i64), EnumeratorShape::new_int("Justification Word Bound", 2i64), EnumeratorShape::new_int("Justification Trim Edge Spaces", 4i64), EnumeratorShape::new_int("Justification After Last Tab", 8i64), EnumeratorShape::new_int("Justification Constrain Ellipsis", 16i64), EnumeratorShape::new_int("Justification Skip Last Line", 32i64), EnumeratorShape::new_int("Justification Skip Last Line With Visible Chars", 64i64), EnumeratorShape::new_int("Justification Do Not Skip Single Line", 128i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.JustificationFlag")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for JustificationFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for JustificationFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for JustificationFlag {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for JustificationFlag {
    
}
impl crate::meta::Element for JustificationFlag {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AutowrapMode {
    ord: i32
}
impl AutowrapMode {
    #[doc(alias = "AUTOWRAP_OFF")]
    #[doc = "Godot enumerator name: `AUTOWRAP_OFF`"]
    pub const OFF: AutowrapMode = AutowrapMode {
        ord: 0i32
    };
    #[doc(alias = "AUTOWRAP_ARBITRARY")]
    #[doc = "Godot enumerator name: `AUTOWRAP_ARBITRARY`"]
    pub const ARBITRARY: AutowrapMode = AutowrapMode {
        ord: 1i32
    };
    #[doc(alias = "AUTOWRAP_WORD")]
    #[doc = "Godot enumerator name: `AUTOWRAP_WORD`"]
    pub const WORD: AutowrapMode = AutowrapMode {
        ord: 2i32
    };
    #[doc(alias = "AUTOWRAP_WORD_SMART")]
    #[doc = "Godot enumerator name: `AUTOWRAP_WORD_SMART`"]
    pub const WORD_SMART: AutowrapMode = AutowrapMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AutowrapMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AutowrapMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AutowrapMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OFF => "OFF", Self::ARBITRARY => "ARBITRARY", Self::WORD => "WORD", Self::WORD_SMART => "WORD_SMART", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AutowrapMode::OFF, AutowrapMode::ARBITRARY, AutowrapMode::WORD, AutowrapMode::WORD_SMART]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AutowrapMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OFF", "AUTOWRAP_OFF", AutowrapMode::OFF), crate::meta::inspect::EnumConstant::new("ARBITRARY", "AUTOWRAP_ARBITRARY", AutowrapMode::ARBITRARY), crate::meta::inspect::EnumConstant::new("WORD", "AUTOWRAP_WORD", AutowrapMode::WORD), crate::meta::inspect::EnumConstant::new("WORD_SMART", "AUTOWRAP_WORD_SMART", AutowrapMode::WORD_SMART)]
        }
    }
}
impl crate::meta::GodotConvert for AutowrapMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Autowrap Off", 0i64), EnumeratorShape::new_int("Autowrap Arbitrary", 1i64), EnumeratorShape::new_int("Autowrap Word", 2i64), EnumeratorShape::new_int("Autowrap Word Smart", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.AutowrapMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AutowrapMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AutowrapMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AutowrapMode {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for AutowrapMode {
    
}
impl crate::meta::Element for AutowrapMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct LineBreakFlag {
    ord: u64
}
impl LineBreakFlag {
    #[doc(alias = "BREAK_NONE")]
    #[doc = "Godot enumerator name: `BREAK_NONE`"]
    pub const NONE: LineBreakFlag = LineBreakFlag {
        ord: 0u64
    };
    #[doc(alias = "BREAK_MANDATORY")]
    #[doc = "Godot enumerator name: `BREAK_MANDATORY`"]
    pub const MANDATORY: LineBreakFlag = LineBreakFlag {
        ord: 1u64
    };
    #[doc(alias = "BREAK_WORD_BOUND")]
    #[doc = "Godot enumerator name: `BREAK_WORD_BOUND`"]
    pub const WORD_BOUND: LineBreakFlag = LineBreakFlag {
        ord: 2u64
    };
    #[doc(alias = "BREAK_GRAPHEME_BOUND")]
    #[doc = "Godot enumerator name: `BREAK_GRAPHEME_BOUND`"]
    pub const GRAPHEME_BOUND: LineBreakFlag = LineBreakFlag {
        ord: 4u64
    };
    #[doc(alias = "BREAK_ADAPTIVE")]
    #[doc = "Godot enumerator name: `BREAK_ADAPTIVE`"]
    pub const ADAPTIVE: LineBreakFlag = LineBreakFlag {
        ord: 8u64
    };
    #[doc(alias = "BREAK_TRIM_EDGE_SPACES")]
    #[doc = "Godot enumerator name: `BREAK_TRIM_EDGE_SPACES`"]
    pub const TRIM_EDGE_SPACES: LineBreakFlag = LineBreakFlag {
        ord: 16u64
    };
    #[doc(alias = "BREAK_TRIM_INDENT")]
    #[doc = "Godot enumerator name: `BREAK_TRIM_INDENT`"]
    pub const TRIM_INDENT: LineBreakFlag = LineBreakFlag {
        ord: 32u64
    };
    #[doc(alias = "BREAK_TRIM_START_EDGE_SPACES")]
    #[doc = "Godot enumerator name: `BREAK_TRIM_START_EDGE_SPACES`"]
    pub const TRIM_START_EDGE_SPACES: LineBreakFlag = LineBreakFlag {
        ord: 64u64
    };
    #[doc(alias = "BREAK_TRIM_END_EDGE_SPACES")]
    #[doc = "Godot enumerator name: `BREAK_TRIM_END_EDGE_SPACES`"]
    pub const TRIM_END_EDGE_SPACES: LineBreakFlag = LineBreakFlag {
        ord: 128u64
    };
    
}
impl std::fmt::Debug for LineBreakFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for LineBreakFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LineBreakFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "BREAK_NONE", LineBreakFlag::NONE), crate::meta::inspect::EnumConstant::new("MANDATORY", "BREAK_MANDATORY", LineBreakFlag::MANDATORY), crate::meta::inspect::EnumConstant::new("WORD_BOUND", "BREAK_WORD_BOUND", LineBreakFlag::WORD_BOUND), crate::meta::inspect::EnumConstant::new("GRAPHEME_BOUND", "BREAK_GRAPHEME_BOUND", LineBreakFlag::GRAPHEME_BOUND), crate::meta::inspect::EnumConstant::new("ADAPTIVE", "BREAK_ADAPTIVE", LineBreakFlag::ADAPTIVE), crate::meta::inspect::EnumConstant::new("TRIM_EDGE_SPACES", "BREAK_TRIM_EDGE_SPACES", LineBreakFlag::TRIM_EDGE_SPACES), crate::meta::inspect::EnumConstant::new("TRIM_INDENT", "BREAK_TRIM_INDENT", LineBreakFlag::TRIM_INDENT), crate::meta::inspect::EnumConstant::new("TRIM_START_EDGE_SPACES", "BREAK_TRIM_START_EDGE_SPACES", LineBreakFlag::TRIM_START_EDGE_SPACES), crate::meta::inspect::EnumConstant::new("TRIM_END_EDGE_SPACES", "BREAK_TRIM_END_EDGE_SPACES", LineBreakFlag::TRIM_END_EDGE_SPACES)]
        }
    }
}
impl std::ops::BitOr for LineBreakFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for LineBreakFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for LineBreakFlag {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Break None", 0i64), EnumeratorShape::new_int("Break Mandatory", 1i64), EnumeratorShape::new_int("Break Word Bound", 2i64), EnumeratorShape::new_int("Break Grapheme Bound", 4i64), EnumeratorShape::new_int("Break Adaptive", 8i64), EnumeratorShape::new_int("Break Trim Edge Spaces", 16i64), EnumeratorShape::new_int("Break Trim Indent", 32i64), EnumeratorShape::new_int("Break Trim Start Edge Spaces", 64i64), EnumeratorShape::new_int("Break Trim End Edge Spaces", 128i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.LineBreakFlag")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for LineBreakFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LineBreakFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LineBreakFlag {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for LineBreakFlag {
    
}
impl crate::meta::Element for LineBreakFlag {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibleCharactersBehavior {
    ord: i32
}
impl VisibleCharactersBehavior {
    #[doc(alias = "VC_CHARS_BEFORE_SHAPING")]
    #[doc = "Godot enumerator name: `VC_CHARS_BEFORE_SHAPING`"]
    pub const CHARS_BEFORE_SHAPING: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 0i32
    };
    #[doc(alias = "VC_CHARS_AFTER_SHAPING")]
    #[doc = "Godot enumerator name: `VC_CHARS_AFTER_SHAPING`"]
    pub const CHARS_AFTER_SHAPING: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 1i32
    };
    #[doc(alias = "VC_GLYPHS_AUTO")]
    #[doc = "Godot enumerator name: `VC_GLYPHS_AUTO`"]
    pub const GLYPHS_AUTO: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 2i32
    };
    #[doc(alias = "VC_GLYPHS_LTR")]
    #[doc = "Godot enumerator name: `VC_GLYPHS_LTR`"]
    pub const GLYPHS_LTR: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 3i32
    };
    #[doc(alias = "VC_GLYPHS_RTL")]
    #[doc = "Godot enumerator name: `VC_GLYPHS_RTL`"]
    pub const GLYPHS_RTL: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for VisibleCharactersBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibleCharactersBehavior") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibleCharactersBehavior {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::CHARS_BEFORE_SHAPING => "CHARS_BEFORE_SHAPING", Self::CHARS_AFTER_SHAPING => "CHARS_AFTER_SHAPING", Self::GLYPHS_AUTO => "GLYPHS_AUTO", Self::GLYPHS_LTR => "GLYPHS_LTR", Self::GLYPHS_RTL => "GLYPHS_RTL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VisibleCharactersBehavior::CHARS_BEFORE_SHAPING, VisibleCharactersBehavior::CHARS_AFTER_SHAPING, VisibleCharactersBehavior::GLYPHS_AUTO, VisibleCharactersBehavior::GLYPHS_LTR, VisibleCharactersBehavior::GLYPHS_RTL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VisibleCharactersBehavior >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CHARS_BEFORE_SHAPING", "VC_CHARS_BEFORE_SHAPING", VisibleCharactersBehavior::CHARS_BEFORE_SHAPING), crate::meta::inspect::EnumConstant::new("CHARS_AFTER_SHAPING", "VC_CHARS_AFTER_SHAPING", VisibleCharactersBehavior::CHARS_AFTER_SHAPING), crate::meta::inspect::EnumConstant::new("GLYPHS_AUTO", "VC_GLYPHS_AUTO", VisibleCharactersBehavior::GLYPHS_AUTO), crate::meta::inspect::EnumConstant::new("GLYPHS_LTR", "VC_GLYPHS_LTR", VisibleCharactersBehavior::GLYPHS_LTR), crate::meta::inspect::EnumConstant::new("GLYPHS_RTL", "VC_GLYPHS_RTL", VisibleCharactersBehavior::GLYPHS_RTL)]
        }
    }
}
impl crate::meta::GodotConvert for VisibleCharactersBehavior {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Vc Chars Before Shaping", 0i64), EnumeratorShape::new_int("Vc Chars After Shaping", 1i64), EnumeratorShape::new_int("Vc Glyphs Auto", 2i64), EnumeratorShape::new_int("Vc Glyphs Ltr", 3i64), EnumeratorShape::new_int("Vc Glyphs Rtl", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.VisibleCharactersBehavior")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for VisibleCharactersBehavior {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibleCharactersBehavior {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for VisibleCharactersBehavior {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for VisibleCharactersBehavior {
    
}
impl crate::meta::Element for VisibleCharactersBehavior {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OverrunBehavior {
    ord: i32
}
impl OverrunBehavior {
    #[doc(alias = "OVERRUN_NO_TRIMMING")]
    #[doc = "Godot enumerator name: `OVERRUN_NO_TRIMMING`"]
    pub const NO_TRIMMING: OverrunBehavior = OverrunBehavior {
        ord: 0i32
    };
    #[doc(alias = "OVERRUN_TRIM_CHAR")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_CHAR`"]
    pub const TRIM_CHAR: OverrunBehavior = OverrunBehavior {
        ord: 1i32
    };
    #[doc(alias = "OVERRUN_TRIM_WORD")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_WORD`"]
    pub const TRIM_WORD: OverrunBehavior = OverrunBehavior {
        ord: 2i32
    };
    #[doc(alias = "OVERRUN_TRIM_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_ELLIPSIS`"]
    pub const TRIM_ELLIPSIS: OverrunBehavior = OverrunBehavior {
        ord: 3i32
    };
    #[doc(alias = "OVERRUN_TRIM_WORD_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_WORD_ELLIPSIS`"]
    pub const TRIM_WORD_ELLIPSIS: OverrunBehavior = OverrunBehavior {
        ord: 4i32
    };
    #[doc(alias = "OVERRUN_TRIM_ELLIPSIS_FORCE")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_ELLIPSIS_FORCE`"]
    pub const TRIM_ELLIPSIS_FORCE: OverrunBehavior = OverrunBehavior {
        ord: 5i32
    };
    #[doc(alias = "OVERRUN_TRIM_WORD_ELLIPSIS_FORCE")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_WORD_ELLIPSIS_FORCE`"]
    pub const TRIM_WORD_ELLIPSIS_FORCE: OverrunBehavior = OverrunBehavior {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for OverrunBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("OverrunBehavior") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for OverrunBehavior {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NO_TRIMMING => "NO_TRIMMING", Self::TRIM_CHAR => "TRIM_CHAR", Self::TRIM_WORD => "TRIM_WORD", Self::TRIM_ELLIPSIS => "TRIM_ELLIPSIS", Self::TRIM_WORD_ELLIPSIS => "TRIM_WORD_ELLIPSIS", Self::TRIM_ELLIPSIS_FORCE => "TRIM_ELLIPSIS_FORCE", Self::TRIM_WORD_ELLIPSIS_FORCE => "TRIM_WORD_ELLIPSIS_FORCE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[OverrunBehavior::NO_TRIMMING, OverrunBehavior::TRIM_CHAR, OverrunBehavior::TRIM_WORD, OverrunBehavior::TRIM_ELLIPSIS, OverrunBehavior::TRIM_WORD_ELLIPSIS, OverrunBehavior::TRIM_ELLIPSIS_FORCE, OverrunBehavior::TRIM_WORD_ELLIPSIS_FORCE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < OverrunBehavior >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NO_TRIMMING", "OVERRUN_NO_TRIMMING", OverrunBehavior::NO_TRIMMING), crate::meta::inspect::EnumConstant::new("TRIM_CHAR", "OVERRUN_TRIM_CHAR", OverrunBehavior::TRIM_CHAR), crate::meta::inspect::EnumConstant::new("TRIM_WORD", "OVERRUN_TRIM_WORD", OverrunBehavior::TRIM_WORD), crate::meta::inspect::EnumConstant::new("TRIM_ELLIPSIS", "OVERRUN_TRIM_ELLIPSIS", OverrunBehavior::TRIM_ELLIPSIS), crate::meta::inspect::EnumConstant::new("TRIM_WORD_ELLIPSIS", "OVERRUN_TRIM_WORD_ELLIPSIS", OverrunBehavior::TRIM_WORD_ELLIPSIS), crate::meta::inspect::EnumConstant::new("TRIM_ELLIPSIS_FORCE", "OVERRUN_TRIM_ELLIPSIS_FORCE", OverrunBehavior::TRIM_ELLIPSIS_FORCE), crate::meta::inspect::EnumConstant::new("TRIM_WORD_ELLIPSIS_FORCE", "OVERRUN_TRIM_WORD_ELLIPSIS_FORCE", OverrunBehavior::TRIM_WORD_ELLIPSIS_FORCE)]
        }
    }
}
impl crate::meta::GodotConvert for OverrunBehavior {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Overrun No Trimming", 0i64), EnumeratorShape::new_int("Overrun Trim Char", 1i64), EnumeratorShape::new_int("Overrun Trim Word", 2i64), EnumeratorShape::new_int("Overrun Trim Ellipsis", 3i64), EnumeratorShape::new_int("Overrun Trim Word Ellipsis", 4i64), EnumeratorShape::new_int("Overrun Trim Ellipsis Force", 5i64), EnumeratorShape::new_int("Overrun Trim Word Ellipsis Force", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.OverrunBehavior")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for OverrunBehavior {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for OverrunBehavior {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for OverrunBehavior {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for OverrunBehavior {
    
}
impl crate::meta::Element for OverrunBehavior {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct TextOverrunFlag {
    ord: u64
}
impl TextOverrunFlag {
    #[doc(alias = "OVERRUN_NO_TRIM")]
    #[doc = "Godot enumerator name: `OVERRUN_NO_TRIM`"]
    pub const NO_TRIM: TextOverrunFlag = TextOverrunFlag {
        ord: 0u64
    };
    #[doc(alias = "OVERRUN_TRIM")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM`"]
    pub const TRIM: TextOverrunFlag = TextOverrunFlag {
        ord: 1u64
    };
    #[doc(alias = "OVERRUN_TRIM_WORD_ONLY")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_WORD_ONLY`"]
    pub const TRIM_WORD_ONLY: TextOverrunFlag = TextOverrunFlag {
        ord: 2u64
    };
    #[doc(alias = "OVERRUN_ADD_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_ADD_ELLIPSIS`"]
    pub const ADD_ELLIPSIS: TextOverrunFlag = TextOverrunFlag {
        ord: 4u64
    };
    #[doc(alias = "OVERRUN_ENFORCE_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_ENFORCE_ELLIPSIS`"]
    pub const ENFORCE_ELLIPSIS: TextOverrunFlag = TextOverrunFlag {
        ord: 8u64
    };
    #[doc(alias = "OVERRUN_JUSTIFICATION_AWARE")]
    #[doc = "Godot enumerator name: `OVERRUN_JUSTIFICATION_AWARE`"]
    pub const JUSTIFICATION_AWARE: TextOverrunFlag = TextOverrunFlag {
        ord: 16u64
    };
    #[doc(alias = "OVERRUN_SHORT_STRING_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_SHORT_STRING_ELLIPSIS`"]
    pub const SHORT_STRING_ELLIPSIS: TextOverrunFlag = TextOverrunFlag {
        ord: 32u64
    };
    
}
impl std::fmt::Debug for TextOverrunFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for TextOverrunFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextOverrunFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NO_TRIM", "OVERRUN_NO_TRIM", TextOverrunFlag::NO_TRIM), crate::meta::inspect::EnumConstant::new("TRIM", "OVERRUN_TRIM", TextOverrunFlag::TRIM), crate::meta::inspect::EnumConstant::new("TRIM_WORD_ONLY", "OVERRUN_TRIM_WORD_ONLY", TextOverrunFlag::TRIM_WORD_ONLY), crate::meta::inspect::EnumConstant::new("ADD_ELLIPSIS", "OVERRUN_ADD_ELLIPSIS", TextOverrunFlag::ADD_ELLIPSIS), crate::meta::inspect::EnumConstant::new("ENFORCE_ELLIPSIS", "OVERRUN_ENFORCE_ELLIPSIS", TextOverrunFlag::ENFORCE_ELLIPSIS), crate::meta::inspect::EnumConstant::new("JUSTIFICATION_AWARE", "OVERRUN_JUSTIFICATION_AWARE", TextOverrunFlag::JUSTIFICATION_AWARE), crate::meta::inspect::EnumConstant::new("SHORT_STRING_ELLIPSIS", "OVERRUN_SHORT_STRING_ELLIPSIS", TextOverrunFlag::SHORT_STRING_ELLIPSIS)]
        }
    }
}
impl std::ops::BitOr for TextOverrunFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for TextOverrunFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for TextOverrunFlag {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Overrun No Trim", 0i64), EnumeratorShape::new_int("Overrun Trim", 1i64), EnumeratorShape::new_int("Overrun Trim Word Only", 2i64), EnumeratorShape::new_int("Overrun Add Ellipsis", 4i64), EnumeratorShape::new_int("Overrun Enforce Ellipsis", 8i64), EnumeratorShape::new_int("Overrun Justification Aware", 16i64), EnumeratorShape::new_int("Overrun Short String Ellipsis", 32i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.TextOverrunFlag")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for TextOverrunFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextOverrunFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TextOverrunFlag {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for TextOverrunFlag {
    
}
impl crate::meta::Element for TextOverrunFlag {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct GraphemeFlag {
    ord: u64
}
impl GraphemeFlag {
    #[doc(alias = "GRAPHEME_IS_VALID")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_VALID`"]
    pub const VALID: GraphemeFlag = GraphemeFlag {
        ord: 1u64
    };
    #[doc(alias = "GRAPHEME_IS_RTL")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_RTL`"]
    pub const RTL: GraphemeFlag = GraphemeFlag {
        ord: 2u64
    };
    #[doc(alias = "GRAPHEME_IS_VIRTUAL")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_VIRTUAL`"]
    pub const VIRTUAL: GraphemeFlag = GraphemeFlag {
        ord: 4u64
    };
    #[doc(alias = "GRAPHEME_IS_SPACE")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_SPACE`"]
    pub const SPACE: GraphemeFlag = GraphemeFlag {
        ord: 8u64
    };
    #[doc(alias = "GRAPHEME_IS_BREAK_HARD")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_BREAK_HARD`"]
    pub const BREAK_HARD: GraphemeFlag = GraphemeFlag {
        ord: 16u64
    };
    #[doc(alias = "GRAPHEME_IS_BREAK_SOFT")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_BREAK_SOFT`"]
    pub const BREAK_SOFT: GraphemeFlag = GraphemeFlag {
        ord: 32u64
    };
    #[doc(alias = "GRAPHEME_IS_TAB")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_TAB`"]
    pub const TAB: GraphemeFlag = GraphemeFlag {
        ord: 64u64
    };
    #[doc(alias = "GRAPHEME_IS_ELONGATION")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_ELONGATION`"]
    pub const ELONGATION: GraphemeFlag = GraphemeFlag {
        ord: 128u64
    };
    #[doc(alias = "GRAPHEME_IS_PUNCTUATION")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_PUNCTUATION`"]
    pub const PUNCTUATION: GraphemeFlag = GraphemeFlag {
        ord: 256u64
    };
    #[doc(alias = "GRAPHEME_IS_UNDERSCORE")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_UNDERSCORE`"]
    pub const UNDERSCORE: GraphemeFlag = GraphemeFlag {
        ord: 512u64
    };
    #[doc(alias = "GRAPHEME_IS_CONNECTED")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_CONNECTED`"]
    pub const CONNECTED: GraphemeFlag = GraphemeFlag {
        ord: 1024u64
    };
    #[doc(alias = "GRAPHEME_IS_SAFE_TO_INSERT_TATWEEL")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_SAFE_TO_INSERT_TATWEEL`"]
    pub const SAFE_TO_INSERT_TATWEEL: GraphemeFlag = GraphemeFlag {
        ord: 2048u64
    };
    #[doc(alias = "GRAPHEME_IS_EMBEDDED_OBJECT")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_EMBEDDED_OBJECT`"]
    pub const EMBEDDED_OBJECT: GraphemeFlag = GraphemeFlag {
        ord: 4096u64
    };
    #[doc(alias = "GRAPHEME_IS_SOFT_HYPHEN")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_SOFT_HYPHEN`"]
    pub const SOFT_HYPHEN: GraphemeFlag = GraphemeFlag {
        ord: 8192u64
    };
    
}
impl std::fmt::Debug for GraphemeFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for GraphemeFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GraphemeFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VALID", "GRAPHEME_IS_VALID", GraphemeFlag::VALID), crate::meta::inspect::EnumConstant::new("RTL", "GRAPHEME_IS_RTL", GraphemeFlag::RTL), crate::meta::inspect::EnumConstant::new("VIRTUAL", "GRAPHEME_IS_VIRTUAL", GraphemeFlag::VIRTUAL), crate::meta::inspect::EnumConstant::new("SPACE", "GRAPHEME_IS_SPACE", GraphemeFlag::SPACE), crate::meta::inspect::EnumConstant::new("BREAK_HARD", "GRAPHEME_IS_BREAK_HARD", GraphemeFlag::BREAK_HARD), crate::meta::inspect::EnumConstant::new("BREAK_SOFT", "GRAPHEME_IS_BREAK_SOFT", GraphemeFlag::BREAK_SOFT), crate::meta::inspect::EnumConstant::new("TAB", "GRAPHEME_IS_TAB", GraphemeFlag::TAB), crate::meta::inspect::EnumConstant::new("ELONGATION", "GRAPHEME_IS_ELONGATION", GraphemeFlag::ELONGATION), crate::meta::inspect::EnumConstant::new("PUNCTUATION", "GRAPHEME_IS_PUNCTUATION", GraphemeFlag::PUNCTUATION), crate::meta::inspect::EnumConstant::new("UNDERSCORE", "GRAPHEME_IS_UNDERSCORE", GraphemeFlag::UNDERSCORE), crate::meta::inspect::EnumConstant::new("CONNECTED", "GRAPHEME_IS_CONNECTED", GraphemeFlag::CONNECTED), crate::meta::inspect::EnumConstant::new("SAFE_TO_INSERT_TATWEEL", "GRAPHEME_IS_SAFE_TO_INSERT_TATWEEL", GraphemeFlag::SAFE_TO_INSERT_TATWEEL), crate::meta::inspect::EnumConstant::new("EMBEDDED_OBJECT", "GRAPHEME_IS_EMBEDDED_OBJECT", GraphemeFlag::EMBEDDED_OBJECT), crate::meta::inspect::EnumConstant::new("SOFT_HYPHEN", "GRAPHEME_IS_SOFT_HYPHEN", GraphemeFlag::SOFT_HYPHEN)]
        }
    }
}
impl std::ops::BitOr for GraphemeFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for GraphemeFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for GraphemeFlag {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Grapheme Is Valid", 1i64), EnumeratorShape::new_int("Grapheme Is Rtl", 2i64), EnumeratorShape::new_int("Grapheme Is Virtual", 4i64), EnumeratorShape::new_int("Grapheme Is Space", 8i64), EnumeratorShape::new_int("Grapheme Is Break Hard", 16i64), EnumeratorShape::new_int("Grapheme Is Break Soft", 32i64), EnumeratorShape::new_int("Grapheme Is Tab", 64i64), EnumeratorShape::new_int("Grapheme Is Elongation", 128i64), EnumeratorShape::new_int("Grapheme Is Punctuation", 256i64), EnumeratorShape::new_int("Grapheme Is Underscore", 512i64), EnumeratorShape::new_int("Grapheme Is Connected", 1024i64), EnumeratorShape::new_int("Grapheme Is Safe To Insert Tatweel", 2048i64), EnumeratorShape::new_int("Grapheme Is Embedded Object", 4096i64), EnumeratorShape::new_int("Grapheme Is Soft Hyphen", 8192i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.GraphemeFlag")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for GraphemeFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GraphemeFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for GraphemeFlag {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for GraphemeFlag {
    
}
impl crate::meta::Element for GraphemeFlag {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Hinting {
    ord: i32
}
impl Hinting {
    #[doc(alias = "HINTING_NONE")]
    #[doc = "Godot enumerator name: `HINTING_NONE`"]
    pub const NONE: Hinting = Hinting {
        ord: 0i32
    };
    #[doc(alias = "HINTING_LIGHT")]
    #[doc = "Godot enumerator name: `HINTING_LIGHT`"]
    pub const LIGHT: Hinting = Hinting {
        ord: 1i32
    };
    #[doc(alias = "HINTING_NORMAL")]
    #[doc = "Godot enumerator name: `HINTING_NORMAL`"]
    pub const NORMAL: Hinting = Hinting {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Hinting {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Hinting") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Hinting {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "NONE", Self::LIGHT => "LIGHT", Self::NORMAL => "NORMAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Hinting::NONE, Hinting::LIGHT, Hinting::NORMAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Hinting >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "HINTING_NONE", Hinting::NONE), crate::meta::inspect::EnumConstant::new("LIGHT", "HINTING_LIGHT", Hinting::LIGHT), crate::meta::inspect::EnumConstant::new("NORMAL", "HINTING_NORMAL", Hinting::NORMAL)]
        }
    }
}
impl crate::meta::GodotConvert for Hinting {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Hinting None", 0i64), EnumeratorShape::new_int("Hinting Light", 1i64), EnumeratorShape::new_int("Hinting Normal", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.Hinting")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Hinting {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Hinting {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Hinting {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for Hinting {
    
}
impl crate::meta::Element for Hinting {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SubpixelPositioning {
    ord: i32
}
impl SubpixelPositioning {
    #[doc(alias = "SUBPIXEL_POSITIONING_DISABLED")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_DISABLED`"]
    pub const DISABLED: SubpixelPositioning = SubpixelPositioning {
        ord: 0i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_AUTO")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_AUTO`"]
    pub const AUTO: SubpixelPositioning = SubpixelPositioning {
        ord: 1i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_ONE_HALF")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_ONE_HALF`"]
    pub const ONE_HALF: SubpixelPositioning = SubpixelPositioning {
        ord: 2i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_ONE_QUARTER")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_ONE_QUARTER`"]
    pub const ONE_QUARTER: SubpixelPositioning = SubpixelPositioning {
        ord: 3i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_ONE_HALF_MAX_SIZE")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_ONE_HALF_MAX_SIZE`"]
    pub const ONE_HALF_MAX_SIZE: SubpixelPositioning = SubpixelPositioning {
        ord: 20i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_ONE_QUARTER_MAX_SIZE")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_ONE_QUARTER_MAX_SIZE`"]
    pub const ONE_QUARTER_MAX_SIZE: SubpixelPositioning = SubpixelPositioning {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for SubpixelPositioning {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SubpixelPositioning") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SubpixelPositioning {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 16i32 | ord @ 20i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "DISABLED", Self::AUTO => "AUTO", Self::ONE_HALF => "ONE_HALF", Self::ONE_QUARTER => "ONE_QUARTER", Self::ONE_HALF_MAX_SIZE => "ONE_HALF_MAX_SIZE", Self::ONE_QUARTER_MAX_SIZE => "ONE_QUARTER_MAX_SIZE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SubpixelPositioning::DISABLED, SubpixelPositioning::AUTO, SubpixelPositioning::ONE_HALF, SubpixelPositioning::ONE_QUARTER, SubpixelPositioning::ONE_HALF_MAX_SIZE, SubpixelPositioning::ONE_QUARTER_MAX_SIZE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SubpixelPositioning >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "SUBPIXEL_POSITIONING_DISABLED", SubpixelPositioning::DISABLED), crate::meta::inspect::EnumConstant::new("AUTO", "SUBPIXEL_POSITIONING_AUTO", SubpixelPositioning::AUTO), crate::meta::inspect::EnumConstant::new("ONE_HALF", "SUBPIXEL_POSITIONING_ONE_HALF", SubpixelPositioning::ONE_HALF), crate::meta::inspect::EnumConstant::new("ONE_QUARTER", "SUBPIXEL_POSITIONING_ONE_QUARTER", SubpixelPositioning::ONE_QUARTER), crate::meta::inspect::EnumConstant::new("ONE_HALF_MAX_SIZE", "SUBPIXEL_POSITIONING_ONE_HALF_MAX_SIZE", SubpixelPositioning::ONE_HALF_MAX_SIZE), crate::meta::inspect::EnumConstant::new("ONE_QUARTER_MAX_SIZE", "SUBPIXEL_POSITIONING_ONE_QUARTER_MAX_SIZE", SubpixelPositioning::ONE_QUARTER_MAX_SIZE)]
        }
    }
}
impl crate::meta::GodotConvert for SubpixelPositioning {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Subpixel Positioning Disabled", 0i64), EnumeratorShape::new_int("Subpixel Positioning Auto", 1i64), EnumeratorShape::new_int("Subpixel Positioning One Half", 2i64), EnumeratorShape::new_int("Subpixel Positioning One Quarter", 3i64), EnumeratorShape::new_int("Subpixel Positioning One Half Max Size", 20i64), EnumeratorShape::new_int("Subpixel Positioning One Quarter Max Size", 16i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.SubpixelPositioning")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SubpixelPositioning {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SubpixelPositioning {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SubpixelPositioning {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for SubpixelPositioning {
    
}
impl crate::meta::Element for SubpixelPositioning {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Feature {
    ord: i32
}
impl Feature {
    #[doc(alias = "FEATURE_SIMPLE_LAYOUT")]
    #[doc = "Godot enumerator name: `FEATURE_SIMPLE_LAYOUT`"]
    pub const SIMPLE_LAYOUT: Feature = Feature {
        ord: 1i32
    };
    #[doc(alias = "FEATURE_BIDI_LAYOUT")]
    #[doc = "Godot enumerator name: `FEATURE_BIDI_LAYOUT`"]
    pub const BIDI_LAYOUT: Feature = Feature {
        ord: 2i32
    };
    #[doc(alias = "FEATURE_VERTICAL_LAYOUT")]
    #[doc = "Godot enumerator name: `FEATURE_VERTICAL_LAYOUT`"]
    pub const VERTICAL_LAYOUT: Feature = Feature {
        ord: 4i32
    };
    #[doc(alias = "FEATURE_SHAPING")]
    #[doc = "Godot enumerator name: `FEATURE_SHAPING`"]
    pub const SHAPING: Feature = Feature {
        ord: 8i32
    };
    #[doc(alias = "FEATURE_KASHIDA_JUSTIFICATION")]
    #[doc = "Godot enumerator name: `FEATURE_KASHIDA_JUSTIFICATION`"]
    pub const KASHIDA_JUSTIFICATION: Feature = Feature {
        ord: 16i32
    };
    #[doc(alias = "FEATURE_BREAK_ITERATORS")]
    #[doc = "Godot enumerator name: `FEATURE_BREAK_ITERATORS`"]
    pub const BREAK_ITERATORS: Feature = Feature {
        ord: 32i32
    };
    #[doc(alias = "FEATURE_FONT_BITMAP")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_BITMAP`"]
    pub const FONT_BITMAP: Feature = Feature {
        ord: 64i32
    };
    #[doc(alias = "FEATURE_FONT_DYNAMIC")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_DYNAMIC`"]
    pub const FONT_DYNAMIC: Feature = Feature {
        ord: 128i32
    };
    #[doc(alias = "FEATURE_FONT_MSDF")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_MSDF`"]
    pub const FONT_MSDF: Feature = Feature {
        ord: 256i32
    };
    #[doc(alias = "FEATURE_FONT_SYSTEM")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_SYSTEM`"]
    pub const FONT_SYSTEM: Feature = Feature {
        ord: 512i32
    };
    #[doc(alias = "FEATURE_FONT_VARIABLE")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_VARIABLE`"]
    pub const FONT_VARIABLE: Feature = Feature {
        ord: 1024i32
    };
    #[doc(alias = "FEATURE_CONTEXT_SENSITIVE_CASE_CONVERSION")]
    #[doc = "Godot enumerator name: `FEATURE_CONTEXT_SENSITIVE_CASE_CONVERSION`"]
    pub const CONTEXT_SENSITIVE_CASE_CONVERSION: Feature = Feature {
        ord: 2048i32
    };
    #[doc(alias = "FEATURE_USE_SUPPORT_DATA")]
    #[doc = "Godot enumerator name: `FEATURE_USE_SUPPORT_DATA`"]
    pub const USE_SUPPORT_DATA: Feature = Feature {
        ord: 4096i32
    };
    #[doc(alias = "FEATURE_UNICODE_IDENTIFIERS")]
    #[doc = "Godot enumerator name: `FEATURE_UNICODE_IDENTIFIERS`"]
    pub const UNICODE_IDENTIFIERS: Feature = Feature {
        ord: 8192i32
    };
    #[doc(alias = "FEATURE_UNICODE_SECURITY")]
    #[doc = "Godot enumerator name: `FEATURE_UNICODE_SECURITY`"]
    pub const UNICODE_SECURITY: Feature = Feature {
        ord: 16384i32
    };
    
}
impl std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Feature") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Feature {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 | ord @ 64i32 | ord @ 128i32 | ord @ 256i32 | ord @ 512i32 | ord @ 1024i32 | ord @ 2048i32 | ord @ 4096i32 | ord @ 8192i32 | ord @ 16384i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SIMPLE_LAYOUT => "SIMPLE_LAYOUT", Self::BIDI_LAYOUT => "BIDI_LAYOUT", Self::VERTICAL_LAYOUT => "VERTICAL_LAYOUT", Self::SHAPING => "SHAPING", Self::KASHIDA_JUSTIFICATION => "KASHIDA_JUSTIFICATION", Self::BREAK_ITERATORS => "BREAK_ITERATORS", Self::FONT_BITMAP => "FONT_BITMAP", Self::FONT_DYNAMIC => "FONT_DYNAMIC", Self::FONT_MSDF => "FONT_MSDF", Self::FONT_SYSTEM => "FONT_SYSTEM", Self::FONT_VARIABLE => "FONT_VARIABLE", Self::CONTEXT_SENSITIVE_CASE_CONVERSION => "CONTEXT_SENSITIVE_CASE_CONVERSION", Self::USE_SUPPORT_DATA => "USE_SUPPORT_DATA", Self::UNICODE_IDENTIFIERS => "UNICODE_IDENTIFIERS", Self::UNICODE_SECURITY => "UNICODE_SECURITY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Feature::SIMPLE_LAYOUT, Feature::BIDI_LAYOUT, Feature::VERTICAL_LAYOUT, Feature::SHAPING, Feature::KASHIDA_JUSTIFICATION, Feature::BREAK_ITERATORS, Feature::FONT_BITMAP, Feature::FONT_DYNAMIC, Feature::FONT_MSDF, Feature::FONT_SYSTEM, Feature::FONT_VARIABLE, Feature::CONTEXT_SENSITIVE_CASE_CONVERSION, Feature::USE_SUPPORT_DATA, Feature::UNICODE_IDENTIFIERS, Feature::UNICODE_SECURITY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Feature >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SIMPLE_LAYOUT", "FEATURE_SIMPLE_LAYOUT", Feature::SIMPLE_LAYOUT), crate::meta::inspect::EnumConstant::new("BIDI_LAYOUT", "FEATURE_BIDI_LAYOUT", Feature::BIDI_LAYOUT), crate::meta::inspect::EnumConstant::new("VERTICAL_LAYOUT", "FEATURE_VERTICAL_LAYOUT", Feature::VERTICAL_LAYOUT), crate::meta::inspect::EnumConstant::new("SHAPING", "FEATURE_SHAPING", Feature::SHAPING), crate::meta::inspect::EnumConstant::new("KASHIDA_JUSTIFICATION", "FEATURE_KASHIDA_JUSTIFICATION", Feature::KASHIDA_JUSTIFICATION), crate::meta::inspect::EnumConstant::new("BREAK_ITERATORS", "FEATURE_BREAK_ITERATORS", Feature::BREAK_ITERATORS), crate::meta::inspect::EnumConstant::new("FONT_BITMAP", "FEATURE_FONT_BITMAP", Feature::FONT_BITMAP), crate::meta::inspect::EnumConstant::new("FONT_DYNAMIC", "FEATURE_FONT_DYNAMIC", Feature::FONT_DYNAMIC), crate::meta::inspect::EnumConstant::new("FONT_MSDF", "FEATURE_FONT_MSDF", Feature::FONT_MSDF), crate::meta::inspect::EnumConstant::new("FONT_SYSTEM", "FEATURE_FONT_SYSTEM", Feature::FONT_SYSTEM), crate::meta::inspect::EnumConstant::new("FONT_VARIABLE", "FEATURE_FONT_VARIABLE", Feature::FONT_VARIABLE), crate::meta::inspect::EnumConstant::new("CONTEXT_SENSITIVE_CASE_CONVERSION", "FEATURE_CONTEXT_SENSITIVE_CASE_CONVERSION", Feature::CONTEXT_SENSITIVE_CASE_CONVERSION), crate::meta::inspect::EnumConstant::new("USE_SUPPORT_DATA", "FEATURE_USE_SUPPORT_DATA", Feature::USE_SUPPORT_DATA), crate::meta::inspect::EnumConstant::new("UNICODE_IDENTIFIERS", "FEATURE_UNICODE_IDENTIFIERS", Feature::UNICODE_IDENTIFIERS), crate::meta::inspect::EnumConstant::new("UNICODE_SECURITY", "FEATURE_UNICODE_SECURITY", Feature::UNICODE_SECURITY)]
        }
    }
}
impl crate::meta::GodotConvert for Feature {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Feature Simple Layout", 1i64), EnumeratorShape::new_int("Feature Bidi Layout", 2i64), EnumeratorShape::new_int("Feature Vertical Layout", 4i64), EnumeratorShape::new_int("Feature Shaping", 8i64), EnumeratorShape::new_int("Feature Kashida Justification", 16i64), EnumeratorShape::new_int("Feature Break Iterators", 32i64), EnumeratorShape::new_int("Feature Font Bitmap", 64i64), EnumeratorShape::new_int("Feature Font Dynamic", 128i64), EnumeratorShape::new_int("Feature Font Msdf", 256i64), EnumeratorShape::new_int("Feature Font System", 512i64), EnumeratorShape::new_int("Feature Font Variable", 1024i64), EnumeratorShape::new_int("Feature Context Sensitive Case Conversion", 2048i64), EnumeratorShape::new_int("Feature Use Support Data", 4096i64), EnumeratorShape::new_int("Feature Unicode Identifiers", 8192i64), EnumeratorShape::new_int("Feature Unicode Security", 16384i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.Feature")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Feature {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Feature {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Feature {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for Feature {
    
}
impl crate::meta::Element for Feature {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ContourPointTag {
    ord: i32
}
impl ContourPointTag {
    #[doc(alias = "CONTOUR_CURVE_TAG_ON")]
    #[doc = "Godot enumerator name: `CONTOUR_CURVE_TAG_ON`"]
    pub const ON: ContourPointTag = ContourPointTag {
        ord: 1i32
    };
    #[doc(alias = "CONTOUR_CURVE_TAG_OFF_CONIC")]
    #[doc = "Godot enumerator name: `CONTOUR_CURVE_TAG_OFF_CONIC`"]
    pub const OFF_CONIC: ContourPointTag = ContourPointTag {
        ord: 0i32
    };
    #[doc(alias = "CONTOUR_CURVE_TAG_OFF_CUBIC")]
    #[doc = "Godot enumerator name: `CONTOUR_CURVE_TAG_OFF_CUBIC`"]
    pub const OFF_CUBIC: ContourPointTag = ContourPointTag {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ContourPointTag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ContourPointTag") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ContourPointTag {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ON => "ON", Self::OFF_CONIC => "OFF_CONIC", Self::OFF_CUBIC => "OFF_CUBIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ContourPointTag::ON, ContourPointTag::OFF_CONIC, ContourPointTag::OFF_CUBIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ContourPointTag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ON", "CONTOUR_CURVE_TAG_ON", ContourPointTag::ON), crate::meta::inspect::EnumConstant::new("OFF_CONIC", "CONTOUR_CURVE_TAG_OFF_CONIC", ContourPointTag::OFF_CONIC), crate::meta::inspect::EnumConstant::new("OFF_CUBIC", "CONTOUR_CURVE_TAG_OFF_CUBIC", ContourPointTag::OFF_CUBIC)]
        }
    }
}
impl crate::meta::GodotConvert for ContourPointTag {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Contour Curve Tag On", 1i64), EnumeratorShape::new_int("Contour Curve Tag Off Conic", 0i64), EnumeratorShape::new_int("Contour Curve Tag Off Cubic", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.ContourPointTag")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ContourPointTag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ContourPointTag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ContourPointTag {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for ContourPointTag {
    
}
impl crate::meta::Element for ContourPointTag {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpacingType {
    ord: i32
}
impl SpacingType {
    #[doc(alias = "SPACING_GLYPH")]
    #[doc = "Godot enumerator name: `SPACING_GLYPH`"]
    pub const GLYPH: SpacingType = SpacingType {
        ord: 0i32
    };
    #[doc(alias = "SPACING_SPACE")]
    #[doc = "Godot enumerator name: `SPACING_SPACE`"]
    pub const SPACE: SpacingType = SpacingType {
        ord: 1i32
    };
    #[doc(alias = "SPACING_TOP")]
    #[doc = "Godot enumerator name: `SPACING_TOP`"]
    pub const TOP: SpacingType = SpacingType {
        ord: 2i32
    };
    #[doc(alias = "SPACING_BOTTOM")]
    #[doc = "Godot enumerator name: `SPACING_BOTTOM`"]
    pub const BOTTOM: SpacingType = SpacingType {
        ord: 3i32
    };
    #[doc(alias = "SPACING_MAX")]
    #[doc = "Godot enumerator name: `SPACING_MAX`"]
    pub const MAX: SpacingType = SpacingType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for SpacingType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpacingType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpacingType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::GLYPH => "GLYPH", Self::SPACE => "SPACE", Self::TOP => "TOP", Self::BOTTOM => "BOTTOM", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SpacingType::GLYPH, SpacingType::SPACE, SpacingType::TOP, SpacingType::BOTTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SpacingType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GLYPH", "SPACING_GLYPH", SpacingType::GLYPH), crate::meta::inspect::EnumConstant::new("SPACE", "SPACING_SPACE", SpacingType::SPACE), crate::meta::inspect::EnumConstant::new("TOP", "SPACING_TOP", SpacingType::TOP), crate::meta::inspect::EnumConstant::new("BOTTOM", "SPACING_BOTTOM", SpacingType::BOTTOM), crate::meta::inspect::EnumConstant::new("MAX", "SPACING_MAX", SpacingType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for SpacingType {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for SpacingType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Spacing Glyph", 0i64), EnumeratorShape::new_int("Spacing Space", 1i64), EnumeratorShape::new_int("Spacing Top", 2i64), EnumeratorShape::new_int("Spacing Bottom", 3i64), EnumeratorShape::new_int("Spacing Max", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.SpacingType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SpacingType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpacingType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SpacingType {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for SpacingType {
    
}
impl crate::meta::Element for SpacingType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct FontStyle {
    ord: u64
}
impl FontStyle {
    #[doc(alias = "FONT_BOLD")]
    #[doc = "Godot enumerator name: `FONT_BOLD`"]
    pub const BOLD: FontStyle = FontStyle {
        ord: 1u64
    };
    #[doc(alias = "FONT_ITALIC")]
    #[doc = "Godot enumerator name: `FONT_ITALIC`"]
    pub const ITALIC: FontStyle = FontStyle {
        ord: 2u64
    };
    #[doc(alias = "FONT_FIXED_WIDTH")]
    #[doc = "Godot enumerator name: `FONT_FIXED_WIDTH`"]
    pub const FIXED_WIDTH: FontStyle = FontStyle {
        ord: 4u64
    };
    
}
impl std::fmt::Debug for FontStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for FontStyle {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FontStyle >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BOLD", "FONT_BOLD", FontStyle::BOLD), crate::meta::inspect::EnumConstant::new("ITALIC", "FONT_ITALIC", FontStyle::ITALIC), crate::meta::inspect::EnumConstant::new("FIXED_WIDTH", "FONT_FIXED_WIDTH", FontStyle::FIXED_WIDTH)]
        }
    }
}
impl std::ops::BitOr for FontStyle {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for FontStyle {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for FontStyle {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Font Bold", 1i64), EnumeratorShape::new_int("Font Italic", 2i64), EnumeratorShape::new_int("Font Fixed Width", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.FontStyle")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for FontStyle {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FontStyle {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for FontStyle {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for FontStyle {
    
}
impl crate::meta::Element for FontStyle {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StructuredTextParser {
    ord: i32
}
impl StructuredTextParser {
    #[doc(alias = "STRUCTURED_TEXT_DEFAULT")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_DEFAULT`"]
    pub const DEFAULT: StructuredTextParser = StructuredTextParser {
        ord: 0i32
    };
    #[doc(alias = "STRUCTURED_TEXT_URI")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_URI`"]
    pub const URI: StructuredTextParser = StructuredTextParser {
        ord: 1i32
    };
    #[doc(alias = "STRUCTURED_TEXT_FILE")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_FILE`"]
    pub const FILE: StructuredTextParser = StructuredTextParser {
        ord: 2i32
    };
    #[doc(alias = "STRUCTURED_TEXT_EMAIL")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_EMAIL`"]
    pub const EMAIL: StructuredTextParser = StructuredTextParser {
        ord: 3i32
    };
    #[doc(alias = "STRUCTURED_TEXT_LIST")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_LIST`"]
    pub const LIST: StructuredTextParser = StructuredTextParser {
        ord: 4i32
    };
    #[doc(alias = "STRUCTURED_TEXT_GDSCRIPT")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_GDSCRIPT`"]
    pub const GDSCRIPT: StructuredTextParser = StructuredTextParser {
        ord: 5i32
    };
    #[doc(alias = "STRUCTURED_TEXT_CUSTOM")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_CUSTOM`"]
    pub const CUSTOM: StructuredTextParser = StructuredTextParser {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for StructuredTextParser {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StructuredTextParser") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StructuredTextParser {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DEFAULT => "DEFAULT", Self::URI => "URI", Self::FILE => "FILE", Self::EMAIL => "EMAIL", Self::LIST => "LIST", Self::GDSCRIPT => "GDSCRIPT", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[StructuredTextParser::DEFAULT, StructuredTextParser::URI, StructuredTextParser::FILE, StructuredTextParser::EMAIL, StructuredTextParser::LIST, StructuredTextParser::GDSCRIPT, StructuredTextParser::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < StructuredTextParser >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "STRUCTURED_TEXT_DEFAULT", StructuredTextParser::DEFAULT), crate::meta::inspect::EnumConstant::new("URI", "STRUCTURED_TEXT_URI", StructuredTextParser::URI), crate::meta::inspect::EnumConstant::new("FILE", "STRUCTURED_TEXT_FILE", StructuredTextParser::FILE), crate::meta::inspect::EnumConstant::new("EMAIL", "STRUCTURED_TEXT_EMAIL", StructuredTextParser::EMAIL), crate::meta::inspect::EnumConstant::new("LIST", "STRUCTURED_TEXT_LIST", StructuredTextParser::LIST), crate::meta::inspect::EnumConstant::new("GDSCRIPT", "STRUCTURED_TEXT_GDSCRIPT", StructuredTextParser::GDSCRIPT), crate::meta::inspect::EnumConstant::new("CUSTOM", "STRUCTURED_TEXT_CUSTOM", StructuredTextParser::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for StructuredTextParser {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Structured Text Default", 0i64), EnumeratorShape::new_int("Structured Text Uri", 1i64), EnumeratorShape::new_int("Structured Text File", 2i64), EnumeratorShape::new_int("Structured Text Email", 3i64), EnumeratorShape::new_int("Structured Text List", 4i64), EnumeratorShape::new_int("Structured Text Gdscript", 5i64), EnumeratorShape::new_int("Structured Text Custom", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.StructuredTextParser")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for StructuredTextParser {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StructuredTextParser {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for StructuredTextParser {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for StructuredTextParser {
    
}
impl crate::meta::Element for StructuredTextParser {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FixedSizeScaleMode {
    ord: i32
}
impl FixedSizeScaleMode {
    #[doc(alias = "FIXED_SIZE_SCALE_DISABLE")]
    #[doc = "Godot enumerator name: `FIXED_SIZE_SCALE_DISABLE`"]
    pub const DISABLE: FixedSizeScaleMode = FixedSizeScaleMode {
        ord: 0i32
    };
    #[doc(alias = "FIXED_SIZE_SCALE_INTEGER_ONLY")]
    #[doc = "Godot enumerator name: `FIXED_SIZE_SCALE_INTEGER_ONLY`"]
    pub const INTEGER_ONLY: FixedSizeScaleMode = FixedSizeScaleMode {
        ord: 1i32
    };
    #[doc(alias = "FIXED_SIZE_SCALE_ENABLED")]
    #[doc = "Godot enumerator name: `FIXED_SIZE_SCALE_ENABLED`"]
    pub const ENABLED: FixedSizeScaleMode = FixedSizeScaleMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for FixedSizeScaleMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FixedSizeScaleMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FixedSizeScaleMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLE => "DISABLE", Self::INTEGER_ONLY => "INTEGER_ONLY", Self::ENABLED => "ENABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FixedSizeScaleMode::DISABLE, FixedSizeScaleMode::INTEGER_ONLY, FixedSizeScaleMode::ENABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FixedSizeScaleMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLE", "FIXED_SIZE_SCALE_DISABLE", FixedSizeScaleMode::DISABLE), crate::meta::inspect::EnumConstant::new("INTEGER_ONLY", "FIXED_SIZE_SCALE_INTEGER_ONLY", FixedSizeScaleMode::INTEGER_ONLY), crate::meta::inspect::EnumConstant::new("ENABLED", "FIXED_SIZE_SCALE_ENABLED", FixedSizeScaleMode::ENABLED)]
        }
    }
}
impl crate::meta::GodotConvert for FixedSizeScaleMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Fixed Size Scale Disable", 0i64), EnumeratorShape::new_int("Fixed Size Scale Integer Only", 1i64), EnumeratorShape::new_int("Fixed Size Scale Enabled", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextServer.FixedSizeScaleMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for FixedSizeScaleMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FixedSizeScaleMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for FixedSizeScaleMode {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for FixedSizeScaleMode {
    
}
impl crate::meta::Element for FixedSizeScaleMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TextServer;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TextServer {
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
#![doc = "Sidecar module for class [`TextServerExtension`][crate::classes::TextServerExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextServerExtension` enums](https://docs.godotengine.org/en/stable/classes/class_textserverextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextServerExtension`.\n\nInherits [`TextServer`][crate::classes::TextServer].\n\nRelated symbols:\n\n* [`ITextServerExtension`][crate::classes::ITextServerExtension]: virtual methods\n\n\nSee also [Godot docs for `TextServerExtension`](https://docs.godotengine.org/en/stable/classes/class_textserverextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TextServerExtension::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nExternal [`TextServer`][crate::classes::TextServer] implementations should inherit from this class."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextServerExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TextServerExtension`][crate::classes::TextServerExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`ITextServer`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `TextServerExtension` methods](https://docs.godotengine.org/en/stable/classes/class_textserverextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextServerExtension: crate::obj::GodotClass < Base = TextServerExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Returns `true` if the server supports a feature."]
        fn has_feature(&self, feature: crate::classes::text_server::Feature,) -> bool;
        #[doc = "Returns the name of the server interface."]
        fn get_name(&self,) -> GString;
        #[doc = "Returns text server features, see \\[enum TextServer.Feature]."]
        fn get_features(&self,) -> i64;
        #[doc = "Frees an object created by this [`TextServer`][crate::classes::TextServer]."]
        fn free_rid(&mut self, rid: Rid,);
        #[doc = "Returns `true` if `rid` is valid resource owned by this text server."]
        fn has(&mut self, rid: Rid,) -> bool;
        #[doc = "Loads optional TextServer database (e.g. ICU break iterators and dictionaries)."]
        fn load_support_data(&mut self, filename: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Returns default TextServer database (e.g. ICU break iterators and dictionaries) filename."]
        fn get_support_data_filename(&self,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns TextServer database (e.g. ICU break iterators and dictionaries) description."]
        fn get_support_data_info(&self,) -> GString {
            unimplemented !()
        }
        #[doc = "Saves optional TextServer database (e.g. ICU break iterators and dictionaries) to the file."]
        fn save_support_data(&self, filename: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Returns default TextServer database (e.g. ICU break iterators and dictionaries)."]
        fn get_support_data(&self,) -> PackedByteArray {
            unimplemented !()
        }
        #[doc = "Returns `true` if the locale requires text server support data for line/word breaking."]
        fn is_locale_using_support_data(&self, locale: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Returns `true` if locale is right-to-left."]
        fn is_locale_right_to_left(&self, locale: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Converts the given readable name of a feature, variation, script, or language to an OpenType tag."]
        fn name_to_tag(&self, name: GString,) -> i64 {
            unimplemented !()
        }
        #[doc = "Converts the given OpenType tag to the readable name of a feature, variation, script, or language."]
        fn tag_to_name(&self, tag: i64,) -> GString {
            unimplemented !()
        }
        #[doc = "Creates a new, empty font cache entry resource."]
        fn create_font(&mut self,) -> Rid;
        #[doc = "Optional, implement if font supports extra spacing or baseline offset.\n\nCreates a new variation existing font which is reusing the same glyph cache and font data."]
        fn create_font_linked_variation(&mut self, font_rid: Rid,) -> Rid {
            unimplemented !()
        }
        #[doc = "Sets font source data, e.g contents of the dynamic font source file."]
        fn font_set_data(&mut self, font_rid: Rid, data: PackedByteArray,) {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nSets pointer to the font source data, e.g contents of the dynamic font source file."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn font_set_data_ptr_rawptr(&mut self, font_rid: Rid, data_ptr: crate::meta::RawPtr < * const u8 >, data_size: i64,) {
            unimplemented !()
        }
        #[doc = "Sets an active face index in the TrueType / OpenType collection."]
        fn font_set_face_index(&mut self, font_rid: Rid, face_index: i64,) {
            unimplemented !()
        }
        #[doc = "Returns an active face index in the TrueType / OpenType collection."]
        fn font_get_face_index(&self, font_rid: Rid,) -> i64 {
            unimplemented !()
        }
        #[doc = "Returns number of faces in the TrueType / OpenType collection."]
        fn font_get_face_count(&self, font_rid: Rid,) -> i64 {
            unimplemented !()
        }
        #[doc = "Sets the font style flags."]
        fn font_set_style(&mut self, font_rid: Rid, style: crate::classes::text_server::FontStyle,) {
            unimplemented !()
        }
        #[doc = "Returns font style flags."]
        fn font_get_style(&self, font_rid: Rid,) -> crate::classes::text_server::FontStyle {
            unimplemented !()
        }
        #[doc = "Sets the font family name."]
        fn font_set_name(&mut self, font_rid: Rid, name: GString,) {
            unimplemented !()
        }
        #[doc = "Returns font family name."]
        fn font_get_name(&self, font_rid: Rid,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns [`Dictionary`][crate::builtin::Dictionary] with OpenType font name strings (localized font names, version, description, license information, sample text, etc.)."]
        fn font_get_ot_name_strings(&self, font_rid: Rid,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "Sets the font style name."]
        fn font_set_style_name(&mut self, font_rid: Rid, name_style: GString,) {
            unimplemented !()
        }
        #[doc = "Returns font style name."]
        fn font_get_style_name(&self, font_rid: Rid,) -> GString {
            unimplemented !()
        }
        #[doc = "Sets weight (boldness) of the font. A value in the `100...999` range, normal font weight is `400`, bold font weight is `700`."]
        fn font_set_weight(&mut self, font_rid: Rid, weight: i64,) {
            unimplemented !()
        }
        #[doc = "Returns weight (boldness) of the font. A value in the `100...999` range, normal font weight is `400`, bold font weight is `700`."]
        fn font_get_weight(&self, font_rid: Rid,) -> i64 {
            unimplemented !()
        }
        #[doc = "Sets font stretch amount, compared to a normal width. A percentage value between `50%` and `200%`."]
        fn font_set_stretch(&mut self, font_rid: Rid, stretch: i64,) {
            unimplemented !()
        }
        #[doc = "Returns font stretch amount, compared to a normal width. A percentage value between `50%` and `200%`."]
        fn font_get_stretch(&self, font_rid: Rid,) -> i64 {
            unimplemented !()
        }
        #[doc = "Sets font anti-aliasing mode."]
        fn font_set_antialiasing(&mut self, font_rid: Rid, antialiasing: crate::classes::text_server::FontAntialiasing,) {
            unimplemented !()
        }
        #[doc = "Returns font anti-aliasing mode."]
        fn font_get_antialiasing(&self, font_rid: Rid,) -> crate::classes::text_server::FontAntialiasing {
            unimplemented !()
        }
        #[doc = "If set to `true`, embedded font bitmap loading is disabled."]
        fn font_set_disable_embedded_bitmaps(&mut self, font_rid: Rid, disable_embedded_bitmaps: bool,) {
            unimplemented !()
        }
        #[doc = "Returns whether the font's embedded bitmap loading is disabled."]
        fn font_get_disable_embedded_bitmaps(&self, font_rid: Rid,) -> bool {
            unimplemented !()
        }
        #[doc = "If set to `true` font texture mipmap generation is enabled."]
        fn font_set_generate_mipmaps(&mut self, font_rid: Rid, generate_mipmaps: bool,) {
            unimplemented !()
        }
        #[doc = "Returns `true` if font texture mipmap generation is enabled."]
        fn font_get_generate_mipmaps(&self, font_rid: Rid,) -> bool {
            unimplemented !()
        }
        #[doc = "If set to `true`, glyphs of all sizes are rendered using single multichannel signed distance field generated from the dynamic font vector data. MSDF rendering allows displaying the font at any scaling factor without blurriness, and without incurring a CPU cost when the font size changes (since the font no longer needs to be rasterized on the CPU). As a downside, font hinting is not available with MSDF. The lack of font hinting may result in less crisp and less readable fonts at small sizes."]
        fn font_set_multichannel_signed_distance_field(&mut self, font_rid: Rid, msdf: bool,) {
            unimplemented !()
        }
        #[doc = "Returns `true` if glyphs of all sizes are rendered using single multichannel signed distance field generated from the dynamic font vector data."]
        fn font_is_multichannel_signed_distance_field(&self, font_rid: Rid,) -> bool {
            unimplemented !()
        }
        #[doc = "Sets the width of the range around the shape between the minimum and maximum representable signed distance."]
        fn font_set_msdf_pixel_range(&mut self, font_rid: Rid, msdf_pixel_range: i64,) {
            unimplemented !()
        }
        #[doc = "Returns the width of the range around the shape between the minimum and maximum representable signed distance."]
        fn font_get_msdf_pixel_range(&self, font_rid: Rid,) -> i64 {
            unimplemented !()
        }
        #[doc = "Sets source font size used to generate MSDF textures."]
        fn font_set_msdf_size(&mut self, font_rid: Rid, msdf_size: i64,) {
            unimplemented !()
        }
        #[doc = "Returns source font size used to generate MSDF textures."]
        fn font_get_msdf_size(&self, font_rid: Rid,) -> i64 {
            unimplemented !()
        }
        #[doc = "Sets bitmap font fixed size. If set to value greater than zero, same cache entry will be used for all font sizes."]
        fn font_set_fixed_size(&mut self, font_rid: Rid, fixed_size: i64,);
        #[doc = "Returns bitmap font fixed size."]
        fn font_get_fixed_size(&self, font_rid: Rid,) -> i64;
        #[doc = "Sets bitmap font scaling mode. This property is used only if `fixed_size` is greater than zero."]
        fn font_set_fixed_size_scale_mode(&mut self, font_rid: Rid, fixed_size_scale_mode: crate::classes::text_server::FixedSizeScaleMode,);
        #[doc = "Returns bitmap font scaling mode."]
        fn font_get_fixed_size_scale_mode(&self, font_rid: Rid,) -> crate::classes::text_server::FixedSizeScaleMode;
        #[doc = "If set to `true`, system fonts can be automatically used as fallbacks."]
        fn font_set_allow_system_fallback(&mut self, font_rid: Rid, allow_system_fallback: bool,) {
            unimplemented !()
        }
        #[doc = "Returns `true` if system fonts can be automatically used as fallbacks."]
        fn font_is_allow_system_fallback(&self, font_rid: Rid,) -> bool {
            unimplemented !()
        }
        #[doc = "Frees all automatically loaded system fonts."]
        fn font_clear_system_fallback_cache(&mut self,) {
            unimplemented !()
        }
        #[doc = "If set to `true` auto-hinting is preferred over font built-in hinting."]
        fn font_set_force_autohinter(&mut self, font_rid: Rid, force_autohinter: bool,) {
            unimplemented !()
        }
        #[doc = "Returns `true` if auto-hinting is supported and preferred over font built-in hinting."]
        fn font_is_force_autohinter(&self, font_rid: Rid,) -> bool {
            unimplemented !()
        }
        #[doc = "If set to `true`, color modulation is applied when drawing colored glyphs, otherwise it's applied to the monochrome glyphs only."]
        fn font_set_modulate_color_glyphs(&mut self, font_rid: Rid, modulate: bool,) {
            unimplemented !()
        }
        #[doc = "Returns `true` if color modulation is applied when drawing the font's colored glyphs."]
        fn font_is_modulate_color_glyphs(&self, font_rid: Rid,) -> bool {
            unimplemented !()
        }
        #[doc = "Sets font hinting mode. Used by dynamic fonts only."]
        fn font_set_hinting(&mut self, font_rid: Rid, hinting: crate::classes::text_server::Hinting,) {
            unimplemented !()
        }
        #[doc = "Returns the font hinting mode. Used by dynamic fonts only."]
        fn font_get_hinting(&self, font_rid: Rid,) -> crate::classes::text_server::Hinting {
            unimplemented !()
        }
        #[doc = "Sets font subpixel glyph positioning mode."]
        fn font_set_subpixel_positioning(&mut self, font_rid: Rid, subpixel_positioning: crate::classes::text_server::SubpixelPositioning,) {
            unimplemented !()
        }
        #[doc = "Returns font subpixel glyph positioning mode."]
        fn font_get_subpixel_positioning(&self, font_rid: Rid,) -> crate::classes::text_server::SubpixelPositioning {
            unimplemented !()
        }
        #[doc = "Sets glyph position rounding behavior. If set to `true`, when aligning glyphs to the pixel boundaries rounding remainders are accumulated to ensure more uniform glyph distribution. This setting has no effect if subpixel positioning is enabled."]
        fn font_set_keep_rounding_remainders(&mut self, font_rid: Rid, keep_rounding_remainders: bool,) {
            unimplemented !()
        }
        #[doc = "Returns glyph position rounding behavior. If set to `true`, when aligning glyphs to the pixel boundaries rounding remainders are accumulated to ensure more uniform glyph distribution. This setting has no effect if subpixel positioning is enabled."]
        fn font_get_keep_rounding_remainders(&self, font_rid: Rid,) -> bool {
            unimplemented !()
        }
        #[doc = "Sets font embolden strength. If `strength` is not equal to zero, emboldens the font outlines. Negative values reduce the outline thickness."]
        fn font_set_embolden(&mut self, font_rid: Rid, strength: f64,) {
            unimplemented !()
        }
        #[doc = "Returns font embolden strength."]
        fn font_get_embolden(&self, font_rid: Rid,) -> f64 {
            unimplemented !()
        }
        #[doc = "Sets the spacing for `spacing` to `value` in pixels (not relative to the font size)."]
        fn font_set_spacing(&mut self, font_rid: Rid, spacing: crate::classes::text_server::SpacingType, value: i64,) {
            unimplemented !()
        }
        #[doc = "Returns the spacing for `spacing` in pixels (not relative to the font size)."]
        fn font_get_spacing(&self, font_rid: Rid, spacing: crate::classes::text_server::SpacingType,) -> i64 {
            unimplemented !()
        }
        #[doc = "Sets extra baseline offset (as a fraction of font height)."]
        fn font_set_baseline_offset(&mut self, font_rid: Rid, baseline_offset: f64,) {
            unimplemented !()
        }
        #[doc = "Returns extra baseline offset (as a fraction of font height)."]
        fn font_get_baseline_offset(&self, font_rid: Rid,) -> f64 {
            unimplemented !()
        }
        #[doc = "Sets 2D transform, applied to the font outlines, can be used for slanting, flipping, and rotating glyphs."]
        fn font_set_transform(&mut self, font_rid: Rid, transform: Transform2D,) {
            unimplemented !()
        }
        #[doc = "Returns 2D transform applied to the font outlines."]
        fn font_get_transform(&self, font_rid: Rid,) -> Transform2D {
            unimplemented !()
        }
        #[doc = "Sets variation coordinates for the specified font cache entry."]
        fn font_set_variation_coordinates(&mut self, font_rid: Rid, variation_coordinates: VarDictionary,) {
            unimplemented !()
        }
        #[doc = "Returns variation coordinates for the specified font cache entry."]
        fn font_get_variation_coordinates(&self, font_rid: Rid,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "If set to a positive value, overrides the oversampling factor of the viewport this font is used in. See \\[member Viewport.oversampling]. This value doesn't override the `oversampling` parameter of `draw_*` methods. Used by dynamic fonts only."]
        fn font_set_oversampling(&mut self, font_rid: Rid, oversampling: f64,) {
            unimplemented !()
        }
        #[doc = "Returns oversampling factor override. If set to a positive value, overrides the oversampling factor of the viewport this font is used in. See \\[member Viewport.oversampling]. This value doesn't override the `oversampling` parameter of `draw_*` methods. Used by dynamic fonts only."]
        fn font_get_oversampling(&self, font_rid: Rid,) -> f64 {
            unimplemented !()
        }
        #[doc = "Returns list of the font sizes in the cache. Each size is [`Vector2i`][crate::builtin::Vector2i] with font size and outline size."]
        fn font_get_size_cache_list(&self, font_rid: Rid,) -> Array < Vector2i >;
        #[doc = "Removes all font sizes from the cache entry."]
        fn font_clear_size_cache(&mut self, font_rid: Rid,);
        #[doc = "Removes specified font size from the cache entry."]
        fn font_remove_size_cache(&mut self, font_rid: Rid, size: Vector2i,);
        #[doc = "Returns font cache information, each entry contains the following fields: `Vector2i size_px` - font size in pixels, `float viewport_oversampling` - viewport oversampling factor, `int glyphs` - number of rendered glyphs, `int textures` - number of used textures, `int textures_size` - size of texture data in bytes."]
        fn font_get_size_cache_info(&self, font_rid: Rid,) -> Array < AnyDictionary > {
            unimplemented !()
        }
        #[doc = "Sets the font ascent (number of pixels above the baseline)."]
        fn font_set_ascent(&mut self, font_rid: Rid, size: i64, ascent: f64,);
        #[doc = "Returns the font ascent (number of pixels above the baseline)."]
        fn font_get_ascent(&self, font_rid: Rid, size: i64,) -> f64;
        #[doc = "Sets the font descent (number of pixels below the baseline)."]
        fn font_set_descent(&mut self, font_rid: Rid, size: i64, descent: f64,);
        #[doc = "Returns the font descent (number of pixels below the baseline)."]
        fn font_get_descent(&self, font_rid: Rid, size: i64,) -> f64;
        #[doc = "Sets pixel offset of the underline below the baseline."]
        fn font_set_underline_position(&mut self, font_rid: Rid, size: i64, underline_position: f64,);
        #[doc = "Returns pixel offset of the underline below the baseline."]
        fn font_get_underline_position(&self, font_rid: Rid, size: i64,) -> f64;
        #[doc = "Sets thickness of the underline in pixels."]
        fn font_set_underline_thickness(&mut self, font_rid: Rid, size: i64, underline_thickness: f64,);
        #[doc = "Returns thickness of the underline in pixels."]
        fn font_get_underline_thickness(&self, font_rid: Rid, size: i64,) -> f64;
        #[doc = "Sets scaling factor of the color bitmap font."]
        fn font_set_scale(&mut self, font_rid: Rid, size: i64, scale: f64,);
        #[doc = "Returns scaling factor of the color bitmap font."]
        fn font_get_scale(&self, font_rid: Rid, size: i64,) -> f64;
        #[doc = "Returns number of textures used by font cache entry."]
        fn font_get_texture_count(&self, font_rid: Rid, size: Vector2i,) -> i64;
        #[doc = "Removes all textures from font cache entry."]
        fn font_clear_textures(&mut self, font_rid: Rid, size: Vector2i,);
        #[doc = "Removes specified texture from the cache entry."]
        fn font_remove_texture(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64,);
        #[doc = "Sets font cache texture image data."]
        fn font_set_texture_image(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64, image: Option < Gd < crate::classes::Image > >,);
        #[doc = "Returns font cache texture image data."]
        fn font_get_texture_image(&self, font_rid: Rid, size: Vector2i, texture_index: i64,) -> Option < Gd < crate::classes::Image > >;
        #[doc = "Sets array containing glyph packing data."]
        fn font_set_texture_offsets(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64, offset: PackedInt32Array,) {
            unimplemented !()
        }
        #[doc = "Returns array containing glyph packing data."]
        fn font_get_texture_offsets(&self, font_rid: Rid, size: Vector2i, texture_index: i64,) -> PackedInt32Array {
            unimplemented !()
        }
        #[doc = "Returns list of rendered glyphs in the cache entry."]
        fn font_get_glyph_list(&self, font_rid: Rid, size: Vector2i,) -> PackedInt32Array;
        #[doc = "Removes all rendered glyph information from the cache entry."]
        fn font_clear_glyphs(&mut self, font_rid: Rid, size: Vector2i,);
        #[doc = "Removes specified rendered glyph information from the cache entry."]
        fn font_remove_glyph(&mut self, font_rid: Rid, size: Vector2i, glyph: i64,);
        #[doc = "Returns glyph advance (offset of the next glyph)."]
        fn font_get_glyph_advance(&self, font_rid: Rid, size: i64, glyph: i64,) -> Vector2;
        #[doc = "Sets glyph advance (offset of the next glyph)."]
        fn font_set_glyph_advance(&mut self, font_rid: Rid, size: i64, glyph: i64, advance: Vector2,);
        #[doc = "Returns glyph offset from the baseline."]
        fn font_get_glyph_offset(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2;
        #[doc = "Sets glyph offset from the baseline."]
        fn font_set_glyph_offset(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, offset: Vector2,);
        #[doc = "Returns size of the glyph."]
        fn font_get_glyph_size(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2;
        #[doc = "Sets size of the glyph."]
        fn font_set_glyph_size(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, gl_size: Vector2,);
        #[doc = "Returns rectangle in the cache texture containing the glyph."]
        fn font_get_glyph_uv_rect(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Rect2;
        #[doc = "Sets rectangle in the cache texture containing the glyph."]
        fn font_set_glyph_uv_rect(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, uv_rect: Rect2,);
        #[doc = "Returns index of the cache texture containing the glyph."]
        fn font_get_glyph_texture_idx(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> i64;
        #[doc = "Sets index of the cache texture containing the glyph."]
        fn font_set_glyph_texture_idx(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, texture_idx: i64,);
        #[doc = "Returns resource ID of the cache texture containing the glyph."]
        fn font_get_glyph_texture_rid(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Rid;
        #[doc = "Returns size of the cache texture containing the glyph."]
        fn font_get_glyph_texture_size(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2;
        #[doc = "Returns outline contours of the glyph."]
        fn font_get_glyph_contours(&self, font_rid: Rid, size: i64, index: i64,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "Returns list of the kerning overrides."]
        fn font_get_kerning_list(&self, font_rid: Rid, size: i64,) -> Array < Vector2i > {
            unimplemented !()
        }
        #[doc = "Removes all kerning overrides."]
        fn font_clear_kerning_map(&mut self, font_rid: Rid, size: i64,) {
            unimplemented !()
        }
        #[doc = "Removes kerning override for the pair of glyphs."]
        fn font_remove_kerning(&mut self, font_rid: Rid, size: i64, glyph_pair: Vector2i,) {
            unimplemented !()
        }
        #[doc = "Sets kerning for the pair of glyphs."]
        fn font_set_kerning(&mut self, font_rid: Rid, size: i64, glyph_pair: Vector2i, kerning: Vector2,) {
            unimplemented !()
        }
        #[doc = "Returns kerning for the pair of glyphs."]
        fn font_get_kerning(&self, font_rid: Rid, size: i64, glyph_pair: Vector2i,) -> Vector2 {
            unimplemented !()
        }
        #[doc = "Returns the glyph index of a `char`, optionally modified by the `variation_selector`."]
        fn font_get_glyph_index(&self, font_rid: Rid, size: i64, char: i64, variation_selector: i64,) -> i64;
        #[doc = "Returns character code associated with `glyph_index`, or `0` if `glyph_index` is invalid."]
        fn font_get_char_from_glyph_index(&self, font_rid: Rid, size: i64, glyph_index: i64,) -> i64;
        #[doc = "Returns `true` if a Unicode `char` is available in the font."]
        fn font_has_char(&self, font_rid: Rid, char: i64,) -> bool;
        #[doc = "Returns a string containing all the characters available in the font."]
        fn font_get_supported_chars(&self, font_rid: Rid,) -> GString;
        #[doc = "Returns an array containing all glyph indices in the font."]
        fn font_get_supported_glyphs(&self, font_rid: Rid,) -> PackedInt32Array;
        #[doc = "Renders the range of characters to the font cache texture."]
        fn font_render_range(&mut self, font_rid: Rid, size: Vector2i, start: i64, end: i64,) {
            unimplemented !()
        }
        #[doc = "Renders specified glyph to the font cache texture."]
        fn font_render_glyph(&mut self, font_rid: Rid, size: Vector2i, index: i64,) {
            unimplemented !()
        }
        #[doc = "Draws single glyph into a canvas item at the position, using `font_rid` at the size `size`. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        fn font_draw_glyph(&self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color, oversampling: f32,);
        #[doc = "Draws single glyph outline of size `outline_size` into a canvas item at the position, using `font_rid` at the size `size`. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        fn font_draw_glyph_outline(&self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64, color: Color, oversampling: f32,);
        #[doc = "Returns `true` if the font supports the given language (as a [ISO 639](https://en.wikipedia.org/wiki/ISO_639-1) code)."]
        fn font_is_language_supported(&self, font_rid: Rid, language: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Adds override for [`font_is_language_supported`][`crate::classes::ITextServerExtension::font_is_language_supported`]."]
        fn font_set_language_support_override(&mut self, font_rid: Rid, language: GString, supported: bool,) {
            unimplemented !()
        }
        #[doc = "Returns `true` if support override is enabled for the `language`."]
        fn font_get_language_support_override(&mut self, font_rid: Rid, language: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Remove language support override."]
        fn font_remove_language_support_override(&mut self, font_rid: Rid, language: GString,) {
            unimplemented !()
        }
        #[doc = "Returns list of language support overrides."]
        fn font_get_language_support_overrides(&mut self, font_rid: Rid,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Returns `true` if the font supports the given script (as a [ISO 15924](https://en.wikipedia.org/wiki/ISO_15924) code)."]
        fn font_is_script_supported(&self, font_rid: Rid, script: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Adds override for [`font_is_script_supported`][`crate::classes::ITextServerExtension::font_is_script_supported`]."]
        fn font_set_script_support_override(&mut self, font_rid: Rid, script: GString, supported: bool,) {
            unimplemented !()
        }
        #[doc = "Returns `true` if support override is enabled for the `script`."]
        fn font_get_script_support_override(&mut self, font_rid: Rid, script: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Removes script support override."]
        fn font_remove_script_support_override(&mut self, font_rid: Rid, script: GString,) {
            unimplemented !()
        }
        #[doc = "Returns list of script support overrides."]
        fn font_get_script_support_overrides(&mut self, font_rid: Rid,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Sets font OpenType feature set override."]
        fn font_set_opentype_feature_overrides(&mut self, font_rid: Rid, overrides: VarDictionary,) {
            unimplemented !()
        }
        #[doc = "Returns font OpenType feature set override."]
        fn font_get_opentype_feature_overrides(&self, font_rid: Rid,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "Returns the dictionary of the supported OpenType features."]
        fn font_supported_feature_list(&self, font_rid: Rid,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "Returns the dictionary of the supported OpenType variation coordinates."]
        fn font_supported_variation_list(&self, font_rid: Rid,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "Returns the font oversampling factor, shared by all fonts in the TextServer."]
        fn font_get_global_oversampling(&self,) -> f64 {
            unimplemented !()
        }
        #[doc = "Sets oversampling factor, shared by all font in the TextServer."]
        fn font_set_global_oversampling(&mut self, oversampling: f64,) {
            unimplemented !()
        }
        #[doc = "Increases the reference count of the specified oversampling level. This method is called by [`Viewport`][crate::classes::Viewport], and should not be used directly."]
        fn reference_oversampling_level(&mut self, oversampling: f64,) {
            unimplemented !()
        }
        #[doc = "Decreases the reference count of the specified oversampling level, and frees the font cache for oversampling level when the reference count reaches zero. This method is called by [`Viewport`][crate::classes::Viewport], and should not be used directly."]
        fn unreference_oversampling_level(&mut self, oversampling: f64,) {
            unimplemented !()
        }
        #[doc = "Returns size of the replacement character (box with character hexadecimal code that is drawn in place of invalid characters)."]
        fn get_hex_code_box_size(&self, size: i64, index: i64,) -> Vector2 {
            unimplemented !()
        }
        #[doc = "Draws box displaying character hexadecimal code."]
        fn draw_hex_code_box(&self, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color,) {
            unimplemented !()
        }
        #[doc = "Creates a new buffer for complex text layout, with the given `direction` and `orientation`."]
        fn create_shaped_text(&mut self, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,) -> Rid;
        #[doc = "Clears text buffer (removes text and inline objects)."]
        fn shaped_text_clear(&mut self, shaped: Rid,);
        #[doc = "Duplicates shaped text buffer."]
        fn shaped_text_duplicate(&mut self, shaped: Rid,) -> Rid;
        #[doc = "Sets desired text direction. If set to [`Direction::AUTO`][`crate::classes::text_server::Direction::AUTO`], direction will be detected based on the buffer contents and current locale."]
        fn shaped_text_set_direction(&mut self, shaped: Rid, direction: crate::classes::text_server::Direction,) {
            unimplemented !()
        }
        #[doc = "Returns direction of the text."]
        fn shaped_text_get_direction(&self, shaped: Rid,) -> crate::classes::text_server::Direction {
            unimplemented !()
        }
        #[doc = "Returns direction of the text, inferred by the BiDi algorithm."]
        fn shaped_text_get_inferred_direction(&self, shaped: Rid,) -> crate::classes::text_server::Direction {
            unimplemented !()
        }
        #[doc = "Overrides BiDi for the structured text."]
        fn shaped_text_set_bidi_override(&mut self, shaped: Rid, override_: VarArray,) {
            unimplemented !()
        }
        #[doc = "Sets custom punctuation character list, used for word breaking. If set to empty string, server defaults are used."]
        fn shaped_text_set_custom_punctuation(&mut self, shaped: Rid, punct: GString,) {
            unimplemented !()
        }
        #[doc = "Returns custom punctuation character list, used for word breaking. If set to empty string, server defaults are used."]
        fn shaped_text_get_custom_punctuation(&self, shaped: Rid,) -> GString {
            unimplemented !()
        }
        #[doc = "Sets ellipsis character used for text clipping."]
        fn shaped_text_set_custom_ellipsis(&mut self, shaped: Rid, char: i64,) {
            unimplemented !()
        }
        #[doc = "Returns ellipsis character used for text clipping."]
        fn shaped_text_get_custom_ellipsis(&self, shaped: Rid,) -> i64 {
            unimplemented !()
        }
        #[doc = "Sets desired text orientation."]
        fn shaped_text_set_orientation(&mut self, shaped: Rid, orientation: crate::classes::text_server::Orientation,) {
            unimplemented !()
        }
        #[doc = "Returns text orientation."]
        fn shaped_text_get_orientation(&self, shaped: Rid,) -> crate::classes::text_server::Orientation {
            unimplemented !()
        }
        #[doc = "If set to `true` text buffer will display invalid characters as hexadecimal codes, otherwise nothing is displayed."]
        fn shaped_text_set_preserve_invalid(&mut self, shaped: Rid, enabled: bool,) {
            unimplemented !()
        }
        #[doc = "Returns `true` if text buffer is configured to display hexadecimal codes in place of invalid characters."]
        fn shaped_text_get_preserve_invalid(&self, shaped: Rid,) -> bool {
            unimplemented !()
        }
        #[doc = "If set to `true` text buffer will display control characters."]
        fn shaped_text_set_preserve_control(&mut self, shaped: Rid, enabled: bool,) {
            unimplemented !()
        }
        #[doc = "Returns `true` if text buffer is configured to display control characters."]
        fn shaped_text_get_preserve_control(&self, shaped: Rid,) -> bool {
            unimplemented !()
        }
        #[doc = "Sets extra spacing added between glyphs or lines in pixels."]
        fn shaped_text_set_spacing(&mut self, shaped: Rid, spacing: crate::classes::text_server::SpacingType, value: i64,) {
            unimplemented !()
        }
        #[doc = "Returns extra spacing added between glyphs or lines in pixels."]
        fn shaped_text_get_spacing(&self, shaped: Rid, spacing: crate::classes::text_server::SpacingType,) -> i64 {
            unimplemented !()
        }
        #[doc = "Adds text span and font to draw it to the text buffer."]
        fn shaped_text_add_string(&mut self, shaped: Rid, text: GString, fonts: Array < Rid >, size: i64, opentype_features: VarDictionary, language: GString, meta: Variant,) -> bool;
        #[doc = "Adds inline object to the text buffer, `key` must be unique. In the text, object is represented as `length` object replacement characters."]
        fn shaped_text_add_object(&mut self, shaped: Rid, key: Variant, size: Vector2, inline_align: crate::global::InlineAlignment, length: i64, baseline: f64,) -> bool;
        #[doc = "Sets new size and alignment of embedded object."]
        fn shaped_text_resize_object(&mut self, shaped: Rid, key: Variant, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f64,) -> bool;
        #[doc = "Returns `true` if an object with `key` is embedded in this shaped text buffer."]
        fn shaped_text_has_object(&self, shaped: Rid, key: Variant,) -> bool;
        #[doc = "Returns the text buffer source text, including object replacement characters."]
        fn shaped_get_text(&self, shaped: Rid,) -> GString;
        #[doc = "Returns number of text spans added using [`shaped_text_add_string`][`crate::classes::ITextServerExtension::shaped_text_add_string`] or [`shaped_text_add_object`][`crate::classes::ITextServerExtension::shaped_text_add_object`]."]
        fn shaped_get_span_count(&self, shaped: Rid,) -> i64;
        #[doc = "Returns text span metadata."]
        fn shaped_get_span_meta(&self, shaped: Rid, index: i64,) -> Variant;
        #[doc = "Returns text embedded object key."]
        fn shaped_get_span_embedded_object(&self, shaped: Rid, index: i64,) -> Variant;
        #[doc = "Returns the text span source text."]
        fn shaped_get_span_text(&self, shaped: Rid, index: i64,) -> GString;
        #[doc = "Returns the text span embedded object key."]
        fn shaped_get_span_object(&self, shaped: Rid, index: i64,) -> Variant;
        #[doc = "Changes text span font, font size, and OpenType features, without changing the text."]
        fn shaped_set_span_update_font(&mut self, shaped: Rid, index: i64, fonts: Array < Rid >, size: i64, opentype_features: VarDictionary,);
        #[doc = "Returns the number of uniform text runs in the buffer."]
        fn shaped_get_run_count(&self, shaped: Rid,) -> i64 {
            unimplemented !()
        }
        #[doc = "Returns the source text of the `index` text run (in visual order)."]
        fn shaped_get_run_text(&self, shaped: Rid, index: i64,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns the source text range of the `index` text run (in visual order)."]
        fn shaped_get_run_range(&self, shaped: Rid, index: i64,) -> Vector2i {
            unimplemented !()
        }
        #[doc = "Returns the font RID of the `index` text run (in visual order)."]
        fn shaped_get_run_font_rid(&self, shaped: Rid, index: i64,) -> Rid {
            unimplemented !()
        }
        #[doc = "Returns the font size of the `index` text run (in visual order)."]
        fn shaped_get_run_font_size(&self, shaped: Rid, index: i64,) -> i32 {
            unimplemented !()
        }
        #[doc = "Returns the language of the `index` text run (in visual order)."]
        fn shaped_get_run_language(&self, shaped: Rid, index: i64,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns the direction of the `index` text run (in visual order)."]
        fn shaped_get_run_direction(&self, shaped: Rid, index: i64,) -> crate::classes::text_server::Direction {
            unimplemented !()
        }
        #[doc = "Returns the embedded object of the `index` text run (in visual order)."]
        fn shaped_get_run_object(&self, shaped: Rid, index: i64,) -> Variant {
            unimplemented !()
        }
        #[doc = "Returns text buffer for the substring of the text in the `shaped` text buffer (including inline objects)."]
        fn shaped_text_substr(&self, shaped: Rid, start: i64, length: i64,) -> Rid;
        #[doc = "Returns the parent buffer from which the substring originates."]
        fn shaped_text_get_parent(&self, shaped: Rid,) -> Rid;
        #[doc = "Adjusts text width to fit to specified width, returns new text width."]
        fn shaped_text_fit_to_width(&mut self, shaped: Rid, width: f64, justification_flags: crate::classes::text_server::JustificationFlag,) -> f64 {
            unimplemented !()
        }
        #[doc = "Aligns shaped text to the given tab-stops."]
        fn shaped_text_tab_align(&mut self, shaped: Rid, tab_stops: PackedFloat32Array,) -> f64 {
            unimplemented !()
        }
        #[doc = "Shapes buffer if it's not shaped. Returns `true` if the string is shaped successfully."]
        fn shaped_text_shape(&mut self, shaped: Rid,) -> bool;
        #[doc = "Updates break points in the shaped text. This method is called by default implementation of text breaking functions."]
        fn shaped_text_update_breaks(&mut self, shaped: Rid,) -> bool {
            unimplemented !()
        }
        #[doc = "Updates justification points in the shaped text. This method is called by default implementation of text justification functions."]
        fn shaped_text_update_justification_ops(&mut self, shaped: Rid,) -> bool {
            unimplemented !()
        }
        #[doc = "Returns `true` if buffer is successfully shaped."]
        fn shaped_text_is_ready(&self, shaped: Rid,) -> bool;
        #[doc = "\n# Godot docs\nReturns an array of glyphs in the visual order."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn shaped_text_get_glyphs_rawptr(&self, shaped: Rid,) -> crate::meta::RawPtr < * const Glyph >;
        #[doc = "\n# Godot docs\nReturns text glyphs in the logical order."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn shaped_text_sort_logical_rawptr(&mut self, shaped: Rid,) -> crate::meta::RawPtr < * const Glyph >;
        #[doc = "Returns number of glyphs in the buffer."]
        fn shaped_text_get_glyph_count(&self, shaped: Rid,) -> i64;
        #[doc = "Returns substring buffer character range in the parent buffer."]
        fn shaped_text_get_range(&self, shaped: Rid,) -> Vector2i;
        #[doc = "Breaks text to the lines and columns. Returns character ranges for each segment."]
        fn shaped_text_get_line_breaks_adv(&self, shaped: Rid, width: PackedFloat32Array, start: i64, once: bool, break_flags: crate::classes::text_server::LineBreakFlag,) -> PackedInt32Array {
            unimplemented !()
        }
        #[doc = "Breaks text to the lines and returns character ranges for each line."]
        fn shaped_text_get_line_breaks(&self, shaped: Rid, width: f64, start: i64, break_flags: crate::classes::text_server::LineBreakFlag,) -> PackedInt32Array {
            unimplemented !()
        }
        #[doc = "Breaks text into words and returns array of character ranges. Use `grapheme_flags` to set what characters are used for breaking."]
        fn shaped_text_get_word_breaks(&self, shaped: Rid, grapheme_flags: crate::classes::text_server::GraphemeFlag, skip_grapheme_flags: crate::classes::text_server::GraphemeFlag,) -> PackedInt32Array {
            unimplemented !()
        }
        #[doc = "Returns the position of the overrun trim."]
        fn shaped_text_get_trim_pos(&self, shaped: Rid,) -> i64;
        #[doc = "Returns position of the ellipsis."]
        fn shaped_text_get_ellipsis_pos(&self, shaped: Rid,) -> i64;
        #[doc = "Returns number of glyphs in the ellipsis."]
        fn shaped_text_get_ellipsis_glyph_count(&self, shaped: Rid,) -> i64;
        #[doc = "\n# Godot docs\nReturns array of the glyphs in the ellipsis."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn shaped_text_get_ellipsis_glyphs_rawptr(&self, shaped: Rid,) -> crate::meta::RawPtr < * const Glyph >;
        #[doc = "Trims text if it exceeds the given width."]
        fn shaped_text_overrun_trim_to_width(&mut self, shaped: Rid, width: f64, trim_flags: crate::classes::text_server::TextOverrunFlag,) {
            unimplemented !()
        }
        #[doc = "Returns array of inline objects."]
        fn shaped_text_get_objects(&self, shaped: Rid,) -> AnyArray;
        #[doc = "Returns bounding rectangle of the inline object."]
        fn shaped_text_get_object_rect(&self, shaped: Rid, key: Variant,) -> Rect2;
        #[doc = "Returns the character range of the inline object."]
        fn shaped_text_get_object_range(&self, shaped: Rid, key: Variant,) -> Vector2i;
        #[doc = "Returns the glyph index of the inline object."]
        fn shaped_text_get_object_glyph(&self, shaped: Rid, key: Variant,) -> i64;
        #[doc = "Returns size of the text."]
        fn shaped_text_get_size(&self, shaped: Rid,) -> Vector2;
        #[doc = "Returns the text ascent (number of pixels above the baseline for horizontal layout or to the left of baseline for vertical)."]
        fn shaped_text_get_ascent(&self, shaped: Rid,) -> f64;
        #[doc = "Returns the text descent (number of pixels below the baseline for horizontal layout or to the right of baseline for vertical)."]
        fn shaped_text_get_descent(&self, shaped: Rid,) -> f64;
        #[doc = "Returns width (for horizontal layout) or height (for vertical) of the text."]
        fn shaped_text_get_width(&self, shaped: Rid,) -> f64;
        #[doc = "Returns pixel offset of the underline below the baseline."]
        fn shaped_text_get_underline_position(&self, shaped: Rid,) -> f64;
        #[doc = "Returns thickness of the underline."]
        fn shaped_text_get_underline_thickness(&self, shaped: Rid,) -> f64;
        #[doc = "Returns dominant direction of in the range of text."]
        fn shaped_text_get_dominant_direction_in_range(&self, shaped: Rid, start: i64, end: i64,) -> i64 {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nReturns shapes of the carets corresponding to the character offset `position` in the text. Returned caret shape is 1 pixel wide rectangle."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn shaped_text_get_carets_rawptr(&self, shaped: Rid, position: i64, caret: crate::meta::RawPtr < * mut CaretInfo >,) {
            unimplemented !()
        }
        #[doc = "Returns selection rectangles for the specified character range."]
        fn shaped_text_get_selection(&self, shaped: Rid, start: i64, end: i64,) -> PackedVector2Array {
            unimplemented !()
        }
        #[doc = "Returns grapheme index at the specified pixel offset at the baseline, or `-1` if none is found."]
        fn shaped_text_hit_test_grapheme(&self, shaped: Rid, coord: f64,) -> i64 {
            unimplemented !()
        }
        #[doc = "Returns caret character offset at the specified pixel offset at the baseline. This function always returns a valid position."]
        fn shaped_text_hit_test_position(&self, shaped: Rid, coord: f64,) -> i64 {
            unimplemented !()
        }
        #[doc = "Draw shaped text into a canvas item at a given position, with `color`. `pos` specifies the leftmost point of the baseline (for horizontal layout) or topmost point of the baseline (for vertical layout). If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        fn shaped_text_draw(&self, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, color: Color, oversampling: f32,) {
            unimplemented !()
        }
        #[doc = "Draw the outline of the shaped text into a canvas item at a given position, with `color`. `pos` specifies the leftmost point of the baseline (for horizontal layout) or topmost point of the baseline (for vertical layout). If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        fn shaped_text_draw_outline(&self, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, outline_size: i64, color: Color, oversampling: f32,) {
            unimplemented !()
        }
        #[doc = "Returns composite character's bounds as offsets from the start of the line."]
        fn shaped_text_get_grapheme_bounds(&self, shaped: Rid, pos: i64,) -> Vector2 {
            unimplemented !()
        }
        #[doc = "Returns grapheme end position closest to the `pos`."]
        fn shaped_text_next_grapheme_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            unimplemented !()
        }
        #[doc = "Returns grapheme start position closest to the `pos`."]
        fn shaped_text_prev_grapheme_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            unimplemented !()
        }
        #[doc = "Returns array of the composite character boundaries."]
        fn shaped_text_get_character_breaks(&self, shaped: Rid,) -> PackedInt32Array {
            unimplemented !()
        }
        #[doc = "Returns composite character end position closest to the `pos`."]
        fn shaped_text_next_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            unimplemented !()
        }
        #[doc = "Returns composite character start position closest to the `pos`."]
        fn shaped_text_prev_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            unimplemented !()
        }
        #[doc = "Returns composite character position closest to the `pos`."]
        fn shaped_text_closest_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            unimplemented !()
        }
        #[doc = "Converts a number from Western Arabic (0..9) to the numeral system used in the given `language`.\n\nIf `language` is an empty string, the active locale will be used."]
        fn format_number(&self, number: GString, language: GString,) -> GString {
            unimplemented !()
        }
        #[doc = "Converts `number` from the numeral system used in the given `language` to Western Arabic (0..9).\n\nIf `language` is an empty string, the active locale will be used."]
        fn parse_number(&self, number: GString, language: GString,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns percent sign used in the given `language`."]
        fn percent_sign(&self, language: GString,) -> GString {
            unimplemented !()
        }
        #[doc = "Strips diacritics from the string."]
        fn strip_diacritics(&self, string: GString,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns `true` if `string` is a valid identifier."]
        fn is_valid_identifier(&self, string: GString,) -> bool {
            unimplemented !()
        }
        fn is_valid_letter(&self, unicode: u64,) -> bool {
            unimplemented !()
        }
        #[doc = "Returns an array of the word break boundaries. Elements in the returned array are the offsets of the start and end of words. Therefore the length of the array is always even."]
        fn string_get_word_breaks(&self, string: GString, language: GString, chars_per_line: i64,) -> PackedInt32Array {
            unimplemented !()
        }
        #[doc = "Returns array of the composite character boundaries."]
        fn string_get_character_breaks(&self, string: GString, language: GString,) -> PackedInt32Array {
            unimplemented !()
        }
        #[doc = "Returns index of the first string in `dict` which is visually confusable with the `string`, or `-1` if none is found."]
        fn is_confusable(&self, string: GString, dict: PackedStringArray,) -> i64 {
            unimplemented !()
        }
        #[doc = "Returns `true` if `string` is likely to be an attempt at confusing the reader."]
        fn spoof_check(&self, string: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Returns the string converted to `UPPERCASE`."]
        fn string_to_upper(&self, string: GString, language: GString,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns the string converted to `lowercase`."]
        fn string_to_lower(&self, string: GString, language: GString,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns the string converted to `Title Case`."]
        fn string_to_title(&self, string: GString, language: GString,) -> GString {
            unimplemented !()
        }
        #[doc = "Default implementation of the BiDi algorithm override function."]
        fn parse_structured_text(&self, parser_type: crate::classes::text_server::StructuredTextParser, args: VarArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        #[doc = "This method is called before text server is unregistered."]
        fn cleanup(&mut self,) {
            unimplemented !()
        }
    }
    impl TextServerExtension {
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
    impl crate::obj::GodotClass for TextServerExtension {
        type Base = crate::classes::TextServer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextServerExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextServerExtension {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::TextServer > for TextServerExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TextServerExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextServerExtension {
        
    }
    impl crate::obj::cap::GodotDefault for TextServerExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextServerExtension {
        type Target = crate::classes::TextServer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextServerExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextServerExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextServerExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextServerExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextServer > for $Class {
                
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
    use super::re_export::TextServerExtension;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TextServerExtension {
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
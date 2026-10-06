#![doc = "Sidecar module for class [`SystemFont`][crate::classes::SystemFont].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SystemFont` enums](https://docs.godotengine.org/en/stable/classes/class_systemfont.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SystemFont`.\n\nInherits [`Font`][crate::classes::Font].\n\nRelated symbols:\n\n* [`ISystemFont`][crate::classes::ISystemFont]: virtual methods\n\n\nSee also [Godot docs for `SystemFont`](https://docs.godotengine.org/en/stable/classes/class_systemfont.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`SystemFont::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`SystemFont` loads a font from a system font with the first matching name from \\[member font_names].\n\nIt will attempt to match font style, but it's not guaranteed.\n\nThe returned font might be part of a font collection or be a variable font with OpenType \"weight\", \"width\" and/or \"italic\" features set.\n\nYou can create [`FontVariation`][crate::classes::FontVariation] of the system font for precise control over its features.\n\n**Note:** This class is implemented on iOS, Linux, macOS and Windows, on other platforms it will fallback to default theme font."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SystemFont {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`SystemFont`][crate::classes::SystemFont].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IFont`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `SystemFont` methods](https://docs.godotengine.org/en/stable/classes/class_systemfont.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISystemFont: crate::obj::GodotClass < Base = SystemFont > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SystemFont {
        pub fn set_antialiasing(&mut self, antialiasing: crate::classes::text_server::FontAntialiasing,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::FontAntialiasing,);
            let args = (antialiasing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_antialiasing(&self,) -> crate::classes::text_server::FontAntialiasing {
            type CallRet = crate::classes::text_server::FontAntialiasing;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "get_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_embedded_bitmaps(&mut self, disable_embedded_bitmaps: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disable_embedded_bitmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_disable_embedded_bitmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_disable_embedded_bitmaps(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "get_disable_embedded_bitmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_generate_mipmaps(&mut self, generate_mipmaps: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (generate_mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_generate_mipmaps(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "get_generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_system_fallback(&mut self, allow_system_fallback: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow_system_fallback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_allow_system_fallback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_allow_system_fallback(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "is_allow_system_fallback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_force_autohinter(&mut self, force_autohinter: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (force_autohinter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_force_autohinter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_force_autohinter(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "is_force_autohinter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_modulate_color_glyphs(&mut self, modulate: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_modulate_color_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_modulate_color_glyphs(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "is_modulate_color_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hinting(&mut self, hinting: crate::classes::text_server::Hinting,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::Hinting,);
            let args = (hinting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_hinting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hinting(&self,) -> crate::classes::text_server::Hinting {
            type CallRet = crate::classes::text_server::Hinting;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "get_hinting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_subpixel_positioning(&mut self, subpixel_positioning: crate::classes::text_server::SubpixelPositioning,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::SubpixelPositioning,);
            let args = (subpixel_positioning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_subpixel_positioning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_subpixel_positioning(&self,) -> crate::classes::text_server::SubpixelPositioning {
            type CallRet = crate::classes::text_server::SubpixelPositioning;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "get_subpixel_positioning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keep_rounding_remainders(&mut self, keep_rounding_remainders: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (keep_rounding_remainders,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_keep_rounding_remainders", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keep_rounding_remainders(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "get_keep_rounding_remainders", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_multichannel_signed_distance_field(&mut self, msdf: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (msdf,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_multichannel_signed_distance_field", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_multichannel_signed_distance_field(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "is_multichannel_signed_distance_field", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_msdf_pixel_range(&mut self, msdf_pixel_range: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (msdf_pixel_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_msdf_pixel_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_msdf_pixel_range(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "get_msdf_pixel_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_msdf_size(&mut self, msdf_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (msdf_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_msdf_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_msdf_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "get_msdf_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_oversampling(&mut self, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_oversampling(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "get_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_names(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "get_font_names", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_names(&mut self, names: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(names),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_font_names", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_italic(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "get_font_italic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_italic(&mut self, italic: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (italic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_font_italic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_weight(&mut self, weight: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_font_weight", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_stretch(&mut self, stretch: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (stretch,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SystemFont", "set_font_stretch", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SystemFont {
        type Base = crate::classes::Font;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SystemFont"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SystemFont {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Font > for SystemFont {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for SystemFont {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SystemFont {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SystemFont {
        
    }
    impl crate::obj::cap::GodotDefault for SystemFont {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SystemFont {
        type Target = crate::classes::Font;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SystemFont {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SystemFont`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SystemFont__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SystemFont > for $Class {
                
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
    use super::re_export::SystemFont;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for SystemFont {
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
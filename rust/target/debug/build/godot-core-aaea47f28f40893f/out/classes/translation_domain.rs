#![doc = "Sidecar module for class [`TranslationDomain`][crate::classes::TranslationDomain].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TranslationDomain` enums](https://docs.godotengine.org/en/stable/classes/class_translationdomain.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TranslationDomain`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`translation_domain`][crate::classes::translation_domain]: sidecar module with related enum/flag types\n* [`ITranslationDomain`][crate::classes::ITranslationDomain]: virtual methods\n\n\nSee also [Godot docs for `TranslationDomain`](https://docs.godotengine.org/en/stable/classes/class_translationdomain.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TranslationDomain::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`TranslationDomain` is a self-contained collection of [`Translation`][crate::classes::Translation] resources. Translations can be added to or removed from it.\n\nIf you're working with the main translation domain, it is more convenient to use the wrap methods on [`TranslationServer`][crate::classes::TranslationServer]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TranslationDomain {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TranslationDomain`][crate::classes::TranslationDomain].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `TranslationDomain` methods](https://docs.godotengine.org/en/stable/classes/class_translationdomain.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITranslationDomain: crate::obj::GodotClass < Base = TranslationDomain > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TranslationDomain {
        #[doc = "Returns the [`Translation`][crate::classes::Translation] instance that best matches `locale`. Returns `null` if there are no matches."]
        pub fn get_translation_object(&self, locale: impl AsArg < GString >,) -> Option < Gd < crate::classes::Translation > > {
            type CallRet = Option < Gd < crate::classes::Translation > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "get_translation_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a translation."]
        pub fn add_translation(&mut self, translation: impl AsArg < Option < Gd < crate::classes::Translation >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Translation > > >,);
            let args = (translation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "add_translation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the given translation."]
        pub fn remove_translation(&mut self, translation: impl AsArg < Option < Gd < crate::classes::Translation >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Translation > > >,);
            let args = (translation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "remove_translation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all translations."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns all available [`Translation`][crate::classes::Translation] instances as added by [`add_translation`][`crate::classes::TranslationDomain::add_translation`]."]
        pub fn get_translations(&self,) -> Array < Gd < crate::classes::Translation > > {
            type CallRet = Array < Gd < crate::classes::Translation > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "get_translations", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there are any [`Translation`][crate::classes::Translation] instances that match `locale` (see [`compare_locales`][`crate::classes::TranslationServer::compare_locales`]). If `exact` is `true`, only instances whose locale exactly equals `locale` are considered."]
        pub fn has_translation_for_locale(&self, locale: impl AsArg < GString >, exact: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (locale.into_arg(), exact,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "has_translation_for_locale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this translation domain contains the given `translation`."]
        pub fn has_translation(&self, translation: impl AsArg < Option < Gd < crate::classes::Translation >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Translation > > >,);
            let args = (translation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "has_translation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Translation`][crate::classes::Translation] instances that match `locale` (see [`compare_locales`][`crate::classes::TranslationServer::compare_locales`]). If `exact` is `true`, only instances whose locale exactly equals `locale` will be returned."]
        pub fn find_translations(&self, locale: impl AsArg < GString >, exact: bool,) -> Array < Gd < crate::classes::Translation > > {
            type CallRet = Array < Gd < crate::classes::Translation > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (locale.into_arg(), exact,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "find_translations", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current locale's translation for the given message and context."]
        pub(crate) fn translate_full(&self, message: CowArg < StringName >, context: CowArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "translate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`translate_ex`][Self::translate_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current locale's translation for the given message and context."]
        #[inline]
        pub fn translate(&self, message: impl AsArg < StringName >,) -> StringName {
            self.translate_ex(message,) . done()
        }
        #[doc = "Returns the current locale's translation for the given message and context."]
        #[inline]
        pub fn translate_ex < 'ex > (&'ex self, message: impl AsArg < StringName > + 'ex,) -> ExTranslate < 'ex > {
            ExTranslate::new(self, message,)
        }
        #[doc = "Returns the current locale's translation for the given message, plural message and context.\n\nThe number `n` is the number or quantity of the plural object. It will be used to guide the translation system to fetch the correct plural form for the selected language."]
        pub(crate) fn translate_plural_full(&self, message: CowArg < StringName >, message_plural: CowArg < StringName >, n: i32, context: CowArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32, CowArg < 'a2, StringName >,);
            let args = (message, message_plural, n, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "translate_plural", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`translate_plural_ex`][Self::translate_plural_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current locale's translation for the given message, plural message and context.\n\nThe number `n` is the number or quantity of the plural object. It will be used to guide the translation system to fetch the correct plural form for the selected language."]
        #[inline]
        pub fn translate_plural(&self, message: impl AsArg < StringName >, message_plural: impl AsArg < StringName >, n: i32,) -> StringName {
            self.translate_plural_ex(message, message_plural, n,) . done()
        }
        #[doc = "Returns the current locale's translation for the given message, plural message and context.\n\nThe number `n` is the number or quantity of the plural object. It will be used to guide the translation system to fetch the correct plural form for the selected language."]
        #[inline]
        pub fn translate_plural_ex < 'ex > (&'ex self, message: impl AsArg < StringName > + 'ex, message_plural: impl AsArg < StringName > + 'ex, n: i32,) -> ExTranslatePlural < 'ex > {
            ExTranslatePlural::new(self, message, message_plural, n,)
        }
        #[doc = "Returns the locale override of the domain. Returns an empty string if locale override is disabled."]
        pub fn get_locale_override(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "get_locale_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the locale override of the domain.\n\nIf `locale` is an empty string, locale override is disabled. Otherwise, `locale` will be standardized to match known locales (e.g. `en-US` would be matched to `en_US`).\n\n**Note:** Calling this method does not automatically update texts in the scene tree. Please propagate the [`MainLoopNotification::TRANSLATION_CHANGED`][`crate::classes::notify::MainLoopNotification::TRANSLATION_CHANGED`] signal manually."]
        pub fn set_locale_override(&mut self, locale: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "set_locale_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "is_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_pseudolocalization_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "is_pseudolocalization_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pseudolocalization_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "set_pseudolocalization_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_pseudolocalization_accents_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "is_pseudolocalization_accents_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pseudolocalization_accents_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "set_pseudolocalization_accents_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_pseudolocalization_double_vowels_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "is_pseudolocalization_double_vowels_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pseudolocalization_double_vowels_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "set_pseudolocalization_double_vowels_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_pseudolocalization_fake_bidi_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "is_pseudolocalization_fake_bidi_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pseudolocalization_fake_bidi_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "set_pseudolocalization_fake_bidi_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_pseudolocalization_override_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "is_pseudolocalization_override_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pseudolocalization_override_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "set_pseudolocalization_override_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_pseudolocalization_skip_placeholders_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "is_pseudolocalization_skip_placeholders_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pseudolocalization_skip_placeholders_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "set_pseudolocalization_skip_placeholders_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pseudolocalization_expansion_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "get_pseudolocalization_expansion_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pseudolocalization_expansion_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "set_pseudolocalization_expansion_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pseudolocalization_prefix(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "get_pseudolocalization_prefix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pseudolocalization_prefix(&mut self, prefix: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (prefix.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "set_pseudolocalization_prefix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pseudolocalization_suffix(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "get_pseudolocalization_suffix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pseudolocalization_suffix(&mut self, suffix: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (suffix.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "set_pseudolocalization_suffix", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the pseudolocalized string based on the `message` passed in."]
        pub fn pseudolocalize(&self, message: impl AsArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (message.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationDomain", "pseudolocalize", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TranslationDomain {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TranslationDomain"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TranslationDomain {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TranslationDomain {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TranslationDomain {
        
    }
    impl crate::obj::cap::GodotDefault for TranslationDomain {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TranslationDomain {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TranslationDomain {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TranslationDomain`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TranslationDomain__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TranslationDomain > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TranslationDomain::translate_ex`][super::TranslationDomain::translate_ex]."]
#[must_use]
pub struct ExTranslate < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TranslationDomain, message: CowArg < 'ex, StringName >, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExTranslate < 'ex > {
    fn new(surround_object: &'ex re_export::TranslationDomain, message: impl AsArg < StringName > + 'ex,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        let Self {
            _phantom, surround_object, message, context,
        }
        = self;
        re_export::TranslationDomain::translate_full(surround_object, message, context,)
    }
}
#[doc = "Default-param extender for [`TranslationDomain::translate_plural_ex`][super::TranslationDomain::translate_plural_ex]."]
#[must_use]
pub struct ExTranslatePlural < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TranslationDomain, message: CowArg < 'ex, StringName >, message_plural: CowArg < 'ex, StringName >, n: i32, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExTranslatePlural < 'ex > {
    fn new(surround_object: &'ex re_export::TranslationDomain, message: impl AsArg < StringName > + 'ex, message_plural: impl AsArg < StringName > + 'ex, n: i32,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), message_plural: message_plural.into_arg(), n: n, context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        let Self {
            _phantom, surround_object, message, message_plural, n, context,
        }
        = self;
        re_export::TranslationDomain::translate_plural_full(surround_object, message, message_plural, n, context,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TranslationDomain;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TranslationDomain {
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
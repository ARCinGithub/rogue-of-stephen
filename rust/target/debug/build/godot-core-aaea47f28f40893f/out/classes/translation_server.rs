#![doc = "Sidecar module for class [`TranslationServer`][crate::classes::TranslationServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TranslationServer` enums](https://docs.godotengine.org/en/stable/classes/class_translationserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TranslationServer`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`translation_server`][crate::classes::translation_server]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `TranslationServer`](https://docs.godotengine.org/en/stable/classes/class_translationserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThe translation server is the API backend that manages all language translations.\n\nTranslations are stored in [`TranslationDomain`][crate::classes::TranslationDomain]s, which can be accessed by name. The most commonly used translation domain is the main translation domain. It always exists and can be accessed using an empty [`StringName`][crate::builtin::StringName]. The translation server provides wrapper methods for accessing the main translation domain directly, without having to fetch the translation domain first. Custom translation domains are mainly for advanced usages like editor plugins. Names starting with `godot.` are reserved for engine internals."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TranslationServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl TranslationServer {
        #[doc = "Sets the locale of the project. The `locale` string will be standardized to match known locales (e.g. `en-US` would be matched to `en_US`).\n\nIf translations have been loaded beforehand for the new locale, they will be applied."]
        pub fn set_locale(&mut self, locale: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(52usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "set_locale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current locale of the project.\n\nSee also [`get_locale`][`crate::classes::Os::get_locale`] and [`get_locale_language`][`crate::classes::Os::get_locale_language`] to query the locale of the user system."]
        pub fn get_locale(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(53usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_locale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current locale of the editor.\n\n**Note:** When called from an exported project returns the same value as [`get_locale`][`crate::classes::TranslationServer::get_locale`]."]
        pub fn get_tool_locale(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(54usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_tool_locale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Compares two locales and returns a similarity score between `0` (no match) and `10` (full match)."]
        pub fn compare_locales(&self, locale_a: impl AsArg < GString >, locale_b: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (locale_a.into_arg(), locale_b.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(55usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "compare_locales", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a `locale` string standardized to match known locales (e.g. `en-US` would be matched to `en_US`). If `add_defaults` is `true`, the locale may have a default script or country added."]
        pub(crate) fn standardize_locale_full(&self, locale: CowArg < GString >, add_defaults: bool,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (locale, add_defaults,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(56usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "standardize_locale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`standardize_locale_ex`][Self::standardize_locale_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a `locale` string standardized to match known locales (e.g. `en-US` would be matched to `en_US`). If `add_defaults` is `true`, the locale may have a default script or country added."]
        #[inline]
        pub fn standardize_locale(&self, locale: impl AsArg < GString >,) -> GString {
            self.standardize_locale_ex(locale,) . done()
        }
        #[doc = "Returns a `locale` string standardized to match known locales (e.g. `en-US` would be matched to `en_US`). If `add_defaults` is `true`, the locale may have a default script or country added."]
        #[inline]
        pub fn standardize_locale_ex < 'ex > (&'ex self, locale: impl AsArg < GString > + 'ex,) -> ExStandardizeLocale < 'ex > {
            ExStandardizeLocale::new(self, locale,)
        }
        #[doc = "Returns array of known language codes."]
        pub fn get_all_languages(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(57usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_all_languages", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a readable language name for the `language` code."]
        pub fn get_language_name(&self, language: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(58usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_language_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of known script codes."]
        pub fn get_all_scripts(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(59usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_all_scripts", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a readable script name for the `script` code."]
        pub fn get_script_name(&self, script: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(60usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_script_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of known country codes."]
        pub fn get_all_countries(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(61usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_all_countries", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a readable country name for the `country` code."]
        pub fn get_country_name(&self, country: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (country.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(62usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_country_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a locale's language and its variant (e.g. `\"en_US\"` would return `\"English (United States)\"`)."]
        pub fn get_locale_name(&self, locale: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(63usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_locale_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the default plural rules for the `locale`."]
        pub fn get_plural_rules(&self, locale: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(64usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_plural_rules", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current locale's translation for the given message and context.\n\n**Note:** This method always uses the main translation domain."]
        pub(crate) fn translate_full(&self, message: CowArg < StringName >, context: CowArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (message, context,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(65usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "translate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`translate_ex`][Self::translate_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current locale's translation for the given message and context.\n\n**Note:** This method always uses the main translation domain."]
        #[inline]
        pub fn translate(&self, message: impl AsArg < StringName >,) -> StringName {
            self.translate_ex(message,) . done()
        }
        #[doc = "Returns the current locale's translation for the given message and context.\n\n**Note:** This method always uses the main translation domain."]
        #[inline]
        pub fn translate_ex < 'ex > (&'ex self, message: impl AsArg < StringName > + 'ex,) -> ExTranslate < 'ex > {
            ExTranslate::new(self, message,)
        }
        #[doc = "Returns the current locale's translation for the given message, plural message and context.\n\nThe number `n` is the number or quantity of the plural object. It will be used to guide the translation system to fetch the correct plural form for the selected language.\n\n**Note:** This method always uses the main translation domain."]
        pub(crate) fn translate_plural_full(&self, message: CowArg < StringName >, plural_message: CowArg < StringName >, n: i32, context: CowArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32, CowArg < 'a2, StringName >,);
            let args = (message, plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(66usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "translate_plural", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`translate_plural_ex`][Self::translate_plural_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current locale's translation for the given message, plural message and context.\n\nThe number `n` is the number or quantity of the plural object. It will be used to guide the translation system to fetch the correct plural form for the selected language.\n\n**Note:** This method always uses the main translation domain."]
        #[inline]
        pub fn translate_plural(&self, message: impl AsArg < StringName >, plural_message: impl AsArg < StringName >, n: i32,) -> StringName {
            self.translate_plural_ex(message, plural_message, n,) . done()
        }
        #[doc = "Returns the current locale's translation for the given message, plural message and context.\n\nThe number `n` is the number or quantity of the plural object. It will be used to guide the translation system to fetch the correct plural form for the selected language.\n\n**Note:** This method always uses the main translation domain."]
        #[inline]
        pub fn translate_plural_ex < 'ex > (&'ex self, message: impl AsArg < StringName > + 'ex, plural_message: impl AsArg < StringName > + 'ex, n: i32,) -> ExTranslatePlural < 'ex > {
            ExTranslatePlural::new(self, message, plural_message, n,)
        }
        #[doc = "Adds a translation to the main translation domain."]
        pub fn add_translation(&mut self, translation: impl AsArg < Option < Gd < crate::classes::Translation >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Translation > > >,);
            let args = (translation.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(67usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "add_translation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the given translation from the main translation domain."]
        pub fn remove_translation(&mut self, translation: impl AsArg < Option < Gd < crate::classes::Translation >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Translation > > >,);
            let args = (translation.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(68usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "remove_translation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Translation`][crate::classes::Translation] instance that best matches `locale` in the main translation domain. Returns `null` if there are no matches."]
        pub fn get_translation_object(&self, locale: impl AsArg < GString >,) -> Option < Gd < crate::classes::Translation > > {
            type CallRet = Option < Gd < crate::classes::Translation > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(69usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_translation_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns all available [`Translation`][crate::classes::Translation] instances in the main translation domain as added by [`add_translation`][`crate::classes::TranslationServer::add_translation`]."]
        pub fn get_translations(&self,) -> Array < Gd < crate::classes::Translation > > {
            type CallRet = Array < Gd < crate::classes::Translation > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(70usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_translations", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Translation`][crate::classes::Translation] instances in the main translation domain that match `locale` (see [`compare_locales`][`crate::classes::TranslationServer::compare_locales`]). If `exact` is `true`, only instances whose locale exactly equals `locale` will be returned."]
        pub fn find_translations(&self, locale: impl AsArg < GString >, exact: bool,) -> Array < Gd < crate::classes::Translation > > {
            type CallRet = Array < Gd < crate::classes::Translation > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (locale.into_arg(), exact,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(71usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "find_translations", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there are any [`Translation`][crate::classes::Translation] instances in the main translation domain that match `locale` (see [`compare_locales`][`crate::classes::TranslationServer::compare_locales`]). If `exact` is `true`, only instances whose locale exactly equals `locale` are considered."]
        pub fn has_translation_for_locale(&self, locale: impl AsArg < GString >, exact: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (locale.into_arg(), exact,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(72usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "has_translation_for_locale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the main translation domain contains the given `translation`."]
        pub fn has_translation(&self, translation: impl AsArg < Option < Gd < crate::classes::Translation >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Translation > > >,);
            let args = (translation.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(73usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "has_translation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a translation domain with the specified name exists."]
        pub fn has_domain(&self, domain: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (domain.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(74usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "has_domain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the translation domain with the specified name. An empty translation domain will be created and added if it does not exist."]
        pub fn get_or_add_domain(&self, domain: impl AsArg < StringName >,) -> Option < Gd < crate::classes::TranslationDomain > > {
            type CallRet = Option < Gd < crate::classes::TranslationDomain > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (domain.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(75usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_or_add_domain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the translation domain with the specified name.\n\n**Note:** Trying to remove the main translation domain is an error."]
        pub fn remove_domain(&mut self, domain: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (domain.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(76usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "remove_domain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all translations from the main translation domain."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(77usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all loaded locales of the project."]
        pub fn get_loaded_locales(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(78usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_loaded_locales", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts a number from Western Arabic (0..9) to the numeral system used in the given `locale`."]
        pub fn format_number(&self, number: impl AsArg < GString >, locale: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (number.into_arg(), locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(79usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "format_number", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the percent sign used in the given `locale`."]
        pub fn get_percent_sign(&self, locale: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(80usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_percent_sign", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts `number` from the numeral system used in the given `locale` to Western Arabic (0..9)."]
        pub fn parse_number(&self, number: impl AsArg < GString >, locale: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (number.into_arg(), locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(81usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "parse_number", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_pseudolocalization_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(82usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "is_pseudolocalization_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pseudolocalization_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(83usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "set_pseudolocalization_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Reparses the pseudolocalization options and reloads the translation for the main translation domain."]
        pub fn reload_pseudolocalization(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(84usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "reload_pseudolocalization", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the pseudolocalized string based on the `message` passed in.\n\n**Note:** This method always uses the main translation domain."]
        pub fn pseudolocalize(&self, message: impl AsArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (message.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(85usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "pseudolocalize", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TranslationServer {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TranslationServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for TranslationServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TranslationServer {
        
    }
    impl crate::obj::Singleton for TranslationServer {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"TranslationServer"))
            }
        }
    }
    impl std::ops::Deref for TranslationServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TranslationServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TranslationServer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `TranslationServer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`TranslationServer::standardize_locale_ex`][super::TranslationServer::standardize_locale_ex]."]
#[must_use]
pub struct ExStandardizeLocale < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TranslationServer, locale: CowArg < 'ex, GString >, add_defaults: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExStandardizeLocale < 'ex > {
    fn new(surround_object: &'ex re_export::TranslationServer, locale: impl AsArg < GString > + 'ex,) -> Self {
        let add_defaults = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, locale: locale.into_arg(), add_defaults: add_defaults,
        }
    }
    #[inline]
    pub fn add_defaults(self, add_defaults: bool) -> Self {
        Self {
            add_defaults: add_defaults, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, locale, add_defaults,
        }
        = self;
        re_export::TranslationServer::standardize_locale_full(surround_object, locale, add_defaults,)
    }
}
#[doc = "Default-param extender for [`TranslationServer::translate_ex`][super::TranslationServer::translate_ex]."]
#[must_use]
pub struct ExTranslate < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TranslationServer, message: CowArg < 'ex, StringName >, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExTranslate < 'ex > {
    fn new(surround_object: &'ex re_export::TranslationServer, message: impl AsArg < StringName > + 'ex,) -> Self {
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
        re_export::TranslationServer::translate_full(surround_object, message, context,)
    }
}
#[doc = "Default-param extender for [`TranslationServer::translate_plural_ex`][super::TranslationServer::translate_plural_ex]."]
#[must_use]
pub struct ExTranslatePlural < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TranslationServer, message: CowArg < 'ex, StringName >, plural_message: CowArg < 'ex, StringName >, n: i32, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExTranslatePlural < 'ex > {
    fn new(surround_object: &'ex re_export::TranslationServer, message: impl AsArg < StringName > + 'ex, plural_message: impl AsArg < StringName > + 'ex, n: i32,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), plural_message: plural_message.into_arg(), n: n, context: CowArg::Owned(context),
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
            _phantom, surround_object, message, plural_message, n, context,
        }
        = self;
        re_export::TranslationServer::translate_plural_full(surround_object, message, plural_message, n, context,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TranslationServer;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TranslationServer {
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
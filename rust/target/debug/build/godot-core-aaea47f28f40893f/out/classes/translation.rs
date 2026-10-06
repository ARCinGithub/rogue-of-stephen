#![doc = "Sidecar module for class [`Translation`][crate::classes::Translation].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Translation` enums](https://docs.godotengine.org/en/stable/classes/class_translation.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Translation`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`translation`][crate::classes::translation]: sidecar module with related enum/flag types\n* [`ITranslation`][crate::classes::ITranslation]: virtual methods\n\n\nSee also [Godot docs for `Translation`](https://docs.godotengine.org/en/stable/classes/class_translation.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Translation::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`Translation` maps a collection of strings to their individual translations, and also provides convenience methods for pluralization.\n\nA `Translation` consists of messages. A message is identified by its context and untranslated string. Unlike [gettext](https://www.gnu.org/software/gettext/), using an empty context string in Godot means not using any context."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Translation {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Translation`][crate::classes::Translation].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Translation` methods](https://docs.godotengine.org/en/stable/classes/class_translation.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITranslation: crate::obj::GodotClass < Base = Translation > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Virtual method to override [`get_plural_message`][`crate::classes::Translation::get_plural_message`]."]
        fn get_plural_message(&self, src_message: StringName, src_plural_message: StringName, n: i32, context: StringName,) -> StringName {
            unimplemented !()
        }
        #[doc = "Virtual method to override [`get_message`][`crate::classes::Translation::get_message`]."]
        fn get_message(&self, src_message: StringName, context: StringName,) -> StringName {
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
    impl Translation {
        pub fn set_locale(&mut self, locale: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "set_locale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_locale(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_locale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a message if nonexistent, followed by its translation.\n\nAn additional context could be used to specify the translation context or differentiate polysemic words."]
        pub(crate) fn add_message_full(&mut self, src_message: CowArg < StringName >, xlated_message: CowArg < StringName >, context: CowArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (src_message, xlated_message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "add_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_message_ex`][Self::add_message_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a message if nonexistent, followed by its translation.\n\nAn additional context could be used to specify the translation context or differentiate polysemic words."]
        #[inline]
        pub fn add_message(&mut self, src_message: impl AsArg < StringName >, xlated_message: impl AsArg < StringName >,) {
            self.add_message_ex(src_message, xlated_message,) . done()
        }
        #[doc = "Adds a message if nonexistent, followed by its translation.\n\nAn additional context could be used to specify the translation context or differentiate polysemic words."]
        #[inline]
        pub fn add_message_ex < 'ex > (&'ex mut self, src_message: impl AsArg < StringName > + 'ex, xlated_message: impl AsArg < StringName > + 'ex,) -> ExAddMessage < 'ex > {
            ExAddMessage::new(self, src_message, xlated_message,)
        }
        #[doc = "Adds a message involving plural translation if nonexistent, followed by its translation.\n\nAn additional context could be used to specify the translation context or differentiate polysemic words."]
        pub(crate) fn add_plural_message_full(&mut self, src_message: CowArg < StringName >, xlated_messages: RefArg < PackedStringArray >, context: CowArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, RefArg < 'a1, PackedStringArray >, CowArg < 'a2, StringName >,);
            let args = (src_message, xlated_messages, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "add_plural_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_plural_message_ex`][Self::add_plural_message_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a message involving plural translation if nonexistent, followed by its translation.\n\nAn additional context could be used to specify the translation context or differentiate polysemic words."]
        #[inline]
        pub fn add_plural_message(&mut self, src_message: impl AsArg < StringName >, xlated_messages: &PackedStringArray,) {
            self.add_plural_message_ex(src_message, xlated_messages,) . done()
        }
        #[doc = "Adds a message involving plural translation if nonexistent, followed by its translation.\n\nAn additional context could be used to specify the translation context or differentiate polysemic words."]
        #[inline]
        pub fn add_plural_message_ex < 'ex > (&'ex mut self, src_message: impl AsArg < StringName > + 'ex, xlated_messages: &'ex PackedStringArray,) -> ExAddPluralMessage < 'ex > {
            ExAddPluralMessage::new(self, src_message, xlated_messages,)
        }
        #[doc = "Returns a message's translation."]
        pub(crate) fn get_message_full(&self, src_message: CowArg < StringName >, context: CowArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (src_message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_message_ex`][Self::get_message_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a message's translation."]
        #[inline]
        pub fn get_message(&self, src_message: impl AsArg < StringName >,) -> StringName {
            self.get_message_ex(src_message,) . done()
        }
        #[doc = "Returns a message's translation."]
        #[inline]
        pub fn get_message_ex < 'ex > (&'ex self, src_message: impl AsArg < StringName > + 'ex,) -> ExGetMessage < 'ex > {
            ExGetMessage::new(self, src_message,)
        }
        #[doc = "Returns a message's translation involving plurals.\n\nThe number `n` is the number or quantity of the plural object. It will be used to guide the translation system to fetch the correct plural form for the selected language.\n\n**Note:** Plurals are only supported in [gettext-based translations (PO)]($DOCS_URL/tutorials/i18n/localization_using_gettext.html), not CSV."]
        pub(crate) fn get_plural_message_full(&self, src_message: CowArg < StringName >, src_plural_message: CowArg < StringName >, n: i32, context: CowArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32, CowArg < 'a2, StringName >,);
            let args = (src_message, src_plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_plural_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_plural_message_ex`][Self::get_plural_message_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a message's translation involving plurals.\n\nThe number `n` is the number or quantity of the plural object. It will be used to guide the translation system to fetch the correct plural form for the selected language.\n\n**Note:** Plurals are only supported in [gettext-based translations (PO)]($DOCS_URL/tutorials/i18n/localization_using_gettext.html), not CSV."]
        #[inline]
        pub fn get_plural_message(&self, src_message: impl AsArg < StringName >, src_plural_message: impl AsArg < StringName >, n: i32,) -> StringName {
            self.get_plural_message_ex(src_message, src_plural_message, n,) . done()
        }
        #[doc = "Returns a message's translation involving plurals.\n\nThe number `n` is the number or quantity of the plural object. It will be used to guide the translation system to fetch the correct plural form for the selected language.\n\n**Note:** Plurals are only supported in [gettext-based translations (PO)]($DOCS_URL/tutorials/i18n/localization_using_gettext.html), not CSV."]
        #[inline]
        pub fn get_plural_message_ex < 'ex > (&'ex self, src_message: impl AsArg < StringName > + 'ex, src_plural_message: impl AsArg < StringName > + 'ex, n: i32,) -> ExGetPluralMessage < 'ex > {
            ExGetPluralMessage::new(self, src_message, src_plural_message, n,)
        }
        #[doc = "Erases a message."]
        pub(crate) fn erase_message_full(&mut self, src_message: CowArg < StringName >, context: CowArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (src_message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "erase_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`erase_message_ex`][Self::erase_message_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Erases a message."]
        #[inline]
        pub fn erase_message(&mut self, src_message: impl AsArg < StringName >,) {
            self.erase_message_ex(src_message,) . done()
        }
        #[doc = "Erases a message."]
        #[inline]
        pub fn erase_message_ex < 'ex > (&'ex mut self, src_message: impl AsArg < StringName > + 'ex,) -> ExEraseMessage < 'ex > {
            ExEraseMessage::new(self, src_message,)
        }
        #[doc = "Returns the keys of all messages, that is, the context and untranslated strings of each message.\n\n**Note:** If a message does not use a context, the corresponding element is the untranslated string. Otherwise, the corresponding element is the context and untranslated string separated by the EOT character (`U+0004`). This is done for compatibility purposes.\n\n```gdscript\nfor key in translation.get_message_list():\n\tvar p = key.find(\"\\u0004\")\n\tif p == -1:\n\t\tvar untranslated = key\n\t\tprint(\"Message %s\" % untranslated)\n\telse:\n\t\tvar context = key.substr(0, p)\n\t\tvar untranslated = key.substr(p + 1)\n\t\tprint(\"Message %s with context %s\" % [untranslated, context])\n```"]
        pub fn get_message_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_message_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns all the translated strings."]
        pub fn get_translated_message_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_translated_message_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of existing messages."]
        pub fn get_message_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_message_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_plural_rules_override(&mut self, rules: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (rules.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "set_plural_rules_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_plural_rules_override(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_plural_rules_override", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Translation {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Translation"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Translation {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Translation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Translation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Translation {
        
    }
    impl crate::obj::cap::GodotDefault for Translation {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Translation {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Translation {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Translation`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Translation__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Translation > for $Class {
                
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
#[doc = "Default-param extender for [`Translation::add_message_ex`][super::Translation::add_message_ex]."]
#[must_use]
pub struct ExAddMessage < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Translation, src_message: CowArg < 'ex, StringName >, xlated_message: CowArg < 'ex, StringName >, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddMessage < 'ex > {
    fn new(surround_object: &'ex mut re_export::Translation, src_message: impl AsArg < StringName > + 'ex, xlated_message: impl AsArg < StringName > + 'ex,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, src_message: src_message.into_arg(), xlated_message: xlated_message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, src_message, xlated_message, context,
        }
        = self;
        re_export::Translation::add_message_full(surround_object, src_message, xlated_message, context,)
    }
}
#[doc = "Default-param extender for [`Translation::add_plural_message_ex`][super::Translation::add_plural_message_ex]."]
#[must_use]
pub struct ExAddPluralMessage < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Translation, src_message: CowArg < 'ex, StringName >, xlated_messages: CowArg < 'ex, PackedStringArray >, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddPluralMessage < 'ex > {
    fn new(surround_object: &'ex mut re_export::Translation, src_message: impl AsArg < StringName > + 'ex, xlated_messages: &'ex PackedStringArray,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, src_message: src_message.into_arg(), xlated_messages: CowArg::Borrowed(xlated_messages), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, src_message, xlated_messages, context,
        }
        = self;
        re_export::Translation::add_plural_message_full(surround_object, src_message, xlated_messages.cow_as_arg(), context,)
    }
}
#[doc = "Default-param extender for [`Translation::get_message_ex`][super::Translation::get_message_ex]."]
#[must_use]
pub struct ExGetMessage < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Translation, src_message: CowArg < 'ex, StringName >, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetMessage < 'ex > {
    fn new(surround_object: &'ex re_export::Translation, src_message: impl AsArg < StringName > + 'ex,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, src_message: src_message.into_arg(), context: CowArg::Owned(context),
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
            _phantom, surround_object, src_message, context,
        }
        = self;
        re_export::Translation::get_message_full(surround_object, src_message, context,)
    }
}
#[doc = "Default-param extender for [`Translation::get_plural_message_ex`][super::Translation::get_plural_message_ex]."]
#[must_use]
pub struct ExGetPluralMessage < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Translation, src_message: CowArg < 'ex, StringName >, src_plural_message: CowArg < 'ex, StringName >, n: i32, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetPluralMessage < 'ex > {
    fn new(surround_object: &'ex re_export::Translation, src_message: impl AsArg < StringName > + 'ex, src_plural_message: impl AsArg < StringName > + 'ex, n: i32,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, src_message: src_message.into_arg(), src_plural_message: src_plural_message.into_arg(), n: n, context: CowArg::Owned(context),
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
            _phantom, surround_object, src_message, src_plural_message, n, context,
        }
        = self;
        re_export::Translation::get_plural_message_full(surround_object, src_message, src_plural_message, n, context,)
    }
}
#[doc = "Default-param extender for [`Translation::erase_message_ex`][super::Translation::erase_message_ex]."]
#[must_use]
pub struct ExEraseMessage < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Translation, src_message: CowArg < 'ex, StringName >, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExEraseMessage < 'ex > {
    fn new(surround_object: &'ex mut re_export::Translation, src_message: impl AsArg < StringName > + 'ex,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, src_message: src_message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, src_message, context,
        }
        = self;
        re_export::Translation::erase_message_full(surround_object, src_message, context,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Translation;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Translation {
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
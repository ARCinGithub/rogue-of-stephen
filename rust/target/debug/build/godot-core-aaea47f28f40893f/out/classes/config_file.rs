#![doc = "Sidecar module for class [`ConfigFile`][crate::classes::ConfigFile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ConfigFile` enums](https://docs.godotengine.org/en/stable/classes/class_configfile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ConfigFile`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`config_file`][crate::classes::config_file]: sidecar module with related enum/flag types\n* [`IConfigFile`][crate::classes::IConfigFile]: virtual methods\n\n\nSee also [Godot docs for `ConfigFile`](https://docs.godotengine.org/en/stable/classes/class_configfile.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ConfigFile::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThis helper class can be used to store [`Variant`][crate::builtin::Variant] values on the filesystem using INI-style formatting. The stored values are identified by a section and a key:\n\n```text\n[section]\nsome_key=42\nstring_example=\"Hello World3D!\"\na_vector=Vector3(1, 0, 2)\n```\n\nThe stored data can be saved to or parsed from a file, though ConfigFile objects can also be used directly without accessing the filesystem.\n\nThe following example shows how to create a simple `ConfigFile` and save it on disc:\n\n\n```gdscript\n# Create new ConfigFile object.\nvar config = ConfigFile.new()\n\n# Store some values.\nconfig.set_value(\"Player1\", \"player_name\", \"Steve\")\nconfig.set_value(\"Player1\", \"best_score\", 10)\nconfig.set_value(\"Player2\", \"player_name\", \"V3geta\")\nconfig.set_value(\"Player2\", \"best_score\", 9001)\n\n# Save it to a file (overwrite if already exists).\nconfig.save(\"user://scores.cfg\")\n```\n\n\nThis example shows how the above file could be loaded:\n\n\n```gdscript\nvar score_data = {}\nvar config = ConfigFile.new()\n\n# Load data from a file.\nvar err = config.load(\"user://scores.cfg\")\n\n# If the file didn't load, ignore it.\nif err != OK:\n\treturn\n\n# Iterate over all sections.\nfor player in config.get_sections():\n\t# Fetch the data for each section.\n\tvar player_name = config.get_value(player, \"player_name\")\n\tvar player_score = config.get_value(player, \"best_score\")\n\tscore_data[player_name] = player_score\n```\n\n\nAny operation that mutates the ConfigFile such as [`set_value`][`crate::classes::ConfigFile::set_value`], [`clear`][`crate::classes::ConfigFile::clear`], or [`erase_section`][`crate::classes::ConfigFile::erase_section`], only changes what is loaded in memory. If you want to write the change to a file, you have to save the changes with [`save`][`crate::classes::ConfigFile::save`], [`save_encrypted`][`crate::classes::ConfigFile::save_encrypted`], or [`save_encrypted_pass`][`crate::classes::ConfigFile::save_encrypted_pass`].\n\nKeep in mind that section and property names can't contain spaces. Anything after a space will be ignored on save and on load.\n\nConfigFiles can also contain manually written comment lines starting with a semicolon (`;`). Those lines will be ignored when parsing the file. Note that comments will be lost when saving the ConfigFile. This can still be useful for dedicated server configuration files, which are typically never overwritten without explicit user action.\n\n**Note:** The file extension given to a ConfigFile does not have any impact on its formatting or behavior. By convention, the `.cfg` extension is used here, but any other extension such as `.ini` is also valid. Since neither `.cfg` nor `.ini` are standardized, Godot's ConfigFile formatting may differ from files written by other programs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ConfigFile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ConfigFile`][crate::classes::ConfigFile].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ConfigFile` methods](https://docs.godotengine.org/en/stable/classes/class_configfile.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IConfigFile: crate::obj::GodotClass < Base = ConfigFile > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ConfigFile {
        #[doc = "Assigns a value to the specified key of the specified section. If either the section or the key do not exist, they are created. Passing a `null` value deletes the specified key if it exists, and deletes the section if it ends up empty once the key has been removed."]
        pub fn set_value(&mut self, section: impl AsArg < GString >, key: impl AsArg < GString >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Variant >,);
            let args = (section.into_arg(), key.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "set_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current value for the specified section and key. If either the section or the key do not exist, the method returns the fallback `default` value. If `default` is not specified or set to `null`, an error is also raised."]
        pub(crate) fn get_value_full(&self, section: CowArg < GString >, key: CowArg < GString >, default: RefArg < Variant >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Variant >,);
            let args = (section, key, default,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "get_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_value_ex`][Self::get_value_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current value for the specified section and key. If either the section or the key do not exist, the method returns the fallback `default` value. If `default` is not specified or set to `null`, an error is also raised."]
        #[inline]
        pub fn get_value(&self, section: impl AsArg < GString >, key: impl AsArg < GString >,) -> Variant {
            self.get_value_ex(section, key,) . done()
        }
        #[doc = "Returns the current value for the specified section and key. If either the section or the key do not exist, the method returns the fallback `default` value. If `default` is not specified or set to `null`, an error is also raised."]
        #[inline]
        pub fn get_value_ex < 'ex > (&'ex self, section: impl AsArg < GString > + 'ex, key: impl AsArg < GString > + 'ex,) -> ExGetValue < 'ex > {
            ExGetValue::new(self, section, key,)
        }
        #[doc = "Returns `true` if the specified section exists."]
        pub fn has_section(&self, section: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (section.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "has_section", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the specified section-key pair exists."]
        pub fn has_section_key(&self, section: impl AsArg < GString >, key: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (section.into_arg(), key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "has_section_key", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all defined section identifiers."]
        pub fn get_sections(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "get_sections", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all defined key identifiers in the specified section. Raises an error and returns an empty array if the section does not exist."]
        pub fn get_section_keys(&self, section: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (section.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "get_section_keys", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deletes the specified section along with all the key-value pairs inside. Raises an error if the section does not exist."]
        pub fn erase_section(&mut self, section: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (section.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "erase_section", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deletes the specified key in a section. Raises an error if either the section or the key do not exist."]
        pub fn erase_section_key(&mut self, section: impl AsArg < GString >, key: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (section.into_arg(), key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "erase_section_key", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads the config file specified as a parameter. The file's contents are parsed and loaded in the `ConfigFile` object which the method was called on.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the other \\[enum Error] values if the operation failed."]
        pub fn load(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "load", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Parses the passed string as the contents of a config file. The string is parsed and loaded in the ConfigFile object which the method was called on.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the other \\[enum Error] values if the operation failed."]
        pub fn parse(&mut self, data: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (data.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "parse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves the contents of the `ConfigFile` object to the file specified as a parameter. The output file uses an INI-style structure.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the other \\[enum Error] values if the operation failed."]
        pub fn save(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "save", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Obtain the text version of this config file (the same text that would be written to a file)."]
        pub fn encode_to_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "encode_to_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads the encrypted config file specified as a parameter, using the provided `key` to decrypt it. The file's contents are parsed and loaded in the `ConfigFile` object which the method was called on.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the other \\[enum Error] values if the operation failed."]
        pub fn load_encrypted(&mut self, path: impl AsArg < GString >, key: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedByteArray >,);
            let args = (path.into_arg(), RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "load_encrypted", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads the encrypted config file specified as a parameter, using the provided `password` to decrypt it. The file's contents are parsed and loaded in the `ConfigFile` object which the method was called on.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the other \\[enum Error] values if the operation failed."]
        pub fn load_encrypted_pass(&mut self, path: impl AsArg < GString >, password: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (path.into_arg(), password.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "load_encrypted_pass", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves the contents of the `ConfigFile` object to the AES-256 encrypted file specified as a parameter, using the provided `key` to encrypt it. The output file uses an INI-style structure.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the other \\[enum Error] values if the operation failed."]
        pub fn save_encrypted(&mut self, path: impl AsArg < GString >, key: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedByteArray >,);
            let args = (path.into_arg(), RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "save_encrypted", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves the contents of the `ConfigFile` object to the AES-256 encrypted file specified as a parameter, using the provided `password` to encrypt it. The output file uses an INI-style structure.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or one of the other \\[enum Error] values if the operation failed."]
        pub fn save_encrypted_pass(&mut self, path: impl AsArg < GString >, password: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (path.into_arg(), password.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "save_encrypted_pass", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the entire contents of the config."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConfigFile", "clear", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ConfigFile {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ConfigFile"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ConfigFile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ConfigFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ConfigFile {
        
    }
    impl crate::obj::cap::GodotDefault for ConfigFile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ConfigFile {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ConfigFile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ConfigFile`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ConfigFile__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ConfigFile > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ConfigFile::get_value_ex`][super::ConfigFile::get_value_ex]."]
#[must_use]
pub struct ExGetValue < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::ConfigFile, section: CowArg < 'ex, GString >, key: CowArg < 'ex, GString >, default: CowArg < 'ex, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetValue < 'ex > {
    fn new(surround_object: &'ex re_export::ConfigFile, section: impl AsArg < GString > + 'ex, key: impl AsArg < GString > + 'ex,) -> Self {
        let default = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, section: section.into_arg(), key: key.into_arg(), default: CowArg::Owned(default),
        }
    }
    #[inline]
    pub fn default(self, default: &'ex Variant) -> Self {
        Self {
            default: CowArg::Borrowed(default), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, section, key, default,
        }
        = self;
        re_export::ConfigFile::get_value_full(surround_object, section, key, default.cow_as_arg(),)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ConfigFile;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ConfigFile {
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
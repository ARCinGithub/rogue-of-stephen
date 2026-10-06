#![doc = "Sidecar module for class [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ResourceFormatLoader` enums](https://docs.godotengine.org/en/stable/classes/class_resourceformatloader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ResourceFormatLoader`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`resource_format_loader`][crate::classes::resource_format_loader]: sidecar module with related enum/flag types\n* [`IResourceFormatLoader`][crate::classes::IResourceFormatLoader]: virtual methods\n\n\nSee also [Godot docs for `ResourceFormatLoader`](https://docs.godotengine.org/en/stable/classes/class_resourceformatloader.html).\n\n# Specific notes for this class\n\nEnable the `experimental-threads` feature when using custom `ResourceFormatLoader`s. Otherwise the application will panic when the custom `ResourceFormatLoader` is used by Godot in a thread other than the main thread."]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ResourceFormatLoader::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nGodot loads resources in the editor or in exported games using ResourceFormatLoaders. They are queried automatically via the [`ResourceLoader`][crate::classes::ResourceLoader] singleton, or when a resource with internal dependencies is loaded. Each file type may load as a different resource type, so multiple ResourceFormatLoaders are registered in the engine.\n\nExtending this class allows you to define your own loader. Be sure to respect the documented return types and values. You should give it a global class name with `class_name` for it to be registered. Like built-in ResourceFormatLoaders, it will be called automatically when loading resources of its handled type(s). You may also implement a [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver].\n\n**Note:** You can also extend [`EditorImportPlugin`][crate::classes::EditorImportPlugin] if the resource type you need exists but Godot is unable to load its format. Choosing one way over another depends on if the format is suitable or not for the final exported game. For example, it's better to import `.png` textures as `.ctex` ([`CompressedTexture2D`][crate::classes::CompressedTexture2D]) first, so they can be loaded with better efficiency on the graphics card."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ResourceFormatLoader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ResourceFormatLoader` methods](https://docs.godotengine.org/en/stable/classes/class_resourceformatloader.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IResourceFormatLoader: crate::obj::GodotClass < Base = ResourceFormatLoader > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Gets the list of extensions for files this loader is able to read."]
        fn get_recognized_extensions(&self,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Tells whether or not this loader should load a resource from its resource path for a given type.\n\nIf it is not implemented, the default behavior returns whether the path's extension is within the ones provided by [`get_recognized_extensions`][`crate::classes::IResourceFormatLoader::get_recognized_extensions`], and if the type is within the ones provided by [`get_resource_type`][`crate::classes::IResourceFormatLoader::get_resource_type`]."]
        fn recognize_path(&self, path: GString, type_: StringName,) -> bool {
            unimplemented !()
        }
        #[doc = "Tells which resource class this loader can load.\n\n**Note:** Custom resource types defined by scripts aren't known by the [`ClassDB`][crate::classes::ClassDb], so you might just handle `\"Resource\"` for them."]
        fn handles_type(&self, type_: StringName,) -> bool {
            unimplemented !()
        }
        #[doc = "Gets the class name of the resource associated with the given path. If the loader cannot handle it, it should return `\"\"`.\n\n**Note:** Custom resource types defined by scripts aren't known by the [`ClassDB`][crate::classes::ClassDb], so you might just return `\"Resource\"` for them."]
        fn get_resource_type(&self, path: GString,) -> GString {
            unimplemented !()
        }
        #[doc = "Returns the script class name associated with the [`Resource`][crate::classes::Resource] under the given `path`. If the resource has no script or the script isn't a named class, it should return `\"\"`."]
        fn get_resource_script_class(&self, path: GString,) -> GString {
            unimplemented !()
        }
        #[doc = "Should return the unique ID for the resource associated with the given path. If this method is not overridden, a `.uid` file is generated along with the resource file, containing the unique ID."]
        fn get_resource_uid(&self, path: GString,) -> i64 {
            unimplemented !()
        }
        #[doc = "Should return the dependencies for the resource at the given `path`. Each dependency is a string composed of one to three sections separated by `::`, with trailing empty sections omitted:\n\n- The first section should contain the UID if the resource has one. Otherwise, it should contain the file path.\n\n- The second section should contain the class name of the dependency if `add_types` is `true`. Otherwise, it should be empty.\n\n- The third section should contain the fallback path if the resource has a UID. Otherwise, it should be empty.\n\n```gdscript\nfunc _get_dependencies(path, add_types):\n\treturn [\n\t\t\"uid://fqgvuwrkuixh::Script::res://script.gd\",\n\t\t\"uid://fqgvuwrkuixh::::res://script.gd\",\n\t\t\"res://script.gd::Script\",\n\t\t\"res://script.gd\",\n\t]\n```\n\n**Note:** Custom resource types defined by scripts aren't known by the [`ClassDB`][crate::classes::ClassDb], so `\"Resource\"` can be used for the class name."]
        fn get_dependencies(&self, path: GString, add_types: bool,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "If implemented, renames dependencies within the given resource and saves it. `renames` is a dictionary `{ String => String }` mapping old dependency paths to new paths.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success, or an \\[enum Error] constant in case of failure."]
        fn rename_dependencies(&self, path: GString, renames: VarDictionary,) -> crate::global::Error {
            unimplemented !()
        }
        fn exists(&self, path: GString,) -> bool {
            unimplemented !()
        }
        fn get_classes_used(&self, path: GString,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Loads a resource when the engine finds this loader to be compatible. If the loaded resource is the result of an import, `original_path` will target the source file. Returns a [`Resource`][crate::classes::Resource] object on success, or an \\[enum Error] constant in case of failure.\n\nThe `cache_mode` property defines whether and how the cache should be used or updated when loading the resource. See \\[enum CacheMode] for details."]
        fn load(&self, path: GString, original_path: GString, use_sub_threads: bool, cache_mode: i32,) -> Variant;
        
    }
    impl ResourceFormatLoader {
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
    impl crate::obj::GodotClass for ResourceFormatLoader {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ResourceFormatLoader"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ResourceFormatLoader {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ResourceFormatLoader {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ResourceFormatLoader {
        
    }
    impl crate::obj::cap::GodotDefault for ResourceFormatLoader {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ResourceFormatLoader {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ResourceFormatLoader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ResourceFormatLoader`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ResourceFormatLoader__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ResourceFormatLoader > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CacheMode {
    ord: i32
}
impl CacheMode {
    #[doc(alias = "CACHE_MODE_IGNORE")]
    #[doc = "Godot enumerator name: `CACHE_MODE_IGNORE`"]
    pub const IGNORE: CacheMode = CacheMode {
        ord: 0i32
    };
    #[doc(alias = "CACHE_MODE_REUSE")]
    #[doc = "Godot enumerator name: `CACHE_MODE_REUSE`"]
    pub const REUSE: CacheMode = CacheMode {
        ord: 1i32
    };
    #[doc(alias = "CACHE_MODE_REPLACE")]
    #[doc = "Godot enumerator name: `CACHE_MODE_REPLACE`"]
    pub const REPLACE: CacheMode = CacheMode {
        ord: 2i32
    };
    #[doc(alias = "CACHE_MODE_IGNORE_DEEP")]
    #[doc = "Godot enumerator name: `CACHE_MODE_IGNORE_DEEP`"]
    pub const IGNORE_DEEP: CacheMode = CacheMode {
        ord: 3i32
    };
    #[doc(alias = "CACHE_MODE_REPLACE_DEEP")]
    #[doc = "Godot enumerator name: `CACHE_MODE_REPLACE_DEEP`"]
    pub const REPLACE_DEEP: CacheMode = CacheMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for CacheMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CacheMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CacheMode {
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
            Self::IGNORE => "IGNORE", Self::REUSE => "REUSE", Self::REPLACE => "REPLACE", Self::IGNORE_DEEP => "IGNORE_DEEP", Self::REPLACE_DEEP => "REPLACE_DEEP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CacheMode::IGNORE, CacheMode::REUSE, CacheMode::REPLACE, CacheMode::IGNORE_DEEP, CacheMode::REPLACE_DEEP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CacheMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IGNORE", "CACHE_MODE_IGNORE", CacheMode::IGNORE), crate::meta::inspect::EnumConstant::new("REUSE", "CACHE_MODE_REUSE", CacheMode::REUSE), crate::meta::inspect::EnumConstant::new("REPLACE", "CACHE_MODE_REPLACE", CacheMode::REPLACE), crate::meta::inspect::EnumConstant::new("IGNORE_DEEP", "CACHE_MODE_IGNORE_DEEP", CacheMode::IGNORE_DEEP), crate::meta::inspect::EnumConstant::new("REPLACE_DEEP", "CACHE_MODE_REPLACE_DEEP", CacheMode::REPLACE_DEEP)]
        }
    }
}
impl crate::meta::GodotConvert for CacheMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Cache Mode Ignore", 0i64), EnumeratorShape::new_int("Cache Mode Reuse", 1i64), EnumeratorShape::new_int("Cache Mode Replace", 2i64), EnumeratorShape::new_int("Cache Mode Ignore Deep", 3i64), EnumeratorShape::new_int("Cache Mode Replace Deep", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("ResourceFormatLoader.CacheMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CacheMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CacheMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CacheMode {
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
impl crate::registry::property::Export for CacheMode {
    
}
impl crate::meta::Element for CacheMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ResourceFormatLoader;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ResourceFormatLoader {
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
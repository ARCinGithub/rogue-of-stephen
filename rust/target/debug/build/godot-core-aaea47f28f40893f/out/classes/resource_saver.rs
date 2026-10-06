#![doc = "Sidecar module for class [`ResourceSaver`][crate::classes::ResourceSaver].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ResourceSaver` enums](https://docs.godotengine.org/en/stable/classes/class_resourcesaver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ResourceSaver`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`resource_saver`][crate::classes::resource_saver]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `ResourceSaver`](https://docs.godotengine.org/en/stable/classes/class_resourcesaver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nA singleton for saving resource types to the filesystem.\n\nIt uses the many [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver] classes registered in the engine (either built-in or from a plugin) to save resource data to text-based (e.g. `.tres` or `.tscn`) or binary files (e.g. `.res` or `.scn`)."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ResourceSaver {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ResourceSaver {
        #[doc = "Saves a resource to disk to the given path, using a [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver] that recognizes the resource object. If `path` is empty, `ResourceSaver` will try to use \\[member Resource.resource_path].\n\nThe `flags` bitmask can be specified to customize the save behavior.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success.\n\n**Note:** When the project is running, any generated UID associated with the resource will not be saved as the required code is only executed in editor mode."]
        pub(crate) fn save_full(&mut self, resource: CowArg < Gd < crate::classes::Resource > >, path: CowArg < GString >, flags: crate::classes::resource_saver::SaverFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Gd < crate::classes::Resource > >, CowArg < 'a1, GString >, crate::classes::resource_saver::SaverFlags,);
            let args = (resource, path, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceSaver", "save", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`save_ex`][Self::save_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Saves a resource to disk to the given path, using a [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver] that recognizes the resource object. If `path` is empty, `ResourceSaver` will try to use \\[member Resource.resource_path].\n\nThe `flags` bitmask can be specified to customize the save behavior.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success.\n\n**Note:** When the project is running, any generated UID associated with the resource will not be saved as the required code is only executed in editor mode."]
        #[inline]
        pub fn save(&mut self, resource: impl AsArg < Gd < crate::classes::Resource >>,) -> crate::global::Error {
            self.save_ex(resource,) . done()
        }
        #[doc = "Saves a resource to disk to the given path, using a [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver] that recognizes the resource object. If `path` is empty, `ResourceSaver` will try to use \\[member Resource.resource_path].\n\nThe `flags` bitmask can be specified to customize the save behavior.\n\nReturns [`Error::OK`][`crate::global::Error::OK`] on success.\n\n**Note:** When the project is running, any generated UID associated with the resource will not be saved as the required code is only executed in editor mode."]
        #[inline]
        pub fn save_ex < 'ex > (&'ex mut self, resource: impl AsArg < Gd < crate::classes::Resource >> + 'ex,) -> ExSave < 'ex > {
            ExSave::new(self, resource,)
        }
        #[doc = "Sets the UID of the given `resource` path to `uid`. You can generate a new UID using [`create_id`][`crate::classes::ResourceUid::create_id`].\n\nSince resources will normally get a UID automatically, this method is only useful in very specific cases."]
        pub fn set_uid(&mut self, resource: impl AsArg < GString >, uid: i64,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
            let args = (resource.into_arg(), uid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceSaver", "set_uid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of extensions available for saving a resource of a given type."]
        pub fn get_recognized_extensions(&self, type_: impl AsArg < Option < Gd < crate::classes::Resource >> >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >,);
            let args = (type_.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceSaver", "get_recognized_extensions", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers a new [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver]. The ResourceSaver will use the ResourceFormatSaver as described in [`save`][`crate::classes::ResourceSaver::save`].\n\nThis method is performed implicitly for ResourceFormatSavers written in GDScript (see [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver] for more information)."]
        pub(crate) fn add_resource_format_saver_full(&mut self, format_saver: CowArg < Option < Gd < crate::classes::ResourceFormatSaver > > >, at_front: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ResourceFormatSaver > > >, bool,);
            let args = (format_saver, at_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceSaver", "add_resource_format_saver", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_resource_format_saver_ex`][Self::add_resource_format_saver_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Registers a new [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver]. The ResourceSaver will use the ResourceFormatSaver as described in [`save`][`crate::classes::ResourceSaver::save`].\n\nThis method is performed implicitly for ResourceFormatSavers written in GDScript (see [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver] for more information)."]
        #[inline]
        pub fn add_resource_format_saver(&mut self, format_saver: impl AsArg < Option < Gd < crate::classes::ResourceFormatSaver >> >,) {
            self.add_resource_format_saver_ex(format_saver,) . done()
        }
        #[doc = "Registers a new [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver]. The ResourceSaver will use the ResourceFormatSaver as described in [`save`][`crate::classes::ResourceSaver::save`].\n\nThis method is performed implicitly for ResourceFormatSavers written in GDScript (see [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver] for more information)."]
        #[inline]
        pub fn add_resource_format_saver_ex < 'ex > (&'ex mut self, format_saver: impl AsArg < Option < Gd < crate::classes::ResourceFormatSaver >> > + 'ex,) -> ExAddResourceFormatSaver < 'ex > {
            ExAddResourceFormatSaver::new(self, format_saver,)
        }
        #[doc = "Unregisters the given [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver]."]
        pub fn remove_resource_format_saver(&mut self, format_saver: impl AsArg < Option < Gd < crate::classes::ResourceFormatSaver >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ResourceFormatSaver > > >,);
            let args = (format_saver.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceSaver", "remove_resource_format_saver", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the resource ID for the given path. If `generate` is `true`, a new resource ID will be generated if one for the path is not found. If `generate` is `false` and the path is not found, `ResourceUID.INVALID_ID` is returned."]
        pub(crate) fn get_resource_id_for_path_full(&self, path: CowArg < GString >, generate: bool,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (path, generate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceSaver", "get_resource_id_for_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_resource_id_for_path_ex`][Self::get_resource_id_for_path_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the resource ID for the given path. If `generate` is `true`, a new resource ID will be generated if one for the path is not found. If `generate` is `false` and the path is not found, `ResourceUID.INVALID_ID` is returned."]
        #[inline]
        pub fn get_resource_id_for_path(&self, path: impl AsArg < GString >,) -> i64 {
            self.get_resource_id_for_path_ex(path,) . done()
        }
        #[doc = "Returns the resource ID for the given path. If `generate` is `true`, a new resource ID will be generated if one for the path is not found. If `generate` is `false` and the path is not found, `ResourceUID.INVALID_ID` is returned."]
        #[inline]
        pub fn get_resource_id_for_path_ex < 'ex > (&'ex self, path: impl AsArg < GString > + 'ex,) -> ExGetResourceIdForPath < 'ex > {
            ExGetResourceIdForPath::new(self, path,)
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
    impl crate::obj::GodotClass for ResourceSaver {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ResourceSaver"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ResourceSaver {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ResourceSaver {
        
    }
    impl crate::obj::Singleton for ResourceSaver {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"ResourceSaver"))
            }
        }
    }
    impl std::ops::Deref for ResourceSaver {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ResourceSaver {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ResourceSaver__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ResourceSaver` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`ResourceSaver::save_ex`][super::ResourceSaver::save_ex]."]
#[must_use]
pub struct ExSave < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ResourceSaver, resource: CowArg < 'ex, Gd < crate::classes::Resource > >, path: CowArg < 'ex, GString >, flags: crate::classes::resource_saver::SaverFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSave < 'ex > {
    fn new(surround_object: &'ex mut re_export::ResourceSaver, resource: impl AsArg < Gd < crate::classes::Resource >> + 'ex,) -> Self {
        let path = GString::from("");
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, resource: resource.into_arg(), path: CowArg::Owned(path), flags: flags,
        }
    }
    #[inline]
    pub fn path(self, path: impl AsArg < GString > + 'ex) -> Self {
        Self {
            path: path.into_arg(), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::resource_saver::SaverFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, resource, path, flags,
        }
        = self;
        re_export::ResourceSaver::save_full(surround_object, resource, path, flags,)
    }
}
#[doc = "Default-param extender for [`ResourceSaver::add_resource_format_saver_ex`][super::ResourceSaver::add_resource_format_saver_ex]."]
#[must_use]
pub struct ExAddResourceFormatSaver < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ResourceSaver, format_saver: CowArg < 'ex, Option < Gd < crate::classes::ResourceFormatSaver > > >, at_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddResourceFormatSaver < 'ex > {
    fn new(surround_object: &'ex mut re_export::ResourceSaver, format_saver: impl AsArg < Option < Gd < crate::classes::ResourceFormatSaver >> > + 'ex,) -> Self {
        let at_front = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, format_saver: format_saver.into_arg(), at_front: at_front,
        }
    }
    #[inline]
    pub fn at_front(self, at_front: bool) -> Self {
        Self {
            at_front: at_front, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, format_saver, at_front,
        }
        = self;
        re_export::ResourceSaver::add_resource_format_saver_full(surround_object, format_saver, at_front,)
    }
}
#[doc = "Default-param extender for [`ResourceSaver::get_resource_id_for_path_ex`][super::ResourceSaver::get_resource_id_for_path_ex]."]
#[must_use]
pub struct ExGetResourceIdForPath < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::ResourceSaver, path: CowArg < 'ex, GString >, generate: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetResourceIdForPath < 'ex > {
    fn new(surround_object: &'ex re_export::ResourceSaver, path: impl AsArg < GString > + 'ex,) -> Self {
        let generate = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), generate: generate,
        }
    }
    #[inline]
    pub fn generate(self, generate: bool) -> Self {
        Self {
            generate: generate, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, path, generate,
        }
        = self;
        re_export::ResourceSaver::get_resource_id_for_path_full(surround_object, path, generate,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct SaverFlags {
    ord: u64
}
impl SaverFlags {
    #[doc(alias = "FLAG_NONE")]
    #[doc = "Godot enumerator name: `FLAG_NONE`"]
    pub const NONE: SaverFlags = SaverFlags {
        ord: 0u64
    };
    #[doc(alias = "FLAG_RELATIVE_PATHS")]
    #[doc = "Godot enumerator name: `FLAG_RELATIVE_PATHS`"]
    pub const RELATIVE_PATHS: SaverFlags = SaverFlags {
        ord: 1u64
    };
    #[doc(alias = "FLAG_BUNDLE_RESOURCES")]
    #[doc = "Godot enumerator name: `FLAG_BUNDLE_RESOURCES`"]
    pub const BUNDLE_RESOURCES: SaverFlags = SaverFlags {
        ord: 2u64
    };
    #[doc(alias = "FLAG_CHANGE_PATH")]
    #[doc = "Godot enumerator name: `FLAG_CHANGE_PATH`"]
    pub const CHANGE_PATH: SaverFlags = SaverFlags {
        ord: 4u64
    };
    #[doc(alias = "FLAG_OMIT_EDITOR_PROPERTIES")]
    #[doc = "Godot enumerator name: `FLAG_OMIT_EDITOR_PROPERTIES`"]
    pub const OMIT_EDITOR_PROPERTIES: SaverFlags = SaverFlags {
        ord: 8u64
    };
    #[doc(alias = "FLAG_SAVE_BIG_ENDIAN")]
    #[doc = "Godot enumerator name: `FLAG_SAVE_BIG_ENDIAN`"]
    pub const SAVE_BIG_ENDIAN: SaverFlags = SaverFlags {
        ord: 16u64
    };
    #[doc(alias = "FLAG_COMPRESS")]
    #[doc = "Godot enumerator name: `FLAG_COMPRESS`"]
    pub const COMPRESS: SaverFlags = SaverFlags {
        ord: 32u64
    };
    #[doc(alias = "FLAG_REPLACE_SUBRESOURCE_PATHS")]
    #[doc = "Godot enumerator name: `FLAG_REPLACE_SUBRESOURCE_PATHS`"]
    pub const REPLACE_SUBRESOURCE_PATHS: SaverFlags = SaverFlags {
        ord: 64u64
    };
    
}
impl std::fmt::Debug for SaverFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for SaverFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SaverFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "FLAG_NONE", SaverFlags::NONE), crate::meta::inspect::EnumConstant::new("RELATIVE_PATHS", "FLAG_RELATIVE_PATHS", SaverFlags::RELATIVE_PATHS), crate::meta::inspect::EnumConstant::new("BUNDLE_RESOURCES", "FLAG_BUNDLE_RESOURCES", SaverFlags::BUNDLE_RESOURCES), crate::meta::inspect::EnumConstant::new("CHANGE_PATH", "FLAG_CHANGE_PATH", SaverFlags::CHANGE_PATH), crate::meta::inspect::EnumConstant::new("OMIT_EDITOR_PROPERTIES", "FLAG_OMIT_EDITOR_PROPERTIES", SaverFlags::OMIT_EDITOR_PROPERTIES), crate::meta::inspect::EnumConstant::new("SAVE_BIG_ENDIAN", "FLAG_SAVE_BIG_ENDIAN", SaverFlags::SAVE_BIG_ENDIAN), crate::meta::inspect::EnumConstant::new("COMPRESS", "FLAG_COMPRESS", SaverFlags::COMPRESS), crate::meta::inspect::EnumConstant::new("REPLACE_SUBRESOURCE_PATHS", "FLAG_REPLACE_SUBRESOURCE_PATHS", SaverFlags::REPLACE_SUBRESOURCE_PATHS)]
        }
    }
}
impl std::ops::BitOr for SaverFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for SaverFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for SaverFlags {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Flag None", 0i64), EnumeratorShape::new_int("Flag Relative Paths", 1i64), EnumeratorShape::new_int("Flag Bundle Resources", 2i64), EnumeratorShape::new_int("Flag Change Path", 4i64), EnumeratorShape::new_int("Flag Omit Editor Properties", 8i64), EnumeratorShape::new_int("Flag Save Big Endian", 16i64), EnumeratorShape::new_int("Flag Compress", 32i64), EnumeratorShape::new_int("Flag Replace Subresource Paths", 64i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("ResourceSaver.SaverFlags")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for SaverFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SaverFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SaverFlags {
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
impl crate::registry::property::Export for SaverFlags {
    
}
impl crate::meta::Element for SaverFlags {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ResourceSaver;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ResourceSaver {
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
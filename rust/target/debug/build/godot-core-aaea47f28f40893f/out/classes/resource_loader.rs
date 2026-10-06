#![doc = "Sidecar module for class [`ResourceLoader`][crate::classes::ResourceLoader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ResourceLoader` enums](https://docs.godotengine.org/en/stable/classes/class_resourceloader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ResourceLoader`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`resource_loader`][crate::classes::resource_loader]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `ResourceLoader`](https://docs.godotengine.org/en/stable/classes/class_resourceloader.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nA singleton used to load resource files from the filesystem.\n\nIt uses the many [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader] classes registered in the engine (either built-in or from a plugin) to load files into memory and convert them to a format that can be used by the engine.\n\n**Note:** You have to import the files into the engine first to load them using [`load`][`crate::classes::ResourceLoader::load`]. If you want to load [`Image`][crate::classes::Image]s at run-time, you may use [`load`][`crate::classes::Image::load`]. If you want to import audio files, you can use the snippet described in \\[member AudioStreamMP3.data].\n\n**Note:** Non-resource files such as plain text files cannot be read using `ResourceLoader`. Use [`FileAccess`][crate::classes::FileAccess] for those files instead, and be aware that non-resource files are not exported by default (see notes in the [`FileAccess`][crate::classes::FileAccess] class description for instructions on exporting them)."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ResourceLoader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ResourceLoader {
        #[doc = "Loads a resource at the given `path`, caching the result for further access.\n\nThe registered [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]s are queried sequentially to find the first one which can handle the file's extension, and then attempt loading. If loading fails, the remaining ResourceFormatLoaders are also attempted.\n\nAn optional `type_hint` can be used to further specify the [`Resource`][crate::classes::Resource] type that should be handled by the [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]. Anything that inherits from [`Resource`][crate::classes::Resource] can be used as a type hint, for example [`Image`][crate::classes::Image].\n\nThe `cache_mode` property defines whether and how the cache should be used or updated when loading the resource.\n\nReturns an empty resource if no [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader] could handle the file, and prints an error if no file is found at the specified path.\n\nGDScript has a simplified [`load`][`crate::tools::load`] built-in method which can be used in most situations, leaving the use of `ResourceLoader` for more advanced scenarios.\n\n**Note:** If \\[member ProjectSettings.editor/export/convert_text_resources_to_binary] is `true`, [`load`][`crate::tools::load`] will not be able to read converted files in an exported project. If you rely on run-time loading of files present within the PCK, set \\[member ProjectSettings.editor/export/convert_text_resources_to_binary] to `false`.\n\n**Note:** Relative paths will be prefixed with `\"res://\"` before loading, to avoid unexpected results make sure your paths are absolute."]
        pub(crate) fn load_full(&mut self, path: CowArg < GString >, type_hint: CowArg < GString >, cache_mode: crate::classes::resource_loader::CacheMode,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, crate::classes::resource_loader::CacheMode,);
            let args = (path, type_hint, cache_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "load", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`load_ex`][Self::load_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Loads a resource at the given `path`, caching the result for further access.\n\nThe registered [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]s are queried sequentially to find the first one which can handle the file's extension, and then attempt loading. If loading fails, the remaining ResourceFormatLoaders are also attempted.\n\nAn optional `type_hint` can be used to further specify the [`Resource`][crate::classes::Resource] type that should be handled by the [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]. Anything that inherits from [`Resource`][crate::classes::Resource] can be used as a type hint, for example [`Image`][crate::classes::Image].\n\nThe `cache_mode` property defines whether and how the cache should be used or updated when loading the resource.\n\nReturns an empty resource if no [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader] could handle the file, and prints an error if no file is found at the specified path.\n\nGDScript has a simplified [`load`][`crate::tools::load`] built-in method which can be used in most situations, leaving the use of `ResourceLoader` for more advanced scenarios.\n\n**Note:** If \\[member ProjectSettings.editor/export/convert_text_resources_to_binary] is `true`, [`load`][`crate::tools::load`] will not be able to read converted files in an exported project. If you rely on run-time loading of files present within the PCK, set \\[member ProjectSettings.editor/export/convert_text_resources_to_binary] to `false`.\n\n**Note:** Relative paths will be prefixed with `\"res://\"` before loading, to avoid unexpected results make sure your paths are absolute."]
        #[inline]
        pub fn load(&mut self, path: impl AsArg < GString >,) -> Option < Gd < crate::classes::Resource > > {
            self.load_ex(path,) . done()
        }
        #[doc = "Loads a resource at the given `path`, caching the result for further access.\n\nThe registered [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]s are queried sequentially to find the first one which can handle the file's extension, and then attempt loading. If loading fails, the remaining ResourceFormatLoaders are also attempted.\n\nAn optional `type_hint` can be used to further specify the [`Resource`][crate::classes::Resource] type that should be handled by the [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]. Anything that inherits from [`Resource`][crate::classes::Resource] can be used as a type hint, for example [`Image`][crate::classes::Image].\n\nThe `cache_mode` property defines whether and how the cache should be used or updated when loading the resource.\n\nReturns an empty resource if no [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader] could handle the file, and prints an error if no file is found at the specified path.\n\nGDScript has a simplified [`load`][`crate::tools::load`] built-in method which can be used in most situations, leaving the use of `ResourceLoader` for more advanced scenarios.\n\n**Note:** If \\[member ProjectSettings.editor/export/convert_text_resources_to_binary] is `true`, [`load`][`crate::tools::load`] will not be able to read converted files in an exported project. If you rely on run-time loading of files present within the PCK, set \\[member ProjectSettings.editor/export/convert_text_resources_to_binary] to `false`.\n\n**Note:** Relative paths will be prefixed with `\"res://\"` before loading, to avoid unexpected results make sure your paths are absolute."]
        #[inline]
        pub fn load_ex < 'ex > (&'ex mut self, path: impl AsArg < GString > + 'ex,) -> ExLoad < 'ex > {
            ExLoad::new(self, path,)
        }
        #[doc = "Returns the list of recognized extensions for a resource type."]
        pub fn get_recognized_extensions_for_type(&self, type_: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (type_.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "get_recognized_extensions_for_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers a new [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]. The ResourceLoader will use the ResourceFormatLoader as described in [`load`][`crate::classes::ResourceLoader::load`].\n\nThis method is performed implicitly for ResourceFormatLoaders written in GDScript (see [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader] for more information)."]
        pub(crate) fn add_resource_format_loader_full(&mut self, format_loader: CowArg < Option < Gd < crate::classes::ResourceFormatLoader > > >, at_front: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ResourceFormatLoader > > >, bool,);
            let args = (format_loader, at_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "add_resource_format_loader", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_resource_format_loader_ex`][Self::add_resource_format_loader_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Registers a new [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]. The ResourceLoader will use the ResourceFormatLoader as described in [`load`][`crate::classes::ResourceLoader::load`].\n\nThis method is performed implicitly for ResourceFormatLoaders written in GDScript (see [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader] for more information)."]
        #[inline]
        pub fn add_resource_format_loader(&mut self, format_loader: impl AsArg < Option < Gd < crate::classes::ResourceFormatLoader >> >,) {
            self.add_resource_format_loader_ex(format_loader,) . done()
        }
        #[doc = "Registers a new [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]. The ResourceLoader will use the ResourceFormatLoader as described in [`load`][`crate::classes::ResourceLoader::load`].\n\nThis method is performed implicitly for ResourceFormatLoaders written in GDScript (see [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader] for more information)."]
        #[inline]
        pub fn add_resource_format_loader_ex < 'ex > (&'ex mut self, format_loader: impl AsArg < Option < Gd < crate::classes::ResourceFormatLoader >> > + 'ex,) -> ExAddResourceFormatLoader < 'ex > {
            ExAddResourceFormatLoader::new(self, format_loader,)
        }
        #[doc = "Unregisters the given [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]."]
        pub fn remove_resource_format_loader(&mut self, format_loader: impl AsArg < Option < Gd < crate::classes::ResourceFormatLoader >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ResourceFormatLoader > > >,);
            let args = (format_loader.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "remove_resource_format_loader", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Changes the behavior on missing sub-resources. The default behavior is to abort loading."]
        pub fn set_abort_on_missing_resources(&mut self, abort: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (abort,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "set_abort_on_missing_resources", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the dependencies for the resource at the given `path`.\n\nEach dependency is a string that can be divided into sections by `::`. There can be either one section or three sections, with the second section always being empty. When there is one section, it contains the file path. When there are three sections, the first section contains the UID and the third section contains the fallback path.\n\n```gdscript\nfor dependency in ResourceLoader.get_dependencies(path):\n\tif dependency.contains(\"::\"):\n\t\tprint(dependency.get_slice(\"::\", 0)) # Prints the UID.\n\t\tprint(dependency.get_slice(\"::\", 2)) # Prints the fallback path.\n\telse:\n\t\tprint(dependency) # Prints the path.\n```"]
        pub fn get_dependencies(&self, path: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "get_dependencies", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether a cached resource is available for the given `path`.\n\nOnce a resource has been loaded by the engine, it is cached in memory for faster access, and future calls to the [`load`][`crate::classes::ResourceLoader::load`] method will use the cached version. The cached resource can be overridden by using [`take_over_path`][`crate::classes::Resource::take_over_path`] on a new resource for that same path."]
        pub fn has_cached(&self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "has_cached", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the cached resource reference for the given `path`.\n\n**Note:** If the resource is not cached, the returned [`Resource`][crate::classes::Resource] will be invalid."]
        pub fn get_cached_ref(&self, path: impl AsArg < GString >,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "get_cached_ref", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether a recognized resource exists for the given `path`.\n\nAn optional `type_hint` can be used to further specify the [`Resource`][crate::classes::Resource] type that should be handled by the [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]. Anything that inherits from [`Resource`][crate::classes::Resource] can be used as a type hint, for example [`Image`][crate::classes::Image].\n\n**Note:** If you use [`take_over_path`][`crate::classes::Resource::take_over_path`], this method will return `true` for the taken path even if the resource wasn't saved (i.e. exists only in resource cache)."]
        pub(crate) fn exists_full(&mut self, path: CowArg < GString >, type_hint: CowArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (path, type_hint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "exists", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`exists_ex`][Self::exists_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns whether a recognized resource exists for the given `path`.\n\nAn optional `type_hint` can be used to further specify the [`Resource`][crate::classes::Resource] type that should be handled by the [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]. Anything that inherits from [`Resource`][crate::classes::Resource] can be used as a type hint, for example [`Image`][crate::classes::Image].\n\n**Note:** If you use [`take_over_path`][`crate::classes::Resource::take_over_path`], this method will return `true` for the taken path even if the resource wasn't saved (i.e. exists only in resource cache)."]
        #[inline]
        pub fn exists(&mut self, path: impl AsArg < GString >,) -> bool {
            self.exists_ex(path,) . done()
        }
        #[doc = "Returns whether a recognized resource exists for the given `path`.\n\nAn optional `type_hint` can be used to further specify the [`Resource`][crate::classes::Resource] type that should be handled by the [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader]. Anything that inherits from [`Resource`][crate::classes::Resource] can be used as a type hint, for example [`Image`][crate::classes::Image].\n\n**Note:** If you use [`take_over_path`][`crate::classes::Resource::take_over_path`], this method will return `true` for the taken path even if the resource wasn't saved (i.e. exists only in resource cache)."]
        #[inline]
        pub fn exists_ex < 'ex > (&'ex mut self, path: impl AsArg < GString > + 'ex,) -> ExExists < 'ex > {
            ExExists::new(self, path,)
        }
        #[doc = "Returns the ID associated with a given resource path, or `-1` when no such ID exists."]
        pub fn get_resource_uid(&self, path: impl AsArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "get_resource_uid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Lists a directory, returning all resources and subdirectories contained within. The resource files have the original file names as visible in the editor before exporting. The directories have `\"/\"` appended.\n\n```gdscript\n# Prints [\"extra_data/\", \"model.gltf\", \"model.tscn\", \"model_slime.png\"]\nprint(ResourceLoader.list_directory(\"res://assets/enemies/slime\"))\n```\n\n**Note:** The order of files and directories returned by this method is not deterministic, and can vary between operating systems.\n\n**Note:** To normally traverse the filesystem, see [`DirAccess`][crate::classes::DirAccess]."]
        pub fn list_directory(&mut self, directory_path: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (directory_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "list_directory", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ResourceLoader {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ResourceLoader"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ResourceLoader {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ResourceLoader {
        
    }
    impl crate::obj::Singleton for ResourceLoader {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"ResourceLoader"))
            }
        }
    }
    impl std::ops::Deref for ResourceLoader {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ResourceLoader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ResourceLoader__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ResourceLoader` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`ResourceLoader::load_ex`][super::ResourceLoader::load_ex]."]
#[must_use]
pub struct ExLoad < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ResourceLoader, path: CowArg < 'ex, GString >, type_hint: CowArg < 'ex, GString >, cache_mode: crate::classes::resource_loader::CacheMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExLoad < 'ex > {
    fn new(surround_object: &'ex mut re_export::ResourceLoader, path: impl AsArg < GString > + 'ex,) -> Self {
        let type_hint = GString::from("");
        let cache_mode = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), type_hint: CowArg::Owned(type_hint), cache_mode: cache_mode,
        }
    }
    #[inline]
    pub fn type_hint(self, type_hint: impl AsArg < GString > + 'ex) -> Self {
        Self {
            type_hint: type_hint.into_arg(), .. self
        }
    }
    #[inline]
    pub fn cache_mode(self, cache_mode: crate::classes::resource_loader::CacheMode) -> Self {
        Self {
            cache_mode: cache_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Resource > > {
        let Self {
            _phantom, surround_object, path, type_hint, cache_mode,
        }
        = self;
        re_export::ResourceLoader::load_full(surround_object, path, type_hint, cache_mode,)
    }
}
#[doc = "Default-param extender for [`ResourceLoader::add_resource_format_loader_ex`][super::ResourceLoader::add_resource_format_loader_ex]."]
#[must_use]
pub struct ExAddResourceFormatLoader < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ResourceLoader, format_loader: CowArg < 'ex, Option < Gd < crate::classes::ResourceFormatLoader > > >, at_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddResourceFormatLoader < 'ex > {
    fn new(surround_object: &'ex mut re_export::ResourceLoader, format_loader: impl AsArg < Option < Gd < crate::classes::ResourceFormatLoader >> > + 'ex,) -> Self {
        let at_front = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, format_loader: format_loader.into_arg(), at_front: at_front,
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
            _phantom, surround_object, format_loader, at_front,
        }
        = self;
        re_export::ResourceLoader::add_resource_format_loader_full(surround_object, format_loader, at_front,)
    }
}
#[doc = "Default-param extender for [`ResourceLoader::exists_ex`][super::ResourceLoader::exists_ex]."]
#[must_use]
pub struct ExExists < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ResourceLoader, path: CowArg < 'ex, GString >, type_hint: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExExists < 'ex > {
    fn new(surround_object: &'ex mut re_export::ResourceLoader, path: impl AsArg < GString > + 'ex,) -> Self {
        let type_hint = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), type_hint: CowArg::Owned(type_hint),
        }
    }
    #[inline]
    pub fn type_hint(self, type_hint: impl AsArg < GString > + 'ex) -> Self {
        Self {
            type_hint: type_hint.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, path, type_hint,
        }
        = self;
        re_export::ResourceLoader::exists_full(surround_object, path, type_hint,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ThreadLoadStatus {
    ord: i32
}
impl ThreadLoadStatus {
    #[doc(alias = "THREAD_LOAD_INVALID_RESOURCE")]
    #[doc = "Godot enumerator name: `THREAD_LOAD_INVALID_RESOURCE`"]
    pub const INVALID_RESOURCE: ThreadLoadStatus = ThreadLoadStatus {
        ord: 0i32
    };
    #[doc(alias = "THREAD_LOAD_IN_PROGRESS")]
    #[doc = "Godot enumerator name: `THREAD_LOAD_IN_PROGRESS`"]
    pub const IN_PROGRESS: ThreadLoadStatus = ThreadLoadStatus {
        ord: 1i32
    };
    #[doc(alias = "THREAD_LOAD_FAILED")]
    #[doc = "Godot enumerator name: `THREAD_LOAD_FAILED`"]
    pub const FAILED: ThreadLoadStatus = ThreadLoadStatus {
        ord: 2i32
    };
    #[doc(alias = "THREAD_LOAD_LOADED")]
    #[doc = "Godot enumerator name: `THREAD_LOAD_LOADED`"]
    pub const LOADED: ThreadLoadStatus = ThreadLoadStatus {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ThreadLoadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ThreadLoadStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ThreadLoadStatus {
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
            Self::INVALID_RESOURCE => "INVALID_RESOURCE", Self::IN_PROGRESS => "IN_PROGRESS", Self::FAILED => "FAILED", Self::LOADED => "LOADED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ThreadLoadStatus::INVALID_RESOURCE, ThreadLoadStatus::IN_PROGRESS, ThreadLoadStatus::FAILED, ThreadLoadStatus::LOADED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ThreadLoadStatus >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INVALID_RESOURCE", "THREAD_LOAD_INVALID_RESOURCE", ThreadLoadStatus::INVALID_RESOURCE), crate::meta::inspect::EnumConstant::new("IN_PROGRESS", "THREAD_LOAD_IN_PROGRESS", ThreadLoadStatus::IN_PROGRESS), crate::meta::inspect::EnumConstant::new("FAILED", "THREAD_LOAD_FAILED", ThreadLoadStatus::FAILED), crate::meta::inspect::EnumConstant::new("LOADED", "THREAD_LOAD_LOADED", ThreadLoadStatus::LOADED)]
        }
    }
}
impl crate::meta::GodotConvert for ThreadLoadStatus {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Thread Load Invalid Resource", 0i64), EnumeratorShape::new_int("Thread Load In Progress", 1i64), EnumeratorShape::new_int("Thread Load Failed", 2i64), EnumeratorShape::new_int("Thread Load Loaded", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("ResourceLoader.ThreadLoadStatus")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ThreadLoadStatus {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ThreadLoadStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ThreadLoadStatus {
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
impl crate::registry::property::Export for ThreadLoadStatus {
    
}
impl crate::meta::Element for ThreadLoadStatus {
    
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("ResourceLoader.CacheMode")), is_bitfield: false,
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
    use super::re_export::ResourceLoader;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ResourceLoader {
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
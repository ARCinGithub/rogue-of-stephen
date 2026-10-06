#![doc = "Sidecar module for class [`GDExtensionManager`][crate::classes::GDExtensionManager].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GDExtensionManager` enums](https://docs.godotengine.org/en/stable/classes/class_gdextensionmanager.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GDExtensionManager`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`gdextension_manager`][crate::classes::gdextension_manager]: sidecar module with related enum/flag types\n* [`SignalsOfGDExtensionManager`][crate::classes::gdextension_manager::SignalsOfGDExtensionManager]: signal collection\n\n\nSee also [Godot docs for `GDExtensionManager`](https://docs.godotengine.org/en/stable/classes/class_gdextensionmanager.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThe GDExtensionManager loads, initializes, and keeps track of all available [`GDExtension`][crate::classes::GDExtension] libraries in the project.\n\n**Note:** Do not worry about GDExtension unless you know what you are doing."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GDExtensionManager {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl GDExtensionManager {
        #[doc = "Loads an extension by absolute file path. The `path` needs to point to a valid [`GDExtension`][crate::classes::GDExtension]. Returns [`LoadStatus::OK`][`crate::classes::gdextension_manager::LoadStatus::OK`] if successful."]
        pub fn load_extension(&mut self, path: impl AsArg < GString >,) -> crate::classes::gdextension_manager::LoadStatus {
            type CallRet = crate::classes::gdextension_manager::LoadStatus;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GDExtensionManager", "load_extension", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "\n# Godot docs\nLoads the extension already in address space via the given path and initialization function. The `path` needs to be unique and start with `\"libgodot://\"`. Returns [`LoadStatus::OK`][`crate::classes::gdextension_manager::LoadStatus::OK`] if successful."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        pub unsafe fn load_extension_from_function(&mut self, path: impl AsArg < GString >, init_func: crate::meta::RawPtr < * const sys::GDExtensionInitializationFunction >,) -> crate::classes::gdextension_manager::LoadStatus {
            type CallRet = crate::classes::gdextension_manager::LoadStatus;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::meta::RawPtr < * const sys::GDExtensionInitializationFunction >,);
            let args = (path.into_arg(), init_func,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GDExtensionManager", "load_extension_from_function", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Reloads the extension at the given file path. The `path` needs to point to a valid [`GDExtension`][crate::classes::GDExtension], otherwise this method may return either [`LoadStatus::NOT_LOADED`][`crate::classes::gdextension_manager::LoadStatus::NOT_LOADED`] or [`LoadStatus::FAILED`][`crate::classes::gdextension_manager::LoadStatus::FAILED`].\n\n**Note:** You can only reload extensions in the editor. In release builds, this method always fails and returns [`LoadStatus::FAILED`][`crate::classes::gdextension_manager::LoadStatus::FAILED`]."]
        pub fn reload_extension(&mut self, path: impl AsArg < GString >,) -> crate::classes::gdextension_manager::LoadStatus {
            type CallRet = crate::classes::gdextension_manager::LoadStatus;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GDExtensionManager", "reload_extension", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unloads an extension by file path. The `path` needs to point to an already loaded [`GDExtension`][crate::classes::GDExtension], otherwise this method returns [`LoadStatus::NOT_LOADED`][`crate::classes::gdextension_manager::LoadStatus::NOT_LOADED`]."]
        pub fn unload_extension(&mut self, path: impl AsArg < GString >,) -> crate::classes::gdextension_manager::LoadStatus {
            type CallRet = crate::classes::gdextension_manager::LoadStatus;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GDExtensionManager", "unload_extension", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the extension at the given file `path` has already been loaded successfully. See also [`get_loaded_extensions`][`crate::classes::GDExtensionManager::get_loaded_extensions`]."]
        pub fn is_extension_loaded(&self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GDExtensionManager", "is_extension_loaded", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the file paths of all currently loaded extensions."]
        pub fn get_loaded_extensions(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GDExtensionManager", "get_loaded_extensions", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`GDExtension`][crate::classes::GDExtension] at the given file `path`, or `null` if it has not been loaded or does not exist."]
        pub fn get_extension(&self, path: impl AsArg < GString >,) -> Option < Gd < crate::classes::GDExtension > > {
            type CallRet = Option < Gd < crate::classes::GDExtension > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GDExtensionManager", "get_extension", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for GDExtensionManager {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GDExtensionManager"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GDExtensionManager {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GDExtensionManager {
        
    }
    impl crate::obj::Singleton for GDExtensionManager {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"GDExtensionManager"))
            }
        }
    }
    impl std::ops::Deref for GDExtensionManager {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GDExtensionManager {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GDExtensionManager__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `GDExtensionManager` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LoadStatus {
    ord: i32
}
impl LoadStatus {
    #[doc(alias = "LOAD_STATUS_OK")]
    #[doc = "Godot enumerator name: `LOAD_STATUS_OK`"]
    pub const OK: LoadStatus = LoadStatus {
        ord: 0i32
    };
    #[doc(alias = "LOAD_STATUS_FAILED")]
    #[doc = "Godot enumerator name: `LOAD_STATUS_FAILED`"]
    pub const FAILED: LoadStatus = LoadStatus {
        ord: 1i32
    };
    #[doc(alias = "LOAD_STATUS_ALREADY_LOADED")]
    #[doc = "Godot enumerator name: `LOAD_STATUS_ALREADY_LOADED`"]
    pub const ALREADY_LOADED: LoadStatus = LoadStatus {
        ord: 2i32
    };
    #[doc(alias = "LOAD_STATUS_NOT_LOADED")]
    #[doc = "Godot enumerator name: `LOAD_STATUS_NOT_LOADED`"]
    pub const NOT_LOADED: LoadStatus = LoadStatus {
        ord: 3i32
    };
    #[doc(alias = "LOAD_STATUS_NEEDS_RESTART")]
    #[doc = "Godot enumerator name: `LOAD_STATUS_NEEDS_RESTART`"]
    pub const NEEDS_RESTART: LoadStatus = LoadStatus {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for LoadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LoadStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LoadStatus {
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
            Self::OK => "OK", Self::FAILED => "FAILED", Self::ALREADY_LOADED => "ALREADY_LOADED", Self::NOT_LOADED => "NOT_LOADED", Self::NEEDS_RESTART => "NEEDS_RESTART", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LoadStatus::OK, LoadStatus::FAILED, LoadStatus::ALREADY_LOADED, LoadStatus::NOT_LOADED, LoadStatus::NEEDS_RESTART]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LoadStatus >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OK", "LOAD_STATUS_OK", LoadStatus::OK), crate::meta::inspect::EnumConstant::new("FAILED", "LOAD_STATUS_FAILED", LoadStatus::FAILED), crate::meta::inspect::EnumConstant::new("ALREADY_LOADED", "LOAD_STATUS_ALREADY_LOADED", LoadStatus::ALREADY_LOADED), crate::meta::inspect::EnumConstant::new("NOT_LOADED", "LOAD_STATUS_NOT_LOADED", LoadStatus::NOT_LOADED), crate::meta::inspect::EnumConstant::new("NEEDS_RESTART", "LOAD_STATUS_NEEDS_RESTART", LoadStatus::NEEDS_RESTART)]
        }
    }
}
impl crate::meta::GodotConvert for LoadStatus {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Load Status Ok", 0i64), EnumeratorShape::new_int("Load Status Failed", 1i64), EnumeratorShape::new_int("Load Status Already Loaded", 2i64), EnumeratorShape::new_int("Load Status Not Loaded", 3i64), EnumeratorShape::new_int("Load Status Needs Restart", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("GDExtensionManager.LoadStatus")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LoadStatus {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LoadStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LoadStatus {
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
impl crate::registry::property::Export for LoadStatus {
    
}
impl crate::meta::Element for LoadStatus {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GDExtensionManager;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`GDExtensionManager`][crate::classes::GDExtensionManager] class."]
    pub struct SignalsOfGDExtensionManager < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfGDExtensionManager < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn extensions_reloaded(&mut self) -> SigExtensionsReloaded < 'c, C > {
            SigExtensionsReloaded {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "extensions_reloaded")
            }
        }
        #[doc = "Signature: `(extension: Gd<GDExtension>)`"]
        pub fn extension_loaded(&mut self) -> SigExtensionLoaded < 'c, C > {
            SigExtensionLoaded {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "extension_loaded")
            }
        }
        #[doc = "Signature: `(extension: Gd<GDExtension>)`"]
        pub fn extension_unloading(&mut self) -> SigExtensionUnloading < 'c, C > {
            SigExtensionUnloading {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "extension_unloading")
            }
        }
    }
    type TypedSigExtensionsReloaded < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigExtensionsReloaded < 'c, C: WithSignals > {
        typed: TypedSigExtensionsReloaded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigExtensionsReloaded < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigExtensionsReloaded < 'c, C > {
        type Target = TypedSigExtensionsReloaded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigExtensionsReloaded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigExtensionLoaded < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::GDExtension >,) >;
    pub struct SigExtensionLoaded < 'c, C: WithSignals > {
        typed: TypedSigExtensionLoaded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigExtensionLoaded < 'c, C > {
        pub fn emit(&mut self, extension: Gd < crate::classes::GDExtension >,) {
            self.typed.emit_tuple((extension,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigExtensionLoaded < 'c, C > {
        type Target = TypedSigExtensionLoaded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigExtensionLoaded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigExtensionUnloading < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::GDExtension >,) >;
    pub struct SigExtensionUnloading < 'c, C: WithSignals > {
        typed: TypedSigExtensionUnloading < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigExtensionUnloading < 'c, C > {
        pub fn emit(&mut self, extension: Gd < crate::classes::GDExtension >,) {
            self.typed.emit_tuple((extension,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigExtensionUnloading < 'c, C > {
        type Target = TypedSigExtensionUnloading < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigExtensionUnloading < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for GDExtensionManager {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfGDExtensionManager < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfGDExtensionManager < 'c, C > {
        type Target = < < GDExtensionManager as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = GDExtensionManager;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfGDExtensionManager < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = GDExtensionManager;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
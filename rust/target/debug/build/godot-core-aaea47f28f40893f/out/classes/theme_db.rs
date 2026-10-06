#![doc = "Sidecar module for class [`ThemeDb`][crate::classes::ThemeDb].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ThemeDB` enums](https://docs.godotengine.org/en/stable/classes/class_themedb.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ThemeDB`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`theme_db`][crate::classes::theme_db]: sidecar module with related enum/flag types\n* [`SignalsOfThemeDb`][crate::classes::theme_db::SignalsOfThemeDb]: signal collection\n\n\nSee also [Godot docs for `ThemeDB`](https://docs.godotengine.org/en/stable/classes/class_themedb.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThis singleton provides access to static information about [`Theme`][crate::classes::Theme] resources used by the engine and by your projects. You can fetch the default engine theme, as well as your project configured theme.\n\n`ThemeDB` also contains fallback values for theme properties."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ThemeDb {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ThemeDb {
        #[doc = "Returns a reference to the default engine [`Theme`][crate::classes::Theme]. This theme resource is responsible for the out-of-the-box look of [`Control`][crate::classes::Control] nodes and cannot be overridden."]
        pub fn get_default_theme(&self,) -> Option < Gd < crate::classes::Theme > > {
            type CallRet = Option < Gd < crate::classes::Theme > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ThemeDb", "get_default_theme", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a reference to the custom project [`Theme`][crate::classes::Theme]. This theme resources allows to override the default engine theme for every control node in the project.\n\nTo set the project theme, see \\[member ProjectSettings.gui/theme/custom]."]
        pub fn get_project_theme(&self,) -> Option < Gd < crate::classes::Theme > > {
            type CallRet = Option < Gd < crate::classes::Theme > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ThemeDb", "get_project_theme", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fallback_base_scale(&mut self, base_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (base_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ThemeDb", "set_fallback_base_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fallback_base_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ThemeDb", "get_fallback_base_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fallback_font(&mut self, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >,);
            let args = (font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ThemeDb", "set_fallback_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fallback_font(&self,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ThemeDb", "get_fallback_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fallback_font_size(&mut self, font_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ThemeDb", "set_fallback_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fallback_font_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ThemeDb", "get_fallback_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fallback_icon(&mut self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ThemeDb", "set_fallback_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fallback_icon(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ThemeDb", "get_fallback_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fallback_stylebox(&mut self, stylebox: impl AsArg < Option < Gd < crate::classes::StyleBox >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::StyleBox > > >,);
            let args = (stylebox.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ThemeDb", "set_fallback_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fallback_stylebox(&self,) -> Option < Gd < crate::classes::StyleBox > > {
            type CallRet = Option < Gd < crate::classes::StyleBox > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ThemeDb", "get_fallback_stylebox", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ThemeDb {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ThemeDB"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ThemeDb {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ThemeDb {
        
    }
    impl crate::obj::Singleton for ThemeDb {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"ThemeDB"))
            }
        }
    }
    impl std::ops::Deref for ThemeDb {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ThemeDb {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ThemeDb__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ThemeDb` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ThemeDb;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`ThemeDb`][crate::classes::ThemeDb] class."]
    pub struct SignalsOfThemeDb < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfThemeDb < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn fallback_changed(&mut self) -> SigFallbackChanged < 'c, C > {
            SigFallbackChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "fallback_changed")
            }
        }
    }
    type TypedSigFallbackChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFallbackChanged < 'c, C: WithSignals > {
        typed: TypedSigFallbackChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFallbackChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFallbackChanged < 'c, C > {
        type Target = TypedSigFallbackChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFallbackChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for ThemeDb {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfThemeDb < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfThemeDb < 'c, C > {
        type Target = < < ThemeDb as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = ThemeDb;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfThemeDb < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = ThemeDb;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
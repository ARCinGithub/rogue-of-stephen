#![doc = "Sidecar module for class [`Marshalls`][crate::classes::Marshalls].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Marshalls` enums](https://docs.godotengine.org/en/stable/classes/class_marshalls.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Marshalls`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`marshalls`][crate::classes::marshalls]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Marshalls`](https://docs.godotengine.org/en/stable/classes/class_marshalls.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nProvides data transformation and encoding utility functions."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Marshalls {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Marshalls {
        #[doc = "Returns a Base64-encoded string of the [`Variant`][crate::builtin::Variant] `variant`. If `full_objects` is `true`, encoding objects is allowed (and can potentially include code).\n\nInternally, this uses the same encoding mechanism as the [`var_to_bytes`][`crate::global::var_to_bytes`] method."]
        pub(crate) fn variant_to_base64_full(&mut self, variant: RefArg < Variant >, full_objects: bool,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, bool,);
            let args = (variant, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Marshalls", "variant_to_base64", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`variant_to_base64_ex`][Self::variant_to_base64_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a Base64-encoded string of the [`Variant`][crate::builtin::Variant] `variant`. If `full_objects` is `true`, encoding objects is allowed (and can potentially include code).\n\nInternally, this uses the same encoding mechanism as the [`var_to_bytes`][`crate::global::var_to_bytes`] method."]
        #[inline]
        pub fn variant_to_base64(&mut self, variant: &Variant,) -> GString {
            self.variant_to_base64_ex(variant,) . done()
        }
        #[doc = "Returns a Base64-encoded string of the [`Variant`][crate::builtin::Variant] `variant`. If `full_objects` is `true`, encoding objects is allowed (and can potentially include code).\n\nInternally, this uses the same encoding mechanism as the [`var_to_bytes`][`crate::global::var_to_bytes`] method."]
        #[inline]
        pub fn variant_to_base64_ex < 'ex > (&'ex mut self, variant: &'ex Variant,) -> ExVariantToBase64 < 'ex > {
            ExVariantToBase64::new(self, variant,)
        }
        #[doc = "Returns a decoded [`Variant`][crate::builtin::Variant] corresponding to the Base64-encoded string `base64_str`. If `allow_objects` is `true`, decoding objects is allowed.\n\nInternally, this uses the same decoding mechanism as the [`bytes_to_var`][`crate::global::bytes_to_var`] method.\n\n**Warning:** Deserialized objects can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats such as remote code execution."]
        pub(crate) fn base64_to_variant_full(&mut self, base64_str: CowArg < GString >, allow_objects: bool,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (base64_str, allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Marshalls", "base64_to_variant", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`base64_to_variant_ex`][Self::base64_to_variant_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a decoded [`Variant`][crate::builtin::Variant] corresponding to the Base64-encoded string `base64_str`. If `allow_objects` is `true`, decoding objects is allowed.\n\nInternally, this uses the same decoding mechanism as the [`bytes_to_var`][`crate::global::bytes_to_var`] method.\n\n**Warning:** Deserialized objects can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats such as remote code execution."]
        #[inline]
        pub fn base64_to_variant(&mut self, base64_str: impl AsArg < GString >,) -> Variant {
            self.base64_to_variant_ex(base64_str,) . done()
        }
        #[doc = "Returns a decoded [`Variant`][crate::builtin::Variant] corresponding to the Base64-encoded string `base64_str`. If `allow_objects` is `true`, decoding objects is allowed.\n\nInternally, this uses the same decoding mechanism as the [`bytes_to_var`][`crate::global::bytes_to_var`] method.\n\n**Warning:** Deserialized objects can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats such as remote code execution."]
        #[inline]
        pub fn base64_to_variant_ex < 'ex > (&'ex mut self, base64_str: impl AsArg < GString > + 'ex,) -> ExBase64ToVariant < 'ex > {
            ExBase64ToVariant::new(self, base64_str,)
        }
        #[doc = "Returns a Base64-encoded string of a given [`PackedByteArray`][crate::builtin::PackedByteArray]."]
        pub fn raw_to_base64(&mut self, array: &PackedByteArray,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Marshalls", "raw_to_base64", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a decoded [`PackedByteArray`][crate::builtin::PackedByteArray] corresponding to the Base64-encoded string `base64_str`."]
        pub fn base64_to_raw(&mut self, base64_str: impl AsArg < GString >,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (base64_str.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Marshalls", "base64_to_raw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a Base64-encoded string of the UTF-8 string `utf8_str`."]
        pub fn utf8_to_base64(&mut self, utf8_str: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (utf8_str.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Marshalls", "utf8_to_base64", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a decoded string corresponding to the Base64-encoded string `base64_str`."]
        pub fn base64_to_utf8(&mut self, base64_str: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (base64_str.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Marshalls", "base64_to_utf8", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Marshalls {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Marshalls"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Marshalls {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Marshalls {
        
    }
    impl crate::obj::Singleton for Marshalls {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"Marshalls"))
            }
        }
    }
    impl std::ops::Deref for Marshalls {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Marshalls {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Marshalls__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Marshalls` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Marshalls::variant_to_base64_ex`][super::Marshalls::variant_to_base64_ex]."]
#[must_use]
pub struct ExVariantToBase64 < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Marshalls, variant: CowArg < 'ex, Variant >, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExVariantToBase64 < 'ex > {
    fn new(surround_object: &'ex mut re_export::Marshalls, variant: &'ex Variant,) -> Self {
        let full_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, variant: CowArg::Borrowed(variant), full_objects: full_objects,
        }
    }
    #[inline]
    pub fn full_objects(self, full_objects: bool) -> Self {
        Self {
            full_objects: full_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, variant, full_objects,
        }
        = self;
        re_export::Marshalls::variant_to_base64_full(surround_object, variant.cow_as_arg(), full_objects,)
    }
}
#[doc = "Default-param extender for [`Marshalls::base64_to_variant_ex`][super::Marshalls::base64_to_variant_ex]."]
#[must_use]
pub struct ExBase64ToVariant < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Marshalls, base64_str: CowArg < 'ex, GString >, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBase64ToVariant < 'ex > {
    fn new(surround_object: &'ex mut re_export::Marshalls, base64_str: impl AsArg < GString > + 'ex,) -> Self {
        let allow_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, base64_str: base64_str.into_arg(), allow_objects: allow_objects,
        }
    }
    #[inline]
    pub fn allow_objects(self, allow_objects: bool) -> Self {
        Self {
            allow_objects: allow_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, base64_str, allow_objects,
        }
        = self;
        re_export::Marshalls::base64_to_variant_full(surround_object, base64_str, allow_objects,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Marshalls;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Marshalls {
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
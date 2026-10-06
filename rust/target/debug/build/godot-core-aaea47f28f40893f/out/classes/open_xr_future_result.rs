#![doc = "Sidecar module for class [`OpenXrFutureResult`][crate::classes::OpenXrFutureResult].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRFutureResult` enums](https://docs.godotengine.org/en/stable/classes/class_openxrfutureresult.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRFutureResult`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`open_xr_future_result`][crate::classes::open_xr_future_result]: sidecar module with related enum/flag types\n* [`SignalsOfOpenXrFutureResult`][crate::classes::open_xr_future_result::SignalsOfOpenXrFutureResult]: signal collection\n\n\nSee also [Godot docs for `OpenXRFutureResult`](https://docs.godotengine.org/en/stable/classes/class_openxrfutureresult.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<OpenXrFutureResult>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nResult object tracking the asynchronous result of an OpenXR Future object, you can use this object to track the result status."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrFutureResult {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl OpenXrFutureResult {
        #[doc = "Returns the status of this result."]
        pub fn get_status(&self,) -> crate::classes::open_xr_future_result::ResultStatus {
            type CallRet = crate::classes::open_xr_future_result::ResultStatus;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrFutureResult", "get_status", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Return the `XrFutureEXT` value this result relates to."]
        pub fn get_future(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7066usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrFutureResult", "get_future", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Cancel this future, this will interrupt and stop the asynchronous function."]
        pub fn cancel_future(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7067usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrFutureResult", "cancel_future", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stores the result value we expose to the user.\n\n**Note:** This method should only be called by an OpenXR extension that implements an asynchronous function."]
        pub fn set_result_value(&mut self, result_value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (RefArg::new(result_value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrFutureResult", "set_result_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the result value of our asynchronous function (if set by the extension). The type of this result value depends on the function being called. Consult the documentation of the relevant function."]
        pub fn get_result_value(&self,) -> Variant {
            type CallRet = Variant;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrFutureResult", "get_result_value", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrFutureResult {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRFutureResult"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrFutureResult {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for OpenXrFutureResult {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrFutureResult {
        
    }
    impl std::ops::Deref for OpenXrFutureResult {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrFutureResult {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrFutureResult__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `OpenXrFutureResult` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ResultStatus {
    ord: i32
}
impl ResultStatus {
    #[doc(alias = "RESULT_RUNNING")]
    #[doc = "Godot enumerator name: `RESULT_RUNNING`"]
    pub const RUNNING: ResultStatus = ResultStatus {
        ord: 0i32
    };
    #[doc(alias = "RESULT_FINISHED")]
    #[doc = "Godot enumerator name: `RESULT_FINISHED`"]
    pub const FINISHED: ResultStatus = ResultStatus {
        ord: 1i32
    };
    #[doc(alias = "RESULT_CANCELLED")]
    #[doc = "Godot enumerator name: `RESULT_CANCELLED`"]
    pub const CANCELLED: ResultStatus = ResultStatus {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ResultStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ResultStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ResultStatus {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::RUNNING => "RUNNING", Self::FINISHED => "FINISHED", Self::CANCELLED => "CANCELLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ResultStatus::RUNNING, ResultStatus::FINISHED, ResultStatus::CANCELLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ResultStatus >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RUNNING", "RESULT_RUNNING", ResultStatus::RUNNING), crate::meta::inspect::EnumConstant::new("FINISHED", "RESULT_FINISHED", ResultStatus::FINISHED), crate::meta::inspect::EnumConstant::new("CANCELLED", "RESULT_CANCELLED", ResultStatus::CANCELLED)]
        }
    }
}
impl crate::meta::GodotConvert for ResultStatus {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Result Running", 0i64), EnumeratorShape::new_int("Result Finished", 1i64), EnumeratorShape::new_int("Result Cancelled", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("OpenXRFutureResult.ResultStatus")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ResultStatus {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ResultStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ResultStatus {
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
impl crate::registry::property::Export for ResultStatus {
    
}
impl crate::meta::Element for ResultStatus {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrFutureResult;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`OpenXrFutureResult`][crate::classes::OpenXrFutureResult] class."]
    pub struct SignalsOfOpenXrFutureResult < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfOpenXrFutureResult < 'c, C > {
        #[doc = "Signature: `(result: Gd<OpenXrFutureResult>)`"]
        pub fn completed(&mut self) -> SigCompleted < 'c, C > {
            SigCompleted {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "completed")
            }
        }
    }
    type TypedSigCompleted < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::OpenXrFutureResult >,) >;
    pub struct SigCompleted < 'c, C: WithSignals > {
        typed: TypedSigCompleted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCompleted < 'c, C > {
        pub fn emit(&mut self, result: Gd < crate::classes::OpenXrFutureResult >,) {
            self.typed.emit_tuple((result,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCompleted < 'c, C > {
        type Target = TypedSigCompleted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCompleted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for OpenXrFutureResult {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfOpenXrFutureResult < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfOpenXrFutureResult < 'c, C > {
        type Target = < < OpenXrFutureResult as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = OpenXrFutureResult;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfOpenXrFutureResult < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = OpenXrFutureResult;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
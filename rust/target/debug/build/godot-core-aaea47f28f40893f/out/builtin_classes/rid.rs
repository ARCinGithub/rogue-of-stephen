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
pub(super) mod re_export {
    use super::*;
    #[doc(hidden)]
    #[repr(transparent)]
    pub struct InnerRid < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerRid < 'inner > {
    pub fn from_outer(outer: &Rid) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns `true` if the [`RID`][crate::builtin::Rid] is not `0`."]
    pub fn is_valid(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(604usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rid", "is_valid", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the ID of the referenced low-level resource."]
    pub fn get_id(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(605usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rid", "get_id", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerRid;
impl Rid {
    
}
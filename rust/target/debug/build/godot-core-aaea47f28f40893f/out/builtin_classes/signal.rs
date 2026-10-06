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
    pub struct InnerSignal < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerSignal < 'inner > {
    pub fn from_outer(outer: &Signal) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns `true` if this [`Signal`][crate::builtin::Signal] has no object and the signal name is empty. Equivalent to `signal == Signal()`."]
    pub fn is_null(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(627usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "is_null", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the object emitting this signal."]
    pub fn get_object(&self,) -> Option < Gd < crate::classes::Object > > {
        type CallRet = Option < Gd < crate::classes::Object > >;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(628usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "get_object", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the ID of the object emitting this signal (see [`instance_id`][`crate::obj::Gd::instance_id`])."]
    pub fn get_object_id(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(629usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "get_object_id", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the name of this signal."]
    pub fn get_name(&self,) -> StringName {
        type CallRet = StringName;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(630usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "get_name", self.sys_ptr, args)
        }
    }
    #[doc = "Connects this signal to the specified `callable`. Optional `flags` can be also added to configure the connection's behavior (see \\[enum Object.ConnectFlags] constants). You can provide additional arguments to the connected `callable` by using [`bind`][`crate::builtin::Callable::bind`].\n\nA signal can only be connected once to the same [`Callable`][crate::builtin::Callable]. If the signal is already connected, this method returns [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] and generates an error, unless the signal is connected with [`ConnectFlags::REFERENCE_COUNTED`][`crate::classes::object::ConnectFlags::REFERENCE_COUNTED`]. To prevent this, use \\[method is_connected] first to check for existing connections.\n\n```gdscript\nfor button in $Buttons.get_children():\n\tbutton.pressed.connect(_on_pressed.bind(button))\n\nfunc _on_pressed(button):\n\tprint(button.name, \" was pressed\")\n```\n\n**Note:** If the `callable`'s object is freed, the connection will be lost."]
    pub fn connect(&mut self, callable: &Callable, flags: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i64,);
        let args = (RefArg::new(callable), flags,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(631usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "connect", self.sys_ptr, args)
        }
    }
    #[doc = "Disconnects this signal from the specified [`Callable`][crate::builtin::Callable]. If the connection does not exist, generates an error. Use \\[method is_connected] to make sure that the connection exists."]
    pub fn disconnect(&mut self, callable: &Callable,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(callable),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(632usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "disconnect", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the specified [`Callable`][crate::builtin::Callable] is connected to this signal."]
    pub fn is_connected(&self, callable: &Callable,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(callable),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(633usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "is_connected", self.sys_ptr, args)
        }
    }
    #[doc = "Returns an [`Array`][crate::builtin::Array] of connections for this signal. Each connection is represented as a [`Dictionary`][crate::builtin::Dictionary] that contains three entries:\n\n- `signal` is a reference to this signal;\n\n- `callable` is a reference to the connected [`Callable`][crate::builtin::Callable];\n\n- `flags` is a combination of \\[enum Object.ConnectFlags]."]
    pub fn get_connections(&self,) -> VarArray {
        type CallRet = VarArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(634usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "get_connections", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if any [`Callable`][crate::builtin::Callable] is connected to this signal."]
    pub fn has_connections(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(635usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "has_connections", self.sys_ptr, args)
        }
    }
    #[doc = "Emits this signal. All [`Callable`][crate::builtin::Callable]s connected to this signal will be triggered. This method supports a variable number of arguments, so parameters can be passed as a comma separated list."]
    pub fn emit(&self, varargs: &[Variant]) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(636usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall_varargs(method_bind, "Signal", "emit", self.sys_ptr, args, varargs)
        }
    }
}
pub use re_export::InnerSignal;
impl Signal {
    
}
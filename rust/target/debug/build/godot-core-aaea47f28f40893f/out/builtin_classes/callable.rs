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
    pub struct InnerCallable < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerCallable < 'inner > {
    pub fn from_outer(outer: &Callable) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Creates a new [`Callable`][crate::builtin::Callable] for the method named `method` in the specified `variant`. To represent a method of a built-in [`Variant`][crate::builtin::Variant] type, a custom callable is used (see \\[method is_custom]). If `variant` is [`Object`][crate::classes::Object], then a standard callable will be created instead.\n\n**Note:** This method is always necessary for the [`Dictionary`][crate::builtin::Dictionary] type, as property syntax is used to access its entries. You may also use this method when `variant`'s type is not known in advance (for polymorphism)."]
    pub fn create(variant: &Variant, method: impl AsArg < StringName >,) -> Callable {
        type CallRet = Callable;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, CowArg < 'a1, StringName >,);
        let args = (RefArg::new(variant), method.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(606usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "create", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Calls the method represented by this [`Callable`][crate::builtin::Callable]. Unlike \\[method call], this method expects all arguments to be contained inside the `arguments` [`Array`][crate::builtin::Array]."]
    pub fn callv(&self, arguments: &AnyArray,) -> Variant {
        type CallRet = Variant;
        type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
        let args = (RefArg::new(arguments),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(607usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "callv", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this [`Callable`][crate::builtin::Callable] has no target to call the method on. Equivalent to `callable == Callable()`.\n\n**Note:** This is _not_ the same as `not is_valid()` and using `not is_null()` will _not_ guarantee that this callable can be called. Use \\[method is_valid] instead."]
    pub fn is_null(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(608usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "is_null", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this [`Callable`][crate::builtin::Callable] is a custom callable. Custom callables are used:\n\n- for binding/unbinding arguments (see \\[method bind] and \\[method unbind]);\n\n- for representing methods of built-in [`Variant`][crate::builtin::Variant] types (see \\[method create]);\n\n- for representing global, lambda, and RPC functions in GDScript;\n\n- for other purposes in the core, GDExtension, and C#."]
    pub fn is_custom(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(609usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "is_custom", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this [`Callable`][crate::builtin::Callable] is a standard callable. This method is the opposite of \\[method is_custom]. Returns `false` if this callable is a lambda function."]
    pub fn is_standard(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(610usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "is_standard", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the callable's object exists and has a valid method name assigned, or is a custom callable."]
    pub fn is_valid(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(611usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "is_valid", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the object on which this [`Callable`][crate::builtin::Callable] is called."]
    pub fn get_object(&self,) -> Option < Gd < crate::classes::Object > > {
        type CallRet = Option < Gd < crate::classes::Object > >;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(612usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_object", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the ID of this [`Callable`][crate::builtin::Callable]'s object (see [`instance_id`][`crate::obj::Gd::instance_id`])."]
    pub fn get_object_id(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(613usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_object_id", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the name of the method represented by this [`Callable`][crate::builtin::Callable]. If the callable is a GDScript lambda function, returns the function's name or `\"<anonymous lambda>\"`."]
    pub fn get_method(&self,) -> StringName {
        type CallRet = StringName;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(614usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_method", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the total number of arguments this [`Callable`][crate::builtin::Callable] should take, including optional arguments. This means that any arguments bound with \\[method bind] are _subtracted_ from the result, and any arguments unbound with \\[method unbind] are _added_ to the result."]
    pub fn get_argument_count(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(615usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_argument_count", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the total amount of arguments bound via successive \\[method bind] or \\[method unbind] calls. This is the same as the size of the array returned by \\[method get_bound_arguments]. See \\[method get_bound_arguments] for details.\n\n**Note:** The \\[method get_bound_arguments_count] and \\[method get_unbound_arguments_count] methods can both return positive values."]
    pub fn get_bound_arguments_count(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(616usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_bound_arguments_count", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the total amount of arguments unbound via successive \\[method bind] or \\[method unbind] calls. See \\[method get_bound_arguments] for details.\n\n**Note:** The \\[method get_bound_arguments_count] and \\[method get_unbound_arguments_count] methods can both return positive values."]
    pub fn get_unbound_arguments_count(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(618usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_unbound_arguments_count", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the 32-bit hash value of this [`Callable`][crate::builtin::Callable]'s object.\n\n**Note:** [`Callable`][crate::builtin::Callable]s with equal content will always produce identical hash values. However, the reverse is not true. Returning identical hash values does _not_ imply the callables are equal, because different callables can have identical hash values due to hash collisions. The engine uses a 32-bit hash algorithm for \\[method hash]."]
    pub fn hash(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(619usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "hash", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this [`Callable`][crate::builtin::Callable] with one or more arguments bound, reading them from an array. When called, the bound arguments are passed _after_ the arguments supplied by \\[method call]. See also \\[method unbind].\n\n**Note:** When this method is chained with other similar methods, the order in which the argument list is modified is read from right to left."]
    pub fn bindv(&mut self, arguments: &AnyArray,) -> Callable {
        type CallRet = Callable;
        type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
        let args = (RefArg::new(arguments),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(620usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "bindv", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this [`Callable`][crate::builtin::Callable] with a number of arguments unbound. In other words, when the new callable is called the last few arguments supplied by the user are ignored, according to `argcount`. The remaining arguments are passed to the callable. This allows to use the original callable in a context that attempts to pass more arguments than this callable can handle, e.g. a signal with a fixed number of arguments. See also \\[method bind].\n\n**Note:** When this method is chained with other similar methods, the order in which the argument list is modified is read from right to left.\n\n```gdscript\nfunc _ready():\n\tfoo.unbind(1).call(1, 2) # Calls foo(1).\n\tfoo.bind(3, 4).unbind(1).call(1, 2) # Calls foo(1, 3, 4), note that it does not change the arguments from bind.\n```"]
    pub fn unbind(&self, argcount: i64,) -> Callable {
        type CallRet = Callable;
        type CallParams = (i64,);
        let args = (argcount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(621usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "unbind", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerCallable;
impl Callable {
    #[doc = "Returns the array of arguments bound via successive \\[method bind] or \\[method unbind] calls. These arguments will be added _after_ the arguments passed to the call, from which \\[method get_unbound_arguments_count] arguments on the right have been previously excluded.\n\n```gdscript\nfunc get_effective_arguments(callable, call_args):\n\tassert(call_args.size() - callable.get_unbound_arguments_count() >= 0)\n\tvar result = call_args.slice(0, call_args.size() - callable.get_unbound_arguments_count())\n\tresult.append_array(callable.get_bound_arguments())\n\treturn result\n```"]
    pub fn get_bound_arguments(&self,) -> VarArray {
        type CallRet = VarArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(617usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_bound_arguments", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "Calls the method represented by this [`Callable`][crate::builtin::Callable]. Arguments can be passed and should match the method's signature."]
    pub fn call(&self, varargs: &[Variant]) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(622usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "call", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
    #[doc = "Calls the method represented by this [`Callable`][crate::builtin::Callable] in deferred mode, i.e. at the end of the current frame. Arguments can be passed and should match the method's signature.\n\n\n```gdscript\nfunc _ready():\n\tgrab_focus.call_deferred()\n```\n\n\n**Note:** Deferred calls are processed at idle time. Idle time happens mainly at the end of process and physics frames. In it, deferred calls will be run until there are none left, which means you can defer calls from other deferred calls and they'll still be run in the current idle time cycle. This means you should not call a method deferred from itself (or from a method called by it), as this causes infinite recursion the same way as if you had called the method directly.\n\nSee also [`call_deferred`][`crate::classes::Object::call_deferred`]."]
    pub fn call_deferred(&self, varargs: &[Variant]) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(623usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "call_deferred", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
    #[doc = "Perform an RPC (Remote Procedure Call) on all connected peers. This is used for multiplayer and is normally not available, unless the function being called has been marked as _RPC_ (using `@GDScript.@rpc` or [`rpc_config`][`crate::classes::Node::rpc_config`]). Calling this method on unsupported functions will result in an error. See [`rpc`][`crate::classes::Node::rpc`]."]
    pub fn rpc(&self, varargs: &[Variant]) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(624usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "rpc", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
    #[doc = "Perform an RPC (Remote Procedure Call) on a specific peer ID (see multiplayer documentation for reference). This is used for multiplayer and is normally not available unless the function being called has been marked as _RPC_ (using `@GDScript.@rpc` or [`rpc_config`][`crate::classes::Node::rpc_config`]). Calling this method on unsupported functions will result in an error. See [`rpc_id`][`crate::classes::Node::rpc_id`]."]
    pub fn rpc_id(&self, peer_id: i64, varargs: &[Variant]) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (peer_id,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(625usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "rpc_id", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
    #[doc = "Returns a copy of this [`Callable`][crate::builtin::Callable] with one or more arguments bound. When called, the bound arguments are passed _after_ the arguments supplied by \\[method call]. See also \\[method unbind].\n\n**Note:** When this method is chained with other similar methods, the order in which the argument list is modified is read from right to left."]
    pub fn bind(&self, varargs: &[Variant]) -> Callable {
        type CallRet = Callable;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(626usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "bind", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
}
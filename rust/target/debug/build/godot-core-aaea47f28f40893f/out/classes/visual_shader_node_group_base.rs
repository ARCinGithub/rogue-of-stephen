#![doc = "Sidecar module for class [`VisualShaderNodeGroupBase`][crate::classes::VisualShaderNodeGroupBase].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeGroupBase` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodegroupbase.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeGroupBase`.\n\nInherits [`VisualShaderNodeResizableBase`][crate::classes::VisualShaderNodeResizableBase].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `VisualShaderNodeGroupBase`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodegroupbase.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<VisualShaderNodeGroupBase>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nCurrently, has no direct usage, use the derived classes instead."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeGroupBase {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl VisualShaderNodeGroupBase {
        #[doc = "Defines all input ports using a [`String`][crate::builtin::GString] formatted as a colon-separated list: `id,type,name;` (see [`add_input_port`][`crate::classes::VisualShaderNodeGroupBase::add_input_port`])."]
        pub fn set_inputs(&mut self, inputs: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (inputs.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "set_inputs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`String`][crate::builtin::GString] description of the input ports as a colon-separated list using the format `id,type,name;` (see [`add_input_port`][`crate::classes::VisualShaderNodeGroupBase::add_input_port`])."]
        pub fn get_inputs(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2009usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "get_inputs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Defines all output ports using a [`String`][crate::builtin::GString] formatted as a colon-separated list: `id,type,name;` (see [`add_output_port`][`crate::classes::VisualShaderNodeGroupBase::add_output_port`])."]
        pub fn set_outputs(&mut self, outputs: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (outputs.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2010usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "set_outputs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`String`][crate::builtin::GString] description of the output ports as a colon-separated list using the format `id,type,name;` (see [`add_output_port`][`crate::classes::VisualShaderNodeGroupBase::add_output_port`])."]
        pub fn get_outputs(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2011usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "get_outputs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the specified port name does not override an existed port name and is valid within the shader."]
        pub fn is_valid_port_name(&self, name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "is_valid_port_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an input port with the specified `type` (see \\[enum VisualShaderNode.PortType]) and `name`."]
        pub fn add_input_port(&mut self, id: i32, type_: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (id, type_, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2013usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "add_input_port", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the specified input port."]
        pub fn remove_input_port(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2014usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "remove_input_port", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of input ports in use. Alternative for [`get_free_input_port_id`][`crate::classes::VisualShaderNodeGroupBase::get_free_input_port_id`]."]
        pub fn get_input_port_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2015usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "get_input_port_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the specified input port exists."]
        pub fn has_input_port(&self, id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "has_input_port", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all previously specified input ports."]
        pub fn clear_input_ports(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "clear_input_ports", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an output port with the specified `type` (see \\[enum VisualShaderNode.PortType]) and `name`."]
        pub fn add_output_port(&mut self, id: i32, type_: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (id, type_, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2018usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "add_output_port", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the specified output port."]
        pub fn remove_output_port(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2019usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "remove_output_port", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of output ports in use. Alternative for [`get_free_output_port_id`][`crate::classes::VisualShaderNodeGroupBase::get_free_output_port_id`]."]
        pub fn get_output_port_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2020usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "get_output_port_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the specified output port exists."]
        pub fn has_output_port(&self, id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2021usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "has_output_port", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all previously specified output ports."]
        pub fn clear_output_ports(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2022usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "clear_output_ports", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renames the specified input port."]
        pub fn set_input_port_name(&mut self, id: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (id, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2023usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "set_input_port_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the specified input port's type (see \\[enum VisualShaderNode.PortType])."]
        pub fn set_input_port_type(&mut self, id: i32, type_: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (id, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2024usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "set_input_port_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renames the specified output port."]
        pub fn set_output_port_name(&mut self, id: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (id, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "set_output_port_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the specified output port's type (see \\[enum VisualShaderNode.PortType])."]
        pub fn set_output_port_type(&mut self, id: i32, type_: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (id, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "set_output_port_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a free input port ID which can be used in [`add_input_port`][`crate::classes::VisualShaderNodeGroupBase::add_input_port`]."]
        pub fn get_free_input_port_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "get_free_input_port_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a free output port ID which can be used in [`add_output_port`][`crate::classes::VisualShaderNodeGroupBase::add_output_port`]."]
        pub fn get_free_output_port_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "get_free_output_port_id", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeGroupBase {
        type Base = crate::classes::VisualShaderNodeResizableBase;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisualShaderNodeGroupBase"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeGroupBase {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNodeResizableBase > for VisualShaderNodeGroupBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeGroupBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeGroupBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeGroupBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeGroupBase {
        
    }
    impl std::ops::Deref for VisualShaderNodeGroupBase {
        type Target = crate::classes::VisualShaderNodeResizableBase;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeGroupBase {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeGroupBase__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `VisualShaderNodeGroupBase` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNodeGroupBase;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeGroupBase {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
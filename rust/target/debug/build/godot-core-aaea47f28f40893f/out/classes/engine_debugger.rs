#![doc = "Sidecar module for class [`EngineDebugger`][crate::classes::EngineDebugger].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EngineDebugger` enums](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EngineDebugger`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`engine_debugger`][crate::classes::engine_debugger]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `EngineDebugger`](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\n`EngineDebugger` handles the communication between the editor and the running game. It is active in the running game. Messages can be sent/received through it. It also manages the profilers."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EngineDebugger {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl EngineDebugger {
        #[doc = "Returns `true` if the debugger is active otherwise `false`."]
        pub fn is_active(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "is_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers a profiler with the given `name`. See [`EngineProfiler`][crate::classes::EngineProfiler] for more information."]
        pub fn register_profiler(&mut self, name: impl AsArg < StringName >, profiler: impl AsArg < Option < Gd < crate::classes::EngineProfiler >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::EngineProfiler > > >,);
            let args = (name.into_arg(), profiler.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "register_profiler", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unregisters a profiler with given `name`."]
        pub fn unregister_profiler(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "unregister_profiler", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a profiler with the given name is present and active otherwise `false`."]
        pub fn is_profiling(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "is_profiling", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a profiler with the given name is present otherwise `false`."]
        pub fn has_profiler(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "has_profiler", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calls the `add` callable of the profiler with given `name` and `data`."]
        pub fn profiler_add_frame_data(&mut self, name: impl AsArg < StringName >, data: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, AnyArray >,);
            let args = (name.into_arg(), RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "profiler_add_frame_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calls the `toggle` callable of the profiler with given `name` and `arguments`. Enables/Disables the same profiler depending on `enable` argument."]
        pub(crate) fn profiler_enable_full(&mut self, name: CowArg < StringName >, enable: bool, arguments: RefArg < AnyArray >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, bool, RefArg < 'a1, AnyArray >,);
            let args = (name, enable, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "profiler_enable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`profiler_enable_ex`][Self::profiler_enable_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Calls the `toggle` callable of the profiler with given `name` and `arguments`. Enables/Disables the same profiler depending on `enable` argument."]
        #[inline]
        pub fn profiler_enable(&mut self, name: impl AsArg < StringName >, enable: bool,) {
            self.profiler_enable_ex(name, enable,) . done()
        }
        #[doc = "Calls the `toggle` callable of the profiler with given `name` and `arguments`. Enables/Disables the same profiler depending on `enable` argument."]
        #[inline]
        pub fn profiler_enable_ex < 'ex > (&'ex mut self, name: impl AsArg < StringName > + 'ex, enable: bool,) -> ExProfilerEnable < 'ex > {
            ExProfilerEnable::new(self, name, enable,)
        }
        #[doc = "Registers a message capture with given `name`. If `name` is \"my_message\" then messages starting with \"my_message:\" will be called with the given callable.\n\nThe callable must accept a message string and a data array as argument. The callable should return `true` if the message is recognized.\n\n**Note:** The callable will receive the message with the prefix stripped, unlike [`capture`][`crate::classes::IEditorDebuggerPlugin::capture`]. See the [`EditorDebuggerPlugin`][crate::classes::EditorDebuggerPlugin] description for an example."]
        pub fn register_message_capture(&mut self, name: impl AsArg < StringName >, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Callable >,);
            let args = (name.into_arg(), RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "register_message_capture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unregisters the message capture with given `name`."]
        pub fn unregister_message_capture(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "unregister_message_capture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a capture with the given name is present otherwise `false`."]
        pub fn has_capture(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "has_capture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Forces a processing loop of debugger events. The purpose of this method is just processing events every now and then when the script might get too busy, so that bugs like infinite loops can be caught."]
        pub fn line_poll(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "line_poll", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sends a message with given `message` and `data` array."]
        pub fn send_message(&mut self, message: impl AsArg < GString >, data: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, AnyArray >,);
            let args = (message.into_arg(), RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "send_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Starts a debug break in script execution, optionally specifying whether the program can continue based on `can_continue` and whether the break was due to a breakpoint."]
        pub(crate) fn debug_full(&mut self, can_continue: bool, is_error_breakpoint: bool,) {
            type CallRet = ();
            type CallParams = (bool, bool,);
            let args = (can_continue, is_error_breakpoint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "debug", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`debug_ex`][Self::debug_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Starts a debug break in script execution, optionally specifying whether the program can continue based on `can_continue` and whether the break was due to a breakpoint."]
        #[inline]
        pub fn debug(&mut self,) {
            self.debug_ex() . done()
        }
        #[doc = "Starts a debug break in script execution, optionally specifying whether the program can continue based on `can_continue` and whether the break was due to a breakpoint."]
        #[inline]
        pub fn debug_ex < 'ex > (&'ex mut self,) -> ExDebug < 'ex > {
            ExDebug::new(self,)
        }
        #[doc = "Starts a debug break in script execution, optionally specifying whether the program can continue based on `can_continue` and whether the break was due to a breakpoint."]
        pub(crate) fn script_debug_full(&mut self, language: CowArg < Option < Gd < crate::classes::ScriptLanguage > > >, can_continue: bool, is_error_breakpoint: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ScriptLanguage > > >, bool, bool,);
            let args = (language, can_continue, is_error_breakpoint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "script_debug", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`script_debug_ex`][Self::script_debug_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Starts a debug break in script execution, optionally specifying whether the program can continue based on `can_continue` and whether the break was due to a breakpoint."]
        #[inline]
        pub fn script_debug(&mut self, language: impl AsArg < Option < Gd < crate::classes::ScriptLanguage >> >,) {
            self.script_debug_ex(language,) . done()
        }
        #[doc = "Starts a debug break in script execution, optionally specifying whether the program can continue based on `can_continue` and whether the break was due to a breakpoint."]
        #[inline]
        pub fn script_debug_ex < 'ex > (&'ex mut self, language: impl AsArg < Option < Gd < crate::classes::ScriptLanguage >> > + 'ex,) -> ExScriptDebug < 'ex > {
            ExScriptDebug::new(self, language,)
        }
        #[doc = "Sets the current debugging lines that remain."]
        pub fn set_lines_left(&mut self, lines: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (lines,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "set_lines_left", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of lines that remain."]
        pub fn get_lines_left(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "get_lines_left", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the current debugging depth."]
        pub fn set_depth(&mut self, depth: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "set_depth", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current debug depth."]
        pub fn get_depth(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "get_depth", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `source` and `line` represent an existing breakpoint."]
        pub fn is_breakpoint(&self, line: i32, source: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (line, source.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "is_breakpoint", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the debugger is skipping breakpoints otherwise `false`."]
        pub fn is_skipping_breakpoints(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "is_skipping_breakpoints", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Inserts a new breakpoint with the given `source` and `line`."]
        pub fn insert_breakpoint(&mut self, line: i32, source: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (line, source.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "insert_breakpoint", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a breakpoint with the given `source` and `line`."]
        pub fn remove_breakpoint(&mut self, line: i32, source: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (line, source.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "remove_breakpoint", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all breakpoints."]
        pub fn clear_breakpoints(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "clear_breakpoints", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EngineDebugger {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EngineDebugger"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for EngineDebugger {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EngineDebugger {
        
    }
    impl crate::obj::Singleton for EngineDebugger {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"EngineDebugger"))
            }
        }
    }
    impl std::ops::Deref for EngineDebugger {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EngineDebugger {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EngineDebugger__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `EngineDebugger` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`EngineDebugger::profiler_enable_ex`][super::EngineDebugger::profiler_enable_ex]."]
#[must_use]
pub struct ExProfilerEnable < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EngineDebugger, name: CowArg < 'ex, StringName >, enable: bool, arguments: CowArg < 'ex, AnyArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExProfilerEnable < 'ex > {
    fn new(surround_object: &'ex mut re_export::EngineDebugger, name: impl AsArg < StringName > + 'ex, enable: bool,) -> Self {
        let arguments = AnyArray::new_untyped();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), enable: enable, arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'ex AnyArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, enable, arguments,
        }
        = self;
        re_export::EngineDebugger::profiler_enable_full(surround_object, name, enable, arguments.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`EngineDebugger::debug_ex`][super::EngineDebugger::debug_ex]."]
#[must_use]
pub struct ExDebug < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EngineDebugger, can_continue: bool, is_error_breakpoint: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDebug < 'ex > {
    fn new(surround_object: &'ex mut re_export::EngineDebugger,) -> Self {
        let can_continue = true;
        let is_error_breakpoint = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, can_continue: can_continue, is_error_breakpoint: is_error_breakpoint,
        }
    }
    #[inline]
    pub fn can_continue(self, can_continue: bool) -> Self {
        Self {
            can_continue: can_continue, .. self
        }
    }
    #[inline]
    pub fn is_error_breakpoint(self, is_error_breakpoint: bool) -> Self {
        Self {
            is_error_breakpoint: is_error_breakpoint, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, can_continue, is_error_breakpoint,
        }
        = self;
        re_export::EngineDebugger::debug_full(surround_object, can_continue, is_error_breakpoint,)
    }
}
#[doc = "Default-param extender for [`EngineDebugger::script_debug_ex`][super::EngineDebugger::script_debug_ex]."]
#[must_use]
pub struct ExScriptDebug < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EngineDebugger, language: CowArg < 'ex, Option < Gd < crate::classes::ScriptLanguage > > >, can_continue: bool, is_error_breakpoint: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExScriptDebug < 'ex > {
    fn new(surround_object: &'ex mut re_export::EngineDebugger, language: impl AsArg < Option < Gd < crate::classes::ScriptLanguage >> > + 'ex,) -> Self {
        let can_continue = true;
        let is_error_breakpoint = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, language: language.into_arg(), can_continue: can_continue, is_error_breakpoint: is_error_breakpoint,
        }
    }
    #[inline]
    pub fn can_continue(self, can_continue: bool) -> Self {
        Self {
            can_continue: can_continue, .. self
        }
    }
    #[inline]
    pub fn is_error_breakpoint(self, is_error_breakpoint: bool) -> Self {
        Self {
            is_error_breakpoint: is_error_breakpoint, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, language, can_continue, is_error_breakpoint,
        }
        = self;
        re_export::EngineDebugger::script_debug_full(surround_object, language, can_continue, is_error_breakpoint,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EngineDebugger;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EngineDebugger {
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
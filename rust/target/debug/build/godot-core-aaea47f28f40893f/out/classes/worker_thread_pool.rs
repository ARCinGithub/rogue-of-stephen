#![doc = "Sidecar module for class [`WorkerThreadPool`][crate::classes::WorkerThreadPool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WorkerThreadPool` enums](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WorkerThreadPool`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`worker_thread_pool`][crate::classes::worker_thread_pool]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `WorkerThreadPool`](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThe `WorkerThreadPool` singleton allocates a set of `Thread`s (called worker threads) on project startup and provides methods for offloading tasks to them. This can be used for simple multithreading without having to create `Thread`s.\n\nTasks hold the [`Callable`][crate::builtin::Callable] to be run by the threads. `WorkerThreadPool` can be used to create regular tasks, which will be taken by one worker thread, or group tasks, which can be distributed between multiple worker threads. Group tasks execute the [`Callable`][crate::builtin::Callable] multiple times, which makes them useful for iterating over a lot of elements, such as the enemies in an arena.\n\nHere's a sample on how to offload an expensive function to worker threads:\n\n\n```gdscript\nvar enemies = [] # An array to be filled with enemies.\n\nfunc process_enemy_ai(enemy_index):\n\tvar processed_enemy = enemies[enemy_index]\n\t# Expensive logic...\n\nfunc _process(delta):\n\tvar task_id = WorkerThreadPool.add_group_task(process_enemy_ai, enemies.size())\n\t# Other code...\n\tWorkerThreadPool.wait_for_group_task_completion(task_id)\n\t# Other code that depends on the enemy AI already being processed.\n```\n\n\nThe above code relies on the number of elements in the `enemies` array remaining constant during the multithreaded part.\n\n**Note:** Using this singleton could affect performance negatively if the task being distributed between threads is not computationally expensive."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WorkerThreadPool {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl WorkerThreadPool {
        #[doc = "Adds `action` as a task to be executed by a worker thread. `high_priority` determines if the task has a high priority or a low priority (default). You can optionally provide a `description` to help with debugging.\n\nReturns a task ID that can be used by other methods.\n\n**Warning:** Every task must be waited for completion using [`wait_for_task_completion`][`crate::classes::WorkerThreadPool::wait_for_task_completion`] or [`wait_for_group_task_completion`][`crate::classes::WorkerThreadPool::wait_for_group_task_completion`] at some point so that any allocated resources inside the task can be cleaned up."]
        pub(crate) fn add_task_full(&mut self, action: RefArg < Callable >, high_priority: bool, description: CowArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Callable >, bool, CowArg < 'a1, GString >,);
            let args = (action, high_priority, description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "add_task", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_task_ex`][Self::add_task_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds `action` as a task to be executed by a worker thread. `high_priority` determines if the task has a high priority or a low priority (default). You can optionally provide a `description` to help with debugging.\n\nReturns a task ID that can be used by other methods.\n\n**Warning:** Every task must be waited for completion using [`wait_for_task_completion`][`crate::classes::WorkerThreadPool::wait_for_task_completion`] or [`wait_for_group_task_completion`][`crate::classes::WorkerThreadPool::wait_for_group_task_completion`] at some point so that any allocated resources inside the task can be cleaned up."]
        #[inline]
        pub fn add_task(&mut self, action: &Callable,) -> i64 {
            self.add_task_ex(action,) . done()
        }
        #[doc = "Adds `action` as a task to be executed by a worker thread. `high_priority` determines if the task has a high priority or a low priority (default). You can optionally provide a `description` to help with debugging.\n\nReturns a task ID that can be used by other methods.\n\n**Warning:** Every task must be waited for completion using [`wait_for_task_completion`][`crate::classes::WorkerThreadPool::wait_for_task_completion`] or [`wait_for_group_task_completion`][`crate::classes::WorkerThreadPool::wait_for_group_task_completion`] at some point so that any allocated resources inside the task can be cleaned up."]
        #[inline]
        pub fn add_task_ex < 'ex > (&'ex mut self, action: &'ex Callable,) -> ExAddTask < 'ex > {
            ExAddTask::new(self, action,)
        }
        #[doc = "Returns `true` if the task with the given ID is completed.\n\n**Note:** You should only call this method between adding the task and awaiting its completion."]
        pub fn is_task_completed(&self, task_id: i64,) -> bool {
            type CallRet = bool;
            type CallParams = (i64,);
            let args = (task_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "is_task_completed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Pauses the thread that calls this method until the task with the given ID is completed.\n\nReturns `@GlobalScope.OK` if the task could be successfully awaited.\n\nReturns `@GlobalScope.ERR_INVALID_PARAMETER` if a task with the passed ID does not exist (maybe because it was already awaited and disposed of).\n\nReturns `@GlobalScope.ERR_BUSY` if the call is made from another running task and, due to task scheduling, there's potential for deadlocking (e.g., the task to await may be at a lower level in the call stack and therefore can't progress). This is an advanced situation that should only matter when some tasks depend on others (in the current implementation, the tricky case is a task trying to wait on an older one)."]
        pub fn wait_for_task_completion(&mut self, task_id: i64,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (i64,);
            let args = (task_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "wait_for_task_completion", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the task ID of the current thread calling this method, or `-1` if the task is a group task, invalid or the current thread is not part of the thread pool (e.g. the main thread).\n\nCan be used by a task to get its own task ID, or to determine whether the current code is running inside the worker thread pool.\n\n**Note:** Group tasks have their own IDs, so this method will return `-1` for group tasks."]
        pub fn get_caller_task_id(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "get_caller_task_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds `action` as a group task to be executed by the worker threads. The [`Callable`][crate::builtin::Callable] will be called a number of times based on `elements`, with the first thread calling it with the value `0` as a parameter, and each consecutive execution incrementing this value by 1 until it reaches `element - 1`.\n\nThe number of threads the task is distributed to is defined by `tasks_needed`, where the default value `-1` means it is distributed to all worker threads. `high_priority` determines if the task has a high priority or a low priority (default). You can optionally provide a `description` to help with debugging.\n\nReturns a group task ID that can be used by other methods.\n\n**Warning:** Every task must be waited for completion using [`wait_for_task_completion`][`crate::classes::WorkerThreadPool::wait_for_task_completion`] or [`wait_for_group_task_completion`][`crate::classes::WorkerThreadPool::wait_for_group_task_completion`] at some point so that any allocated resources inside the task can be cleaned up."]
        pub(crate) fn add_group_task_full(&mut self, action: RefArg < Callable >, elements: i32, tasks_needed: i32, high_priority: bool, description: CowArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Callable >, i32, i32, bool, CowArg < 'a1, GString >,);
            let args = (action, elements, tasks_needed, high_priority, description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "add_group_task", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_group_task_ex`][Self::add_group_task_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds `action` as a group task to be executed by the worker threads. The [`Callable`][crate::builtin::Callable] will be called a number of times based on `elements`, with the first thread calling it with the value `0` as a parameter, and each consecutive execution incrementing this value by 1 until it reaches `element - 1`.\n\nThe number of threads the task is distributed to is defined by `tasks_needed`, where the default value `-1` means it is distributed to all worker threads. `high_priority` determines if the task has a high priority or a low priority (default). You can optionally provide a `description` to help with debugging.\n\nReturns a group task ID that can be used by other methods.\n\n**Warning:** Every task must be waited for completion using [`wait_for_task_completion`][`crate::classes::WorkerThreadPool::wait_for_task_completion`] or [`wait_for_group_task_completion`][`crate::classes::WorkerThreadPool::wait_for_group_task_completion`] at some point so that any allocated resources inside the task can be cleaned up."]
        #[inline]
        pub fn add_group_task(&mut self, action: &Callable, elements: i32,) -> i64 {
            self.add_group_task_ex(action, elements,) . done()
        }
        #[doc = "Adds `action` as a group task to be executed by the worker threads. The [`Callable`][crate::builtin::Callable] will be called a number of times based on `elements`, with the first thread calling it with the value `0` as a parameter, and each consecutive execution incrementing this value by 1 until it reaches `element - 1`.\n\nThe number of threads the task is distributed to is defined by `tasks_needed`, where the default value `-1` means it is distributed to all worker threads. `high_priority` determines if the task has a high priority or a low priority (default). You can optionally provide a `description` to help with debugging.\n\nReturns a group task ID that can be used by other methods.\n\n**Warning:** Every task must be waited for completion using [`wait_for_task_completion`][`crate::classes::WorkerThreadPool::wait_for_task_completion`] or [`wait_for_group_task_completion`][`crate::classes::WorkerThreadPool::wait_for_group_task_completion`] at some point so that any allocated resources inside the task can be cleaned up."]
        #[inline]
        pub fn add_group_task_ex < 'ex > (&'ex mut self, action: &'ex Callable, elements: i32,) -> ExAddGroupTask < 'ex > {
            ExAddGroupTask::new(self, action, elements,)
        }
        #[doc = "Returns `true` if the group task with the given ID is completed.\n\n**Note:** You should only call this method between adding the group task and awaiting its completion."]
        pub fn is_group_task_completed(&self, group_id: i64,) -> bool {
            type CallRet = bool;
            type CallParams = (i64,);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "is_group_task_completed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns how many times the [`Callable`][crate::builtin::Callable] of the group task with the given ID has already been executed by the worker threads.\n\n**Note:** If a thread has started executing the [`Callable`][crate::builtin::Callable] but is yet to finish, it won't be counted."]
        pub fn get_group_processed_element_count(&self, group_id: i64,) -> u32 {
            type CallRet = u32;
            type CallParams = (i64,);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "get_group_processed_element_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Pauses the thread that calls this method until the group task with the given ID is completed."]
        pub fn wait_for_group_task_completion(&mut self, group_id: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "wait_for_group_task_completion", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the task group ID of the current thread calling this method, or `-1` if invalid or the current thread is not part of a task group."]
        pub fn get_caller_group_id(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "get_caller_group_id", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for WorkerThreadPool {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("WorkerThreadPool"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WorkerThreadPool {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WorkerThreadPool {
        
    }
    impl crate::obj::Singleton for WorkerThreadPool {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"WorkerThreadPool"))
            }
        }
    }
    impl std::ops::Deref for WorkerThreadPool {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WorkerThreadPool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_WorkerThreadPool__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `WorkerThreadPool` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`WorkerThreadPool::add_task_ex`][super::WorkerThreadPool::add_task_ex]."]
#[must_use]
pub struct ExAddTask < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::WorkerThreadPool, action: CowArg < 'ex, Callable >, high_priority: bool, description: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddTask < 'ex > {
    fn new(surround_object: &'ex mut re_export::WorkerThreadPool, action: &'ex Callable,) -> Self {
        let high_priority = false;
        let description = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: CowArg::Borrowed(action), high_priority: high_priority, description: CowArg::Owned(description),
        }
    }
    #[inline]
    pub fn high_priority(self, high_priority: bool) -> Self {
        Self {
            high_priority: high_priority, .. self
        }
    }
    #[inline]
    pub fn description(self, description: impl AsArg < GString > + 'ex) -> Self {
        Self {
            description: description.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, action, high_priority, description,
        }
        = self;
        re_export::WorkerThreadPool::add_task_full(surround_object, action.cow_as_arg(), high_priority, description,)
    }
}
#[doc = "Default-param extender for [`WorkerThreadPool::add_group_task_ex`][super::WorkerThreadPool::add_group_task_ex]."]
#[must_use]
pub struct ExAddGroupTask < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::WorkerThreadPool, action: CowArg < 'ex, Callable >, elements: i32, tasks_needed: i32, high_priority: bool, description: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddGroupTask < 'ex > {
    fn new(surround_object: &'ex mut re_export::WorkerThreadPool, action: &'ex Callable, elements: i32,) -> Self {
        let tasks_needed = - 1i32;
        let high_priority = false;
        let description = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: CowArg::Borrowed(action), elements: elements, tasks_needed: tasks_needed, high_priority: high_priority, description: CowArg::Owned(description),
        }
    }
    #[inline]
    pub fn tasks_needed(self, tasks_needed: i32) -> Self {
        Self {
            tasks_needed: tasks_needed, .. self
        }
    }
    #[inline]
    pub fn high_priority(self, high_priority: bool) -> Self {
        Self {
            high_priority: high_priority, .. self
        }
    }
    #[inline]
    pub fn description(self, description: impl AsArg < GString > + 'ex) -> Self {
        Self {
            description: description.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, action, elements, tasks_needed, high_priority, description,
        }
        = self;
        re_export::WorkerThreadPool::add_group_task_full(surround_object, action.cow_as_arg(), elements, tasks_needed, high_priority, description,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::WorkerThreadPool;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for WorkerThreadPool {
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
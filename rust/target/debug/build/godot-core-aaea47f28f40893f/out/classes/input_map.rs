#![doc = "Sidecar module for class [`InputMap`][crate::classes::InputMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputMap` enums](https://docs.godotengine.org/en/stable/classes/class_inputmap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputMap`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`input_map`][crate::classes::input_map]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `InputMap`](https://docs.godotengine.org/en/stable/classes/class_inputmap.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nManages all [`InputEventAction`][crate::classes::InputEventAction] which can be created/modified from the project settings menu **Project > Project Settings > Input Map** or in code with [`add_action`][`crate::classes::InputMap::add_action`] and [`action_add_event`][`crate::classes::InputMap::action_add_event`]. See [`input`][`crate::classes::INode::input`]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl InputMap {
        #[doc = "Returns `true` if the `InputMap` has a registered action with the given name."]
        pub fn has_action(&self, action: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "has_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all actions in the `InputMap`."]
        pub fn get_actions(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "get_actions", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an empty action to the `InputMap` with a configurable `deadzone`.\n\nAn [`InputEvent`][crate::classes::InputEvent] can then be added to this action with [`action_add_event`][`crate::classes::InputMap::action_add_event`]."]
        pub(crate) fn add_action_full(&mut self, action: CowArg < StringName >, deadzone: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f32,);
            let args = (action, deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "add_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_action_ex`][Self::add_action_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an empty action to the `InputMap` with a configurable `deadzone`.\n\nAn [`InputEvent`][crate::classes::InputEvent] can then be added to this action with [`action_add_event`][`crate::classes::InputMap::action_add_event`]."]
        #[inline]
        pub fn add_action(&mut self, action: impl AsArg < StringName >,) {
            self.add_action_ex(action,) . done()
        }
        #[doc = "Adds an empty action to the `InputMap` with a configurable `deadzone`.\n\nAn [`InputEvent`][crate::classes::InputEvent] can then be added to this action with [`action_add_event`][`crate::classes::InputMap::action_add_event`]."]
        #[inline]
        pub fn add_action_ex < 'ex > (&'ex mut self, action: impl AsArg < StringName > + 'ex,) -> ExAddAction < 'ex > {
            ExAddAction::new(self, action,)
        }
        #[doc = "Removes an action from the `InputMap`."]
        pub fn erase_action(&mut self, action: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "erase_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the human-readable description of the given action."]
        pub fn get_action_description(&self, action: impl AsArg < StringName >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "get_action_description", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a deadzone value for the action."]
        pub fn action_set_deadzone(&mut self, action: impl AsArg < StringName >, deadzone: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f32,);
            let args = (action.into_arg(), deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9652usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_set_deadzone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a deadzone value for the action."]
        pub fn action_get_deadzone(&mut self, action: impl AsArg < StringName >,) -> f32 {
            type CallRet = f32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9653usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_get_deadzone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an [`InputEvent`][crate::classes::InputEvent] to an action. This [`InputEvent`][crate::classes::InputEvent] will trigger the action."]
        pub fn action_add_event(&mut self, action: impl AsArg < StringName >, event: impl AsArg < Gd < crate::classes::InputEvent >>,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Gd < crate::classes::InputEvent > >,);
            let args = (action.into_arg(), event.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9654usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_add_event", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the action has the given [`InputEvent`][crate::classes::InputEvent] associated with it."]
        pub fn action_has_event(&mut self, action: impl AsArg < StringName >, event: impl AsArg < Gd < crate::classes::InputEvent >>,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Gd < crate::classes::InputEvent > >,);
            let args = (action.into_arg(), event.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9655usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_has_event", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes an [`InputEvent`][crate::classes::InputEvent] from an action."]
        pub fn action_erase_event(&mut self, action: impl AsArg < StringName >, event: impl AsArg < Gd < crate::classes::InputEvent >>,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Gd < crate::classes::InputEvent > >,);
            let args = (action.into_arg(), event.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9656usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_erase_event", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all events from an action."]
        pub fn action_erase_events(&mut self, action: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9657usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_erase_events", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of [`InputEvent`][crate::classes::InputEvent]s associated with a given action.\n\n**Note:** When used in the editor (e.g. a tool script or [`EditorPlugin`][crate::classes::EditorPlugin]), this method will return events for the editor action. If you want to access your project's input binds from the editor, read the `input/*` settings from [`ProjectSettings`][crate::classes::ProjectSettings]."]
        pub fn action_get_events(&mut self, action: impl AsArg < StringName >,) -> Array < Gd < crate::classes::InputEvent > > {
            type CallRet = Array < Gd < crate::classes::InputEvent > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9658usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_get_events", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given event is part of an existing action. This method ignores keyboard modifiers if the given [`InputEvent`][crate::classes::InputEvent] is not pressed (for proper release detection). See [`action_has_event`][`crate::classes::InputMap::action_has_event`] if you don't want this behavior.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        pub(crate) fn event_is_action_full(&self, event: CowArg < Gd < crate::classes::InputEvent > >, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Gd < crate::classes::InputEvent > >, CowArg < 'a1, StringName >, bool,);
            let args = (event, action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "event_is_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`event_is_action_ex`][Self::event_is_action_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the given event is part of an existing action. This method ignores keyboard modifiers if the given [`InputEvent`][crate::classes::InputEvent] is not pressed (for proper release detection). See [`action_has_event`][`crate::classes::InputMap::action_has_event`] if you don't want this behavior.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn event_is_action(&self, event: impl AsArg < Gd < crate::classes::InputEvent >>, action: impl AsArg < StringName >,) -> bool {
            self.event_is_action_ex(event, action,) . done()
        }
        #[doc = "Returns `true` if the given event is part of an existing action. This method ignores keyboard modifiers if the given [`InputEvent`][crate::classes::InputEvent] is not pressed (for proper release detection). See [`action_has_event`][`crate::classes::InputMap::action_has_event`] if you don't want this behavior.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn event_is_action_ex < 'ex > (&'ex self, event: impl AsArg < Gd < crate::classes::InputEvent >> + 'ex, action: impl AsArg < StringName > + 'ex,) -> ExEventIsAction < 'ex > {
            ExEventIsAction::new(self, event, action,)
        }
        #[doc = "Clears all [`InputEventAction`][crate::classes::InputEventAction] in the `InputMap` and load it anew from [`ProjectSettings`][crate::classes::ProjectSettings]."]
        pub fn load_from_project_settings(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "load_from_project_settings", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for InputMap {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("InputMap"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputMap {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputMap {
        
    }
    impl crate::obj::Singleton for InputMap {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"InputMap"))
            }
        }
    }
    impl std::ops::Deref for InputMap {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_InputMap__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `InputMap` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`InputMap::add_action_ex`][super::InputMap::add_action_ex]."]
#[must_use]
pub struct ExAddAction < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::InputMap, action: CowArg < 'ex, StringName >, deadzone: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddAction < 'ex > {
    fn new(surround_object: &'ex mut re_export::InputMap, action: impl AsArg < StringName > + 'ex,) -> Self {
        let deadzone = 0.2f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), deadzone: deadzone,
        }
    }
    #[inline]
    pub fn deadzone(self, deadzone: f32) -> Self {
        Self {
            deadzone: deadzone, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, action, deadzone,
        }
        = self;
        re_export::InputMap::add_action_full(surround_object, action, deadzone,)
    }
}
#[doc = "Default-param extender for [`InputMap::event_is_action_ex`][super::InputMap::event_is_action_ex]."]
#[must_use]
pub struct ExEventIsAction < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::InputMap, event: CowArg < 'ex, Gd < crate::classes::InputEvent > >, action: CowArg < 'ex, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExEventIsAction < 'ex > {
    fn new(surround_object: &'ex re_export::InputMap, event: impl AsArg < Gd < crate::classes::InputEvent >> + 'ex, action: impl AsArg < StringName > + 'ex,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, event: event.into_arg(), action: action.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, event, action, exact_match,
        }
        = self;
        re_export::InputMap::event_is_action_full(surround_object, event, action, exact_match,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::InputMap;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for InputMap {
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
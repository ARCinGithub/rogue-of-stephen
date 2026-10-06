#![doc = "Sidecar module for class [`InputEvent`][crate::classes::InputEvent].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEvent` enums](https://docs.godotengine.org/en/stable/classes/class_inputevent.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEvent`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`input_event`][crate::classes::input_event]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `InputEvent`](https://docs.godotengine.org/en/stable/classes/class_inputevent.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<InputEvent>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nAbstract base class of all types of input events. See [`input`][`crate::classes::INode::input`]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEvent {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl InputEvent {
        pub fn set_device(&mut self, device: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11519usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "set_device", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_device(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11520usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "get_device", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this input event matches a pre-defined action of any type.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        pub(crate) fn is_action_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11521usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_action_ex`][Self::is_action_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if this input event matches a pre-defined action of any type.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn is_action(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_ex(action,) . done()
        }
        #[doc = "Returns `true` if this input event matches a pre-defined action of any type.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn is_action_ex < 'ex > (&'ex self, action: impl AsArg < StringName > + 'ex,) -> ExIsAction < 'ex > {
            ExIsAction::new(self, action,)
        }
        #[doc = "Returns `true` if the given action matches this event and is being pressed (and is not an echo event for [`InputEventKey`][crate::classes::InputEventKey] events, unless `allow_echo` is `true`). Not relevant for events of type [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] or [`InputEventScreenDrag`][crate::classes::InputEventScreenDrag].\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** Due to keyboard ghosting, [`is_action_pressed`][`crate::classes::InputEvent::is_action_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information."]
        pub(crate) fn is_action_pressed_full(&self, action: CowArg < StringName >, allow_echo: bool, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool, bool,);
            let args = (action, allow_echo, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11522usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_action_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_action_pressed_ex`][Self::is_action_pressed_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the given action matches this event and is being pressed (and is not an echo event for [`InputEventKey`][crate::classes::InputEventKey] events, unless `allow_echo` is `true`). Not relevant for events of type [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] or [`InputEventScreenDrag`][crate::classes::InputEventScreenDrag].\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** Due to keyboard ghosting, [`is_action_pressed`][`crate::classes::InputEvent::is_action_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information."]
        #[inline]
        pub fn is_action_pressed(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_pressed_ex(action,) . done()
        }
        #[doc = "Returns `true` if the given action matches this event and is being pressed (and is not an echo event for [`InputEventKey`][crate::classes::InputEventKey] events, unless `allow_echo` is `true`). Not relevant for events of type [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] or [`InputEventScreenDrag`][crate::classes::InputEventScreenDrag].\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** Due to keyboard ghosting, [`is_action_pressed`][`crate::classes::InputEvent::is_action_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information."]
        #[inline]
        pub fn is_action_pressed_ex < 'ex > (&'ex self, action: impl AsArg < StringName > + 'ex,) -> ExIsActionPressed < 'ex > {
            ExIsActionPressed::new(self, action,)
        }
        #[doc = "Returns `true` if the given action matches this event and is released (i.e. not pressed). Not relevant for events of type [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] or [`InputEventScreenDrag`][crate::classes::InputEventScreenDrag].\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        pub(crate) fn is_action_released_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11523usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_action_released", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_action_released_ex`][Self::is_action_released_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the given action matches this event and is released (i.e. not pressed). Not relevant for events of type [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] or [`InputEventScreenDrag`][crate::classes::InputEventScreenDrag].\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn is_action_released(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_released_ex(action,) . done()
        }
        #[doc = "Returns `true` if the given action matches this event and is released (i.e. not pressed). Not relevant for events of type [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] or [`InputEventScreenDrag`][crate::classes::InputEventScreenDrag].\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn is_action_released_ex < 'ex > (&'ex self, action: impl AsArg < StringName > + 'ex,) -> ExIsActionReleased < 'ex > {
            ExIsActionReleased::new(self, action,)
        }
        #[doc = "Returns a value between 0.0 and 1.0 depending on the given actions' state. Useful for getting the value of events of type [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion].\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        pub(crate) fn get_action_strength_full(&self, action: CowArg < StringName >, exact_match: bool,) -> f32 {
            type CallRet = f32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11524usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "get_action_strength", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_action_strength_ex`][Self::get_action_strength_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a value between 0.0 and 1.0 depending on the given actions' state. Useful for getting the value of events of type [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion].\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn get_action_strength(&self, action: impl AsArg < StringName >,) -> f32 {
            self.get_action_strength_ex(action,) . done()
        }
        #[doc = "Returns a value between 0.0 and 1.0 depending on the given actions' state. Useful for getting the value of events of type [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion].\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn get_action_strength_ex < 'ex > (&'ex self, action: impl AsArg < StringName > + 'ex,) -> ExGetActionStrength < 'ex > {
            ExGetActionStrength::new(self, action,)
        }
        #[doc = "Returns `true` if this input event has been canceled."]
        pub fn is_canceled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11525usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_canceled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this input event is pressed. Not relevant for events of type [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] or [`InputEventScreenDrag`][crate::classes::InputEventScreenDrag].\n\n**Note:** Due to keyboard ghosting, [`is_pressed`][`crate::classes::InputEvent::is_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information."]
        pub fn is_pressed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11526usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this input event is released. Not relevant for events of type [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] or [`InputEventScreenDrag`][crate::classes::InputEventScreenDrag]."]
        pub fn is_released(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11527usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_released", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this input event is an echo event (only for events of type [`InputEventKey`][crate::classes::InputEventKey]). An echo event is a repeated key event sent when the user is holding down the key. Any other event type returns `false`.\n\n**Note:** The rate at which echo events are sent is typically around 20 events per second (after holding down the key for roughly half a second). However, the key repeat delay/speed can be changed by the user or disabled entirely in the operating system settings. To ensure your project works correctly on all configurations, do not assume the user has a specific key repeat configuration in your project's behavior."]
        pub fn is_echo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11528usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_echo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`String`][crate::builtin::GString] representation of the event."]
        pub fn as_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11529usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "as_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the specified `event` matches this event. Only valid for action events, which include key ([`InputEventKey`][crate::classes::InputEventKey]), button ([`InputEventMouseButton`][crate::classes::InputEventMouseButton] or [`InputEventJoypadButton`][crate::classes::InputEventJoypadButton]), axis [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion], and action ([`InputEventAction`][crate::classes::InputEventAction]) events.\n\nIf `exact_match` is `false`, the check ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** This method only considers the event configuration (such as the keyboard key or the joypad axis), not state information like [`is_pressed`][`crate::classes::InputEvent::is_pressed`], [`is_released`][`crate::classes::InputEvent::is_released`], [`is_echo`][`crate::classes::InputEvent::is_echo`], or [`is_canceled`][`crate::classes::InputEvent::is_canceled`]."]
        pub(crate) fn is_match_full(&self, event: CowArg < Option < Gd < crate::classes::InputEvent > > >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::InputEvent > > >, bool,);
            let args = (event, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11530usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_match", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_match_ex`][Self::is_match_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the specified `event` matches this event. Only valid for action events, which include key ([`InputEventKey`][crate::classes::InputEventKey]), button ([`InputEventMouseButton`][crate::classes::InputEventMouseButton] or [`InputEventJoypadButton`][crate::classes::InputEventJoypadButton]), axis [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion], and action ([`InputEventAction`][crate::classes::InputEventAction]) events.\n\nIf `exact_match` is `false`, the check ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** This method only considers the event configuration (such as the keyboard key or the joypad axis), not state information like [`is_pressed`][`crate::classes::InputEvent::is_pressed`], [`is_released`][`crate::classes::InputEvent::is_released`], [`is_echo`][`crate::classes::InputEvent::is_echo`], or [`is_canceled`][`crate::classes::InputEvent::is_canceled`]."]
        #[inline]
        pub fn is_match(&self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) -> bool {
            self.is_match_ex(event,) . done()
        }
        #[doc = "Returns `true` if the specified `event` matches this event. Only valid for action events, which include key ([`InputEventKey`][crate::classes::InputEventKey]), button ([`InputEventMouseButton`][crate::classes::InputEventMouseButton] or [`InputEventJoypadButton`][crate::classes::InputEventJoypadButton]), axis [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion], and action ([`InputEventAction`][crate::classes::InputEventAction]) events.\n\nIf `exact_match` is `false`, the check ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** This method only considers the event configuration (such as the keyboard key or the joypad axis), not state information like [`is_pressed`][`crate::classes::InputEvent::is_pressed`], [`is_released`][`crate::classes::InputEvent::is_released`], [`is_echo`][`crate::classes::InputEvent::is_echo`], or [`is_canceled`][`crate::classes::InputEvent::is_canceled`]."]
        #[inline]
        pub fn is_match_ex < 'ex > (&'ex self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'ex,) -> ExIsMatch < 'ex > {
            ExIsMatch::new(self, event,)
        }
        #[doc = "Returns `true` if this input event's type is one that can be assigned to an input action: [`InputEventKey`][crate::classes::InputEventKey], [`InputEventMouseButton`][crate::classes::InputEventMouseButton], [`InputEventJoypadButton`][crate::classes::InputEventJoypadButton], [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion], [`InputEventAction`][crate::classes::InputEventAction]. Returns `false` for all other input event types."]
        pub fn is_action_type(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11531usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_action_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given input event and this input event can be added together (only for events of type [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion]).\n\nThe given input event's position, global position and speed will be copied. The resulting `relative` is a sum of both events. Both events' modifiers have to be identical."]
        pub fn accumulate(&mut self, with_event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::InputEvent > > >,);
            let args = (with_event.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11532usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "accumulate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a copy of the given input event which has been offset by `local_ofs` and transformed by `xform`. Relevant for events of type [`InputEventMouseButton`][crate::classes::InputEventMouseButton], [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion], [`InputEventScreenTouch`][crate::classes::InputEventScreenTouch], [`InputEventScreenDrag`][crate::classes::InputEventScreenDrag], [`InputEventMagnifyGesture`][crate::classes::InputEventMagnifyGesture] and [`InputEventPanGesture`][crate::classes::InputEventPanGesture]."]
        pub(crate) fn xformed_by_full(&self, xform: Transform2D, local_ofs: Vector2,) -> Gd < crate::classes::InputEvent > {
            type CallRet = Gd < crate::classes::InputEvent >;
            type CallParams = (Transform2D, Vector2,);
            let args = (xform, local_ofs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11533usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "xformed_by", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`xformed_by_ex`][Self::xformed_by_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a copy of the given input event which has been offset by `local_ofs` and transformed by `xform`. Relevant for events of type [`InputEventMouseButton`][crate::classes::InputEventMouseButton], [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion], [`InputEventScreenTouch`][crate::classes::InputEventScreenTouch], [`InputEventScreenDrag`][crate::classes::InputEventScreenDrag], [`InputEventMagnifyGesture`][crate::classes::InputEventMagnifyGesture] and [`InputEventPanGesture`][crate::classes::InputEventPanGesture]."]
        #[inline]
        pub fn xformed_by(&self, xform: Transform2D,) -> Gd < crate::classes::InputEvent > {
            self.xformed_by_ex(xform,) . done()
        }
        #[doc = "Returns a copy of the given input event which has been offset by `local_ofs` and transformed by `xform`. Relevant for events of type [`InputEventMouseButton`][crate::classes::InputEventMouseButton], [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion], [`InputEventScreenTouch`][crate::classes::InputEventScreenTouch], [`InputEventScreenDrag`][crate::classes::InputEventScreenDrag], [`InputEventMagnifyGesture`][crate::classes::InputEventMagnifyGesture] and [`InputEventPanGesture`][crate::classes::InputEventPanGesture]."]
        #[inline]
        pub fn xformed_by_ex < 'ex > (&'ex self, xform: Transform2D,) -> ExXformedBy < 'ex > {
            ExXformedBy::new(self, xform,)
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
        pub const DEVICE_ID_EMULATION: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for InputEvent {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("InputEvent"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEvent {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for InputEvent {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for InputEvent {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputEvent {
        
    }
    impl std::ops::Deref for InputEvent {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEvent {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_InputEvent__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `InputEvent` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`InputEvent::is_action_ex`][super::InputEvent::is_action_ex]."]
#[must_use]
pub struct ExIsAction < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::InputEvent, action: CowArg < 'ex, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsAction < 'ex > {
    fn new(surround_object: &'ex re_export::InputEvent, action: impl AsArg < StringName > + 'ex,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
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
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::InputEvent::is_action_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::is_action_pressed_ex`][super::InputEvent::is_action_pressed_ex]."]
#[must_use]
pub struct ExIsActionPressed < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::InputEvent, action: CowArg < 'ex, StringName >, allow_echo: bool, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsActionPressed < 'ex > {
    fn new(surround_object: &'ex re_export::InputEvent, action: impl AsArg < StringName > + 'ex,) -> Self {
        let allow_echo = false;
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), allow_echo: allow_echo, exact_match: exact_match,
        }
    }
    #[inline]
    pub fn allow_echo(self, allow_echo: bool) -> Self {
        Self {
            allow_echo: allow_echo, .. self
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
            _phantom, surround_object, action, allow_echo, exact_match,
        }
        = self;
        re_export::InputEvent::is_action_pressed_full(surround_object, action, allow_echo, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::is_action_released_ex`][super::InputEvent::is_action_released_ex]."]
#[must_use]
pub struct ExIsActionReleased < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::InputEvent, action: CowArg < 'ex, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsActionReleased < 'ex > {
    fn new(surround_object: &'ex re_export::InputEvent, action: impl AsArg < StringName > + 'ex,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
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
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::InputEvent::is_action_released_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::get_action_strength_ex`][super::InputEvent::get_action_strength_ex]."]
#[must_use]
pub struct ExGetActionStrength < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::InputEvent, action: CowArg < 'ex, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetActionStrength < 'ex > {
    fn new(surround_object: &'ex re_export::InputEvent, action: impl AsArg < StringName > + 'ex,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::InputEvent::get_action_strength_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::is_match_ex`][super::InputEvent::is_match_ex]."]
#[must_use]
pub struct ExIsMatch < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::InputEvent, event: CowArg < 'ex, Option < Gd < crate::classes::InputEvent > > >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsMatch < 'ex > {
    fn new(surround_object: &'ex re_export::InputEvent, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'ex,) -> Self {
        let exact_match = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, event: event.into_arg(), exact_match: exact_match,
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
            _phantom, surround_object, event, exact_match,
        }
        = self;
        re_export::InputEvent::is_match_full(surround_object, event, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::xformed_by_ex`][super::InputEvent::xformed_by_ex]."]
#[must_use]
pub struct ExXformedBy < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::InputEvent, xform: Transform2D, local_ofs: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExXformedBy < 'ex > {
    fn new(surround_object: &'ex re_export::InputEvent, xform: Transform2D,) -> Self {
        let local_ofs = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, xform: xform, local_ofs: local_ofs,
        }
    }
    #[inline]
    pub fn local_ofs(self, local_ofs: Vector2) -> Self {
        Self {
            local_ofs: local_ofs, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Gd < crate::classes::InputEvent > {
        let Self {
            _phantom, surround_object, xform, local_ofs,
        }
        = self;
        re_export::InputEvent::xformed_by_full(surround_object, xform, local_ofs,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::InputEvent;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for InputEvent {
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
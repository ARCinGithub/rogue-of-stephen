#![doc = "Sidecar module for class [`Input`][crate::classes::Input].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Input` enums](https://docs.godotengine.org/en/stable/classes/class_input.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Input`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`input`][crate::classes::input]: sidecar module with related enum/flag types\n* [`SignalsOfInput`][crate::classes::input::SignalsOfInput]: signal collection\n\n\nSee also [Godot docs for `Input`](https://docs.godotengine.org/en/stable/classes/class_input.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThe `Input` singleton handles key presses, mouse buttons and movement, gamepads, and input actions. Actions and their events can be set in the **Input Map** tab in **Project > Project Settings**, or with the [`InputMap`][crate::classes::InputMap] class.\n\n**Note:** `Input`'s methods reflect the global input state and are not affected by [`accept_event`][`crate::classes::Control::accept_event`] or [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`], as those methods only deal with the way input is propagated in the [`SceneTree`][crate::classes::SceneTree]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Input {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Input {
        #[doc = "Returns `true` if any action, key, joypad button, or mouse button is being pressed. This will also return `true` if any action is simulated via code by calling [`action_press`][`crate::classes::Input::action_press`]."]
        pub fn is_anything_pressed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_anything_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if you are pressing the Latin key in the current keyboard layout. You can pass a \\[enum Key] constant.\n\n[`is_key_pressed`][`crate::classes::Input::is_key_pressed`] is only recommended over [`is_physical_key_pressed`][`crate::classes::Input::is_physical_key_pressed`] in non-game applications. This ensures that shortcut keys behave as expected depending on the user's keyboard layout, as keyboard shortcuts are generally dependent on the keyboard layout in non-game applications. If in doubt, use [`is_physical_key_pressed`][`crate::classes::Input::is_physical_key_pressed`].\n\n**Note:** Due to keyboard ghosting, [`is_key_pressed`][`crate::classes::Input::is_key_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information."]
        pub fn is_key_pressed(&self, keycode: crate::global::Key,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::global::Key,);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_key_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if you are pressing the key in the physical location on the 101/102-key US QWERTY keyboard. You can pass a \\[enum Key] constant.\n\n[`is_physical_key_pressed`][`crate::classes::Input::is_physical_key_pressed`] is recommended over [`is_key_pressed`][`crate::classes::Input::is_key_pressed`] for in-game actions, as it will make `W`/`A`/`S`/`D` layouts work regardless of the user's keyboard layout. [`is_physical_key_pressed`][`crate::classes::Input::is_physical_key_pressed`] will also ensure that the top row number keys work on any keyboard layout. If in doubt, use [`is_physical_key_pressed`][`crate::classes::Input::is_physical_key_pressed`].\n\n**Note:** Due to keyboard ghosting, [`is_physical_key_pressed`][`crate::classes::Input::is_physical_key_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information."]
        pub fn is_physical_key_pressed(&self, keycode: crate::global::Key,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::global::Key,);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_physical_key_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if you are pressing the key with the `keycode` printed on it. You can pass a \\[enum Key] constant or any Unicode character code."]
        pub fn is_key_label_pressed(&self, keycode: crate::global::Key,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::global::Key,);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_key_label_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if you are pressing the mouse button specified with \\[enum MouseButton]."]
        pub fn is_mouse_button_pressed(&self, button: crate::global::MouseButton,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::global::MouseButton,);
            let args = (button,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_mouse_button_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if you are pressing the joypad button at index `button`."]
        pub fn is_joy_button_pressed(&self, device: i32, button: crate::global::JoyButton,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, crate::global::JoyButton,);
            let args = (device, button,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_joy_button_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if you are pressing the action event.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** Due to keyboard ghosting, [`is_action_pressed`][`crate::classes::Input::is_action_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information."]
        pub(crate) fn is_action_pressed_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_action_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_action_pressed_ex`][Self::is_action_pressed_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if you are pressing the action event.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** Due to keyboard ghosting, [`is_action_pressed`][`crate::classes::Input::is_action_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information."]
        #[inline]
        pub fn is_action_pressed(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_pressed_ex(action,) . done()
        }
        #[doc = "Returns `true` if you are pressing the action event.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** Due to keyboard ghosting, [`is_action_pressed`][`crate::classes::Input::is_action_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information."]
        #[inline]
        pub fn is_action_pressed_ex < 'ex > (&'ex self, action: impl AsArg < StringName > + 'ex,) -> ExIsActionPressed < 'ex > {
            ExIsActionPressed::new(self, action,)
        }
        #[doc = "Returns `true` when the user has _started_ pressing the action event in the current frame or physics tick. It will only return `true` on the frame or tick that the user pressed down the button.\n\nThis is useful for code that needs to run only once when an action is pressed, instead of every frame while it's pressed.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** Returning `true` does not imply that the action is _still_ pressed. An action can be pressed and released again rapidly, and `true` will still be returned so as not to miss input.\n\n**Note:** Due to keyboard ghosting, [`is_action_just_pressed`][`crate::classes::Input::is_action_just_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information.\n\n**Note:** During input handling (e.g. [`input`][`crate::classes::INode::input`]), use [`is_action_pressed`][`crate::classes::InputEvent::is_action_pressed`] instead to query the action state of the current event. See also [`is_action_just_pressed_by_event`][`crate::classes::Input::is_action_just_pressed_by_event`]."]
        pub(crate) fn is_action_just_pressed_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_action_just_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_action_just_pressed_ex`][Self::is_action_just_pressed_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` when the user has _started_ pressing the action event in the current frame or physics tick. It will only return `true` on the frame or tick that the user pressed down the button.\n\nThis is useful for code that needs to run only once when an action is pressed, instead of every frame while it's pressed.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** Returning `true` does not imply that the action is _still_ pressed. An action can be pressed and released again rapidly, and `true` will still be returned so as not to miss input.\n\n**Note:** Due to keyboard ghosting, [`is_action_just_pressed`][`crate::classes::Input::is_action_just_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information.\n\n**Note:** During input handling (e.g. [`input`][`crate::classes::INode::input`]), use [`is_action_pressed`][`crate::classes::InputEvent::is_action_pressed`] instead to query the action state of the current event. See also [`is_action_just_pressed_by_event`][`crate::classes::Input::is_action_just_pressed_by_event`]."]
        #[inline]
        pub fn is_action_just_pressed(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_just_pressed_ex(action,) . done()
        }
        #[doc = "Returns `true` when the user has _started_ pressing the action event in the current frame or physics tick. It will only return `true` on the frame or tick that the user pressed down the button.\n\nThis is useful for code that needs to run only once when an action is pressed, instead of every frame while it's pressed.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** Returning `true` does not imply that the action is _still_ pressed. An action can be pressed and released again rapidly, and `true` will still be returned so as not to miss input.\n\n**Note:** Due to keyboard ghosting, [`is_action_just_pressed`][`crate::classes::Input::is_action_just_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information.\n\n**Note:** During input handling (e.g. [`input`][`crate::classes::INode::input`]), use [`is_action_pressed`][`crate::classes::InputEvent::is_action_pressed`] instead to query the action state of the current event. See also [`is_action_just_pressed_by_event`][`crate::classes::Input::is_action_just_pressed_by_event`]."]
        #[inline]
        pub fn is_action_just_pressed_ex < 'ex > (&'ex self, action: impl AsArg < StringName > + 'ex,) -> ExIsActionJustPressed < 'ex > {
            ExIsActionJustPressed::new(self, action,)
        }
        #[doc = "Returns `true` when the user _stops_ pressing the action event in the current frame or physics tick. It will only return `true` on the frame or tick that the user releases the button.\n\n**Note:** Returning `true` does not imply that the action is _still_ not pressed. An action can be released and pressed again rapidly, and `true` will still be returned so as not to miss input.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** During input handling (e.g. [`input`][`crate::classes::INode::input`]), use [`is_action_released`][`crate::classes::InputEvent::is_action_released`] instead to query the action state of the current event. See also [`is_action_just_released_by_event`][`crate::classes::Input::is_action_just_released_by_event`]."]
        pub(crate) fn is_action_just_released_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_action_just_released", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_action_just_released_ex`][Self::is_action_just_released_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` when the user _stops_ pressing the action event in the current frame or physics tick. It will only return `true` on the frame or tick that the user releases the button.\n\n**Note:** Returning `true` does not imply that the action is _still_ not pressed. An action can be released and pressed again rapidly, and `true` will still be returned so as not to miss input.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** During input handling (e.g. [`input`][`crate::classes::INode::input`]), use [`is_action_released`][`crate::classes::InputEvent::is_action_released`] instead to query the action state of the current event. See also [`is_action_just_released_by_event`][`crate::classes::Input::is_action_just_released_by_event`]."]
        #[inline]
        pub fn is_action_just_released(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_just_released_ex(action,) . done()
        }
        #[doc = "Returns `true` when the user _stops_ pressing the action event in the current frame or physics tick. It will only return `true` on the frame or tick that the user releases the button.\n\n**Note:** Returning `true` does not imply that the action is _still_ not pressed. An action can be released and pressed again rapidly, and `true` will still be returned so as not to miss input.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** During input handling (e.g. [`input`][`crate::classes::INode::input`]), use [`is_action_released`][`crate::classes::InputEvent::is_action_released`] instead to query the action state of the current event. See also [`is_action_just_released_by_event`][`crate::classes::Input::is_action_just_released_by_event`]."]
        #[inline]
        pub fn is_action_just_released_ex < 'ex > (&'ex self, action: impl AsArg < StringName > + 'ex,) -> ExIsActionJustReleased < 'ex > {
            ExIsActionJustReleased::new(self, action,)
        }
        #[doc = "Returns `true` when the user has _started_ pressing the action event in the current frame or physics tick, and the first event that triggered action press in the current frame/physics tick was `event`. It will only return `true` on the frame or tick that the user pressed down the button.\n\nThis is useful for code that needs to run only once when an action is pressed, and the action is processed during input handling (e.g. [`input`][`crate::classes::INode::input`]).\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** Returning `true` does not imply that the action is _still_ pressed. An action can be pressed and released again rapidly, and `true` will still be returned so as not to miss input.\n\n**Note:** Due to keyboard ghosting, [`is_action_just_pressed`][`crate::classes::Input::is_action_just_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information."]
        pub(crate) fn is_action_just_pressed_by_event_full(&self, action: CowArg < StringName >, event: CowArg < Gd < crate::classes::InputEvent > >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Gd < crate::classes::InputEvent > >, bool,);
            let args = (action, event, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_action_just_pressed_by_event", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_action_just_pressed_by_event_ex`][Self::is_action_just_pressed_by_event_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` when the user has _started_ pressing the action event in the current frame or physics tick, and the first event that triggered action press in the current frame/physics tick was `event`. It will only return `true` on the frame or tick that the user pressed down the button.\n\nThis is useful for code that needs to run only once when an action is pressed, and the action is processed during input handling (e.g. [`input`][`crate::classes::INode::input`]).\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** Returning `true` does not imply that the action is _still_ pressed. An action can be pressed and released again rapidly, and `true` will still be returned so as not to miss input.\n\n**Note:** Due to keyboard ghosting, [`is_action_just_pressed`][`crate::classes::Input::is_action_just_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information."]
        #[inline]
        pub fn is_action_just_pressed_by_event(&self, action: impl AsArg < StringName >, event: impl AsArg < Gd < crate::classes::InputEvent >>,) -> bool {
            self.is_action_just_pressed_by_event_ex(action, event,) . done()
        }
        #[doc = "Returns `true` when the user has _started_ pressing the action event in the current frame or physics tick, and the first event that triggered action press in the current frame/physics tick was `event`. It will only return `true` on the frame or tick that the user pressed down the button.\n\nThis is useful for code that needs to run only once when an action is pressed, and the action is processed during input handling (e.g. [`input`][`crate::classes::INode::input`]).\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events.\n\n**Note:** Returning `true` does not imply that the action is _still_ pressed. An action can be pressed and released again rapidly, and `true` will still be returned so as not to miss input.\n\n**Note:** Due to keyboard ghosting, [`is_action_just_pressed`][`crate::classes::Input::is_action_just_pressed`] may return `false` even if one of the action's keys is pressed. See [Input examples]($DOCS_URL/tutorials/inputs/input_examples.html#keyboard-events) in the documentation for more information."]
        #[inline]
        pub fn is_action_just_pressed_by_event_ex < 'ex > (&'ex self, action: impl AsArg < StringName > + 'ex, event: impl AsArg < Gd < crate::classes::InputEvent >> + 'ex,) -> ExIsActionJustPressedByEvent < 'ex > {
            ExIsActionJustPressedByEvent::new(self, action, event,)
        }
        #[doc = "Returns `true` when the user _stops_ pressing the action event in the current frame or physics tick, and the first event that triggered action release in the current frame/physics tick was `event`. It will only return `true` on the frame or tick that the user releases the button.\n\nThis is useful when an action is processed during input handling (e.g. [`input`][`crate::classes::INode::input`]).\n\n**Note:** Returning `true` does not imply that the action is _still_ not pressed. An action can be released and pressed again rapidly, and `true` will still be returned so as not to miss input.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        pub(crate) fn is_action_just_released_by_event_full(&self, action: CowArg < StringName >, event: CowArg < Gd < crate::classes::InputEvent > >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Gd < crate::classes::InputEvent > >, bool,);
            let args = (action, event, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_action_just_released_by_event", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_action_just_released_by_event_ex`][Self::is_action_just_released_by_event_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` when the user _stops_ pressing the action event in the current frame or physics tick, and the first event that triggered action release in the current frame/physics tick was `event`. It will only return `true` on the frame or tick that the user releases the button.\n\nThis is useful when an action is processed during input handling (e.g. [`input`][`crate::classes::INode::input`]).\n\n**Note:** Returning `true` does not imply that the action is _still_ not pressed. An action can be released and pressed again rapidly, and `true` will still be returned so as not to miss input.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn is_action_just_released_by_event(&self, action: impl AsArg < StringName >, event: impl AsArg < Gd < crate::classes::InputEvent >>,) -> bool {
            self.is_action_just_released_by_event_ex(action, event,) . done()
        }
        #[doc = "Returns `true` when the user _stops_ pressing the action event in the current frame or physics tick, and the first event that triggered action release in the current frame/physics tick was `event`. It will only return `true` on the frame or tick that the user releases the button.\n\nThis is useful when an action is processed during input handling (e.g. [`input`][`crate::classes::INode::input`]).\n\n**Note:** Returning `true` does not imply that the action is _still_ not pressed. An action can be released and pressed again rapidly, and `true` will still be returned so as not to miss input.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn is_action_just_released_by_event_ex < 'ex > (&'ex self, action: impl AsArg < StringName > + 'ex, event: impl AsArg < Gd < crate::classes::InputEvent >> + 'ex,) -> ExIsActionJustReleasedByEvent < 'ex > {
            ExIsActionJustReleasedByEvent::new(self, action, event,)
        }
        #[doc = "Returns a value between 0 and 1 representing the intensity of the given action. In a joypad, for example, the further away the axis (analog sticks or L2, R2 triggers) is from the dead zone, the closer the value will be to 1. If the action is mapped to a control that has no axis such as the keyboard, the value returned will be 0 or 1.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        pub(crate) fn get_action_strength_full(&self, action: CowArg < StringName >, exact_match: bool,) -> f32 {
            type CallRet = f32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_action_strength", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_action_strength_ex`][Self::get_action_strength_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a value between 0 and 1 representing the intensity of the given action. In a joypad, for example, the further away the axis (analog sticks or L2, R2 triggers) is from the dead zone, the closer the value will be to 1. If the action is mapped to a control that has no axis such as the keyboard, the value returned will be 0 or 1.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn get_action_strength(&self, action: impl AsArg < StringName >,) -> f32 {
            self.get_action_strength_ex(action,) . done()
        }
        #[doc = "Returns a value between 0 and 1 representing the intensity of the given action. In a joypad, for example, the further away the axis (analog sticks or L2, R2 triggers) is from the dead zone, the closer the value will be to 1. If the action is mapped to a control that has no axis such as the keyboard, the value returned will be 0 or 1.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn get_action_strength_ex < 'ex > (&'ex self, action: impl AsArg < StringName > + 'ex,) -> ExGetActionStrength < 'ex > {
            ExGetActionStrength::new(self, action,)
        }
        #[doc = "Returns a value between 0 and 1 representing the raw intensity of the given action, ignoring the action's deadzone. In most cases, you should use [`get_action_strength`][`crate::classes::Input::get_action_strength`] instead.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        pub(crate) fn get_action_raw_strength_full(&self, action: CowArg < StringName >, exact_match: bool,) -> f32 {
            type CallRet = f32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_action_raw_strength", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_action_raw_strength_ex`][Self::get_action_raw_strength_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a value between 0 and 1 representing the raw intensity of the given action, ignoring the action's deadzone. In most cases, you should use [`get_action_strength`][`crate::classes::Input::get_action_strength`] instead.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn get_action_raw_strength(&self, action: impl AsArg < StringName >,) -> f32 {
            self.get_action_raw_strength_ex(action,) . done()
        }
        #[doc = "Returns a value between 0 and 1 representing the raw intensity of the given action, ignoring the action's deadzone. In most cases, you should use [`get_action_strength`][`crate::classes::Input::get_action_strength`] instead.\n\nIf `exact_match` is `false`, it ignores additional input modifiers for [`InputEventKey`][crate::classes::InputEventKey] and [`InputEventMouseButton`][crate::classes::InputEventMouseButton] events, and the direction for [`InputEventJoypadMotion`][crate::classes::InputEventJoypadMotion] events."]
        #[inline]
        pub fn get_action_raw_strength_ex < 'ex > (&'ex self, action: impl AsArg < StringName > + 'ex,) -> ExGetActionRawStrength < 'ex > {
            ExGetActionRawStrength::new(self, action,)
        }
        #[doc = "Get axis input by specifying two actions, one negative and one positive.\n\nThis is a shorthand for writing `Input.get_action_strength(\"positive_action\") - Input.get_action_strength(\"negative_action\")`."]
        pub fn get_axis(&self, negative_action: impl AsArg < StringName >, positive_action: impl AsArg < StringName >,) -> f32 {
            type CallRet = f32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (negative_action.into_arg(), positive_action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_axis", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets an input vector by specifying four actions for the positive and negative X and Y axes.\n\nThis method is useful when getting vector input, such as from a joystick, directional pad, arrows, or WASD. The vector has its length limited to 1 and has a circular deadzone, which is useful for using vector input as movement.\n\nBy default, the deadzone is automatically calculated from the average of the action deadzones. However, you can override the deadzone to be whatever you want (on the range of 0 to 1)."]
        pub(crate) fn get_vector_full(&self, negative_x: CowArg < StringName >, positive_x: CowArg < StringName >, negative_y: CowArg < StringName >, positive_y: CowArg < StringName >, deadzone: f32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >, CowArg < 'a3, StringName >, f32,);
            let args = (negative_x, positive_x, negative_y, positive_y, deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_vector", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_vector_ex`][Self::get_vector_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Gets an input vector by specifying four actions for the positive and negative X and Y axes.\n\nThis method is useful when getting vector input, such as from a joystick, directional pad, arrows, or WASD. The vector has its length limited to 1 and has a circular deadzone, which is useful for using vector input as movement.\n\nBy default, the deadzone is automatically calculated from the average of the action deadzones. However, you can override the deadzone to be whatever you want (on the range of 0 to 1)."]
        #[inline]
        pub fn get_vector(&self, negative_x: impl AsArg < StringName >, positive_x: impl AsArg < StringName >, negative_y: impl AsArg < StringName >, positive_y: impl AsArg < StringName >,) -> Vector2 {
            self.get_vector_ex(negative_x, positive_x, negative_y, positive_y,) . done()
        }
        #[doc = "Gets an input vector by specifying four actions for the positive and negative X and Y axes.\n\nThis method is useful when getting vector input, such as from a joystick, directional pad, arrows, or WASD. The vector has its length limited to 1 and has a circular deadzone, which is useful for using vector input as movement.\n\nBy default, the deadzone is automatically calculated from the average of the action deadzones. However, you can override the deadzone to be whatever you want (on the range of 0 to 1)."]
        #[inline]
        pub fn get_vector_ex < 'ex > (&'ex self, negative_x: impl AsArg < StringName > + 'ex, positive_x: impl AsArg < StringName > + 'ex, negative_y: impl AsArg < StringName > + 'ex, positive_y: impl AsArg < StringName > + 'ex,) -> ExGetVector < 'ex > {
            ExGetVector::new(self, negative_x, positive_x, negative_y, positive_y,)
        }
        #[doc = "Adds a new mapping entry (in SDL2 format) to the mapping database. Optionally update already connected devices."]
        pub(crate) fn add_joy_mapping_full(&mut self, mapping: CowArg < GString >, update_existing: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (mapping, update_existing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "add_joy_mapping", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_joy_mapping_ex`][Self::add_joy_mapping_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new mapping entry (in SDL2 format) to the mapping database. Optionally update already connected devices."]
        #[inline]
        pub fn add_joy_mapping(&mut self, mapping: impl AsArg < GString >,) {
            self.add_joy_mapping_ex(mapping,) . done()
        }
        #[doc = "Adds a new mapping entry (in SDL2 format) to the mapping database. Optionally update already connected devices."]
        #[inline]
        pub fn add_joy_mapping_ex < 'ex > (&'ex mut self, mapping: impl AsArg < GString > + 'ex,) -> ExAddJoyMapping < 'ex > {
            ExAddJoyMapping::new(self, mapping,)
        }
        #[doc = "Removes all mappings from the internal database that match the given GUID. All currently connected joypads that use this GUID will become unmapped.\n\nOn Android, Godot will map to an internal fallback mapping."]
        pub fn remove_joy_mapping(&mut self, guid: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (guid.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "remove_joy_mapping", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the system knows the specified device. This means that it sets all button and axis indices. Unknown joypads are not expected to match these constants, but you can still retrieve events from them."]
        pub fn is_joy_known(&self, device: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_joy_known", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current value of the joypad axis at index `axis`."]
        pub fn get_joy_axis(&self, device: i32, axis: crate::global::JoyAxis,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, crate::global::JoyAxis,);
            let args = (device, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_joy_axis", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the joypad at the specified device index, e.g. `PS4 Controller`. Godot uses the [SDL2 game controller database](https://github.com/gabomdq/SDL_GameControllerDB) to determine gamepad names."]
        pub fn get_joy_name(&self, device: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_joy_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an SDL2-compatible device GUID on platforms that use gamepad remapping, e.g. `030000004c050000c405000000010000`. Returns an empty string if it cannot be found. Godot uses the [SDL2 game controller database](https://github.com/gabomdq/SDL_GameControllerDB) to determine gamepad names and mappings based on this GUID.\n\nOn Windows, all XInput joypad GUIDs will be overridden by Godot to `__XINPUT_DEVICE__`, because their mappings are the same."]
        pub fn get_joy_guid(&self, device: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_joy_guid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a dictionary with extra platform-specific information about the device, e.g. the raw gamepad name from the OS or the Steam Input index.\n\nOn Windows, Linux, and macOS, the dictionary contains the following fields:\n\n`raw_name`: The name of the controller as it came from the OS, before getting renamed by the controller database.\n\n`vendor_id`: The USB vendor ID of the device.\n\n`product_id`: The USB product ID of the device.\n\n`steam_input_index`: The Steam Input gamepad index, if the device is not a Steam Input device this key won't be present.\n\nOn Windows, the dictionary can have an additional field:\n\n`xinput_index`: The index of the controller in the XInput system. This key won't be present for devices not handled by XInput.\n\n**Note:** The returned dictionary is always empty on Android, iOS, visionOS, and Web."]
        pub fn get_joy_info(&self, device: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_joy_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Queries whether an input device should be ignored or not. Devices can be ignored by setting the environment variable `SDL_GAMECONTROLLER_IGNORE_DEVICES`. Read the [SDL documentation](https://wiki.libsdl.org/SDL2) for more information.\n\n**Note:** Some 3rd party tools can contribute to the list of ignored devices. For example, _SteamInput_ creates virtual devices from physical devices for remapping purposes. To avoid handling the same input device twice, the original device is added to the ignore list."]
        pub fn should_ignore_device(&self, vendor_id: i32, product_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (vendor_id, product_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "should_ignore_device", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] containing the device IDs of all currently connected joypads."]
        pub fn get_connected_joypads(&self,) -> Array < i64 > {
            type CallRet = Array < i64 >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_connected_joypads", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the strength of the joypad vibration: x is the strength of the weak motor, and y is the strength of the strong motor."]
        pub fn get_joy_vibration_strength(&self, device: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_joy_vibration_strength", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the duration of the current vibration effect in seconds."]
        pub fn get_joy_vibration_duration(&self, device: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_joy_vibration_duration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Starts to vibrate the joypad. Joypads usually come with two rumble motors, a strong and a weak one. `weak_magnitude` is the strength of the weak motor (between 0 and 1) and `strong_magnitude` is the strength of the strong motor (between 0 and 1). `duration` is the duration of the effect in seconds (a duration of 0 will try to play the vibration indefinitely). The vibration can be stopped early by calling [`stop_joy_vibration`][`crate::classes::Input::stop_joy_vibration`].\n\n**Note:** Not every hardware is compatible with long effect durations; it is recommended to restart an effect if it has to be played for more than a few seconds.\n\n**Note:** For macOS, vibration is only supported in macOS 11 and later."]
        pub(crate) fn start_joy_vibration_full(&mut self, device: i32, weak_magnitude: f32, strong_magnitude: f32, duration: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32, f32, f32,);
            let args = (device, weak_magnitude, strong_magnitude, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "start_joy_vibration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`start_joy_vibration_ex`][Self::start_joy_vibration_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Starts to vibrate the joypad. Joypads usually come with two rumble motors, a strong and a weak one. `weak_magnitude` is the strength of the weak motor (between 0 and 1) and `strong_magnitude` is the strength of the strong motor (between 0 and 1). `duration` is the duration of the effect in seconds (a duration of 0 will try to play the vibration indefinitely). The vibration can be stopped early by calling [`stop_joy_vibration`][`crate::classes::Input::stop_joy_vibration`].\n\n**Note:** Not every hardware is compatible with long effect durations; it is recommended to restart an effect if it has to be played for more than a few seconds.\n\n**Note:** For macOS, vibration is only supported in macOS 11 and later."]
        #[inline]
        pub fn start_joy_vibration(&mut self, device: i32, weak_magnitude: f32, strong_magnitude: f32,) {
            self.start_joy_vibration_ex(device, weak_magnitude, strong_magnitude,) . done()
        }
        #[doc = "Starts to vibrate the joypad. Joypads usually come with two rumble motors, a strong and a weak one. `weak_magnitude` is the strength of the weak motor (between 0 and 1) and `strong_magnitude` is the strength of the strong motor (between 0 and 1). `duration` is the duration of the effect in seconds (a duration of 0 will try to play the vibration indefinitely). The vibration can be stopped early by calling [`stop_joy_vibration`][`crate::classes::Input::stop_joy_vibration`].\n\n**Note:** Not every hardware is compatible with long effect durations; it is recommended to restart an effect if it has to be played for more than a few seconds.\n\n**Note:** For macOS, vibration is only supported in macOS 11 and later."]
        #[inline]
        pub fn start_joy_vibration_ex < 'ex > (&'ex mut self, device: i32, weak_magnitude: f32, strong_magnitude: f32,) -> ExStartJoyVibration < 'ex > {
            ExStartJoyVibration::new(self, device, weak_magnitude, strong_magnitude,)
        }
        #[doc = "Stops the vibration of the joypad started with [`start_joy_vibration`][`crate::classes::Input::start_joy_vibration`]."]
        pub fn stop_joy_vibration(&mut self, device: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "stop_joy_vibration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Vibrate the handheld device for the specified duration in milliseconds.\n\n`amplitude` is the strength of the vibration, as a value between `0.0` and `1.0`. If set to `-1.0`, the default vibration strength of the device is used.\n\n**Note:** This method is implemented on Android, iOS, and Web. It has no effect on other platforms.\n\n**Note:** For Android, [`vibrate_handheld`][`crate::classes::Input::vibrate_handheld`] requires enabling the `VIBRATE` permission in the export preset. Otherwise, [`vibrate_handheld`][`crate::classes::Input::vibrate_handheld`] will have no effect.\n\n**Note:** For iOS, specifying the duration is only supported in iOS 13 and later.\n\n**Note:** For Web, the amplitude cannot be changed.\n\n**Note:** Some web browsers such as Safari and Firefox for Android do not support [`vibrate_handheld`][`crate::classes::Input::vibrate_handheld`]."]
        pub(crate) fn vibrate_handheld_full(&mut self, duration_ms: i32, amplitude: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (duration_ms, amplitude,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "vibrate_handheld", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`vibrate_handheld_ex`][Self::vibrate_handheld_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Vibrate the handheld device for the specified duration in milliseconds.\n\n`amplitude` is the strength of the vibration, as a value between `0.0` and `1.0`. If set to `-1.0`, the default vibration strength of the device is used.\n\n**Note:** This method is implemented on Android, iOS, and Web. It has no effect on other platforms.\n\n**Note:** For Android, [`vibrate_handheld`][`crate::classes::Input::vibrate_handheld`] requires enabling the `VIBRATE` permission in the export preset. Otherwise, [`vibrate_handheld`][`crate::classes::Input::vibrate_handheld`] will have no effect.\n\n**Note:** For iOS, specifying the duration is only supported in iOS 13 and later.\n\n**Note:** For Web, the amplitude cannot be changed.\n\n**Note:** Some web browsers such as Safari and Firefox for Android do not support [`vibrate_handheld`][`crate::classes::Input::vibrate_handheld`]."]
        #[inline]
        pub fn vibrate_handheld(&mut self,) {
            self.vibrate_handheld_ex() . done()
        }
        #[doc = "Vibrate the handheld device for the specified duration in milliseconds.\n\n`amplitude` is the strength of the vibration, as a value between `0.0` and `1.0`. If set to `-1.0`, the default vibration strength of the device is used.\n\n**Note:** This method is implemented on Android, iOS, and Web. It has no effect on other platforms.\n\n**Note:** For Android, [`vibrate_handheld`][`crate::classes::Input::vibrate_handheld`] requires enabling the `VIBRATE` permission in the export preset. Otherwise, [`vibrate_handheld`][`crate::classes::Input::vibrate_handheld`] will have no effect.\n\n**Note:** For iOS, specifying the duration is only supported in iOS 13 and later.\n\n**Note:** For Web, the amplitude cannot be changed.\n\n**Note:** Some web browsers such as Safari and Firefox for Android do not support [`vibrate_handheld`][`crate::classes::Input::vibrate_handheld`]."]
        #[inline]
        pub fn vibrate_handheld_ex < 'ex > (&'ex mut self,) -> ExVibrateHandheld < 'ex > {
            ExVibrateHandheld::new(self,)
        }
        #[doc = "Returns the gravity in m/s² of the device's accelerometer sensor, if the device has one. Otherwise, the method returns `Vector3.ZERO`.\n\n**Note:** This method only works on Android and iOS. On other platforms, it always returns `Vector3.ZERO`.\n\n**Note:** For Android, \\[member ProjectSettings.input_devices/sensors/enable_gravity] must be enabled."]
        pub fn get_gravity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_gravity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the acceleration in m/s² of the device's accelerometer sensor, if the device has one. Otherwise, the method returns `Vector3.ZERO`.\n\nNote this method returns an empty [`Vector3`][crate::builtin::Vector3] when running from the editor even when your device has an accelerometer. You must export your project to a supported device to read values from the accelerometer.\n\n**Note:** This method only works on Android and iOS. On other platforms, it always returns `Vector3.ZERO`.\n\n**Note:** For Android, \\[member ProjectSettings.input_devices/sensors/enable_accelerometer] must be enabled."]
        pub fn get_accelerometer(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_accelerometer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the magnetic field strength in micro-Tesla for all axes of the device's magnetometer sensor, if the device has one. Otherwise, the method returns `Vector3.ZERO`.\n\n**Note:** This method only works on Android and iOS. On other platforms, it always returns `Vector3.ZERO`.\n\n**Note:** For Android, \\[member ProjectSettings.input_devices/sensors/enable_magnetometer] must be enabled."]
        pub fn get_magnetometer(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_magnetometer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the rotation rate in rad/s around a device's X, Y, and Z axes of the gyroscope sensor, if the device has one. Otherwise, the method returns `Vector3.ZERO`.\n\n**Note:** This method only works on Android and iOS. On other platforms, it always returns `Vector3.ZERO`.\n\n**Note:** For Android, \\[member ProjectSettings.input_devices/sensors/enable_gyroscope] must be enabled."]
        pub fn get_gyroscope(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_gyroscope", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the gravity value of the accelerometer sensor. Can be used for debugging on devices without a hardware sensor, for example in an editor on a PC.\n\n**Note:** This value can be immediately overwritten by the hardware sensor value on Android and iOS."]
        pub fn set_gravity(&mut self, value: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_gravity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the acceleration value of the accelerometer sensor. Can be used for debugging on devices without a hardware sensor, for example in an editor on a PC.\n\n**Note:** This value can be immediately overwritten by the hardware sensor value on Android and iOS."]
        pub fn set_accelerometer(&mut self, value: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_accelerometer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of the magnetic field of the magnetometer sensor. Can be used for debugging on devices without a hardware sensor, for example in an editor on a PC.\n\n**Note:** This value can be immediately overwritten by the hardware sensor value on Android and iOS."]
        pub fn set_magnetometer(&mut self, value: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_magnetometer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of the rotation rate of the gyroscope sensor. Can be used for debugging on devices without a hardware sensor, for example in an editor on a PC.\n\n**Note:** This value can be immediately overwritten by the hardware sensor value on Android and iOS."]
        pub fn set_gyroscope(&mut self, value: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_gyroscope", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the joypad's LED light, if available, to the specified color. See also [`has_joy_light`][`crate::classes::Input::has_joy_light`].\n\n**Note:** There is no way to get the color of the light from a joypad. If you need to know the assigned color, store it separately.\n\n**Note:** This feature is only supported on Windows, Linux, and macOS."]
        pub fn set_joy_light(&mut self, device: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (device, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_joy_light", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the joypad has an LED light that can change colors and/or brightness. See also [`set_joy_light`][`crate::classes::Input::set_joy_light`].\n\n**Note:** This feature is only supported on Windows, Linux, and macOS."]
        pub fn has_joy_light(&self, device: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "has_joy_light", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the last mouse velocity. To provide a precise and jitter-free velocity, mouse velocity is only calculated every 0.1s. Therefore, mouse velocity will lag mouse movements."]
        pub fn get_last_mouse_velocity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_last_mouse_velocity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the last mouse velocity in screen coordinates. To provide a precise and jitter-free velocity, mouse velocity is only calculated every 0.1s. Therefore, mouse velocity will lag mouse movements."]
        pub fn get_last_mouse_screen_velocity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_last_mouse_screen_velocity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns mouse buttons as a bitmask. If multiple mouse buttons are pressed at the same time, the bits are added together. Equivalent to [`mouse_get_button_state`][`crate::classes::DisplayServer::mouse_get_button_state`]."]
        pub fn get_mouse_button_mask(&self,) -> crate::global::MouseButtonMask {
            type CallRet = crate::global::MouseButtonMask;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_mouse_button_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mouse_mode(&mut self, mode: crate::classes::input::MouseMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::input::MouseMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_mouse_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mouse_mode(&self,) -> crate::classes::input::MouseMode {
            type CallRet = crate::classes::input::MouseMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_mouse_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the mouse position to the specified vector, provided in pixels and relative to an origin at the upper left corner of the currently focused Window Manager game window.\n\nMouse position is clipped to the limits of the screen resolution, or to the limits of the game window if \\[enum MouseMode] is set to [`MouseMode::CONFINED`][`crate::classes::input::MouseMode::CONFINED`] or [`MouseMode::CONFINED_HIDDEN`][`crate::classes::input::MouseMode::CONFINED_HIDDEN`].\n\n**Note:** [`warp_mouse`][`crate::classes::Input::warp_mouse`] is only supported on Windows, macOS and Linux. It has no effect on Android, iOS and Web."]
        pub fn warp_mouse(&mut self, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "warp_mouse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This will simulate pressing the specified action.\n\nThe strength can be used for non-boolean actions, it's ranged between 0 and 1 representing the intensity of the given action.\n\n**Note:** This method will not cause any [`input`][`crate::classes::INode::input`] calls. It is intended to be used with [`is_action_pressed`][`crate::classes::Input::is_action_pressed`] and [`is_action_just_pressed`][`crate::classes::Input::is_action_just_pressed`]. If you want to simulate `_input`, use [`parse_input_event`][`crate::classes::Input::parse_input_event`] instead."]
        pub(crate) fn action_press_full(&mut self, action: CowArg < StringName >, strength: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f32,);
            let args = (action, strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "action_press", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`action_press_ex`][Self::action_press_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "This will simulate pressing the specified action.\n\nThe strength can be used for non-boolean actions, it's ranged between 0 and 1 representing the intensity of the given action.\n\n**Note:** This method will not cause any [`input`][`crate::classes::INode::input`] calls. It is intended to be used with [`is_action_pressed`][`crate::classes::Input::is_action_pressed`] and [`is_action_just_pressed`][`crate::classes::Input::is_action_just_pressed`]. If you want to simulate `_input`, use [`parse_input_event`][`crate::classes::Input::parse_input_event`] instead."]
        #[inline]
        pub fn action_press(&mut self, action: impl AsArg < StringName >,) {
            self.action_press_ex(action,) . done()
        }
        #[doc = "This will simulate pressing the specified action.\n\nThe strength can be used for non-boolean actions, it's ranged between 0 and 1 representing the intensity of the given action.\n\n**Note:** This method will not cause any [`input`][`crate::classes::INode::input`] calls. It is intended to be used with [`is_action_pressed`][`crate::classes::Input::is_action_pressed`] and [`is_action_just_pressed`][`crate::classes::Input::is_action_just_pressed`]. If you want to simulate `_input`, use [`parse_input_event`][`crate::classes::Input::parse_input_event`] instead."]
        #[inline]
        pub fn action_press_ex < 'ex > (&'ex mut self, action: impl AsArg < StringName > + 'ex,) -> ExActionPress < 'ex > {
            ExActionPress::new(self, action,)
        }
        #[doc = "If the specified action is already pressed, this will release it."]
        pub fn action_release(&mut self, action: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "action_release", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the default cursor shape to be used in the viewport instead of [`CursorShape::ARROW`][`crate::classes::input::CursorShape::ARROW`].\n\n**Note:** If you want to change the default cursor shape for [`Control`][crate::classes::Control]'s nodes, use \\[member Control.mouse_default_cursor_shape] instead.\n\n**Note:** This method generates an [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] to update cursor immediately."]
        pub(crate) fn set_default_cursor_shape_full(&mut self, shape: crate::classes::input::CursorShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::input::CursorShape,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_default_cursor_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_default_cursor_shape_ex`][Self::set_default_cursor_shape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the default cursor shape to be used in the viewport instead of [`CursorShape::ARROW`][`crate::classes::input::CursorShape::ARROW`].\n\n**Note:** If you want to change the default cursor shape for [`Control`][crate::classes::Control]'s nodes, use \\[member Control.mouse_default_cursor_shape] instead.\n\n**Note:** This method generates an [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] to update cursor immediately."]
        #[inline]
        pub fn set_default_cursor_shape(&mut self,) {
            self.set_default_cursor_shape_ex() . done()
        }
        #[doc = "Sets the default cursor shape to be used in the viewport instead of [`CursorShape::ARROW`][`crate::classes::input::CursorShape::ARROW`].\n\n**Note:** If you want to change the default cursor shape for [`Control`][crate::classes::Control]'s nodes, use \\[member Control.mouse_default_cursor_shape] instead.\n\n**Note:** This method generates an [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] to update cursor immediately."]
        #[inline]
        pub fn set_default_cursor_shape_ex < 'ex > (&'ex mut self,) -> ExSetDefaultCursorShape < 'ex > {
            ExSetDefaultCursorShape::new(self,)
        }
        #[doc = "Returns the currently assigned cursor shape."]
        pub fn get_current_cursor_shape(&self,) -> crate::classes::input::CursorShape {
            type CallRet = crate::classes::input::CursorShape;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "get_current_cursor_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a custom mouse cursor image, which is only visible inside the game window, for the given mouse `shape`. The hotspot can also be specified. Passing `null` to the image parameter resets to the system cursor.\n\n`image` can be either [`Texture2D`][crate::classes::Texture2D] or [`Image`][crate::classes::Image] and its size must be lower than or equal to 256×256. To avoid rendering issues, sizes lower than or equal to 128×128 are recommended.\n\n`hotspot` must be within `image`'s size.\n\n**Note:** [`AnimatedTexture`][crate::classes::AnimatedTexture]s aren't supported as custom mouse cursors. If using an [`AnimatedTexture`][crate::classes::AnimatedTexture], only the first frame will be displayed.\n\n**Note:** The **Lossless**, **Lossy** or **Uncompressed** compression modes are recommended. The **Video RAM** compression mode can be used, but it will be decompressed on the CPU, which means loading times are slowed down and no memory is saved compared to lossless modes.\n\n**Note:** On the web platform, the maximum allowed cursor image size is 128×128. Cursor images larger than 32×32 will also only be displayed if the mouse cursor image is entirely located within the page for [security reasons](https://chromestatus.com/feature/5825971391299584)."]
        pub(crate) fn set_custom_mouse_cursor_full(&mut self, image: CowArg < Option < Gd < crate::classes::Resource > > >, shape: crate::classes::input::CursorShape, hotspot: Vector2,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >, crate::classes::input::CursorShape, Vector2,);
            let args = (image, shape, hotspot,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_custom_mouse_cursor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_custom_mouse_cursor_ex`][Self::set_custom_mouse_cursor_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets a custom mouse cursor image, which is only visible inside the game window, for the given mouse `shape`. The hotspot can also be specified. Passing `null` to the image parameter resets to the system cursor.\n\n`image` can be either [`Texture2D`][crate::classes::Texture2D] or [`Image`][crate::classes::Image] and its size must be lower than or equal to 256×256. To avoid rendering issues, sizes lower than or equal to 128×128 are recommended.\n\n`hotspot` must be within `image`'s size.\n\n**Note:** [`AnimatedTexture`][crate::classes::AnimatedTexture]s aren't supported as custom mouse cursors. If using an [`AnimatedTexture`][crate::classes::AnimatedTexture], only the first frame will be displayed.\n\n**Note:** The **Lossless**, **Lossy** or **Uncompressed** compression modes are recommended. The **Video RAM** compression mode can be used, but it will be decompressed on the CPU, which means loading times are slowed down and no memory is saved compared to lossless modes.\n\n**Note:** On the web platform, the maximum allowed cursor image size is 128×128. Cursor images larger than 32×32 will also only be displayed if the mouse cursor image is entirely located within the page for [security reasons](https://chromestatus.com/feature/5825971391299584)."]
        #[inline]
        pub fn set_custom_mouse_cursor(&mut self, image: impl AsArg < Option < Gd < crate::classes::Resource >> >,) {
            self.set_custom_mouse_cursor_ex(image,) . done()
        }
        #[doc = "Sets a custom mouse cursor image, which is only visible inside the game window, for the given mouse `shape`. The hotspot can also be specified. Passing `null` to the image parameter resets to the system cursor.\n\n`image` can be either [`Texture2D`][crate::classes::Texture2D] or [`Image`][crate::classes::Image] and its size must be lower than or equal to 256×256. To avoid rendering issues, sizes lower than or equal to 128×128 are recommended.\n\n`hotspot` must be within `image`'s size.\n\n**Note:** [`AnimatedTexture`][crate::classes::AnimatedTexture]s aren't supported as custom mouse cursors. If using an [`AnimatedTexture`][crate::classes::AnimatedTexture], only the first frame will be displayed.\n\n**Note:** The **Lossless**, **Lossy** or **Uncompressed** compression modes are recommended. The **Video RAM** compression mode can be used, but it will be decompressed on the CPU, which means loading times are slowed down and no memory is saved compared to lossless modes.\n\n**Note:** On the web platform, the maximum allowed cursor image size is 128×128. Cursor images larger than 32×32 will also only be displayed if the mouse cursor image is entirely located within the page for [security reasons](https://chromestatus.com/feature/5825971391299584)."]
        #[inline]
        pub fn set_custom_mouse_cursor_ex < 'ex > (&'ex mut self, image: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'ex,) -> ExSetCustomMouseCursor < 'ex > {
            ExSetCustomMouseCursor::new(self, image,)
        }
        #[doc = "Feeds an [`InputEvent`][crate::classes::InputEvent] to the game. Can be used to artificially trigger input events from code. Also generates [`input`][`crate::classes::INode::input`] calls.\n\n\n```gdscript\nvar cancel_event = InputEventAction.new()\ncancel_event.action = \"ui_cancel\"\ncancel_event.pressed = true\nInput.parse_input_event(cancel_event)\n```\n\n\n**Note:** Calling this function has no influence on the operating system. So for example sending an [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] will not move the OS mouse cursor to the specified position (use [`warp_mouse`][`crate::classes::Input::warp_mouse`] instead) and sending `Alt/Cmd + Tab` as [`InputEventKey`][crate::classes::InputEventKey] won't toggle between active windows."]
        pub fn parse_input_event(&mut self, event: impl AsArg < Gd < crate::classes::InputEvent >>,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::InputEvent > >,);
            let args = (event.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "parse_input_event", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_accumulated_input(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_use_accumulated_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_accumulated_input(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_using_accumulated_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sends all input events which are in the current buffer to the game loop. These events may have been buffered as a result of accumulated input (\\[member use_accumulated_input]) or agile input flushing (\\[member ProjectSettings.input_devices/buffering/agile_event_flushing]).\n\nThe engine will already do this itself at key execution points (at least once per frame). However, this can be useful in advanced cases where you want precise control over the timing of event handling."]
        pub fn flush_buffered_events(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "flush_buffered_events", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emulate_mouse_from_touch(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_emulate_mouse_from_touch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_emulating_mouse_from_touch(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_emulating_mouse_from_touch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emulate_touch_from_mouse(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "set_emulate_touch_from_mouse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_emulating_touch_from_mouse(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9718usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Input", "is_emulating_touch_from_mouse", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Input {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Input"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Input {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Input {
        
    }
    impl crate::obj::Singleton for Input {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"Input"))
            }
        }
    }
    impl std::ops::Deref for Input {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Input {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Input__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Input` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Input::is_action_pressed_ex`][super::Input::is_action_pressed_ex]."]
#[must_use]
pub struct ExIsActionPressed < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Input, action: CowArg < 'ex, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsActionPressed < 'ex > {
    fn new(surround_object: &'ex re_export::Input, action: impl AsArg < StringName > + 'ex,) -> Self {
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
        re_export::Input::is_action_pressed_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::is_action_just_pressed_ex`][super::Input::is_action_just_pressed_ex]."]
#[must_use]
pub struct ExIsActionJustPressed < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Input, action: CowArg < 'ex, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsActionJustPressed < 'ex > {
    fn new(surround_object: &'ex re_export::Input, action: impl AsArg < StringName > + 'ex,) -> Self {
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
        re_export::Input::is_action_just_pressed_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::is_action_just_released_ex`][super::Input::is_action_just_released_ex]."]
#[must_use]
pub struct ExIsActionJustReleased < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Input, action: CowArg < 'ex, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsActionJustReleased < 'ex > {
    fn new(surround_object: &'ex re_export::Input, action: impl AsArg < StringName > + 'ex,) -> Self {
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
        re_export::Input::is_action_just_released_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::is_action_just_pressed_by_event_ex`][super::Input::is_action_just_pressed_by_event_ex]."]
#[must_use]
pub struct ExIsActionJustPressedByEvent < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Input, action: CowArg < 'ex, StringName >, event: CowArg < 'ex, Gd < crate::classes::InputEvent > >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsActionJustPressedByEvent < 'ex > {
    fn new(surround_object: &'ex re_export::Input, action: impl AsArg < StringName > + 'ex, event: impl AsArg < Gd < crate::classes::InputEvent >> + 'ex,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), event: event.into_arg(), exact_match: exact_match,
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
            _phantom, surround_object, action, event, exact_match,
        }
        = self;
        re_export::Input::is_action_just_pressed_by_event_full(surround_object, action, event, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::is_action_just_released_by_event_ex`][super::Input::is_action_just_released_by_event_ex]."]
#[must_use]
pub struct ExIsActionJustReleasedByEvent < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Input, action: CowArg < 'ex, StringName >, event: CowArg < 'ex, Gd < crate::classes::InputEvent > >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsActionJustReleasedByEvent < 'ex > {
    fn new(surround_object: &'ex re_export::Input, action: impl AsArg < StringName > + 'ex, event: impl AsArg < Gd < crate::classes::InputEvent >> + 'ex,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), event: event.into_arg(), exact_match: exact_match,
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
            _phantom, surround_object, action, event, exact_match,
        }
        = self;
        re_export::Input::is_action_just_released_by_event_full(surround_object, action, event, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::get_action_strength_ex`][super::Input::get_action_strength_ex]."]
#[must_use]
pub struct ExGetActionStrength < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Input, action: CowArg < 'ex, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetActionStrength < 'ex > {
    fn new(surround_object: &'ex re_export::Input, action: impl AsArg < StringName > + 'ex,) -> Self {
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
        re_export::Input::get_action_strength_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::get_action_raw_strength_ex`][super::Input::get_action_raw_strength_ex]."]
#[must_use]
pub struct ExGetActionRawStrength < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Input, action: CowArg < 'ex, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetActionRawStrength < 'ex > {
    fn new(surround_object: &'ex re_export::Input, action: impl AsArg < StringName > + 'ex,) -> Self {
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
        re_export::Input::get_action_raw_strength_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`Input::get_vector_ex`][super::Input::get_vector_ex]."]
#[must_use]
pub struct ExGetVector < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Input, negative_x: CowArg < 'ex, StringName >, positive_x: CowArg < 'ex, StringName >, negative_y: CowArg < 'ex, StringName >, positive_y: CowArg < 'ex, StringName >, deadzone: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetVector < 'ex > {
    fn new(surround_object: &'ex re_export::Input, negative_x: impl AsArg < StringName > + 'ex, positive_x: impl AsArg < StringName > + 'ex, negative_y: impl AsArg < StringName > + 'ex, positive_y: impl AsArg < StringName > + 'ex,) -> Self {
        let deadzone = - 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, negative_x: negative_x.into_arg(), positive_x: positive_x.into_arg(), negative_y: negative_y.into_arg(), positive_y: positive_y.into_arg(), deadzone: deadzone,
        }
    }
    #[inline]
    pub fn deadzone(self, deadzone: f32) -> Self {
        Self {
            deadzone: deadzone, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        let Self {
            _phantom, surround_object, negative_x, positive_x, negative_y, positive_y, deadzone,
        }
        = self;
        re_export::Input::get_vector_full(surround_object, negative_x, positive_x, negative_y, positive_y, deadzone,)
    }
}
#[doc = "Default-param extender for [`Input::add_joy_mapping_ex`][super::Input::add_joy_mapping_ex]."]
#[must_use]
pub struct ExAddJoyMapping < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Input, mapping: CowArg < 'ex, GString >, update_existing: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddJoyMapping < 'ex > {
    fn new(surround_object: &'ex mut re_export::Input, mapping: impl AsArg < GString > + 'ex,) -> Self {
        let update_existing = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mapping: mapping.into_arg(), update_existing: update_existing,
        }
    }
    #[inline]
    pub fn update_existing(self, update_existing: bool) -> Self {
        Self {
            update_existing: update_existing, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, mapping, update_existing,
        }
        = self;
        re_export::Input::add_joy_mapping_full(surround_object, mapping, update_existing,)
    }
}
#[doc = "Default-param extender for [`Input::start_joy_vibration_ex`][super::Input::start_joy_vibration_ex]."]
#[must_use]
pub struct ExStartJoyVibration < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Input, device: i32, weak_magnitude: f32, strong_magnitude: f32, duration: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExStartJoyVibration < 'ex > {
    fn new(surround_object: &'ex mut re_export::Input, device: i32, weak_magnitude: f32, strong_magnitude: f32,) -> Self {
        let duration = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, device: device, weak_magnitude: weak_magnitude, strong_magnitude: strong_magnitude, duration: duration,
        }
    }
    #[inline]
    pub fn duration(self, duration: f32) -> Self {
        Self {
            duration: duration, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, device, weak_magnitude, strong_magnitude, duration,
        }
        = self;
        re_export::Input::start_joy_vibration_full(surround_object, device, weak_magnitude, strong_magnitude, duration,)
    }
}
#[doc = "Default-param extender for [`Input::vibrate_handheld_ex`][super::Input::vibrate_handheld_ex]."]
#[must_use]
pub struct ExVibrateHandheld < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Input, duration_ms: i32, amplitude: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExVibrateHandheld < 'ex > {
    fn new(surround_object: &'ex mut re_export::Input,) -> Self {
        let duration_ms = 500i32;
        let amplitude = - 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, duration_ms: duration_ms, amplitude: amplitude,
        }
    }
    #[inline]
    pub fn duration_ms(self, duration_ms: i32) -> Self {
        Self {
            duration_ms: duration_ms, .. self
        }
    }
    #[inline]
    pub fn amplitude(self, amplitude: f32) -> Self {
        Self {
            amplitude: amplitude, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, duration_ms, amplitude,
        }
        = self;
        re_export::Input::vibrate_handheld_full(surround_object, duration_ms, amplitude,)
    }
}
#[doc = "Default-param extender for [`Input::action_press_ex`][super::Input::action_press_ex]."]
#[must_use]
pub struct ExActionPress < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Input, action: CowArg < 'ex, StringName >, strength: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExActionPress < 'ex > {
    fn new(surround_object: &'ex mut re_export::Input, action: impl AsArg < StringName > + 'ex,) -> Self {
        let strength = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), strength: strength,
        }
    }
    #[inline]
    pub fn strength(self, strength: f32) -> Self {
        Self {
            strength: strength, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, action, strength,
        }
        = self;
        re_export::Input::action_press_full(surround_object, action, strength,)
    }
}
#[doc = "Default-param extender for [`Input::set_default_cursor_shape_ex`][super::Input::set_default_cursor_shape_ex]."]
#[must_use]
pub struct ExSetDefaultCursorShape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Input, shape: crate::classes::input::CursorShape,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetDefaultCursorShape < 'ex > {
    fn new(surround_object: &'ex mut re_export::Input,) -> Self {
        let shape = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shape: shape,
        }
    }
    #[inline]
    pub fn shape(self, shape: crate::classes::input::CursorShape) -> Self {
        Self {
            shape: shape, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shape,
        }
        = self;
        re_export::Input::set_default_cursor_shape_full(surround_object, shape,)
    }
}
#[doc = "Default-param extender for [`Input::set_custom_mouse_cursor_ex`][super::Input::set_custom_mouse_cursor_ex]."]
#[must_use]
pub struct ExSetCustomMouseCursor < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Input, image: CowArg < 'ex, Option < Gd < crate::classes::Resource > > >, shape: crate::classes::input::CursorShape, hotspot: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetCustomMouseCursor < 'ex > {
    fn new(surround_object: &'ex mut re_export::Input, image: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'ex,) -> Self {
        let shape = crate::obj::EngineEnum::from_ord(0);
        let hotspot = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, image: image.into_arg(), shape: shape, hotspot: hotspot,
        }
    }
    #[inline]
    pub fn shape(self, shape: crate::classes::input::CursorShape) -> Self {
        Self {
            shape: shape, .. self
        }
    }
    #[inline]
    pub fn hotspot(self, hotspot: Vector2) -> Self {
        Self {
            hotspot: hotspot, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, image, shape, hotspot,
        }
        = self;
        re_export::Input::set_custom_mouse_cursor_full(surround_object, image, shape, hotspot,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MouseMode {
    ord: i32
}
impl MouseMode {
    #[doc(alias = "MOUSE_MODE_VISIBLE")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_VISIBLE`"]
    pub const VISIBLE: MouseMode = MouseMode {
        ord: 0i32
    };
    #[doc(alias = "MOUSE_MODE_HIDDEN")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_HIDDEN`"]
    pub const HIDDEN: MouseMode = MouseMode {
        ord: 1i32
    };
    #[doc(alias = "MOUSE_MODE_CAPTURED")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CAPTURED`"]
    pub const CAPTURED: MouseMode = MouseMode {
        ord: 2i32
    };
    #[doc(alias = "MOUSE_MODE_CONFINED")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CONFINED`"]
    pub const CONFINED: MouseMode = MouseMode {
        ord: 3i32
    };
    #[doc(alias = "MOUSE_MODE_CONFINED_HIDDEN")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CONFINED_HIDDEN`"]
    pub const CONFINED_HIDDEN: MouseMode = MouseMode {
        ord: 4i32
    };
    #[doc(alias = "MOUSE_MODE_MAX")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_MAX`"]
    pub const MAX: MouseMode = MouseMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for MouseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MouseMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MouseMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::VISIBLE => "VISIBLE", Self::HIDDEN => "HIDDEN", Self::CAPTURED => "CAPTURED", Self::CONFINED => "CONFINED", Self::CONFINED_HIDDEN => "CONFINED_HIDDEN", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MouseMode::VISIBLE, MouseMode::HIDDEN, MouseMode::CAPTURED, MouseMode::CONFINED, MouseMode::CONFINED_HIDDEN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MouseMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VISIBLE", "MOUSE_MODE_VISIBLE", MouseMode::VISIBLE), crate::meta::inspect::EnumConstant::new("HIDDEN", "MOUSE_MODE_HIDDEN", MouseMode::HIDDEN), crate::meta::inspect::EnumConstant::new("CAPTURED", "MOUSE_MODE_CAPTURED", MouseMode::CAPTURED), crate::meta::inspect::EnumConstant::new("CONFINED", "MOUSE_MODE_CONFINED", MouseMode::CONFINED), crate::meta::inspect::EnumConstant::new("CONFINED_HIDDEN", "MOUSE_MODE_CONFINED_HIDDEN", MouseMode::CONFINED_HIDDEN), crate::meta::inspect::EnumConstant::new("MAX", "MOUSE_MODE_MAX", MouseMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for MouseMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for MouseMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Mouse Mode Visible", 0i64), EnumeratorShape::new_int("Mouse Mode Hidden", 1i64), EnumeratorShape::new_int("Mouse Mode Captured", 2i64), EnumeratorShape::new_int("Mouse Mode Confined", 3i64), EnumeratorShape::new_int("Mouse Mode Confined Hidden", 4i64), EnumeratorShape::new_int("Mouse Mode Max", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Input.MouseMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for MouseMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MouseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for MouseMode {
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
impl crate::registry::property::Export for MouseMode {
    
}
impl crate::meta::Element for MouseMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CursorShape {
    ord: i32
}
impl CursorShape {
    #[doc(alias = "CURSOR_ARROW")]
    #[doc = "Godot enumerator name: `CURSOR_ARROW`"]
    pub const ARROW: CursorShape = CursorShape {
        ord: 0i32
    };
    #[doc(alias = "CURSOR_IBEAM")]
    #[doc = "Godot enumerator name: `CURSOR_IBEAM`"]
    pub const IBEAM: CursorShape = CursorShape {
        ord: 1i32
    };
    #[doc(alias = "CURSOR_POINTING_HAND")]
    #[doc = "Godot enumerator name: `CURSOR_POINTING_HAND`"]
    pub const POINTING_HAND: CursorShape = CursorShape {
        ord: 2i32
    };
    #[doc(alias = "CURSOR_CROSS")]
    #[doc = "Godot enumerator name: `CURSOR_CROSS`"]
    pub const CROSS: CursorShape = CursorShape {
        ord: 3i32
    };
    #[doc(alias = "CURSOR_WAIT")]
    #[doc = "Godot enumerator name: `CURSOR_WAIT`"]
    pub const WAIT: CursorShape = CursorShape {
        ord: 4i32
    };
    #[doc(alias = "CURSOR_BUSY")]
    #[doc = "Godot enumerator name: `CURSOR_BUSY`"]
    pub const BUSY: CursorShape = CursorShape {
        ord: 5i32
    };
    #[doc(alias = "CURSOR_DRAG")]
    #[doc = "Godot enumerator name: `CURSOR_DRAG`"]
    pub const DRAG: CursorShape = CursorShape {
        ord: 6i32
    };
    #[doc(alias = "CURSOR_CAN_DROP")]
    #[doc = "Godot enumerator name: `CURSOR_CAN_DROP`"]
    pub const CAN_DROP: CursorShape = CursorShape {
        ord: 7i32
    };
    #[doc(alias = "CURSOR_FORBIDDEN")]
    #[doc = "Godot enumerator name: `CURSOR_FORBIDDEN`"]
    pub const FORBIDDEN: CursorShape = CursorShape {
        ord: 8i32
    };
    #[doc(alias = "CURSOR_VSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_VSIZE`"]
    pub const VSIZE: CursorShape = CursorShape {
        ord: 9i32
    };
    #[doc(alias = "CURSOR_HSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_HSIZE`"]
    pub const HSIZE: CursorShape = CursorShape {
        ord: 10i32
    };
    #[doc(alias = "CURSOR_BDIAGSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_BDIAGSIZE`"]
    pub const BDIAGSIZE: CursorShape = CursorShape {
        ord: 11i32
    };
    #[doc(alias = "CURSOR_FDIAGSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_FDIAGSIZE`"]
    pub const FDIAGSIZE: CursorShape = CursorShape {
        ord: 12i32
    };
    #[doc(alias = "CURSOR_MOVE")]
    #[doc = "Godot enumerator name: `CURSOR_MOVE`"]
    pub const MOVE: CursorShape = CursorShape {
        ord: 13i32
    };
    #[doc(alias = "CURSOR_VSPLIT")]
    #[doc = "Godot enumerator name: `CURSOR_VSPLIT`"]
    pub const VSPLIT: CursorShape = CursorShape {
        ord: 14i32
    };
    #[doc(alias = "CURSOR_HSPLIT")]
    #[doc = "Godot enumerator name: `CURSOR_HSPLIT`"]
    pub const HSPLIT: CursorShape = CursorShape {
        ord: 15i32
    };
    #[doc(alias = "CURSOR_HELP")]
    #[doc = "Godot enumerator name: `CURSOR_HELP`"]
    pub const HELP: CursorShape = CursorShape {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for CursorShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CursorShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CursorShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 => Some(Self {
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
            Self::ARROW => "ARROW", Self::IBEAM => "IBEAM", Self::POINTING_HAND => "POINTING_HAND", Self::CROSS => "CROSS", Self::WAIT => "WAIT", Self::BUSY => "BUSY", Self::DRAG => "DRAG", Self::CAN_DROP => "CAN_DROP", Self::FORBIDDEN => "FORBIDDEN", Self::VSIZE => "VSIZE", Self::HSIZE => "HSIZE", Self::BDIAGSIZE => "BDIAGSIZE", Self::FDIAGSIZE => "FDIAGSIZE", Self::MOVE => "MOVE", Self::VSPLIT => "VSPLIT", Self::HSPLIT => "HSPLIT", Self::HELP => "HELP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CursorShape::ARROW, CursorShape::IBEAM, CursorShape::POINTING_HAND, CursorShape::CROSS, CursorShape::WAIT, CursorShape::BUSY, CursorShape::DRAG, CursorShape::CAN_DROP, CursorShape::FORBIDDEN, CursorShape::VSIZE, CursorShape::HSIZE, CursorShape::BDIAGSIZE, CursorShape::FDIAGSIZE, CursorShape::MOVE, CursorShape::VSPLIT, CursorShape::HSPLIT, CursorShape::HELP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CursorShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ARROW", "CURSOR_ARROW", CursorShape::ARROW), crate::meta::inspect::EnumConstant::new("IBEAM", "CURSOR_IBEAM", CursorShape::IBEAM), crate::meta::inspect::EnumConstant::new("POINTING_HAND", "CURSOR_POINTING_HAND", CursorShape::POINTING_HAND), crate::meta::inspect::EnumConstant::new("CROSS", "CURSOR_CROSS", CursorShape::CROSS), crate::meta::inspect::EnumConstant::new("WAIT", "CURSOR_WAIT", CursorShape::WAIT), crate::meta::inspect::EnumConstant::new("BUSY", "CURSOR_BUSY", CursorShape::BUSY), crate::meta::inspect::EnumConstant::new("DRAG", "CURSOR_DRAG", CursorShape::DRAG), crate::meta::inspect::EnumConstant::new("CAN_DROP", "CURSOR_CAN_DROP", CursorShape::CAN_DROP), crate::meta::inspect::EnumConstant::new("FORBIDDEN", "CURSOR_FORBIDDEN", CursorShape::FORBIDDEN), crate::meta::inspect::EnumConstant::new("VSIZE", "CURSOR_VSIZE", CursorShape::VSIZE), crate::meta::inspect::EnumConstant::new("HSIZE", "CURSOR_HSIZE", CursorShape::HSIZE), crate::meta::inspect::EnumConstant::new("BDIAGSIZE", "CURSOR_BDIAGSIZE", CursorShape::BDIAGSIZE), crate::meta::inspect::EnumConstant::new("FDIAGSIZE", "CURSOR_FDIAGSIZE", CursorShape::FDIAGSIZE), crate::meta::inspect::EnumConstant::new("MOVE", "CURSOR_MOVE", CursorShape::MOVE), crate::meta::inspect::EnumConstant::new("VSPLIT", "CURSOR_VSPLIT", CursorShape::VSPLIT), crate::meta::inspect::EnumConstant::new("HSPLIT", "CURSOR_HSPLIT", CursorShape::HSPLIT), crate::meta::inspect::EnumConstant::new("HELP", "CURSOR_HELP", CursorShape::HELP)]
        }
    }
}
impl crate::meta::GodotConvert for CursorShape {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Cursor Arrow", 0i64), EnumeratorShape::new_int("Cursor Ibeam", 1i64), EnumeratorShape::new_int("Cursor Pointing Hand", 2i64), EnumeratorShape::new_int("Cursor Cross", 3i64), EnumeratorShape::new_int("Cursor Wait", 4i64), EnumeratorShape::new_int("Cursor Busy", 5i64), EnumeratorShape::new_int("Cursor Drag", 6i64), EnumeratorShape::new_int("Cursor Can Drop", 7i64), EnumeratorShape::new_int("Cursor Forbidden", 8i64), EnumeratorShape::new_int("Cursor Vsize", 9i64), EnumeratorShape::new_int("Cursor Hsize", 10i64), EnumeratorShape::new_int("Cursor Bdiagsize", 11i64), EnumeratorShape::new_int("Cursor Fdiagsize", 12i64), EnumeratorShape::new_int("Cursor Move", 13i64), EnumeratorShape::new_int("Cursor Vsplit", 14i64), EnumeratorShape::new_int("Cursor Hsplit", 15i64), EnumeratorShape::new_int("Cursor Help", 16i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Input.CursorShape")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CursorShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CursorShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CursorShape {
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
impl crate::registry::property::Export for CursorShape {
    
}
impl crate::meta::Element for CursorShape {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Input;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Input`][crate::classes::Input] class."]
    pub struct SignalsOfInput < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfInput < 'c, C > {
        #[doc = "Signature: `(device: i64, connected: bool)`"]
        pub fn joy_connection_changed(&mut self) -> SigJoyConnectionChanged < 'c, C > {
            SigJoyConnectionChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "joy_connection_changed")
            }
        }
    }
    type TypedSigJoyConnectionChanged < 'c, C > = TypedSignal < 'c, C, (i64, bool,) >;
    pub struct SigJoyConnectionChanged < 'c, C: WithSignals > {
        typed: TypedSigJoyConnectionChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigJoyConnectionChanged < 'c, C > {
        pub fn emit(&mut self, device: i64, connected: bool,) {
            self.typed.emit_tuple((device, connected,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigJoyConnectionChanged < 'c, C > {
        type Target = TypedSigJoyConnectionChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigJoyConnectionChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Input {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfInput < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfInput < 'c, C > {
        type Target = < < Input as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Input;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfInput < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Input;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
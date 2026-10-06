#![doc = "Sidecar module for class [`PopupMenu`][crate::classes::PopupMenu].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PopupMenu` enums](https://docs.godotengine.org/en/stable/classes/class_popupmenu.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PopupMenu`.\n\nInherits [`Popup`][crate::classes::Popup].\n\nRelated symbols:\n\n* [`popup_menu`][crate::classes::popup_menu]: sidecar module with related enum/flag types\n* [`IPopupMenu`][crate::classes::IPopupMenu]: virtual methods\n* [`SignalsOfPopupMenu`][crate::classes::popup_menu::SignalsOfPopupMenu]: signal collection\n\n\nSee also [Godot docs for `PopupMenu`](https://docs.godotengine.org/en/stable/classes/class_popupmenu.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`PopupMenu::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\n`PopupMenu` is a modal window used to display a list of options. Useful for toolbars and context menus.\n\nThe size of a `PopupMenu` can be limited by using \\[member Window.max_size]. If the height of the list of items is larger than the maximum height of the `PopupMenu`, a [`ScrollContainer`][crate::classes::ScrollContainer] within the popup will allow the user to scroll the contents. If no maximum size is set, or if it is set to `0`, the `PopupMenu` height will be limited by its parent rect.\n\nAll `set_*` methods allow negative item indices, i.e. `-1` to access the last item, `-2` to select the second-to-last item, and so on.\n\n**Incremental search:** Like [`ItemList`][crate::classes::ItemList] and [`Tree`][crate::classes::Tree], `PopupMenu` supports searching within the list while the control is focused. Press a key that matches the first letter of an item's name to select the first item starting with the given letter. After that point, there are two ways to perform incremental search: 1) Press the same key again before the timeout duration to select the next item starting with the same letter. 2) Press letter keys that match the rest of the word before the timeout duration to match to select the item in question directly. Both of these actions will be reset to the beginning of the list if the timeout duration has passed since the last keystroke was registered. You can adjust the timeout duration by changing \\[member ProjectSettings.gui/timers/incremental_search_max_interval_msec].\n\n**Note:** `PopupMenu` is invisible by default. To make it visible, call one of the `popup_*` methods from [`Window`][crate::classes::Window] on the node, such as [`popup_centered_clamped`][`crate::classes::Window::popup_centered_clamped`].\n\n**Note:** The ID values used for items are limited to 32 bits, not full 64 bits of `int`. This has a range of `-2^32` to `2^32 - 1`, i.e. `-2147483648` to `2147483647`."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PopupMenu {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PopupMenu`][crate::classes::PopupMenu].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IPopup`][crate::classes::IPopup] > [`IWindow`][crate::classes::IWindow] > ~~`IViewport`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `PopupMenu` methods](https://docs.godotengine.org/en/stable/classes/class_popupmenu.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPopupMenu: crate::obj::GodotClass < Base = PopupMenu > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = "Called when the node enters the [`SceneTree`][crate::classes::SceneTree] (e.g. upon instantiating, scene changing, or after calling [`add_child`][`crate::classes::Node::add_child`] in a script). If the node has children, its [`enter_tree`][`crate::classes::INode::enter_tree`] callback will be called first, and then that of the children.\n\nCorresponds to the [`NodeNotification::ENTER_TREE`][`crate::classes::notify::NodeNotification::ENTER_TREE`] notification in [`on_notification`][`crate::classes::IObject::on_notification`]."]
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the node is \"ready\", i.e. when both the node and its children have entered the scene tree. If the node has children, their [`ready`][`crate::classes::INode::ready`] callbacks get triggered first, and the parent node will receive the ready notification afterwards.\n\nCorresponds to the [`NodeNotification::READY`][`crate::classes::notify::NodeNotification::READY`] notification in [`on_notification`][`crate::classes::IObject::on_notification`]. See also the `@onready` annotation for variables.\n\nUsually used for initialization. For even earlier initialization, [`init`][`crate::classes::IObject::init`] may be used. See also [`enter_tree`][`crate::classes::INode::enter_tree`].\n\n**Note:** This method may be called only once for each node. After removing a node from the scene tree and adding it again, [`ready`][`crate::classes::INode::ready`] will **not** be called a second time. This can be bypassed by requesting another call with [`request_ready`][`crate::classes::Node::request_ready`], which may be called anywhere before adding the node again."]
        fn ready(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the node is about to leave the [`SceneTree`][crate::classes::SceneTree] (e.g. upon freeing, scene changing, or after calling [`remove_child`][`crate::classes::Node::remove_child`] in a script). If the node has children, its [`exit_tree`][`crate::classes::INode::exit_tree`] callback will be called last, after all its children have left the tree.\n\nCorresponds to the [`NodeNotification::EXIT_TREE`][`crate::classes::notify::NodeNotification::EXIT_TREE`] notification in [`on_notification`][`crate::classes::IObject::on_notification`] and signal `tree_exiting`. To get notified when the node has already left the active tree, connect to the `tree_exited`."]
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called on each idle frame, prior to rendering, and after physics ticks have been processed. `delta` is the time between frames in seconds.\n\nIt is only called if processing is enabled for this Node, which is done automatically if this method is overridden, and can be toggled with [`set_process`][`crate::classes::Node::set_process`].\n\nProcessing happens in order of \\[member process_priority], lower priority values are called first. Nodes with the same priority are processed in tree order, or top to bottom as seen in the editor (also known as pre-order traversal).\n\nCorresponds to the [`NodeNotification::PROCESS`][`crate::classes::notify::NodeNotification::PROCESS`] notification in [`on_notification`][`crate::classes::IObject::on_notification`].\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan).\n\n**Note:** When the engine is struggling and the frame rate is lowered, `delta` will increase. When `delta` is increased, it's capped at a maximum of \\[member Engine.time_scale] * \\[member Engine.max_physics_steps_per_frame] / \\[member Engine.physics_ticks_per_second]. As a result, accumulated `delta` may not represent real world time.\n\n**Note:** When `--fixed-fps` is enabled or the engine is running in Movie Maker mode (see [`MovieWriter`][crate::classes::MovieWriter]), process `delta` will always be the same for every frame, regardless of how much time the frame took to render.\n\n**Note:** Frame delta may be post-processed by \\[member OS.delta_smoothing] if this is enabled for the project."]
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        #[doc = "Called once on each physics tick, and allows Nodes to synchronize their logic with physics ticks. `delta` is the logical time between physics ticks in seconds and is equal to \\[member Engine.time_scale] / \\[member Engine.physics_ticks_per_second].\n\nIt is only called if physics processing is enabled for this Node, which is done automatically if this method is overridden, and can be toggled with [`set_physics_process`][`crate::classes::Node::set_physics_process`].\n\nProcessing happens in order of \\[member process_physics_priority], lower priority values are called first. Nodes with the same priority are processed in tree order, or top to bottom as seen in the editor (also known as pre-order traversal).\n\nCorresponds to the [`NodeNotification::PHYSICS_PROCESS`][`crate::classes::notify::NodeNotification::PHYSICS_PROCESS`] notification in [`on_notification`][`crate::classes::IObject::on_notification`].\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan).\n\n**Note:** Accumulated `delta` may diverge from real world seconds."]
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        #[doc = "Called when there is an input event. The input event propagates up through the node tree until a node consumes it.\n\nIt is only called if input processing is enabled, which is done automatically if this method is overridden, and can be toggled with [`set_process_input`][`crate::classes::Node::set_process_input`].\n\nTo consume the input event and stop it propagating further to other nodes, [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`] can be called.\n\nFor gameplay input, [`unhandled_input`][`crate::classes::INode::unhandled_input`] and [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`] are usually a better fit as they allow the GUI to intercept the events first.\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan)."]
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        #[doc = "Called when an [`InputEventKey`][crate::classes::InputEventKey], [`InputEventShortcut`][crate::classes::InputEventShortcut], or [`InputEventJoypadButton`][crate::classes::InputEventJoypadButton] hasn't been consumed by [`input`][`crate::classes::INode::input`] or any GUI [`Control`][crate::classes::Control] item. It is called before [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`] and [`unhandled_input`][`crate::classes::INode::unhandled_input`]. The input event propagates up through the node tree until a node consumes it.\n\nIt is only called if shortcut processing is enabled, which is done automatically if this method is overridden, and can be toggled with [`set_process_shortcut_input`][`crate::classes::Node::set_process_shortcut_input`].\n\nTo consume the input event and stop it propagating further to other nodes, [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`] can be called.\n\nThis method can be used to handle shortcuts. For generic GUI events, use [`input`][`crate::classes::INode::input`] instead. Gameplay events should usually be handled with either [`unhandled_input`][`crate::classes::INode::unhandled_input`] or [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`].\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not orphan)."]
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        #[doc = "Called when an [`InputEventKey`][crate::classes::InputEventKey] hasn't been consumed by [`input`][`crate::classes::INode::input`] or any GUI [`Control`][crate::classes::Control] item. It is called after [`shortcut_input`][`crate::classes::INode::shortcut_input`] but before [`unhandled_input`][`crate::classes::INode::unhandled_input`]. The input event propagates up through the node tree until a node consumes it.\n\nIt is only called if unhandled key input processing is enabled, which is done automatically if this method is overridden, and can be toggled with [`set_process_unhandled_key_input`][`crate::classes::Node::set_process_unhandled_key_input`].\n\nTo consume the input event and stop it propagating further to other nodes, [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`] can be called.\n\nThis method can be used to handle Unicode character input with `Alt`, `Alt + Ctrl`, and `Alt + Shift` modifiers, after shortcuts were handled.\n\nFor gameplay input, this and [`unhandled_input`][`crate::classes::INode::unhandled_input`] are usually a better fit than [`input`][`crate::classes::INode::input`], as GUI events should be handled first. This method also performs better than [`unhandled_input`][`crate::classes::INode::unhandled_input`], since unrelated events such as [`InputEventMouseMotion`][crate::classes::InputEventMouseMotion] are automatically filtered. For shortcuts, consider using [`shortcut_input`][`crate::classes::INode::shortcut_input`] instead.\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan)."]
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        #[doc = "Called when an [`InputEvent`][crate::classes::InputEvent] hasn't been consumed by [`input`][`crate::classes::INode::input`] or any GUI [`Control`][crate::classes::Control] item. It is called after [`shortcut_input`][`crate::classes::INode::shortcut_input`] and after [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`]. The input event propagates up through the node tree until a node consumes it.\n\nIt is only called if unhandled input processing is enabled, which is done automatically if this method is overridden, and can be toggled with [`set_process_unhandled_input`][`crate::classes::Node::set_process_unhandled_input`].\n\nTo consume the input event and stop it propagating further to other nodes, [`set_input_as_handled`][`crate::classes::Viewport::set_input_as_handled`] can be called.\n\nFor gameplay input, this method is usually a better fit than [`input`][`crate::classes::INode::input`], as GUI events need a higher priority. For keyboard shortcuts, consider using [`shortcut_input`][`crate::classes::INode::shortcut_input`] instead, as it is called before this method. Finally, to handle keyboard events, consider using [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`] for performance reasons.\n\n**Note:** This method is only called if the node is present in the scene tree (i.e. if it's not an orphan)."]
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: WindowNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn on_get(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn on_set(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `on_get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        fn on_validate_property(&self, property: &mut crate::registry::info::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        #[cfg_attr(published_docs, doc(cfg(since_api = "4.3")))]
        fn on_get_property_list(&mut self) -> Vec < crate::registry::info::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn on_property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = "Virtual method to be implemented by the user. Overrides the value returned by [`get_contents_minimum_size`][`crate::classes::Window::get_contents_minimum_size`]."]
        fn get_contents_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        #[doc = "The elements in the array returned from this method are displayed as warnings in the Scene dock if the script that overrides it is a `tool` script.\n\nReturning an empty array produces no warnings.\n\nCall [`update_configuration_warnings`][`crate::classes::Node::update_configuration_warnings`] when the warnings need to be updated for this node.\n\n```gdscript\n@export var energy = 0:\n\tset(value):\n\t\tenergy = value\n\t\tupdate_configuration_warnings()\n\nfunc _get_configuration_warnings():\n\tif energy < 0:\n\t\treturn [\"Energy must be 0 or greater.\"]\n\telse:\n\t\treturn []\n```"]
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "The elements in the array returned from this method are displayed as warnings in the Scene dock if the script that overrides it is a `tool` script, and accessibility warnings are enabled in the editor settings.\n\nReturning an empty array produces no warnings."]
        fn get_accessibility_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Called during accessibility information updates to determine the currently focused sub-element, should return a sub-element RID or the value returned by [`get_accessibility_element`][`crate::classes::Node::get_accessibility_element`]."]
        fn get_focused_accessibility_element(&self,) -> Rid {
            unimplemented !()
        }
    }
    impl PopupMenu {
        #[doc = "Checks the provided `event` against the `PopupMenu`'s shortcuts and accelerators, and activates the first item with matching events. If `for_global_only` is `true`, only shortcuts and accelerators with `global` set to `true` will be called.\n\nReturns `true` if an item was successfully activated.\n\n**Note:** Certain [`Control`][crate::classes::Control]s, such as [`MenuButton`][crate::classes::MenuButton], will call this method automatically."]
        pub(crate) fn activate_item_by_event_full(&mut self, event: CowArg < Option < Gd < crate::classes::InputEvent > > >, for_global_only: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::InputEvent > > >, bool,);
            let args = (event, for_global_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8980usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "activate_item_by_event", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`activate_item_by_event_ex`][Self::activate_item_by_event_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Checks the provided `event` against the `PopupMenu`'s shortcuts and accelerators, and activates the first item with matching events. If `for_global_only` is `true`, only shortcuts and accelerators with `global` set to `true` will be called.\n\nReturns `true` if an item was successfully activated.\n\n**Note:** Certain [`Control`][crate::classes::Control]s, such as [`MenuButton`][crate::classes::MenuButton], will call this method automatically."]
        #[inline]
        pub fn activate_item_by_event(&mut self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) -> bool {
            self.activate_item_by_event_ex(event,) . done()
        }
        #[doc = "Checks the provided `event` against the `PopupMenu`'s shortcuts and accelerators, and activates the first item with matching events. If `for_global_only` is `true`, only shortcuts and accelerators with `global` set to `true` will be called.\n\nReturns `true` if an item was successfully activated.\n\n**Note:** Certain [`Control`][crate::classes::Control]s, such as [`MenuButton`][crate::classes::MenuButton], will call this method automatically."]
        #[inline]
        pub fn activate_item_by_event_ex < 'ex > (&'ex mut self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'ex,) -> ExActivateItemByEvent < 'ex > {
            ExActivateItemByEvent::new(self, event,)
        }
        pub fn set_prefer_native_menu(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8981usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_prefer_native_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_prefer_native_menu(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "is_prefer_native_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the system native menu is supported and currently used by this `PopupMenu`."]
        pub fn is_native_menu(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "is_native_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a new item with text `label`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n**Note:** The provided `id` is used only in `id_pressed` and `id_focused` signals. It's not related to the `index` arguments in e.g. [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`]."]
        pub(crate) fn add_item_full(&mut self, label: CowArg < GString >, id: i32, accel: crate::global::Key,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, crate::global::Key,);
            let args = (label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_item_ex`][Self::add_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new item with text `label`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n**Note:** The provided `id` is used only in `id_pressed` and `id_focused` signals. It's not related to the `index` arguments in e.g. [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`]."]
        #[inline]
        pub fn add_item(&mut self, label: impl AsArg < GString >,) {
            self.add_item_ex(label,) . done()
        }
        #[doc = "Adds a new item with text `label`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n**Note:** The provided `id` is used only in `id_pressed` and `id_focused` signals. It's not related to the `index` arguments in e.g. [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`]."]
        #[inline]
        pub fn add_item_ex < 'ex > (&'ex mut self, label: impl AsArg < GString > + 'ex,) -> ExAddItem < 'ex > {
            ExAddItem::new(self, label,)
        }
        #[doc = "Adds a new item with text `label` and icon `texture`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators."]
        pub(crate) fn add_icon_item_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, id: i32, accel: crate::global::Key,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, i32, crate::global::Key,);
            let args = (texture, label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_icon_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_icon_item_ex`][Self::add_icon_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new item with text `label` and icon `texture`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators."]
        #[inline]
        pub fn add_icon_item(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) {
            self.add_icon_item_ex(texture, label,) . done()
        }
        #[doc = "Adds a new item with text `label` and icon `texture`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators."]
        #[inline]
        pub fn add_icon_item_ex < 'ex > (&'ex mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExAddIconItem < 'ex > {
            ExAddIconItem::new(self, texture, label,)
        }
        #[doc = "Adds a new checkable item with text `label`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        pub(crate) fn add_check_item_full(&mut self, label: CowArg < GString >, id: i32, accel: crate::global::Key,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, crate::global::Key,);
            let args = (label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_check_item_ex`][Self::add_check_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new checkable item with text `label`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        #[inline]
        pub fn add_check_item(&mut self, label: impl AsArg < GString >,) {
            self.add_check_item_ex(label,) . done()
        }
        #[doc = "Adds a new checkable item with text `label`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        #[inline]
        pub fn add_check_item_ex < 'ex > (&'ex mut self, label: impl AsArg < GString > + 'ex,) -> ExAddCheckItem < 'ex > {
            ExAddCheckItem::new(self, label,)
        }
        #[doc = "Adds a new checkable item with text `label` and icon `texture`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        pub(crate) fn add_icon_check_item_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, id: i32, accel: crate::global::Key,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, i32, crate::global::Key,);
            let args = (texture, label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_icon_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_icon_check_item_ex`][Self::add_icon_check_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new checkable item with text `label` and icon `texture`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        #[inline]
        pub fn add_icon_check_item(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) {
            self.add_icon_check_item_ex(texture, label,) . done()
        }
        #[doc = "Adds a new checkable item with text `label` and icon `texture`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        #[inline]
        pub fn add_icon_check_item_ex < 'ex > (&'ex mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExAddIconCheckItem < 'ex > {
            ExAddIconCheckItem::new(self, texture, label,)
        }
        #[doc = "Adds a new radio check button with text `label`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        pub(crate) fn add_radio_check_item_full(&mut self, label: CowArg < GString >, id: i32, accel: crate::global::Key,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, crate::global::Key,);
            let args = (label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_radio_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_radio_check_item_ex`][Self::add_radio_check_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new radio check button with text `label`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        #[inline]
        pub fn add_radio_check_item(&mut self, label: impl AsArg < GString >,) {
            self.add_radio_check_item_ex(label,) . done()
        }
        #[doc = "Adds a new radio check button with text `label`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        #[inline]
        pub fn add_radio_check_item_ex < 'ex > (&'ex mut self, label: impl AsArg < GString > + 'ex,) -> ExAddRadioCheckItem < 'ex > {
            ExAddRadioCheckItem::new(self, label,)
        }
        #[doc = "Same as [`add_icon_check_item`][`crate::classes::PopupMenu::add_icon_check_item`], but uses a radio check button."]
        pub(crate) fn add_icon_radio_check_item_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, id: i32, accel: crate::global::Key,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, i32, crate::global::Key,);
            let args = (texture, label, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_icon_radio_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_icon_radio_check_item_ex`][Self::add_icon_radio_check_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Same as [`add_icon_check_item`][`crate::classes::PopupMenu::add_icon_check_item`], but uses a radio check button."]
        #[inline]
        pub fn add_icon_radio_check_item(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) {
            self.add_icon_radio_check_item_ex(texture, label,) . done()
        }
        #[doc = "Same as [`add_icon_check_item`][`crate::classes::PopupMenu::add_icon_check_item`], but uses a radio check button."]
        #[inline]
        pub fn add_icon_radio_check_item_ex < 'ex > (&'ex mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExAddIconRadioCheckItem < 'ex > {
            ExAddIconRadioCheckItem::new(self, texture, label,)
        }
        #[doc = "Adds a new multistate item with text `label`.\n\nContrarily to normal binary items, multistate items can have more than two states, as defined by `max_states`. The default value is defined by `default_state`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n```gdscript\nfunc _ready():\n\tadd_multistate_item(\"Item\", 3, 0)\n\n\tindex_pressed.connect(func(index: int):\n\t\t\ttoggle_item_multistate(index)\n\t\t\tmatch get_item_multistate(index):\n\t\t\t\t0:\n\t\t\t\t\tprint(\"First state\")\n\t\t\t\t1:\n\t\t\t\t\tprint(\"Second state\")\n\t\t\t\t2:\n\t\t\t\t\tprint(\"Third state\")\n\t\t)\n```\n\n**Note:** Multistate items don't update their state automatically and must be done manually. See [`toggle_item_multistate`][`crate::classes::PopupMenu::toggle_item_multistate`], [`set_item_multistate`][`crate::classes::PopupMenu::set_item_multistate`] and [`get_item_multistate`][`crate::classes::PopupMenu::get_item_multistate`] for more info on how to control it."]
        pub(crate) fn add_multistate_item_full(&mut self, label: CowArg < GString >, max_states: i32, default_state: i32, id: i32, accel: crate::global::Key,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32, i32, crate::global::Key,);
            let args = (label, max_states, default_state, id, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_multistate_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_multistate_item_ex`][Self::add_multistate_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new multistate item with text `label`.\n\nContrarily to normal binary items, multistate items can have more than two states, as defined by `max_states`. The default value is defined by `default_state`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n```gdscript\nfunc _ready():\n\tadd_multistate_item(\"Item\", 3, 0)\n\n\tindex_pressed.connect(func(index: int):\n\t\t\ttoggle_item_multistate(index)\n\t\t\tmatch get_item_multistate(index):\n\t\t\t\t0:\n\t\t\t\t\tprint(\"First state\")\n\t\t\t\t1:\n\t\t\t\t\tprint(\"Second state\")\n\t\t\t\t2:\n\t\t\t\t\tprint(\"Third state\")\n\t\t)\n```\n\n**Note:** Multistate items don't update their state automatically and must be done manually. See [`toggle_item_multistate`][`crate::classes::PopupMenu::toggle_item_multistate`], [`set_item_multistate`][`crate::classes::PopupMenu::set_item_multistate`] and [`get_item_multistate`][`crate::classes::PopupMenu::get_item_multistate`] for more info on how to control it."]
        #[inline]
        pub fn add_multistate_item(&mut self, label: impl AsArg < GString >, max_states: i32,) {
            self.add_multistate_item_ex(label, max_states,) . done()
        }
        #[doc = "Adds a new multistate item with text `label`.\n\nContrarily to normal binary items, multistate items can have more than two states, as defined by `max_states`. The default value is defined by `default_state`.\n\nAn `id` can optionally be provided, as well as an accelerator (`accel`). If no `id` is provided, one will be created from the index. If no `accel` is provided, then the default value of 0 (corresponding to `@GlobalScope.KEY_NONE`) will be assigned to the item (which means it won't have any accelerator). See [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] for more info on accelerators.\n\n```gdscript\nfunc _ready():\n\tadd_multistate_item(\"Item\", 3, 0)\n\n\tindex_pressed.connect(func(index: int):\n\t\t\ttoggle_item_multistate(index)\n\t\t\tmatch get_item_multistate(index):\n\t\t\t\t0:\n\t\t\t\t\tprint(\"First state\")\n\t\t\t\t1:\n\t\t\t\t\tprint(\"Second state\")\n\t\t\t\t2:\n\t\t\t\t\tprint(\"Third state\")\n\t\t)\n```\n\n**Note:** Multistate items don't update their state automatically and must be done manually. See [`toggle_item_multistate`][`crate::classes::PopupMenu::toggle_item_multistate`], [`set_item_multistate`][`crate::classes::PopupMenu::set_item_multistate`] and [`get_item_multistate`][`crate::classes::PopupMenu::get_item_multistate`] for more info on how to control it."]
        #[inline]
        pub fn add_multistate_item_ex < 'ex > (&'ex mut self, label: impl AsArg < GString > + 'ex, max_states: i32,) -> ExAddMultistateItem < 'ex > {
            ExAddMultistateItem::new(self, label, max_states,)
        }
        #[doc = "Adds a [`Shortcut`][crate::classes::Shortcut].\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\nIf `allow_echo` is `true`, the shortcut can be activated with echo events."]
        pub(crate) fn add_shortcut_full(&mut self, shortcut: CowArg < Option < Gd < crate::classes::Shortcut > > >, id: i32, global: bool, allow_echo: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Shortcut > > >, i32, bool, bool,);
            let args = (shortcut, id, global, allow_echo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_shortcut_ex`][Self::add_shortcut_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a [`Shortcut`][crate::classes::Shortcut].\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\nIf `allow_echo` is `true`, the shortcut can be activated with echo events."]
        #[inline]
        pub fn add_shortcut(&mut self, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> >,) {
            self.add_shortcut_ex(shortcut,) . done()
        }
        #[doc = "Adds a [`Shortcut`][crate::classes::Shortcut].\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\nIf `allow_echo` is `true`, the shortcut can be activated with echo events."]
        #[inline]
        pub fn add_shortcut_ex < 'ex > (&'ex mut self, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> ExAddShortcut < 'ex > {
            ExAddShortcut::new(self, shortcut,)
        }
        #[doc = "Adds a new item and assigns the specified [`Shortcut`][crate::classes::Shortcut] and icon `texture` to it. Sets the label of the checkbox to the [`Shortcut`][crate::classes::Shortcut]'s name.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\nIf `allow_echo` is `true`, the shortcut can be activated with echo events."]
        pub(crate) fn add_icon_shortcut_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, shortcut: CowArg < Option < Gd < crate::classes::Shortcut > > >, id: i32, global: bool, allow_echo: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, Option < Gd < crate::classes::Shortcut > > >, i32, bool, bool,);
            let args = (texture, shortcut, id, global, allow_echo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_icon_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_icon_shortcut_ex`][Self::add_icon_shortcut_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new item and assigns the specified [`Shortcut`][crate::classes::Shortcut] and icon `texture` to it. Sets the label of the checkbox to the [`Shortcut`][crate::classes::Shortcut]'s name.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\nIf `allow_echo` is `true`, the shortcut can be activated with echo events."]
        #[inline]
        pub fn add_icon_shortcut(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> >,) {
            self.add_icon_shortcut_ex(texture, shortcut,) . done()
        }
        #[doc = "Adds a new item and assigns the specified [`Shortcut`][crate::classes::Shortcut] and icon `texture` to it. Sets the label of the checkbox to the [`Shortcut`][crate::classes::Shortcut]'s name.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\nIf `allow_echo` is `true`, the shortcut can be activated with echo events."]
        #[inline]
        pub fn add_icon_shortcut_ex < 'ex > (&'ex mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> ExAddIconShortcut < 'ex > {
            ExAddIconShortcut::new(self, texture, shortcut,)
        }
        #[doc = "Adds a new checkable item and assigns the specified [`Shortcut`][crate::classes::Shortcut] to it. Sets the label of the checkbox to the [`Shortcut`][crate::classes::Shortcut]'s name.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        pub(crate) fn add_check_shortcut_full(&mut self, shortcut: CowArg < Option < Gd < crate::classes::Shortcut > > >, id: i32, global: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Shortcut > > >, i32, bool,);
            let args = (shortcut, id, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_check_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_check_shortcut_ex`][Self::add_check_shortcut_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new checkable item and assigns the specified [`Shortcut`][crate::classes::Shortcut] to it. Sets the label of the checkbox to the [`Shortcut`][crate::classes::Shortcut]'s name.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        #[inline]
        pub fn add_check_shortcut(&mut self, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> >,) {
            self.add_check_shortcut_ex(shortcut,) . done()
        }
        #[doc = "Adds a new checkable item and assigns the specified [`Shortcut`][crate::classes::Shortcut] to it. Sets the label of the checkbox to the [`Shortcut`][crate::classes::Shortcut]'s name.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        #[inline]
        pub fn add_check_shortcut_ex < 'ex > (&'ex mut self, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> ExAddCheckShortcut < 'ex > {
            ExAddCheckShortcut::new(self, shortcut,)
        }
        #[doc = "Adds a new checkable item and assigns the specified [`Shortcut`][crate::classes::Shortcut] and icon `texture` to it. Sets the label of the checkbox to the [`Shortcut`][crate::classes::Shortcut]'s name.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        pub(crate) fn add_icon_check_shortcut_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, shortcut: CowArg < Option < Gd < crate::classes::Shortcut > > >, id: i32, global: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, Option < Gd < crate::classes::Shortcut > > >, i32, bool,);
            let args = (texture, shortcut, id, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_icon_check_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_icon_check_shortcut_ex`][Self::add_icon_check_shortcut_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new checkable item and assigns the specified [`Shortcut`][crate::classes::Shortcut] and icon `texture` to it. Sets the label of the checkbox to the [`Shortcut`][crate::classes::Shortcut]'s name.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        #[inline]
        pub fn add_icon_check_shortcut(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> >,) {
            self.add_icon_check_shortcut_ex(texture, shortcut,) . done()
        }
        #[doc = "Adds a new checkable item and assigns the specified [`Shortcut`][crate::classes::Shortcut] and icon `texture` to it. Sets the label of the checkbox to the [`Shortcut`][crate::classes::Shortcut]'s name.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        #[inline]
        pub fn add_icon_check_shortcut_ex < 'ex > (&'ex mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> ExAddIconCheckShortcut < 'ex > {
            ExAddIconCheckShortcut::new(self, texture, shortcut,)
        }
        #[doc = "Adds a new radio check button and assigns a [`Shortcut`][crate::classes::Shortcut] to it. Sets the label of the checkbox to the [`Shortcut`][crate::classes::Shortcut]'s name.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        pub(crate) fn add_radio_check_shortcut_full(&mut self, shortcut: CowArg < Option < Gd < crate::classes::Shortcut > > >, id: i32, global: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Shortcut > > >, i32, bool,);
            let args = (shortcut, id, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_radio_check_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_radio_check_shortcut_ex`][Self::add_radio_check_shortcut_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new radio check button and assigns a [`Shortcut`][crate::classes::Shortcut] to it. Sets the label of the checkbox to the [`Shortcut`][crate::classes::Shortcut]'s name.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        #[inline]
        pub fn add_radio_check_shortcut(&mut self, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> >,) {
            self.add_radio_check_shortcut_ex(shortcut,) . done()
        }
        #[doc = "Adds a new radio check button and assigns a [`Shortcut`][crate::classes::Shortcut] to it. Sets the label of the checkbox to the [`Shortcut`][crate::classes::Shortcut]'s name.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::PopupMenu::set_item_checked`] for more info on how to control it."]
        #[inline]
        pub fn add_radio_check_shortcut_ex < 'ex > (&'ex mut self, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> ExAddRadioCheckShortcut < 'ex > {
            ExAddRadioCheckShortcut::new(self, shortcut,)
        }
        #[doc = "Same as [`add_icon_check_shortcut`][`crate::classes::PopupMenu::add_icon_check_shortcut`], but uses a radio check button."]
        pub(crate) fn add_icon_radio_check_shortcut_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, shortcut: CowArg < Option < Gd < crate::classes::Shortcut > > >, id: i32, global: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, Option < Gd < crate::classes::Shortcut > > >, i32, bool,);
            let args = (texture, shortcut, id, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_icon_radio_check_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_icon_radio_check_shortcut_ex`][Self::add_icon_radio_check_shortcut_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Same as [`add_icon_check_shortcut`][`crate::classes::PopupMenu::add_icon_check_shortcut`], but uses a radio check button."]
        #[inline]
        pub fn add_icon_radio_check_shortcut(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> >,) {
            self.add_icon_radio_check_shortcut_ex(texture, shortcut,) . done()
        }
        #[doc = "Same as [`add_icon_check_shortcut`][`crate::classes::PopupMenu::add_icon_check_shortcut`], but uses a radio check button."]
        #[inline]
        pub fn add_icon_radio_check_shortcut_ex < 'ex > (&'ex mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> ExAddIconRadioCheckShortcut < 'ex > {
            ExAddIconRadioCheckShortcut::new(self, texture, shortcut,)
        }
        #[doc = "Adds an item that will act as a submenu of the parent `PopupMenu` node when clicked. The `submenu` argument must be the name of an existing `PopupMenu` that has been added as a child to this node. This submenu will be shown when the item is clicked, hovered for long enough, or activated using the `ui_select` or `ui_right` input actions.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index."]
        pub(crate) fn add_submenu_item_full(&mut self, label: CowArg < GString >, submenu: CowArg < GString >, id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32,);
            let args = (label, submenu, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_submenu_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_submenu_item_ex`][Self::add_submenu_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an item that will act as a submenu of the parent `PopupMenu` node when clicked. The `submenu` argument must be the name of an existing `PopupMenu` that has been added as a child to this node. This submenu will be shown when the item is clicked, hovered for long enough, or activated using the `ui_select` or `ui_right` input actions.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index."]
        #[inline]
        pub fn add_submenu_item(&mut self, label: impl AsArg < GString >, submenu: impl AsArg < GString >,) {
            self.add_submenu_item_ex(label, submenu,) . done()
        }
        #[doc = "Adds an item that will act as a submenu of the parent `PopupMenu` node when clicked. The `submenu` argument must be the name of an existing `PopupMenu` that has been added as a child to this node. This submenu will be shown when the item is clicked, hovered for long enough, or activated using the `ui_select` or `ui_right` input actions.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index."]
        #[inline]
        pub fn add_submenu_item_ex < 'ex > (&'ex mut self, label: impl AsArg < GString > + 'ex, submenu: impl AsArg < GString > + 'ex,) -> ExAddSubmenuItem < 'ex > {
            ExAddSubmenuItem::new(self, label, submenu,)
        }
        #[doc = "Adds an item that will act as a submenu of the parent `PopupMenu` node when clicked. This submenu will be shown when the item is clicked, hovered for long enough, or activated using the `ui_select` or `ui_right` input actions.\n\n`submenu` must be either child of this `PopupMenu` or has no parent node (in which case it will be automatically added as a child). If the `submenu` popup has another parent, this method will fail.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index."]
        pub(crate) fn add_submenu_node_item_full(&mut self, label: CowArg < GString >, submenu: CowArg < Option < Gd < crate::classes::PopupMenu > > >, id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::PopupMenu > > >, i32,);
            let args = (label, submenu, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_submenu_node_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_submenu_node_item_ex`][Self::add_submenu_node_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an item that will act as a submenu of the parent `PopupMenu` node when clicked. This submenu will be shown when the item is clicked, hovered for long enough, or activated using the `ui_select` or `ui_right` input actions.\n\n`submenu` must be either child of this `PopupMenu` or has no parent node (in which case it will be automatically added as a child). If the `submenu` popup has another parent, this method will fail.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index."]
        #[inline]
        pub fn add_submenu_node_item(&mut self, label: impl AsArg < GString >, submenu: impl AsArg < Option < Gd < crate::classes::PopupMenu >> >,) {
            self.add_submenu_node_item_ex(label, submenu,) . done()
        }
        #[doc = "Adds an item that will act as a submenu of the parent `PopupMenu` node when clicked. This submenu will be shown when the item is clicked, hovered for long enough, or activated using the `ui_select` or `ui_right` input actions.\n\n`submenu` must be either child of this `PopupMenu` or has no parent node (in which case it will be automatically added as a child). If the `submenu` popup has another parent, this method will fail.\n\nAn `id` can optionally be provided. If no `id` is provided, one will be created from the index."]
        #[inline]
        pub fn add_submenu_node_item_ex < 'ex > (&'ex mut self, label: impl AsArg < GString > + 'ex, submenu: impl AsArg < Option < Gd < crate::classes::PopupMenu >> > + 'ex,) -> ExAddSubmenuNodeItem < 'ex > {
            ExAddSubmenuNodeItem::new(self, label, submenu,)
        }
        #[doc = "Sets the text of the item at the given `index`."]
        pub fn set_item_text(&mut self, index: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (index, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets item's text base writing direction."]
        pub fn set_item_text_direction(&mut self, index: i32, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::control::TextDirection,);
            let args = (index, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the language code of the text for the item at the given index to `language`. This is used for line-breaking and text shaping algorithms. If `language` is empty, the current locale is used."]
        pub fn set_item_language(&mut self, index: i32, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (index, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the auto translate mode of the item at the given `index`.\n\nItems use [`AutoTranslateMode::INHERIT`][`crate::classes::node::AutoTranslateMode::INHERIT`] by default, which uses the same auto translate mode as the `PopupMenu` itself."]
        pub fn set_item_auto_translate_mode(&mut self, index: i32, mode: crate::classes::node::AutoTranslateMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::node::AutoTranslateMode,);
            let args = (index, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Replaces the [`Texture2D`][crate::classes::Texture2D] icon of the item at the given `index`."]
        pub fn set_item_icon(&mut self, index: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (index, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the maximum allowed width of the icon for the item at the given `index`. This limit is applied on top of the default size of the icon and on top of [theme_item icon_max_width]. The height is adjusted according to the icon's ratio."]
        pub fn set_item_icon_max_width(&mut self, index: i32, width: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_icon_max_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a modulating [`Color`][crate::builtin::Color] of the item's icon at the given `index`."]
        pub fn set_item_icon_modulate(&mut self, index: i32, modulate: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (index, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_icon_modulate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the checkstate status of the item at the given `index`."]
        pub fn set_item_checked(&mut self, index: i32, checked: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, checked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_checked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `id` of the item at the given `index`.\n\nThe `id` is used in `id_pressed` and `id_focused` signals."]
        pub fn set_item_id(&mut self, index: i32, id: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the accelerator of the item at the given `index`. An accelerator is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. `accel` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`)."]
        pub fn set_item_accelerator(&mut self, index: i32, accel: crate::global::Key,) {
            type CallRet = ();
            type CallParams = (i32, crate::global::Key,);
            let args = (index, accel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_accelerator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the metadata of an item, which may be of any type. You can later get it with [`get_item_metadata`][`crate::classes::PopupMenu::get_item_metadata`], which provides a simple way of assigning context data to items."]
        pub fn set_item_metadata(&mut self, index: i32, metadata: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (index, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9009usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables/disables the item at the given `index`. When it is disabled, it can't be selected and its action can't be invoked."]
        pub fn set_item_disabled(&mut self, index: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9010usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the submenu of the item at the given `index`. The submenu is the name of a child `PopupMenu` node that would be shown when the item is clicked."]
        pub fn set_item_submenu(&mut self, index: i32, submenu: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (index, submenu.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9011usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_submenu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the submenu of the item at the given `index`. The submenu is a `PopupMenu` node that would be shown when the item is clicked. It must either be a child of this `PopupMenu` or has no parent (in which case it will be automatically added as a child). If the `submenu` popup has another parent, this method will fail."]
        pub fn set_item_submenu_node(&mut self, index: i32, submenu: impl AsArg < Option < Gd < crate::classes::PopupMenu >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::PopupMenu > > >,);
            let args = (index, submenu.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_submenu_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Mark the item at the given `index` as a separator, which means that it would be displayed as a line. If `false`, sets the type of the item to plain text."]
        pub fn set_item_as_separator(&mut self, index: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9013usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_as_separator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether the item at the given `index` has a checkbox. If `false`, sets the type of the item to plain text.\n\n**Note:** Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually."]
        pub fn set_item_as_checkable(&mut self, index: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9014usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_as_checkable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the type of the item at the given `index` to radio button. If `false`, sets the type of the item to plain text."]
        pub fn set_item_as_radio_checkable(&mut self, index: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9015usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_as_radio_checkable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`String`][crate::builtin::GString] tooltip of the item at the given `index`."]
        pub fn set_item_tooltip(&mut self, index: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (index, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a [`Shortcut`][crate::classes::Shortcut] for the item at the given `index`."]
        pub(crate) fn set_item_shortcut_full(&mut self, index: i32, shortcut: CowArg < Option < Gd < crate::classes::Shortcut > > >, global: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Shortcut > > >, bool,);
            let args = (index, shortcut, global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_item_shortcut_ex`][Self::set_item_shortcut_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets a [`Shortcut`][crate::classes::Shortcut] for the item at the given `index`."]
        #[inline]
        pub fn set_item_shortcut(&mut self, index: i32, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> >,) {
            self.set_item_shortcut_ex(index, shortcut,) . done()
        }
        #[doc = "Sets a [`Shortcut`][crate::classes::Shortcut] for the item at the given `index`."]
        #[inline]
        pub fn set_item_shortcut_ex < 'ex > (&'ex mut self, index: i32, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> ExSetItemShortcut < 'ex > {
            ExSetItemShortcut::new(self, index, shortcut,)
        }
        #[doc = "Sets the horizontal offset of the item at the given `index`."]
        pub fn set_item_indent(&mut self, index: i32, indent: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, indent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9018usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_indent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the state of a multistate item. See [`add_multistate_item`][`crate::classes::PopupMenu::add_multistate_item`] for details."]
        pub fn set_item_multistate(&mut self, index: i32, state: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9019usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_multistate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the max states of a multistate item. See [`add_multistate_item`][`crate::classes::PopupMenu::add_multistate_item`] for details."]
        pub fn set_item_multistate_max(&mut self, index: i32, max_states: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, max_states,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9020usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_multistate_max", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Disables the [`Shortcut`][crate::classes::Shortcut] of the item at the given `index`."]
        pub fn set_item_shortcut_disabled(&mut self, index: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9021usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_shortcut_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Toggles the check state of the item at the given `index`."]
        pub fn toggle_item_checked(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9022usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "toggle_item_checked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Cycle to the next state of a multistate item. See [`add_multistate_item`][`crate::classes::PopupMenu::add_multistate_item`] for details."]
        pub fn toggle_item_multistate(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9023usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "toggle_item_multistate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text of the item at the given `index`."]
        pub fn get_item_text(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9024usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns item's text base writing direction."]
        pub fn get_item_text_direction(&self, index: i32,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns item's text language code."]
        pub fn get_item_language(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the auto translate mode of the item at the given `index`."]
        pub fn get_item_auto_translate_mode(&self, index: i32,) -> crate::classes::node::AutoTranslateMode {
            type CallRet = crate::classes::node::AutoTranslateMode;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the icon of the item at the given `index`."]
        pub fn get_item_icon(&self, index: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the maximum allowed width of the icon for the item at the given `index`."]
        pub fn get_item_icon_max_width(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_icon_max_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Color`][crate::builtin::Color] modulating the item's icon at the given `index`."]
        pub fn get_item_icon_modulate(&self, index: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_icon_modulate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at the given `index` is checked."]
        pub fn is_item_checked(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "is_item_checked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID of the item at the given `index`. `id` can be manually assigned, while index can not."]
        pub fn get_item_id(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the item containing the specified `id`. Index is automatically assigned to each item by the engine and can not be set manually."]
        pub fn get_item_index(&self, id: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the accelerator of the item at the given `index`. An accelerator is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The return value is an integer which is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`). If no accelerator is defined for the specified `index`, [`get_item_accelerator`][`crate::classes::PopupMenu::get_item_accelerator`] returns `0` (corresponding to `@GlobalScope.KEY_NONE`)."]
        pub fn get_item_accelerator(&self, index: i32,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_accelerator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the metadata of the specified item, which might be of any type. You can set it with [`set_item_metadata`][`crate::classes::PopupMenu::set_item_metadata`], which provides a simple way of assigning context data to items."]
        pub fn get_item_metadata(&self, index: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at the given `index` is disabled. When it is disabled it can't be selected, or its action invoked.\n\nSee [`set_item_disabled`][`crate::classes::PopupMenu::set_item_disabled`] for more info on how to disable an item."]
        pub fn is_item_disabled(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "is_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the submenu name of the item at the given `index`. See [`add_submenu_item`][`crate::classes::PopupMenu::add_submenu_item`] for more info on how to add a submenu."]
        pub fn get_item_submenu(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_submenu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the submenu of the item at the given `index`, or `null` if no submenu was added. See [`add_submenu_node_item`][`crate::classes::PopupMenu::add_submenu_node_item`] for more info on how to add a submenu."]
        pub fn get_item_submenu_node(&self, index: i32,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallRet = Option < Gd < crate::classes::PopupMenu > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_submenu_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item is a separator. If it is, it will be displayed as a line. See [`add_separator`][`crate::classes::PopupMenu::add_separator`] for more info on how to add a separator."]
        pub fn is_item_separator(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "is_item_separator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at the given `index` is checkable in some way, i.e. if it has a checkbox or radio button.\n\n**Note:** Checkable items just display a checkmark or radio button, but don't have any built-in checking behavior and must be checked/unchecked manually."]
        pub fn is_item_checkable(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9040usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "is_item_checkable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at the given `index` has radio button-style checkability.\n\n**Note:** This is purely cosmetic; you must add the logic for checking/unchecking items in radio groups."]
        pub fn is_item_radio_checkable(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9041usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "is_item_radio_checkable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the specified item's shortcut is disabled."]
        pub fn is_item_shortcut_disabled(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9042usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "is_item_shortcut_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tooltip associated with the item at the given `index`."]
        pub fn get_item_tooltip(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9043usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Shortcut`][crate::classes::Shortcut] associated with the item at the given `index`."]
        pub fn get_item_shortcut(&self, index: i32,) -> Option < Gd < crate::classes::Shortcut > > {
            type CallRet = Option < Gd < crate::classes::Shortcut > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9044usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the horizontal offset of the item at the given `index`."]
        pub fn get_item_indent(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9045usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_indent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the max states of the item at the given `index`."]
        pub fn get_item_multistate_max(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9046usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_multistate_max", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the state of the item at the given `index`."]
        pub fn get_item_multistate(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9047usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_multistate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the currently focused item as the given `index`.\n\nPassing `-1` as the index makes so that no item is focused."]
        pub fn set_focused_item(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9048usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_focused_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the currently focused item. Returns `-1` if no item is focused."]
        pub fn get_focused_item(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9049usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_focused_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9050usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_item_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9051usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_item_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves the scroll view to make the item at the given `index` visible."]
        pub fn scroll_to_item(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9052usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "scroll_to_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the item at the given `index` from the menu.\n\n**Note:** The indices of items after the removed item will be shifted by one."]
        pub fn remove_item(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9053usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "remove_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a separator between items. Separators also occupy an index, which you can set by using the `id` parameter.\n\nA `label` can optionally be provided, which will appear at the center of the separator."]
        pub(crate) fn add_separator_full(&mut self, label: CowArg < GString >, id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (label, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9054usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "add_separator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_separator_ex`][Self::add_separator_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a separator between items. Separators also occupy an index, which you can set by using the `id` parameter.\n\nA `label` can optionally be provided, which will appear at the center of the separator."]
        #[inline]
        pub fn add_separator(&mut self,) {
            self.add_separator_ex() . done()
        }
        #[doc = "Adds a separator between items. Separators also occupy an index, which you can set by using the `id` parameter.\n\nA `label` can optionally be provided, which will appear at the center of the separator."]
        #[inline]
        pub fn add_separator_ex < 'ex > (&'ex mut self,) -> ExAddSeparator < 'ex > {
            ExAddSeparator::new(self,)
        }
        #[doc = "Removes all items from the `PopupMenu`. If `free_submenus` is `true`, the submenu nodes are automatically freed."]
        pub(crate) fn clear_full(&mut self, free_submenus: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (free_submenus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9055usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`clear_ex`][Self::clear_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Removes all items from the `PopupMenu`. If `free_submenus` is `true`, the submenu nodes are automatically freed."]
        #[inline]
        pub fn clear(&mut self,) {
            self.clear_ex() . done()
        }
        #[doc = "Removes all items from the `PopupMenu`. If `free_submenus` is `true`, the submenu nodes are automatically freed."]
        #[inline]
        pub fn clear_ex < 'ex > (&'ex mut self,) -> ExClear < 'ex > {
            ExClear::new(self,)
        }
        pub fn set_hide_on_item_selection(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9056usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_hide_on_item_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_hide_on_item_selection(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9057usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "is_hide_on_item_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hide_on_checkable_item_selection(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9058usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_hide_on_checkable_item_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_hide_on_checkable_item_selection(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9059usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "is_hide_on_checkable_item_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hide_on_state_item_selection(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9060usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_hide_on_state_item_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_hide_on_state_item_selection(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9061usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "is_hide_on_state_item_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_submenu_popup_delay(&mut self, seconds: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (seconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_submenu_popup_delay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_submenu_popup_delay(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9063usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_submenu_popup_delay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_search(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9064usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_allow_search", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_search(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_allow_search", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the menu is bound to the special system menu."]
        pub fn is_system_menu(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9066usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "is_system_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_system_menu(&mut self, system_menu_id: crate::classes::native_menu::SystemMenus,) {
            type CallRet = ();
            type CallParams = (crate::classes::native_menu::SystemMenus,);
            let args = (system_menu_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9067usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_system_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_system_menu(&self,) -> crate::classes::native_menu::SystemMenus {
            type CallRet = crate::classes::native_menu::SystemMenus;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_system_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shrink_height(&mut self, shrink: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (shrink,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_shrink_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shrink_height(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_shrink_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shrink_width(&mut self, shrink: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (shrink,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "set_shrink_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shrink_width(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PopupMenu", "get_shrink_width", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PopupMenu {
        type Base = crate::classes::Popup;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PopupMenu"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PopupMenu {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Popup > for PopupMenu {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Window > for PopupMenu {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Viewport > for PopupMenu {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for PopupMenu {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PopupMenu {
        
    }
    impl crate::obj::cap::GodotDefault for PopupMenu {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PopupMenu {
        type Target = crate::classes::Popup;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PopupMenu {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PopupMenu`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PopupMenu__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PopupMenu > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Popup > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Window > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Viewport > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PopupMenu::activate_item_by_event_ex`][super::PopupMenu::activate_item_by_event_ex]."]
#[must_use]
pub struct ExActivateItemByEvent < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, event: CowArg < 'ex, Option < Gd < crate::classes::InputEvent > > >, for_global_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExActivateItemByEvent < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'ex,) -> Self {
        let for_global_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, event: event.into_arg(), for_global_only: for_global_only,
        }
    }
    #[inline]
    pub fn for_global_only(self, for_global_only: bool) -> Self {
        Self {
            for_global_only: for_global_only, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, event, for_global_only,
        }
        = self;
        re_export::PopupMenu::activate_item_by_event_full(surround_object, event, for_global_only,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_item_ex`][super::PopupMenu::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, label: CowArg < 'ex, GString >, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, label: impl AsArg < GString > + 'ex,) -> Self {
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), id: id, accel: accel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, id, accel,
        }
        = self;
        re_export::PopupMenu::add_item_full(surround_object, label, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_item_ex`][super::PopupMenu::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'ex, GString >, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddIconItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), label: label.into_arg(), id: id, accel: accel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, label, id, accel,
        }
        = self;
        re_export::PopupMenu::add_icon_item_full(surround_object, texture, label, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_check_item_ex`][super::PopupMenu::add_check_item_ex]."]
#[must_use]
pub struct ExAddCheckItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, label: CowArg < 'ex, GString >, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddCheckItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, label: impl AsArg < GString > + 'ex,) -> Self {
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), id: id, accel: accel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, id, accel,
        }
        = self;
        re_export::PopupMenu::add_check_item_full(surround_object, label, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_check_item_ex`][super::PopupMenu::add_icon_check_item_ex]."]
#[must_use]
pub struct ExAddIconCheckItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'ex, GString >, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddIconCheckItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), label: label.into_arg(), id: id, accel: accel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, label, id, accel,
        }
        = self;
        re_export::PopupMenu::add_icon_check_item_full(surround_object, texture, label, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_radio_check_item_ex`][super::PopupMenu::add_radio_check_item_ex]."]
#[must_use]
pub struct ExAddRadioCheckItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, label: CowArg < 'ex, GString >, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddRadioCheckItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, label: impl AsArg < GString > + 'ex,) -> Self {
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), id: id, accel: accel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, id, accel,
        }
        = self;
        re_export::PopupMenu::add_radio_check_item_full(surround_object, label, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_radio_check_item_ex`][super::PopupMenu::add_icon_radio_check_item_ex]."]
#[must_use]
pub struct ExAddIconRadioCheckItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'ex, GString >, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddIconRadioCheckItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), label: label.into_arg(), id: id, accel: accel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, label, id, accel,
        }
        = self;
        re_export::PopupMenu::add_icon_radio_check_item_full(surround_object, texture, label, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_multistate_item_ex`][super::PopupMenu::add_multistate_item_ex]."]
#[must_use]
pub struct ExAddMultistateItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, label: CowArg < 'ex, GString >, max_states: i32, default_state: i32, id: i32, accel: crate::global::Key,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddMultistateItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, label: impl AsArg < GString > + 'ex, max_states: i32,) -> Self {
        let default_state = 0i32;
        let id = - 1i32;
        let accel = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), max_states: max_states, default_state: default_state, id: id, accel: accel,
        }
    }
    #[inline]
    pub fn default_state(self, default_state: i32) -> Self {
        Self {
            default_state: default_state, .. self
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn accel(self, accel: crate::global::Key) -> Self {
        Self {
            accel: accel, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, max_states, default_state, id, accel,
        }
        = self;
        re_export::PopupMenu::add_multistate_item_full(surround_object, label, max_states, default_state, id, accel,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_shortcut_ex`][super::PopupMenu::add_shortcut_ex]."]
#[must_use]
pub struct ExAddShortcut < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, shortcut: CowArg < 'ex, Option < Gd < crate::classes::Shortcut > > >, id: i32, global: bool, allow_echo: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddShortcut < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> Self {
        let id = - 1i32;
        let global = false;
        let allow_echo = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shortcut: shortcut.into_arg(), id: id, global: global, allow_echo: allow_echo,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn allow_echo(self, allow_echo: bool) -> Self {
        Self {
            allow_echo: allow_echo, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shortcut, id, global, allow_echo,
        }
        = self;
        re_export::PopupMenu::add_shortcut_full(surround_object, shortcut, id, global, allow_echo,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_shortcut_ex`][super::PopupMenu::add_icon_shortcut_ex]."]
#[must_use]
pub struct ExAddIconShortcut < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, shortcut: CowArg < 'ex, Option < Gd < crate::classes::Shortcut > > >, id: i32, global: bool, allow_echo: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddIconShortcut < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> Self {
        let id = - 1i32;
        let global = false;
        let allow_echo = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), shortcut: shortcut.into_arg(), id: id, global: global, allow_echo: allow_echo,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn allow_echo(self, allow_echo: bool) -> Self {
        Self {
            allow_echo: allow_echo, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, shortcut, id, global, allow_echo,
        }
        = self;
        re_export::PopupMenu::add_icon_shortcut_full(surround_object, texture, shortcut, id, global, allow_echo,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_check_shortcut_ex`][super::PopupMenu::add_check_shortcut_ex]."]
#[must_use]
pub struct ExAddCheckShortcut < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, shortcut: CowArg < 'ex, Option < Gd < crate::classes::Shortcut > > >, id: i32, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddCheckShortcut < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> Self {
        let id = - 1i32;
        let global = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shortcut: shortcut.into_arg(), id: id, global: global,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shortcut, id, global,
        }
        = self;
        re_export::PopupMenu::add_check_shortcut_full(surround_object, shortcut, id, global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_check_shortcut_ex`][super::PopupMenu::add_icon_check_shortcut_ex]."]
#[must_use]
pub struct ExAddIconCheckShortcut < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, shortcut: CowArg < 'ex, Option < Gd < crate::classes::Shortcut > > >, id: i32, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddIconCheckShortcut < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> Self {
        let id = - 1i32;
        let global = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), shortcut: shortcut.into_arg(), id: id, global: global,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, shortcut, id, global,
        }
        = self;
        re_export::PopupMenu::add_icon_check_shortcut_full(surround_object, texture, shortcut, id, global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_radio_check_shortcut_ex`][super::PopupMenu::add_radio_check_shortcut_ex]."]
#[must_use]
pub struct ExAddRadioCheckShortcut < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, shortcut: CowArg < 'ex, Option < Gd < crate::classes::Shortcut > > >, id: i32, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddRadioCheckShortcut < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> Self {
        let id = - 1i32;
        let global = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shortcut: shortcut.into_arg(), id: id, global: global,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shortcut, id, global,
        }
        = self;
        re_export::PopupMenu::add_radio_check_shortcut_full(surround_object, shortcut, id, global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_icon_radio_check_shortcut_ex`][super::PopupMenu::add_icon_radio_check_shortcut_ex]."]
#[must_use]
pub struct ExAddIconRadioCheckShortcut < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, shortcut: CowArg < 'ex, Option < Gd < crate::classes::Shortcut > > >, id: i32, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddIconRadioCheckShortcut < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> Self {
        let id = - 1i32;
        let global = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), shortcut: shortcut.into_arg(), id: id, global: global,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, shortcut, id, global,
        }
        = self;
        re_export::PopupMenu::add_icon_radio_check_shortcut_full(surround_object, texture, shortcut, id, global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_submenu_item_ex`][super::PopupMenu::add_submenu_item_ex]."]
#[must_use]
pub struct ExAddSubmenuItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, label: CowArg < 'ex, GString >, submenu: CowArg < 'ex, GString >, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddSubmenuItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, label: impl AsArg < GString > + 'ex, submenu: impl AsArg < GString > + 'ex,) -> Self {
        let id = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), submenu: submenu.into_arg(), id: id,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, submenu, id,
        }
        = self;
        re_export::PopupMenu::add_submenu_item_full(surround_object, label, submenu, id,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_submenu_node_item_ex`][super::PopupMenu::add_submenu_node_item_ex]."]
#[must_use]
pub struct ExAddSubmenuNodeItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, label: CowArg < 'ex, GString >, submenu: CowArg < 'ex, Option < Gd < crate::classes::PopupMenu > > >, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddSubmenuNodeItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, label: impl AsArg < GString > + 'ex, submenu: impl AsArg < Option < Gd < crate::classes::PopupMenu >> > + 'ex,) -> Self {
        let id = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), submenu: submenu.into_arg(), id: id,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, submenu, id,
        }
        = self;
        re_export::PopupMenu::add_submenu_node_item_full(surround_object, label, submenu, id,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::set_item_shortcut_ex`][super::PopupMenu::set_item_shortcut_ex]."]
#[must_use]
pub struct ExSetItemShortcut < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, index: i32, shortcut: CowArg < 'ex, Option < Gd < crate::classes::Shortcut > > >, global: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetItemShortcut < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu, index: i32, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'ex,) -> Self {
        let global = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index, shortcut: shortcut.into_arg(), global: global,
        }
    }
    #[inline]
    pub fn global(self, global: bool) -> Self {
        Self {
            global: global, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, index, shortcut, global,
        }
        = self;
        re_export::PopupMenu::set_item_shortcut_full(surround_object, index, shortcut, global,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::add_separator_ex`][super::PopupMenu::add_separator_ex]."]
#[must_use]
pub struct ExAddSeparator < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, label: CowArg < 'ex, GString >, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddSeparator < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu,) -> Self {
        let label = GString::from("");
        let id = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: CowArg::Owned(label), id: id,
        }
    }
    #[inline]
    pub fn label(self, label: impl AsArg < GString > + 'ex) -> Self {
        Self {
            label: label.into_arg(), .. self
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, id,
        }
        = self;
        re_export::PopupMenu::add_separator_full(surround_object, label, id,)
    }
}
#[doc = "Default-param extender for [`PopupMenu::clear_ex`][super::PopupMenu::clear_ex]."]
#[must_use]
pub struct ExClear < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PopupMenu, free_submenus: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExClear < 'ex > {
    fn new(surround_object: &'ex mut re_export::PopupMenu,) -> Self {
        let free_submenus = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, free_submenus: free_submenus,
        }
    }
    #[inline]
    pub fn free_submenus(self, free_submenus: bool) -> Self {
        Self {
            free_submenus: free_submenus, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, free_submenus,
        }
        = self;
        re_export::PopupMenu::clear_full(surround_object, free_submenus,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PopupMenu;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`PopupMenu`][crate::classes::PopupMenu] class."]
    pub struct SignalsOfPopupMenu < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfPopupMenu < 'c, C > {
        #[doc = "Signature: `(id: i64)`"]
        pub fn id_pressed(&mut self) -> SigIdPressed < 'c, C > {
            SigIdPressed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "id_pressed")
            }
        }
        #[doc = "Signature: `(id: i64)`"]
        pub fn id_focused(&mut self) -> SigIdFocused < 'c, C > {
            SigIdFocused {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "id_focused")
            }
        }
        #[doc = "Signature: `(index: i64)`"]
        pub fn index_pressed(&mut self) -> SigIndexPressed < 'c, C > {
            SigIndexPressed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "index_pressed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn menu_changed(&mut self) -> SigMenuChanged < 'c, C > {
            SigMenuChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "menu_changed")
            }
        }
    }
    type TypedSigIdPressed < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigIdPressed < 'c, C: WithSignals > {
        typed: TypedSigIdPressed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigIdPressed < 'c, C > {
        pub fn emit(&mut self, id: i64,) {
            self.typed.emit_tuple((id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigIdPressed < 'c, C > {
        type Target = TypedSigIdPressed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigIdPressed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigIdFocused < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigIdFocused < 'c, C: WithSignals > {
        typed: TypedSigIdFocused < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigIdFocused < 'c, C > {
        pub fn emit(&mut self, id: i64,) {
            self.typed.emit_tuple((id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigIdFocused < 'c, C > {
        type Target = TypedSigIdFocused < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigIdFocused < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigIndexPressed < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigIndexPressed < 'c, C: WithSignals > {
        typed: TypedSigIndexPressed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigIndexPressed < 'c, C > {
        pub fn emit(&mut self, index: i64,) {
            self.typed.emit_tuple((index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigIndexPressed < 'c, C > {
        type Target = TypedSigIndexPressed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigIndexPressed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMenuChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMenuChanged < 'c, C: WithSignals > {
        typed: TypedSigMenuChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMenuChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMenuChanged < 'c, C > {
        type Target = TypedSigMenuChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMenuChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for PopupMenu {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfPopupMenu < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfPopupMenu < 'c, C > {
        type Target = < < PopupMenu as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = PopupMenu;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfPopupMenu < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = PopupMenu;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
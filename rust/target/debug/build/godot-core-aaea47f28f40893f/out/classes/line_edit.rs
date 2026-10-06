#![doc = "Sidecar module for class [`LineEdit`][crate::classes::LineEdit].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `LineEdit` enums](https://docs.godotengine.org/en/stable/classes/class_lineedit.html#enumerations).\n\n"]
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
    #[doc = "Godot class `LineEdit`.\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`line_edit`][crate::classes::line_edit]: sidecar module with related enum/flag types\n* [`ILineEdit`][crate::classes::ILineEdit]: virtual methods\n* [`SignalsOfLineEdit`][crate::classes::line_edit::SignalsOfLineEdit]: signal collection\n\n\nSee also [Godot docs for `LineEdit`](https://docs.godotengine.org/en/stable/classes/class_lineedit.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`LineEdit::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\n`LineEdit` provides an input field for editing a single line of text.\n\n- When the `LineEdit` control is focused using the keyboard arrow keys, it will only gain focus and not enter edit mode.\n\n- To enter edit mode, click on the control with the mouse, see also \\[member keep_editing_on_text_submit].\n\n- To exit edit mode, press `ui_text_submit` or `ui_cancel` (by default `Escape`) actions.\n\n- Check [`edit`][`crate::classes::LineEdit::edit`], [`unedit`][`crate::classes::LineEdit::unedit`], [`is_editing`][`crate::classes::LineEdit::is_editing`], and `editing_toggled` for more information.\n\nWhile entering text, it is possible to insert special characters using Unicode, OEM or Windows alt codes:\n\n- To enter Unicode codepoints, hold `Alt` and type the codepoint on the numpad. For example, to enter the character `á` (U+00E1), hold `Alt` and type `+E1` on the numpad (the leading zeroes can be omitted).\n\n- To enter OEM codepoints, hold `Alt` and type the code on the numpad. For example, to enter the character `á` (OEM 160), hold `Alt` and type `160` on the numpad.\n\n- To enter Windows codepoints, hold `Alt` and type the code on the numpad. For example, to enter the character `á` (Windows 0225), hold `Alt` and type `0`, `2`, `2`, `5` on the numpad. The leading zero here must **not** be omitted, as this is how Windows codepoints are distinguished from OEM codepoints.\n\n**Important:**\n\n- Focusing the `LineEdit` with `ui_focus_next` (by default `Tab`) or `ui_focus_prev` (by default `Shift + Tab`) or [`grab_focus`][`crate::classes::Control::grab_focus`] still enters edit mode (for compatibility).\n\n`LineEdit` features many built-in shortcuts that are always available (`Ctrl` here maps to `Cmd` on macOS):\n\n- `Ctrl + C`: Copy\n\n- `Ctrl + X`: Cut\n\n- `Ctrl + V` or `Ctrl + Y`: Paste/\"yank\"\n\n- `Ctrl + Z`: Undo\n\n- `Ctrl + ~`: Swap input direction.\n\n- `Ctrl + Shift + Z`: Redo\n\n- `Ctrl + U`: Delete text from the caret position to the beginning of the line\n\n- `Ctrl + K`: Delete text from the caret position to the end of the line\n\n- `Ctrl + A`: Select all text\n\n- `Up Arrow`/`Down Arrow`: Move the caret to the beginning/end of the line\n\nOn macOS, some extra keyboard shortcuts are available:\n\n- `Cmd + F`: Same as `Right Arrow`, move the caret one character right\n\n- `Cmd + B`: Same as `Left Arrow`, move the caret one character left\n\n- `Cmd + P`: Same as `Up Arrow`, move the caret to the previous line\n\n- `Cmd + N`: Same as `Down Arrow`, move the caret to the next line\n\n- `Cmd + D`: Same as `Delete`, delete the character on the right side of caret\n\n- `Cmd + H`: Same as `Backspace`, delete the character on the left side of the caret\n\n- `Cmd + A`: Same as `Home`, move the caret to the beginning of the line\n\n- `Cmd + E`: Same as `End`, move the caret to the end of the line\n\n- `Cmd + Left Arrow`: Same as `Home`, move the caret to the beginning of the line\n\n- `Cmd + Right Arrow`: Same as `End`, move the caret to the end of the line\n\n**Note:** Caret movement shortcuts listed above are not affected by \\[member shortcut_keys_enabled]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct LineEdit {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`LineEdit`][crate::classes::LineEdit].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `LineEdit` methods](https://docs.godotengine.org/en/stable/classes/class_lineedit.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILineEdit: crate::obj::GodotClass < Base = LineEdit > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
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
        #[doc = "Virtual method to be implemented by the user. Returns whether the given `point` is inside this control.\n\nIf not overridden, default behavior is checking if the point is within control's Rect.\n\n**Note:** If you want to check if a point is inside the control, you can use `Rect2(Vector2.ZERO, size).has_point(point)`."]
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        #[doc = "User defined BiDi algorithm override function.\n\nReturns an [`Array`][crate::builtin::Array] of [`Vector3i`][crate::builtin::Vector3i] text ranges and text base directions, in the left-to-right order. Ranges should cover full source `text` without overlaps. BiDi algorithm will be used on each range separately."]
        fn structured_text_parser(&self, args: VarArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        #[doc = "Virtual method to be implemented by the user. Returns the minimum size for this control. Alternative to \\[member custom_minimum_size] for controlling minimum size via code. The actual minimum size will be the max value of these two (in each axis separately).\n\nIf not overridden, defaults to `Vector2.ZERO`.\n\n**Note:** This method will not be called when the script is attached to a `Control` node that already overrides its minimum size (e.g. [`Label`][crate::classes::Label], [`Button`][crate::classes::Button], [`PanelContainer`][crate::classes::PanelContainer] etc.). It can only be used with most basic GUI nodes, like `Control`, [`Container`][crate::classes::Container], [`Panel`][crate::classes::Panel] etc."]
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        #[doc = "Virtual method to be implemented by the user. Returns the tooltip text for the position `at_position` in control's local coordinates, which will typically appear when the cursor is resting over this control. See [`get_tooltip`][`crate::classes::Control::get_tooltip`].\n\n**Note:** If this method returns an empty [`String`][crate::builtin::GString] and [`make_custom_tooltip`][`crate::classes::IControl::make_custom_tooltip`] is not overridden, no tooltip is displayed."]
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        #[doc = "Godot calls this method to get data that can be dragged and dropped onto controls that expect drop data. Returns `null` if there is no data to drag. Controls that want to receive drop data should implement [`can_drop_data`][`crate::classes::IControl::can_drop_data`] and [`drop_data`][`crate::classes::IControl::drop_data`]. `at_position` is local to this control. Drag may be forced with [`force_drag`][`crate::classes::Control::force_drag`].\n\nA preview that will follow the mouse that should represent the data can be set with [`set_drag_preview`][`crate::classes::Control::set_drag_preview`]. A good time to set the preview is in this method.\n\n**Note:** If the drag was initiated by a keyboard shortcut or [`accessibility_drag`][`crate::classes::Control::accessibility_drag`], `at_position` is set to `Vector2.INF`, and the currently selected item/text position should be used as the drag position.\n\n\n```gdscript\nfunc _get_drag_data(position):\n\tvar mydata = make_data() # This is your custom method generating the drag data.\n\tset_drag_preview(make_preview(mydata)) # This is your custom method generating the preview of the drag data.\n\treturn mydata\n```\n"]
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        #[doc = "Godot calls this method to test if `data` from a control's [`get_drag_data`][`crate::classes::IControl::get_drag_data`] can be dropped at `at_position`. `at_position` is local to this control.\n\nThis method should only be used to test the data. Process the data in [`drop_data`][`crate::classes::IControl::drop_data`].\n\n**Note:** If the drag was initiated by a keyboard shortcut or [`accessibility_drag`][`crate::classes::Control::accessibility_drag`], `at_position` is set to `Vector2.INF`, and the currently selected item/text position should be used as the drop position.\n\n\n```gdscript\nfunc _can_drop_data(position, data):\n\t# Check position if it is relevant to you\n\t# Otherwise, just check data\n\treturn typeof(data) == TYPE_DICTIONARY and data.has(\"expected\")\n```\n"]
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        #[doc = "Godot calls this method to pass you the `data` from a control's [`get_drag_data`][`crate::classes::IControl::get_drag_data`] result. Godot first calls [`can_drop_data`][`crate::classes::IControl::can_drop_data`] to test if `data` is allowed to drop at `at_position` where `at_position` is local to this control.\n\n**Note:** If the drag was initiated by a keyboard shortcut or [`accessibility_drag`][`crate::classes::Control::accessibility_drag`], `at_position` is set to `Vector2.INF`, and the currently selected item/text position should be used as the drop position.\n\n\n```gdscript\nfunc _can_drop_data(position, data):\n\treturn typeof(data) == TYPE_DICTIONARY and data.has(\"color\")\n\nfunc _drop_data(position, data):\n\tvar color = data[\"color\"]\n```\n"]
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        #[doc = "Virtual method to be implemented by the user. Returns a `Control` node that should be used as a tooltip instead of the default one. `for_text` is the return value of [`get_tooltip`][`crate::classes::Control::get_tooltip`].\n\nThe returned node must be of type `Control` or Control-derived. It can have child nodes of any type. It is freed when the tooltip disappears, so make sure you always provide a new instance (if you want to use a pre-existing node from your scene tree, you can duplicate it and pass the duplicated instance). When `null` or a non-Control node is returned, the default tooltip will be used instead.\n\nThe returned node will be added as child to a [`PopupPanel`][crate::classes::PopupPanel], so you should only provide the contents of that panel. That [`PopupPanel`][crate::classes::PopupPanel] can be themed using [`set_stylebox`][`crate::classes::Theme::set_stylebox`] for the type `\"TooltipPanel\"` (see \\[member tooltip_text] for an example).\n\n**Note:** The tooltip is shrunk to minimal size. If you want to ensure it's fully visible, you might want to set its \\[member custom_minimum_size] to some non-zero value.\n\n**Note:** The node (and any relevant children) should have their \\[member CanvasItem.visible] set to `true` when returned, otherwise, the viewport that instantiates it will not be able to calculate its minimum size reliably.\n\n**Note:** If overridden, this method is called even if [`get_tooltip`][`crate::classes::Control::get_tooltip`] returns an empty string. When this happens with the default tooltip, it is not displayed. To copy this behavior, return `null` in this method when `for_text` is empty.\n\n**Example:** Use a constructed node as a tooltip:\n\n\n```gdscript\nfunc _make_custom_tooltip(for_text):\n\tvar label = Label.new()\n\tlabel.text = for_text\n\treturn label\n```\n\n\n**Example:** Use a scene instance as a tooltip:\n\n\n```gdscript\nfunc _make_custom_tooltip(for_text):\n\tvar tooltip = preload(\"res://some_tooltip_scene.tscn\").instantiate()\n\ttooltip.get_node(\"Label\").text = for_text\n\treturn tooltip\n```\n"]
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        #[doc = "Return the description of the keyboard shortcuts and other contextual help for this control."]
        fn accessibility_get_contextual_info(&self,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to return a human-readable description of the position of the child `node` in the custom container, added to the \\[member accessibility_name]."]
        fn get_accessibility_container_name(&self, node: Option < Gd < crate::classes::Node > >,) -> GString {
            unimplemented !()
        }
        #[doc = "Virtual method to be implemented by the user. Override this method to handle and accept inputs on UI elements. See also [`accept_event`][`crate::classes::Control::accept_event`].\n\n**Example:** Click on the control to print a message:\n\n\n```gdscript\nfunc _gui_input(event):\n\tif event is InputEventMouseButton:\n\t\tif event.button_index == MOUSE_BUTTON_LEFT and event.pressed:\n\t\t\tprint(\"I've been clicked D:\")\n```\n\n\nIf the `event` inherits [`InputEventMouse`][crate::classes::InputEventMouse], this method will **not** be called when:\n\n- the control's \\[member mouse_filter] is set to [`MouseFilter::IGNORE`][`crate::classes::control::MouseFilter::IGNORE`];\n\n- the control is obstructed by another control on top, that doesn't have \\[member mouse_filter] set to [`MouseFilter::IGNORE`][`crate::classes::control::MouseFilter::IGNORE`];\n\n- the control's parent has \\[member mouse_filter] set to [`MouseFilter::STOP`][`crate::classes::control::MouseFilter::STOP`] or has accepted the event;\n\n- the control's parent has \\[member clip_contents] enabled and the `event`'s position is outside the parent's rectangle;\n\n- the `event`'s position is outside the control (see [`has_point`][`crate::classes::IControl::has_point`]).\n\n**Note:** The `event`'s position is relative to this control's origin."]
        fn gui_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        #[doc = "Called when `CanvasItem` has been requested to redraw (after [`queue_redraw`][`crate::classes::CanvasItem::queue_redraw`] is called, either manually or by the engine).\n\nCorresponds to the [`CanvasItemNotification::DRAW`][`crate::classes::notify::CanvasItemNotification::DRAW`] notification in [`on_notification`][`crate::classes::IObject::on_notification`]."]
        fn draw(&mut self,) {
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
    impl LineEdit {
        #[doc = "Returns `true` if the user has text in the [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) (IME)."]
        pub fn has_ime_text(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "has_ime_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Closes the [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) (IME) if it is open. Any text in the IME will be lost."]
        pub fn cancel_ime(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "cancel_ime", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies text from the [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) (IME) and closes the IME if it is open."]
        pub fn apply_ime(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "apply_ime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::HorizontalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Allows entering edit mode whether the `LineEdit` is focused or not. If `hide_focus` is `true`, the focused state will not be shown (see [`grab_focus`][`crate::classes::Control::grab_focus`]).\n\nSee also \\[member keep_editing_on_text_submit]."]
        pub(crate) fn edit_full(&mut self, hide_focus: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (hide_focus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "edit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`edit_ex`][Self::edit_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Allows entering edit mode whether the `LineEdit` is focused or not. If `hide_focus` is `true`, the focused state will not be shown (see [`grab_focus`][`crate::classes::Control::grab_focus`]).\n\nSee also \\[member keep_editing_on_text_submit]."]
        #[inline]
        pub fn edit(&mut self,) {
            self.edit_ex() . done()
        }
        #[doc = "Allows entering edit mode whether the `LineEdit` is focused or not. If `hide_focus` is `true`, the focused state will not be shown (see [`grab_focus`][`crate::classes::Control::grab_focus`]).\n\nSee also \\[member keep_editing_on_text_submit]."]
        #[inline]
        pub fn edit_ex < 'ex > (&'ex mut self,) -> ExEdit < 'ex > {
            ExEdit::new(self,)
        }
        #[doc = "Allows exiting edit mode while preserving focus."]
        pub fn unedit(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "unedit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether the `LineEdit` is being edited."]
        pub fn is_editing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_editing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keep_editing_on_text_submit(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_keep_editing_on_text_submit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editing_kept_on_text_submit(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_editing_kept_on_text_submit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Erases the `LineEdit`'s \\[member text]."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Selects characters inside `LineEdit` between `from` and `to`. By default, `from` is at the beginning and `to` at the end.\n\n\n```gdscript\ntext = \"Welcome\"\nselect() # Will select \"Welcome\".\nselect(4) # Will select \"ome\".\nselect(2, 5) # Will select \"lco\".\n```\n"]
        pub(crate) fn select_full(&mut self, from: i32, to: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "select", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`select_ex`][Self::select_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Selects characters inside `LineEdit` between `from` and `to`. By default, `from` is at the beginning and `to` at the end.\n\n\n```gdscript\ntext = \"Welcome\"\nselect() # Will select \"Welcome\".\nselect(4) # Will select \"ome\".\nselect(2, 5) # Will select \"lco\".\n```\n"]
        #[inline]
        pub fn select(&mut self,) {
            self.select_ex() . done()
        }
        #[doc = "Selects characters inside `LineEdit` between `from` and `to`. By default, `from` is at the beginning and `to` at the end.\n\n\n```gdscript\ntext = \"Welcome\"\nselect() # Will select \"Welcome\".\nselect(4) # Will select \"ome\".\nselect(2, 5) # Will select \"lco\".\n```\n"]
        #[inline]
        pub fn select_ex < 'ex > (&'ex mut self,) -> ExSelect < 'ex > {
            ExSelect::new(self,)
        }
        #[doc = "Selects the whole [`String`][crate::builtin::GString]."]
        pub fn select_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "select_all", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears the current selection."]
        pub fn deselect(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "deselect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if an \"undo\" action is available."]
        pub fn has_undo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "has_undo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a \"redo\" action is available."]
        pub fn has_redo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "has_redo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the user has selected text."]
        pub fn has_selection(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "has_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text inside the selection."]
        pub fn get_selected_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_selected_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the selection begin column."]
        pub fn get_selection_from_column(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_selection_from_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the selection end column."]
        pub fn get_selection_to_column(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_selection_to_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_draw_control_chars(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_draw_control_chars", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_control_chars(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_draw_control_chars", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::TextDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::StructuredTextParser,);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallRet = crate::classes::text_server::StructuredTextParser;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_placeholder(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_placeholder(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_column(&mut self, position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_caret_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_caret_column(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_caret_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the correct column at the end of a composite character like ❤\u{fe0f}\u{200d}🩹 (mending heart; Unicode: `U+2764 U+FE0F U+200D U+1FA79`) which is comprised of more than one Unicode code point, if the caret is at the start of the composite character. Also returns the correct column with the caret at mid grapheme and for non-composite characters.\n\n**Note:** To check at caret location use `get_next_composite_character_column(get_caret_column())`"]
        pub fn get_next_composite_character_column(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_next_composite_character_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the correct column at the start of a composite character like ❤\u{fe0f}\u{200d}🩹 (mending heart; Unicode: `U+2764 U+FE0F U+200D U+1FA79`) which is comprised of more than one Unicode code point, if the caret is at the end of the composite character. Also returns the correct column with the caret at mid grapheme and for non-composite characters.\n\n**Note:** To check at caret location use `get_previous_composite_character_column(get_caret_column())`"]
        pub fn get_previous_composite_character_column(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_previous_composite_character_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the scroll offset due to \\[member caret_column], as a number of characters."]
        pub fn get_scroll_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_scroll_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_expand_to_text_length_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_expand_to_text_length_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_expand_to_text_length_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_expand_to_text_length_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_blink_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_caret_blink_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_caret_blink_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_caret_blink_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_mid_grapheme_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_caret_mid_grapheme_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_caret_mid_grapheme_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_caret_mid_grapheme_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_force_displayed(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_caret_force_displayed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_caret_force_displayed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_caret_force_displayed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_blink_interval(&mut self, interval: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_caret_blink_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_caret_blink_interval(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(369usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_caret_blink_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_length(&mut self, chars: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (chars,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(370usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_max_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_length(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(371usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_max_length", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Inserts `text` at the caret. If the resulting value is longer than \\[member max_length], nothing happens."]
        pub fn insert_text_at_caret(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(372usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "insert_text_at_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deletes one character at the caret's current position (equivalent to pressing `Delete`)."]
        pub fn delete_char_at_caret(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(373usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "delete_char_at_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deletes a section of the \\[member text] going from position `from_column` to `to_column`. Both parameters should be within the text's length."]
        pub fn delete_text(&mut self, from_column: i32, to_column: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_column, to_column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(374usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "delete_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_editable(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(375usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_editable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_editable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_secret(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_secret", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_secret(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_secret", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_secret_character(&mut self, character: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (character.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_secret_character", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_secret_character(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_secret_character", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Executes a given action as defined in the \\[enum MenuItems] enum."]
        pub fn menu_option(&mut self, option: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "menu_option", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`PopupMenu`][crate::classes::PopupMenu] of this `LineEdit`. By default, this menu is displayed when right-clicking on the `LineEdit`.\n\nYou can add custom menu items or remove standard ones. Make sure your IDs don't conflict with the standard ones (see \\[enum MenuItems]). For example:\n\n\n```gdscript\nfunc _ready():\n\tvar menu = get_menu()\n\t# Remove all items after \"Redo\".\n\tmenu.item_count = menu.get_item_index(MENU_REDO) + 1\n\t# Add custom items.\n\tmenu.add_separator()\n\tmenu.add_item(\"Insert Date\", MENU_MAX + 1)\n\t# Connect callback.\n\tmenu.id_pressed.connect(_on_item_pressed)\n\nfunc _on_item_pressed(id):\n\tif id == MENU_MAX + 1:\n\t\tinsert_text_at_caret(Time.get_date_string_from_system())\n```\n\n\n**Warning:** This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their \\[member Window.visible] property."]
        pub fn get_menu(&self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallRet = Option < Gd < crate::classes::PopupMenu > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether the menu is visible. Use this instead of `get_menu().visible` to improve performance (so the creation of the menu is avoided)."]
        pub fn is_menu_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_menu_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_context_menu_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_context_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_context_menu_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_context_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emoji_menu_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_emoji_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_emoji_menu_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_emoji_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_backspace_deletes_composite_character_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_backspace_deletes_composite_character_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_backspace_deletes_composite_character_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_backspace_deletes_composite_character_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_virtual_keyboard_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_virtual_keyboard_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_virtual_keyboard_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_virtual_keyboard_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_virtual_keyboard_show_on_focus(&mut self, show_on_focus: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (show_on_focus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_virtual_keyboard_show_on_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_virtual_keyboard_show_on_focus(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_virtual_keyboard_show_on_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_virtual_keyboard_type(&mut self, type_: crate::classes::line_edit::VirtualKeyboardType,) {
            type CallRet = ();
            type CallParams = (crate::classes::line_edit::VirtualKeyboardType,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_virtual_keyboard_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_virtual_keyboard_type(&self,) -> crate::classes::line_edit::VirtualKeyboardType {
            type CallRet = crate::classes::line_edit::VirtualKeyboardType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_virtual_keyboard_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clear_button_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_clear_button_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_clear_button_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_clear_button_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shortcut_keys_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_shortcut_keys_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_shortcut_keys_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_shortcut_keys_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_middle_mouse_paste_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_middle_mouse_paste_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_middle_mouse_paste_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_middle_mouse_paste_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_selecting_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_selecting_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_selecting_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_selecting_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_deselect_on_focus_loss_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_deselect_on_focus_loss_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_deselect_on_focus_loss_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_deselect_on_focus_loss_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_and_drop_selection_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_drag_and_drop_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drag_and_drop_selection_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_drag_and_drop_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_right_icon(&mut self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_right_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_right_icon(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_right_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_icon_expand_mode(&mut self, mode: crate::classes::line_edit::ExpandMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::line_edit::ExpandMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_icon_expand_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon_expand_mode(&self,) -> crate::classes::line_edit::ExpandMode {
            type CallRet = crate::classes::line_edit::ExpandMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_icon_expand_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_right_icon_scale(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_right_icon_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_right_icon_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_right_icon_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flat(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_flat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_flat(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_flat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_select_all_on_focus(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_select_all_on_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_select_all_on_focus(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_select_all_on_focus", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for LineEdit {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("LineEdit"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for LineEdit {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for LineEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for LineEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for LineEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for LineEdit {
        
    }
    impl crate::obj::cap::GodotDefault for LineEdit {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for LineEdit {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for LineEdit {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`LineEdit`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_LineEdit__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::LineEdit > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Control > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`LineEdit::edit_ex`][super::LineEdit::edit_ex]."]
#[must_use]
pub struct ExEdit < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::LineEdit, hide_focus: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExEdit < 'ex > {
    fn new(surround_object: &'ex mut re_export::LineEdit,) -> Self {
        let hide_focus = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, hide_focus: hide_focus,
        }
    }
    #[inline]
    pub fn hide_focus(self, hide_focus: bool) -> Self {
        Self {
            hide_focus: hide_focus, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, hide_focus,
        }
        = self;
        re_export::LineEdit::edit_full(surround_object, hide_focus,)
    }
}
#[doc = "Default-param extender for [`LineEdit::select_ex`][super::LineEdit::select_ex]."]
#[must_use]
pub struct ExSelect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::LineEdit, from: i32, to: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSelect < 'ex > {
    fn new(surround_object: &'ex mut re_export::LineEdit,) -> Self {
        let from = 0i32;
        let to = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, to: to,
        }
    }
    #[inline]
    pub fn from(self, from: i32) -> Self {
        Self {
            from: from, .. self
        }
    }
    #[inline]
    pub fn to(self, to: i32) -> Self {
        Self {
            to: to, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from, to,
        }
        = self;
        re_export::LineEdit::select_full(surround_object, from, to,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MenuItems {
    ord: i32
}
impl MenuItems {
    #[doc(alias = "MENU_CUT")]
    #[doc = "Godot enumerator name: `MENU_CUT`"]
    pub const CUT: MenuItems = MenuItems {
        ord: 0i32
    };
    #[doc(alias = "MENU_COPY")]
    #[doc = "Godot enumerator name: `MENU_COPY`"]
    pub const COPY: MenuItems = MenuItems {
        ord: 1i32
    };
    #[doc(alias = "MENU_PASTE")]
    #[doc = "Godot enumerator name: `MENU_PASTE`"]
    pub const PASTE: MenuItems = MenuItems {
        ord: 2i32
    };
    #[doc(alias = "MENU_CLEAR")]
    #[doc = "Godot enumerator name: `MENU_CLEAR`"]
    pub const CLEAR: MenuItems = MenuItems {
        ord: 3i32
    };
    #[doc(alias = "MENU_SELECT_ALL")]
    #[doc = "Godot enumerator name: `MENU_SELECT_ALL`"]
    pub const SELECT_ALL: MenuItems = MenuItems {
        ord: 4i32
    };
    #[doc(alias = "MENU_UNDO")]
    #[doc = "Godot enumerator name: `MENU_UNDO`"]
    pub const UNDO: MenuItems = MenuItems {
        ord: 5i32
    };
    #[doc(alias = "MENU_REDO")]
    #[doc = "Godot enumerator name: `MENU_REDO`"]
    pub const REDO: MenuItems = MenuItems {
        ord: 6i32
    };
    #[doc(alias = "MENU_SUBMENU_TEXT_DIR")]
    #[doc = "Godot enumerator name: `MENU_SUBMENU_TEXT_DIR`"]
    pub const SUBMENU_TEXT_DIR: MenuItems = MenuItems {
        ord: 7i32
    };
    #[doc(alias = "MENU_DIR_INHERITED")]
    #[doc = "Godot enumerator name: `MENU_DIR_INHERITED`"]
    pub const DIR_INHERITED: MenuItems = MenuItems {
        ord: 8i32
    };
    #[doc(alias = "MENU_DIR_AUTO")]
    #[doc = "Godot enumerator name: `MENU_DIR_AUTO`"]
    pub const DIR_AUTO: MenuItems = MenuItems {
        ord: 9i32
    };
    #[doc(alias = "MENU_DIR_LTR")]
    #[doc = "Godot enumerator name: `MENU_DIR_LTR`"]
    pub const DIR_LTR: MenuItems = MenuItems {
        ord: 10i32
    };
    #[doc(alias = "MENU_DIR_RTL")]
    #[doc = "Godot enumerator name: `MENU_DIR_RTL`"]
    pub const DIR_RTL: MenuItems = MenuItems {
        ord: 11i32
    };
    #[doc(alias = "MENU_DISPLAY_UCC")]
    #[doc = "Godot enumerator name: `MENU_DISPLAY_UCC`"]
    pub const DISPLAY_UCC: MenuItems = MenuItems {
        ord: 12i32
    };
    #[doc(alias = "MENU_SUBMENU_INSERT_UCC")]
    #[doc = "Godot enumerator name: `MENU_SUBMENU_INSERT_UCC`"]
    pub const SUBMENU_INSERT_UCC: MenuItems = MenuItems {
        ord: 13i32
    };
    #[doc(alias = "MENU_INSERT_LRM")]
    #[doc = "Godot enumerator name: `MENU_INSERT_LRM`"]
    pub const INSERT_LRM: MenuItems = MenuItems {
        ord: 14i32
    };
    #[doc(alias = "MENU_INSERT_RLM")]
    #[doc = "Godot enumerator name: `MENU_INSERT_RLM`"]
    pub const INSERT_RLM: MenuItems = MenuItems {
        ord: 15i32
    };
    #[doc(alias = "MENU_INSERT_LRE")]
    #[doc = "Godot enumerator name: `MENU_INSERT_LRE`"]
    pub const INSERT_LRE: MenuItems = MenuItems {
        ord: 16i32
    };
    #[doc(alias = "MENU_INSERT_RLE")]
    #[doc = "Godot enumerator name: `MENU_INSERT_RLE`"]
    pub const INSERT_RLE: MenuItems = MenuItems {
        ord: 17i32
    };
    #[doc(alias = "MENU_INSERT_LRO")]
    #[doc = "Godot enumerator name: `MENU_INSERT_LRO`"]
    pub const INSERT_LRO: MenuItems = MenuItems {
        ord: 18i32
    };
    #[doc(alias = "MENU_INSERT_RLO")]
    #[doc = "Godot enumerator name: `MENU_INSERT_RLO`"]
    pub const INSERT_RLO: MenuItems = MenuItems {
        ord: 19i32
    };
    #[doc(alias = "MENU_INSERT_PDF")]
    #[doc = "Godot enumerator name: `MENU_INSERT_PDF`"]
    pub const INSERT_PDF: MenuItems = MenuItems {
        ord: 20i32
    };
    #[doc(alias = "MENU_INSERT_ALM")]
    #[doc = "Godot enumerator name: `MENU_INSERT_ALM`"]
    pub const INSERT_ALM: MenuItems = MenuItems {
        ord: 21i32
    };
    #[doc(alias = "MENU_INSERT_LRI")]
    #[doc = "Godot enumerator name: `MENU_INSERT_LRI`"]
    pub const INSERT_LRI: MenuItems = MenuItems {
        ord: 22i32
    };
    #[doc(alias = "MENU_INSERT_RLI")]
    #[doc = "Godot enumerator name: `MENU_INSERT_RLI`"]
    pub const INSERT_RLI: MenuItems = MenuItems {
        ord: 23i32
    };
    #[doc(alias = "MENU_INSERT_FSI")]
    #[doc = "Godot enumerator name: `MENU_INSERT_FSI`"]
    pub const INSERT_FSI: MenuItems = MenuItems {
        ord: 24i32
    };
    #[doc(alias = "MENU_INSERT_PDI")]
    #[doc = "Godot enumerator name: `MENU_INSERT_PDI`"]
    pub const INSERT_PDI: MenuItems = MenuItems {
        ord: 25i32
    };
    #[doc(alias = "MENU_INSERT_ZWJ")]
    #[doc = "Godot enumerator name: `MENU_INSERT_ZWJ`"]
    pub const INSERT_ZWJ: MenuItems = MenuItems {
        ord: 26i32
    };
    #[doc(alias = "MENU_INSERT_ZWNJ")]
    #[doc = "Godot enumerator name: `MENU_INSERT_ZWNJ`"]
    pub const INSERT_ZWNJ: MenuItems = MenuItems {
        ord: 27i32
    };
    #[doc(alias = "MENU_INSERT_WJ")]
    #[doc = "Godot enumerator name: `MENU_INSERT_WJ`"]
    pub const INSERT_WJ: MenuItems = MenuItems {
        ord: 28i32
    };
    #[doc(alias = "MENU_INSERT_SHY")]
    #[doc = "Godot enumerator name: `MENU_INSERT_SHY`"]
    pub const INSERT_SHY: MenuItems = MenuItems {
        ord: 29i32
    };
    #[doc(alias = "MENU_EMOJI_AND_SYMBOL")]
    #[doc = "Godot enumerator name: `MENU_EMOJI_AND_SYMBOL`"]
    pub const EMOJI_AND_SYMBOL: MenuItems = MenuItems {
        ord: 30i32
    };
    #[doc(alias = "MENU_MAX")]
    #[doc = "Godot enumerator name: `MENU_MAX`"]
    pub const MAX: MenuItems = MenuItems {
        ord: 31i32
    };
    
}
impl std::fmt::Debug for MenuItems {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MenuItems") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MenuItems {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 => Some(Self {
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
            Self::CUT => "CUT", Self::COPY => "COPY", Self::PASTE => "PASTE", Self::CLEAR => "CLEAR", Self::SELECT_ALL => "SELECT_ALL", Self::UNDO => "UNDO", Self::REDO => "REDO", Self::SUBMENU_TEXT_DIR => "SUBMENU_TEXT_DIR", Self::DIR_INHERITED => "DIR_INHERITED", Self::DIR_AUTO => "DIR_AUTO", Self::DIR_LTR => "DIR_LTR", Self::DIR_RTL => "DIR_RTL", Self::DISPLAY_UCC => "DISPLAY_UCC", Self::SUBMENU_INSERT_UCC => "SUBMENU_INSERT_UCC", Self::INSERT_LRM => "INSERT_LRM", Self::INSERT_RLM => "INSERT_RLM", Self::INSERT_LRE => "INSERT_LRE", Self::INSERT_RLE => "INSERT_RLE", Self::INSERT_LRO => "INSERT_LRO", Self::INSERT_RLO => "INSERT_RLO", Self::INSERT_PDF => "INSERT_PDF", Self::INSERT_ALM => "INSERT_ALM", Self::INSERT_LRI => "INSERT_LRI", Self::INSERT_RLI => "INSERT_RLI", Self::INSERT_FSI => "INSERT_FSI", Self::INSERT_PDI => "INSERT_PDI", Self::INSERT_ZWJ => "INSERT_ZWJ", Self::INSERT_ZWNJ => "INSERT_ZWNJ", Self::INSERT_WJ => "INSERT_WJ", Self::INSERT_SHY => "INSERT_SHY", Self::EMOJI_AND_SYMBOL => "EMOJI_AND_SYMBOL", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MenuItems::CUT, MenuItems::COPY, MenuItems::PASTE, MenuItems::CLEAR, MenuItems::SELECT_ALL, MenuItems::UNDO, MenuItems::REDO, MenuItems::SUBMENU_TEXT_DIR, MenuItems::DIR_INHERITED, MenuItems::DIR_AUTO, MenuItems::DIR_LTR, MenuItems::DIR_RTL, MenuItems::DISPLAY_UCC, MenuItems::SUBMENU_INSERT_UCC, MenuItems::INSERT_LRM, MenuItems::INSERT_RLM, MenuItems::INSERT_LRE, MenuItems::INSERT_RLE, MenuItems::INSERT_LRO, MenuItems::INSERT_RLO, MenuItems::INSERT_PDF, MenuItems::INSERT_ALM, MenuItems::INSERT_LRI, MenuItems::INSERT_RLI, MenuItems::INSERT_FSI, MenuItems::INSERT_PDI, MenuItems::INSERT_ZWJ, MenuItems::INSERT_ZWNJ, MenuItems::INSERT_WJ, MenuItems::INSERT_SHY, MenuItems::EMOJI_AND_SYMBOL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MenuItems >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CUT", "MENU_CUT", MenuItems::CUT), crate::meta::inspect::EnumConstant::new("COPY", "MENU_COPY", MenuItems::COPY), crate::meta::inspect::EnumConstant::new("PASTE", "MENU_PASTE", MenuItems::PASTE), crate::meta::inspect::EnumConstant::new("CLEAR", "MENU_CLEAR", MenuItems::CLEAR), crate::meta::inspect::EnumConstant::new("SELECT_ALL", "MENU_SELECT_ALL", MenuItems::SELECT_ALL), crate::meta::inspect::EnumConstant::new("UNDO", "MENU_UNDO", MenuItems::UNDO), crate::meta::inspect::EnumConstant::new("REDO", "MENU_REDO", MenuItems::REDO), crate::meta::inspect::EnumConstant::new("SUBMENU_TEXT_DIR", "MENU_SUBMENU_TEXT_DIR", MenuItems::SUBMENU_TEXT_DIR), crate::meta::inspect::EnumConstant::new("DIR_INHERITED", "MENU_DIR_INHERITED", MenuItems::DIR_INHERITED), crate::meta::inspect::EnumConstant::new("DIR_AUTO", "MENU_DIR_AUTO", MenuItems::DIR_AUTO), crate::meta::inspect::EnumConstant::new("DIR_LTR", "MENU_DIR_LTR", MenuItems::DIR_LTR), crate::meta::inspect::EnumConstant::new("DIR_RTL", "MENU_DIR_RTL", MenuItems::DIR_RTL), crate::meta::inspect::EnumConstant::new("DISPLAY_UCC", "MENU_DISPLAY_UCC", MenuItems::DISPLAY_UCC), crate::meta::inspect::EnumConstant::new("SUBMENU_INSERT_UCC", "MENU_SUBMENU_INSERT_UCC", MenuItems::SUBMENU_INSERT_UCC), crate::meta::inspect::EnumConstant::new("INSERT_LRM", "MENU_INSERT_LRM", MenuItems::INSERT_LRM), crate::meta::inspect::EnumConstant::new("INSERT_RLM", "MENU_INSERT_RLM", MenuItems::INSERT_RLM), crate::meta::inspect::EnumConstant::new("INSERT_LRE", "MENU_INSERT_LRE", MenuItems::INSERT_LRE), crate::meta::inspect::EnumConstant::new("INSERT_RLE", "MENU_INSERT_RLE", MenuItems::INSERT_RLE), crate::meta::inspect::EnumConstant::new("INSERT_LRO", "MENU_INSERT_LRO", MenuItems::INSERT_LRO), crate::meta::inspect::EnumConstant::new("INSERT_RLO", "MENU_INSERT_RLO", MenuItems::INSERT_RLO), crate::meta::inspect::EnumConstant::new("INSERT_PDF", "MENU_INSERT_PDF", MenuItems::INSERT_PDF), crate::meta::inspect::EnumConstant::new("INSERT_ALM", "MENU_INSERT_ALM", MenuItems::INSERT_ALM), crate::meta::inspect::EnumConstant::new("INSERT_LRI", "MENU_INSERT_LRI", MenuItems::INSERT_LRI), crate::meta::inspect::EnumConstant::new("INSERT_RLI", "MENU_INSERT_RLI", MenuItems::INSERT_RLI), crate::meta::inspect::EnumConstant::new("INSERT_FSI", "MENU_INSERT_FSI", MenuItems::INSERT_FSI), crate::meta::inspect::EnumConstant::new("INSERT_PDI", "MENU_INSERT_PDI", MenuItems::INSERT_PDI), crate::meta::inspect::EnumConstant::new("INSERT_ZWJ", "MENU_INSERT_ZWJ", MenuItems::INSERT_ZWJ), crate::meta::inspect::EnumConstant::new("INSERT_ZWNJ", "MENU_INSERT_ZWNJ", MenuItems::INSERT_ZWNJ), crate::meta::inspect::EnumConstant::new("INSERT_WJ", "MENU_INSERT_WJ", MenuItems::INSERT_WJ), crate::meta::inspect::EnumConstant::new("INSERT_SHY", "MENU_INSERT_SHY", MenuItems::INSERT_SHY), crate::meta::inspect::EnumConstant::new("EMOJI_AND_SYMBOL", "MENU_EMOJI_AND_SYMBOL", MenuItems::EMOJI_AND_SYMBOL), crate::meta::inspect::EnumConstant::new("MAX", "MENU_MAX", MenuItems::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for MenuItems {
    const ENUMERATOR_COUNT: usize = 31usize;
    
}
impl crate::meta::GodotConvert for MenuItems {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Menu Cut", 0i64), EnumeratorShape::new_int("Menu Copy", 1i64), EnumeratorShape::new_int("Menu Paste", 2i64), EnumeratorShape::new_int("Menu Clear", 3i64), EnumeratorShape::new_int("Menu Select All", 4i64), EnumeratorShape::new_int("Menu Undo", 5i64), EnumeratorShape::new_int("Menu Redo", 6i64), EnumeratorShape::new_int("Menu Submenu Text Dir", 7i64), EnumeratorShape::new_int("Menu Dir Inherited", 8i64), EnumeratorShape::new_int("Menu Dir Auto", 9i64), EnumeratorShape::new_int("Menu Dir Ltr", 10i64), EnumeratorShape::new_int("Menu Dir Rtl", 11i64), EnumeratorShape::new_int("Menu Display Ucc", 12i64), EnumeratorShape::new_int("Menu Submenu Insert Ucc", 13i64), EnumeratorShape::new_int("Menu Insert Lrm", 14i64), EnumeratorShape::new_int("Menu Insert Rlm", 15i64), EnumeratorShape::new_int("Menu Insert Lre", 16i64), EnumeratorShape::new_int("Menu Insert Rle", 17i64), EnumeratorShape::new_int("Menu Insert Lro", 18i64), EnumeratorShape::new_int("Menu Insert Rlo", 19i64), EnumeratorShape::new_int("Menu Insert Pdf", 20i64), EnumeratorShape::new_int("Menu Insert Alm", 21i64), EnumeratorShape::new_int("Menu Insert Lri", 22i64), EnumeratorShape::new_int("Menu Insert Rli", 23i64), EnumeratorShape::new_int("Menu Insert Fsi", 24i64), EnumeratorShape::new_int("Menu Insert Pdi", 25i64), EnumeratorShape::new_int("Menu Insert Zwj", 26i64), EnumeratorShape::new_int("Menu Insert Zwnj", 27i64), EnumeratorShape::new_int("Menu Insert Wj", 28i64), EnumeratorShape::new_int("Menu Insert Shy", 29i64), EnumeratorShape::new_int("Menu Emoji And Symbol", 30i64), EnumeratorShape::new_int("Menu Max", 31i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("LineEdit.MenuItems")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for MenuItems {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MenuItems {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for MenuItems {
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
impl crate::registry::property::Export for MenuItems {
    
}
impl crate::meta::Element for MenuItems {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VirtualKeyboardType {
    ord: i32
}
impl VirtualKeyboardType {
    #[doc(alias = "KEYBOARD_TYPE_DEFAULT")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_DEFAULT`"]
    pub const DEFAULT: VirtualKeyboardType = VirtualKeyboardType {
        ord: 0i32
    };
    #[doc(alias = "KEYBOARD_TYPE_MULTILINE")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_MULTILINE`"]
    pub const MULTILINE: VirtualKeyboardType = VirtualKeyboardType {
        ord: 1i32
    };
    #[doc(alias = "KEYBOARD_TYPE_NUMBER")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_NUMBER`"]
    pub const NUMBER: VirtualKeyboardType = VirtualKeyboardType {
        ord: 2i32
    };
    #[doc(alias = "KEYBOARD_TYPE_NUMBER_DECIMAL")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_NUMBER_DECIMAL`"]
    pub const NUMBER_DECIMAL: VirtualKeyboardType = VirtualKeyboardType {
        ord: 3i32
    };
    #[doc(alias = "KEYBOARD_TYPE_PHONE")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_PHONE`"]
    pub const PHONE: VirtualKeyboardType = VirtualKeyboardType {
        ord: 4i32
    };
    #[doc(alias = "KEYBOARD_TYPE_EMAIL_ADDRESS")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_EMAIL_ADDRESS`"]
    pub const EMAIL_ADDRESS: VirtualKeyboardType = VirtualKeyboardType {
        ord: 5i32
    };
    #[doc(alias = "KEYBOARD_TYPE_PASSWORD")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_PASSWORD`"]
    pub const PASSWORD: VirtualKeyboardType = VirtualKeyboardType {
        ord: 6i32
    };
    #[doc(alias = "KEYBOARD_TYPE_URL")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_URL`"]
    pub const URL: VirtualKeyboardType = VirtualKeyboardType {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for VirtualKeyboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VirtualKeyboardType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VirtualKeyboardType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::DEFAULT => "DEFAULT", Self::MULTILINE => "MULTILINE", Self::NUMBER => "NUMBER", Self::NUMBER_DECIMAL => "NUMBER_DECIMAL", Self::PHONE => "PHONE", Self::EMAIL_ADDRESS => "EMAIL_ADDRESS", Self::PASSWORD => "PASSWORD", Self::URL => "URL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VirtualKeyboardType::DEFAULT, VirtualKeyboardType::MULTILINE, VirtualKeyboardType::NUMBER, VirtualKeyboardType::NUMBER_DECIMAL, VirtualKeyboardType::PHONE, VirtualKeyboardType::EMAIL_ADDRESS, VirtualKeyboardType::PASSWORD, VirtualKeyboardType::URL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VirtualKeyboardType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "KEYBOARD_TYPE_DEFAULT", VirtualKeyboardType::DEFAULT), crate::meta::inspect::EnumConstant::new("MULTILINE", "KEYBOARD_TYPE_MULTILINE", VirtualKeyboardType::MULTILINE), crate::meta::inspect::EnumConstant::new("NUMBER", "KEYBOARD_TYPE_NUMBER", VirtualKeyboardType::NUMBER), crate::meta::inspect::EnumConstant::new("NUMBER_DECIMAL", "KEYBOARD_TYPE_NUMBER_DECIMAL", VirtualKeyboardType::NUMBER_DECIMAL), crate::meta::inspect::EnumConstant::new("PHONE", "KEYBOARD_TYPE_PHONE", VirtualKeyboardType::PHONE), crate::meta::inspect::EnumConstant::new("EMAIL_ADDRESS", "KEYBOARD_TYPE_EMAIL_ADDRESS", VirtualKeyboardType::EMAIL_ADDRESS), crate::meta::inspect::EnumConstant::new("PASSWORD", "KEYBOARD_TYPE_PASSWORD", VirtualKeyboardType::PASSWORD), crate::meta::inspect::EnumConstant::new("URL", "KEYBOARD_TYPE_URL", VirtualKeyboardType::URL)]
        }
    }
}
impl crate::meta::GodotConvert for VirtualKeyboardType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Keyboard Type Default", 0i64), EnumeratorShape::new_int("Keyboard Type Multiline", 1i64), EnumeratorShape::new_int("Keyboard Type Number", 2i64), EnumeratorShape::new_int("Keyboard Type Number Decimal", 3i64), EnumeratorShape::new_int("Keyboard Type Phone", 4i64), EnumeratorShape::new_int("Keyboard Type Email Address", 5i64), EnumeratorShape::new_int("Keyboard Type Password", 6i64), EnumeratorShape::new_int("Keyboard Type Url", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("LineEdit.VirtualKeyboardType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for VirtualKeyboardType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VirtualKeyboardType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for VirtualKeyboardType {
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
impl crate::registry::property::Export for VirtualKeyboardType {
    
}
impl crate::meta::Element for VirtualKeyboardType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ExpandMode {
    ord: i32
}
impl ExpandMode {
    #[doc(alias = "EXPAND_MODE_ORIGINAL_SIZE")]
    #[doc = "Godot enumerator name: `EXPAND_MODE_ORIGINAL_SIZE`"]
    pub const ORIGINAL_SIZE: ExpandMode = ExpandMode {
        ord: 0i32
    };
    #[doc(alias = "EXPAND_MODE_FIT_TO_TEXT")]
    #[doc = "Godot enumerator name: `EXPAND_MODE_FIT_TO_TEXT`"]
    pub const FIT_TO_TEXT: ExpandMode = ExpandMode {
        ord: 1i32
    };
    #[doc(alias = "EXPAND_MODE_FIT_TO_LINE_EDIT")]
    #[doc = "Godot enumerator name: `EXPAND_MODE_FIT_TO_LINE_EDIT`"]
    pub const FIT_TO_LINE_EDIT: ExpandMode = ExpandMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ExpandMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ExpandMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ExpandMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::ORIGINAL_SIZE => "ORIGINAL_SIZE", Self::FIT_TO_TEXT => "FIT_TO_TEXT", Self::FIT_TO_LINE_EDIT => "FIT_TO_LINE_EDIT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ExpandMode::ORIGINAL_SIZE, ExpandMode::FIT_TO_TEXT, ExpandMode::FIT_TO_LINE_EDIT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ExpandMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ORIGINAL_SIZE", "EXPAND_MODE_ORIGINAL_SIZE", ExpandMode::ORIGINAL_SIZE), crate::meta::inspect::EnumConstant::new("FIT_TO_TEXT", "EXPAND_MODE_FIT_TO_TEXT", ExpandMode::FIT_TO_TEXT), crate::meta::inspect::EnumConstant::new("FIT_TO_LINE_EDIT", "EXPAND_MODE_FIT_TO_LINE_EDIT", ExpandMode::FIT_TO_LINE_EDIT)]
        }
    }
}
impl crate::meta::GodotConvert for ExpandMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Expand Mode Original Size", 0i64), EnumeratorShape::new_int("Expand Mode Fit To Text", 1i64), EnumeratorShape::new_int("Expand Mode Fit To Line Edit", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("LineEdit.ExpandMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ExpandMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ExpandMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ExpandMode {
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
impl crate::registry::property::Export for ExpandMode {
    
}
impl crate::meta::Element for ExpandMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::LineEdit;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`LineEdit`][crate::classes::LineEdit] class."]
    pub struct SignalsOfLineEdit < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfLineEdit < 'c, C > {
        #[doc = "Signature: `(new_text: GString)`"]
        pub fn text_changed(&mut self) -> SigTextChanged < 'c, C > {
            SigTextChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "text_changed")
            }
        }
        #[doc = "Signature: `(rejected_substring: GString)`"]
        pub fn text_change_rejected(&mut self) -> SigTextChangeRejected < 'c, C > {
            SigTextChangeRejected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "text_change_rejected")
            }
        }
        #[doc = "Signature: `(new_text: GString)`"]
        pub fn text_submitted(&mut self) -> SigTextSubmitted < 'c, C > {
            SigTextSubmitted {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "text_submitted")
            }
        }
        #[doc = "Signature: `(toggled_on: bool)`"]
        pub fn editing_toggled(&mut self) -> SigEditingToggled < 'c, C > {
            SigEditingToggled {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "editing_toggled")
            }
        }
    }
    type TypedSigTextChanged < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigTextChanged < 'c, C: WithSignals > {
        typed: TypedSigTextChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTextChanged < 'c, C > {
        pub fn emit(&mut self, new_text: GString,) {
            self.typed.emit_tuple((new_text,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTextChanged < 'c, C > {
        type Target = TypedSigTextChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTextChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTextChangeRejected < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigTextChangeRejected < 'c, C: WithSignals > {
        typed: TypedSigTextChangeRejected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTextChangeRejected < 'c, C > {
        pub fn emit(&mut self, rejected_substring: GString,) {
            self.typed.emit_tuple((rejected_substring,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTextChangeRejected < 'c, C > {
        type Target = TypedSigTextChangeRejected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTextChangeRejected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTextSubmitted < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigTextSubmitted < 'c, C: WithSignals > {
        typed: TypedSigTextSubmitted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTextSubmitted < 'c, C > {
        pub fn emit(&mut self, new_text: GString,) {
            self.typed.emit_tuple((new_text,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTextSubmitted < 'c, C > {
        type Target = TypedSigTextSubmitted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTextSubmitted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigEditingToggled < 'c, C > = TypedSignal < 'c, C, (bool,) >;
    pub struct SigEditingToggled < 'c, C: WithSignals > {
        typed: TypedSigEditingToggled < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEditingToggled < 'c, C > {
        pub fn emit(&mut self, toggled_on: bool,) {
            self.typed.emit_tuple((toggled_on,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigEditingToggled < 'c, C > {
        type Target = TypedSigEditingToggled < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigEditingToggled < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for LineEdit {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfLineEdit < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfLineEdit < 'c, C > {
        type Target = < < LineEdit as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = LineEdit;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfLineEdit < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = LineEdit;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
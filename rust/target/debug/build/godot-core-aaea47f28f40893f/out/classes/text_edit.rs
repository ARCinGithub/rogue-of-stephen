#![doc = "Sidecar module for class [`TextEdit`][crate::classes::TextEdit].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextEdit` enums](https://docs.godotengine.org/en/stable/classes/class_textedit.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextEdit`.\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`text_edit`][crate::classes::text_edit]: sidecar module with related enum/flag types\n* [`ITextEdit`][crate::classes::ITextEdit]: virtual methods\n* [`SignalsOfTextEdit`][crate::classes::text_edit::SignalsOfTextEdit]: signal collection\n\n\nSee also [Godot docs for `TextEdit`](https://docs.godotengine.org/en/stable/classes/class_textedit.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TextEdit::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nA multiline text editor. It also has limited facilities for editing code, such as syntax highlighting support. For more advanced facilities for editing code, see [`CodeEdit`][crate::classes::CodeEdit].\n\nWhile entering text, it is possible to insert special characters using Unicode, OEM or Windows alt codes:\n\n- To enter Unicode codepoints, hold `Alt` and type the codepoint on the numpad. For example, to enter the character `á` (U+00E1), hold `Alt` and type `+E1` on the numpad (the leading zeroes can be omitted).\n\n- To enter OEM codepoints, hold `Alt` and type the code on the numpad. For example, to enter the character `á` (OEM 160), hold `Alt` and type `160` on the numpad.\n\n- To enter Windows codepoints, hold `Alt` and type the code on the numpad. For example, to enter the character `á` (Windows 0225), hold `Alt` and type `0`, `2`, `2`, `5` on the numpad. The leading zero here must **not** be omitted, as this is how Windows codepoints are distinguished from OEM codepoints.\n\n**Note:** Most viewport, caret, and edit methods contain a `caret_index` argument for \\[member caret_multiple] support. The argument should be one of the following: `-1` for all carets, `0` for the main caret, or greater than `0` for secondary carets in the order they were created.\n\n**Note:** When holding down `Alt`, the vertical scroll wheel will scroll 5 times as fast as it would normally do. This also works in the Godot script editor."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextEdit {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TextEdit`][crate::classes::TextEdit].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `TextEdit` methods](https://docs.godotengine.org/en/stable/classes/class_textedit.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextEdit: crate::obj::GodotClass < Base = TextEdit > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Override this method to define what happens when the user types in the provided key `unicode_char`."]
        fn handle_unicode_input(&mut self, unicode_char: i32, caret_index: i32,) {
            unimplemented !()
        }
        #[doc = "Override this method to define what happens when the user presses the backspace key."]
        fn backspace(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        #[doc = "Override this method to define what happens when the user performs a cut operation."]
        fn cut(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        #[doc = "Override this method to define what happens when the user performs a copy operation."]
        fn copy(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        #[doc = "Override this method to define what happens when the user performs a paste operation."]
        fn paste(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        #[doc = "Override this method to define what happens when the user performs a paste operation with middle mouse button.\n\n**Note:** This method is only implemented on Linux."]
        fn paste_primary_clipboard(&mut self, caret_index: i32,) {
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
    impl TextEdit {
        #[doc = "Returns `true` if the user has text in the [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) (IME)."]
        pub fn has_ime_text(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "has_ime_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Closes the [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) (IME) if it is open. Any text in the IME will be lost."]
        pub fn cancel_ime(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "cancel_ime", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies text from the [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) (IME) to each caret and closes the IME if it is open."]
        pub fn apply_ime(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "apply_ime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_editable(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_editable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_editable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::TextDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::StructuredTextParser,);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallRet = crate::classes::text_server::StructuredTextParser;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the tab size for the `TextEdit` to use."]
        pub fn set_tab_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_tab_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the `TextEdit`'s' tab size."]
        pub fn get_tab_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_tab_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_indent_wrapped_lines(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_indent_wrapped_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_indent_wrapped_lines(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_indent_wrapped_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_input_mode(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_tab_input_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_input_mode(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_tab_input_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, enables overtype mode. In this mode, typing overrides existing text instead of inserting text. The \\[member ProjectSettings.input/ui_text_toggle_insert_mode] action toggles overtype mode. See [`is_overtype_mode_enabled`][`crate::classes::TextEdit::is_overtype_mode_enabled`]."]
        pub fn set_overtype_mode_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_overtype_mode_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if overtype mode is enabled. See [`set_overtype_mode_enabled`][`crate::classes::TextEdit::set_overtype_mode_enabled`]."]
        pub fn is_overtype_mode_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_overtype_mode_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_context_menu_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_context_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_context_menu_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_context_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emoji_menu_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_emoji_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_emoji_menu_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_emoji_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_backspace_deletes_composite_character_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_backspace_deletes_composite_character_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_backspace_deletes_composite_character_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_backspace_deletes_composite_character_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shortcut_keys_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_shortcut_keys_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_shortcut_keys_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_shortcut_keys_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_virtual_keyboard_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_virtual_keyboard_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_virtual_keyboard_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_virtual_keyboard_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_virtual_keyboard_show_on_focus(&mut self, show_on_focus: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (show_on_focus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_virtual_keyboard_show_on_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_virtual_keyboard_show_on_focus(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_virtual_keyboard_show_on_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_middle_mouse_paste_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_middle_mouse_paste_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_middle_mouse_paste_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_middle_mouse_paste_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_empty_selection_clipboard_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_empty_selection_clipboard_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_empty_selection_clipboard_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_empty_selection_clipboard_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Performs a full reset of `TextEdit`, including undo history."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of lines in the text."]
        pub fn get_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_placeholder(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_placeholder(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the text for a specific `line`.\n\nCarets on the line will attempt to keep their visual x position."]
        pub fn set_line(&mut self, line: i32, new_text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (line, new_text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text of a specific line."]
        pub fn get_line(&self, line: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns line text as it is currently displayed, including IME composition string."]
        pub fn get_line_with_ime(&self, line: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_with_ime", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the width in pixels of the `wrap_index` on `line`."]
        pub(crate) fn get_line_width_full(&self, line: i32, wrap_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_line_width_ex`][Self::get_line_width_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the width in pixels of the `wrap_index` on `line`."]
        #[inline]
        pub fn get_line_width(&self, line: i32,) -> i32 {
            self.get_line_width_ex(line,) . done()
        }
        #[doc = "Returns the width in pixels of the `wrap_index` on `line`."]
        #[inline]
        pub fn get_line_width_ex < 'ex > (&'ex self, line: i32,) -> ExGetLineWidth < 'ex > {
            ExGetLineWidth::new(self, line,)
        }
        #[doc = "Returns the maximum value of the line height among all lines.\n\n**Note:** The return value is influenced by [theme_item line_spacing] and [theme_item font_size]. And it will not be less than `1`."]
        pub fn get_line_height(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the indent level of the given line. This is the number of spaces and tabs at the beginning of the line, with the tabs taking the tab size into account (see [`get_tab_size`][`crate::classes::TextEdit::get_tab_size`])."]
        pub fn get_indent_level(&self, line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_indent_level", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the first column containing a non-whitespace character on the given line. If there is only whitespace, returns the number of characters."]
        pub fn get_first_non_whitespace_column(&self, line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_first_non_whitespace_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Swaps the two lines. Carets will be swapped with the lines."]
        pub fn swap_lines(&mut self, from_line: i32, to_line: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "swap_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Inserts a new line with `text` at `line`."]
        pub fn insert_line_at(&mut self, line: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (line, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "insert_line_at", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the line of text at `line`. Carets on this line will attempt to match their previous visual x position.\n\nIf `move_carets_down` is `true` carets will move to the next line down, otherwise carets will move up."]
        pub(crate) fn remove_line_at_full(&mut self, line: i32, move_carets_down: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (line, move_carets_down,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "remove_line_at", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`remove_line_at_ex`][Self::remove_line_at_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Removes the line of text at `line`. Carets on this line will attempt to match their previous visual x position.\n\nIf `move_carets_down` is `true` carets will move to the next line down, otherwise carets will move up."]
        #[inline]
        pub fn remove_line_at(&mut self, line: i32,) {
            self.remove_line_at_ex(line,) . done()
        }
        #[doc = "Removes the line of text at `line`. Carets on this line will attempt to match their previous visual x position.\n\nIf `move_carets_down` is `true` carets will move to the next line down, otherwise carets will move up."]
        #[inline]
        pub fn remove_line_at_ex < 'ex > (&'ex mut self, line: i32,) -> ExRemoveLineAt < 'ex > {
            ExRemoveLineAt::new(self, line,)
        }
        #[doc = "Insert the specified text at the caret position."]
        pub(crate) fn insert_text_at_caret_full(&mut self, text: CowArg < GString >, caret_index: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (text, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "insert_text_at_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`insert_text_at_caret_ex`][Self::insert_text_at_caret_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Insert the specified text at the caret position."]
        #[inline]
        pub fn insert_text_at_caret(&mut self, text: impl AsArg < GString >,) {
            self.insert_text_at_caret_ex(text,) . done()
        }
        #[doc = "Insert the specified text at the caret position."]
        #[inline]
        pub fn insert_text_at_caret_ex < 'ex > (&'ex mut self, text: impl AsArg < GString > + 'ex,) -> ExInsertTextAtCaret < 'ex > {
            ExInsertTextAtCaret::new(self, text,)
        }
        #[doc = "Inserts the `text` at `line` and `column`.\n\nIf `before_selection_begin` is `true`, carets and selections that begin at `line` and `column` will moved to the end of the inserted text, along with all carets after it.\n\nIf `before_selection_end` is `true`, selections that end at `line` and `column` will be extended to the end of the inserted text. These parameters can be used to insert text inside of or outside of selections."]
        pub(crate) fn insert_text_full(&mut self, text: CowArg < GString >, line: i32, column: i32, before_selection_begin: bool, before_selection_end: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32, bool, bool,);
            let args = (text, line, column, before_selection_begin, before_selection_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "insert_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`insert_text_ex`][Self::insert_text_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Inserts the `text` at `line` and `column`.\n\nIf `before_selection_begin` is `true`, carets and selections that begin at `line` and `column` will moved to the end of the inserted text, along with all carets after it.\n\nIf `before_selection_end` is `true`, selections that end at `line` and `column` will be extended to the end of the inserted text. These parameters can be used to insert text inside of or outside of selections."]
        #[inline]
        pub fn insert_text(&mut self, text: impl AsArg < GString >, line: i32, column: i32,) {
            self.insert_text_ex(text, line, column,) . done()
        }
        #[doc = "Inserts the `text` at `line` and `column`.\n\nIf `before_selection_begin` is `true`, carets and selections that begin at `line` and `column` will moved to the end of the inserted text, along with all carets after it.\n\nIf `before_selection_end` is `true`, selections that end at `line` and `column` will be extended to the end of the inserted text. These parameters can be used to insert text inside of or outside of selections."]
        #[inline]
        pub fn insert_text_ex < 'ex > (&'ex mut self, text: impl AsArg < GString > + 'ex, line: i32, column: i32,) -> ExInsertText < 'ex > {
            ExInsertText::new(self, text, line, column,)
        }
        #[doc = "Removes text between the given positions."]
        pub fn remove_text(&mut self, from_line: i32, from_column: i32, to_line: i32, to_column: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32, i32,);
            let args = (from_line, from_column, to_line, to_column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "remove_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the last unhidden line in the entire `TextEdit`."]
        pub fn get_last_unhidden_line(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_last_unhidden_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the count to the next visible line from `line` to `line + visible_amount`. Can also count backwards. For example if a `TextEdit` has 5 lines with lines 2 and 3 hidden, calling this with `line = 1, visible_amount = 1` would return 3."]
        pub fn get_next_visible_line_offset_from(&self, line: i32, visible_amount: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, visible_amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_next_visible_line_offset_from", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Similar to [`get_next_visible_line_offset_from`][`crate::classes::TextEdit::get_next_visible_line_offset_from`], but takes into account the line wrap indexes. In the returned vector, `x` is the line, `y` is the wrap index."]
        pub fn get_next_visible_line_index_offset_from(&self, line: i32, wrap_index: i32, visible_amount: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32, i32, i32,);
            let args = (line, wrap_index, visible_amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_next_visible_line_index_offset_from", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Called when the user presses the backspace key. Can be overridden with [`backspace`][`crate::classes::ITextEdit::backspace`]."]
        pub(crate) fn backspace_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "backspace", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`backspace_ex`][Self::backspace_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Called when the user presses the backspace key. Can be overridden with [`backspace`][`crate::classes::ITextEdit::backspace`]."]
        #[inline]
        pub fn backspace(&mut self,) {
            self.backspace_ex() . done()
        }
        #[doc = "Called when the user presses the backspace key. Can be overridden with [`backspace`][`crate::classes::ITextEdit::backspace`]."]
        #[inline]
        pub fn backspace_ex < 'ex > (&'ex mut self,) -> ExBackspace < 'ex > {
            ExBackspace::new(self,)
        }
        #[doc = "Cut's the current selection. Can be overridden with [`cut`][`crate::classes::ITextEdit::cut`]."]
        pub(crate) fn cut_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "cut", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`cut_ex`][Self::cut_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Cut's the current selection. Can be overridden with [`cut`][`crate::classes::ITextEdit::cut`]."]
        #[inline]
        pub fn cut(&mut self,) {
            self.cut_ex() . done()
        }
        #[doc = "Cut's the current selection. Can be overridden with [`cut`][`crate::classes::ITextEdit::cut`]."]
        #[inline]
        pub fn cut_ex < 'ex > (&'ex mut self,) -> ExCut < 'ex > {
            ExCut::new(self,)
        }
        #[doc = "Copies the current text selection. Can be overridden with [`copy`][`crate::classes::ITextEdit::copy`]."]
        pub(crate) fn copy_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "copy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`copy_ex`][Self::copy_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Copies the current text selection. Can be overridden with [`copy`][`crate::classes::ITextEdit::copy`]."]
        #[inline]
        pub fn copy(&mut self,) {
            self.copy_ex() . done()
        }
        #[doc = "Copies the current text selection. Can be overridden with [`copy`][`crate::classes::ITextEdit::copy`]."]
        #[inline]
        pub fn copy_ex < 'ex > (&'ex mut self,) -> ExCopy < 'ex > {
            ExCopy::new(self,)
        }
        #[doc = "Paste at the current location. Can be overridden with [`paste`][`crate::classes::ITextEdit::paste`]."]
        pub(crate) fn paste_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "paste", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`paste_ex`][Self::paste_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Paste at the current location. Can be overridden with [`paste`][`crate::classes::ITextEdit::paste`]."]
        #[inline]
        pub fn paste(&mut self,) {
            self.paste_ex() . done()
        }
        #[doc = "Paste at the current location. Can be overridden with [`paste`][`crate::classes::ITextEdit::paste`]."]
        #[inline]
        pub fn paste_ex < 'ex > (&'ex mut self,) -> ExPaste < 'ex > {
            ExPaste::new(self,)
        }
        #[doc = "Pastes the primary clipboard."]
        pub(crate) fn paste_primary_clipboard_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "paste_primary_clipboard", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`paste_primary_clipboard_ex`][Self::paste_primary_clipboard_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Pastes the primary clipboard."]
        #[inline]
        pub fn paste_primary_clipboard(&mut self,) {
            self.paste_primary_clipboard_ex() . done()
        }
        #[doc = "Pastes the primary clipboard."]
        #[inline]
        pub fn paste_primary_clipboard_ex < 'ex > (&'ex mut self,) -> ExPastePrimaryClipboard < 'ex > {
            ExPastePrimaryClipboard::new(self,)
        }
        #[doc = "Starts an action, will end the current action if `action` is different.\n\nAn action will also end after a call to [`end_action`][`crate::classes::TextEdit::end_action`], after \\[member ProjectSettings.gui/timers/text_edit_idle_detect_sec] is triggered or a new undoable step outside the [`start_action`][`crate::classes::TextEdit::start_action`] and [`end_action`][`crate::classes::TextEdit::end_action`] calls."]
        pub fn start_action(&mut self, action: crate::classes::text_edit::EditAction,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_edit::EditAction,);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "start_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Marks the end of steps in the current action started with [`start_action`][`crate::classes::TextEdit::start_action`]."]
        pub fn end_action(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "end_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Starts a multipart edit. All edits will be treated as one action until [`end_complex_operation`][`crate::classes::TextEdit::end_complex_operation`] is called."]
        pub fn begin_complex_operation(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "begin_complex_operation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Ends a multipart edit, started with [`begin_complex_operation`][`crate::classes::TextEdit::begin_complex_operation`]. If called outside a complex operation, the current operation is pushed onto the undo/redo stack."]
        pub fn end_complex_operation(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "end_complex_operation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if an \"undo\" action is available."]
        pub fn has_undo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "has_undo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a \"redo\" action is available."]
        pub fn has_redo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "has_redo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Perform undo operation."]
        pub fn undo(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "undo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Perform redo operation."]
        pub fn redo(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "redo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears the undo history."]
        pub fn clear_undo_history(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "clear_undo_history", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Tag the current version as saved."]
        pub fn tag_saved_version(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "tag_saved_version", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current version of the `TextEdit`. The version is a count of recorded operations by the undo/redo history."]
        pub fn get_version(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_version", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the last tagged saved version from [`tag_saved_version`][`crate::classes::TextEdit::tag_saved_version`]."]
        pub fn get_saved_version(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_saved_version", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the search text. See [`set_search_flags`][`crate::classes::TextEdit::set_search_flags`]."]
        pub fn set_search_text(&mut self, search_text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (search_text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_search_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the search `flags`. This is used with [`set_search_text`][`crate::classes::TextEdit::set_search_text`] to highlight occurrences of the searched text. Search flags can be specified from the \\[enum SearchFlags] enum."]
        pub fn set_search_flags(&mut self, flags: crate::classes::text_edit::SearchFlags,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_edit::SearchFlags,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_search_flags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Perform a search inside the text. Search flags can be specified in the \\[enum SearchFlags] enum.\n\nIn the returned vector, `x` is the column, `y` is the line. If no results are found, both are equal to `-1`.\n\n\n```gdscript\nvar result = search(\"print\", SEARCH_WHOLE_WORDS, 0, 0)\nif result.x != -1:\n\t# Result found.\n\tvar line_number = result.y\n\tvar column_number = result.x\n```\n"]
        pub fn search(&self, text: impl AsArg < GString >, flags: crate::classes::text_edit::SearchFlags, from_line: i32, from_column: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::text_edit::SearchFlags, i32, i32,);
            let args = (text.into_arg(), flags, from_line, from_column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "search", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Provide custom tooltip text. The callback method must take the following args: `hovered_word: String`."]
        pub fn set_tooltip_request_func(&mut self, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_tooltip_request_func", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the local mouse position adjusted for the text direction."]
        pub fn get_local_mouse_pos(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_local_mouse_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the word at `position`."]
        pub fn get_word_at_pos(&self, position: Vector2,) -> GString {
            type CallRet = GString;
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_word_at_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the line and column at the given position. In the returned vector, `x` is the column and `y` is the line.\n\nIf `clamp_line` is `false` and `position` is below the last line, `Vector2i(-1, -1)` is returned.\n\nIf `clamp_column` is `false` and `position` is outside the column range of the line, `Vector2i(-1, -1)` is returned."]
        pub(crate) fn get_line_column_at_pos_full(&self, position: Vector2i, clamp_line: bool, clamp_column: bool,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2i, bool, bool,);
            let args = (position, clamp_line, clamp_column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_column_at_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_line_column_at_pos_ex`][Self::get_line_column_at_pos_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the line and column at the given position. In the returned vector, `x` is the column and `y` is the line.\n\nIf `clamp_line` is `false` and `position` is below the last line, `Vector2i(-1, -1)` is returned.\n\nIf `clamp_column` is `false` and `position` is outside the column range of the line, `Vector2i(-1, -1)` is returned."]
        #[inline]
        pub fn get_line_column_at_pos(&self, position: Vector2i,) -> Vector2i {
            self.get_line_column_at_pos_ex(position,) . done()
        }
        #[doc = "Returns the line and column at the given position. In the returned vector, `x` is the column and `y` is the line.\n\nIf `clamp_line` is `false` and `position` is below the last line, `Vector2i(-1, -1)` is returned.\n\nIf `clamp_column` is `false` and `position` is outside the column range of the line, `Vector2i(-1, -1)` is returned."]
        #[inline]
        pub fn get_line_column_at_pos_ex < 'ex > (&'ex self, position: Vector2i,) -> ExGetLineColumnAtPos < 'ex > {
            ExGetLineColumnAtPos::new(self, position,)
        }
        #[doc = "Returns the local position for the given `line` and `column`. If `x` or `y` of the returned vector equal `-1`, the position is outside of the viewable area of the control.\n\n**Note:** The Y position corresponds to the bottom side of the line. Use [`get_rect_at_line_column`][`crate::classes::TextEdit::get_rect_at_line_column`] to get the top side position."]
        pub fn get_pos_at_line_column(&self, line: i32, column: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_pos_at_line_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the local position and size for the grapheme at the given `line` and `column`. If `x` or `y` position of the returned rect equal `-1`, the position is outside of the viewable area of the control.\n\n**Note:** The Y position of the returned rect corresponds to the top side of the line, unlike [`get_pos_at_line_column`][`crate::classes::TextEdit::get_pos_at_line_column`] which returns the bottom side."]
        pub fn get_rect_at_line_column(&self, line: i32, column: i32,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_rect_at_line_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the equivalent minimap line at `position`."]
        pub fn get_minimap_line_at_pos(&self, position: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_minimap_line_at_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the user is dragging their mouse for scrolling, selecting, or text dragging."]
        pub fn is_dragging_cursor(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_dragging_cursor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the mouse is over a selection. If `edges` is `true`, the edges are considered part of the selection."]
        pub(crate) fn is_mouse_over_selection_full(&self, edges: bool, caret_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (bool, i32,);
            let args = (edges, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_mouse_over_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_mouse_over_selection_ex`][Self::is_mouse_over_selection_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the mouse is over a selection. If `edges` is `true`, the edges are considered part of the selection."]
        #[inline]
        pub fn is_mouse_over_selection(&self, edges: bool,) -> bool {
            self.is_mouse_over_selection_ex(edges,) . done()
        }
        #[doc = "Returns `true` if the mouse is over a selection. If `edges` is `true`, the edges are considered part of the selection."]
        #[inline]
        pub fn is_mouse_over_selection_ex < 'ex > (&'ex self, edges: bool,) -> ExIsMouseOverSelection < 'ex > {
            ExIsMouseOverSelection::new(self, edges,)
        }
        pub fn set_caret_type(&mut self, type_: crate::classes::text_edit::CaretType,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_edit::CaretType,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_caret_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_caret_type(&self,) -> crate::classes::text_edit::CaretType {
            type CallRet = crate::classes::text_edit::CaretType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_blink_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_caret_blink_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_caret_blink_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_caret_blink_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_blink_interval(&mut self, interval: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_caret_blink_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_caret_blink_interval(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_blink_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_caret_when_editable_disabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_draw_caret_when_editable_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_caret_when_editable_disabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_drawing_caret_when_editable_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_move_caret_on_right_click_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_move_caret_on_right_click_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_move_caret_on_right_click_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_move_caret_on_right_click_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_mid_grapheme_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_caret_mid_grapheme_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_caret_mid_grapheme_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_caret_mid_grapheme_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_multiple_carets_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_multiple_carets_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_multiple_carets_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_multiple_carets_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a new caret at the given location. Returns the index of the new caret, or `-1` if the location is invalid."]
        pub fn add_caret(&mut self, line: i32, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "add_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the given caret index.\n\n**Note:** This can result in adjustment of all other caret indices."]
        pub fn remove_caret(&mut self, caret: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "remove_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all additional carets."]
        pub fn remove_secondary_carets(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "remove_secondary_carets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of carets in this `TextEdit`."]
        pub fn get_caret_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an additional caret above or below every caret. If `below` is `true` the new caret will be added below and above otherwise."]
        pub fn add_caret_at_carets(&mut self, below: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (below,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "add_caret_at_carets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the carets sorted by selection beginning from lowest line and column to highest (from top to bottom of text).\n\nIf `include_ignored_carets` is `false`, carets from [`multicaret_edit_ignore_caret`][`crate::classes::TextEdit::multicaret_edit_ignore_caret`] will be ignored."]
        pub(crate) fn get_sorted_carets_full(&self, include_ignored_carets: bool,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (bool,);
            let args = (include_ignored_carets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_sorted_carets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_sorted_carets_ex`][Self::get_sorted_carets_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the carets sorted by selection beginning from lowest line and column to highest (from top to bottom of text).\n\nIf `include_ignored_carets` is `false`, carets from [`multicaret_edit_ignore_caret`][`crate::classes::TextEdit::multicaret_edit_ignore_caret`] will be ignored."]
        #[inline]
        pub fn get_sorted_carets(&self,) -> PackedInt32Array {
            self.get_sorted_carets_ex() . done()
        }
        #[doc = "Returns the carets sorted by selection beginning from lowest line and column to highest (from top to bottom of text).\n\nIf `include_ignored_carets` is `false`, carets from [`multicaret_edit_ignore_caret`][`crate::classes::TextEdit::multicaret_edit_ignore_caret`] will be ignored."]
        #[inline]
        pub fn get_sorted_carets_ex < 'ex > (&'ex self,) -> ExGetSortedCarets < 'ex > {
            ExGetSortedCarets::new(self,)
        }
        #[doc = "Collapse all carets in the given range to the `from_line` and `from_column` position.\n\n`inclusive` applies to both ends.\n\nIf [`is_in_mulitcaret_edit`][`crate::classes::TextEdit::is_in_mulitcaret_edit`] is `true`, carets that are collapsed will be `true` for [`multicaret_edit_ignore_caret`][`crate::classes::TextEdit::multicaret_edit_ignore_caret`].\n\n[`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`] will be called if any carets were collapsed."]
        pub(crate) fn collapse_carets_full(&mut self, from_line: i32, from_column: i32, to_line: i32, to_column: i32, inclusive: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32, i32, bool,);
            let args = (from_line, from_column, to_line, to_column, inclusive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "collapse_carets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`collapse_carets_ex`][Self::collapse_carets_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Collapse all carets in the given range to the `from_line` and `from_column` position.\n\n`inclusive` applies to both ends.\n\nIf [`is_in_mulitcaret_edit`][`crate::classes::TextEdit::is_in_mulitcaret_edit`] is `true`, carets that are collapsed will be `true` for [`multicaret_edit_ignore_caret`][`crate::classes::TextEdit::multicaret_edit_ignore_caret`].\n\n[`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`] will be called if any carets were collapsed."]
        #[inline]
        pub fn collapse_carets(&mut self, from_line: i32, from_column: i32, to_line: i32, to_column: i32,) {
            self.collapse_carets_ex(from_line, from_column, to_line, to_column,) . done()
        }
        #[doc = "Collapse all carets in the given range to the `from_line` and `from_column` position.\n\n`inclusive` applies to both ends.\n\nIf [`is_in_mulitcaret_edit`][`crate::classes::TextEdit::is_in_mulitcaret_edit`] is `true`, carets that are collapsed will be `true` for [`multicaret_edit_ignore_caret`][`crate::classes::TextEdit::multicaret_edit_ignore_caret`].\n\n[`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`] will be called if any carets were collapsed."]
        #[inline]
        pub fn collapse_carets_ex < 'ex > (&'ex mut self, from_line: i32, from_column: i32, to_line: i32, to_column: i32,) -> ExCollapseCarets < 'ex > {
            ExCollapseCarets::new(self, from_line, from_column, to_line, to_column,)
        }
        #[doc = "Merges any overlapping carets. Will favor the newest caret, or the caret with a selection.\n\nIf [`is_in_mulitcaret_edit`][`crate::classes::TextEdit::is_in_mulitcaret_edit`] is `true`, the merge will be queued to happen at the end of the multicaret edit. See [`begin_multicaret_edit`][`crate::classes::TextEdit::begin_multicaret_edit`] and [`end_multicaret_edit`][`crate::classes::TextEdit::end_multicaret_edit`].\n\n**Note:** This is not called when a caret changes position but after certain actions, so it is possible to get into a state where carets overlap."]
        pub fn merge_overlapping_carets(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "merge_overlapping_carets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Starts an edit for multiple carets. The edit must be ended with [`end_multicaret_edit`][`crate::classes::TextEdit::end_multicaret_edit`]. Multicaret edits can be used to edit text at multiple carets and delay merging the carets until the end, so the caret indexes aren't affected immediately. [`begin_multicaret_edit`][`crate::classes::TextEdit::begin_multicaret_edit`] and [`end_multicaret_edit`][`crate::classes::TextEdit::end_multicaret_edit`] can be nested, and the merge will happen at the last [`end_multicaret_edit`][`crate::classes::TextEdit::end_multicaret_edit`].\n\n```gdscript\nbegin_complex_operation()\nbegin_multicaret_edit()\nfor i in range(get_caret_count()):\n\tif multicaret_edit_ignore_caret(i):\n\t\tcontinue\n\t# Logic here.\nend_multicaret_edit()\nend_complex_operation()\n```"]
        pub fn begin_multicaret_edit(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "begin_multicaret_edit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Ends an edit for multiple carets, that was started with [`begin_multicaret_edit`][`crate::classes::TextEdit::begin_multicaret_edit`]. If this was the last [`end_multicaret_edit`][`crate::classes::TextEdit::end_multicaret_edit`] and [`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`] was called, carets will be merged."]
        pub fn end_multicaret_edit(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "end_multicaret_edit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a [`begin_multicaret_edit`][`crate::classes::TextEdit::begin_multicaret_edit`] has been called and [`end_multicaret_edit`][`crate::classes::TextEdit::end_multicaret_edit`] has not yet been called."]
        pub fn is_in_mulitcaret_edit(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_in_mulitcaret_edit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `caret_index` should be ignored as part of a multicaret edit. See [`begin_multicaret_edit`][`crate::classes::TextEdit::begin_multicaret_edit`] and [`end_multicaret_edit`][`crate::classes::TextEdit::end_multicaret_edit`]. Carets that should be ignored are ones that were part of removed text and will likely be merged at the end of the edit, or carets that were added during the edit.\n\nIt is recommended to `continue` within a loop iterating on multiple carets if a caret should be ignored."]
        pub fn multicaret_edit_ignore_caret(&self, caret_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "multicaret_edit_ignore_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the caret is visible, `false` otherwise. A caret will be considered hidden if it is outside the scrollable area when scrolling is enabled.\n\n**Note:** [`is_caret_visible`][`crate::classes::TextEdit::is_caret_visible`] does not account for a caret being off-screen if it is still within the scrollable area. It will return `true` even if the caret is off-screen as long as it meets `TextEdit`'s own conditions for being visible. This includes uses of \\[member scroll_fit_content_width] and \\[member scroll_fit_content_height] that cause the `TextEdit` to expand beyond the viewport's bounds."]
        pub(crate) fn is_caret_visible_full(&self, caret_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_caret_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_caret_visible_ex`][Self::is_caret_visible_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the caret is visible, `false` otherwise. A caret will be considered hidden if it is outside the scrollable area when scrolling is enabled.\n\n**Note:** [`is_caret_visible`][`crate::classes::TextEdit::is_caret_visible`] does not account for a caret being off-screen if it is still within the scrollable area. It will return `true` even if the caret is off-screen as long as it meets `TextEdit`'s own conditions for being visible. This includes uses of \\[member scroll_fit_content_width] and \\[member scroll_fit_content_height] that cause the `TextEdit` to expand beyond the viewport's bounds."]
        #[inline]
        pub fn is_caret_visible(&self,) -> bool {
            self.is_caret_visible_ex() . done()
        }
        #[doc = "Returns `true` if the caret is visible, `false` otherwise. A caret will be considered hidden if it is outside the scrollable area when scrolling is enabled.\n\n**Note:** [`is_caret_visible`][`crate::classes::TextEdit::is_caret_visible`] does not account for a caret being off-screen if it is still within the scrollable area. It will return `true` even if the caret is off-screen as long as it meets `TextEdit`'s own conditions for being visible. This includes uses of \\[member scroll_fit_content_width] and \\[member scroll_fit_content_height] that cause the `TextEdit` to expand beyond the viewport's bounds."]
        #[inline]
        pub fn is_caret_visible_ex < 'ex > (&'ex self,) -> ExIsCaretVisible < 'ex > {
            ExIsCaretVisible::new(self,)
        }
        #[doc = "Returns the caret pixel draw position."]
        pub(crate) fn get_caret_draw_pos_full(&self, caret_index: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_draw_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_caret_draw_pos_ex`][Self::get_caret_draw_pos_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the caret pixel draw position."]
        #[inline]
        pub fn get_caret_draw_pos(&self,) -> Vector2 {
            self.get_caret_draw_pos_ex() . done()
        }
        #[doc = "Returns the caret pixel draw position."]
        #[inline]
        pub fn get_caret_draw_pos_ex < 'ex > (&'ex self,) -> ExGetCaretDrawPos < 'ex > {
            ExGetCaretDrawPos::new(self,)
        }
        #[doc = "Moves the caret to the specified `line` index. The caret column will be moved to the same visual position it was at the last time [`set_caret_column`][`crate::classes::TextEdit::set_caret_column`] was called, or clamped to the end of the line.\n\nIf `adjust_viewport` is `true`, the viewport will center at the caret position after the move occurs.\n\nIf `can_be_hidden` is `true`, the specified `line` can be hidden.\n\nIf `wrap_index` is `-1`, the caret column will be clamped to the `line`'s length. If `wrap_index` is greater than `-1`, the column will be moved to attempt to match the visual x position on the line's `wrap_index` to the position from the last time [`set_caret_column`][`crate::classes::TextEdit::set_caret_column`] was called.\n\n**Note:** If supporting multiple carets this will not check for any overlap. See [`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`]."]
        pub(crate) fn set_caret_line_full(&mut self, line: i32, adjust_viewport: bool, can_be_hidden: bool, wrap_index: i32, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, bool, bool, i32, i32,);
            let args = (line, adjust_viewport, can_be_hidden, wrap_index, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_caret_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_caret_line_ex`][Self::set_caret_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Moves the caret to the specified `line` index. The caret column will be moved to the same visual position it was at the last time [`set_caret_column`][`crate::classes::TextEdit::set_caret_column`] was called, or clamped to the end of the line.\n\nIf `adjust_viewport` is `true`, the viewport will center at the caret position after the move occurs.\n\nIf `can_be_hidden` is `true`, the specified `line` can be hidden.\n\nIf `wrap_index` is `-1`, the caret column will be clamped to the `line`'s length. If `wrap_index` is greater than `-1`, the column will be moved to attempt to match the visual x position on the line's `wrap_index` to the position from the last time [`set_caret_column`][`crate::classes::TextEdit::set_caret_column`] was called.\n\n**Note:** If supporting multiple carets this will not check for any overlap. See [`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`]."]
        #[inline]
        pub fn set_caret_line(&mut self, line: i32,) {
            self.set_caret_line_ex(line,) . done()
        }
        #[doc = "Moves the caret to the specified `line` index. The caret column will be moved to the same visual position it was at the last time [`set_caret_column`][`crate::classes::TextEdit::set_caret_column`] was called, or clamped to the end of the line.\n\nIf `adjust_viewport` is `true`, the viewport will center at the caret position after the move occurs.\n\nIf `can_be_hidden` is `true`, the specified `line` can be hidden.\n\nIf `wrap_index` is `-1`, the caret column will be clamped to the `line`'s length. If `wrap_index` is greater than `-1`, the column will be moved to attempt to match the visual x position on the line's `wrap_index` to the position from the last time [`set_caret_column`][`crate::classes::TextEdit::set_caret_column`] was called.\n\n**Note:** If supporting multiple carets this will not check for any overlap. See [`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`]."]
        #[inline]
        pub fn set_caret_line_ex < 'ex > (&'ex mut self, line: i32,) -> ExSetCaretLine < 'ex > {
            ExSetCaretLine::new(self, line,)
        }
        #[doc = "Returns the line the editing caret is on."]
        pub(crate) fn get_caret_line_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_caret_line_ex`][Self::get_caret_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the line the editing caret is on."]
        #[inline]
        pub fn get_caret_line(&self,) -> i32 {
            self.get_caret_line_ex() . done()
        }
        #[doc = "Returns the line the editing caret is on."]
        #[inline]
        pub fn get_caret_line_ex < 'ex > (&'ex self,) -> ExGetCaretLine < 'ex > {
            ExGetCaretLine::new(self,)
        }
        #[doc = "Moves the caret to the specified `column` index.\n\nIf `adjust_viewport` is `true`, the viewport will center at the caret position after the move occurs.\n\n**Note:** If supporting multiple carets this will not check for any overlap. See [`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`]."]
        pub(crate) fn set_caret_column_full(&mut self, column: i32, adjust_viewport: bool, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, bool, i32,);
            let args = (column, adjust_viewport, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_caret_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_caret_column_ex`][Self::set_caret_column_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Moves the caret to the specified `column` index.\n\nIf `adjust_viewport` is `true`, the viewport will center at the caret position after the move occurs.\n\n**Note:** If supporting multiple carets this will not check for any overlap. See [`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`]."]
        #[inline]
        pub fn set_caret_column(&mut self, column: i32,) {
            self.set_caret_column_ex(column,) . done()
        }
        #[doc = "Moves the caret to the specified `column` index.\n\nIf `adjust_viewport` is `true`, the viewport will center at the caret position after the move occurs.\n\n**Note:** If supporting multiple carets this will not check for any overlap. See [`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`]."]
        #[inline]
        pub fn set_caret_column_ex < 'ex > (&'ex mut self, column: i32,) -> ExSetCaretColumn < 'ex > {
            ExSetCaretColumn::new(self, column,)
        }
        #[doc = "Returns the column the editing caret is at."]
        pub(crate) fn get_caret_column_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_caret_column_ex`][Self::get_caret_column_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the column the editing caret is at."]
        #[inline]
        pub fn get_caret_column(&self,) -> i32 {
            self.get_caret_column_ex() . done()
        }
        #[doc = "Returns the column the editing caret is at."]
        #[inline]
        pub fn get_caret_column_ex < 'ex > (&'ex self,) -> ExGetCaretColumn < 'ex > {
            ExGetCaretColumn::new(self,)
        }
        #[doc = "Returns the correct column at the end of a composite character like ❤\u{fe0f}\u{200d}🩹 (mending heart; Unicode: `U+2764 U+FE0F U+200D U+1FA79`) which is comprised of more than one Unicode code point, if the caret is at the start of the composite character. Also returns the correct column with the caret at mid grapheme and for non-composite characters.\n\n**Note:** To check at caret location use `get_next_composite_character_column(get_caret_line(), get_caret_column())`"]
        pub fn get_next_composite_character_column(&self, line: i32, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_next_composite_character_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the correct column at the start of a composite character like ❤\u{fe0f}\u{200d}🩹 (mending heart; Unicode: `U+2764 U+FE0F U+200D U+1FA79`) which is comprised of more than one Unicode code point, if the caret is at the end of the composite character. Also returns the correct column with the caret at mid grapheme and for non-composite characters.\n\n**Note:** To check at caret location use `get_previous_composite_character_column(get_caret_line(), get_caret_column())`"]
        pub fn get_previous_composite_character_column(&self, line: i32, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_previous_composite_character_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the wrap index the editing caret is on."]
        pub(crate) fn get_caret_wrap_index_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_wrap_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_caret_wrap_index_ex`][Self::get_caret_wrap_index_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the wrap index the editing caret is on."]
        #[inline]
        pub fn get_caret_wrap_index(&self,) -> i32 {
            self.get_caret_wrap_index_ex() . done()
        }
        #[doc = "Returns the wrap index the editing caret is on."]
        #[inline]
        pub fn get_caret_wrap_index_ex < 'ex > (&'ex self,) -> ExGetCaretWrapIndex < 'ex > {
            ExGetCaretWrapIndex::new(self,)
        }
        #[doc = "Returns a [`String`][crate::builtin::GString] text with the word under the caret's location."]
        pub(crate) fn get_word_under_caret_full(&self, caret_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_word_under_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_word_under_caret_ex`][Self::get_word_under_caret_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a [`String`][crate::builtin::GString] text with the word under the caret's location."]
        #[inline]
        pub fn get_word_under_caret(&self,) -> GString {
            self.get_word_under_caret_ex() . done()
        }
        #[doc = "Returns a [`String`][crate::builtin::GString] text with the word under the caret's location."]
        #[inline]
        pub fn get_word_under_caret_ex < 'ex > (&'ex self,) -> ExGetWordUnderCaret < 'ex > {
            ExGetWordUnderCaret::new(self,)
        }
        pub fn set_use_default_word_separators(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_use_default_word_separators", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_default_word_separators_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_default_word_separators_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_custom_word_separators(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_use_custom_word_separators", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_custom_word_separators_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_custom_word_separators_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_word_separators(&mut self, custom_word_separators: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (custom_word_separators.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_custom_word_separators", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_word_separators(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_custom_word_separators", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_selecting_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_selecting_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_selecting_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_selecting_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_deselect_on_focus_loss_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_deselect_on_focus_loss_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_deselect_on_focus_loss_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_deselect_on_focus_loss_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_and_drop_selection_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_drag_and_drop_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drag_and_drop_selection_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_drag_and_drop_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the current selection mode."]
        pub fn set_selection_mode(&mut self, mode: crate::classes::text_edit::SelectionMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_edit::SelectionMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_selection_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current selection mode."]
        pub fn get_selection_mode(&self,) -> crate::classes::text_edit::SelectionMode {
            type CallRet = crate::classes::text_edit::SelectionMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Select all the text.\n\nIf \\[member selecting_enabled] is `false`, no selection will occur."]
        pub fn select_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "select_all", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Selects the word under the caret."]
        pub(crate) fn select_word_under_caret_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "select_word_under_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`select_word_under_caret_ex`][Self::select_word_under_caret_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Selects the word under the caret."]
        #[inline]
        pub fn select_word_under_caret(&mut self,) {
            self.select_word_under_caret_ex() . done()
        }
        #[doc = "Selects the word under the caret."]
        #[inline]
        pub fn select_word_under_caret_ex < 'ex > (&'ex mut self,) -> ExSelectWordUnderCaret < 'ex > {
            ExSelectWordUnderCaret::new(self,)
        }
        #[doc = "Adds a selection and a caret for the next occurrence of the current selection. If there is no active selection, selects word under caret."]
        pub fn add_selection_for_next_occurrence(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "add_selection_for_next_occurrence", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves a selection and a caret for the next occurrence of the current selection. If there is no active selection, moves to the next occurrence of the word under caret."]
        pub fn skip_selection_for_next_occurrence(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "skip_selection_for_next_occurrence", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Selects text from `origin_line` and `origin_column` to `caret_line` and `caret_column` for the given `caret_index`. This moves the selection origin and the caret. If the positions are the same, the selection will be deselected.\n\nIf \\[member selecting_enabled] is `false`, no selection will occur.\n\n**Note:** If supporting multiple carets this will not check for any overlap. See [`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`]."]
        pub(crate) fn select_full(&mut self, origin_line: i32, origin_column: i32, caret_line: i32, caret_column: i32, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32, i32, i32,);
            let args = (origin_line, origin_column, caret_line, caret_column, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "select", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`select_ex`][Self::select_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Selects text from `origin_line` and `origin_column` to `caret_line` and `caret_column` for the given `caret_index`. This moves the selection origin and the caret. If the positions are the same, the selection will be deselected.\n\nIf \\[member selecting_enabled] is `false`, no selection will occur.\n\n**Note:** If supporting multiple carets this will not check for any overlap. See [`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`]."]
        #[inline]
        pub fn select(&mut self, origin_line: i32, origin_column: i32, caret_line: i32, caret_column: i32,) {
            self.select_ex(origin_line, origin_column, caret_line, caret_column,) . done()
        }
        #[doc = "Selects text from `origin_line` and `origin_column` to `caret_line` and `caret_column` for the given `caret_index`. This moves the selection origin and the caret. If the positions are the same, the selection will be deselected.\n\nIf \\[member selecting_enabled] is `false`, no selection will occur.\n\n**Note:** If supporting multiple carets this will not check for any overlap. See [`merge_overlapping_carets`][`crate::classes::TextEdit::merge_overlapping_carets`]."]
        #[inline]
        pub fn select_ex < 'ex > (&'ex mut self, origin_line: i32, origin_column: i32, caret_line: i32, caret_column: i32,) -> ExSelect < 'ex > {
            ExSelect::new(self, origin_line, origin_column, caret_line, caret_column,)
        }
        #[doc = "Returns `true` if the user has selected text."]
        pub(crate) fn has_selection_full(&self, caret_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "has_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`has_selection_ex`][Self::has_selection_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the user has selected text."]
        #[inline]
        pub fn has_selection(&self,) -> bool {
            self.has_selection_ex() . done()
        }
        #[doc = "Returns `true` if the user has selected text."]
        #[inline]
        pub fn has_selection_ex < 'ex > (&'ex self,) -> ExHasSelection < 'ex > {
            ExHasSelection::new(self,)
        }
        #[doc = "Returns the text inside the selection of a caret, or all the carets if `caret_index` is its default value `-1`."]
        pub(crate) fn get_selected_text_full(&self, caret_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selected_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_selected_text_ex`][Self::get_selected_text_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the text inside the selection of a caret, or all the carets if `caret_index` is its default value `-1`."]
        #[inline]
        pub fn get_selected_text(&self,) -> GString {
            self.get_selected_text_ex() . done()
        }
        #[doc = "Returns the text inside the selection of a caret, or all the carets if `caret_index` is its default value `-1`."]
        #[inline]
        pub fn get_selected_text_ex < 'ex > (&'ex self,) -> ExGetSelectedText < 'ex > {
            ExGetSelectedText::new(self,)
        }
        #[doc = "Returns the caret index of the selection at the given `line` and `column`, or `-1` if there is none.\n\nIf `include_edges` is `false`, the position must be inside the selection and not at either end. If `only_selections` is `false`, carets without a selection will also be considered."]
        pub(crate) fn get_selection_at_line_column_full(&self, line: i32, column: i32, include_edges: bool, only_selections: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32, bool, bool,);
            let args = (line, column, include_edges, only_selections,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_at_line_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_selection_at_line_column_ex`][Self::get_selection_at_line_column_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the caret index of the selection at the given `line` and `column`, or `-1` if there is none.\n\nIf `include_edges` is `false`, the position must be inside the selection and not at either end. If `only_selections` is `false`, carets without a selection will also be considered."]
        #[inline]
        pub fn get_selection_at_line_column(&self, line: i32, column: i32,) -> i32 {
            self.get_selection_at_line_column_ex(line, column,) . done()
        }
        #[doc = "Returns the caret index of the selection at the given `line` and `column`, or `-1` if there is none.\n\nIf `include_edges` is `false`, the position must be inside the selection and not at either end. If `only_selections` is `false`, carets without a selection will also be considered."]
        #[inline]
        pub fn get_selection_at_line_column_ex < 'ex > (&'ex self, line: i32, column: i32,) -> ExGetSelectionAtLineColumn < 'ex > {
            ExGetSelectionAtLineColumn::new(self, line, column,)
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of line ranges where `x` is the first line and `y` is the last line. All lines within these ranges will have a caret on them or be part of a selection. Each line will only be part of one line range, even if it has multiple carets on it.\n\nIf a selection's end column ([`get_selection_to_column`][`crate::classes::TextEdit::get_selection_to_column`]) is at column `0`, that line will not be included. If a selection begins on the line after another selection ends and `merge_adjacent` is `true`, or they begin and end on the same line, one line range will include both selections."]
        pub(crate) fn get_line_ranges_from_carets_full(&self, only_selections: bool, merge_adjacent: bool,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (bool, bool,);
            let args = (only_selections, merge_adjacent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_ranges_from_carets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_line_ranges_from_carets_ex`][Self::get_line_ranges_from_carets_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an [`Array`][crate::builtin::Array] of line ranges where `x` is the first line and `y` is the last line. All lines within these ranges will have a caret on them or be part of a selection. Each line will only be part of one line range, even if it has multiple carets on it.\n\nIf a selection's end column ([`get_selection_to_column`][`crate::classes::TextEdit::get_selection_to_column`]) is at column `0`, that line will not be included. If a selection begins on the line after another selection ends and `merge_adjacent` is `true`, or they begin and end on the same line, one line range will include both selections."]
        #[inline]
        pub fn get_line_ranges_from_carets(&self,) -> Array < Vector2i > {
            self.get_line_ranges_from_carets_ex() . done()
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of line ranges where `x` is the first line and `y` is the last line. All lines within these ranges will have a caret on them or be part of a selection. Each line will only be part of one line range, even if it has multiple carets on it.\n\nIf a selection's end column ([`get_selection_to_column`][`crate::classes::TextEdit::get_selection_to_column`]) is at column `0`, that line will not be included. If a selection begins on the line after another selection ends and `merge_adjacent` is `true`, or they begin and end on the same line, one line range will include both selections."]
        #[inline]
        pub fn get_line_ranges_from_carets_ex < 'ex > (&'ex self,) -> ExGetLineRangesFromCarets < 'ex > {
            ExGetLineRangesFromCarets::new(self,)
        }
        #[doc = "Returns the origin line of the selection. This is the opposite end from the caret."]
        pub(crate) fn get_selection_origin_line_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_origin_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_selection_origin_line_ex`][Self::get_selection_origin_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the origin line of the selection. This is the opposite end from the caret."]
        #[inline]
        pub fn get_selection_origin_line(&self,) -> i32 {
            self.get_selection_origin_line_ex() . done()
        }
        #[doc = "Returns the origin line of the selection. This is the opposite end from the caret."]
        #[inline]
        pub fn get_selection_origin_line_ex < 'ex > (&'ex self,) -> ExGetSelectionOriginLine < 'ex > {
            ExGetSelectionOriginLine::new(self,)
        }
        #[doc = "Returns the origin column of the selection. This is the opposite end from the caret."]
        pub(crate) fn get_selection_origin_column_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_origin_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_selection_origin_column_ex`][Self::get_selection_origin_column_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the origin column of the selection. This is the opposite end from the caret."]
        #[inline]
        pub fn get_selection_origin_column(&self,) -> i32 {
            self.get_selection_origin_column_ex() . done()
        }
        #[doc = "Returns the origin column of the selection. This is the opposite end from the caret."]
        #[inline]
        pub fn get_selection_origin_column_ex < 'ex > (&'ex self,) -> ExGetSelectionOriginColumn < 'ex > {
            ExGetSelectionOriginColumn::new(self,)
        }
        #[doc = "Sets the selection origin line to the `line` for the given `caret_index`. If the selection origin is moved to the caret position, the selection will deselect.\n\nIf `can_be_hidden` is `false`, The line will be set to the nearest unhidden line below or above.\n\nIf `wrap_index` is `-1`, the selection origin column will be clamped to the `line`'s length. If `wrap_index` is greater than `-1`, the column will be moved to attempt to match the visual x position on the line's `wrap_index` to the position from the last time [`set_selection_origin_column`][`crate::classes::TextEdit::set_selection_origin_column`] or [`select`][`crate::classes::TextEdit::select`] was called."]
        pub(crate) fn set_selection_origin_line_full(&mut self, line: i32, can_be_hidden: bool, wrap_index: i32, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, bool, i32, i32,);
            let args = (line, can_be_hidden, wrap_index, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_selection_origin_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_selection_origin_line_ex`][Self::set_selection_origin_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the selection origin line to the `line` for the given `caret_index`. If the selection origin is moved to the caret position, the selection will deselect.\n\nIf `can_be_hidden` is `false`, The line will be set to the nearest unhidden line below or above.\n\nIf `wrap_index` is `-1`, the selection origin column will be clamped to the `line`'s length. If `wrap_index` is greater than `-1`, the column will be moved to attempt to match the visual x position on the line's `wrap_index` to the position from the last time [`set_selection_origin_column`][`crate::classes::TextEdit::set_selection_origin_column`] or [`select`][`crate::classes::TextEdit::select`] was called."]
        #[inline]
        pub fn set_selection_origin_line(&mut self, line: i32,) {
            self.set_selection_origin_line_ex(line,) . done()
        }
        #[doc = "Sets the selection origin line to the `line` for the given `caret_index`. If the selection origin is moved to the caret position, the selection will deselect.\n\nIf `can_be_hidden` is `false`, The line will be set to the nearest unhidden line below or above.\n\nIf `wrap_index` is `-1`, the selection origin column will be clamped to the `line`'s length. If `wrap_index` is greater than `-1`, the column will be moved to attempt to match the visual x position on the line's `wrap_index` to the position from the last time [`set_selection_origin_column`][`crate::classes::TextEdit::set_selection_origin_column`] or [`select`][`crate::classes::TextEdit::select`] was called."]
        #[inline]
        pub fn set_selection_origin_line_ex < 'ex > (&'ex mut self, line: i32,) -> ExSetSelectionOriginLine < 'ex > {
            ExSetSelectionOriginLine::new(self, line,)
        }
        #[doc = "Sets the selection origin column to the `column` for the given `caret_index`. If the selection origin is moved to the caret position, the selection will deselect."]
        pub(crate) fn set_selection_origin_column_full(&mut self, column: i32, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (column, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_selection_origin_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_selection_origin_column_ex`][Self::set_selection_origin_column_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the selection origin column to the `column` for the given `caret_index`. If the selection origin is moved to the caret position, the selection will deselect."]
        #[inline]
        pub fn set_selection_origin_column(&mut self, column: i32,) {
            self.set_selection_origin_column_ex(column,) . done()
        }
        #[doc = "Sets the selection origin column to the `column` for the given `caret_index`. If the selection origin is moved to the caret position, the selection will deselect."]
        #[inline]
        pub fn set_selection_origin_column_ex < 'ex > (&'ex mut self, column: i32,) -> ExSetSelectionOriginColumn < 'ex > {
            ExSetSelectionOriginColumn::new(self, column,)
        }
        #[doc = "Returns the selection begin line. Returns the caret line if there is no selection."]
        pub(crate) fn get_selection_from_line_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_from_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_selection_from_line_ex`][Self::get_selection_from_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the selection begin line. Returns the caret line if there is no selection."]
        #[inline]
        pub fn get_selection_from_line(&self,) -> i32 {
            self.get_selection_from_line_ex() . done()
        }
        #[doc = "Returns the selection begin line. Returns the caret line if there is no selection."]
        #[inline]
        pub fn get_selection_from_line_ex < 'ex > (&'ex self,) -> ExGetSelectionFromLine < 'ex > {
            ExGetSelectionFromLine::new(self,)
        }
        #[doc = "Returns the selection begin column. Returns the caret column if there is no selection."]
        pub(crate) fn get_selection_from_column_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_from_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_selection_from_column_ex`][Self::get_selection_from_column_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the selection begin column. Returns the caret column if there is no selection."]
        #[inline]
        pub fn get_selection_from_column(&self,) -> i32 {
            self.get_selection_from_column_ex() . done()
        }
        #[doc = "Returns the selection begin column. Returns the caret column if there is no selection."]
        #[inline]
        pub fn get_selection_from_column_ex < 'ex > (&'ex self,) -> ExGetSelectionFromColumn < 'ex > {
            ExGetSelectionFromColumn::new(self,)
        }
        #[doc = "Returns the selection end line. Returns the caret line if there is no selection."]
        pub(crate) fn get_selection_to_line_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_to_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_selection_to_line_ex`][Self::get_selection_to_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the selection end line. Returns the caret line if there is no selection."]
        #[inline]
        pub fn get_selection_to_line(&self,) -> i32 {
            self.get_selection_to_line_ex() . done()
        }
        #[doc = "Returns the selection end line. Returns the caret line if there is no selection."]
        #[inline]
        pub fn get_selection_to_line_ex < 'ex > (&'ex self,) -> ExGetSelectionToLine < 'ex > {
            ExGetSelectionToLine::new(self,)
        }
        #[doc = "Returns the selection end column. Returns the caret column if there is no selection."]
        pub(crate) fn get_selection_to_column_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_to_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_selection_to_column_ex`][Self::get_selection_to_column_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the selection end column. Returns the caret column if there is no selection."]
        #[inline]
        pub fn get_selection_to_column(&self,) -> i32 {
            self.get_selection_to_column_ex() . done()
        }
        #[doc = "Returns the selection end column. Returns the caret column if there is no selection."]
        #[inline]
        pub fn get_selection_to_column_ex < 'ex > (&'ex self,) -> ExGetSelectionToColumn < 'ex > {
            ExGetSelectionToColumn::new(self,)
        }
        #[doc = "Returns `true` if the caret of the selection is after the selection origin. This can be used to determine the direction of the selection."]
        pub(crate) fn is_caret_after_selection_origin_full(&self, caret_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_caret_after_selection_origin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_caret_after_selection_origin_ex`][Self::is_caret_after_selection_origin_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the caret of the selection is after the selection origin. This can be used to determine the direction of the selection."]
        #[inline]
        pub fn is_caret_after_selection_origin(&self,) -> bool {
            self.is_caret_after_selection_origin_ex() . done()
        }
        #[doc = "Returns `true` if the caret of the selection is after the selection origin. This can be used to determine the direction of the selection."]
        #[inline]
        pub fn is_caret_after_selection_origin_ex < 'ex > (&'ex self,) -> ExIsCaretAfterSelectionOrigin < 'ex > {
            ExIsCaretAfterSelectionOrigin::new(self,)
        }
        #[doc = "Deselects the current selection."]
        pub(crate) fn deselect_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "deselect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`deselect_ex`][Self::deselect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Deselects the current selection."]
        #[inline]
        pub fn deselect(&mut self,) {
            self.deselect_ex() . done()
        }
        #[doc = "Deselects the current selection."]
        #[inline]
        pub fn deselect_ex < 'ex > (&'ex mut self,) -> ExDeselect < 'ex > {
            ExDeselect::new(self,)
        }
        #[doc = "Deletes the selected text."]
        pub(crate) fn delete_selection_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "delete_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`delete_selection_ex`][Self::delete_selection_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Deletes the selected text."]
        #[inline]
        pub fn delete_selection(&mut self,) {
            self.delete_selection_ex() . done()
        }
        #[doc = "Deletes the selected text."]
        #[inline]
        pub fn delete_selection_ex < 'ex > (&'ex mut self,) -> ExDeleteSelection < 'ex > {
            ExDeleteSelection::new(self,)
        }
        pub fn set_line_wrapping_mode(&mut self, mode: crate::classes::text_edit::LineWrappingMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_edit::LineWrappingMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_wrapping_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_wrapping_mode(&self,) -> crate::classes::text_edit::LineWrappingMode {
            type CallRet = crate::classes::text_edit::LineWrappingMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_wrapping_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::AutowrapMode,);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::classes::text_server::AutowrapMode {
            type CallRet = crate::classes::text_server::AutowrapMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if the given line is wrapped."]
        pub fn is_line_wrapped(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_line_wrapped", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of times the given line is wrapped."]
        pub fn get_line_wrap_count(&self, line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_wrap_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the wrap index of the given column on the given line. This ranges from `0` to [`get_line_wrap_count`][`crate::classes::TextEdit::get_line_wrap_count`]."]
        pub fn get_line_wrap_index_at_column(&self, line: i32, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_wrap_index_at_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of [`String`][crate::builtin::GString]s representing each wrapped index."]
        pub fn get_line_wrapped_text(&self, line: i32,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_wrapped_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_smooth_scroll_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_smooth_scroll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_smooth_scroll_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_smooth_scroll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`VScrollBar`][crate::classes::VScrollBar] of the `TextEdit`."]
        pub fn get_v_scroll_bar(&self,) -> Option < Gd < crate::classes::VScrollBar > > {
            type CallRet = Option < Gd < crate::classes::VScrollBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_v_scroll_bar", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`HScrollBar`][crate::classes::HScrollBar] used by `TextEdit`."]
        pub fn get_h_scroll_bar(&self,) -> Option < Gd < crate::classes::HScrollBar > > {
            type CallRet = Option < Gd < crate::classes::HScrollBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_h_scroll_bar", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_v_scroll(&mut self, value: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_v_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_scroll(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_v_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_h_scroll(&mut self, value: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_h_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_h_scroll(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_h_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scroll_past_end_of_file_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_scroll_past_end_of_file_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scroll_past_end_of_file_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_scroll_past_end_of_file_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_v_scroll_speed(&mut self, speed: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_v_scroll_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_scroll_speed(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_v_scroll_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fit_content_height_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_fit_content_height_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_fit_content_height_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_fit_content_height_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fit_content_width_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_fit_content_width_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_fit_content_width_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_fit_content_width_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the scroll position for `wrap_index` of `line`."]
        pub(crate) fn get_scroll_pos_for_line_full(&self, line: i32, wrap_index: i32,) -> f64 {
            type CallRet = f64;
            type CallParams = (i32, i32,);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_scroll_pos_for_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_scroll_pos_for_line_ex`][Self::get_scroll_pos_for_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the scroll position for `wrap_index` of `line`."]
        #[inline]
        pub fn get_scroll_pos_for_line(&self, line: i32,) -> f64 {
            self.get_scroll_pos_for_line_ex(line,) . done()
        }
        #[doc = "Returns the scroll position for `wrap_index` of `line`."]
        #[inline]
        pub fn get_scroll_pos_for_line_ex < 'ex > (&'ex self, line: i32,) -> ExGetScrollPosForLine < 'ex > {
            ExGetScrollPosForLine::new(self, line,)
        }
        #[doc = "Positions the `wrap_index` of `line` at the top of the viewport."]
        pub(crate) fn set_line_as_first_visible_full(&mut self, line: i32, wrap_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_as_first_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_line_as_first_visible_ex`][Self::set_line_as_first_visible_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Positions the `wrap_index` of `line` at the top of the viewport."]
        #[inline]
        pub fn set_line_as_first_visible(&mut self, line: i32,) {
            self.set_line_as_first_visible_ex(line,) . done()
        }
        #[doc = "Positions the `wrap_index` of `line` at the top of the viewport."]
        #[inline]
        pub fn set_line_as_first_visible_ex < 'ex > (&'ex mut self, line: i32,) -> ExSetLineAsFirstVisible < 'ex > {
            ExSetLineAsFirstVisible::new(self, line,)
        }
        #[doc = "Returns the first visible line."]
        pub fn get_first_visible_line(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_first_visible_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Positions the `wrap_index` of `line` at the center of the viewport."]
        pub(crate) fn set_line_as_center_visible_full(&mut self, line: i32, wrap_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_as_center_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_line_as_center_visible_ex`][Self::set_line_as_center_visible_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Positions the `wrap_index` of `line` at the center of the viewport."]
        #[inline]
        pub fn set_line_as_center_visible(&mut self, line: i32,) {
            self.set_line_as_center_visible_ex(line,) . done()
        }
        #[doc = "Positions the `wrap_index` of `line` at the center of the viewport."]
        #[inline]
        pub fn set_line_as_center_visible_ex < 'ex > (&'ex mut self, line: i32,) -> ExSetLineAsCenterVisible < 'ex > {
            ExSetLineAsCenterVisible::new(self, line,)
        }
        #[doc = "Positions the `wrap_index` of `line` at the bottom of the viewport."]
        pub(crate) fn set_line_as_last_visible_full(&mut self, line: i32, wrap_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_as_last_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_line_as_last_visible_ex`][Self::set_line_as_last_visible_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Positions the `wrap_index` of `line` at the bottom of the viewport."]
        #[inline]
        pub fn set_line_as_last_visible(&mut self, line: i32,) {
            self.set_line_as_last_visible_ex(line,) . done()
        }
        #[doc = "Positions the `wrap_index` of `line` at the bottom of the viewport."]
        #[inline]
        pub fn set_line_as_last_visible_ex < 'ex > (&'ex mut self, line: i32,) -> ExSetLineAsLastVisible < 'ex > {
            ExSetLineAsLastVisible::new(self, line,)
        }
        #[doc = "Returns the last visible line. Use [`get_last_full_visible_line_wrap_index`][`crate::classes::TextEdit::get_last_full_visible_line_wrap_index`] for the wrap index."]
        pub fn get_last_full_visible_line(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_last_full_visible_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the last visible wrap index of the last visible line."]
        pub fn get_last_full_visible_line_wrap_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_last_full_visible_line_wrap_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of lines that can visually fit, rounded down, based on this control's height."]
        pub fn get_visible_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_visible_line_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total number of lines between `from_line` and `to_line` (inclusive) in the text. This includes wrapped lines and excludes folded lines. If the range covers all lines it is equivalent to [`get_total_visible_line_count`][`crate::classes::TextEdit::get_total_visible_line_count`]."]
        pub fn get_visible_line_count_in_range(&self, from_line: i32, to_line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_visible_line_count_in_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total number of lines in the text. This includes wrapped lines and excludes folded lines. If \\[member wrap_mode] is set to [`LineWrappingMode::NONE`][`crate::classes::text_edit::LineWrappingMode::NONE`] and no lines are folded (see [`is_line_folded`][`crate::classes::CodeEdit::is_line_folded`]) then this is equivalent to [`get_line_count`][`crate::classes::TextEdit::get_line_count`]. See [`get_visible_line_count_in_range`][`crate::classes::TextEdit::get_visible_line_count_in_range`] for a limited range of lines."]
        pub fn get_total_visible_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_total_visible_line_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adjust the viewport so the caret is visible."]
        pub(crate) fn adjust_viewport_to_caret_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "adjust_viewport_to_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`adjust_viewport_to_caret_ex`][Self::adjust_viewport_to_caret_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adjust the viewport so the caret is visible."]
        #[inline]
        pub fn adjust_viewport_to_caret(&mut self,) {
            self.adjust_viewport_to_caret_ex() . done()
        }
        #[doc = "Adjust the viewport so the caret is visible."]
        #[inline]
        pub fn adjust_viewport_to_caret_ex < 'ex > (&'ex mut self,) -> ExAdjustViewportToCaret < 'ex > {
            ExAdjustViewportToCaret::new(self,)
        }
        #[doc = "Centers the viewport on the line the editing caret is at. This also resets the \\[member scroll_horizontal] value to `0`."]
        pub(crate) fn center_viewport_to_caret_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "center_viewport_to_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`center_viewport_to_caret_ex`][Self::center_viewport_to_caret_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Centers the viewport on the line the editing caret is at. This also resets the \\[member scroll_horizontal] value to `0`."]
        #[inline]
        pub fn center_viewport_to_caret(&mut self,) {
            self.center_viewport_to_caret_ex() . done()
        }
        #[doc = "Centers the viewport on the line the editing caret is at. This also resets the \\[member scroll_horizontal] value to `0`."]
        #[inline]
        pub fn center_viewport_to_caret_ex < 'ex > (&'ex mut self,) -> ExCenterViewportToCaret < 'ex > {
            ExCenterViewportToCaret::new(self,)
        }
        pub fn set_draw_minimap(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_draw_minimap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_minimap(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_drawing_minimap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_minimap_width(&mut self, width: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_minimap_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_minimap_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_minimap_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of lines that may be drawn on the minimap."]
        pub fn get_minimap_visible_lines(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_minimap_visible_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Register a new gutter to this `TextEdit`. Use `at` to have a specific gutter order. A value of `-1` appends the gutter to the right."]
        pub(crate) fn add_gutter_full(&mut self, at: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (at,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "add_gutter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_gutter_ex`][Self::add_gutter_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Register a new gutter to this `TextEdit`. Use `at` to have a specific gutter order. A value of `-1` appends the gutter to the right."]
        #[inline]
        pub fn add_gutter(&mut self,) {
            self.add_gutter_ex() . done()
        }
        #[doc = "Register a new gutter to this `TextEdit`. Use `at` to have a specific gutter order. A value of `-1` appends the gutter to the right."]
        #[inline]
        pub fn add_gutter_ex < 'ex > (&'ex mut self,) -> ExAddGutter < 'ex > {
            ExAddGutter::new(self,)
        }
        #[doc = "Removes the gutter at the given index."]
        pub fn remove_gutter(&mut self, gutter: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "remove_gutter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of gutters registered."]
        pub fn get_gutter_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_gutter_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the name of the gutter at the given index."]
        pub fn set_gutter_name(&mut self, gutter: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (gutter, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the gutter at the given index."]
        pub fn get_gutter_name(&self, gutter: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_gutter_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the type of gutter at the given index. Gutters can contain icons, text, or custom visuals."]
        pub fn set_gutter_type(&mut self, gutter: i32, type_: crate::classes::text_edit::GutterType,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::text_edit::GutterType,);
            let args = (gutter, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the type of the gutter at the given index. Gutters can contain icons, text, or custom visuals."]
        pub fn get_gutter_type(&self, gutter: i32,) -> crate::classes::text_edit::GutterType {
            type CallRet = crate::classes::text_edit::GutterType;
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_gutter_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set the width of the gutter at the given index."]
        pub fn set_gutter_width(&mut self, gutter: i32, width: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (gutter, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the width of the gutter at the given index."]
        pub fn get_gutter_width(&self, gutter: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_gutter_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the gutter at the given index is drawn. The gutter type ([`set_gutter_type`][`crate::classes::TextEdit::set_gutter_type`]) determines how it is drawn. See [`is_gutter_drawn`][`crate::classes::TextEdit::is_gutter_drawn`]."]
        pub fn set_gutter_draw(&mut self, gutter: i32, draw: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (gutter, draw,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the gutter at the given index is currently drawn. See [`set_gutter_draw`][`crate::classes::TextEdit::set_gutter_draw`]."]
        pub fn is_gutter_drawn(&self, gutter: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_gutter_drawn", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the mouse cursor will change to a pointing hand ([`CursorShape::POINTING_HAND`][`crate::classes::control::CursorShape::POINTING_HAND`]) when hovering over the gutter at the given index. See [`is_gutter_clickable`][`crate::classes::TextEdit::is_gutter_clickable`] and [`set_line_gutter_clickable`][`crate::classes::TextEdit::set_line_gutter_clickable`]."]
        pub fn set_gutter_clickable(&mut self, gutter: i32, clickable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (gutter, clickable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_clickable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the gutter at the given index is clickable. See [`set_gutter_clickable`][`crate::classes::TextEdit::set_gutter_clickable`]."]
        pub fn is_gutter_clickable(&self, gutter: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_gutter_clickable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the line data of the gutter at the given index can be overridden when using [`merge_gutters`][`crate::classes::TextEdit::merge_gutters`]. See [`is_gutter_overwritable`][`crate::classes::TextEdit::is_gutter_overwritable`]."]
        pub fn set_gutter_overwritable(&mut self, gutter: i32, overwritable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (gutter, overwritable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_overwritable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the gutter at the given index is overwritable. See [`set_gutter_overwritable`][`crate::classes::TextEdit::set_gutter_overwritable`]."]
        pub fn is_gutter_overwritable(&self, gutter: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_gutter_overwritable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Merge the gutters from `from_line` into `to_line`. Only overwritable gutters will be copied. See [`set_gutter_overwritable`][`crate::classes::TextEdit::set_gutter_overwritable`]."]
        pub fn merge_gutters(&mut self, from_line: i32, to_line: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "merge_gutters", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set a custom draw callback for the gutter at the given index. `draw_callback` must take the following arguments: A line index `int`, a gutter index `int`, and an area [`Rect2`][crate::builtin::Rect2]. This callback only works when the gutter type is [`GutterType::CUSTOM`][`crate::classes::text_edit::GutterType::CUSTOM`] (see [`set_gutter_type`][`crate::classes::TextEdit::set_gutter_type`])."]
        pub fn set_gutter_custom_draw(&mut self, column: i32, draw_callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Callable >,);
            let args = (column, RefArg::new(draw_callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_custom_draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total width of all gutters and internal padding."]
        pub fn get_total_gutter_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_total_gutter_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the metadata for `gutter` on `line` to `metadata`."]
        pub fn set_line_gutter_metadata(&mut self, line: i32, gutter: i32, metadata: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, RefArg < 'a0, Variant >,);
            let args = (line, gutter, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_gutter_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the metadata currently in `gutter` at `line`."]
        pub fn get_line_gutter_metadata(&self, line: i32, gutter: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32, i32,);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_gutter_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the text for `gutter` on `line` to `text`. This only works when the gutter type is [`GutterType::STRING`][`crate::classes::text_edit::GutterType::STRING`] (see [`set_gutter_type`][`crate::classes::TextEdit::set_gutter_type`])."]
        pub fn set_line_gutter_text(&mut self, line: i32, gutter: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (line, gutter, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_gutter_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text currently in `gutter` at `line`. This only works when the gutter type is [`GutterType::STRING`][`crate::classes::text_edit::GutterType::STRING`] (see [`set_gutter_type`][`crate::classes::TextEdit::set_gutter_type`])."]
        pub fn get_line_gutter_text(&self, line: i32, gutter: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_gutter_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the icon for `gutter` on `line` to `icon`. This only works when the gutter type is [`GutterType::ICON`][`crate::classes::text_edit::GutterType::ICON`] (see [`set_gutter_type`][`crate::classes::TextEdit::set_gutter_type`])."]
        pub fn set_line_gutter_icon(&mut self, line: i32, gutter: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (line, gutter, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_gutter_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the icon currently in `gutter` at `line`. This only works when the gutter type is [`GutterType::ICON`][`crate::classes::text_edit::GutterType::ICON`] (see [`set_gutter_type`][`crate::classes::TextEdit::set_gutter_type`])."]
        pub fn get_line_gutter_icon(&self, line: i32, gutter: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32, i32,);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_gutter_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the color for `gutter` on `line` to `color`."]
        pub fn set_line_gutter_item_color(&mut self, line: i32, gutter: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, i32, Color,);
            let args = (line, gutter, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_gutter_item_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the color currently in `gutter` at `line`."]
        pub fn get_line_gutter_item_color(&self, line: i32, gutter: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32, i32,);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_gutter_item_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `clickable` is `true`, makes the `gutter` on the given `line` clickable. This is like [`set_gutter_clickable`][`crate::classes::TextEdit::set_gutter_clickable`], but for a single line. If [`is_gutter_clickable`][`crate::classes::TextEdit::is_gutter_clickable`] is `true`, this will not have any effect. See [`is_line_gutter_clickable`][`crate::classes::TextEdit::is_line_gutter_clickable`] and `gutter_clicked`."]
        pub fn set_line_gutter_clickable(&mut self, line: i32, gutter: i32, clickable: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, bool,);
            let args = (line, gutter, clickable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_gutter_clickable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the gutter at the given index on the given line is clickable. See [`set_line_gutter_clickable`][`crate::classes::TextEdit::set_line_gutter_clickable`]."]
        pub fn is_line_gutter_clickable(&self, line: i32, gutter: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_line_gutter_clickable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the custom background color of the given line. If transparent, this color is applied on top of the default background color (See [theme_item background_color]). If set to `Color(0, 0, 0, 0)`, no additional color is applied."]
        pub fn set_line_background_color(&mut self, line: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (line, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_background_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the custom background color of the given line. If no color is set, returns `Color(0, 0, 0, 0)`."]
        pub fn get_line_background_color(&self, line: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_background_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_syntax_highlighter(&mut self, syntax_highlighter: impl AsArg < Option < Gd < crate::classes::SyntaxHighlighter >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::SyntaxHighlighter > > >,);
            let args = (syntax_highlighter.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_syntax_highlighter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_syntax_highlighter(&self,) -> Option < Gd < crate::classes::SyntaxHighlighter > > {
            type CallRet = Option < Gd < crate::classes::SyntaxHighlighter > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_syntax_highlighter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_highlight_current_line(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_highlight_current_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_highlight_current_line_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_highlight_current_line_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_highlight_all_occurrences(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_highlight_all_occurrences", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_highlight_all_occurrences_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_highlight_all_occurrences_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_draw_control_chars(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_draw_control_chars", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_control_chars(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_draw_control_chars", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_tabs(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_draw_tabs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_tabs(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_drawing_tabs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_spaces(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_draw_spaces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_spaces(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_drawing_spaces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`PopupMenu`][crate::classes::PopupMenu] of this `TextEdit`. By default, this menu is displayed when right-clicking on the `TextEdit`.\n\nYou can add custom menu items or remove standard ones. Make sure your IDs don't conflict with the standard ones (see \\[enum MenuItems]). For example:\n\n\n```gdscript\nfunc _ready():\n\tvar menu = get_menu()\n\t# Remove all items after \"Redo\".\n\tmenu.item_count = menu.get_item_index(MENU_REDO) + 1\n\t# Add custom items.\n\tmenu.add_separator()\n\tmenu.add_item(\"Insert Date\", MENU_MAX + 1)\n\t# Connect callback.\n\tmenu.id_pressed.connect(_on_item_pressed)\n\nfunc _on_item_pressed(id):\n\tif id == MENU_MAX + 1:\n\t\tinsert_text_at_caret(Time.get_date_string_from_system())\n```\n\n\n**Warning:** This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their \\[member Window.visible] property."]
        pub fn get_menu(&self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallRet = Option < Gd < crate::classes::PopupMenu > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the menu is visible. Use this instead of `get_menu().visible` to improve performance (so the creation of the menu is avoided). See [`get_menu`][`crate::classes::TextEdit::get_menu`]."]
        pub fn is_menu_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_menu_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Executes a given action as defined in the \\[enum MenuItems] enum."]
        pub fn menu_option(&mut self, option: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "menu_option", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This method does nothing."]
        pub fn adjust_carets_after_edit(&mut self, caret: i32, from_line: i32, from_col: i32, to_line: i32, to_col: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32, i32, i32,);
            let args = (caret, from_line, from_col, to_line, to_col,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "adjust_carets_after_edit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of caret indexes in their edit order, this done from bottom to top. Edit order refers to the way actions such as [`insert_text_at_caret`][`crate::classes::TextEdit::insert_text_at_caret`] are applied."]
        pub fn get_caret_index_edit_order(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_index_edit_order", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the original start line of the selection."]
        pub(crate) fn get_selection_line_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_selection_line_ex`][Self::get_selection_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the original start line of the selection."]
        #[inline]
        pub fn get_selection_line(&self,) -> i32 {
            self.get_selection_line_ex() . done()
        }
        #[doc = "Returns the original start line of the selection."]
        #[inline]
        pub fn get_selection_line_ex < 'ex > (&'ex self,) -> ExGetSelectionLine < 'ex > {
            ExGetSelectionLine::new(self,)
        }
        #[doc = "Returns the original start column of the selection."]
        pub(crate) fn get_selection_column_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_selection_column_ex`][Self::get_selection_column_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the original start column of the selection."]
        #[inline]
        pub fn get_selection_column(&self,) -> i32 {
            self.get_selection_column_ex() . done()
        }
        #[doc = "Returns the original start column of the selection."]
        #[inline]
        pub fn get_selection_column_ex < 'ex > (&'ex self,) -> ExGetSelectionColumn < 'ex > {
            ExGetSelectionColumn::new(self,)
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
    impl crate::obj::GodotClass for TextEdit {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextEdit"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextEdit {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for TextEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TextEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TextEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextEdit {
        
    }
    impl crate::obj::cap::GodotDefault for TextEdit {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextEdit {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextEdit {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextEdit`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextEdit__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextEdit > for $Class {
                
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
#[doc = "Default-param extender for [`TextEdit::get_line_width_ex`][super::TextEdit::get_line_width_ex]."]
#[must_use]
pub struct ExGetLineWidth < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetLineWidth < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit, line: i32,) -> Self {
        let wrap_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, wrap_index: wrap_index,
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line, wrap_index,
        }
        = self;
        re_export::TextEdit::get_line_width_full(surround_object, line, wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::remove_line_at_ex`][super::TextEdit::remove_line_at_ex]."]
#[must_use]
pub struct ExRemoveLineAt < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, line: i32, move_carets_down: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExRemoveLineAt < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit, line: i32,) -> Self {
        let move_carets_down = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, move_carets_down: move_carets_down,
        }
    }
    #[inline]
    pub fn move_carets_down(self, move_carets_down: bool) -> Self {
        Self {
            move_carets_down: move_carets_down, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, line, move_carets_down,
        }
        = self;
        re_export::TextEdit::remove_line_at_full(surround_object, line, move_carets_down,)
    }
}
#[doc = "Default-param extender for [`TextEdit::insert_text_at_caret_ex`][super::TextEdit::insert_text_at_caret_ex]."]
#[must_use]
pub struct ExInsertTextAtCaret < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, text: CowArg < 'ex, GString >, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExInsertTextAtCaret < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit, text: impl AsArg < GString > + 'ex,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text, caret_index,
        }
        = self;
        re_export::TextEdit::insert_text_at_caret_full(surround_object, text, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::insert_text_ex`][super::TextEdit::insert_text_ex]."]
#[must_use]
pub struct ExInsertText < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, text: CowArg < 'ex, GString >, line: i32, column: i32, before_selection_begin: bool, before_selection_end: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExInsertText < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit, text: impl AsArg < GString > + 'ex, line: i32, column: i32,) -> Self {
        let before_selection_begin = true;
        let before_selection_end = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), line: line, column: column, before_selection_begin: before_selection_begin, before_selection_end: before_selection_end,
        }
    }
    #[inline]
    pub fn before_selection_begin(self, before_selection_begin: bool) -> Self {
        Self {
            before_selection_begin: before_selection_begin, .. self
        }
    }
    #[inline]
    pub fn before_selection_end(self, before_selection_end: bool) -> Self {
        Self {
            before_selection_end: before_selection_end, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text, line, column, before_selection_begin, before_selection_end,
        }
        = self;
        re_export::TextEdit::insert_text_full(surround_object, text, line, column, before_selection_begin, before_selection_end,)
    }
}
#[doc = "Default-param extender for [`TextEdit::backspace_ex`][super::TextEdit::backspace_ex]."]
#[must_use]
pub struct ExBackspace < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBackspace < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::backspace_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::cut_ex`][super::TextEdit::cut_ex]."]
#[must_use]
pub struct ExCut < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCut < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::cut_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::copy_ex`][super::TextEdit::copy_ex]."]
#[must_use]
pub struct ExCopy < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCopy < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::copy_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::paste_ex`][super::TextEdit::paste_ex]."]
#[must_use]
pub struct ExPaste < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPaste < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::paste_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::paste_primary_clipboard_ex`][super::TextEdit::paste_primary_clipboard_ex]."]
#[must_use]
pub struct ExPastePrimaryClipboard < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPastePrimaryClipboard < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::paste_primary_clipboard_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_line_column_at_pos_ex`][super::TextEdit::get_line_column_at_pos_ex]."]
#[must_use]
pub struct ExGetLineColumnAtPos < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, position: Vector2i, clamp_line: bool, clamp_column: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetLineColumnAtPos < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit, position: Vector2i,) -> Self {
        let clamp_line = true;
        let clamp_column = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, clamp_line: clamp_line, clamp_column: clamp_column,
        }
    }
    #[inline]
    pub fn clamp_line(self, clamp_line: bool) -> Self {
        Self {
            clamp_line: clamp_line, .. self
        }
    }
    #[inline]
    pub fn clamp_column(self, clamp_column: bool) -> Self {
        Self {
            clamp_column: clamp_column, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, position, clamp_line, clamp_column,
        }
        = self;
        re_export::TextEdit::get_line_column_at_pos_full(surround_object, position, clamp_line, clamp_column,)
    }
}
#[doc = "Default-param extender for [`TextEdit::is_mouse_over_selection_ex`][super::TextEdit::is_mouse_over_selection_ex]."]
#[must_use]
pub struct ExIsMouseOverSelection < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, edges: bool, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsMouseOverSelection < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit, edges: bool,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, edges: edges, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, edges, caret_index,
        }
        = self;
        re_export::TextEdit::is_mouse_over_selection_full(surround_object, edges, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_sorted_carets_ex`][super::TextEdit::get_sorted_carets_ex]."]
#[must_use]
pub struct ExGetSortedCarets < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, include_ignored_carets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSortedCarets < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let include_ignored_carets = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_ignored_carets: include_ignored_carets,
        }
    }
    #[inline]
    pub fn include_ignored_carets(self, include_ignored_carets: bool) -> Self {
        Self {
            include_ignored_carets: include_ignored_carets, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, include_ignored_carets,
        }
        = self;
        re_export::TextEdit::get_sorted_carets_full(surround_object, include_ignored_carets,)
    }
}
#[doc = "Default-param extender for [`TextEdit::collapse_carets_ex`][super::TextEdit::collapse_carets_ex]."]
#[must_use]
pub struct ExCollapseCarets < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, from_line: i32, from_column: i32, to_line: i32, to_column: i32, inclusive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCollapseCarets < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit, from_line: i32, from_column: i32, to_line: i32, to_column: i32,) -> Self {
        let inclusive = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_line: from_line, from_column: from_column, to_line: to_line, to_column: to_column, inclusive: inclusive,
        }
    }
    #[inline]
    pub fn inclusive(self, inclusive: bool) -> Self {
        Self {
            inclusive: inclusive, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_line, from_column, to_line, to_column, inclusive,
        }
        = self;
        re_export::TextEdit::collapse_carets_full(surround_object, from_line, from_column, to_line, to_column, inclusive,)
    }
}
#[doc = "Default-param extender for [`TextEdit::is_caret_visible_ex`][super::TextEdit::is_caret_visible_ex]."]
#[must_use]
pub struct ExIsCaretVisible < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsCaretVisible < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::is_caret_visible_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_caret_draw_pos_ex`][super::TextEdit::get_caret_draw_pos_ex]."]
#[must_use]
pub struct ExGetCaretDrawPos < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetCaretDrawPos < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_caret_draw_pos_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_caret_line_ex`][super::TextEdit::set_caret_line_ex]."]
#[must_use]
pub struct ExSetCaretLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, line: i32, adjust_viewport: bool, can_be_hidden: bool, wrap_index: i32, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetCaretLine < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit, line: i32,) -> Self {
        let adjust_viewport = true;
        let can_be_hidden = true;
        let wrap_index = 0i32;
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, adjust_viewport: adjust_viewport, can_be_hidden: can_be_hidden, wrap_index: wrap_index, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn adjust_viewport(self, adjust_viewport: bool) -> Self {
        Self {
            adjust_viewport: adjust_viewport, .. self
        }
    }
    #[inline]
    pub fn can_be_hidden(self, can_be_hidden: bool) -> Self {
        Self {
            can_be_hidden: can_be_hidden, .. self
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, line, adjust_viewport, can_be_hidden, wrap_index, caret_index,
        }
        = self;
        re_export::TextEdit::set_caret_line_full(surround_object, line, adjust_viewport, can_be_hidden, wrap_index, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_caret_line_ex`][super::TextEdit::get_caret_line_ex]."]
#[must_use]
pub struct ExGetCaretLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetCaretLine < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_caret_line_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_caret_column_ex`][super::TextEdit::set_caret_column_ex]."]
#[must_use]
pub struct ExSetCaretColumn < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, column: i32, adjust_viewport: bool, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetCaretColumn < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit, column: i32,) -> Self {
        let adjust_viewport = true;
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, adjust_viewport: adjust_viewport, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn adjust_viewport(self, adjust_viewport: bool) -> Self {
        Self {
            adjust_viewport: adjust_viewport, .. self
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, adjust_viewport, caret_index,
        }
        = self;
        re_export::TextEdit::set_caret_column_full(surround_object, column, adjust_viewport, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_caret_column_ex`][super::TextEdit::get_caret_column_ex]."]
#[must_use]
pub struct ExGetCaretColumn < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetCaretColumn < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_caret_column_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_caret_wrap_index_ex`][super::TextEdit::get_caret_wrap_index_ex]."]
#[must_use]
pub struct ExGetCaretWrapIndex < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetCaretWrapIndex < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_caret_wrap_index_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_word_under_caret_ex`][super::TextEdit::get_word_under_caret_ex]."]
#[must_use]
pub struct ExGetWordUnderCaret < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetWordUnderCaret < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_word_under_caret_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::select_word_under_caret_ex`][super::TextEdit::select_word_under_caret_ex]."]
#[must_use]
pub struct ExSelectWordUnderCaret < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSelectWordUnderCaret < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::select_word_under_caret_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::select_ex`][super::TextEdit::select_ex]."]
#[must_use]
pub struct ExSelect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, origin_line: i32, origin_column: i32, caret_line: i32, caret_column: i32, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSelect < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit, origin_line: i32, origin_column: i32, caret_line: i32, caret_column: i32,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, origin_line: origin_line, origin_column: origin_column, caret_line: caret_line, caret_column: caret_column, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, origin_line, origin_column, caret_line, caret_column, caret_index,
        }
        = self;
        re_export::TextEdit::select_full(surround_object, origin_line, origin_column, caret_line, caret_column, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::has_selection_ex`][super::TextEdit::has_selection_ex]."]
#[must_use]
pub struct ExHasSelection < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExHasSelection < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::has_selection_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selected_text_ex`][super::TextEdit::get_selected_text_ex]."]
#[must_use]
pub struct ExGetSelectedText < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSelectedText < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selected_text_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_at_line_column_ex`][super::TextEdit::get_selection_at_line_column_ex]."]
#[must_use]
pub struct ExGetSelectionAtLineColumn < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, line: i32, column: i32, include_edges: bool, only_selections: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSelectionAtLineColumn < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit, line: i32, column: i32,) -> Self {
        let include_edges = true;
        let only_selections = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, column: column, include_edges: include_edges, only_selections: only_selections,
        }
    }
    #[inline]
    pub fn include_edges(self, include_edges: bool) -> Self {
        Self {
            include_edges: include_edges, .. self
        }
    }
    #[inline]
    pub fn only_selections(self, only_selections: bool) -> Self {
        Self {
            only_selections: only_selections, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line, column, include_edges, only_selections,
        }
        = self;
        re_export::TextEdit::get_selection_at_line_column_full(surround_object, line, column, include_edges, only_selections,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_line_ranges_from_carets_ex`][super::TextEdit::get_line_ranges_from_carets_ex]."]
#[must_use]
pub struct ExGetLineRangesFromCarets < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, only_selections: bool, merge_adjacent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetLineRangesFromCarets < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let only_selections = false;
        let merge_adjacent = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, only_selections: only_selections, merge_adjacent: merge_adjacent,
        }
    }
    #[inline]
    pub fn only_selections(self, only_selections: bool) -> Self {
        Self {
            only_selections: only_selections, .. self
        }
    }
    #[inline]
    pub fn merge_adjacent(self, merge_adjacent: bool) -> Self {
        Self {
            merge_adjacent: merge_adjacent, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Vector2i > {
        let Self {
            _phantom, surround_object, only_selections, merge_adjacent,
        }
        = self;
        re_export::TextEdit::get_line_ranges_from_carets_full(surround_object, only_selections, merge_adjacent,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_origin_line_ex`][super::TextEdit::get_selection_origin_line_ex]."]
#[must_use]
pub struct ExGetSelectionOriginLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSelectionOriginLine < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_origin_line_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_origin_column_ex`][super::TextEdit::get_selection_origin_column_ex]."]
#[must_use]
pub struct ExGetSelectionOriginColumn < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSelectionOriginColumn < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_origin_column_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_selection_origin_line_ex`][super::TextEdit::set_selection_origin_line_ex]."]
#[must_use]
pub struct ExSetSelectionOriginLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, line: i32, can_be_hidden: bool, wrap_index: i32, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetSelectionOriginLine < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit, line: i32,) -> Self {
        let can_be_hidden = true;
        let wrap_index = - 1i32;
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, can_be_hidden: can_be_hidden, wrap_index: wrap_index, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn can_be_hidden(self, can_be_hidden: bool) -> Self {
        Self {
            can_be_hidden: can_be_hidden, .. self
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, line, can_be_hidden, wrap_index, caret_index,
        }
        = self;
        re_export::TextEdit::set_selection_origin_line_full(surround_object, line, can_be_hidden, wrap_index, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_selection_origin_column_ex`][super::TextEdit::set_selection_origin_column_ex]."]
#[must_use]
pub struct ExSetSelectionOriginColumn < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, column: i32, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetSelectionOriginColumn < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit, column: i32,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, caret_index,
        }
        = self;
        re_export::TextEdit::set_selection_origin_column_full(surround_object, column, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_from_line_ex`][super::TextEdit::get_selection_from_line_ex]."]
#[must_use]
pub struct ExGetSelectionFromLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSelectionFromLine < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_from_line_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_from_column_ex`][super::TextEdit::get_selection_from_column_ex]."]
#[must_use]
pub struct ExGetSelectionFromColumn < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSelectionFromColumn < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_from_column_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_to_line_ex`][super::TextEdit::get_selection_to_line_ex]."]
#[must_use]
pub struct ExGetSelectionToLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSelectionToLine < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_to_line_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_to_column_ex`][super::TextEdit::get_selection_to_column_ex]."]
#[must_use]
pub struct ExGetSelectionToColumn < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSelectionToColumn < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_to_column_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::is_caret_after_selection_origin_ex`][super::TextEdit::is_caret_after_selection_origin_ex]."]
#[must_use]
pub struct ExIsCaretAfterSelectionOrigin < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsCaretAfterSelectionOrigin < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::is_caret_after_selection_origin_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::deselect_ex`][super::TextEdit::deselect_ex]."]
#[must_use]
pub struct ExDeselect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDeselect < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::deselect_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::delete_selection_ex`][super::TextEdit::delete_selection_ex]."]
#[must_use]
pub struct ExDeleteSelection < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDeleteSelection < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::delete_selection_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_scroll_pos_for_line_ex`][super::TextEdit::get_scroll_pos_for_line_ex]."]
#[must_use]
pub struct ExGetScrollPosForLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetScrollPosForLine < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit, line: i32,) -> Self {
        let wrap_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, wrap_index: wrap_index,
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f64 {
        let Self {
            _phantom, surround_object, line, wrap_index,
        }
        = self;
        re_export::TextEdit::get_scroll_pos_for_line_full(surround_object, line, wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_line_as_first_visible_ex`][super::TextEdit::set_line_as_first_visible_ex]."]
#[must_use]
pub struct ExSetLineAsFirstVisible < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetLineAsFirstVisible < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit, line: i32,) -> Self {
        let wrap_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, wrap_index: wrap_index,
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, line, wrap_index,
        }
        = self;
        re_export::TextEdit::set_line_as_first_visible_full(surround_object, line, wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_line_as_center_visible_ex`][super::TextEdit::set_line_as_center_visible_ex]."]
#[must_use]
pub struct ExSetLineAsCenterVisible < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetLineAsCenterVisible < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit, line: i32,) -> Self {
        let wrap_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, wrap_index: wrap_index,
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, line, wrap_index,
        }
        = self;
        re_export::TextEdit::set_line_as_center_visible_full(surround_object, line, wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_line_as_last_visible_ex`][super::TextEdit::set_line_as_last_visible_ex]."]
#[must_use]
pub struct ExSetLineAsLastVisible < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetLineAsLastVisible < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit, line: i32,) -> Self {
        let wrap_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, wrap_index: wrap_index,
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, line, wrap_index,
        }
        = self;
        re_export::TextEdit::set_line_as_last_visible_full(surround_object, line, wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::adjust_viewport_to_caret_ex`][super::TextEdit::adjust_viewport_to_caret_ex]."]
#[must_use]
pub struct ExAdjustViewportToCaret < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAdjustViewportToCaret < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::adjust_viewport_to_caret_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::center_viewport_to_caret_ex`][super::TextEdit::center_viewport_to_caret_ex]."]
#[must_use]
pub struct ExCenterViewportToCaret < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCenterViewportToCaret < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::center_viewport_to_caret_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::add_gutter_ex`][super::TextEdit::add_gutter_ex]."]
#[must_use]
pub struct ExAddGutter < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TextEdit, at: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddGutter < 'ex > {
    fn new(surround_object: &'ex mut re_export::TextEdit,) -> Self {
        let at = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, at: at,
        }
    }
    #[inline]
    pub fn at(self, at: i32) -> Self {
        Self {
            at: at, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, at,
        }
        = self;
        re_export::TextEdit::add_gutter_full(surround_object, at,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_line_ex`][super::TextEdit::get_selection_line_ex]."]
#[must_use]
pub struct ExGetSelectionLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSelectionLine < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_line_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_column_ex`][super::TextEdit::get_selection_column_ex]."]
#[must_use]
pub struct ExGetSelectionColumn < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSelectionColumn < 'ex > {
    fn new(surround_object: &'ex re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_column_full(surround_object, caret_index,)
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextEdit.MenuItems")), is_bitfield: false,
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
pub struct EditAction {
    ord: i32
}
impl EditAction {
    #[doc(alias = "ACTION_NONE")]
    #[doc = "Godot enumerator name: `ACTION_NONE`"]
    pub const NONE: EditAction = EditAction {
        ord: 0i32
    };
    #[doc(alias = "ACTION_TYPING")]
    #[doc = "Godot enumerator name: `ACTION_TYPING`"]
    pub const TYPING: EditAction = EditAction {
        ord: 1i32
    };
    #[doc(alias = "ACTION_BACKSPACE")]
    #[doc = "Godot enumerator name: `ACTION_BACKSPACE`"]
    pub const BACKSPACE: EditAction = EditAction {
        ord: 2i32
    };
    #[doc(alias = "ACTION_DELETE")]
    #[doc = "Godot enumerator name: `ACTION_DELETE`"]
    pub const DELETE: EditAction = EditAction {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EditAction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EditAction {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::NONE => "NONE", Self::TYPING => "TYPING", Self::BACKSPACE => "BACKSPACE", Self::DELETE => "DELETE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EditAction::NONE, EditAction::TYPING, EditAction::BACKSPACE, EditAction::DELETE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EditAction >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "ACTION_NONE", EditAction::NONE), crate::meta::inspect::EnumConstant::new("TYPING", "ACTION_TYPING", EditAction::TYPING), crate::meta::inspect::EnumConstant::new("BACKSPACE", "ACTION_BACKSPACE", EditAction::BACKSPACE), crate::meta::inspect::EnumConstant::new("DELETE", "ACTION_DELETE", EditAction::DELETE)]
        }
    }
}
impl crate::meta::GodotConvert for EditAction {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Action None", 0i64), EnumeratorShape::new_int("Action Typing", 1i64), EnumeratorShape::new_int("Action Backspace", 2i64), EnumeratorShape::new_int("Action Delete", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextEdit.EditAction")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EditAction {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EditAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EditAction {
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
impl crate::registry::property::Export for EditAction {
    
}
impl crate::meta::Element for EditAction {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct SearchFlags {
    ord: u64
}
impl SearchFlags {
    #[doc(alias = "SEARCH_MATCH_CASE")]
    #[doc = "Godot enumerator name: `SEARCH_MATCH_CASE`"]
    pub const MATCH_CASE: SearchFlags = SearchFlags {
        ord: 1u64
    };
    #[doc(alias = "SEARCH_WHOLE_WORDS")]
    #[doc = "Godot enumerator name: `SEARCH_WHOLE_WORDS`"]
    pub const WHOLE_WORDS: SearchFlags = SearchFlags {
        ord: 2u64
    };
    #[doc(alias = "SEARCH_BACKWARDS")]
    #[doc = "Godot enumerator name: `SEARCH_BACKWARDS`"]
    pub const BACKWARDS: SearchFlags = SearchFlags {
        ord: 4u64
    };
    
}
impl std::fmt::Debug for SearchFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for SearchFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SearchFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MATCH_CASE", "SEARCH_MATCH_CASE", SearchFlags::MATCH_CASE), crate::meta::inspect::EnumConstant::new("WHOLE_WORDS", "SEARCH_WHOLE_WORDS", SearchFlags::WHOLE_WORDS), crate::meta::inspect::EnumConstant::new("BACKWARDS", "SEARCH_BACKWARDS", SearchFlags::BACKWARDS)]
        }
    }
}
impl std::ops::BitOr for SearchFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for SearchFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for SearchFlags {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Search Match Case", 1i64), EnumeratorShape::new_int("Search Whole Words", 2i64), EnumeratorShape::new_int("Search Backwards", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextEdit.SearchFlags")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for SearchFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SearchFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SearchFlags {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* field)
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
impl crate::registry::property::Export for SearchFlags {
    
}
impl crate::meta::Element for SearchFlags {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CaretType {
    ord: i32
}
impl CaretType {
    #[doc(alias = "CARET_TYPE_LINE")]
    #[doc = "Godot enumerator name: `CARET_TYPE_LINE`"]
    pub const LINE: CaretType = CaretType {
        ord: 0i32
    };
    #[doc(alias = "CARET_TYPE_BLOCK")]
    #[doc = "Godot enumerator name: `CARET_TYPE_BLOCK`"]
    pub const BLOCK: CaretType = CaretType {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for CaretType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CaretType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CaretType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::LINE => "LINE", Self::BLOCK => "BLOCK", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CaretType::LINE, CaretType::BLOCK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CaretType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINE", "CARET_TYPE_LINE", CaretType::LINE), crate::meta::inspect::EnumConstant::new("BLOCK", "CARET_TYPE_BLOCK", CaretType::BLOCK)]
        }
    }
}
impl crate::meta::GodotConvert for CaretType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Caret Type Line", 0i64), EnumeratorShape::new_int("Caret Type Block", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextEdit.CaretType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CaretType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CaretType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CaretType {
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
impl crate::registry::property::Export for CaretType {
    
}
impl crate::meta::Element for CaretType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SelectionMode {
    ord: i32
}
impl SelectionMode {
    #[doc(alias = "SELECTION_MODE_NONE")]
    #[doc = "Godot enumerator name: `SELECTION_MODE_NONE`"]
    pub const NONE: SelectionMode = SelectionMode {
        ord: 0i32
    };
    #[doc(alias = "SELECTION_MODE_SHIFT")]
    #[doc = "Godot enumerator name: `SELECTION_MODE_SHIFT`"]
    pub const SHIFT: SelectionMode = SelectionMode {
        ord: 1i32
    };
    #[doc(alias = "SELECTION_MODE_POINTER")]
    #[doc = "Godot enumerator name: `SELECTION_MODE_POINTER`"]
    pub const POINTER: SelectionMode = SelectionMode {
        ord: 2i32
    };
    #[doc(alias = "SELECTION_MODE_WORD")]
    #[doc = "Godot enumerator name: `SELECTION_MODE_WORD`"]
    pub const WORD: SelectionMode = SelectionMode {
        ord: 3i32
    };
    #[doc(alias = "SELECTION_MODE_LINE")]
    #[doc = "Godot enumerator name: `SELECTION_MODE_LINE`"]
    pub const LINE: SelectionMode = SelectionMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for SelectionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SelectionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SelectionMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::NONE => "NONE", Self::SHIFT => "SHIFT", Self::POINTER => "POINTER", Self::WORD => "WORD", Self::LINE => "LINE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SelectionMode::NONE, SelectionMode::SHIFT, SelectionMode::POINTER, SelectionMode::WORD, SelectionMode::LINE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SelectionMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "SELECTION_MODE_NONE", SelectionMode::NONE), crate::meta::inspect::EnumConstant::new("SHIFT", "SELECTION_MODE_SHIFT", SelectionMode::SHIFT), crate::meta::inspect::EnumConstant::new("POINTER", "SELECTION_MODE_POINTER", SelectionMode::POINTER), crate::meta::inspect::EnumConstant::new("WORD", "SELECTION_MODE_WORD", SelectionMode::WORD), crate::meta::inspect::EnumConstant::new("LINE", "SELECTION_MODE_LINE", SelectionMode::LINE)]
        }
    }
}
impl crate::meta::GodotConvert for SelectionMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Selection Mode None", 0i64), EnumeratorShape::new_int("Selection Mode Shift", 1i64), EnumeratorShape::new_int("Selection Mode Pointer", 2i64), EnumeratorShape::new_int("Selection Mode Word", 3i64), EnumeratorShape::new_int("Selection Mode Line", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextEdit.SelectionMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SelectionMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SelectionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SelectionMode {
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
impl crate::registry::property::Export for SelectionMode {
    
}
impl crate::meta::Element for SelectionMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LineWrappingMode {
    ord: i32
}
impl LineWrappingMode {
    #[doc(alias = "LINE_WRAPPING_NONE")]
    #[doc = "Godot enumerator name: `LINE_WRAPPING_NONE`"]
    pub const NONE: LineWrappingMode = LineWrappingMode {
        ord: 0i32
    };
    #[doc(alias = "LINE_WRAPPING_BOUNDARY")]
    #[doc = "Godot enumerator name: `LINE_WRAPPING_BOUNDARY`"]
    pub const BOUNDARY: LineWrappingMode = LineWrappingMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for LineWrappingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LineWrappingMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LineWrappingMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::NONE => "NONE", Self::BOUNDARY => "BOUNDARY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LineWrappingMode::NONE, LineWrappingMode::BOUNDARY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LineWrappingMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "LINE_WRAPPING_NONE", LineWrappingMode::NONE), crate::meta::inspect::EnumConstant::new("BOUNDARY", "LINE_WRAPPING_BOUNDARY", LineWrappingMode::BOUNDARY)]
        }
    }
}
impl crate::meta::GodotConvert for LineWrappingMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Line Wrapping None", 0i64), EnumeratorShape::new_int("Line Wrapping Boundary", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextEdit.LineWrappingMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LineWrappingMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LineWrappingMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LineWrappingMode {
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
impl crate::registry::property::Export for LineWrappingMode {
    
}
impl crate::meta::Element for LineWrappingMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GutterType {
    ord: i32
}
impl GutterType {
    #[doc(alias = "GUTTER_TYPE_STRING")]
    #[doc = "Godot enumerator name: `GUTTER_TYPE_STRING`"]
    pub const STRING: GutterType = GutterType {
        ord: 0i32
    };
    #[doc(alias = "GUTTER_TYPE_ICON")]
    #[doc = "Godot enumerator name: `GUTTER_TYPE_ICON`"]
    pub const ICON: GutterType = GutterType {
        ord: 1i32
    };
    #[doc(alias = "GUTTER_TYPE_CUSTOM")]
    #[doc = "Godot enumerator name: `GUTTER_TYPE_CUSTOM`"]
    pub const CUSTOM: GutterType = GutterType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for GutterType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GutterType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GutterType {
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
            Self::STRING => "STRING", Self::ICON => "ICON", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GutterType::STRING, GutterType::ICON, GutterType::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GutterType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STRING", "GUTTER_TYPE_STRING", GutterType::STRING), crate::meta::inspect::EnumConstant::new("ICON", "GUTTER_TYPE_ICON", GutterType::ICON), crate::meta::inspect::EnumConstant::new("CUSTOM", "GUTTER_TYPE_CUSTOM", GutterType::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for GutterType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Gutter Type String", 0i64), EnumeratorShape::new_int("Gutter Type Icon", 1i64), EnumeratorShape::new_int("Gutter Type Custom", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextEdit.GutterType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for GutterType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GutterType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for GutterType {
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
impl crate::registry::property::Export for GutterType {
    
}
impl crate::meta::Element for GutterType {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TextEdit;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`TextEdit`][crate::classes::TextEdit] class."]
    pub struct SignalsOfTextEdit < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTextEdit < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn text_set(&mut self) -> SigTextSet < 'c, C > {
            SigTextSet {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "text_set")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn text_changed(&mut self) -> SigTextChanged < 'c, C > {
            SigTextChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "text_changed")
            }
        }
        #[doc = "Signature: `(from_line: i64, to_line: i64)`"]
        pub fn lines_edited_from(&mut self) -> SigLinesEditedFrom < 'c, C > {
            SigLinesEditedFrom {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "lines_edited_from")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn caret_changed(&mut self) -> SigCaretChanged < 'c, C > {
            SigCaretChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "caret_changed")
            }
        }
        #[doc = "Signature: `(line: i64, gutter: i64)`"]
        pub fn gutter_clicked(&mut self) -> SigGutterClicked < 'c, C > {
            SigGutterClicked {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "gutter_clicked")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn gutter_added(&mut self) -> SigGutterAdded < 'c, C > {
            SigGutterAdded {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "gutter_added")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn gutter_removed(&mut self) -> SigGutterRemoved < 'c, C > {
            SigGutterRemoved {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "gutter_removed")
            }
        }
    }
    type TypedSigTextSet < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTextSet < 'c, C: WithSignals > {
        typed: TypedSigTextSet < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTextSet < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTextSet < 'c, C > {
        type Target = TypedSigTextSet < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTextSet < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTextChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTextChanged < 'c, C: WithSignals > {
        typed: TypedSigTextChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTextChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
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
    type TypedSigLinesEditedFrom < 'c, C > = TypedSignal < 'c, C, (i64, i64,) >;
    pub struct SigLinesEditedFrom < 'c, C: WithSignals > {
        typed: TypedSigLinesEditedFrom < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigLinesEditedFrom < 'c, C > {
        pub fn emit(&mut self, from_line: i64, to_line: i64,) {
            self.typed.emit_tuple((from_line, to_line,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigLinesEditedFrom < 'c, C > {
        type Target = TypedSigLinesEditedFrom < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigLinesEditedFrom < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCaretChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigCaretChanged < 'c, C: WithSignals > {
        typed: TypedSigCaretChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCaretChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCaretChanged < 'c, C > {
        type Target = TypedSigCaretChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCaretChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigGutterClicked < 'c, C > = TypedSignal < 'c, C, (i64, i64,) >;
    pub struct SigGutterClicked < 'c, C: WithSignals > {
        typed: TypedSigGutterClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigGutterClicked < 'c, C > {
        pub fn emit(&mut self, line: i64, gutter: i64,) {
            self.typed.emit_tuple((line, gutter,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigGutterClicked < 'c, C > {
        type Target = TypedSigGutterClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigGutterClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigGutterAdded < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigGutterAdded < 'c, C: WithSignals > {
        typed: TypedSigGutterAdded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigGutterAdded < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigGutterAdded < 'c, C > {
        type Target = TypedSigGutterAdded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigGutterAdded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigGutterRemoved < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigGutterRemoved < 'c, C: WithSignals > {
        typed: TypedSigGutterRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigGutterRemoved < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigGutterRemoved < 'c, C > {
        type Target = TypedSigGutterRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigGutterRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for TextEdit {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTextEdit < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTextEdit < 'c, C > {
        type Target = < < TextEdit as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = TextEdit;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTextEdit < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = TextEdit;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
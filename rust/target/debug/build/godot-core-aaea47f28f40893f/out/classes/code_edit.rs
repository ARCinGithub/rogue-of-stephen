#![doc = "Sidecar module for class [`CodeEdit`][crate::classes::CodeEdit].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CodeEdit` enums](https://docs.godotengine.org/en/stable/classes/class_codeedit.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CodeEdit`.\n\nInherits [`TextEdit`][crate::classes::TextEdit].\n\nRelated symbols:\n\n* [`code_edit`][crate::classes::code_edit]: sidecar module with related enum/flag types\n* [`ICodeEdit`][crate::classes::ICodeEdit]: virtual methods\n* [`SignalsOfCodeEdit`][crate::classes::code_edit::SignalsOfCodeEdit]: signal collection\n\n\nSee also [Godot docs for `CodeEdit`](https://docs.godotengine.org/en/stable/classes/class_codeedit.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CodeEdit::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nCodeEdit is a specialized [`TextEdit`][crate::classes::TextEdit] designed for editing plain text code files. It has many features commonly found in code editors such as line numbers, line folding, code completion, indent management, and string/comment management.\n\n**Note:** Regardless of locale, `CodeEdit` will by default always use left-to-right text direction to correctly display source code."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CodeEdit {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CodeEdit`][crate::classes::CodeEdit].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ITextEdit`][crate::classes::ITextEdit] > [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `CodeEdit` methods](https://docs.godotengine.org/en/stable/classes/class_codeedit.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICodeEdit: crate::obj::GodotClass < Base = CodeEdit > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Override this method to define how the selected entry should be inserted. If `replace` is `true`, any existing text should be replaced."]
        fn confirm_code_completion(&mut self, replace: bool,) {
            unimplemented !()
        }
        #[doc = "Override this method to define what happens when the user requests code completion. If `force` is `true`, any checks should be bypassed."]
        fn request_code_completion(&mut self, force: bool,) {
            unimplemented !()
        }
        #[doc = "Override this method to define what items in `candidates` should be displayed.\n\nBoth `candidates` and the return is an [`Array`][crate::builtin::Array] of [`Dictionary`][crate::builtin::Dictionary], see [`get_code_completion_option`][`crate::classes::CodeEdit::get_code_completion_option`] for [`Dictionary`][crate::builtin::Dictionary] content."]
        fn filter_code_completion_candidates(&self, candidates: Array < VarDictionary >,) -> Array < AnyDictionary > {
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
    impl CodeEdit {
        pub fn set_indent_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_indent_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_indent_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_indent_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_indent_using_spaces(&mut self, use_spaces: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_spaces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_indent_using_spaces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_indent_using_spaces(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_indent_using_spaces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_indent_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_auto_indent_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_auto_indent_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_auto_indent_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_indent_prefixes(&mut self, prefixes: &Array < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < GString > >,);
            let args = (RefArg::new(prefixes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_auto_indent_prefixes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_indent_prefixes(&self,) -> Array < GString > {
            type CallRet = Array < GString >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_auto_indent_prefixes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If there is no selection, indentation is inserted at the caret. Otherwise, the selected lines are indented like [`indent_lines`][`crate::classes::CodeEdit::indent_lines`]. Equivalent to the \\[member ProjectSettings.input/ui_text_indent] action. The indentation characters used depend on \\[member indent_use_spaces] and \\[member indent_size]."]
        pub fn do_indent(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "do_indent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Indents all lines that are selected or have a caret on them. Uses spaces or a tab depending on \\[member indent_use_spaces]. See [`unindent_lines`][`crate::classes::CodeEdit::unindent_lines`]."]
        pub fn indent_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "indent_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unindents all lines that are selected or have a caret on them. Uses spaces or a tab depending on \\[member indent_use_spaces]. Equivalent to the \\[member ProjectSettings.input/ui_text_dedent] action. See [`indent_lines`][`crate::classes::CodeEdit::indent_lines`]."]
        pub fn unindent_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "unindent_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the indents of lines between `from_line` and `to_line` to tabs or spaces as set by \\[member indent_use_spaces].\n\nValues of `-1` convert the entire text."]
        pub(crate) fn convert_indent_full(&mut self, from_line: i32, to_line: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "convert_indent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`convert_indent_ex`][Self::convert_indent_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Converts the indents of lines between `from_line` and `to_line` to tabs or spaces as set by \\[member indent_use_spaces].\n\nValues of `-1` convert the entire text."]
        #[inline]
        pub fn convert_indent(&mut self,) {
            self.convert_indent_ex() . done()
        }
        #[doc = "Converts the indents of lines between `from_line` and `to_line` to tabs or spaces as set by \\[member indent_use_spaces].\n\nValues of `-1` convert the entire text."]
        #[inline]
        pub fn convert_indent_ex < 'ex > (&'ex mut self,) -> ExConvertIndent < 'ex > {
            ExConvertIndent::new(self,)
        }
        pub fn set_auto_brace_completion_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_auto_brace_completion_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_auto_brace_completion_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_auto_brace_completion_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_highlight_matching_braces_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_highlight_matching_braces_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_highlight_matching_braces_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_highlight_matching_braces_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a brace pair.\n\nBoth the start and end keys must be symbols. Only the start key has to be unique."]
        pub fn add_auto_brace_completion_pair(&mut self, start_key: impl AsArg < GString >, end_key: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (start_key.into_arg(), end_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "add_auto_brace_completion_pair", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_brace_completion_pairs(&mut self, pairs: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(pairs),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_auto_brace_completion_pairs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_brace_completion_pairs(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_auto_brace_completion_pairs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if open key `open_key` exists."]
        pub fn has_auto_brace_completion_open_key(&self, open_key: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (open_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "has_auto_brace_completion_open_key", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if close key `close_key` exists."]
        pub fn has_auto_brace_completion_close_key(&self, close_key: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (close_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "has_auto_brace_completion_close_key", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the matching auto brace close key for `open_key`."]
        pub fn get_auto_brace_completion_close_key(&self, open_key: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (open_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_auto_brace_completion_close_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_breakpoints_gutter(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_breakpoints_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_breakpoints_gutter(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_drawing_breakpoints_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_bookmarks_gutter(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_bookmarks_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_bookmarks_gutter(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_drawing_bookmarks_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_executing_lines_gutter(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_executing_lines_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_executing_lines_gutter(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_drawing_executing_lines_gutter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given line as a breakpoint. If `true` and \\[member gutters_draw_breakpoints_gutter] is `true`, draws the [theme_item breakpoint] icon in the gutter for this line. See [`get_breakpointed_lines`][`crate::classes::CodeEdit::get_breakpointed_lines`] and [`is_line_breakpointed`][`crate::classes::CodeEdit::is_line_breakpointed`]."]
        pub fn set_line_as_breakpoint(&mut self, line: i32, breakpointed: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (line, breakpointed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_as_breakpoint", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given line is breakpointed. See [`set_line_as_breakpoint`][`crate::classes::CodeEdit::set_line_as_breakpoint`]."]
        pub fn is_line_breakpointed(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_breakpointed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all breakpointed lines."]
        pub fn clear_breakpointed_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_breakpointed_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets all breakpointed lines."]
        pub fn get_breakpointed_lines(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_breakpointed_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given line as bookmarked. If `true` and \\[member gutters_draw_bookmarks] is `true`, draws the [theme_item bookmark] icon in the gutter for this line. See [`get_bookmarked_lines`][`crate::classes::CodeEdit::get_bookmarked_lines`] and [`is_line_bookmarked`][`crate::classes::CodeEdit::is_line_bookmarked`]."]
        pub fn set_line_as_bookmarked(&mut self, line: i32, bookmarked: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (line, bookmarked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_as_bookmarked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given line is bookmarked. See [`set_line_as_bookmarked`][`crate::classes::CodeEdit::set_line_as_bookmarked`]."]
        pub fn is_line_bookmarked(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_bookmarked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all bookmarked lines."]
        pub fn clear_bookmarked_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_bookmarked_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets all bookmarked lines."]
        pub fn get_bookmarked_lines(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_bookmarked_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given line as executing. If `true` and \\[member gutters_draw_executing_lines] is `true`, draws the [theme_item executing_line] icon in the gutter for this line. See [`get_executing_lines`][`crate::classes::CodeEdit::get_executing_lines`] and [`is_line_executing`][`crate::classes::CodeEdit::is_line_executing`]."]
        pub fn set_line_as_executing(&mut self, line: i32, executing: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (line, executing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_as_executing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given line is marked as executing. See [`set_line_as_executing`][`crate::classes::CodeEdit::set_line_as_executing`]."]
        pub fn is_line_executing(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_executing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all executed lines."]
        pub fn clear_executing_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_executing_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets all executing lines."]
        pub fn get_executing_lines(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_executing_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_line_numbers(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_line_numbers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_draw_line_numbers_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_draw_line_numbers_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_numbers_zero_padded(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_numbers_zero_padded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_line_numbers_zero_padded(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_numbers_zero_padded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_numbers_min_digits(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_numbers_min_digits", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_numbers_min_digits(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_line_numbers_min_digits", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_fold_gutter(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_fold_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_fold_gutter(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_drawing_fold_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_folding_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_folding_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_line_folding_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_folding_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given line is foldable. A line is foldable if it is the start of a valid code region (see [`get_code_region_start_tag`][`crate::classes::CodeEdit::get_code_region_start_tag`]), if it is the start of a comment or string block, or if the next non-empty line is more indented (see [`get_indent_level`][`crate::classes::TextEdit::get_indent_level`])."]
        pub fn can_fold_line(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "can_fold_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Folds the given line, if possible (see [`can_fold_line`][`crate::classes::CodeEdit::can_fold_line`])."]
        pub fn fold_line(&mut self, line: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "fold_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unfolds the given line if it is folded or if it is hidden under a folded line."]
        pub fn unfold_line(&mut self, line: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "unfold_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Folds all lines that are possible to be folded (see [`can_fold_line`][`crate::classes::CodeEdit::can_fold_line`])."]
        pub fn fold_all_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "fold_all_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unfolds all lines that are folded."]
        pub fn unfold_all_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "unfold_all_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Toggle the folding of the code block at the given line."]
        pub fn toggle_foldable_line(&mut self, line: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "toggle_foldable_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Toggle the folding of the code block on all lines with a caret on them."]
        pub fn toggle_foldable_lines_at_carets(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "toggle_foldable_lines_at_carets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given line is folded. See [`fold_line`][`crate::classes::CodeEdit::fold_line`]."]
        pub fn is_line_folded(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_folded", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns all lines that are currently folded."]
        pub fn get_folded_lines(&self,) -> Array < i64 > {
            type CallRet = Array < i64 >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_folded_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new code region with the selection. At least one single line comment delimiter have to be defined (see [`add_comment_delimiter`][`crate::classes::CodeEdit::add_comment_delimiter`]).\n\nA code region is a part of code that is highlighted when folded and can help organize your script.\n\nCode region start and end tags can be customized (see [`set_code_region_tags`][`crate::classes::CodeEdit::set_code_region_tags`]).\n\nCode regions are delimited using start and end tags (respectively `region` and `endregion` by default) preceded by one line comment delimiter. (eg. `#region` and `#endregion`)"]
        pub fn create_code_region(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "create_code_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the code region start tag (without comment delimiter)."]
        pub fn get_code_region_start_tag(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_region_start_tag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the code region end tag (without comment delimiter)."]
        pub fn get_code_region_end_tag(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_region_end_tag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the code region start and end tags (without comment delimiter)."]
        pub(crate) fn set_code_region_tags_full(&mut self, start: CowArg < GString >, end: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_region_tags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_code_region_tags_ex`][Self::set_code_region_tags_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the code region start and end tags (without comment delimiter)."]
        #[inline]
        pub fn set_code_region_tags(&mut self,) {
            self.set_code_region_tags_ex() . done()
        }
        #[doc = "Sets the code region start and end tags (without comment delimiter)."]
        #[inline]
        pub fn set_code_region_tags_ex < 'ex > (&'ex mut self,) -> ExSetCodeRegionTags < 'ex > {
            ExSetCodeRegionTags::new(self,)
        }
        #[doc = "Returns `true` if the given line is a code region start. See [`set_code_region_tags`][`crate::classes::CodeEdit::set_code_region_tags`]."]
        pub fn is_line_code_region_start(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_code_region_start", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given line is a code region end. See [`set_code_region_tags`][`crate::classes::CodeEdit::set_code_region_tags`]."]
        pub fn is_line_code_region_end(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_code_region_end", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Defines a string delimiter from `start_key` to `end_key`. Both keys should be symbols, and `start_key` must not be shared with other delimiters.\n\nIf `line_only` is `true` or `end_key` is an empty [`String`][crate::builtin::GString], the region does not carry over to the next line."]
        pub(crate) fn add_string_delimiter_full(&mut self, start_key: CowArg < GString >, end_key: CowArg < GString >, line_only: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool,);
            let args = (start_key, end_key, line_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "add_string_delimiter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_string_delimiter_ex`][Self::add_string_delimiter_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Defines a string delimiter from `start_key` to `end_key`. Both keys should be symbols, and `start_key` must not be shared with other delimiters.\n\nIf `line_only` is `true` or `end_key` is an empty [`String`][crate::builtin::GString], the region does not carry over to the next line."]
        #[inline]
        pub fn add_string_delimiter(&mut self, start_key: impl AsArg < GString >, end_key: impl AsArg < GString >,) {
            self.add_string_delimiter_ex(start_key, end_key,) . done()
        }
        #[doc = "Defines a string delimiter from `start_key` to `end_key`. Both keys should be symbols, and `start_key` must not be shared with other delimiters.\n\nIf `line_only` is `true` or `end_key` is an empty [`String`][crate::builtin::GString], the region does not carry over to the next line."]
        #[inline]
        pub fn add_string_delimiter_ex < 'ex > (&'ex mut self, start_key: impl AsArg < GString > + 'ex, end_key: impl AsArg < GString > + 'ex,) -> ExAddStringDelimiter < 'ex > {
            ExAddStringDelimiter::new(self, start_key, end_key,)
        }
        #[doc = "Removes the string delimiter with `start_key`."]
        pub fn remove_string_delimiter(&mut self, start_key: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "remove_string_delimiter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if string `start_key` exists."]
        pub fn has_string_delimiter(&self, start_key: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "has_string_delimiter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_string_delimiters(&mut self, string_delimiters: &Array < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < GString > >,);
            let args = (RefArg::new(string_delimiters),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_string_delimiters", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all string delimiters."]
        pub fn clear_string_delimiters(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_string_delimiters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_string_delimiters(&self,) -> Array < GString > {
            type CallRet = Array < GString >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_string_delimiters", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the delimiter index if `line` `column` is in a string. If `column` is not provided, will return the delimiter index if the entire `line` is a string. Otherwise `-1`."]
        pub(crate) fn is_in_string_full(&self, line: i32, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_in_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_in_string_ex`][Self::is_in_string_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the delimiter index if `line` `column` is in a string. If `column` is not provided, will return the delimiter index if the entire `line` is a string. Otherwise `-1`."]
        #[inline]
        pub fn is_in_string(&self, line: i32,) -> i32 {
            self.is_in_string_ex(line,) . done()
        }
        #[doc = "Returns the delimiter index if `line` `column` is in a string. If `column` is not provided, will return the delimiter index if the entire `line` is a string. Otherwise `-1`."]
        #[inline]
        pub fn is_in_string_ex < 'ex > (&'ex self, line: i32,) -> ExIsInString < 'ex > {
            ExIsInString::new(self, line,)
        }
        #[doc = "Adds a comment delimiter from `start_key` to `end_key`. Both keys should be symbols, and `start_key` must not be shared with other delimiters.\n\nIf `line_only` is `true` or `end_key` is an empty [`String`][crate::builtin::GString], the region does not carry over to the next line."]
        pub(crate) fn add_comment_delimiter_full(&mut self, start_key: CowArg < GString >, end_key: CowArg < GString >, line_only: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool,);
            let args = (start_key, end_key, line_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "add_comment_delimiter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_comment_delimiter_ex`][Self::add_comment_delimiter_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a comment delimiter from `start_key` to `end_key`. Both keys should be symbols, and `start_key` must not be shared with other delimiters.\n\nIf `line_only` is `true` or `end_key` is an empty [`String`][crate::builtin::GString], the region does not carry over to the next line."]
        #[inline]
        pub fn add_comment_delimiter(&mut self, start_key: impl AsArg < GString >, end_key: impl AsArg < GString >,) {
            self.add_comment_delimiter_ex(start_key, end_key,) . done()
        }
        #[doc = "Adds a comment delimiter from `start_key` to `end_key`. Both keys should be symbols, and `start_key` must not be shared with other delimiters.\n\nIf `line_only` is `true` or `end_key` is an empty [`String`][crate::builtin::GString], the region does not carry over to the next line."]
        #[inline]
        pub fn add_comment_delimiter_ex < 'ex > (&'ex mut self, start_key: impl AsArg < GString > + 'ex, end_key: impl AsArg < GString > + 'ex,) -> ExAddCommentDelimiter < 'ex > {
            ExAddCommentDelimiter::new(self, start_key, end_key,)
        }
        #[doc = "Removes the comment delimiter with `start_key`."]
        pub fn remove_comment_delimiter(&mut self, start_key: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "remove_comment_delimiter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if comment `start_key` exists."]
        pub fn has_comment_delimiter(&self, start_key: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "has_comment_delimiter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_comment_delimiters(&mut self, comment_delimiters: &Array < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < GString > >,);
            let args = (RefArg::new(comment_delimiters),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_comment_delimiters", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all comment delimiters."]
        pub fn clear_comment_delimiters(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_comment_delimiters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_comment_delimiters(&self,) -> Array < GString > {
            type CallRet = Array < GString >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_comment_delimiters", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns delimiter index if `line` `column` is in a comment. If `column` is not provided, will return delimiter index if the entire `line` is a comment. Otherwise `-1`."]
        pub(crate) fn is_in_comment_full(&self, line: i32, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_in_comment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_in_comment_ex`][Self::is_in_comment_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns delimiter index if `line` `column` is in a comment. If `column` is not provided, will return delimiter index if the entire `line` is a comment. Otherwise `-1`."]
        #[inline]
        pub fn is_in_comment(&self, line: i32,) -> i32 {
            self.is_in_comment_ex(line,) . done()
        }
        #[doc = "Returns delimiter index if `line` `column` is in a comment. If `column` is not provided, will return delimiter index if the entire `line` is a comment. Otherwise `-1`."]
        #[inline]
        pub fn is_in_comment_ex < 'ex > (&'ex self, line: i32,) -> ExIsInComment < 'ex > {
            ExIsInComment::new(self, line,)
        }
        #[doc = "Gets the start key for a string or comment region index."]
        pub fn get_delimiter_start_key(&self, delimiter_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (delimiter_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_delimiter_start_key", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the end key for a string or comment region index."]
        pub fn get_delimiter_end_key(&self, delimiter_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (delimiter_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_delimiter_end_key", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `line` `column` is in a string or comment, returns the start position of the region. If not or no start could be found, both [`Vector2`][crate::builtin::Vector2] values will be `-1`."]
        pub fn get_delimiter_start_position(&self, line: i32, column: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_delimiter_start_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `line` `column` is in a string or comment, returns the end position of the region. If not or no end could be found, both [`Vector2`][crate::builtin::Vector2] values will be `-1`."]
        pub fn get_delimiter_end_position(&self, line: i32, column: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_delimiter_end_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the code hint text. Pass an empty string to clear."]
        pub fn set_code_hint(&mut self, code_hint: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (code_hint.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_hint", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the code hint will draw below the main caret. If `false`, the code hint will draw above the main caret. See [`set_code_hint`][`crate::classes::CodeEdit::set_code_hint`]."]
        pub fn set_code_hint_draw_below(&mut self, draw_below: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (draw_below,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_hint_draw_below", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the full text with char `0xFFFF` at the caret location."]
        pub fn get_text_for_code_completion(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_text_for_code_completion", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Emits `code_completion_requested`, if `force` is `true` will bypass all checks. Otherwise will check that the caret is in a word or in front of a prefix. Will ignore the request if all current options are of type file path, node path, or signal."]
        pub(crate) fn request_code_completion_full(&mut self, force: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "request_code_completion", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`request_code_completion_ex`][Self::request_code_completion_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Emits `code_completion_requested`, if `force` is `true` will bypass all checks. Otherwise will check that the caret is in a word or in front of a prefix. Will ignore the request if all current options are of type file path, node path, or signal."]
        #[inline]
        pub fn request_code_completion(&mut self,) {
            self.request_code_completion_ex() . done()
        }
        #[doc = "Emits `code_completion_requested`, if `force` is `true` will bypass all checks. Otherwise will check that the caret is in a word or in front of a prefix. Will ignore the request if all current options are of type file path, node path, or signal."]
        #[inline]
        pub fn request_code_completion_ex < 'ex > (&'ex mut self,) -> ExRequestCodeCompletion < 'ex > {
            ExRequestCodeCompletion::new(self,)
        }
        #[doc = "Submits an item to the queue of potential candidates for the autocomplete menu. Call [`update_code_completion_options`][`crate::classes::CodeEdit::update_code_completion_options`] to update the list.\n\n`location` indicates location of the option relative to the location of the code completion query. See \\[enum CodeEdit.CodeCompletionLocation] for how to set this value.\n\n**Note:** This list will replace all current candidates."]
        pub(crate) fn add_code_completion_option_full(&mut self, type_: crate::classes::code_edit::CodeCompletionKind, display_text: CowArg < GString >, insert_text: CowArg < GString >, text_color: Color, icon: CowArg < Option < Gd < crate::classes::Resource > > >, value: RefArg < Variant >, location: crate::classes::code_edit::CodeCompletionLocation,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (crate::classes::code_edit::CodeCompletionKind, CowArg < 'a0, GString >, CowArg < 'a1, GString >, Color, CowArg < 'a2, Option < Gd < crate::classes::Resource > > >, RefArg < 'a3, Variant >, crate::classes::code_edit::CodeCompletionLocation,);
            let args = (type_, display_text, insert_text, text_color, icon, value, location,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "add_code_completion_option", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_code_completion_option_ex`][Self::add_code_completion_option_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Submits an item to the queue of potential candidates for the autocomplete menu. Call [`update_code_completion_options`][`crate::classes::CodeEdit::update_code_completion_options`] to update the list.\n\n`location` indicates location of the option relative to the location of the code completion query. See \\[enum CodeEdit.CodeCompletionLocation] for how to set this value.\n\n**Note:** This list will replace all current candidates."]
        #[inline]
        pub fn add_code_completion_option(&mut self, type_: crate::classes::code_edit::CodeCompletionKind, display_text: impl AsArg < GString >, insert_text: impl AsArg < GString >,) {
            self.add_code_completion_option_ex(type_, display_text, insert_text,) . done()
        }
        #[doc = "Submits an item to the queue of potential candidates for the autocomplete menu. Call [`update_code_completion_options`][`crate::classes::CodeEdit::update_code_completion_options`] to update the list.\n\n`location` indicates location of the option relative to the location of the code completion query. See \\[enum CodeEdit.CodeCompletionLocation] for how to set this value.\n\n**Note:** This list will replace all current candidates."]
        #[inline]
        pub fn add_code_completion_option_ex < 'ex > (&'ex mut self, type_: crate::classes::code_edit::CodeCompletionKind, display_text: impl AsArg < GString > + 'ex, insert_text: impl AsArg < GString > + 'ex,) -> ExAddCodeCompletionOption < 'ex > {
            ExAddCodeCompletionOption::new(self, type_, display_text, insert_text,)
        }
        #[doc = "Submits all completion options added with [`add_code_completion_option`][`crate::classes::CodeEdit::add_code_completion_option`]. Will try to force the autocomplete menu to popup, if `force` is `true`.\n\n**Note:** This will replace all current candidates."]
        pub fn update_code_completion_options(&mut self, force: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "update_code_completion_options", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets all completion options, see [`get_code_completion_option`][`crate::classes::CodeEdit::get_code_completion_option`] for return content."]
        pub fn get_code_completion_options(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_completion_options", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the completion option at `index`. The return [`Dictionary`][crate::builtin::Dictionary] has the following key-values:\n\n`kind`: \\[enum CodeCompletionKind]\n\n`display_text`: Text that is shown on the autocomplete menu.\n\n`insert_text`: Text that is to be inserted when this item is selected.\n\n`font_color`: Color of the text on the autocomplete menu.\n\n`icon`: Icon to draw on the autocomplete menu.\n\n`default_value`: Value of the symbol."]
        pub fn get_code_completion_option(&self, index: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_completion_option", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the index of the current selected completion option."]
        pub fn get_code_completion_selected_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_completion_selected_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the current selected completion option."]
        pub fn set_code_completion_selected_index(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_completion_selected_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Inserts the selected entry into the text. If `replace` is `true`, any existing text is replaced rather than merged."]
        pub(crate) fn confirm_code_completion_full(&mut self, replace: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (replace,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "confirm_code_completion", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`confirm_code_completion_ex`][Self::confirm_code_completion_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Inserts the selected entry into the text. If `replace` is `true`, any existing text is replaced rather than merged."]
        #[inline]
        pub fn confirm_code_completion(&mut self,) {
            self.confirm_code_completion_ex() . done()
        }
        #[doc = "Inserts the selected entry into the text. If `replace` is `true`, any existing text is replaced rather than merged."]
        #[inline]
        pub fn confirm_code_completion_ex < 'ex > (&'ex mut self,) -> ExConfirmCodeCompletion < 'ex > {
            ExConfirmCodeCompletion::new(self,)
        }
        #[doc = "Cancels the autocomplete menu."]
        pub fn cancel_code_completion(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "cancel_code_completion", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_code_completion_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_completion_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_code_completion_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10369usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_code_completion_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_code_completion_prefixes(&mut self, prefixes: &Array < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < GString > >,);
            let args = (RefArg::new(prefixes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10370usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_completion_prefixes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_code_completion_prefixes(&self,) -> Array < GString > {
            type CallRet = Array < GString >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10371usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_completion_prefixes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_length_guidelines(&mut self, guideline_columns: &Array < i64 >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < i64 > >,);
            let args = (RefArg::new(guideline_columns),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10372usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_length_guidelines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_length_guidelines(&self,) -> Array < i64 > {
            type CallRet = Array < i64 >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10373usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_line_length_guidelines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_symbol_lookup_on_click_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10374usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_symbol_lookup_on_click_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_symbol_lookup_on_click_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10375usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_symbol_lookup_on_click_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the full text with char `0xFFFF` at the cursor location."]
        pub fn get_text_for_symbol_lookup(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_text_for_symbol_lookup", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the full text with char `0xFFFF` at the specified location."]
        pub fn get_text_with_cursor_char(&self, line: i32, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_text_with_cursor_char", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the symbol emitted by `symbol_validate` as a valid lookup."]
        pub fn set_symbol_lookup_word_as_valid(&mut self, valid: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (valid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_symbol_lookup_word_as_valid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_symbol_tooltip_on_hover_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_symbol_tooltip_on_hover_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_symbol_tooltip_on_hover_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_symbol_tooltip_on_hover_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves all lines up that are selected or have a caret on them."]
        pub fn move_lines_up(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "move_lines_up", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves all lines down that are selected or have a caret on them."]
        pub fn move_lines_down(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "move_lines_down", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deletes all lines that are selected or have a caret on them."]
        pub fn delete_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "delete_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Duplicates all selected text and duplicates all lines with a caret on them."]
        pub fn duplicate_selection(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "duplicate_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Duplicates all lines currently selected with any caret. Duplicates the entire line beneath the current one no matter where the caret is within the line."]
        pub fn duplicate_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "duplicate_lines", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CodeEdit {
        type Base = crate::classes::TextEdit;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CodeEdit"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CodeEdit {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::TextEdit > for CodeEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for CodeEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for CodeEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CodeEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CodeEdit {
        
    }
    impl crate::obj::cap::GodotDefault for CodeEdit {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CodeEdit {
        type Target = crate::classes::TextEdit;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CodeEdit {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CodeEdit`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CodeEdit__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CodeEdit > for $Class {
                
            }
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
#[doc = "Default-param extender for [`CodeEdit::convert_indent_ex`][super::CodeEdit::convert_indent_ex]."]
#[must_use]
pub struct ExConvertIndent < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CodeEdit, from_line: i32, to_line: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExConvertIndent < 'ex > {
    fn new(surround_object: &'ex mut re_export::CodeEdit,) -> Self {
        let from_line = - 1i32;
        let to_line = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_line: from_line, to_line: to_line,
        }
    }
    #[inline]
    pub fn from_line(self, from_line: i32) -> Self {
        Self {
            from_line: from_line, .. self
        }
    }
    #[inline]
    pub fn to_line(self, to_line: i32) -> Self {
        Self {
            to_line: to_line, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_line, to_line,
        }
        = self;
        re_export::CodeEdit::convert_indent_full(surround_object, from_line, to_line,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::set_code_region_tags_ex`][super::CodeEdit::set_code_region_tags_ex]."]
#[must_use]
pub struct ExSetCodeRegionTags < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CodeEdit, start: CowArg < 'ex, GString >, end: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetCodeRegionTags < 'ex > {
    fn new(surround_object: &'ex mut re_export::CodeEdit,) -> Self {
        let start = GString::from("region");
        let end = GString::from("endregion");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start: CowArg::Owned(start), end: CowArg::Owned(end),
        }
    }
    #[inline]
    pub fn start(self, start: impl AsArg < GString > + 'ex) -> Self {
        Self {
            start: start.into_arg(), .. self
        }
    }
    #[inline]
    pub fn end(self, end: impl AsArg < GString > + 'ex) -> Self {
        Self {
            end: end.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start, end,
        }
        = self;
        re_export::CodeEdit::set_code_region_tags_full(surround_object, start, end,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::add_string_delimiter_ex`][super::CodeEdit::add_string_delimiter_ex]."]
#[must_use]
pub struct ExAddStringDelimiter < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CodeEdit, start_key: CowArg < 'ex, GString >, end_key: CowArg < 'ex, GString >, line_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddStringDelimiter < 'ex > {
    fn new(surround_object: &'ex mut re_export::CodeEdit, start_key: impl AsArg < GString > + 'ex, end_key: impl AsArg < GString > + 'ex,) -> Self {
        let line_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start_key: start_key.into_arg(), end_key: end_key.into_arg(), line_only: line_only,
        }
    }
    #[inline]
    pub fn line_only(self, line_only: bool) -> Self {
        Self {
            line_only: line_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start_key, end_key, line_only,
        }
        = self;
        re_export::CodeEdit::add_string_delimiter_full(surround_object, start_key, end_key, line_only,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::is_in_string_ex`][super::CodeEdit::is_in_string_ex]."]
#[must_use]
pub struct ExIsInString < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::CodeEdit, line: i32, column: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsInString < 'ex > {
    fn new(surround_object: &'ex re_export::CodeEdit, line: i32,) -> Self {
        let column = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, column: column,
        }
    }
    #[inline]
    pub fn column(self, column: i32) -> Self {
        Self {
            column: column, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line, column,
        }
        = self;
        re_export::CodeEdit::is_in_string_full(surround_object, line, column,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::add_comment_delimiter_ex`][super::CodeEdit::add_comment_delimiter_ex]."]
#[must_use]
pub struct ExAddCommentDelimiter < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CodeEdit, start_key: CowArg < 'ex, GString >, end_key: CowArg < 'ex, GString >, line_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddCommentDelimiter < 'ex > {
    fn new(surround_object: &'ex mut re_export::CodeEdit, start_key: impl AsArg < GString > + 'ex, end_key: impl AsArg < GString > + 'ex,) -> Self {
        let line_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start_key: start_key.into_arg(), end_key: end_key.into_arg(), line_only: line_only,
        }
    }
    #[inline]
    pub fn line_only(self, line_only: bool) -> Self {
        Self {
            line_only: line_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start_key, end_key, line_only,
        }
        = self;
        re_export::CodeEdit::add_comment_delimiter_full(surround_object, start_key, end_key, line_only,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::is_in_comment_ex`][super::CodeEdit::is_in_comment_ex]."]
#[must_use]
pub struct ExIsInComment < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::CodeEdit, line: i32, column: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsInComment < 'ex > {
    fn new(surround_object: &'ex re_export::CodeEdit, line: i32,) -> Self {
        let column = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, column: column,
        }
    }
    #[inline]
    pub fn column(self, column: i32) -> Self {
        Self {
            column: column, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line, column,
        }
        = self;
        re_export::CodeEdit::is_in_comment_full(surround_object, line, column,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::request_code_completion_ex`][super::CodeEdit::request_code_completion_ex]."]
#[must_use]
pub struct ExRequestCodeCompletion < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CodeEdit, force: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExRequestCodeCompletion < 'ex > {
    fn new(surround_object: &'ex mut re_export::CodeEdit,) -> Self {
        let force = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: bool) -> Self {
        Self {
            force: force, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force,
        }
        = self;
        re_export::CodeEdit::request_code_completion_full(surround_object, force,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::add_code_completion_option_ex`][super::CodeEdit::add_code_completion_option_ex]."]
#[must_use]
pub struct ExAddCodeCompletionOption < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CodeEdit, type_: crate::classes::code_edit::CodeCompletionKind, display_text: CowArg < 'ex, GString >, insert_text: CowArg < 'ex, GString >, text_color: Color, icon: CowArg < 'ex, Option < Gd < crate::classes::Resource > > >, value: CowArg < 'ex, Variant >, location: crate::classes::code_edit::CodeCompletionLocation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddCodeCompletionOption < 'ex > {
    fn new(surround_object: &'ex mut re_export::CodeEdit, type_: crate::classes::code_edit::CodeCompletionKind, display_text: impl AsArg < GString > + 'ex, insert_text: impl AsArg < GString > + 'ex,) -> Self {
        let text_color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let icon = Gd::null_arg();
        let value = Variant::nil();
        let location = crate::obj::EngineEnum::from_ord(1024);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, type_: type_, display_text: display_text.into_arg(), insert_text: insert_text.into_arg(), text_color: text_color, icon: icon.into_arg(), value: CowArg::Owned(value), location: location,
        }
    }
    #[inline]
    pub fn text_color(self, text_color: Color) -> Self {
        Self {
            text_color: text_color, .. self
        }
    }
    #[inline]
    pub fn icon(self, icon: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'ex) -> Self {
        Self {
            icon: icon.into_arg(), .. self
        }
    }
    #[inline]
    pub fn value(self, value: &'ex Variant) -> Self {
        Self {
            value: CowArg::Borrowed(value), .. self
        }
    }
    #[inline]
    pub fn location(self, location: crate::classes::code_edit::CodeCompletionLocation) -> Self {
        Self {
            location: location, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, type_, display_text, insert_text, text_color, icon, value, location,
        }
        = self;
        re_export::CodeEdit::add_code_completion_option_full(surround_object, type_, display_text, insert_text, text_color, icon, value.cow_as_arg(), location,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::confirm_code_completion_ex`][super::CodeEdit::confirm_code_completion_ex]."]
#[must_use]
pub struct ExConfirmCodeCompletion < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CodeEdit, replace: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExConfirmCodeCompletion < 'ex > {
    fn new(surround_object: &'ex mut re_export::CodeEdit,) -> Self {
        let replace = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, replace: replace,
        }
    }
    #[inline]
    pub fn replace(self, replace: bool) -> Self {
        Self {
            replace: replace, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, replace,
        }
        = self;
        re_export::CodeEdit::confirm_code_completion_full(surround_object, replace,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CodeCompletionKind {
    ord: i32
}
impl CodeCompletionKind {
    #[doc(alias = "KIND_CLASS")]
    #[doc = "Godot enumerator name: `KIND_CLASS`"]
    pub const CLASS: CodeCompletionKind = CodeCompletionKind {
        ord: 0i32
    };
    #[doc(alias = "KIND_FUNCTION")]
    #[doc = "Godot enumerator name: `KIND_FUNCTION`"]
    pub const FUNCTION: CodeCompletionKind = CodeCompletionKind {
        ord: 1i32
    };
    #[doc(alias = "KIND_SIGNAL")]
    #[doc = "Godot enumerator name: `KIND_SIGNAL`"]
    pub const SIGNAL: CodeCompletionKind = CodeCompletionKind {
        ord: 2i32
    };
    #[doc(alias = "KIND_VARIABLE")]
    #[doc = "Godot enumerator name: `KIND_VARIABLE`"]
    pub const VARIABLE: CodeCompletionKind = CodeCompletionKind {
        ord: 3i32
    };
    #[doc(alias = "KIND_MEMBER")]
    #[doc = "Godot enumerator name: `KIND_MEMBER`"]
    pub const MEMBER: CodeCompletionKind = CodeCompletionKind {
        ord: 4i32
    };
    #[doc(alias = "KIND_ENUM")]
    #[doc = "Godot enumerator name: `KIND_ENUM`"]
    pub const ENUM: CodeCompletionKind = CodeCompletionKind {
        ord: 5i32
    };
    #[doc(alias = "KIND_CONSTANT")]
    #[doc = "Godot enumerator name: `KIND_CONSTANT`"]
    pub const CONSTANT: CodeCompletionKind = CodeCompletionKind {
        ord: 6i32
    };
    #[doc(alias = "KIND_NODE_PATH")]
    #[doc = "Godot enumerator name: `KIND_NODE_PATH`"]
    pub const NODE_PATH: CodeCompletionKind = CodeCompletionKind {
        ord: 7i32
    };
    #[doc(alias = "KIND_FILE_PATH")]
    #[doc = "Godot enumerator name: `KIND_FILE_PATH`"]
    pub const FILE_PATH: CodeCompletionKind = CodeCompletionKind {
        ord: 8i32
    };
    #[doc(alias = "KIND_PLAIN_TEXT")]
    #[doc = "Godot enumerator name: `KIND_PLAIN_TEXT`"]
    pub const PLAIN_TEXT: CodeCompletionKind = CodeCompletionKind {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for CodeCompletionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CodeCompletionKind") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CodeCompletionKind {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::CLASS => "CLASS", Self::FUNCTION => "FUNCTION", Self::SIGNAL => "SIGNAL", Self::VARIABLE => "VARIABLE", Self::MEMBER => "MEMBER", Self::ENUM => "ENUM", Self::CONSTANT => "CONSTANT", Self::NODE_PATH => "NODE_PATH", Self::FILE_PATH => "FILE_PATH", Self::PLAIN_TEXT => "PLAIN_TEXT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CodeCompletionKind::CLASS, CodeCompletionKind::FUNCTION, CodeCompletionKind::SIGNAL, CodeCompletionKind::VARIABLE, CodeCompletionKind::MEMBER, CodeCompletionKind::ENUM, CodeCompletionKind::CONSTANT, CodeCompletionKind::NODE_PATH, CodeCompletionKind::FILE_PATH, CodeCompletionKind::PLAIN_TEXT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CodeCompletionKind >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CLASS", "KIND_CLASS", CodeCompletionKind::CLASS), crate::meta::inspect::EnumConstant::new("FUNCTION", "KIND_FUNCTION", CodeCompletionKind::FUNCTION), crate::meta::inspect::EnumConstant::new("SIGNAL", "KIND_SIGNAL", CodeCompletionKind::SIGNAL), crate::meta::inspect::EnumConstant::new("VARIABLE", "KIND_VARIABLE", CodeCompletionKind::VARIABLE), crate::meta::inspect::EnumConstant::new("MEMBER", "KIND_MEMBER", CodeCompletionKind::MEMBER), crate::meta::inspect::EnumConstant::new("ENUM", "KIND_ENUM", CodeCompletionKind::ENUM), crate::meta::inspect::EnumConstant::new("CONSTANT", "KIND_CONSTANT", CodeCompletionKind::CONSTANT), crate::meta::inspect::EnumConstant::new("NODE_PATH", "KIND_NODE_PATH", CodeCompletionKind::NODE_PATH), crate::meta::inspect::EnumConstant::new("FILE_PATH", "KIND_FILE_PATH", CodeCompletionKind::FILE_PATH), crate::meta::inspect::EnumConstant::new("PLAIN_TEXT", "KIND_PLAIN_TEXT", CodeCompletionKind::PLAIN_TEXT)]
        }
    }
}
impl crate::meta::GodotConvert for CodeCompletionKind {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Kind Class", 0i64), EnumeratorShape::new_int("Kind Function", 1i64), EnumeratorShape::new_int("Kind Signal", 2i64), EnumeratorShape::new_int("Kind Variable", 3i64), EnumeratorShape::new_int("Kind Member", 4i64), EnumeratorShape::new_int("Kind Enum", 5i64), EnumeratorShape::new_int("Kind Constant", 6i64), EnumeratorShape::new_int("Kind Node Path", 7i64), EnumeratorShape::new_int("Kind File Path", 8i64), EnumeratorShape::new_int("Kind Plain Text", 9i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CodeEdit.CodeCompletionKind")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CodeCompletionKind {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CodeCompletionKind {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CodeCompletionKind {
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
impl crate::registry::property::Export for CodeCompletionKind {
    
}
impl crate::meta::Element for CodeCompletionKind {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CodeCompletionLocation {
    ord: i32
}
impl CodeCompletionLocation {
    #[doc(alias = "LOCATION_LOCAL")]
    #[doc = "Godot enumerator name: `LOCATION_LOCAL`"]
    pub const LOCAL: CodeCompletionLocation = CodeCompletionLocation {
        ord: 0i32
    };
    #[doc(alias = "LOCATION_PARENT_MASK")]
    #[doc = "Godot enumerator name: `LOCATION_PARENT_MASK`"]
    pub const PARENT_MASK: CodeCompletionLocation = CodeCompletionLocation {
        ord: 256i32
    };
    #[doc(alias = "LOCATION_OTHER_USER_CODE")]
    #[doc = "Godot enumerator name: `LOCATION_OTHER_USER_CODE`"]
    pub const OTHER_USER_CODE: CodeCompletionLocation = CodeCompletionLocation {
        ord: 512i32
    };
    #[doc(alias = "LOCATION_OTHER")]
    #[doc = "Godot enumerator name: `LOCATION_OTHER`"]
    pub const OTHER: CodeCompletionLocation = CodeCompletionLocation {
        ord: 1024i32
    };
    
}
impl std::fmt::Debug for CodeCompletionLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CodeCompletionLocation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CodeCompletionLocation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 256i32 | ord @ 512i32 | ord @ 1024i32 => Some(Self {
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
            Self::LOCAL => "LOCAL", Self::PARENT_MASK => "PARENT_MASK", Self::OTHER_USER_CODE => "OTHER_USER_CODE", Self::OTHER => "OTHER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CodeCompletionLocation::LOCAL, CodeCompletionLocation::PARENT_MASK, CodeCompletionLocation::OTHER_USER_CODE, CodeCompletionLocation::OTHER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CodeCompletionLocation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LOCAL", "LOCATION_LOCAL", CodeCompletionLocation::LOCAL), crate::meta::inspect::EnumConstant::new("PARENT_MASK", "LOCATION_PARENT_MASK", CodeCompletionLocation::PARENT_MASK), crate::meta::inspect::EnumConstant::new("OTHER_USER_CODE", "LOCATION_OTHER_USER_CODE", CodeCompletionLocation::OTHER_USER_CODE), crate::meta::inspect::EnumConstant::new("OTHER", "LOCATION_OTHER", CodeCompletionLocation::OTHER)]
        }
    }
}
impl crate::meta::GodotConvert for CodeCompletionLocation {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Location Local", 0i64), EnumeratorShape::new_int("Location Parent Mask", 256i64), EnumeratorShape::new_int("Location Other User Code", 512i64), EnumeratorShape::new_int("Location Other", 1024i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CodeEdit.CodeCompletionLocation")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CodeCompletionLocation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CodeCompletionLocation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CodeCompletionLocation {
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
impl crate::registry::property::Export for CodeCompletionLocation {
    
}
impl crate::meta::Element for CodeCompletionLocation {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CodeEdit;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`CodeEdit`][crate::classes::CodeEdit] class."]
    pub struct SignalsOfCodeEdit < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfCodeEdit < 'c, C > {
        #[doc = "Signature: `(line: i64)`"]
        pub fn breakpoint_toggled(&mut self) -> SigBreakpointToggled < 'c, C > {
            SigBreakpointToggled {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "breakpoint_toggled")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn code_completion_requested(&mut self) -> SigCodeCompletionRequested < 'c, C > {
            SigCodeCompletionRequested {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "code_completion_requested")
            }
        }
        #[doc = "Signature: `(symbol: GString, line: i64, column: i64)`"]
        pub fn symbol_lookup(&mut self) -> SigSymbolLookup < 'c, C > {
            SigSymbolLookup {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "symbol_lookup")
            }
        }
        #[doc = "Signature: `(symbol: GString)`"]
        pub fn symbol_validate(&mut self) -> SigSymbolValidate < 'c, C > {
            SigSymbolValidate {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "symbol_validate")
            }
        }
        #[doc = "Signature: `(symbol: GString, line: i64, column: i64)`"]
        pub fn symbol_hovered(&mut self) -> SigSymbolHovered < 'c, C > {
            SigSymbolHovered {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "symbol_hovered")
            }
        }
    }
    type TypedSigBreakpointToggled < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigBreakpointToggled < 'c, C: WithSignals > {
        typed: TypedSigBreakpointToggled < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBreakpointToggled < 'c, C > {
        pub fn emit(&mut self, line: i64,) {
            self.typed.emit_tuple((line,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBreakpointToggled < 'c, C > {
        type Target = TypedSigBreakpointToggled < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBreakpointToggled < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCodeCompletionRequested < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigCodeCompletionRequested < 'c, C: WithSignals > {
        typed: TypedSigCodeCompletionRequested < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCodeCompletionRequested < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCodeCompletionRequested < 'c, C > {
        type Target = TypedSigCodeCompletionRequested < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCodeCompletionRequested < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSymbolLookup < 'c, C > = TypedSignal < 'c, C, (GString, i64, i64,) >;
    pub struct SigSymbolLookup < 'c, C: WithSignals > {
        typed: TypedSigSymbolLookup < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSymbolLookup < 'c, C > {
        pub fn emit(&mut self, symbol: GString, line: i64, column: i64,) {
            self.typed.emit_tuple((symbol, line, column,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSymbolLookup < 'c, C > {
        type Target = TypedSigSymbolLookup < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSymbolLookup < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSymbolValidate < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigSymbolValidate < 'c, C: WithSignals > {
        typed: TypedSigSymbolValidate < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSymbolValidate < 'c, C > {
        pub fn emit(&mut self, symbol: GString,) {
            self.typed.emit_tuple((symbol,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSymbolValidate < 'c, C > {
        type Target = TypedSigSymbolValidate < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSymbolValidate < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSymbolHovered < 'c, C > = TypedSignal < 'c, C, (GString, i64, i64,) >;
    pub struct SigSymbolHovered < 'c, C: WithSignals > {
        typed: TypedSigSymbolHovered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSymbolHovered < 'c, C > {
        pub fn emit(&mut self, symbol: GString, line: i64, column: i64,) {
            self.typed.emit_tuple((symbol, line, column,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSymbolHovered < 'c, C > {
        type Target = TypedSigSymbolHovered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSymbolHovered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for CodeEdit {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCodeEdit < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfCodeEdit < 'c, C > {
        type Target = < < CodeEdit as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = CodeEdit;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfCodeEdit < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = CodeEdit;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
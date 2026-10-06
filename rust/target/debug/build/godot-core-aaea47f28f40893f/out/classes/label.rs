#![doc = "Sidecar module for class [`Label`][crate::classes::Label].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Label` enums](https://docs.godotengine.org/en/stable/classes/class_label.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Label`.\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`label`][crate::classes::label]: sidecar module with related enum/flag types\n* [`ILabel`][crate::classes::ILabel]: virtual methods\n\n\nSee also [Godot docs for `Label`](https://docs.godotengine.org/en/stable/classes/class_label.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Label::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nA control for displaying plain text. It gives you control over the horizontal and vertical alignment and can wrap the text inside the node's bounding rectangle. It doesn't support bold, italics, or other rich text formatting. For that, use [`RichTextLabel`][crate::classes::RichTextLabel] instead.\n\n**Note:** A single Label node is not designed to display huge amounts of text. To display large amounts of text in a single node, consider using [`RichTextLabel`][crate::classes::RichTextLabel] instead as it supports features like an integrated scroll bar and threading. [`RichTextLabel`][crate::classes::RichTextLabel] generally performs better when displaying large amounts of text (several pages or more)."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Label {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Label`][crate::classes::Label].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Label` methods](https://docs.godotengine.org/en/stable/classes/class_label.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILabel: crate::obj::GodotClass < Base = Label > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Label {
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::HorizontalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertical_alignment(&mut self, alignment: crate::global::VerticalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::VerticalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_vertical_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertical_alignment(&self,) -> crate::global::VerticalAlignment {
            type CallRet = crate::global::VerticalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_vertical_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_label_settings(&mut self, settings: impl AsArg < Option < Gd < crate::classes::LabelSettings >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::LabelSettings > > >,);
            let args = (settings.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_label_settings", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_label_settings(&self,) -> Option < Gd < crate::classes::LabelSettings > > {
            type CallRet = Option < Gd < crate::classes::LabelSettings > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_label_settings", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::TextDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_paragraph_separator(&mut self, paragraph_separator: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (paragraph_separator.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_paragraph_separator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_paragraph_separator(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_paragraph_separator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::AutowrapMode,);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::classes::text_server::AutowrapMode {
            type CallRet = crate::classes::text_server::AutowrapMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_trim_flags(&mut self, autowrap_trim_flags: crate::classes::text_server::LineBreakFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::LineBreakFlag,);
            let args = (autowrap_trim_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_autowrap_trim_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_trim_flags(&self,) -> crate::classes::text_server::LineBreakFlag {
            type CallRet = crate::classes::text_server::LineBreakFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_autowrap_trim_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_justification_flags(&mut self, justification_flags: crate::classes::text_server::JustificationFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::JustificationFlag,);
            let args = (justification_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_justification_flags(&self,) -> crate::classes::text_server::JustificationFlag {
            type CallRet = crate::classes::text_server::JustificationFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clip_text(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_clip_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_clipping_text(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "is_clipping_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_stops(&mut self, tab_stops: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat32Array >,);
            let args = (RefArg::new(tab_stops),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_tab_stops", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_stops(&self,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_tab_stops", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::OverrunBehavior,);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::classes::text_server::OverrunBehavior {
            type CallRet = crate::classes::text_server::OverrunBehavior;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ellipsis_char(&mut self, char: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (char.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_ellipsis_char", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ellipsis_char(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_ellipsis_char", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uppercase(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_uppercase", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_uppercase(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "is_uppercase", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the height of the line `line`.\n\nIf `line` is set to `-1`, returns the biggest line height.\n\nIf there are no lines, returns font size in pixels."]
        pub(crate) fn get_line_height_full(&self, line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_line_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_line_height_ex`][Self::get_line_height_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the height of the line `line`.\n\nIf `line` is set to `-1`, returns the biggest line height.\n\nIf there are no lines, returns font size in pixels."]
        #[inline]
        pub fn get_line_height(&self,) -> i32 {
            self.get_line_height_ex() . done()
        }
        #[doc = "Returns the height of the line `line`.\n\nIf `line` is set to `-1`, returns the biggest line height.\n\nIf there are no lines, returns font size in pixels."]
        #[inline]
        pub fn get_line_height_ex < 'ex > (&'ex self,) -> ExGetLineHeight < 'ex > {
            ExGetLineHeight::new(self,)
        }
        #[doc = "Returns the number of lines of text the Label has."]
        pub fn get_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_line_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of lines shown. Useful if the `Label`'s height cannot currently display all lines."]
        pub fn get_visible_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_visible_line_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total number of printable characters in the text (excluding spaces and newlines)."]
        pub fn get_total_character_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_total_character_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_characters(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_visible_characters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_characters(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_visible_characters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_characters_behavior(&self,) -> crate::classes::text_server::VisibleCharactersBehavior {
            type CallRet = crate::classes::text_server::VisibleCharactersBehavior;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_visible_characters_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_characters_behavior(&mut self, behavior: crate::classes::text_server::VisibleCharactersBehavior,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::VisibleCharactersBehavior,);
            let args = (behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_visible_characters_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_visible_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_visible_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lines_skipped(&mut self, lines_skipped: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (lines_skipped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_lines_skipped", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lines_skipped(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_lines_skipped", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_lines_visible(&mut self, lines_visible: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (lines_visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_max_lines_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_lines_visible(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_max_lines_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::StructuredTextParser,);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallRet = crate::classes::text_server::StructuredTextParser;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(806usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the bounding rectangle of the character at position `pos` in the label's local coordinate system. If the character is a non-visual character or `pos` is outside the valid range, an empty [`Rect2`][crate::builtin::Rect2] is returned. If the character is a part of a composite grapheme, the bounding rectangle of the whole grapheme is returned."]
        pub fn get_character_bounds(&self, pos: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32,);
            let args = (pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(807usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_character_bounds", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Label {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Label"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Label {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for Label {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Label {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Label {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Label {
        
    }
    impl crate::obj::cap::GodotDefault for Label {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Label {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Label {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Label`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Label__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Label > for $Class {
                
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
#[doc = "Default-param extender for [`Label::get_line_height_ex`][super::Label::get_line_height_ex]."]
#[must_use]
pub struct ExGetLineHeight < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Label, line: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetLineHeight < 'ex > {
    fn new(surround_object: &'ex re_export::Label,) -> Self {
        let line = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line,
        }
    }
    #[inline]
    pub fn line(self, line: i32) -> Self {
        Self {
            line: line, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line,
        }
        = self;
        re_export::Label::get_line_height_full(surround_object, line,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Label;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::control::SignalsOfControl;
    impl WithSignals for Label {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfControl < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
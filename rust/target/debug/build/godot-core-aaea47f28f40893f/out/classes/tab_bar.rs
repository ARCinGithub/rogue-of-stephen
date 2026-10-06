#![doc = "Sidecar module for class [`TabBar`][crate::classes::TabBar].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TabBar` enums](https://docs.godotengine.org/en/stable/classes/class_tabbar.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TabBar`.\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`tab_bar`][crate::classes::tab_bar]: sidecar module with related enum/flag types\n* [`ITabBar`][crate::classes::ITabBar]: virtual methods\n* [`SignalsOfTabBar`][crate::classes::tab_bar::SignalsOfTabBar]: signal collection\n\n\nSee also [Godot docs for `TabBar`](https://docs.godotengine.org/en/stable/classes/class_tabbar.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TabBar::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nA control that provides a horizontal bar with tabs. Similar to [`TabContainer`][crate::classes::TabContainer] but is only in charge of drawing tabs, not interacting with children."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TabBar {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TabBar`][crate::classes::TabBar].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `TabBar` methods](https://docs.godotengine.org/en/stable/classes/class_tabbar.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITabBar: crate::obj::GodotClass < Base = TabBar > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TabBar {
        pub fn set_tab_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(507usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(508usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_tab(&mut self, tab_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(509usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_current_tab", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_tab(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(510usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_current_tab", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the previously active tab index."]
        pub fn get_previous_tab(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(511usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_previous_tab", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Selects the first available tab with lower index than the currently selected. Returns `true` if tab selection changed."]
        pub fn select_previous_available(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(512usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "select_previous_available", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Selects the first available tab with greater index than the currently selected. Returns `true` if tab selection changed."]
        pub fn select_next_available(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(513usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "select_next_available", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a `title` for the tab at index `tab_idx`."]
        pub fn set_tab_title(&mut self, tab_idx: i32, title: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (tab_idx, title.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(514usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_title", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the title of the tab at index `tab_idx`."]
        pub fn get_tab_title(&self, tab_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(515usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_title", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a `tooltip` for tab at index `tab_idx`.\n\n**Note:** By default, if the `tooltip` is empty and the tab text is truncated (not all characters fit into the tab), the title will be displayed as a tooltip. To hide the tooltip, assign `\" \"` as the `tooltip` text."]
        pub fn set_tab_tooltip(&mut self, tab_idx: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (tab_idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(516usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tooltip text of the tab at index `tab_idx`."]
        pub fn get_tab_tooltip(&self, tab_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(517usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets tab title base writing direction."]
        pub fn set_tab_text_direction(&mut self, tab_idx: i32, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::control::TextDirection,);
            let args = (tab_idx, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(518usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns tab title text base writing direction."]
        pub fn get_tab_text_direction(&self, tab_idx: i32,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(519usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the language code of the title for the tab at index `tab_idx` to `language`. This is used for line-breaking and text shaping algorithms. If `language` is empty, the current locale is used."]
        pub fn set_tab_language(&mut self, tab_idx: i32, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (tab_idx, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(520usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns tab title language code."]
        pub fn get_tab_language(&self, tab_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(521usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets an `icon` for the tab at index `tab_idx`."]
        pub fn set_tab_icon(&mut self, tab_idx: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (tab_idx, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(522usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the icon for the tab at index `tab_idx` or `null` if the tab has no icon."]
        pub fn get_tab_icon(&self, tab_idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(523usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the maximum allowed width of the icon for the tab at index `tab_idx`. This limit is applied on top of the default size of the icon and on top of [theme_item icon_max_width]. The height is adjusted according to the icon's ratio."]
        pub fn set_tab_icon_max_width(&mut self, tab_idx: i32, width: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (tab_idx, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(524usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_icon_max_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the maximum allowed width of the icon for the tab at index `tab_idx`."]
        pub fn get_tab_icon_max_width(&self, tab_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(525usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_icon_max_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets an `icon` for the button of the tab at index `tab_idx` (located to the right, before the close button), making it visible and clickable (See `tab_button_pressed`). Giving it a `null` value will hide the button."]
        pub fn set_tab_button_icon(&mut self, tab_idx: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (tab_idx, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(526usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_button_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the icon for the right button of the tab at index `tab_idx` or `null` if the right button has no icon."]
        pub fn get_tab_button_icon(&self, tab_idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(527usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_button_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `disabled` is `true`, disables the tab at index `tab_idx`, making it non-interactable."]
        pub fn set_tab_disabled(&mut self, tab_idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (tab_idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(528usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the tab at index `tab_idx` is disabled."]
        pub fn is_tab_disabled(&self, tab_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(529usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "is_tab_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `hidden` is `true`, hides the tab at index `tab_idx`, making it disappear from the tab area."]
        pub fn set_tab_hidden(&mut self, tab_idx: i32, hidden: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (tab_idx, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(530usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_hidden", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the tab at index `tab_idx` is hidden."]
        pub fn is_tab_hidden(&self, tab_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(531usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "is_tab_hidden", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the metadata value for the tab at index `tab_idx`, which can be retrieved later using [`get_tab_metadata`][`crate::classes::TabBar::get_tab_metadata`]."]
        pub fn set_tab_metadata(&mut self, tab_idx: i32, metadata: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (tab_idx, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(532usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the metadata value set to the tab at index `tab_idx` using [`set_tab_metadata`][`crate::classes::TabBar::set_tab_metadata`]. If no metadata was previously set, returns `null` by default."]
        pub fn get_tab_metadata(&self, tab_idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(533usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the tab at index `tab_idx`."]
        pub fn remove_tab(&mut self, tab_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(534usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "remove_tab", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a new tab."]
        pub(crate) fn add_tab_full(&mut self, title: CowArg < GString >, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (title, icon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(535usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "add_tab", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_tab_ex`][Self::add_tab_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new tab."]
        #[inline]
        pub fn add_tab(&mut self,) {
            self.add_tab_ex() . done()
        }
        #[doc = "Adds a new tab."]
        #[inline]
        pub fn add_tab_ex < 'ex > (&'ex mut self,) -> ExAddTab < 'ex > {
            ExAddTab::new(self,)
        }
        #[doc = "Returns the index of the tab at local coordinates `point`. Returns `-1` if the point is outside the control boundaries or if there's no tab at the queried position."]
        pub fn get_tab_idx_at_point(&self, point: Vector2,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(536usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_idx_at_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_alignment(&mut self, alignment: crate::classes::tab_bar::AlignmentMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::tab_bar::AlignmentMode,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(537usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_alignment(&self,) -> crate::classes::tab_bar::AlignmentMode {
            type CallRet = crate::classes::tab_bar::AlignmentMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(538usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clip_tabs(&mut self, clip_tabs: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (clip_tabs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(539usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_clip_tabs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_clip_tabs(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(540usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_clip_tabs", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of hidden tabs offsetted to the left."]
        pub fn get_tab_offset(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(541usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the offset buttons (the ones that appear when there's not enough space for all tabs) are visible."]
        pub fn get_offset_buttons_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_offset_buttons_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves the scroll view to make the tab visible."]
        pub fn ensure_tab_visible(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "ensure_tab_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns tab [`Rect2`][crate::builtin::Rect2] with local position and size."]
        pub fn get_tab_rect(&self, tab_idx: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves a tab from `from` to `to`."]
        pub fn move_tab(&mut self, from: i32, to: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(545usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "move_tab", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_close_with_middle_mouse(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(546usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_close_with_middle_mouse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_close_with_middle_mouse(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(547usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_close_with_middle_mouse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_close_display_policy(&mut self, policy: crate::classes::tab_bar::CloseButtonDisplayPolicy,) {
            type CallRet = ();
            type CallParams = (crate::classes::tab_bar::CloseButtonDisplayPolicy,);
            let args = (policy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(548usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tab_close_display_policy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_close_display_policy(&self,) -> crate::classes::tab_bar::CloseButtonDisplayPolicy {
            type CallRet = crate::classes::tab_bar::CloseButtonDisplayPolicy;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(549usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tab_close_display_policy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_tab_width(&mut self, width: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(550usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_max_tab_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_tab_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(551usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_max_tab_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scrolling_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(552usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_scrolling_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scrolling_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(553usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_scrolling_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_to_rearrange_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_drag_to_rearrange_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drag_to_rearrange_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_drag_to_rearrange_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_switch_on_drag_hover(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_switch_on_drag_hover", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_switch_on_drag_hover(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_switch_on_drag_hover", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tabs_rearrange_group(&mut self, group_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_tabs_rearrange_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tabs_rearrange_group(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_tabs_rearrange_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scroll_to_selected(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_scroll_to_selected", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scroll_to_selected(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(561usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_scroll_to_selected", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_select_with_rmb(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(562usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_select_with_rmb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_select_with_rmb(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(563usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_select_with_rmb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_deselect_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(564usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "set_deselect_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_deselect_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(565usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "get_deselect_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all tabs."]
        pub fn clear_tabs(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(566usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabBar", "clear_tabs", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TabBar {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TabBar"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TabBar {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for TabBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TabBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TabBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TabBar {
        
    }
    impl crate::obj::cap::GodotDefault for TabBar {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TabBar {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TabBar {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TabBar`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TabBar__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TabBar > for $Class {
                
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
#[doc = "Default-param extender for [`TabBar::add_tab_ex`][super::TabBar::add_tab_ex]."]
#[must_use]
pub struct ExAddTab < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TabBar, title: CowArg < 'ex, GString >, icon: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddTab < 'ex > {
    fn new(surround_object: &'ex mut re_export::TabBar,) -> Self {
        let title = GString::from("");
        let icon = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, title: CowArg::Owned(title), icon: icon.into_arg(),
        }
    }
    #[inline]
    pub fn title(self, title: impl AsArg < GString > + 'ex) -> Self {
        Self {
            title: title.into_arg(), .. self
        }
    }
    #[inline]
    pub fn icon(self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex) -> Self {
        Self {
            icon: icon.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, title, icon,
        }
        = self;
        re_export::TabBar::add_tab_full(surround_object, title, icon,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AlignmentMode {
    ord: i32
}
impl AlignmentMode {
    #[doc(alias = "ALIGNMENT_LEFT")]
    #[doc = "Godot enumerator name: `ALIGNMENT_LEFT`"]
    pub const LEFT: AlignmentMode = AlignmentMode {
        ord: 0i32
    };
    #[doc(alias = "ALIGNMENT_CENTER")]
    #[doc = "Godot enumerator name: `ALIGNMENT_CENTER`"]
    pub const CENTER: AlignmentMode = AlignmentMode {
        ord: 1i32
    };
    #[doc(alias = "ALIGNMENT_RIGHT")]
    #[doc = "Godot enumerator name: `ALIGNMENT_RIGHT`"]
    pub const RIGHT: AlignmentMode = AlignmentMode {
        ord: 2i32
    };
    #[doc(alias = "ALIGNMENT_MAX")]
    #[doc = "Godot enumerator name: `ALIGNMENT_MAX`"]
    pub const MAX: AlignmentMode = AlignmentMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AlignmentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AlignmentMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AlignmentMode {
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
            Self::LEFT => "LEFT", Self::CENTER => "CENTER", Self::RIGHT => "RIGHT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AlignmentMode::LEFT, AlignmentMode::CENTER, AlignmentMode::RIGHT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AlignmentMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LEFT", "ALIGNMENT_LEFT", AlignmentMode::LEFT), crate::meta::inspect::EnumConstant::new("CENTER", "ALIGNMENT_CENTER", AlignmentMode::CENTER), crate::meta::inspect::EnumConstant::new("RIGHT", "ALIGNMENT_RIGHT", AlignmentMode::RIGHT), crate::meta::inspect::EnumConstant::new("MAX", "ALIGNMENT_MAX", AlignmentMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for AlignmentMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for AlignmentMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Alignment Left", 0i64), EnumeratorShape::new_int("Alignment Center", 1i64), EnumeratorShape::new_int("Alignment Right", 2i64), EnumeratorShape::new_int("Alignment Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TabBar.AlignmentMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AlignmentMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AlignmentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AlignmentMode {
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
impl crate::registry::property::Export for AlignmentMode {
    
}
impl crate::meta::Element for AlignmentMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CloseButtonDisplayPolicy {
    ord: i32
}
impl CloseButtonDisplayPolicy {
    #[doc(alias = "CLOSE_BUTTON_SHOW_NEVER")]
    #[doc = "Godot enumerator name: `CLOSE_BUTTON_SHOW_NEVER`"]
    pub const SHOW_NEVER: CloseButtonDisplayPolicy = CloseButtonDisplayPolicy {
        ord: 0i32
    };
    #[doc(alias = "CLOSE_BUTTON_SHOW_ACTIVE_ONLY")]
    #[doc = "Godot enumerator name: `CLOSE_BUTTON_SHOW_ACTIVE_ONLY`"]
    pub const SHOW_ACTIVE_ONLY: CloseButtonDisplayPolicy = CloseButtonDisplayPolicy {
        ord: 1i32
    };
    #[doc(alias = "CLOSE_BUTTON_SHOW_ALWAYS")]
    #[doc = "Godot enumerator name: `CLOSE_BUTTON_SHOW_ALWAYS`"]
    pub const SHOW_ALWAYS: CloseButtonDisplayPolicy = CloseButtonDisplayPolicy {
        ord: 2i32
    };
    #[doc(alias = "CLOSE_BUTTON_MAX")]
    #[doc = "Godot enumerator name: `CLOSE_BUTTON_MAX`"]
    pub const MAX: CloseButtonDisplayPolicy = CloseButtonDisplayPolicy {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for CloseButtonDisplayPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CloseButtonDisplayPolicy") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CloseButtonDisplayPolicy {
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
            Self::SHOW_NEVER => "SHOW_NEVER", Self::SHOW_ACTIVE_ONLY => "SHOW_ACTIVE_ONLY", Self::SHOW_ALWAYS => "SHOW_ALWAYS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CloseButtonDisplayPolicy::SHOW_NEVER, CloseButtonDisplayPolicy::SHOW_ACTIVE_ONLY, CloseButtonDisplayPolicy::SHOW_ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CloseButtonDisplayPolicy >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SHOW_NEVER", "CLOSE_BUTTON_SHOW_NEVER", CloseButtonDisplayPolicy::SHOW_NEVER), crate::meta::inspect::EnumConstant::new("SHOW_ACTIVE_ONLY", "CLOSE_BUTTON_SHOW_ACTIVE_ONLY", CloseButtonDisplayPolicy::SHOW_ACTIVE_ONLY), crate::meta::inspect::EnumConstant::new("SHOW_ALWAYS", "CLOSE_BUTTON_SHOW_ALWAYS", CloseButtonDisplayPolicy::SHOW_ALWAYS), crate::meta::inspect::EnumConstant::new("MAX", "CLOSE_BUTTON_MAX", CloseButtonDisplayPolicy::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CloseButtonDisplayPolicy {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for CloseButtonDisplayPolicy {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Close Button Show Never", 0i64), EnumeratorShape::new_int("Close Button Show Active Only", 1i64), EnumeratorShape::new_int("Close Button Show Always", 2i64), EnumeratorShape::new_int("Close Button Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TabBar.CloseButtonDisplayPolicy")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CloseButtonDisplayPolicy {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CloseButtonDisplayPolicy {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CloseButtonDisplayPolicy {
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
impl crate::registry::property::Export for CloseButtonDisplayPolicy {
    
}
impl crate::meta::Element for CloseButtonDisplayPolicy {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TabBar;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`TabBar`][crate::classes::TabBar] class."]
    pub struct SignalsOfTabBar < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTabBar < 'c, C > {
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_selected(&mut self) -> SigTabSelected < 'c, C > {
            SigTabSelected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "tab_selected")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_changed(&mut self) -> SigTabChanged < 'c, C > {
            SigTabChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "tab_changed")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_clicked(&mut self) -> SigTabClicked < 'c, C > {
            SigTabClicked {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "tab_clicked")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_rmb_clicked(&mut self) -> SigTabRmbClicked < 'c, C > {
            SigTabRmbClicked {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "tab_rmb_clicked")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_close_pressed(&mut self) -> SigTabClosePressed < 'c, C > {
            SigTabClosePressed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "tab_close_pressed")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_button_pressed(&mut self) -> SigTabButtonPressed < 'c, C > {
            SigTabButtonPressed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "tab_button_pressed")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_hovered(&mut self) -> SigTabHovered < 'c, C > {
            SigTabHovered {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "tab_hovered")
            }
        }
        #[doc = "Signature: `(idx_to: i64)`"]
        pub fn active_tab_rearranged(&mut self) -> SigActiveTabRearranged < 'c, C > {
            SigActiveTabRearranged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "active_tab_rearranged")
            }
        }
    }
    type TypedSigTabSelected < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabSelected < 'c, C: WithSignals > {
        typed: TypedSigTabSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabSelected < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabSelected < 'c, C > {
        type Target = TypedSigTabSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabChanged < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabChanged < 'c, C: WithSignals > {
        typed: TypedSigTabChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabChanged < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabChanged < 'c, C > {
        type Target = TypedSigTabChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabClicked < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabClicked < 'c, C: WithSignals > {
        typed: TypedSigTabClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabClicked < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabClicked < 'c, C > {
        type Target = TypedSigTabClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabRmbClicked < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabRmbClicked < 'c, C: WithSignals > {
        typed: TypedSigTabRmbClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabRmbClicked < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabRmbClicked < 'c, C > {
        type Target = TypedSigTabRmbClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabRmbClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabClosePressed < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabClosePressed < 'c, C: WithSignals > {
        typed: TypedSigTabClosePressed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabClosePressed < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabClosePressed < 'c, C > {
        type Target = TypedSigTabClosePressed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabClosePressed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabButtonPressed < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabButtonPressed < 'c, C: WithSignals > {
        typed: TypedSigTabButtonPressed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabButtonPressed < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabButtonPressed < 'c, C > {
        type Target = TypedSigTabButtonPressed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabButtonPressed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabHovered < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabHovered < 'c, C: WithSignals > {
        typed: TypedSigTabHovered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabHovered < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabHovered < 'c, C > {
        type Target = TypedSigTabHovered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabHovered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigActiveTabRearranged < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigActiveTabRearranged < 'c, C: WithSignals > {
        typed: TypedSigActiveTabRearranged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigActiveTabRearranged < 'c, C > {
        pub fn emit(&mut self, idx_to: i64,) {
            self.typed.emit_tuple((idx_to,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigActiveTabRearranged < 'c, C > {
        type Target = TypedSigActiveTabRearranged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigActiveTabRearranged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for TabBar {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTabBar < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTabBar < 'c, C > {
        type Target = < < TabBar as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = TabBar;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTabBar < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = TabBar;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
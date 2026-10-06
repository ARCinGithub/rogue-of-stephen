#![doc = "Sidecar module for class [`Tree`][crate::classes::Tree].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Tree` enums](https://docs.godotengine.org/en/stable/classes/class_tree.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Tree`.\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`tree`][crate::classes::tree]: sidecar module with related enum/flag types\n* [`ITree`][crate::classes::ITree]: virtual methods\n* [`SignalsOfTree`][crate::classes::tree::SignalsOfTree]: signal collection\n\n\nSee also [Godot docs for `Tree`](https://docs.godotengine.org/en/stable/classes/class_tree.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Tree::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nA control used to show a set of internal [`TreeItem`][crate::classes::TreeItem]s in a hierarchical structure. The tree items can be selected, expanded and collapsed. The tree can have multiple columns with custom controls like [`LineEdit`][crate::classes::LineEdit]s, buttons and popups. It can be useful for structured displays and interactions.\n\nTrees are built via code, using [`TreeItem`][crate::classes::TreeItem] objects to create the structure. They have a single root, but multiple roots can be simulated with \\[member hide_root]:\n\n\n```gdscript\nfunc _ready():\n\tvar tree = Tree.new()\n\tvar root = tree.create_item()\n\ttree.hide_root = true\n\tvar child1 = tree.create_item(root)\n\tvar child2 = tree.create_item(root)\n\tvar subchild1 = tree.create_item(child1)\n\tsubchild1.set_text(0, \"Subchild1\")\n```\n\n\nTo iterate over all the [`TreeItem`][crate::classes::TreeItem] objects in a `Tree` object, use [`get_next`][`crate::classes::TreeItem::get_next`] and [`get_first_child`][`crate::classes::TreeItem::get_first_child`] after getting the root through [`get_root`][`crate::classes::Tree::get_root`]. You can use [`free`][`crate::obj::Gd::free`] on a [`TreeItem`][crate::classes::TreeItem] to remove it from the `Tree`.\n\n**Incremental search:** Like [`ItemList`][crate::classes::ItemList] and [`PopupMenu`][crate::classes::PopupMenu], `Tree` supports searching within the list while the control is focused. Press a key that matches the first letter of an item's name to select the first item starting with the given letter. After that point, there are two ways to perform incremental search: 1) Press the same key again before the timeout duration to select the next item starting with the same letter. 2) Press letter keys that match the rest of the word before the timeout duration to match to select the item in question directly. Both of these actions will be reset to the beginning of the list if the timeout duration has passed since the last keystroke was registered. You can adjust the timeout duration by changing \\[member ProjectSettings.gui/timers/incremental_search_max_interval_msec]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Tree {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Tree`][crate::classes::Tree].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Tree` methods](https://docs.godotengine.org/en/stable/classes/class_tree.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITree: crate::obj::GodotClass < Base = Tree > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Tree {
        #[doc = "Clears the tree. This removes all items."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates an item in the tree and adds it as a child of `parent`, which can be either a valid [`TreeItem`][crate::classes::TreeItem] or `null`.\n\nIf `parent` is `null`, the root item will be the parent, or the new item will be the root itself if the tree is empty.\n\nThe new item will be the `index`-th child of parent, or it will be the last child if there are not enough siblings."]
        pub(crate) fn create_item_full(&mut self, parent: CowArg < Option < Gd < crate::classes::TreeItem > > >, index: i32,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >, i32,);
            let args = (parent, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7438usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "create_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_item_ex`][Self::create_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates an item in the tree and adds it as a child of `parent`, which can be either a valid [`TreeItem`][crate::classes::TreeItem] or `null`.\n\nIf `parent` is `null`, the root item will be the parent, or the new item will be the root itself if the tree is empty.\n\nThe new item will be the `index`-th child of parent, or it will be the last child if there are not enough siblings."]
        #[inline]
        pub fn create_item(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.create_item_ex() . done()
        }
        #[doc = "Creates an item in the tree and adds it as a child of `parent`, which can be either a valid [`TreeItem`][crate::classes::TreeItem] or `null`.\n\nIf `parent` is `null`, the root item will be the parent, or the new item will be the root itself if the tree is empty.\n\nThe new item will be the `index`-th child of parent, or it will be the last child if there are not enough siblings."]
        #[inline]
        pub fn create_item_ex < 'ex > (&'ex mut self,) -> ExCreateItem < 'ex > {
            ExCreateItem::new(self,)
        }
        #[doc = "Returns the tree's root item, or `null` if the tree is empty."]
        pub fn get_root(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_root", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Overrides the calculated minimum width of a column. It can be set to `0` to restore the default behavior. Columns that have the \"Expand\" flag will use their \"min_width\" in a similar fashion to \\[member Control.size_flags_stretch_ratio]."]
        pub fn set_column_custom_minimum_width(&mut self, column: i32, min_width: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (column, min_width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_custom_minimum_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the column will have the \"Expand\" flag of [`Control`][crate::classes::Control]. Columns that have the \"Expand\" flag will use their expand ratio in a similar fashion to \\[member Control.size_flags_stretch_ratio] (see [`set_column_expand_ratio`][`crate::classes::Tree::set_column_expand_ratio`])."]
        pub fn set_column_expand(&mut self, column: i32, expand: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, expand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_expand", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the relative expand ratio for a column. See [`set_column_expand`][`crate::classes::Tree::set_column_expand`]."]
        pub fn set_column_expand_ratio(&mut self, column: i32, ratio: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (column, ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_expand_ratio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Allows to enable clipping for column's content, making the content size ignored."]
        pub fn set_column_clip_content(&mut self, column: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_clip_content", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the column has enabled expanding (see [`set_column_expand`][`crate::classes::Tree::set_column_expand`])."]
        pub fn is_column_expanding(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_column_expanding", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the column has enabled clipping (see [`set_column_clip_content`][`crate::classes::Tree::set_column_clip_content`])."]
        pub fn is_column_clipping_content(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_column_clipping_content", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the expand ratio assigned to the column."]
        pub fn get_column_expand_ratio(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_expand_ratio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the column's width in pixels."]
        pub fn get_column_width(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hide_root(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_hide_root", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_root_hidden(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_root_hidden", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next selected [`TreeItem`][crate::classes::TreeItem] after the given one, or `null` if the end is reached.\n\nIf `from` is `null`, this returns the first selected item."]
        pub fn get_next_selected(&self, from: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >,);
            let args = (from.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_next_selected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the currently focused item, or `null` if no item is focused.\n\nIn [`SelectMode::ROW`][`crate::classes::tree::SelectMode::ROW`] and [`SelectMode::SINGLE`][`crate::classes::tree::SelectMode::SINGLE`] modes, the focused item is same as the selected item. In [`SelectMode::MULTI`][`crate::classes::tree::SelectMode::MULTI`] mode, the focused item is the item under the focus cursor, not necessarily selected.\n\nTo get the currently selected item(s), use [`get_next_selected`][`crate::classes::Tree::get_next_selected`]."]
        pub fn get_selected(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_selected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Selects the specified [`TreeItem`][crate::classes::TreeItem] and column."]
        pub fn set_selected(&mut self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> >, column: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >, i32,);
            let args = (item.into_arg(), column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_selected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the currently focused column, or -1 if no column is focused.\n\nIn [`SelectMode::SINGLE`][`crate::classes::tree::SelectMode::SINGLE`] mode, the focused column is the selected column. In [`SelectMode::ROW`][`crate::classes::tree::SelectMode::ROW`] mode, the focused column is always 0 if any item is selected. In [`SelectMode::MULTI`][`crate::classes::tree::SelectMode::MULTI`] mode, the focused column is the column under the focus cursor, and there are not necessarily any column selected.\n\nTo tell whether a column of an item is selected, use [`is_selected`][`crate::classes::TreeItem::is_selected`]."]
        pub fn get_selected_column(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_selected_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the last pressed button's index."]
        pub fn get_pressed_button(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_pressed_button", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_select_mode(&mut self, mode: crate::classes::tree::SelectMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::tree::SelectMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_select_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_select_mode(&self,) -> crate::classes::tree::SelectMode {
            type CallRet = crate::classes::tree::SelectMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_select_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deselects all tree items (rows and columns). In [`SelectMode::MULTI`][`crate::classes::tree::SelectMode::MULTI`] mode also removes selection cursor."]
        pub fn deselect_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "deselect_all", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_columns(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_columns", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_columns(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_columns", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the currently edited item. Can be used with `item_edited` to get the item that was modified.\n\n\n```gdscript\nfunc _ready():\n\t$Tree.item_edited.connect(on_Tree_item_edited)\n\nfunc on_Tree_item_edited():\n\tprint($Tree.get_edited()) # This item just got edited (e.g. checked).\n```\n"]
        pub fn get_edited(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_edited", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the column for the currently edited item."]
        pub fn get_edited_column(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_edited_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Edits the selected tree item as if it was clicked.\n\nEither the item must be set editable with [`set_editable`][`crate::classes::TreeItem::set_editable`] or `force_edit` must be `true`.\n\nReturns `true` if the item could be edited. Fails if no item is selected."]
        pub(crate) fn edit_selected_full(&mut self, force_edit: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (bool,);
            let args = (force_edit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "edit_selected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`edit_selected_ex`][Self::edit_selected_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Edits the selected tree item as if it was clicked.\n\nEither the item must be set editable with [`set_editable`][`crate::classes::TreeItem::set_editable`] or `force_edit` must be `true`.\n\nReturns `true` if the item could be edited. Fails if no item is selected."]
        #[inline]
        pub fn edit_selected(&mut self,) -> bool {
            self.edit_selected_ex() . done()
        }
        #[doc = "Edits the selected tree item as if it was clicked.\n\nEither the item must be set editable with [`set_editable`][`crate::classes::TreeItem::set_editable`] or `force_edit` must be `true`.\n\nReturns `true` if the item could be edited. Fails if no item is selected."]
        #[inline]
        pub fn edit_selected_ex < 'ex > (&'ex mut self,) -> ExEditSelected < 'ex > {
            ExEditSelected::new(self,)
        }
        #[doc = "Returns the rectangle for custom popups. Helper to create custom cell controls that display a popup. See [`set_cell_mode`][`crate::classes::TreeItem::set_cell_mode`]."]
        pub fn get_custom_popup_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_custom_popup_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the rectangle area for the specified [`TreeItem`][crate::classes::TreeItem]. If `column` is specified, only get the position and size of that column, otherwise get the rectangle containing all columns. If a button index is specified, the rectangle of that button will be returned."]
        pub(crate) fn get_item_area_rect_full(&self, item: CowArg < Option < Gd < crate::classes::TreeItem > > >, column: i32, button_index: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >, i32, i32,);
            let args = (item, column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_item_area_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_item_area_rect_ex`][Self::get_item_area_rect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the rectangle area for the specified [`TreeItem`][crate::classes::TreeItem]. If `column` is specified, only get the position and size of that column, otherwise get the rectangle containing all columns. If a button index is specified, the rectangle of that button will be returned."]
        #[inline]
        pub fn get_item_area_rect(&self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) -> Rect2 {
            self.get_item_area_rect_ex(item,) . done()
        }
        #[doc = "Returns the rectangle area for the specified [`TreeItem`][crate::classes::TreeItem]. If `column` is specified, only get the position and size of that column, otherwise get the rectangle containing all columns. If a button index is specified, the rectangle of that button will be returned."]
        #[inline]
        pub fn get_item_area_rect_ex < 'ex > (&'ex self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> > + 'ex,) -> ExGetItemAreaRect < 'ex > {
            ExGetItemAreaRect::new(self, item,)
        }
        #[doc = "Returns the tree item at the specified position (relative to the tree origin position)."]
        pub fn get_item_at_position(&self, position: Vector2,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_item_at_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the column index at `position`, or -1 if no item is there."]
        pub fn get_column_at_position(&self, position: Vector2,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_at_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the drop section at `position`, or -100 if no item is there.\n\nValues -1, 0, or 1 will be returned for the \"above item\", \"on item\", and \"below item\" drop sections, respectively. See \\[enum DropModeFlags] for a description of each drop section.\n\nTo get the item which the returned drop section is relative to, use [`get_item_at_position`][`crate::classes::Tree::get_item_at_position`]."]
        pub fn get_drop_section_at_position(&self, position: Vector2,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_drop_section_at_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the button ID at `position`, or -1 if no button is there."]
        pub fn get_button_id_at_position(&self, position: Vector2,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_button_id_at_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Makes the currently focused cell visible.\n\nThis will scroll the tree if necessary. In [`SelectMode::ROW`][`crate::classes::tree::SelectMode::ROW`] mode, this will not do horizontal scrolling, as all the cells in the selected row is focused logically.\n\n**Note:** Despite the name of this method, the focus cursor itself is only visible in [`SelectMode::MULTI`][`crate::classes::tree::SelectMode::MULTI`] mode."]
        pub fn ensure_cursor_is_visible(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "ensure_cursor_is_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_column_titles_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_titles_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_column_titles_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "are_column_titles_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the title of a column."]
        pub fn set_column_title(&mut self, column: i32, title: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, title.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_title", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the column's title."]
        pub fn get_column_title(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_title", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the column title's tooltip text."]
        pub fn set_column_title_tooltip_text(&mut self, column: i32, tooltip_text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, tooltip_text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7474usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_title_tooltip_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the column title's tooltip text."]
        pub fn get_column_title_tooltip_text(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_title_tooltip_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the column title alignment. Note that `@GlobalScope.HORIZONTAL_ALIGNMENT_FILL` is not supported for column titles."]
        pub fn set_column_title_alignment(&mut self, column: i32, title_alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (i32, crate::global::HorizontalAlignment,);
            let args = (column, title_alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_title_alignment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the column title alignment."]
        pub fn get_column_title_alignment(&self, column: i32,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_title_alignment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets column title base writing direction."]
        pub fn set_column_title_direction(&mut self, column: i32, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::control::TextDirection,);
            let args = (column, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_title_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns column title base writing direction."]
        pub fn get_column_title_direction(&self, column: i32,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_title_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the language code of the given `column`'s title to `language`. This is used for line-breaking and text shaping algorithms. If `language` is empty, the current locale is used."]
        pub fn set_column_title_language(&mut self, column: i32, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_title_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns column title language code."]
        pub fn get_column_title_language(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_title_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current scrolling position."]
        pub fn get_scroll(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_scroll", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Causes the `Tree` to jump to the specified [`TreeItem`][crate::classes::TreeItem]."]
        pub(crate) fn scroll_to_item_full(&mut self, item: CowArg < Option < Gd < crate::classes::TreeItem > > >, center_on_item: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >, bool,);
            let args = (item, center_on_item,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "scroll_to_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`scroll_to_item_ex`][Self::scroll_to_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Causes the `Tree` to jump to the specified [`TreeItem`][crate::classes::TreeItem]."]
        #[inline]
        pub fn scroll_to_item(&mut self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) {
            self.scroll_to_item_ex(item,) . done()
        }
        #[doc = "Causes the `Tree` to jump to the specified [`TreeItem`][crate::classes::TreeItem]."]
        #[inline]
        pub fn scroll_to_item_ex < 'ex > (&'ex mut self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> > + 'ex,) -> ExScrollToItem < 'ex > {
            ExScrollToItem::new(self, item,)
        }
        pub fn set_h_scroll_enabled(&mut self, h_scroll: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (h_scroll,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7484usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_h_scroll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_h_scroll_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7485usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_h_scroll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_v_scroll_enabled(&mut self, h_scroll: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (h_scroll,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7486usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_v_scroll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_v_scroll_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7487usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_v_scroll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scroll_hint_mode(&mut self, scroll_hint_mode: crate::classes::tree::ScrollHintMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::tree::ScrollHintMode,);
            let args = (scroll_hint_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7488usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_scroll_hint_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scroll_hint_mode(&self,) -> crate::classes::tree::ScrollHintMode {
            type CallRet = crate::classes::tree::ScrollHintMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7489usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_scroll_hint_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tile_scroll_hint(&mut self, tile_scroll_hint: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (tile_scroll_hint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7490usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_tile_scroll_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scroll_hint_tiled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7491usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_scroll_hint_tiled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hide_folding(&mut self, hide: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (hide,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7492usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_hide_folding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_folding_hidden(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7493usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_folding_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_recursive_folding(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7494usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_enable_recursive_folding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_recursive_folding_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7495usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_recursive_folding_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_drag_unfolding(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7496usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_enable_drag_unfolding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drag_unfolding_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7497usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_drag_unfolding_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drop_mode_flags(&mut self, flags: crate::classes::tree::DropModeFlags,) {
            type CallRet = ();
            type CallParams = (crate::classes::tree::DropModeFlags,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7498usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_drop_mode_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drop_mode_flags(&self,) -> crate::classes::tree::DropModeFlags {
            type CallRet = crate::classes::tree::DropModeFlags;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7499usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_drop_mode_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_rmb_select(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7500usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_allow_rmb_select", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_rmb_select(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7501usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_allow_rmb_select", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_reselect(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7502usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_allow_reselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_reselect(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7503usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_allow_reselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_search(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7504usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_allow_search", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_search(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7505usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_allow_search", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_tooltip(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7506usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_auto_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_auto_tooltip_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7507usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_auto_tooltip_enabled", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Tree {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Tree"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Tree {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for Tree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Tree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Tree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Tree {
        
    }
    impl crate::obj::cap::GodotDefault for Tree {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Tree {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Tree {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Tree`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Tree__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Tree > for $Class {
                
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
#[doc = "Default-param extender for [`Tree::create_item_ex`][super::Tree::create_item_ex]."]
#[must_use]
pub struct ExCreateItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Tree, parent: CowArg < 'ex, Option < Gd < crate::classes::TreeItem > > >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::Tree,) -> Self {
        let parent = Gd::null_arg();
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parent: parent.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn parent(self, parent: impl AsArg < Option < Gd < crate::classes::TreeItem >> > + 'ex) -> Self {
        Self {
            parent: parent.into_arg(), .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, parent, index,
        }
        = self;
        re_export::Tree::create_item_full(surround_object, parent, index,)
    }
}
#[doc = "Default-param extender for [`Tree::edit_selected_ex`][super::Tree::edit_selected_ex]."]
#[must_use]
pub struct ExEditSelected < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Tree, force_edit: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExEditSelected < 'ex > {
    fn new(surround_object: &'ex mut re_export::Tree,) -> Self {
        let force_edit = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force_edit: force_edit,
        }
    }
    #[inline]
    pub fn force_edit(self, force_edit: bool) -> Self {
        Self {
            force_edit: force_edit, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, force_edit,
        }
        = self;
        re_export::Tree::edit_selected_full(surround_object, force_edit,)
    }
}
#[doc = "Default-param extender for [`Tree::get_item_area_rect_ex`][super::Tree::get_item_area_rect_ex]."]
#[must_use]
pub struct ExGetItemAreaRect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Tree, item: CowArg < 'ex, Option < Gd < crate::classes::TreeItem > > >, column: i32, button_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetItemAreaRect < 'ex > {
    fn new(surround_object: &'ex re_export::Tree, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> > + 'ex,) -> Self {
        let column = - 1i32;
        let button_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item.into_arg(), column: column, button_index: button_index,
        }
    }
    #[inline]
    pub fn column(self, column: i32) -> Self {
        Self {
            column: column, .. self
        }
    }
    #[inline]
    pub fn button_index(self, button_index: i32) -> Self {
        Self {
            button_index: button_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2 {
        let Self {
            _phantom, surround_object, item, column, button_index,
        }
        = self;
        re_export::Tree::get_item_area_rect_full(surround_object, item, column, button_index,)
    }
}
#[doc = "Default-param extender for [`Tree::scroll_to_item_ex`][super::Tree::scroll_to_item_ex]."]
#[must_use]
pub struct ExScrollToItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Tree, item: CowArg < 'ex, Option < Gd < crate::classes::TreeItem > > >, center_on_item: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExScrollToItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::Tree, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> > + 'ex,) -> Self {
        let center_on_item = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item.into_arg(), center_on_item: center_on_item,
        }
    }
    #[inline]
    pub fn center_on_item(self, center_on_item: bool) -> Self {
        Self {
            center_on_item: center_on_item, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, center_on_item,
        }
        = self;
        re_export::Tree::scroll_to_item_full(surround_object, item, center_on_item,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SelectMode {
    ord: i32
}
impl SelectMode {
    #[doc(alias = "SELECT_SINGLE")]
    #[doc = "Godot enumerator name: `SELECT_SINGLE`"]
    pub const SINGLE: SelectMode = SelectMode {
        ord: 0i32
    };
    #[doc(alias = "SELECT_ROW")]
    #[doc = "Godot enumerator name: `SELECT_ROW`"]
    pub const ROW: SelectMode = SelectMode {
        ord: 1i32
    };
    #[doc(alias = "SELECT_MULTI")]
    #[doc = "Godot enumerator name: `SELECT_MULTI`"]
    pub const MULTI: SelectMode = SelectMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for SelectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SelectMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SelectMode {
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
            Self::SINGLE => "SINGLE", Self::ROW => "ROW", Self::MULTI => "MULTI", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SelectMode::SINGLE, SelectMode::ROW, SelectMode::MULTI]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SelectMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SINGLE", "SELECT_SINGLE", SelectMode::SINGLE), crate::meta::inspect::EnumConstant::new("ROW", "SELECT_ROW", SelectMode::ROW), crate::meta::inspect::EnumConstant::new("MULTI", "SELECT_MULTI", SelectMode::MULTI)]
        }
    }
}
impl crate::meta::GodotConvert for SelectMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Select Single", 0i64), EnumeratorShape::new_int("Select Row", 1i64), EnumeratorShape::new_int("Select Multi", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Tree.SelectMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SelectMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SelectMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SelectMode {
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
impl crate::registry::property::Export for SelectMode {
    
}
impl crate::meta::Element for SelectMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DropModeFlags {
    ord: i32
}
impl DropModeFlags {
    #[doc(alias = "DROP_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `DROP_MODE_DISABLED`"]
    pub const DISABLED: DropModeFlags = DropModeFlags {
        ord: 0i32
    };
    #[doc(alias = "DROP_MODE_ON_ITEM")]
    #[doc = "Godot enumerator name: `DROP_MODE_ON_ITEM`"]
    pub const ON_ITEM: DropModeFlags = DropModeFlags {
        ord: 1i32
    };
    #[doc(alias = "DROP_MODE_INBETWEEN")]
    #[doc = "Godot enumerator name: `DROP_MODE_INBETWEEN`"]
    pub const INBETWEEN: DropModeFlags = DropModeFlags {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DropModeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DropModeFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DropModeFlags {
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
            Self::DISABLED => "DISABLED", Self::ON_ITEM => "ON_ITEM", Self::INBETWEEN => "INBETWEEN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DropModeFlags::DISABLED, DropModeFlags::ON_ITEM, DropModeFlags::INBETWEEN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DropModeFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "DROP_MODE_DISABLED", DropModeFlags::DISABLED), crate::meta::inspect::EnumConstant::new("ON_ITEM", "DROP_MODE_ON_ITEM", DropModeFlags::ON_ITEM), crate::meta::inspect::EnumConstant::new("INBETWEEN", "DROP_MODE_INBETWEEN", DropModeFlags::INBETWEEN)]
        }
    }
}
impl crate::meta::GodotConvert for DropModeFlags {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Drop Mode Disabled", 0i64), EnumeratorShape::new_int("Drop Mode On Item", 1i64), EnumeratorShape::new_int("Drop Mode Inbetween", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Tree.DropModeFlags")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DropModeFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DropModeFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DropModeFlags {
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
impl crate::registry::property::Export for DropModeFlags {
    
}
impl crate::meta::Element for DropModeFlags {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ScrollHintMode {
    ord: i32
}
impl ScrollHintMode {
    #[doc(alias = "SCROLL_HINT_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_MODE_DISABLED`"]
    pub const DISABLED: ScrollHintMode = ScrollHintMode {
        ord: 0i32
    };
    #[doc(alias = "SCROLL_HINT_MODE_BOTH")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_MODE_BOTH`"]
    pub const BOTH: ScrollHintMode = ScrollHintMode {
        ord: 1i32
    };
    #[doc(alias = "SCROLL_HINT_MODE_TOP")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_MODE_TOP`"]
    pub const TOP: ScrollHintMode = ScrollHintMode {
        ord: 2i32
    };
    #[doc(alias = "SCROLL_HINT_MODE_BOTTOM")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_MODE_BOTTOM`"]
    pub const BOTTOM: ScrollHintMode = ScrollHintMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ScrollHintMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ScrollHintMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ScrollHintMode {
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
            Self::DISABLED => "DISABLED", Self::BOTH => "BOTH", Self::TOP => "TOP", Self::BOTTOM => "BOTTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ScrollHintMode::DISABLED, ScrollHintMode::BOTH, ScrollHintMode::TOP, ScrollHintMode::BOTTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ScrollHintMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "SCROLL_HINT_MODE_DISABLED", ScrollHintMode::DISABLED), crate::meta::inspect::EnumConstant::new("BOTH", "SCROLL_HINT_MODE_BOTH", ScrollHintMode::BOTH), crate::meta::inspect::EnumConstant::new("TOP", "SCROLL_HINT_MODE_TOP", ScrollHintMode::TOP), crate::meta::inspect::EnumConstant::new("BOTTOM", "SCROLL_HINT_MODE_BOTTOM", ScrollHintMode::BOTTOM)]
        }
    }
}
impl crate::meta::GodotConvert for ScrollHintMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Scroll Hint Mode Disabled", 0i64), EnumeratorShape::new_int("Scroll Hint Mode Both", 1i64), EnumeratorShape::new_int("Scroll Hint Mode Top", 2i64), EnumeratorShape::new_int("Scroll Hint Mode Bottom", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Tree.ScrollHintMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ScrollHintMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ScrollHintMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ScrollHintMode {
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
impl crate::registry::property::Export for ScrollHintMode {
    
}
impl crate::meta::Element for ScrollHintMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Tree;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Tree`][crate::classes::Tree] class."]
    pub struct SignalsOfTree < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTree < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn item_selected(&mut self) -> SigItemSelected < 'c, C > {
            SigItemSelected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "item_selected")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn cell_selected(&mut self) -> SigCellSelected < 'c, C > {
            SigCellSelected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "cell_selected")
            }
        }
        #[doc = "Signature: `(item: Gd<TreeItem>, column: i64, selected: bool)`"]
        pub fn multi_selected(&mut self) -> SigMultiSelected < 'c, C > {
            SigMultiSelected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "multi_selected")
            }
        }
        #[doc = "Signature: `(mouse_position: Vector2, mouse_button_index: i64)`"]
        pub fn item_mouse_selected(&mut self) -> SigItemMouseSelected < 'c, C > {
            SigItemMouseSelected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "item_mouse_selected")
            }
        }
        #[doc = "Signature: `(click_position: Vector2, mouse_button_index: i64)`"]
        pub fn empty_clicked(&mut self) -> SigEmptyClicked < 'c, C > {
            SigEmptyClicked {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "empty_clicked")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn item_edited(&mut self) -> SigItemEdited < 'c, C > {
            SigItemEdited {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "item_edited")
            }
        }
        #[doc = "Signature: `(mouse_button_index: i64)`"]
        pub fn custom_item_clicked(&mut self) -> SigCustomItemClicked < 'c, C > {
            SigCustomItemClicked {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "custom_item_clicked")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn item_icon_double_clicked(&mut self) -> SigItemIconDoubleClicked < 'c, C > {
            SigItemIconDoubleClicked {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "item_icon_double_clicked")
            }
        }
        #[doc = "Signature: `(item: Gd<TreeItem>)`"]
        pub fn item_collapsed(&mut self) -> SigItemCollapsed < 'c, C > {
            SigItemCollapsed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "item_collapsed")
            }
        }
        #[doc = "Signature: `(item: Gd<TreeItem>, column: i64)`"]
        pub fn check_propagated_to_item(&mut self) -> SigCheckPropagatedToItem < 'c, C > {
            SigCheckPropagatedToItem {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "check_propagated_to_item")
            }
        }
        #[doc = "Signature: `(item: Gd<TreeItem>, column: i64, id: i64, mouse_button_index: i64)`"]
        pub fn button_clicked(&mut self) -> SigButtonClicked < 'c, C > {
            SigButtonClicked {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "button_clicked")
            }
        }
        #[doc = "Signature: `(arrow_clicked: bool)`"]
        pub fn custom_popup_edited(&mut self) -> SigCustomPopupEdited < 'c, C > {
            SigCustomPopupEdited {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "custom_popup_edited")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn item_activated(&mut self) -> SigItemActivated < 'c, C > {
            SigItemActivated {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "item_activated")
            }
        }
        #[doc = "Signature: `(column: i64, mouse_button_index: i64)`"]
        pub fn column_title_clicked(&mut self) -> SigColumnTitleClicked < 'c, C > {
            SigColumnTitleClicked {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "column_title_clicked")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn nothing_selected(&mut self) -> SigNothingSelected < 'c, C > {
            SigNothingSelected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "nothing_selected")
            }
        }
    }
    type TypedSigItemSelected < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigItemSelected < 'c, C: WithSignals > {
        typed: TypedSigItemSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemSelected < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemSelected < 'c, C > {
        type Target = TypedSigItemSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCellSelected < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigCellSelected < 'c, C: WithSignals > {
        typed: TypedSigCellSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCellSelected < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCellSelected < 'c, C > {
        type Target = TypedSigCellSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCellSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMultiSelected < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::TreeItem >, i64, bool,) >;
    pub struct SigMultiSelected < 'c, C: WithSignals > {
        typed: TypedSigMultiSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMultiSelected < 'c, C > {
        pub fn emit(&mut self, item: Gd < crate::classes::TreeItem >, column: i64, selected: bool,) {
            self.typed.emit_tuple((item, column, selected,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMultiSelected < 'c, C > {
        type Target = TypedSigMultiSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMultiSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemMouseSelected < 'c, C > = TypedSignal < 'c, C, (Vector2, i64,) >;
    pub struct SigItemMouseSelected < 'c, C: WithSignals > {
        typed: TypedSigItemMouseSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemMouseSelected < 'c, C > {
        pub fn emit(&mut self, mouse_position: Vector2, mouse_button_index: i64,) {
            self.typed.emit_tuple((mouse_position, mouse_button_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemMouseSelected < 'c, C > {
        type Target = TypedSigItemMouseSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemMouseSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigEmptyClicked < 'c, C > = TypedSignal < 'c, C, (Vector2, i64,) >;
    pub struct SigEmptyClicked < 'c, C: WithSignals > {
        typed: TypedSigEmptyClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEmptyClicked < 'c, C > {
        pub fn emit(&mut self, click_position: Vector2, mouse_button_index: i64,) {
            self.typed.emit_tuple((click_position, mouse_button_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigEmptyClicked < 'c, C > {
        type Target = TypedSigEmptyClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigEmptyClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemEdited < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigItemEdited < 'c, C: WithSignals > {
        typed: TypedSigItemEdited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemEdited < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemEdited < 'c, C > {
        type Target = TypedSigItemEdited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemEdited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCustomItemClicked < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigCustomItemClicked < 'c, C: WithSignals > {
        typed: TypedSigCustomItemClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCustomItemClicked < 'c, C > {
        pub fn emit(&mut self, mouse_button_index: i64,) {
            self.typed.emit_tuple((mouse_button_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCustomItemClicked < 'c, C > {
        type Target = TypedSigCustomItemClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCustomItemClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemIconDoubleClicked < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigItemIconDoubleClicked < 'c, C: WithSignals > {
        typed: TypedSigItemIconDoubleClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemIconDoubleClicked < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemIconDoubleClicked < 'c, C > {
        type Target = TypedSigItemIconDoubleClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemIconDoubleClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemCollapsed < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::TreeItem >,) >;
    pub struct SigItemCollapsed < 'c, C: WithSignals > {
        typed: TypedSigItemCollapsed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemCollapsed < 'c, C > {
        pub fn emit(&mut self, item: Gd < crate::classes::TreeItem >,) {
            self.typed.emit_tuple((item,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemCollapsed < 'c, C > {
        type Target = TypedSigItemCollapsed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemCollapsed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCheckPropagatedToItem < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::TreeItem >, i64,) >;
    pub struct SigCheckPropagatedToItem < 'c, C: WithSignals > {
        typed: TypedSigCheckPropagatedToItem < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCheckPropagatedToItem < 'c, C > {
        pub fn emit(&mut self, item: Gd < crate::classes::TreeItem >, column: i64,) {
            self.typed.emit_tuple((item, column,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCheckPropagatedToItem < 'c, C > {
        type Target = TypedSigCheckPropagatedToItem < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCheckPropagatedToItem < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigButtonClicked < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::TreeItem >, i64, i64, i64,) >;
    pub struct SigButtonClicked < 'c, C: WithSignals > {
        typed: TypedSigButtonClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigButtonClicked < 'c, C > {
        pub fn emit(&mut self, item: Gd < crate::classes::TreeItem >, column: i64, id: i64, mouse_button_index: i64,) {
            self.typed.emit_tuple((item, column, id, mouse_button_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigButtonClicked < 'c, C > {
        type Target = TypedSigButtonClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigButtonClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCustomPopupEdited < 'c, C > = TypedSignal < 'c, C, (bool,) >;
    pub struct SigCustomPopupEdited < 'c, C: WithSignals > {
        typed: TypedSigCustomPopupEdited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCustomPopupEdited < 'c, C > {
        pub fn emit(&mut self, arrow_clicked: bool,) {
            self.typed.emit_tuple((arrow_clicked,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCustomPopupEdited < 'c, C > {
        type Target = TypedSigCustomPopupEdited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCustomPopupEdited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemActivated < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigItemActivated < 'c, C: WithSignals > {
        typed: TypedSigItemActivated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemActivated < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemActivated < 'c, C > {
        type Target = TypedSigItemActivated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemActivated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigColumnTitleClicked < 'c, C > = TypedSignal < 'c, C, (i64, i64,) >;
    pub struct SigColumnTitleClicked < 'c, C: WithSignals > {
        typed: TypedSigColumnTitleClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigColumnTitleClicked < 'c, C > {
        pub fn emit(&mut self, column: i64, mouse_button_index: i64,) {
            self.typed.emit_tuple((column, mouse_button_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigColumnTitleClicked < 'c, C > {
        type Target = TypedSigColumnTitleClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigColumnTitleClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigNothingSelected < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigNothingSelected < 'c, C: WithSignals > {
        typed: TypedSigNothingSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigNothingSelected < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigNothingSelected < 'c, C > {
        type Target = TypedSigNothingSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigNothingSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Tree {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTree < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTree < 'c, C > {
        type Target = < < Tree as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Tree;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTree < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Tree;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
#![doc = "Sidecar module for class [`ItemList`][crate::classes::ItemList].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ItemList` enums](https://docs.godotengine.org/en/stable/classes/class_itemlist.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ItemList`.\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`item_list`][crate::classes::item_list]: sidecar module with related enum/flag types\n* [`IItemList`][crate::classes::IItemList]: virtual methods\n* [`SignalsOfItemList`][crate::classes::item_list::SignalsOfItemList]: signal collection\n\n\nSee also [Godot docs for `ItemList`](https://docs.godotengine.org/en/stable/classes/class_itemlist.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ItemList::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nThis control provides a vertical list of selectable items that may be in a single or in multiple columns, with each item having options for text and an icon. Tooltips are supported and may be different for every item in the list.\n\nSelectable items in the list may be selected or deselected and multiple selection may be enabled. Selection with right mouse button may also be enabled to allow use of popup context menus. Items may also be \"activated\" by double-clicking them or by pressing `Enter`.\n\nItem text only supports single-line strings. Newline characters (e.g. `\\n`) in the string won't produce a newline. Text wrapping is enabled in [`IconMode::TOP`][`crate::classes::item_list::IconMode::TOP`] mode, but the column's width is adjusted to fully fit its content by default. You need to set \\[member fixed_column_width] greater than zero to wrap the text.\n\nAll `set_*` methods allow negative item indices, i.e. `-1` to access the last item, `-2` to select the second-to-last item, and so on.\n\n**Incremental search:** Like [`PopupMenu`][crate::classes::PopupMenu] and [`Tree`][crate::classes::Tree], `ItemList` supports searching within the list while the control is focused. Press a key that matches the first letter of an item's name to select the first item starting with the given letter. After that point, there are two ways to perform incremental search: 1) Press the same key again before the timeout duration to select the next item starting with the same letter. 2) Press letter keys that match the rest of the word before the timeout duration to match to select the item in question directly. Both of these actions will be reset to the beginning of the list if the timeout duration has passed since the last keystroke was registered. You can adjust the timeout duration by changing \\[member ProjectSettings.gui/timers/incremental_search_max_interval_msec]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ItemList {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ItemList`][crate::classes::ItemList].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `ItemList` methods](https://docs.godotengine.org/en/stable/classes/class_itemlist.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IItemList: crate::obj::GodotClass < Base = ItemList > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ItemList {
        #[doc = "Adds an item to the item list with specified text. Returns the index of an added item.\n\nSpecify an `icon`, or use `null` as the `icon` for a list item with no icon.\n\nIf `selectable` is `true`, the list item will be selectable."]
        pub(crate) fn add_item_full(&mut self, text: CowArg < GString >, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, selectable: bool,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, bool,);
            let args = (text, icon, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9563usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "add_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_item_ex`][Self::add_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an item to the item list with specified text. Returns the index of an added item.\n\nSpecify an `icon`, or use `null` as the `icon` for a list item with no icon.\n\nIf `selectable` is `true`, the list item will be selectable."]
        #[inline]
        pub fn add_item(&mut self, text: impl AsArg < GString >,) -> i32 {
            self.add_item_ex(text,) . done()
        }
        #[doc = "Adds an item to the item list with specified text. Returns the index of an added item.\n\nSpecify an `icon`, or use `null` as the `icon` for a list item with no icon.\n\nIf `selectable` is `true`, the list item will be selectable."]
        #[inline]
        pub fn add_item_ex < 'ex > (&'ex mut self, text: impl AsArg < GString > + 'ex,) -> ExAddItem < 'ex > {
            ExAddItem::new(self, text,)
        }
        #[doc = "Adds an item to the item list with no text, only an icon. Returns the index of an added item."]
        pub(crate) fn add_icon_item_full(&mut self, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, selectable: bool,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, bool,);
            let args = (icon, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9564usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "add_icon_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_icon_item_ex`][Self::add_icon_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an item to the item list with no text, only an icon. Returns the index of an added item."]
        #[inline]
        pub fn add_icon_item(&mut self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) -> i32 {
            self.add_icon_item_ex(icon,) . done()
        }
        #[doc = "Adds an item to the item list with no text, only an icon. Returns the index of an added item."]
        #[inline]
        pub fn add_icon_item_ex < 'ex > (&'ex mut self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> ExAddIconItem < 'ex > {
            ExAddIconItem::new(self, icon,)
        }
        #[doc = "Sets text of the item associated with the specified index."]
        pub fn set_item_text(&mut self, idx: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (idx, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9565usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text associated with the specified index."]
        pub fn get_item_text(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9566usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets (or replaces) the icon's [`Texture2D`][crate::classes::Texture2D] associated with the specified index."]
        pub fn set_item_icon(&mut self, idx: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (idx, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9567usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the icon associated with the specified index."]
        pub fn get_item_icon(&self, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9568usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets item's text base writing direction."]
        pub fn set_item_text_direction(&mut self, idx: i32, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::control::TextDirection,);
            let args = (idx, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9569usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns item's text base writing direction."]
        pub fn get_item_text_direction(&self, idx: i32,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9570usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the language code of the text for the item at the given index to `language`. This is used for line-breaking and text shaping algorithms. If `language` is empty, the current locale is used."]
        pub fn set_item_language(&mut self, idx: i32, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (idx, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns item's text language code."]
        pub fn get_item_language(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the auto translate mode of the item associated with the specified index.\n\nItems use [`AutoTranslateMode::INHERIT`][`crate::classes::node::AutoTranslateMode::INHERIT`] by default, which uses the same auto translate mode as the `ItemList` itself."]
        pub fn set_item_auto_translate_mode(&mut self, idx: i32, mode: crate::classes::node::AutoTranslateMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::node::AutoTranslateMode,);
            let args = (idx, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9573usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns item's auto translate mode."]
        pub fn get_item_auto_translate_mode(&self, idx: i32,) -> crate::classes::node::AutoTranslateMode {
            type CallRet = crate::classes::node::AutoTranslateMode;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9574usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether the item icon will be drawn transposed."]
        pub fn set_item_icon_transposed(&mut self, idx: i32, transposed: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (idx, transposed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9575usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_icon_transposed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item icon will be drawn transposed, i.e. the X and Y axes are swapped."]
        pub fn is_item_icon_transposed(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9576usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_item_icon_transposed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the region of item's icon used. The whole icon will be used if the region has no area."]
        pub fn set_item_icon_region(&mut self, idx: i32, rect: Rect2,) {
            type CallRet = ();
            type CallParams = (i32, Rect2,);
            let args = (idx, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9577usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_icon_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the region of item's icon used. The whole icon will be used if the region has no area."]
        pub fn get_item_icon_region(&self, idx: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9578usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_icon_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a modulating [`Color`][crate::builtin::Color] of the item associated with the specified index."]
        pub fn set_item_icon_modulate(&mut self, idx: i32, modulate: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (idx, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9579usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_icon_modulate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Color`][crate::builtin::Color] modulating item's icon at the specified index."]
        pub fn get_item_icon_modulate(&self, idx: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9580usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_icon_modulate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Allows or disallows selection of the item associated with the specified index."]
        pub fn set_item_selectable(&mut self, idx: i32, selectable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (idx, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_selectable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at the specified index is selectable."]
        pub fn is_item_selectable(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_item_selectable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Disables (or enables) the item at the specified index.\n\nDisabled items cannot be selected and do not trigger activation signals (when double-clicking or pressing `Enter`)."]
        pub fn set_item_disabled(&mut self, idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at the specified index is disabled."]
        pub fn is_item_disabled(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a value (of any type) to be stored with the item associated with the specified index."]
        pub fn set_item_metadata(&mut self, idx: i32, metadata: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (idx, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the metadata value of the specified index."]
        pub fn get_item_metadata(&self, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the background color of the item specified by `idx` index to the specified [`Color`][crate::builtin::Color]."]
        pub fn set_item_custom_bg_color(&mut self, idx: i32, custom_bg_color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (idx, custom_bg_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_custom_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the custom background color of the item specified by `idx` index."]
        pub fn get_item_custom_bg_color(&self, idx: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_custom_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the foreground color of the item specified by `idx` index to the specified [`Color`][crate::builtin::Color]."]
        pub fn set_item_custom_fg_color(&mut self, idx: i32, custom_fg_color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (idx, custom_fg_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_custom_fg_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the custom foreground color of the item specified by `idx` index."]
        pub fn get_item_custom_fg_color(&self, idx: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_custom_fg_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the position and size of the item with the specified index, in the coordinate system of the `ItemList` node. If `expand` is `true` the last column expands to fill the rest of the row.\n\n**Note:** The returned value is unreliable if called right after modifying the `ItemList`, before it redraws in the next frame."]
        pub(crate) fn get_item_rect_full(&self, idx: i32, expand: bool,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32, bool,);
            let args = (idx, expand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_item_rect_ex`][Self::get_item_rect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the position and size of the item with the specified index, in the coordinate system of the `ItemList` node. If `expand` is `true` the last column expands to fill the rest of the row.\n\n**Note:** The returned value is unreliable if called right after modifying the `ItemList`, before it redraws in the next frame."]
        #[inline]
        pub fn get_item_rect(&self, idx: i32,) -> Rect2 {
            self.get_item_rect_ex(idx,) . done()
        }
        #[doc = "Returns the position and size of the item with the specified index, in the coordinate system of the `ItemList` node. If `expand` is `true` the last column expands to fill the rest of the row.\n\n**Note:** The returned value is unreliable if called right after modifying the `ItemList`, before it redraws in the next frame."]
        #[inline]
        pub fn get_item_rect_ex < 'ex > (&'ex self, idx: i32,) -> ExGetItemRect < 'ex > {
            ExGetItemRect::new(self, idx,)
        }
        #[doc = "Sets whether the tooltip hint is enabled for specified item index."]
        pub fn set_item_tooltip_enabled(&mut self, idx: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (idx, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_tooltip_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the tooltip is enabled for specified item index."]
        pub fn is_item_tooltip_enabled(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_item_tooltip_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the tooltip hint for the item associated with the specified index."]
        pub fn set_item_tooltip(&mut self, idx: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tooltip hint associated with the specified index."]
        pub fn get_item_tooltip(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9595usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Select the item at the specified index.\n\n**Note:** This method does not trigger the item selection signal."]
        pub(crate) fn select_full(&mut self, idx: i32, single: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (idx, single,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9596usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "select", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`select_ex`][Self::select_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Select the item at the specified index.\n\n**Note:** This method does not trigger the item selection signal."]
        #[inline]
        pub fn select(&mut self, idx: i32,) {
            self.select_ex(idx,) . done()
        }
        #[doc = "Select the item at the specified index.\n\n**Note:** This method does not trigger the item selection signal."]
        #[inline]
        pub fn select_ex < 'ex > (&'ex mut self, idx: i32,) -> ExSelect < 'ex > {
            ExSelect::new(self, idx,)
        }
        #[doc = "Ensures the item associated with the specified index is not selected."]
        pub fn deselect(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9597usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "deselect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Ensures there are no items selected."]
        pub fn deselect_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9598usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "deselect_all", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at the specified index is currently selected."]
        pub fn is_selected(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9599usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_selected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array with the indexes of the selected items."]
        pub fn get_selected_items(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9600usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_selected_items", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves item from index `from_idx` to `to_idx`."]
        pub fn move_item(&mut self, from_idx: i32, to_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_idx, to_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9601usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "move_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9602usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9603usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the item specified by `idx` index from the list."]
        pub fn remove_item(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9604usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "remove_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all items from the list."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9605usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sorts items in the list by their text."]
        pub fn sort_items_by_text(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "sort_items_by_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fixed_column_width(&mut self, width: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_fixed_column_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fixed_column_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_fixed_column_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_same_column_width(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_same_column_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_same_column_width(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_same_column_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_text_lines(&mut self, lines: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (lines,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_max_text_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_text_lines(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_max_text_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_columns(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_max_columns", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_columns(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_max_columns", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_select_mode(&mut self, mode: crate::classes::item_list::SelectMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::item_list::SelectMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9615usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_select_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_select_mode(&self,) -> crate::classes::item_list::SelectMode {
            type CallRet = crate::classes::item_list::SelectMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9616usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_select_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_icon_mode(&mut self, mode: crate::classes::item_list::IconMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::item_list::IconMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9617usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_icon_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon_mode(&self,) -> crate::classes::item_list::IconMode {
            type CallRet = crate::classes::item_list::IconMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9618usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_icon_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fixed_icon_size(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9619usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_fixed_icon_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fixed_icon_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9620usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_fixed_icon_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_icon_scale(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9621usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_icon_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9622usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_icon_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_rmb_select(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9623usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_allow_rmb_select", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_rmb_select(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9624usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_allow_rmb_select", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_reselect(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_allow_reselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_reselect(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_allow_reselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_search(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_allow_search", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_search(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_allow_search", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_width(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_auto_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_auto_width(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "has_auto_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_height(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_auto_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_auto_height(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "has_auto_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if one or more items are selected."]
        pub fn is_anything_selected(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9633usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_anything_selected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the item index at the given `position`.\n\nWhen there is no item at that point, -1 will be returned if `exact` is `true`, and the closest item index will be returned otherwise.\n\n**Note:** The returned value is unreliable if called right after modifying the `ItemList`, before it redraws in the next frame."]
        pub(crate) fn get_item_at_position_full(&self, position: Vector2, exact: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2, bool,);
            let args = (position, exact,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9634usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_at_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_item_at_position_ex`][Self::get_item_at_position_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the item index at the given `position`.\n\nWhen there is no item at that point, -1 will be returned if `exact` is `true`, and the closest item index will be returned otherwise.\n\n**Note:** The returned value is unreliable if called right after modifying the `ItemList`, before it redraws in the next frame."]
        #[inline]
        pub fn get_item_at_position(&self, position: Vector2,) -> i32 {
            self.get_item_at_position_ex(position,) . done()
        }
        #[doc = "Returns the item index at the given `position`.\n\nWhen there is no item at that point, -1 will be returned if `exact` is `true`, and the closest item index will be returned otherwise.\n\n**Note:** The returned value is unreliable if called right after modifying the `ItemList`, before it redraws in the next frame."]
        #[inline]
        pub fn get_item_at_position_ex < 'ex > (&'ex self, position: Vector2,) -> ExGetItemAtPosition < 'ex > {
            ExGetItemAtPosition::new(self, position,)
        }
        #[doc = "Ensure current selection is visible, adjusting the scroll position as necessary."]
        pub fn ensure_current_is_visible(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "ensure_current_is_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the vertical scrollbar.\n\n**Warning:** This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their \\[member CanvasItem.visible] property."]
        pub fn get_v_scroll_bar(&self,) -> Option < Gd < crate::classes::VScrollBar > > {
            type CallRet = Option < Gd < crate::classes::VScrollBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_v_scroll_bar", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the horizontal scrollbar.\n\n**Warning:** This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their \\[member CanvasItem.visible] property."]
        pub fn get_h_scroll_bar(&self,) -> Option < Gd < crate::classes::HScrollBar > > {
            type CallRet = Option < Gd < crate::classes::HScrollBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_h_scroll_bar", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scroll_hint_mode(&mut self, scroll_hint_mode: crate::classes::item_list::ScrollHintMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::item_list::ScrollHintMode,);
            let args = (scroll_hint_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_scroll_hint_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scroll_hint_mode(&self,) -> crate::classes::item_list::ScrollHintMode {
            type CallRet = crate::classes::item_list::ScrollHintMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_scroll_hint_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tile_scroll_hint(&mut self, tile_scroll_hint: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (tile_scroll_hint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_tile_scroll_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scroll_hint_tiled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_scroll_hint_tiled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::OverrunBehavior,);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::classes::text_server::OverrunBehavior {
            type CallRet = crate::classes::text_server::OverrunBehavior;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_wraparound_items(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_wraparound_items", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_wraparound_items(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "has_wraparound_items", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Forces an update to the list size based on its items. This happens automatically whenever size of the items, or other relevant settings like \\[member auto_height], change. The method can be used to trigger the update ahead of next drawing pass."]
        pub fn force_update_list_size(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "force_update_list_size", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ItemList {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ItemList"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ItemList {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for ItemList {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for ItemList {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ItemList {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ItemList {
        
    }
    impl crate::obj::cap::GodotDefault for ItemList {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ItemList {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ItemList {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ItemList`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ItemList__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ItemList > for $Class {
                
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
#[doc = "Default-param extender for [`ItemList::add_item_ex`][super::ItemList::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ItemList, text: CowArg < 'ex, GString >, icon: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, selectable: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::ItemList, text: impl AsArg < GString > + 'ex,) -> Self {
        let icon = Gd::null_arg();
        let selectable = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), icon: icon.into_arg(), selectable: selectable,
        }
    }
    #[inline]
    pub fn icon(self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex) -> Self {
        Self {
            icon: icon.into_arg(), .. self
        }
    }
    #[inline]
    pub fn selectable(self, selectable: bool) -> Self {
        Self {
            selectable: selectable, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, text, icon, selectable,
        }
        = self;
        re_export::ItemList::add_item_full(surround_object, text, icon, selectable,)
    }
}
#[doc = "Default-param extender for [`ItemList::add_icon_item_ex`][super::ItemList::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ItemList, icon: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, selectable: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddIconItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::ItemList, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> Self {
        let selectable = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, icon: icon.into_arg(), selectable: selectable,
        }
    }
    #[inline]
    pub fn selectable(self, selectable: bool) -> Self {
        Self {
            selectable: selectable, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, icon, selectable,
        }
        = self;
        re_export::ItemList::add_icon_item_full(surround_object, icon, selectable,)
    }
}
#[doc = "Default-param extender for [`ItemList::get_item_rect_ex`][super::ItemList::get_item_rect_ex]."]
#[must_use]
pub struct ExGetItemRect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::ItemList, idx: i32, expand: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetItemRect < 'ex > {
    fn new(surround_object: &'ex re_export::ItemList, idx: i32,) -> Self {
        let expand = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx, expand: expand,
        }
    }
    #[inline]
    pub fn expand(self, expand: bool) -> Self {
        Self {
            expand: expand, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2 {
        let Self {
            _phantom, surround_object, idx, expand,
        }
        = self;
        re_export::ItemList::get_item_rect_full(surround_object, idx, expand,)
    }
}
#[doc = "Default-param extender for [`ItemList::select_ex`][super::ItemList::select_ex]."]
#[must_use]
pub struct ExSelect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ItemList, idx: i32, single: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSelect < 'ex > {
    fn new(surround_object: &'ex mut re_export::ItemList, idx: i32,) -> Self {
        let single = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx, single: single,
        }
    }
    #[inline]
    pub fn single(self, single: bool) -> Self {
        Self {
            single: single, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, idx, single,
        }
        = self;
        re_export::ItemList::select_full(surround_object, idx, single,)
    }
}
#[doc = "Default-param extender for [`ItemList::get_item_at_position_ex`][super::ItemList::get_item_at_position_ex]."]
#[must_use]
pub struct ExGetItemAtPosition < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::ItemList, position: Vector2, exact: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetItemAtPosition < 'ex > {
    fn new(surround_object: &'ex re_export::ItemList, position: Vector2,) -> Self {
        let exact = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, exact: exact,
        }
    }
    #[inline]
    pub fn exact(self, exact: bool) -> Self {
        Self {
            exact: exact, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, position, exact,
        }
        = self;
        re_export::ItemList::get_item_at_position_full(surround_object, position, exact,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct IconMode {
    ord: i32
}
impl IconMode {
    #[doc(alias = "ICON_MODE_TOP")]
    #[doc = "Godot enumerator name: `ICON_MODE_TOP`"]
    pub const TOP: IconMode = IconMode {
        ord: 0i32
    };
    #[doc(alias = "ICON_MODE_LEFT")]
    #[doc = "Godot enumerator name: `ICON_MODE_LEFT`"]
    pub const LEFT: IconMode = IconMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for IconMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("IconMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for IconMode {
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
            Self::TOP => "TOP", Self::LEFT => "LEFT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[IconMode::TOP, IconMode::LEFT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < IconMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOP", "ICON_MODE_TOP", IconMode::TOP), crate::meta::inspect::EnumConstant::new("LEFT", "ICON_MODE_LEFT", IconMode::LEFT)]
        }
    }
}
impl crate::meta::GodotConvert for IconMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Icon Mode Top", 0i64), EnumeratorShape::new_int("Icon Mode Left", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("ItemList.IconMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for IconMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for IconMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for IconMode {
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
impl crate::registry::property::Export for IconMode {
    
}
impl crate::meta::Element for IconMode {
    
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
    #[doc(alias = "SELECT_MULTI")]
    #[doc = "Godot enumerator name: `SELECT_MULTI`"]
    pub const MULTI: SelectMode = SelectMode {
        ord: 1i32
    };
    #[doc(alias = "SELECT_TOGGLE")]
    #[doc = "Godot enumerator name: `SELECT_TOGGLE`"]
    pub const TOGGLE: SelectMode = SelectMode {
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
            Self::SINGLE => "SINGLE", Self::MULTI => "MULTI", Self::TOGGLE => "TOGGLE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SelectMode::SINGLE, SelectMode::MULTI, SelectMode::TOGGLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SelectMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SINGLE", "SELECT_SINGLE", SelectMode::SINGLE), crate::meta::inspect::EnumConstant::new("MULTI", "SELECT_MULTI", SelectMode::MULTI), crate::meta::inspect::EnumConstant::new("TOGGLE", "SELECT_TOGGLE", SelectMode::TOGGLE)]
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
            &[EnumeratorShape::new_int("Select Single", 0i64), EnumeratorShape::new_int("Select Multi", 1i64), EnumeratorShape::new_int("Select Toggle", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("ItemList.SelectMode")), is_bitfield: false,
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("ItemList.ScrollHintMode")), is_bitfield: false,
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
    use super::re_export::ItemList;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`ItemList`][crate::classes::ItemList] class."]
    pub struct SignalsOfItemList < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfItemList < 'c, C > {
        #[doc = "Signature: `(index: i64)`"]
        pub fn item_selected(&mut self) -> SigItemSelected < 'c, C > {
            SigItemSelected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "item_selected")
            }
        }
        #[doc = "Signature: `(at_position: Vector2, mouse_button_index: i64)`"]
        pub fn empty_clicked(&mut self) -> SigEmptyClicked < 'c, C > {
            SigEmptyClicked {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "empty_clicked")
            }
        }
        #[doc = "Signature: `(index: i64, at_position: Vector2, mouse_button_index: i64)`"]
        pub fn item_clicked(&mut self) -> SigItemClicked < 'c, C > {
            SigItemClicked {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "item_clicked")
            }
        }
        #[doc = "Signature: `(index: i64, selected: bool)`"]
        pub fn multi_selected(&mut self) -> SigMultiSelected < 'c, C > {
            SigMultiSelected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "multi_selected")
            }
        }
        #[doc = "Signature: `(index: i64)`"]
        pub fn item_activated(&mut self) -> SigItemActivated < 'c, C > {
            SigItemActivated {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "item_activated")
            }
        }
    }
    type TypedSigItemSelected < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigItemSelected < 'c, C: WithSignals > {
        typed: TypedSigItemSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemSelected < 'c, C > {
        pub fn emit(&mut self, index: i64,) {
            self.typed.emit_tuple((index,));
            
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
    type TypedSigEmptyClicked < 'c, C > = TypedSignal < 'c, C, (Vector2, i64,) >;
    pub struct SigEmptyClicked < 'c, C: WithSignals > {
        typed: TypedSigEmptyClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEmptyClicked < 'c, C > {
        pub fn emit(&mut self, at_position: Vector2, mouse_button_index: i64,) {
            self.typed.emit_tuple((at_position, mouse_button_index,));
            
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
    type TypedSigItemClicked < 'c, C > = TypedSignal < 'c, C, (i64, Vector2, i64,) >;
    pub struct SigItemClicked < 'c, C: WithSignals > {
        typed: TypedSigItemClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemClicked < 'c, C > {
        pub fn emit(&mut self, index: i64, at_position: Vector2, mouse_button_index: i64,) {
            self.typed.emit_tuple((index, at_position, mouse_button_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemClicked < 'c, C > {
        type Target = TypedSigItemClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMultiSelected < 'c, C > = TypedSignal < 'c, C, (i64, bool,) >;
    pub struct SigMultiSelected < 'c, C: WithSignals > {
        typed: TypedSigMultiSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMultiSelected < 'c, C > {
        pub fn emit(&mut self, index: i64, selected: bool,) {
            self.typed.emit_tuple((index, selected,));
            
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
    type TypedSigItemActivated < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigItemActivated < 'c, C: WithSignals > {
        typed: TypedSigItemActivated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemActivated < 'c, C > {
        pub fn emit(&mut self, index: i64,) {
            self.typed.emit_tuple((index,));
            
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
    use crate::obj::WithSignals;
    impl WithSignals for ItemList {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfItemList < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfItemList < 'c, C > {
        type Target = < < ItemList as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = ItemList;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfItemList < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = ItemList;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
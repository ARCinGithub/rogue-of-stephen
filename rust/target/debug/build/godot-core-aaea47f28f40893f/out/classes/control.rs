#![doc = "Sidecar module for class [`Control`][crate::classes::Control].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Control` enums](https://docs.godotengine.org/en/stable/classes/class_control.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Control`.\n\nInherits [`CanvasItem`][crate::classes::CanvasItem].\n\nRelated symbols:\n\n* [`control`][crate::classes::control]: sidecar module with related enum/flag types\n* [`IControl`][crate::classes::IControl]: virtual methods\n* [`SignalsOfControl`][crate::classes::control::SignalsOfControl]: signal collection\n* [`ControlNotification`][crate::classes::notify::ControlNotification]: notification type\n\n\nSee also [Godot docs for `Control`](https://docs.godotengine.org/en/stable/classes/class_control.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Control::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nBase class for all UI-related nodes. `Control` features a bounding rectangle that defines its extents, an anchor position relative to its parent control or the current viewport, and offsets relative to the anchor. The offsets update automatically when the node, any of its parents, or the screen size change.\n\nFor more information on Godot's UI system, anchors, offsets, and containers, see the related tutorials in the manual. To build flexible UIs, you'll need a mix of UI elements that inherit from `Control` and [`Container`][crate::classes::Container] nodes.\n\n**Note:** Since both [`Node2D`][crate::classes::Node2D] and `Control` inherit from [`CanvasItem`][crate::classes::CanvasItem], they share several concepts from the class such as the \\[member CanvasItem.z_index] and \\[member CanvasItem.visible] properties.\n\n**User Interface nodes and input**\n\nGodot propagates input events via viewports. Each [`Viewport`][crate::classes::Viewport] is responsible for propagating [`InputEvent`][crate::classes::InputEvent]s to their child nodes. As the \\[member SceneTree.root] is a [`Window`][crate::classes::Window], this already happens automatically for all UI elements in your game.\n\nInput events are propagated through the [`SceneTree`][crate::classes::SceneTree] from the root node to all child nodes by calling [`input`][`crate::classes::INode::input`]. For UI elements specifically, it makes more sense to override the virtual method [`gui_input`][`crate::classes::IControl::gui_input`], which filters out unrelated input events, such as by checking z-order, \\[member mouse_filter], focus, or if the event was inside of the control's bounding box.\n\nCall [`accept_event`][`crate::classes::Control::accept_event`] so no other node receives the event. Once you accept an input, it becomes handled so [`unhandled_input`][`crate::classes::INode::unhandled_input`] will not process it.\n\nOnly one `Control` node can be in focus. Only the node in focus will receive events. To get the focus, call [`grab_focus`][`crate::classes::Control::grab_focus`]. `Control` nodes lose focus when another node grabs it, or if you hide the node in focus. Focus will not be represented visually if gained via mouse/touch input, only appearing with keyboard/gamepad input (for accessibility), or via [`grab_focus`][`crate::classes::Control::grab_focus`].\n\nSet \\[member mouse_filter] to [`MouseFilter::IGNORE`][`crate::classes::control::MouseFilter::IGNORE`] to tell a `Control` node to ignore mouse or touch events. You'll need it if you place an icon on top of a button.\n\n[`Theme`][crate::classes::Theme] resources change the control's appearance. The \\[member theme] of a `Control` node affects all of its direct and indirect children (as long as a chain of controls is uninterrupted). To override some of the theme items, call one of the `add_theme_*_override` methods, like [`add_theme_font_override`][`crate::classes::Control::add_theme_font_override`]. You can also override theme items in the Inspector.\n\n**Note:** Theme items are _not_ [`Object`][crate::classes::Object] properties. This means you can't access their values using [`get`][`crate::classes::Object::get`] and [`set`][`crate::classes::Object::set`]. Instead, use the `get_theme_*` and `add_theme_*_override` methods provided by this class."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Control {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Control`][crate::classes::Control].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Control` methods](https://docs.godotengine.org/en/stable/classes/class_control.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IControl: crate::obj::GodotClass < Base = Control > + crate::private::You_forgot_the_attribute__godot_api {
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
    #[doc = "Notification type for class [`Control`][crate::classes::Control]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum ControlNotification {
        RESIZED = 40i32, MOUSE_ENTER = 41i32, MOUSE_EXIT = 42i32, MOUSE_ENTER_SELF = 60i32, MOUSE_EXIT_SELF = 61i32, FOCUS_ENTER = 43i32, FOCUS_EXIT = 44i32, THEME_CHANGED = 45i32, SCROLL_BEGIN = 47i32, SCROLL_END = 48i32, LAYOUT_DIRECTION_CHANGED = 49i32, TRANSFORM_CHANGED = 2000i32, LOCAL_TRANSFORM_CHANGED = 35i32, DRAW = 30i32, VISIBILITY_CHANGED = 31i32, ENTER_CANVAS = 32i32, EXIT_CANVAS = 33i32, WORLD_2D_CHANGED = 36i32, ENTER_TREE = 10i32, EXIT_TREE = 11i32, MOVED_IN_PARENT = 12i32, READY = 13i32, PAUSED = 14i32, UNPAUSED = 15i32, PHYSICS_PROCESS = 16i32, PROCESS = 17i32, PARENTED = 18i32, UNPARENTED = 19i32, SCENE_INSTANTIATED = 20i32, DRAG_BEGIN = 21i32, DRAG_END = 22i32, PATH_RENAMED = 23i32, CHILD_ORDER_CHANGED = 24i32, INTERNAL_PROCESS = 25i32, INTERNAL_PHYSICS_PROCESS = 26i32, POST_ENTER_TREE = 27i32, DISABLED = 28i32, ENABLED = 29i32, RESET_PHYSICS_INTERPOLATION = 2001i32, EDITOR_PRE_SAVE = 9001i32, EDITOR_POST_SAVE = 9002i32, WM_MOUSE_ENTER = 1002i32, WM_MOUSE_EXIT = 1003i32, WM_WINDOW_FOCUS_IN = 1004i32, WM_WINDOW_FOCUS_OUT = 1005i32, WM_CLOSE_REQUEST = 1006i32, WM_GO_BACK_REQUEST = 1007i32, WM_SIZE_CHANGED = 1008i32, WM_DPI_CHANGE = 1009i32, VP_MOUSE_ENTER = 1010i32, VP_MOUSE_EXIT = 1011i32, WM_POSITION_CHANGED = 1012i32, OS_MEMORY_WARNING = 2009i32, TRANSLATION_CHANGED = 2010i32, WM_ABOUT = 2011i32, CRASH = 2012i32, OS_IME_UPDATE = 2013i32, APPLICATION_RESUMED = 2014i32, APPLICATION_PAUSED = 2015i32, APPLICATION_FOCUS_IN = 2016i32, APPLICATION_FOCUS_OUT = 2017i32, TEXT_SERVER_CHANGED = 2018i32, ACCESSIBILITY_UPDATE = 3000i32, ACCESSIBILITY_INVALIDATE = 3001i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for ControlNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                40i32 => Self::RESIZED, 41i32 => Self::MOUSE_ENTER, 42i32 => Self::MOUSE_EXIT, 60i32 => Self::MOUSE_ENTER_SELF, 61i32 => Self::MOUSE_EXIT_SELF, 43i32 => Self::FOCUS_ENTER, 44i32 => Self::FOCUS_EXIT, 45i32 => Self::THEME_CHANGED, 47i32 => Self::SCROLL_BEGIN, 48i32 => Self::SCROLL_END, 49i32 => Self::LAYOUT_DIRECTION_CHANGED, 2000i32 => Self::TRANSFORM_CHANGED, 35i32 => Self::LOCAL_TRANSFORM_CHANGED, 30i32 => Self::DRAW, 31i32 => Self::VISIBILITY_CHANGED, 32i32 => Self::ENTER_CANVAS, 33i32 => Self::EXIT_CANVAS, 36i32 => Self::WORLD_2D_CHANGED, 10i32 => Self::ENTER_TREE, 11i32 => Self::EXIT_TREE, 12i32 => Self::MOVED_IN_PARENT, 13i32 => Self::READY, 14i32 => Self::PAUSED, 15i32 => Self::UNPAUSED, 16i32 => Self::PHYSICS_PROCESS, 17i32 => Self::PROCESS, 18i32 => Self::PARENTED, 19i32 => Self::UNPARENTED, 20i32 => Self::SCENE_INSTANTIATED, 21i32 => Self::DRAG_BEGIN, 22i32 => Self::DRAG_END, 23i32 => Self::PATH_RENAMED, 24i32 => Self::CHILD_ORDER_CHANGED, 25i32 => Self::INTERNAL_PROCESS, 26i32 => Self::INTERNAL_PHYSICS_PROCESS, 27i32 => Self::POST_ENTER_TREE, 28i32 => Self::DISABLED, 29i32 => Self::ENABLED, 2001i32 => Self::RESET_PHYSICS_INTERPOLATION, 9001i32 => Self::EDITOR_PRE_SAVE, 9002i32 => Self::EDITOR_POST_SAVE, 1002i32 => Self::WM_MOUSE_ENTER, 1003i32 => Self::WM_MOUSE_EXIT, 1004i32 => Self::WM_WINDOW_FOCUS_IN, 1005i32 => Self::WM_WINDOW_FOCUS_OUT, 1006i32 => Self::WM_CLOSE_REQUEST, 1007i32 => Self::WM_GO_BACK_REQUEST, 1008i32 => Self::WM_SIZE_CHANGED, 1009i32 => Self::WM_DPI_CHANGE, 1010i32 => Self::VP_MOUSE_ENTER, 1011i32 => Self::VP_MOUSE_EXIT, 1012i32 => Self::WM_POSITION_CHANGED, 2009i32 => Self::OS_MEMORY_WARNING, 2010i32 => Self::TRANSLATION_CHANGED, 2011i32 => Self::WM_ABOUT, 2012i32 => Self::CRASH, 2013i32 => Self::OS_IME_UPDATE, 2014i32 => Self::APPLICATION_RESUMED, 2015i32 => Self::APPLICATION_PAUSED, 2016i32 => Self::APPLICATION_FOCUS_IN, 2017i32 => Self::APPLICATION_FOCUS_OUT, 2018i32 => Self::TEXT_SERVER_CHANGED, 3000i32 => Self::ACCESSIBILITY_UPDATE, 3001i32 => Self::ACCESSIBILITY_INVALIDATE, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < ControlNotification > for i32 {
        fn from(notification: ControlNotification) -> i32 {
            match notification {
                ControlNotification::RESIZED => 40i32, ControlNotification::MOUSE_ENTER => 41i32, ControlNotification::MOUSE_EXIT => 42i32, ControlNotification::MOUSE_ENTER_SELF => 60i32, ControlNotification::MOUSE_EXIT_SELF => 61i32, ControlNotification::FOCUS_ENTER => 43i32, ControlNotification::FOCUS_EXIT => 44i32, ControlNotification::THEME_CHANGED => 45i32, ControlNotification::SCROLL_BEGIN => 47i32, ControlNotification::SCROLL_END => 48i32, ControlNotification::LAYOUT_DIRECTION_CHANGED => 49i32, ControlNotification::TRANSFORM_CHANGED => 2000i32, ControlNotification::LOCAL_TRANSFORM_CHANGED => 35i32, ControlNotification::DRAW => 30i32, ControlNotification::VISIBILITY_CHANGED => 31i32, ControlNotification::ENTER_CANVAS => 32i32, ControlNotification::EXIT_CANVAS => 33i32, ControlNotification::WORLD_2D_CHANGED => 36i32, ControlNotification::ENTER_TREE => 10i32, ControlNotification::EXIT_TREE => 11i32, ControlNotification::MOVED_IN_PARENT => 12i32, ControlNotification::READY => 13i32, ControlNotification::PAUSED => 14i32, ControlNotification::UNPAUSED => 15i32, ControlNotification::PHYSICS_PROCESS => 16i32, ControlNotification::PROCESS => 17i32, ControlNotification::PARENTED => 18i32, ControlNotification::UNPARENTED => 19i32, ControlNotification::SCENE_INSTANTIATED => 20i32, ControlNotification::DRAG_BEGIN => 21i32, ControlNotification::DRAG_END => 22i32, ControlNotification::PATH_RENAMED => 23i32, ControlNotification::CHILD_ORDER_CHANGED => 24i32, ControlNotification::INTERNAL_PROCESS => 25i32, ControlNotification::INTERNAL_PHYSICS_PROCESS => 26i32, ControlNotification::POST_ENTER_TREE => 27i32, ControlNotification::DISABLED => 28i32, ControlNotification::ENABLED => 29i32, ControlNotification::RESET_PHYSICS_INTERPOLATION => 2001i32, ControlNotification::EDITOR_PRE_SAVE => 9001i32, ControlNotification::EDITOR_POST_SAVE => 9002i32, ControlNotification::WM_MOUSE_ENTER => 1002i32, ControlNotification::WM_MOUSE_EXIT => 1003i32, ControlNotification::WM_WINDOW_FOCUS_IN => 1004i32, ControlNotification::WM_WINDOW_FOCUS_OUT => 1005i32, ControlNotification::WM_CLOSE_REQUEST => 1006i32, ControlNotification::WM_GO_BACK_REQUEST => 1007i32, ControlNotification::WM_SIZE_CHANGED => 1008i32, ControlNotification::WM_DPI_CHANGE => 1009i32, ControlNotification::VP_MOUSE_ENTER => 1010i32, ControlNotification::VP_MOUSE_EXIT => 1011i32, ControlNotification::WM_POSITION_CHANGED => 1012i32, ControlNotification::OS_MEMORY_WARNING => 2009i32, ControlNotification::TRANSLATION_CHANGED => 2010i32, ControlNotification::WM_ABOUT => 2011i32, ControlNotification::CRASH => 2012i32, ControlNotification::OS_IME_UPDATE => 2013i32, ControlNotification::APPLICATION_RESUMED => 2014i32, ControlNotification::APPLICATION_PAUSED => 2015i32, ControlNotification::APPLICATION_FOCUS_IN => 2016i32, ControlNotification::APPLICATION_FOCUS_OUT => 2017i32, ControlNotification::TEXT_SERVER_CHANGED => 2018i32, ControlNotification::ACCESSIBILITY_UPDATE => 3000i32, ControlNotification::ACCESSIBILITY_INVALIDATE => 3001i32, ControlNotification::POSTINITIALIZE => 0i32, ControlNotification::PREDELETE => 1i32, ControlNotification::EXTENSION_RELOADED => 2i32, ControlNotification::Unknown(int) => int,
            }
        }
    }
    impl Control {
        #[doc = "Marks an input event as handled. Once you accept an input event, it stops propagating, even to nodes listening to [`unhandled_input`][`crate::classes::INode::unhandled_input`] or [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`].\n\n**Note:** This does not affect the methods in [`Input`][crate::classes::Input], only the way events are propagated."]
        pub fn accept_event(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "accept_event", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the minimum size for this control. See \\[member custom_minimum_size]."]
        pub fn get_minimum_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_minimum_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns combined minimum size from \\[member custom_minimum_size] and [`get_minimum_size`][`crate::classes::Control::get_minimum_size`]."]
        pub fn get_combined_minimum_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_combined_minimum_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the anchors to a `preset` from \\[enum Control.LayoutPreset] enum. This is the code equivalent to using the Layout menu in the 2D editor.\n\nIf `keep_offsets` is `true`, control's position will also be updated."]
        pub(crate) fn set_anchors_preset_full(&mut self, preset: crate::classes::control::LayoutPreset, keep_offsets: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::LayoutPreset, bool,);
            let args = (preset, keep_offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_anchors_preset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_anchors_preset_ex`][Self::set_anchors_preset_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the anchors to a `preset` from \\[enum Control.LayoutPreset] enum. This is the code equivalent to using the Layout menu in the 2D editor.\n\nIf `keep_offsets` is `true`, control's position will also be updated."]
        #[inline]
        pub fn set_anchors_preset(&mut self, preset: crate::classes::control::LayoutPreset,) {
            self.set_anchors_preset_ex(preset,) . done()
        }
        #[doc = "Sets the anchors to a `preset` from \\[enum Control.LayoutPreset] enum. This is the code equivalent to using the Layout menu in the 2D editor.\n\nIf `keep_offsets` is `true`, control's position will also be updated."]
        #[inline]
        pub fn set_anchors_preset_ex < 'ex > (&'ex mut self, preset: crate::classes::control::LayoutPreset,) -> ExSetAnchorsPreset < 'ex > {
            ExSetAnchorsPreset::new(self, preset,)
        }
        #[doc = "Sets the offsets to a `preset` from \\[enum Control.LayoutPreset] enum. This is the code equivalent to using the Layout menu in the 2D editor.\n\nUse parameter `resize_mode` with constants from \\[enum Control.LayoutPresetMode] to better determine the resulting size of the `Control`. Constant size will be ignored if used with presets that change size, e.g. [`LayoutPreset::LEFT_WIDE`][`crate::classes::control::LayoutPreset::LEFT_WIDE`].\n\nUse parameter `margin` to determine the gap between the `Control` and the edges."]
        pub(crate) fn set_offsets_preset_full(&mut self, preset: crate::classes::control::LayoutPreset, resize_mode: crate::classes::control::LayoutPresetMode, margin: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::LayoutPreset, crate::classes::control::LayoutPresetMode, i32,);
            let args = (preset, resize_mode, margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_offsets_preset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_offsets_preset_ex`][Self::set_offsets_preset_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the offsets to a `preset` from \\[enum Control.LayoutPreset] enum. This is the code equivalent to using the Layout menu in the 2D editor.\n\nUse parameter `resize_mode` with constants from \\[enum Control.LayoutPresetMode] to better determine the resulting size of the `Control`. Constant size will be ignored if used with presets that change size, e.g. [`LayoutPreset::LEFT_WIDE`][`crate::classes::control::LayoutPreset::LEFT_WIDE`].\n\nUse parameter `margin` to determine the gap between the `Control` and the edges."]
        #[inline]
        pub fn set_offsets_preset(&mut self, preset: crate::classes::control::LayoutPreset,) {
            self.set_offsets_preset_ex(preset,) . done()
        }
        #[doc = "Sets the offsets to a `preset` from \\[enum Control.LayoutPreset] enum. This is the code equivalent to using the Layout menu in the 2D editor.\n\nUse parameter `resize_mode` with constants from \\[enum Control.LayoutPresetMode] to better determine the resulting size of the `Control`. Constant size will be ignored if used with presets that change size, e.g. [`LayoutPreset::LEFT_WIDE`][`crate::classes::control::LayoutPreset::LEFT_WIDE`].\n\nUse parameter `margin` to determine the gap between the `Control` and the edges."]
        #[inline]
        pub fn set_offsets_preset_ex < 'ex > (&'ex mut self, preset: crate::classes::control::LayoutPreset,) -> ExSetOffsetsPreset < 'ex > {
            ExSetOffsetsPreset::new(self, preset,)
        }
        #[doc = "Sets both anchor preset and offset preset. See [`set_anchors_preset`][`crate::classes::Control::set_anchors_preset`] and [`set_offsets_preset`][`crate::classes::Control::set_offsets_preset`]."]
        pub(crate) fn set_anchors_and_offsets_preset_full(&mut self, preset: crate::classes::control::LayoutPreset, resize_mode: crate::classes::control::LayoutPresetMode, margin: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::LayoutPreset, crate::classes::control::LayoutPresetMode, i32,);
            let args = (preset, resize_mode, margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_anchors_and_offsets_preset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_anchors_and_offsets_preset_ex`][Self::set_anchors_and_offsets_preset_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets both anchor preset and offset preset. See [`set_anchors_preset`][`crate::classes::Control::set_anchors_preset`] and [`set_offsets_preset`][`crate::classes::Control::set_offsets_preset`]."]
        #[inline]
        pub fn set_anchors_and_offsets_preset(&mut self, preset: crate::classes::control::LayoutPreset,) {
            self.set_anchors_and_offsets_preset_ex(preset,) . done()
        }
        #[doc = "Sets both anchor preset and offset preset. See [`set_anchors_preset`][`crate::classes::Control::set_anchors_preset`] and [`set_offsets_preset`][`crate::classes::Control::set_offsets_preset`]."]
        #[inline]
        pub fn set_anchors_and_offsets_preset_ex < 'ex > (&'ex mut self, preset: crate::classes::control::LayoutPreset,) -> ExSetAnchorsAndOffsetsPreset < 'ex > {
            ExSetAnchorsAndOffsetsPreset::new(self, preset,)
        }
        #[doc = "Sets the anchor for the specified \\[enum Side] to `anchor`. A setter method for \\[member anchor_bottom], \\[member anchor_left], \\[member anchor_right] and \\[member anchor_top].\n\nIf `keep_offset` is `true`, offsets aren't updated after this operation.\n\nIf `push_opposite_anchor` is `true` and the opposite anchor overlaps this anchor, the opposite one will have its value overridden. For example, when setting left anchor to 1 and the right anchor has value of 0.5, the right anchor will also get value of 1. If `push_opposite_anchor` was `false`, the left anchor would get value 0.5."]
        pub(crate) fn set_anchor_full(&mut self, side: crate::builtin::Side, anchor: f32, keep_offset: bool, push_opposite_anchor: bool,) {
            type CallRet = ();
            type CallParams = (crate::builtin::Side, f32, bool, bool,);
            let args = (side, anchor, keep_offset, push_opposite_anchor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_anchor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_anchor_ex`][Self::set_anchor_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the anchor for the specified \\[enum Side] to `anchor`. A setter method for \\[member anchor_bottom], \\[member anchor_left], \\[member anchor_right] and \\[member anchor_top].\n\nIf `keep_offset` is `true`, offsets aren't updated after this operation.\n\nIf `push_opposite_anchor` is `true` and the opposite anchor overlaps this anchor, the opposite one will have its value overridden. For example, when setting left anchor to 1 and the right anchor has value of 0.5, the right anchor will also get value of 1. If `push_opposite_anchor` was `false`, the left anchor would get value 0.5."]
        #[inline]
        pub fn set_anchor(&mut self, side: crate::builtin::Side, anchor: f32,) {
            self.set_anchor_ex(side, anchor,) . done()
        }
        #[doc = "Sets the anchor for the specified \\[enum Side] to `anchor`. A setter method for \\[member anchor_bottom], \\[member anchor_left], \\[member anchor_right] and \\[member anchor_top].\n\nIf `keep_offset` is `true`, offsets aren't updated after this operation.\n\nIf `push_opposite_anchor` is `true` and the opposite anchor overlaps this anchor, the opposite one will have its value overridden. For example, when setting left anchor to 1 and the right anchor has value of 0.5, the right anchor will also get value of 1. If `push_opposite_anchor` was `false`, the left anchor would get value 0.5."]
        #[inline]
        pub fn set_anchor_ex < 'ex > (&'ex mut self, side: crate::builtin::Side, anchor: f32,) -> ExSetAnchor < 'ex > {
            ExSetAnchor::new(self, side, anchor,)
        }
        #[doc = "Returns the anchor for the specified \\[enum Side]. A getter method for \\[member anchor_bottom], \\[member anchor_left], \\[member anchor_right] and \\[member anchor_top]."]
        pub fn get_anchor(&self, side: crate::builtin::Side,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::builtin::Side,);
            let args = (side,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_anchor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the offset for the specified \\[enum Side] to `offset`. A setter method for \\[member offset_bottom], \\[member offset_left], \\[member offset_right] and \\[member offset_top]."]
        pub fn set_offset(&mut self, side: crate::builtin::Side, offset: f32,) {
            type CallRet = ();
            type CallParams = (crate::builtin::Side, f32,);
            let args = (side, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the offset for the specified \\[enum Side]. A getter method for \\[member offset_bottom], \\[member offset_left], \\[member offset_right] and \\[member offset_top]."]
        pub fn get_offset(&self, offset: crate::builtin::Side,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::builtin::Side,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Works the same as [`set_anchor`][`crate::classes::Control::set_anchor`], but instead of `keep_offset` argument and automatic update of offset, it allows to set the offset yourself (see [`set_offset`][`crate::classes::Control::set_offset`])."]
        pub(crate) fn set_anchor_and_offset_full(&mut self, side: crate::builtin::Side, anchor: f32, offset: f32, push_opposite_anchor: bool,) {
            type CallRet = ();
            type CallParams = (crate::builtin::Side, f32, f32, bool,);
            let args = (side, anchor, offset, push_opposite_anchor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_anchor_and_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_anchor_and_offset_ex`][Self::set_anchor_and_offset_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Works the same as [`set_anchor`][`crate::classes::Control::set_anchor`], but instead of `keep_offset` argument and automatic update of offset, it allows to set the offset yourself (see [`set_offset`][`crate::classes::Control::set_offset`])."]
        #[inline]
        pub fn set_anchor_and_offset(&mut self, side: crate::builtin::Side, anchor: f32, offset: f32,) {
            self.set_anchor_and_offset_ex(side, anchor, offset,) . done()
        }
        #[doc = "Works the same as [`set_anchor`][`crate::classes::Control::set_anchor`], but instead of `keep_offset` argument and automatic update of offset, it allows to set the offset yourself (see [`set_offset`][`crate::classes::Control::set_offset`])."]
        #[inline]
        pub fn set_anchor_and_offset_ex < 'ex > (&'ex mut self, side: crate::builtin::Side, anchor: f32, offset: f32,) -> ExSetAnchorAndOffset < 'ex > {
            ExSetAnchorAndOffset::new(self, side, anchor, offset,)
        }
        #[doc = "Sets \\[member offset_left] and \\[member offset_top] at the same time. Equivalent of changing \\[member position]."]
        pub fn set_begin(&mut self, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_begin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets \\[member offset_right] and \\[member offset_bottom] at the same time."]
        pub fn set_end(&mut self, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_end", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member position] to given `position`.\n\nIf `keep_offsets` is `true`, control's anchors will be updated instead of offsets."]
        pub(crate) fn set_position_full(&mut self, position: Vector2, keep_offsets: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, bool,);
            let args = (position, keep_offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_position_ex`][Self::set_position_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the \\[member position] to given `position`.\n\nIf `keep_offsets` is `true`, control's anchors will be updated instead of offsets."]
        #[inline]
        pub fn set_position(&mut self, position: Vector2,) {
            self.set_position_ex(position,) . done()
        }
        #[doc = "Sets the \\[member position] to given `position`.\n\nIf `keep_offsets` is `true`, control's anchors will be updated instead of offsets."]
        #[inline]
        pub fn set_position_ex < 'ex > (&'ex mut self, position: Vector2,) -> ExSetPosition < 'ex > {
            ExSetPosition::new(self, position,)
        }
        #[doc = "Sets the size (see \\[member size]).\n\nIf `keep_offsets` is `true`, control's anchors will be updated instead of offsets."]
        pub(crate) fn set_size_full(&mut self, size: Vector2, keep_offsets: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, bool,);
            let args = (size, keep_offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_size_ex`][Self::set_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the size (see \\[member size]).\n\nIf `keep_offsets` is `true`, control's anchors will be updated instead of offsets."]
        #[inline]
        pub fn set_size(&mut self, size: Vector2,) {
            self.set_size_ex(size,) . done()
        }
        #[doc = "Sets the size (see \\[member size]).\n\nIf `keep_offsets` is `true`, control's anchors will be updated instead of offsets."]
        #[inline]
        pub fn set_size_ex < 'ex > (&'ex mut self, size: Vector2,) -> ExSetSize < 'ex > {
            ExSetSize::new(self, size,)
        }
        #[doc = "Resets the size to [`get_combined_minimum_size`][`crate::classes::Control::get_combined_minimum_size`]. This is equivalent to calling `set_size(Vector2())` (or any size below the minimum)."]
        pub fn reset_size(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "reset_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_minimum_size(&mut self, size: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_custom_minimum_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member global_position] to given `position`.\n\nIf `keep_offsets` is `true`, control's anchors will be updated instead of offsets."]
        pub(crate) fn set_global_position_full(&mut self, position: Vector2, keep_offsets: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, bool,);
            let args = (position, keep_offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_global_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_global_position_ex`][Self::set_global_position_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the \\[member global_position] to given `position`.\n\nIf `keep_offsets` is `true`, control's anchors will be updated instead of offsets."]
        #[inline]
        pub fn set_global_position(&mut self, position: Vector2,) {
            self.set_global_position_ex(position,) . done()
        }
        #[doc = "Sets the \\[member global_position] to given `position`.\n\nIf `keep_offsets` is `true`, control's anchors will be updated instead of offsets."]
        #[inline]
        pub fn set_global_position_ex < 'ex > (&'ex mut self, position: Vector2,) -> ExSetGlobalPosition < 'ex > {
            ExSetGlobalPosition::new(self, position,)
        }
        pub fn set_rotation(&mut self, radians: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation_degrees(&mut self, degrees: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_rotation_degrees", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scale(&mut self, scale: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pivot_offset(&mut self, pivot_offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (pivot_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_pivot_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pivot_offset_ratio(&mut self, ratio: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_pivot_offset_ratio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns \\[member offset_left] and \\[member offset_top]. See also \\[member position]."]
        pub fn get_begin(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_begin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns \\[member offset_right] and \\[member offset_bottom]."]
        pub fn get_end(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation_degrees(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_rotation_degrees", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scale(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pivot_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_pivot_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pivot_offset_ratio(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_pivot_offset_ratio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the combined value of \\[member pivot_offset] and \\[member pivot_offset_ratio], in pixels. The ratio is multiplied by the control's size."]
        pub fn get_combined_pivot_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_combined_pivot_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_minimum_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_custom_minimum_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the width/height occupied in the parent control."]
        pub fn get_parent_area_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_parent_area_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_global_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the position of this `Control` in global screen coordinates (i.e. taking window position into account). Mostly useful for editor plugins.\n\nEquivalent to `get_screen_transform().origin` (see [`get_screen_transform`][`crate::classes::CanvasItem::get_screen_transform`]).\n\n**Example:** Show a popup at the mouse position:\n\n```gdscript\npopup_menu.position = get_screen_position() + get_screen_transform().basis_xform(get_local_mouse_position())\n\n# The above code is equivalent to:\npopup_menu.position = get_screen_transform() * get_local_mouse_position()\n\npopup_menu.reset_size()\npopup_menu.popup()\n```"]
        pub fn get_screen_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_screen_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the position and size of the control in the coordinate system of the containing node. See \\[member position], \\[member scale] and \\[member size].\n\n**Note:** If \\[member rotation] is not the default rotation, the resulting size is not meaningful.\n\n**Note:** Setting \\[member Viewport.gui_snap_controls_to_pixels] to `true` can lead to rounding inaccuracies between the displayed control and the returned [`Rect2`][crate::builtin::Rect2]."]
        pub fn get_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the position and size of the control relative to the containing canvas. See \\[member global_position] and \\[member size].\n\n**Note:** If the node itself or any parent [`CanvasItem`][crate::classes::CanvasItem] between the node and the canvas have a non default rotation or skew, the resulting size is likely not meaningful.\n\n**Note:** Setting \\[member Viewport.gui_snap_controls_to_pixels] to `true` can lead to rounding inaccuracies between the displayed control and the returned [`Rect2`][crate::builtin::Rect2]."]
        pub fn get_global_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_global_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_focus_mode(&mut self, mode: crate::classes::control::FocusMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::FocusMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_focus_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_focus_mode(&self,) -> crate::classes::control::FocusMode {
            type CallRet = crate::classes::control::FocusMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_focus_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the \\[member focus_mode], but takes the \\[member focus_behavior_recursive] into account. If \\[member focus_behavior_recursive] is set to [`FocusBehaviorRecursive::DISABLED`][`crate::classes::control::FocusBehaviorRecursive::DISABLED`], or it is set to [`FocusBehaviorRecursive::INHERITED`][`crate::classes::control::FocusBehaviorRecursive::INHERITED`] and its ancestor is set to [`FocusBehaviorRecursive::DISABLED`][`crate::classes::control::FocusBehaviorRecursive::DISABLED`], then this returns [`FocusMode::NONE`][`crate::classes::control::FocusMode::NONE`]."]
        pub fn get_focus_mode_with_override(&self,) -> crate::classes::control::FocusMode {
            type CallRet = crate::classes::control::FocusMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_focus_mode_with_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_focus_behavior_recursive(&mut self, focus_behavior_recursive: crate::classes::control::FocusBehaviorRecursive,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::FocusBehaviorRecursive,);
            let args = (focus_behavior_recursive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_focus_behavior_recursive", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_focus_behavior_recursive(&self,) -> crate::classes::control::FocusBehaviorRecursive {
            type CallRet = crate::classes::control::FocusBehaviorRecursive;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_focus_behavior_recursive", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this is the current focused control. See \\[member focus_mode].\n\nIf `ignore_hidden_focus` is `true`, controls that have their focus hidden will always return `false`. Hidden focus happens automatically when controls gain focus via mouse input, or manually using [`grab_focus`][`crate::classes::Control::grab_focus`] with `hide_focus` set to `true`."]
        pub(crate) fn has_focus_full(&self, ignore_hidden_focus: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (bool,);
            let args = (ignore_hidden_focus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_focus", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`has_focus_ex`][Self::has_focus_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if this is the current focused control. See \\[member focus_mode].\n\nIf `ignore_hidden_focus` is `true`, controls that have their focus hidden will always return `false`. Hidden focus happens automatically when controls gain focus via mouse input, or manually using [`grab_focus`][`crate::classes::Control::grab_focus`] with `hide_focus` set to `true`."]
        #[inline]
        pub fn has_focus(&self,) -> bool {
            self.has_focus_ex() . done()
        }
        #[doc = "Returns `true` if this is the current focused control. See \\[member focus_mode].\n\nIf `ignore_hidden_focus` is `true`, controls that have their focus hidden will always return `false`. Hidden focus happens automatically when controls gain focus via mouse input, or manually using [`grab_focus`][`crate::classes::Control::grab_focus`] with `hide_focus` set to `true`."]
        #[inline]
        pub fn has_focus_ex < 'ex > (&'ex self,) -> ExHasFocus < 'ex > {
            ExHasFocus::new(self,)
        }
        #[doc = "Steal the focus from another control and become the focused control (see \\[member focus_mode]).\n\nIf `hide_focus` is `true`, the control will not visually show its focused state. Has no effect for [`LineEdit`][crate::classes::LineEdit] and [`TextEdit`][crate::classes::TextEdit] when \\[member ProjectSettings.gui/common/show_focus_state_on_pointer_event] is set to `Control Supports Keyboard Input`, or for any control when it is set to `Always`.\n\n**Note:** Using this method together with [`call_deferred`][`crate::builtin::Callable::call_deferred`] makes it more reliable, especially when called inside [`ready`][`crate::classes::INode::ready`]."]
        pub(crate) fn grab_focus_full(&mut self, hide_focus: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (hide_focus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "grab_focus", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`grab_focus_ex`][Self::grab_focus_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Steal the focus from another control and become the focused control (see \\[member focus_mode]).\n\nIf `hide_focus` is `true`, the control will not visually show its focused state. Has no effect for [`LineEdit`][crate::classes::LineEdit] and [`TextEdit`][crate::classes::TextEdit] when \\[member ProjectSettings.gui/common/show_focus_state_on_pointer_event] is set to `Control Supports Keyboard Input`, or for any control when it is set to `Always`.\n\n**Note:** Using this method together with [`call_deferred`][`crate::builtin::Callable::call_deferred`] makes it more reliable, especially when called inside [`ready`][`crate::classes::INode::ready`]."]
        #[inline]
        pub fn grab_focus(&mut self,) {
            self.grab_focus_ex() . done()
        }
        #[doc = "Steal the focus from another control and become the focused control (see \\[member focus_mode]).\n\nIf `hide_focus` is `true`, the control will not visually show its focused state. Has no effect for [`LineEdit`][crate::classes::LineEdit] and [`TextEdit`][crate::classes::TextEdit] when \\[member ProjectSettings.gui/common/show_focus_state_on_pointer_event] is set to `Control Supports Keyboard Input`, or for any control when it is set to `Always`.\n\n**Note:** Using this method together with [`call_deferred`][`crate::builtin::Callable::call_deferred`] makes it more reliable, especially when called inside [`ready`][`crate::classes::INode::ready`]."]
        #[inline]
        pub fn grab_focus_ex < 'ex > (&'ex mut self,) -> ExGrabFocus < 'ex > {
            ExGrabFocus::new(self,)
        }
        #[doc = "Give up the focus. No other control will be able to receive input."]
        pub fn release_focus(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "release_focus", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Finds the previous (above in the tree) `Control` that can receive the focus."]
        pub fn find_prev_valid_focus(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallRet = Option < Gd < crate::classes::Control > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "find_prev_valid_focus", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Finds the next (below in the tree) `Control` that can receive the focus."]
        pub fn find_next_valid_focus(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallRet = Option < Gd < crate::classes::Control > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "find_next_valid_focus", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Finds the next `Control` that can receive the focus on the specified \\[enum Side].\n\n**Note:** This is different from [`get_focus_neighbor`][`crate::classes::Control::get_focus_neighbor`], which returns the path of a specified focus neighbor."]
        pub fn find_valid_focus_neighbor(&self, side: crate::builtin::Side,) -> Option < Gd < crate::classes::Control > > {
            type CallRet = Option < Gd < crate::classes::Control > >;
            type CallParams = (crate::builtin::Side,);
            let args = (side,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "find_valid_focus_neighbor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_h_size_flags(&mut self, flags: crate::classes::control::SizeFlags,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::SizeFlags,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_h_size_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_h_size_flags(&self,) -> crate::classes::control::SizeFlags {
            type CallRet = crate::classes::control::SizeFlags;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_h_size_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stretch_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_stretch_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stretch_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_stretch_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_v_size_flags(&mut self, flags: crate::classes::control::SizeFlags,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::SizeFlags,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_v_size_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_size_flags(&self,) -> crate::classes::control::SizeFlags {
            type CallRet = crate::classes::control::SizeFlags;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_v_size_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_theme(&mut self, theme: impl AsArg < Option < Gd < crate::classes::Theme >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Theme > > >,);
            let args = (theme.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_theme", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_theme(&self,) -> Option < Gd < crate::classes::Theme > > {
            type CallRet = Option < Gd < crate::classes::Theme > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_theme", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_theme_type_variation(&mut self, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_theme_type_variation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_theme_type_variation(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_theme_type_variation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Prevents `*_theme_*_override` methods from emitting [`ControlNotification::THEME_CHANGED`][`crate::classes::notify::ControlNotification::THEME_CHANGED`] until [`end_bulk_theme_override`][`crate::classes::Control::end_bulk_theme_override`] is called."]
        pub fn begin_bulk_theme_override(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "begin_bulk_theme_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Ends a bulk theme override update. See [`begin_bulk_theme_override`][`crate::classes::Control::begin_bulk_theme_override`]."]
        pub fn end_bulk_theme_override(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "end_bulk_theme_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a local override for a theme icon with the specified `name`. Local overrides always take precedence when fetching theme items for the control. An override can be removed with [`remove_theme_icon_override`][`crate::classes::Control::remove_theme_icon_override`].\n\nSee also [`get_theme_icon`][`crate::classes::Control::get_theme_icon`]."]
        pub fn add_theme_icon_override(&mut self, name: impl AsArg < StringName >, texture: impl AsArg < Gd < crate::classes::Texture2D >>,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Gd < crate::classes::Texture2D > >,);
            let args = (name.into_arg(), texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "add_theme_icon_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a local override for a theme [`StyleBox`][crate::classes::StyleBox] with the specified `name`. Local overrides always take precedence when fetching theme items for the control. An override can be removed with [`remove_theme_stylebox_override`][`crate::classes::Control::remove_theme_stylebox_override`].\n\nSee also [`get_theme_stylebox`][`crate::classes::Control::get_theme_stylebox`].\n\n**Example:** Modify a property in a [`StyleBox`][crate::classes::StyleBox] by duplicating it:\n\n\n```gdscript\n# The snippet below assumes the child node \"MyButton\" has a StyleBoxFlat assigned.\n# Resources are shared across instances, so we need to duplicate it\n# to avoid modifying the appearance of all other buttons.\nvar new_stylebox_normal = $MyButton.get_theme_stylebox(\"normal\").duplicate()\nnew_stylebox_normal.border_width_top = 3\nnew_stylebox_normal.border_color = Color(0, 1, 0.5)\n$MyButton.add_theme_stylebox_override(\"normal\", new_stylebox_normal)\n# Remove the stylebox override.\n$MyButton.remove_theme_stylebox_override(\"normal\")\n```\n"]
        pub fn add_theme_stylebox_override(&mut self, name: impl AsArg < StringName >, stylebox: impl AsArg < Gd < crate::classes::StyleBox >>,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Gd < crate::classes::StyleBox > >,);
            let args = (name.into_arg(), stylebox.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "add_theme_stylebox_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a local override for a theme [`Font`][crate::classes::Font] with the specified `name`. Local overrides always take precedence when fetching theme items for the control. An override can be removed with [`remove_theme_font_override`][`crate::classes::Control::remove_theme_font_override`].\n\nSee also [`get_theme_font`][`crate::classes::Control::get_theme_font`]."]
        pub fn add_theme_font_override(&mut self, name: impl AsArg < StringName >, font: impl AsArg < Gd < crate::classes::Font >>,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Gd < crate::classes::Font > >,);
            let args = (name.into_arg(), font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "add_theme_font_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a local override for a theme font size with the specified `name`. Local overrides always take precedence when fetching theme items for the control. An override can be removed with [`remove_theme_font_size_override`][`crate::classes::Control::remove_theme_font_size_override`].\n\nSee also [`get_theme_font_size`][`crate::classes::Control::get_theme_font_size`]."]
        pub fn add_theme_font_size_override(&mut self, name: impl AsArg < StringName >, font_size: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, i32,);
            let args = (name.into_arg(), font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "add_theme_font_size_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a local override for a theme [`Color`][crate::builtin::Color] with the specified `name`. Local overrides always take precedence when fetching theme items for the control. An override can be removed with [`remove_theme_color_override`][`crate::classes::Control::remove_theme_color_override`].\n\nSee also [`get_theme_color`][`crate::classes::Control::get_theme_color`].\n\n**Example:** Override a [`Label`][crate::classes::Label]'s color and reset it later:\n\n\n```gdscript\n# Given the child Label node \"MyLabel\", override its font color with a custom value.\n$MyLabel.add_theme_color_override(\"font_color\", Color(1, 0.5, 0))\n# Reset the font color of the child label.\n$MyLabel.remove_theme_color_override(\"font_color\")\n# Alternatively it can be overridden with the default value from the Label type.\n$MyLabel.add_theme_color_override(\"font_color\", get_theme_color(\"font_color\", \"Label\"))\n```\n"]
        pub fn add_theme_color_override(&mut self, name: impl AsArg < StringName >, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, Color,);
            let args = (name.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "add_theme_color_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a local override for a theme constant with the specified `name`. Local overrides always take precedence when fetching theme items for the control. An override can be removed with [`remove_theme_constant_override`][`crate::classes::Control::remove_theme_constant_override`].\n\nSee also [`get_theme_constant`][`crate::classes::Control::get_theme_constant`]."]
        pub fn add_theme_constant_override(&mut self, name: impl AsArg < StringName >, constant: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, i32,);
            let args = (name.into_arg(), constant,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "add_theme_constant_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a local override for a theme icon with the specified `name` previously added by [`add_theme_icon_override`][`crate::classes::Control::add_theme_icon_override`] or via the Inspector dock."]
        pub fn remove_theme_icon_override(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "remove_theme_icon_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a local override for a theme [`StyleBox`][crate::classes::StyleBox] with the specified `name` previously added by [`add_theme_stylebox_override`][`crate::classes::Control::add_theme_stylebox_override`] or via the Inspector dock."]
        pub fn remove_theme_stylebox_override(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "remove_theme_stylebox_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a local override for a theme [`Font`][crate::classes::Font] with the specified `name` previously added by [`add_theme_font_override`][`crate::classes::Control::add_theme_font_override`] or via the Inspector dock."]
        pub fn remove_theme_font_override(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "remove_theme_font_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a local override for a theme font size with the specified `name` previously added by [`add_theme_font_size_override`][`crate::classes::Control::add_theme_font_size_override`] or via the Inspector dock."]
        pub fn remove_theme_font_size_override(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "remove_theme_font_size_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a local override for a theme [`Color`][crate::builtin::Color] with the specified `name` previously added by [`add_theme_color_override`][`crate::classes::Control::add_theme_color_override`] or via the Inspector dock."]
        pub fn remove_theme_color_override(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "remove_theme_color_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a local override for a theme constant with the specified `name` previously added by [`add_theme_constant_override`][`crate::classes::Control::add_theme_constant_override`] or via the Inspector dock."]
        pub fn remove_theme_constant_override(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "remove_theme_constant_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an icon from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has an icon item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub(crate) fn get_theme_icon_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_theme_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_theme_icon_ex`][Self::get_theme_icon_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an icon from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has an icon item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn get_theme_icon(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Texture2D > > {
            self.get_theme_icon_ex(name,) . done()
        }
        #[doc = "Returns an icon from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has an icon item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn get_theme_icon_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExGetThemeIcon < 'ex > {
            ExGetThemeIcon::new(self, name,)
        }
        #[doc = "Returns a [`StyleBox`][crate::classes::StyleBox] from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a stylebox item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub(crate) fn get_theme_stylebox_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> Option < Gd < crate::classes::StyleBox > > {
            type CallRet = Option < Gd < crate::classes::StyleBox > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_theme_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_theme_stylebox_ex`][Self::get_theme_stylebox_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a [`StyleBox`][crate::classes::StyleBox] from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a stylebox item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn get_theme_stylebox(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::StyleBox > > {
            self.get_theme_stylebox_ex(name,) . done()
        }
        #[doc = "Returns a [`StyleBox`][crate::classes::StyleBox] from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a stylebox item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn get_theme_stylebox_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExGetThemeStylebox < 'ex > {
            ExGetThemeStylebox::new(self, name,)
        }
        #[doc = "Returns a [`Font`][crate::classes::Font] from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a font item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub(crate) fn get_theme_font_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_theme_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_theme_font_ex`][Self::get_theme_font_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a [`Font`][crate::classes::Font] from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a font item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn get_theme_font(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Font > > {
            self.get_theme_font_ex(name,) . done()
        }
        #[doc = "Returns a [`Font`][crate::classes::Font] from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a font item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn get_theme_font_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExGetThemeFont < 'ex > {
            ExGetThemeFont::new(self, name,)
        }
        #[doc = "Returns a font size from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a font size item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub(crate) fn get_theme_font_size_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_theme_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_theme_font_size_ex`][Self::get_theme_font_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a font size from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a font size item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn get_theme_font_size(&self, name: impl AsArg < StringName >,) -> i32 {
            self.get_theme_font_size_ex(name,) . done()
        }
        #[doc = "Returns a font size from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a font size item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn get_theme_font_size_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExGetThemeFontSize < 'ex > {
            ExGetThemeFontSize::new(self, name,)
        }
        #[doc = "Returns a [`Color`][crate::builtin::Color] from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a color item with the specified `name` and `theme_type`. If `theme_type` is omitted the class name of the current control is used as the type, or \\[member theme_type_variation] if it is defined. If the type is a class name its parent classes are also checked, in order of inheritance. If the type is a variation its base types are checked, in order of dependency, then the control's class name and its parent classes are checked.\n\nFor the current control its local overrides are considered first (see [`add_theme_color_override`][`crate::classes::Control::add_theme_color_override`]), then its assigned \\[member theme]. After the current control, each parent control and its assigned \\[member theme] are considered; controls without a \\[member theme] assigned are skipped. If no matching [`Theme`][crate::classes::Theme] is found in the tree, the custom project [`Theme`][crate::classes::Theme] (see \\[member ProjectSettings.gui/theme/custom]) and the default [`Theme`][crate::classes::Theme] are used (see [`ThemeDB`][crate::classes::ThemeDb]).\n\n\n```gdscript\nfunc _ready():\n\t# Get the font color defined for the current Control's class, if it exists.\n\tmodulate = get_theme_color(\"font_color\")\n\t# Get the font color defined for the Button class.\n\tmodulate = get_theme_color(\"font_color\", \"Button\")\n```\n"]
        pub(crate) fn get_theme_color_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> Color {
            type CallRet = Color;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_theme_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_theme_color_ex`][Self::get_theme_color_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a [`Color`][crate::builtin::Color] from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a color item with the specified `name` and `theme_type`. If `theme_type` is omitted the class name of the current control is used as the type, or \\[member theme_type_variation] if it is defined. If the type is a class name its parent classes are also checked, in order of inheritance. If the type is a variation its base types are checked, in order of dependency, then the control's class name and its parent classes are checked.\n\nFor the current control its local overrides are considered first (see [`add_theme_color_override`][`crate::classes::Control::add_theme_color_override`]), then its assigned \\[member theme]. After the current control, each parent control and its assigned \\[member theme] are considered; controls without a \\[member theme] assigned are skipped. If no matching [`Theme`][crate::classes::Theme] is found in the tree, the custom project [`Theme`][crate::classes::Theme] (see \\[member ProjectSettings.gui/theme/custom]) and the default [`Theme`][crate::classes::Theme] are used (see [`ThemeDB`][crate::classes::ThemeDb]).\n\n\n```gdscript\nfunc _ready():\n\t# Get the font color defined for the current Control's class, if it exists.\n\tmodulate = get_theme_color(\"font_color\")\n\t# Get the font color defined for the Button class.\n\tmodulate = get_theme_color(\"font_color\", \"Button\")\n```\n"]
        #[inline]
        pub fn get_theme_color(&self, name: impl AsArg < StringName >,) -> Color {
            self.get_theme_color_ex(name,) . done()
        }
        #[doc = "Returns a [`Color`][crate::builtin::Color] from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a color item with the specified `name` and `theme_type`. If `theme_type` is omitted the class name of the current control is used as the type, or \\[member theme_type_variation] if it is defined. If the type is a class name its parent classes are also checked, in order of inheritance. If the type is a variation its base types are checked, in order of dependency, then the control's class name and its parent classes are checked.\n\nFor the current control its local overrides are considered first (see [`add_theme_color_override`][`crate::classes::Control::add_theme_color_override`]), then its assigned \\[member theme]. After the current control, each parent control and its assigned \\[member theme] are considered; controls without a \\[member theme] assigned are skipped. If no matching [`Theme`][crate::classes::Theme] is found in the tree, the custom project [`Theme`][crate::classes::Theme] (see \\[member ProjectSettings.gui/theme/custom]) and the default [`Theme`][crate::classes::Theme] are used (see [`ThemeDB`][crate::classes::ThemeDb]).\n\n\n```gdscript\nfunc _ready():\n\t# Get the font color defined for the current Control's class, if it exists.\n\tmodulate = get_theme_color(\"font_color\")\n\t# Get the font color defined for the Button class.\n\tmodulate = get_theme_color(\"font_color\", \"Button\")\n```\n"]
        #[inline]
        pub fn get_theme_color_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExGetThemeColor < 'ex > {
            ExGetThemeColor::new(self, name,)
        }
        #[doc = "Returns a constant from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a constant item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub(crate) fn get_theme_constant_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_theme_constant", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_theme_constant_ex`][Self::get_theme_constant_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a constant from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a constant item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn get_theme_constant(&self, name: impl AsArg < StringName >,) -> i32 {
            self.get_theme_constant_ex(name,) . done()
        }
        #[doc = "Returns a constant from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a constant item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn get_theme_constant_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExGetThemeConstant < 'ex > {
            ExGetThemeConstant::new(self, name,)
        }
        #[doc = "Returns `true` if there is a local override for a theme icon with the specified `name` in this `Control` node.\n\nSee [`add_theme_icon_override`][`crate::classes::Control::add_theme_icon_override`]."]
        pub fn has_theme_icon_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_theme_icon_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there is a local override for a theme [`StyleBox`][crate::classes::StyleBox] with the specified `name` in this `Control` node.\n\nSee [`add_theme_stylebox_override`][`crate::classes::Control::add_theme_stylebox_override`]."]
        pub fn has_theme_stylebox_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_theme_stylebox_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there is a local override for a theme [`Font`][crate::classes::Font] with the specified `name` in this `Control` node.\n\nSee [`add_theme_font_override`][`crate::classes::Control::add_theme_font_override`]."]
        pub fn has_theme_font_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_theme_font_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there is a local override for a theme font size with the specified `name` in this `Control` node.\n\nSee [`add_theme_font_size_override`][`crate::classes::Control::add_theme_font_size_override`]."]
        pub fn has_theme_font_size_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_theme_font_size_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there is a local override for a theme [`Color`][crate::builtin::Color] with the specified `name` in this `Control` node.\n\nSee [`add_theme_color_override`][`crate::classes::Control::add_theme_color_override`]."]
        pub fn has_theme_color_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_theme_color_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there is a local override for a theme constant with the specified `name` in this `Control` node.\n\nSee [`add_theme_constant_override`][`crate::classes::Control::add_theme_constant_override`]."]
        pub fn has_theme_constant_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_theme_constant_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has an icon item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub(crate) fn has_theme_icon_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_theme_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`has_theme_icon_ex`][Self::has_theme_icon_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has an icon item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn has_theme_icon(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_icon_ex(name,) . done()
        }
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has an icon item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn has_theme_icon_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExHasThemeIcon < 'ex > {
            ExHasThemeIcon::new(self, name,)
        }
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a stylebox item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub(crate) fn has_theme_stylebox_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_theme_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`has_theme_stylebox_ex`][Self::has_theme_stylebox_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a stylebox item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn has_theme_stylebox(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_stylebox_ex(name,) . done()
        }
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a stylebox item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn has_theme_stylebox_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExHasThemeStylebox < 'ex > {
            ExHasThemeStylebox::new(self, name,)
        }
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a font item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub(crate) fn has_theme_font_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_theme_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`has_theme_font_ex`][Self::has_theme_font_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a font item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn has_theme_font(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_font_ex(name,) . done()
        }
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a font item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn has_theme_font_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExHasThemeFont < 'ex > {
            ExHasThemeFont::new(self, name,)
        }
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a font size item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub(crate) fn has_theme_font_size_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_theme_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`has_theme_font_size_ex`][Self::has_theme_font_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a font size item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn has_theme_font_size(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_font_size_ex(name,) . done()
        }
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a font size item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn has_theme_font_size_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExHasThemeFontSize < 'ex > {
            ExHasThemeFontSize::new(self, name,)
        }
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a color item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub(crate) fn has_theme_color_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_theme_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`has_theme_color_ex`][Self::has_theme_color_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a color item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn has_theme_color(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_color_ex(name,) . done()
        }
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a color item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn has_theme_color_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExHasThemeColor < 'ex > {
            ExHasThemeColor::new(self, name,)
        }
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a constant item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub(crate) fn has_theme_constant_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "has_theme_constant", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`has_theme_constant_ex`][Self::has_theme_constant_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a constant item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn has_theme_constant(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_constant_ex(name,) . done()
        }
        #[doc = "Returns `true` if there is a matching [`Theme`][crate::classes::Theme] in the tree that has a constant item with the specified `name` and `theme_type`.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        #[inline]
        pub fn has_theme_constant_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExHasThemeConstant < 'ex > {
            ExHasThemeConstant::new(self, name,)
        }
        #[doc = "Returns the default base scale value from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a valid \\[member Theme.default_base_scale] value.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub fn get_theme_default_base_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_theme_default_base_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the default font from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a valid \\[member Theme.default_font] value.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub fn get_theme_default_font(&self,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_theme_default_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the default font size value from the first matching [`Theme`][crate::classes::Theme] in the tree if that [`Theme`][crate::classes::Theme] has a valid \\[member Theme.default_font_size] value.\n\nSee [`get_theme_color`][`crate::classes::Control::get_theme_color`] for details."]
        pub fn get_theme_default_font_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_theme_default_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the parent control node."]
        pub fn get_parent_control(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallRet = Option < Gd < crate::classes::Control > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_parent_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_h_grow_direction(&mut self, direction: crate::classes::control::GrowDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::GrowDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_h_grow_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_h_grow_direction(&self,) -> crate::classes::control::GrowDirection {
            type CallRet = crate::classes::control::GrowDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_h_grow_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_v_grow_direction(&mut self, direction: crate::classes::control::GrowDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::GrowDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_v_grow_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_grow_direction(&self,) -> crate::classes::control::GrowDirection {
            type CallRet = crate::classes::control::GrowDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_v_grow_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tooltip_auto_translate_mode(&mut self, mode: crate::classes::node::AutoTranslateMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::node::AutoTranslateMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_tooltip_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tooltip_auto_translate_mode(&self,) -> crate::classes::node::AutoTranslateMode {
            type CallRet = crate::classes::node::AutoTranslateMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_tooltip_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tooltip_text(&mut self, hint: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (hint.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_tooltip_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tooltip_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_tooltip_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tooltip text for the position `at_position` in control's local coordinates, which will typically appear when the cursor is resting over this control. By default, it returns \\[member tooltip_text].\n\nThis method can be overridden to customize its behavior. See [`get_tooltip`][`crate::classes::IControl::get_tooltip`].\n\n**Note:** If this method returns an empty [`String`][crate::builtin::GString] and [`make_custom_tooltip`][`crate::classes::IControl::make_custom_tooltip`] is not overridden, no tooltip is displayed."]
        pub(crate) fn get_tooltip_full(&self, at_position: Vector2,) -> GString {
            type CallRet = GString;
            type CallParams = (Vector2,);
            let args = (at_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_tooltip_ex`][Self::get_tooltip_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the tooltip text for the position `at_position` in control's local coordinates, which will typically appear when the cursor is resting over this control. By default, it returns \\[member tooltip_text].\n\nThis method can be overridden to customize its behavior. See [`get_tooltip`][`crate::classes::IControl::get_tooltip`].\n\n**Note:** If this method returns an empty [`String`][crate::builtin::GString] and [`make_custom_tooltip`][`crate::classes::IControl::make_custom_tooltip`] is not overridden, no tooltip is displayed."]
        #[inline]
        pub fn get_tooltip(&self,) -> GString {
            self.get_tooltip_ex() . done()
        }
        #[doc = "Returns the tooltip text for the position `at_position` in control's local coordinates, which will typically appear when the cursor is resting over this control. By default, it returns \\[member tooltip_text].\n\nThis method can be overridden to customize its behavior. See [`get_tooltip`][`crate::classes::IControl::get_tooltip`].\n\n**Note:** If this method returns an empty [`String`][crate::builtin::GString] and [`make_custom_tooltip`][`crate::classes::IControl::make_custom_tooltip`] is not overridden, no tooltip is displayed."]
        #[inline]
        pub fn get_tooltip_ex < 'ex > (&'ex self,) -> ExGetTooltip < 'ex > {
            ExGetTooltip::new(self,)
        }
        pub fn set_default_cursor_shape(&mut self, shape: crate::classes::control::CursorShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::CursorShape,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_default_cursor_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_default_cursor_shape(&self,) -> crate::classes::control::CursorShape {
            type CallRet = crate::classes::control::CursorShape;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_default_cursor_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the mouse cursor shape for this control when hovered over `position` in local coordinates. For most controls, this is the same as \\[member mouse_default_cursor_shape], but some built-in controls implement more complex logic."]
        pub(crate) fn get_cursor_shape_full(&self, position: Vector2,) -> crate::classes::control::CursorShape {
            type CallRet = crate::classes::control::CursorShape;
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_cursor_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_cursor_shape_ex`][Self::get_cursor_shape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the mouse cursor shape for this control when hovered over `position` in local coordinates. For most controls, this is the same as \\[member mouse_default_cursor_shape], but some built-in controls implement more complex logic."]
        #[inline]
        pub fn get_cursor_shape(&self,) -> crate::classes::control::CursorShape {
            self.get_cursor_shape_ex() . done()
        }
        #[doc = "Returns the mouse cursor shape for this control when hovered over `position` in local coordinates. For most controls, this is the same as \\[member mouse_default_cursor_shape], but some built-in controls implement more complex logic."]
        #[inline]
        pub fn get_cursor_shape_ex < 'ex > (&'ex self,) -> ExGetCursorShape < 'ex > {
            ExGetCursorShape::new(self,)
        }
        #[doc = "Sets the focus neighbor for the specified \\[enum Side] to the `Control` at `neighbor` node path. A setter method for \\[member focus_neighbor_bottom], \\[member focus_neighbor_left], \\[member focus_neighbor_right] and \\[member focus_neighbor_top]."]
        pub fn set_focus_neighbor(&mut self, side: crate::builtin::Side, neighbor: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::builtin::Side, CowArg < 'a0, NodePath >,);
            let args = (side, neighbor.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_focus_neighbor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the focus neighbor for the specified \\[enum Side]. A getter method for \\[member focus_neighbor_bottom], \\[member focus_neighbor_left], \\[member focus_neighbor_right] and \\[member focus_neighbor_top].\n\n**Note:** To find the next `Control` on the specific \\[enum Side], even if a neighbor is not assigned, use [`find_valid_focus_neighbor`][`crate::classes::Control::find_valid_focus_neighbor`]."]
        pub fn get_focus_neighbor(&self, side: crate::builtin::Side,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (crate::builtin::Side,);
            let args = (side,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_focus_neighbor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_focus_next(&mut self, next: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (next.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_focus_next", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_focus_next(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_focus_next", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_focus_previous(&mut self, previous: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (previous.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_focus_previous", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_focus_previous(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_focus_previous", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Forces drag and bypasses [`get_drag_data`][`crate::classes::IControl::get_drag_data`] and [`set_drag_preview`][`crate::classes::Control::set_drag_preview`] by passing `data` and `preview`. Drag will start even if the mouse is neither over nor pressed on this control.\n\nThe methods [`can_drop_data`][`crate::classes::IControl::can_drop_data`] and [`drop_data`][`crate::classes::IControl::drop_data`] must be implemented on controls that want to receive drop data."]
        pub fn force_drag(&mut self, data: &Variant, preview: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, CowArg < 'a1, Option < Gd < crate::classes::Control > > >,);
            let args = (RefArg::new(data), preview.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "force_drag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Starts drag-and-drop operation without using a mouse."]
        pub fn accessibility_drag(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "accessibility_drag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Ends drag-and-drop operation without using a mouse."]
        pub fn accessibility_drop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "accessibility_drop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accessibility_name(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_accessibility_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessibility_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_accessibility_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accessibility_description(&mut self, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_accessibility_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessibility_description(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_accessibility_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accessibility_live(&mut self, mode: crate::classes::display_server::AccessibilityLiveMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::AccessibilityLiveMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_accessibility_live", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessibility_live(&self,) -> crate::classes::display_server::AccessibilityLiveMode {
            type CallRet = crate::classes::display_server::AccessibilityLiveMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_accessibility_live", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accessibility_controls_nodes(&mut self, node_path: &Array < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < NodePath > >,);
            let args = (RefArg::new(node_path),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_accessibility_controls_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessibility_controls_nodes(&self,) -> Array < NodePath > {
            type CallRet = Array < NodePath >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_accessibility_controls_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accessibility_described_by_nodes(&mut self, node_path: &Array < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < NodePath > >,);
            let args = (RefArg::new(node_path),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_accessibility_described_by_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessibility_described_by_nodes(&self,) -> Array < NodePath > {
            type CallRet = Array < NodePath >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_accessibility_described_by_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accessibility_labeled_by_nodes(&mut self, node_path: &Array < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < NodePath > >,);
            let args = (RefArg::new(node_path),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_accessibility_labeled_by_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessibility_labeled_by_nodes(&self,) -> Array < NodePath > {
            type CallRet = Array < NodePath >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_accessibility_labeled_by_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accessibility_flow_to_nodes(&mut self, node_path: &Array < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < NodePath > >,);
            let args = (RefArg::new(node_path),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_accessibility_flow_to_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessibility_flow_to_nodes(&self,) -> Array < NodePath > {
            type CallRet = Array < NodePath >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_accessibility_flow_to_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mouse_filter(&mut self, filter: crate::classes::control::MouseFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::MouseFilter,);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_mouse_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mouse_filter(&self,) -> crate::classes::control::MouseFilter {
            type CallRet = crate::classes::control::MouseFilter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_mouse_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the \\[member mouse_filter], but takes the \\[member mouse_behavior_recursive] into account. If \\[member mouse_behavior_recursive] is set to [`MouseBehaviorRecursive::DISABLED`][`crate::classes::control::MouseBehaviorRecursive::DISABLED`], or it is set to [`MouseBehaviorRecursive::INHERITED`][`crate::classes::control::MouseBehaviorRecursive::INHERITED`] and its ancestor is set to [`MouseBehaviorRecursive::DISABLED`][`crate::classes::control::MouseBehaviorRecursive::DISABLED`], then this returns [`MouseFilter::IGNORE`][`crate::classes::control::MouseFilter::IGNORE`]."]
        pub fn get_mouse_filter_with_override(&self,) -> crate::classes::control::MouseFilter {
            type CallRet = crate::classes::control::MouseFilter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_mouse_filter_with_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mouse_behavior_recursive(&mut self, mouse_behavior_recursive: crate::classes::control::MouseBehaviorRecursive,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::MouseBehaviorRecursive,);
            let args = (mouse_behavior_recursive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_mouse_behavior_recursive", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mouse_behavior_recursive(&self,) -> crate::classes::control::MouseBehaviorRecursive {
            type CallRet = crate::classes::control::MouseBehaviorRecursive;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_mouse_behavior_recursive", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_force_pass_scroll_events(&mut self, force_pass_scroll_events: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (force_pass_scroll_events,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_force_pass_scroll_events", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_force_pass_scroll_events(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "is_force_pass_scroll_events", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clip_contents(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_clip_contents", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_clipping_contents(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "is_clipping_contents", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates an [`InputEventMouseButton`][crate::classes::InputEventMouseButton] that attempts to click the control. If the event is received, the control gains focus.\n\n\n```gdscript\nfunc _process(delta):\n\tgrab_click_focus() # When clicking another Control node, this node will be clicked instead.\n```\n"]
        pub fn grab_click_focus(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "grab_click_focus", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given callables to be used instead of the control's own drag-and-drop virtual methods. If a callable is empty, its respective virtual method is used as normal.\n\nThe arguments for each callable should be exactly the same as their respective virtual methods, which would be:\n\n- `drag_func` corresponds to [`get_drag_data`][`crate::classes::IControl::get_drag_data`] and requires a [`Vector2`][crate::builtin::Vector2];\n\n- `can_drop_func` corresponds to [`can_drop_data`][`crate::classes::IControl::can_drop_data`] and requires both a [`Vector2`][crate::builtin::Vector2] and a [`Variant`][crate::builtin::Variant];\n\n- `drop_func` corresponds to [`drop_data`][`crate::classes::IControl::drop_data`] and requires both a [`Vector2`][crate::builtin::Vector2] and a [`Variant`][crate::builtin::Variant]."]
        pub fn set_drag_forwarding(&mut self, drag_func: &Callable, can_drop_func: &Callable, drop_func: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Callable >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >,);
            let args = (RefArg::new(drag_func), RefArg::new(can_drop_func), RefArg::new(drop_func),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_drag_forwarding", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Shows the given control at the mouse pointer. A good time to call this method is in [`get_drag_data`][`crate::classes::IControl::get_drag_data`]. The control must not be in the scene tree. You should not free the control, and you should not keep a reference to the control beyond the duration of the drag. It will be deleted automatically after the drag has ended.\n\n\n```gdscript\n@export var color = Color(1, 0, 0, 1)\n\nfunc _get_drag_data(position):\n\t# Use a control that is not in the tree\n\tvar cpb = ColorPickerButton.new()\n\tcpb.color = color\n\tcpb.size = Vector2(50, 50)\n\tset_drag_preview(cpb)\n\treturn color\n```\n"]
        pub fn set_drag_preview(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (control.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_drag_preview", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a drag operation is successful. Alternative to [`gui_is_drag_successful`][`crate::classes::Viewport::gui_is_drag_successful`].\n\nBest used with [`NodeNotification::DRAG_END`][`crate::classes::notify::NodeNotification::DRAG_END`]."]
        pub fn is_drag_successful(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "is_drag_successful", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves the mouse cursor to `position`, relative to \\[member position] of this `Control`.\n\n**Note:** [`warp_mouse`][`crate::classes::Control::warp_mouse`] is only supported on Windows, macOS and Linux. It has no effect on Android, iOS and Web."]
        pub fn warp_mouse(&mut self, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "warp_mouse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shortcut_context(&mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_shortcut_context", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shortcut_context(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_shortcut_context", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Invalidates the size cache in this node and in parent nodes up to top level. Intended to be used with [`get_minimum_size`][`crate::classes::Control::get_minimum_size`] when the return value is changed. Setting \\[member custom_minimum_size] directly calls this method automatically."]
        pub fn update_minimum_size(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "update_minimum_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_layout_direction(&mut self, direction: crate::classes::control::LayoutDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::LayoutDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_layout_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_layout_direction(&self,) -> crate::classes::control::LayoutDirection {
            type CallRet = crate::classes::control::LayoutDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "get_layout_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the layout is right-to-left. See also \\[member layout_direction]."]
        pub fn is_layout_rtl(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "is_layout_rtl", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_translate(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_auto_translate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_auto_translating(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "is_auto_translating", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_localize_numeral_system(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "set_localize_numeral_system", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_localizing_numeral_system(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Control", "is_localizing_numeral_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: ControlNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: ControlNotification) {
            self.notification(i32::from(what), true);
            
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
        pub(crate) const NOTIFICATION_RESIZED: i32 = 40i32;
        pub(crate) const NOTIFICATION_MOUSE_ENTER: i32 = 41i32;
        pub(crate) const NOTIFICATION_MOUSE_EXIT: i32 = 42i32;
        pub(crate) const NOTIFICATION_MOUSE_ENTER_SELF: i32 = 60i32;
        pub(crate) const NOTIFICATION_MOUSE_EXIT_SELF: i32 = 61i32;
        pub(crate) const NOTIFICATION_FOCUS_ENTER: i32 = 43i32;
        pub(crate) const NOTIFICATION_FOCUS_EXIT: i32 = 44i32;
        pub(crate) const NOTIFICATION_THEME_CHANGED: i32 = 45i32;
        pub(crate) const NOTIFICATION_SCROLL_BEGIN: i32 = 47i32;
        pub(crate) const NOTIFICATION_SCROLL_END: i32 = 48i32;
        pub(crate) const NOTIFICATION_LAYOUT_DIRECTION_CHANGED: i32 = 49i32;
        
    }
    impl crate::obj::GodotClass for Control {
        type Base = crate::classes::CanvasItem;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Control"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Control {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Control {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Control {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Control {
        
    }
    impl crate::obj::cap::GodotDefault for Control {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Control {
        type Target = crate::classes::CanvasItem;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Control {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Control`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Control__ensure_class_exists {
        ($Class: ident) => {
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
#[doc = "Default-param extender for [`Control::set_anchors_preset_ex`][super::Control::set_anchors_preset_ex]."]
#[must_use]
pub struct ExSetAnchorsPreset < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Control, preset: crate::classes::control::LayoutPreset, keep_offsets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetAnchorsPreset < 'ex > {
    fn new(surround_object: &'ex mut re_export::Control, preset: crate::classes::control::LayoutPreset,) -> Self {
        let keep_offsets = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset, keep_offsets: keep_offsets,
        }
    }
    #[inline]
    pub fn keep_offsets(self, keep_offsets: bool) -> Self {
        Self {
            keep_offsets: keep_offsets, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, preset, keep_offsets,
        }
        = self;
        re_export::Control::set_anchors_preset_full(surround_object, preset, keep_offsets,)
    }
}
#[doc = "Default-param extender for [`Control::set_offsets_preset_ex`][super::Control::set_offsets_preset_ex]."]
#[must_use]
pub struct ExSetOffsetsPreset < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Control, preset: crate::classes::control::LayoutPreset, resize_mode: crate::classes::control::LayoutPresetMode, margin: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetOffsetsPreset < 'ex > {
    fn new(surround_object: &'ex mut re_export::Control, preset: crate::classes::control::LayoutPreset,) -> Self {
        let resize_mode = crate::obj::EngineEnum::from_ord(0);
        let margin = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset, resize_mode: resize_mode, margin: margin,
        }
    }
    #[inline]
    pub fn resize_mode(self, resize_mode: crate::classes::control::LayoutPresetMode) -> Self {
        Self {
            resize_mode: resize_mode, .. self
        }
    }
    #[inline]
    pub fn margin(self, margin: i32) -> Self {
        Self {
            margin: margin, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, preset, resize_mode, margin,
        }
        = self;
        re_export::Control::set_offsets_preset_full(surround_object, preset, resize_mode, margin,)
    }
}
#[doc = "Default-param extender for [`Control::set_anchors_and_offsets_preset_ex`][super::Control::set_anchors_and_offsets_preset_ex]."]
#[must_use]
pub struct ExSetAnchorsAndOffsetsPreset < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Control, preset: crate::classes::control::LayoutPreset, resize_mode: crate::classes::control::LayoutPresetMode, margin: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetAnchorsAndOffsetsPreset < 'ex > {
    fn new(surround_object: &'ex mut re_export::Control, preset: crate::classes::control::LayoutPreset,) -> Self {
        let resize_mode = crate::obj::EngineEnum::from_ord(0);
        let margin = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset, resize_mode: resize_mode, margin: margin,
        }
    }
    #[inline]
    pub fn resize_mode(self, resize_mode: crate::classes::control::LayoutPresetMode) -> Self {
        Self {
            resize_mode: resize_mode, .. self
        }
    }
    #[inline]
    pub fn margin(self, margin: i32) -> Self {
        Self {
            margin: margin, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, preset, resize_mode, margin,
        }
        = self;
        re_export::Control::set_anchors_and_offsets_preset_full(surround_object, preset, resize_mode, margin,)
    }
}
#[doc = "Default-param extender for [`Control::set_anchor_ex`][super::Control::set_anchor_ex]."]
#[must_use]
pub struct ExSetAnchor < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Control, side: crate::builtin::Side, anchor: f32, keep_offset: bool, push_opposite_anchor: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetAnchor < 'ex > {
    fn new(surround_object: &'ex mut re_export::Control, side: crate::builtin::Side, anchor: f32,) -> Self {
        let keep_offset = false;
        let push_opposite_anchor = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, side: side, anchor: anchor, keep_offset: keep_offset, push_opposite_anchor: push_opposite_anchor,
        }
    }
    #[inline]
    pub fn keep_offset(self, keep_offset: bool) -> Self {
        Self {
            keep_offset: keep_offset, .. self
        }
    }
    #[inline]
    pub fn push_opposite_anchor(self, push_opposite_anchor: bool) -> Self {
        Self {
            push_opposite_anchor: push_opposite_anchor, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, side, anchor, keep_offset, push_opposite_anchor,
        }
        = self;
        re_export::Control::set_anchor_full(surround_object, side, anchor, keep_offset, push_opposite_anchor,)
    }
}
#[doc = "Default-param extender for [`Control::set_anchor_and_offset_ex`][super::Control::set_anchor_and_offset_ex]."]
#[must_use]
pub struct ExSetAnchorAndOffset < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Control, side: crate::builtin::Side, anchor: f32, offset: f32, push_opposite_anchor: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetAnchorAndOffset < 'ex > {
    fn new(surround_object: &'ex mut re_export::Control, side: crate::builtin::Side, anchor: f32, offset: f32,) -> Self {
        let push_opposite_anchor = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, side: side, anchor: anchor, offset: offset, push_opposite_anchor: push_opposite_anchor,
        }
    }
    #[inline]
    pub fn push_opposite_anchor(self, push_opposite_anchor: bool) -> Self {
        Self {
            push_opposite_anchor: push_opposite_anchor, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, side, anchor, offset, push_opposite_anchor,
        }
        = self;
        re_export::Control::set_anchor_and_offset_full(surround_object, side, anchor, offset, push_opposite_anchor,)
    }
}
#[doc = "Default-param extender for [`Control::set_position_ex`][super::Control::set_position_ex]."]
#[must_use]
pub struct ExSetPosition < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Control, position: Vector2, keep_offsets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetPosition < 'ex > {
    fn new(surround_object: &'ex mut re_export::Control, position: Vector2,) -> Self {
        let keep_offsets = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, keep_offsets: keep_offsets,
        }
    }
    #[inline]
    pub fn keep_offsets(self, keep_offsets: bool) -> Self {
        Self {
            keep_offsets: keep_offsets, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, keep_offsets,
        }
        = self;
        re_export::Control::set_position_full(surround_object, position, keep_offsets,)
    }
}
#[doc = "Default-param extender for [`Control::set_size_ex`][super::Control::set_size_ex]."]
#[must_use]
pub struct ExSetSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Control, size: Vector2, keep_offsets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetSize < 'ex > {
    fn new(surround_object: &'ex mut re_export::Control, size: Vector2,) -> Self {
        let keep_offsets = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size: size, keep_offsets: keep_offsets,
        }
    }
    #[inline]
    pub fn keep_offsets(self, keep_offsets: bool) -> Self {
        Self {
            keep_offsets: keep_offsets, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, size, keep_offsets,
        }
        = self;
        re_export::Control::set_size_full(surround_object, size, keep_offsets,)
    }
}
#[doc = "Default-param extender for [`Control::set_global_position_ex`][super::Control::set_global_position_ex]."]
#[must_use]
pub struct ExSetGlobalPosition < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Control, position: Vector2, keep_offsets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetGlobalPosition < 'ex > {
    fn new(surround_object: &'ex mut re_export::Control, position: Vector2,) -> Self {
        let keep_offsets = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, keep_offsets: keep_offsets,
        }
    }
    #[inline]
    pub fn keep_offsets(self, keep_offsets: bool) -> Self {
        Self {
            keep_offsets: keep_offsets, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, keep_offsets,
        }
        = self;
        re_export::Control::set_global_position_full(surround_object, position, keep_offsets,)
    }
}
#[doc = "Default-param extender for [`Control::has_focus_ex`][super::Control::has_focus_ex]."]
#[must_use]
pub struct ExHasFocus < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, ignore_hidden_focus: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExHasFocus < 'ex > {
    fn new(surround_object: &'ex re_export::Control,) -> Self {
        let ignore_hidden_focus = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, ignore_hidden_focus: ignore_hidden_focus,
        }
    }
    #[inline]
    pub fn ignore_hidden_focus(self, ignore_hidden_focus: bool) -> Self {
        Self {
            ignore_hidden_focus: ignore_hidden_focus, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, ignore_hidden_focus,
        }
        = self;
        re_export::Control::has_focus_full(surround_object, ignore_hidden_focus,)
    }
}
#[doc = "Default-param extender for [`Control::grab_focus_ex`][super::Control::grab_focus_ex]."]
#[must_use]
pub struct ExGrabFocus < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Control, hide_focus: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGrabFocus < 'ex > {
    fn new(surround_object: &'ex mut re_export::Control,) -> Self {
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
        re_export::Control::grab_focus_full(surround_object, hide_focus,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_icon_ex`][super::Control::get_theme_icon_ex]."]
#[must_use]
pub struct ExGetThemeIcon < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, name: CowArg < 'ex, StringName >, theme_type: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetThemeIcon < 'ex > {
    fn new(surround_object: &'ex re_export::Control, name: impl AsArg < StringName > + 'ex,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Texture2D > > {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::get_theme_icon_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_stylebox_ex`][super::Control::get_theme_stylebox_ex]."]
#[must_use]
pub struct ExGetThemeStylebox < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, name: CowArg < 'ex, StringName >, theme_type: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetThemeStylebox < 'ex > {
    fn new(surround_object: &'ex re_export::Control, name: impl AsArg < StringName > + 'ex,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::StyleBox > > {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::get_theme_stylebox_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_font_ex`][super::Control::get_theme_font_ex]."]
#[must_use]
pub struct ExGetThemeFont < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, name: CowArg < 'ex, StringName >, theme_type: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetThemeFont < 'ex > {
    fn new(surround_object: &'ex re_export::Control, name: impl AsArg < StringName > + 'ex,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Font > > {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::get_theme_font_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_font_size_ex`][super::Control::get_theme_font_size_ex]."]
#[must_use]
pub struct ExGetThemeFontSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, name: CowArg < 'ex, StringName >, theme_type: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetThemeFontSize < 'ex > {
    fn new(surround_object: &'ex re_export::Control, name: impl AsArg < StringName > + 'ex,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::get_theme_font_size_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_color_ex`][super::Control::get_theme_color_ex]."]
#[must_use]
pub struct ExGetThemeColor < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, name: CowArg < 'ex, StringName >, theme_type: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetThemeColor < 'ex > {
    fn new(surround_object: &'ex re_export::Control, name: impl AsArg < StringName > + 'ex,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Color {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::get_theme_color_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_constant_ex`][super::Control::get_theme_constant_ex]."]
#[must_use]
pub struct ExGetThemeConstant < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, name: CowArg < 'ex, StringName >, theme_type: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetThemeConstant < 'ex > {
    fn new(surround_object: &'ex re_export::Control, name: impl AsArg < StringName > + 'ex,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::get_theme_constant_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_icon_ex`][super::Control::has_theme_icon_ex]."]
#[must_use]
pub struct ExHasThemeIcon < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, name: CowArg < 'ex, StringName >, theme_type: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExHasThemeIcon < 'ex > {
    fn new(surround_object: &'ex re_export::Control, name: impl AsArg < StringName > + 'ex,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::has_theme_icon_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_stylebox_ex`][super::Control::has_theme_stylebox_ex]."]
#[must_use]
pub struct ExHasThemeStylebox < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, name: CowArg < 'ex, StringName >, theme_type: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExHasThemeStylebox < 'ex > {
    fn new(surround_object: &'ex re_export::Control, name: impl AsArg < StringName > + 'ex,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::has_theme_stylebox_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_font_ex`][super::Control::has_theme_font_ex]."]
#[must_use]
pub struct ExHasThemeFont < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, name: CowArg < 'ex, StringName >, theme_type: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExHasThemeFont < 'ex > {
    fn new(surround_object: &'ex re_export::Control, name: impl AsArg < StringName > + 'ex,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::has_theme_font_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_font_size_ex`][super::Control::has_theme_font_size_ex]."]
#[must_use]
pub struct ExHasThemeFontSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, name: CowArg < 'ex, StringName >, theme_type: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExHasThemeFontSize < 'ex > {
    fn new(surround_object: &'ex re_export::Control, name: impl AsArg < StringName > + 'ex,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::has_theme_font_size_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_color_ex`][super::Control::has_theme_color_ex]."]
#[must_use]
pub struct ExHasThemeColor < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, name: CowArg < 'ex, StringName >, theme_type: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExHasThemeColor < 'ex > {
    fn new(surround_object: &'ex re_export::Control, name: impl AsArg < StringName > + 'ex,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::has_theme_color_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_constant_ex`][super::Control::has_theme_constant_ex]."]
#[must_use]
pub struct ExHasThemeConstant < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, name: CowArg < 'ex, StringName >, theme_type: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExHasThemeConstant < 'ex > {
    fn new(surround_object: &'ex re_export::Control, name: impl AsArg < StringName > + 'ex,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::has_theme_constant_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_tooltip_ex`][super::Control::get_tooltip_ex]."]
#[must_use]
pub struct ExGetTooltip < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, at_position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetTooltip < 'ex > {
    fn new(surround_object: &'ex re_export::Control,) -> Self {
        let at_position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, at_position: at_position,
        }
    }
    #[inline]
    pub fn at_position(self, at_position: Vector2) -> Self {
        Self {
            at_position: at_position, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, at_position,
        }
        = self;
        re_export::Control::get_tooltip_full(surround_object, at_position,)
    }
}
#[doc = "Default-param extender for [`Control::get_cursor_shape_ex`][super::Control::get_cursor_shape_ex]."]
#[must_use]
pub struct ExGetCursorShape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Control, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetCursorShape < 'ex > {
    fn new(surround_object: &'ex re_export::Control,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::control::CursorShape {
        let Self {
            _phantom, surround_object, position,
        }
        = self;
        re_export::Control::get_cursor_shape_full(surround_object, position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FocusMode {
    ord: i32
}
impl FocusMode {
    #[doc(alias = "FOCUS_NONE")]
    #[doc = "Godot enumerator name: `FOCUS_NONE`"]
    pub const NONE: FocusMode = FocusMode {
        ord: 0i32
    };
    #[doc(alias = "FOCUS_CLICK")]
    #[doc = "Godot enumerator name: `FOCUS_CLICK`"]
    pub const CLICK: FocusMode = FocusMode {
        ord: 1i32
    };
    #[doc(alias = "FOCUS_ALL")]
    #[doc = "Godot enumerator name: `FOCUS_ALL`"]
    pub const ALL: FocusMode = FocusMode {
        ord: 2i32
    };
    #[doc(alias = "FOCUS_ACCESSIBILITY")]
    #[doc = "Godot enumerator name: `FOCUS_ACCESSIBILITY`"]
    pub const ACCESSIBILITY: FocusMode = FocusMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for FocusMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FocusMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FocusMode {
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
            Self::NONE => "NONE", Self::CLICK => "CLICK", Self::ALL => "ALL", Self::ACCESSIBILITY => "ACCESSIBILITY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FocusMode::NONE, FocusMode::CLICK, FocusMode::ALL, FocusMode::ACCESSIBILITY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FocusMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "FOCUS_NONE", FocusMode::NONE), crate::meta::inspect::EnumConstant::new("CLICK", "FOCUS_CLICK", FocusMode::CLICK), crate::meta::inspect::EnumConstant::new("ALL", "FOCUS_ALL", FocusMode::ALL), crate::meta::inspect::EnumConstant::new("ACCESSIBILITY", "FOCUS_ACCESSIBILITY", FocusMode::ACCESSIBILITY)]
        }
    }
}
impl crate::meta::GodotConvert for FocusMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Focus None", 0i64), EnumeratorShape::new_int("Focus Click", 1i64), EnumeratorShape::new_int("Focus All", 2i64), EnumeratorShape::new_int("Focus Accessibility", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Control.FocusMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for FocusMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FocusMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for FocusMode {
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
impl crate::registry::property::Export for FocusMode {
    
}
impl crate::meta::Element for FocusMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FocusBehaviorRecursive {
    ord: i32
}
impl FocusBehaviorRecursive {
    #[doc(alias = "FOCUS_BEHAVIOR_INHERITED")]
    #[doc = "Godot enumerator name: `FOCUS_BEHAVIOR_INHERITED`"]
    pub const INHERITED: FocusBehaviorRecursive = FocusBehaviorRecursive {
        ord: 0i32
    };
    #[doc(alias = "FOCUS_BEHAVIOR_DISABLED")]
    #[doc = "Godot enumerator name: `FOCUS_BEHAVIOR_DISABLED`"]
    pub const DISABLED: FocusBehaviorRecursive = FocusBehaviorRecursive {
        ord: 1i32
    };
    #[doc(alias = "FOCUS_BEHAVIOR_ENABLED")]
    #[doc = "Godot enumerator name: `FOCUS_BEHAVIOR_ENABLED`"]
    pub const ENABLED: FocusBehaviorRecursive = FocusBehaviorRecursive {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for FocusBehaviorRecursive {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FocusBehaviorRecursive") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FocusBehaviorRecursive {
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
            Self::INHERITED => "INHERITED", Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FocusBehaviorRecursive::INHERITED, FocusBehaviorRecursive::DISABLED, FocusBehaviorRecursive::ENABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FocusBehaviorRecursive >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERITED", "FOCUS_BEHAVIOR_INHERITED", FocusBehaviorRecursive::INHERITED), crate::meta::inspect::EnumConstant::new("DISABLED", "FOCUS_BEHAVIOR_DISABLED", FocusBehaviorRecursive::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "FOCUS_BEHAVIOR_ENABLED", FocusBehaviorRecursive::ENABLED)]
        }
    }
}
impl crate::meta::GodotConvert for FocusBehaviorRecursive {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Focus Behavior Inherited", 0i64), EnumeratorShape::new_int("Focus Behavior Disabled", 1i64), EnumeratorShape::new_int("Focus Behavior Enabled", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Control.FocusBehaviorRecursive")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for FocusBehaviorRecursive {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FocusBehaviorRecursive {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for FocusBehaviorRecursive {
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
impl crate::registry::property::Export for FocusBehaviorRecursive {
    
}
impl crate::meta::Element for FocusBehaviorRecursive {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MouseBehaviorRecursive {
    ord: i32
}
impl MouseBehaviorRecursive {
    #[doc(alias = "MOUSE_BEHAVIOR_INHERITED")]
    #[doc = "Godot enumerator name: `MOUSE_BEHAVIOR_INHERITED`"]
    pub const INHERITED: MouseBehaviorRecursive = MouseBehaviorRecursive {
        ord: 0i32
    };
    #[doc(alias = "MOUSE_BEHAVIOR_DISABLED")]
    #[doc = "Godot enumerator name: `MOUSE_BEHAVIOR_DISABLED`"]
    pub const DISABLED: MouseBehaviorRecursive = MouseBehaviorRecursive {
        ord: 1i32
    };
    #[doc(alias = "MOUSE_BEHAVIOR_ENABLED")]
    #[doc = "Godot enumerator name: `MOUSE_BEHAVIOR_ENABLED`"]
    pub const ENABLED: MouseBehaviorRecursive = MouseBehaviorRecursive {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MouseBehaviorRecursive {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MouseBehaviorRecursive") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MouseBehaviorRecursive {
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
            Self::INHERITED => "INHERITED", Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MouseBehaviorRecursive::INHERITED, MouseBehaviorRecursive::DISABLED, MouseBehaviorRecursive::ENABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MouseBehaviorRecursive >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERITED", "MOUSE_BEHAVIOR_INHERITED", MouseBehaviorRecursive::INHERITED), crate::meta::inspect::EnumConstant::new("DISABLED", "MOUSE_BEHAVIOR_DISABLED", MouseBehaviorRecursive::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "MOUSE_BEHAVIOR_ENABLED", MouseBehaviorRecursive::ENABLED)]
        }
    }
}
impl crate::meta::GodotConvert for MouseBehaviorRecursive {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Mouse Behavior Inherited", 0i64), EnumeratorShape::new_int("Mouse Behavior Disabled", 1i64), EnumeratorShape::new_int("Mouse Behavior Enabled", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Control.MouseBehaviorRecursive")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for MouseBehaviorRecursive {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MouseBehaviorRecursive {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for MouseBehaviorRecursive {
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
impl crate::registry::property::Export for MouseBehaviorRecursive {
    
}
impl crate::meta::Element for MouseBehaviorRecursive {
    
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Control.CursorShape")), is_bitfield: false,
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LayoutPreset {
    ord: i32
}
impl LayoutPreset {
    #[doc(alias = "PRESET_TOP_LEFT")]
    #[doc = "Godot enumerator name: `PRESET_TOP_LEFT`"]
    pub const TOP_LEFT: LayoutPreset = LayoutPreset {
        ord: 0i32
    };
    #[doc(alias = "PRESET_TOP_RIGHT")]
    #[doc = "Godot enumerator name: `PRESET_TOP_RIGHT`"]
    pub const TOP_RIGHT: LayoutPreset = LayoutPreset {
        ord: 1i32
    };
    #[doc(alias = "PRESET_BOTTOM_LEFT")]
    #[doc = "Godot enumerator name: `PRESET_BOTTOM_LEFT`"]
    pub const BOTTOM_LEFT: LayoutPreset = LayoutPreset {
        ord: 2i32
    };
    #[doc(alias = "PRESET_BOTTOM_RIGHT")]
    #[doc = "Godot enumerator name: `PRESET_BOTTOM_RIGHT`"]
    pub const BOTTOM_RIGHT: LayoutPreset = LayoutPreset {
        ord: 3i32
    };
    #[doc(alias = "PRESET_CENTER_LEFT")]
    #[doc = "Godot enumerator name: `PRESET_CENTER_LEFT`"]
    pub const CENTER_LEFT: LayoutPreset = LayoutPreset {
        ord: 4i32
    };
    #[doc(alias = "PRESET_CENTER_TOP")]
    #[doc = "Godot enumerator name: `PRESET_CENTER_TOP`"]
    pub const CENTER_TOP: LayoutPreset = LayoutPreset {
        ord: 5i32
    };
    #[doc(alias = "PRESET_CENTER_RIGHT")]
    #[doc = "Godot enumerator name: `PRESET_CENTER_RIGHT`"]
    pub const CENTER_RIGHT: LayoutPreset = LayoutPreset {
        ord: 6i32
    };
    #[doc(alias = "PRESET_CENTER_BOTTOM")]
    #[doc = "Godot enumerator name: `PRESET_CENTER_BOTTOM`"]
    pub const CENTER_BOTTOM: LayoutPreset = LayoutPreset {
        ord: 7i32
    };
    #[doc(alias = "PRESET_CENTER")]
    #[doc = "Godot enumerator name: `PRESET_CENTER`"]
    pub const CENTER: LayoutPreset = LayoutPreset {
        ord: 8i32
    };
    #[doc(alias = "PRESET_LEFT_WIDE")]
    #[doc = "Godot enumerator name: `PRESET_LEFT_WIDE`"]
    pub const LEFT_WIDE: LayoutPreset = LayoutPreset {
        ord: 9i32
    };
    #[doc(alias = "PRESET_TOP_WIDE")]
    #[doc = "Godot enumerator name: `PRESET_TOP_WIDE`"]
    pub const TOP_WIDE: LayoutPreset = LayoutPreset {
        ord: 10i32
    };
    #[doc(alias = "PRESET_RIGHT_WIDE")]
    #[doc = "Godot enumerator name: `PRESET_RIGHT_WIDE`"]
    pub const RIGHT_WIDE: LayoutPreset = LayoutPreset {
        ord: 11i32
    };
    #[doc(alias = "PRESET_BOTTOM_WIDE")]
    #[doc = "Godot enumerator name: `PRESET_BOTTOM_WIDE`"]
    pub const BOTTOM_WIDE: LayoutPreset = LayoutPreset {
        ord: 12i32
    };
    #[doc(alias = "PRESET_VCENTER_WIDE")]
    #[doc = "Godot enumerator name: `PRESET_VCENTER_WIDE`"]
    pub const VCENTER_WIDE: LayoutPreset = LayoutPreset {
        ord: 13i32
    };
    #[doc(alias = "PRESET_HCENTER_WIDE")]
    #[doc = "Godot enumerator name: `PRESET_HCENTER_WIDE`"]
    pub const HCENTER_WIDE: LayoutPreset = LayoutPreset {
        ord: 14i32
    };
    #[doc(alias = "PRESET_FULL_RECT")]
    #[doc = "Godot enumerator name: `PRESET_FULL_RECT`"]
    pub const FULL_RECT: LayoutPreset = LayoutPreset {
        ord: 15i32
    };
    
}
impl std::fmt::Debug for LayoutPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LayoutPreset") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LayoutPreset {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 => Some(Self {
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
            Self::TOP_LEFT => "TOP_LEFT", Self::TOP_RIGHT => "TOP_RIGHT", Self::BOTTOM_LEFT => "BOTTOM_LEFT", Self::BOTTOM_RIGHT => "BOTTOM_RIGHT", Self::CENTER_LEFT => "CENTER_LEFT", Self::CENTER_TOP => "CENTER_TOP", Self::CENTER_RIGHT => "CENTER_RIGHT", Self::CENTER_BOTTOM => "CENTER_BOTTOM", Self::CENTER => "CENTER", Self::LEFT_WIDE => "LEFT_WIDE", Self::TOP_WIDE => "TOP_WIDE", Self::RIGHT_WIDE => "RIGHT_WIDE", Self::BOTTOM_WIDE => "BOTTOM_WIDE", Self::VCENTER_WIDE => "VCENTER_WIDE", Self::HCENTER_WIDE => "HCENTER_WIDE", Self::FULL_RECT => "FULL_RECT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LayoutPreset::TOP_LEFT, LayoutPreset::TOP_RIGHT, LayoutPreset::BOTTOM_LEFT, LayoutPreset::BOTTOM_RIGHT, LayoutPreset::CENTER_LEFT, LayoutPreset::CENTER_TOP, LayoutPreset::CENTER_RIGHT, LayoutPreset::CENTER_BOTTOM, LayoutPreset::CENTER, LayoutPreset::LEFT_WIDE, LayoutPreset::TOP_WIDE, LayoutPreset::RIGHT_WIDE, LayoutPreset::BOTTOM_WIDE, LayoutPreset::VCENTER_WIDE, LayoutPreset::HCENTER_WIDE, LayoutPreset::FULL_RECT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LayoutPreset >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOP_LEFT", "PRESET_TOP_LEFT", LayoutPreset::TOP_LEFT), crate::meta::inspect::EnumConstant::new("TOP_RIGHT", "PRESET_TOP_RIGHT", LayoutPreset::TOP_RIGHT), crate::meta::inspect::EnumConstant::new("BOTTOM_LEFT", "PRESET_BOTTOM_LEFT", LayoutPreset::BOTTOM_LEFT), crate::meta::inspect::EnumConstant::new("BOTTOM_RIGHT", "PRESET_BOTTOM_RIGHT", LayoutPreset::BOTTOM_RIGHT), crate::meta::inspect::EnumConstant::new("CENTER_LEFT", "PRESET_CENTER_LEFT", LayoutPreset::CENTER_LEFT), crate::meta::inspect::EnumConstant::new("CENTER_TOP", "PRESET_CENTER_TOP", LayoutPreset::CENTER_TOP), crate::meta::inspect::EnumConstant::new("CENTER_RIGHT", "PRESET_CENTER_RIGHT", LayoutPreset::CENTER_RIGHT), crate::meta::inspect::EnumConstant::new("CENTER_BOTTOM", "PRESET_CENTER_BOTTOM", LayoutPreset::CENTER_BOTTOM), crate::meta::inspect::EnumConstant::new("CENTER", "PRESET_CENTER", LayoutPreset::CENTER), crate::meta::inspect::EnumConstant::new("LEFT_WIDE", "PRESET_LEFT_WIDE", LayoutPreset::LEFT_WIDE), crate::meta::inspect::EnumConstant::new("TOP_WIDE", "PRESET_TOP_WIDE", LayoutPreset::TOP_WIDE), crate::meta::inspect::EnumConstant::new("RIGHT_WIDE", "PRESET_RIGHT_WIDE", LayoutPreset::RIGHT_WIDE), crate::meta::inspect::EnumConstant::new("BOTTOM_WIDE", "PRESET_BOTTOM_WIDE", LayoutPreset::BOTTOM_WIDE), crate::meta::inspect::EnumConstant::new("VCENTER_WIDE", "PRESET_VCENTER_WIDE", LayoutPreset::VCENTER_WIDE), crate::meta::inspect::EnumConstant::new("HCENTER_WIDE", "PRESET_HCENTER_WIDE", LayoutPreset::HCENTER_WIDE), crate::meta::inspect::EnumConstant::new("FULL_RECT", "PRESET_FULL_RECT", LayoutPreset::FULL_RECT)]
        }
    }
}
impl crate::meta::GodotConvert for LayoutPreset {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Preset Top Left", 0i64), EnumeratorShape::new_int("Preset Top Right", 1i64), EnumeratorShape::new_int("Preset Bottom Left", 2i64), EnumeratorShape::new_int("Preset Bottom Right", 3i64), EnumeratorShape::new_int("Preset Center Left", 4i64), EnumeratorShape::new_int("Preset Center Top", 5i64), EnumeratorShape::new_int("Preset Center Right", 6i64), EnumeratorShape::new_int("Preset Center Bottom", 7i64), EnumeratorShape::new_int("Preset Center", 8i64), EnumeratorShape::new_int("Preset Left Wide", 9i64), EnumeratorShape::new_int("Preset Top Wide", 10i64), EnumeratorShape::new_int("Preset Right Wide", 11i64), EnumeratorShape::new_int("Preset Bottom Wide", 12i64), EnumeratorShape::new_int("Preset Vcenter Wide", 13i64), EnumeratorShape::new_int("Preset Hcenter Wide", 14i64), EnumeratorShape::new_int("Preset Full Rect", 15i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Control.LayoutPreset")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LayoutPreset {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LayoutPreset {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LayoutPreset {
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
impl crate::registry::property::Export for LayoutPreset {
    
}
impl crate::meta::Element for LayoutPreset {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LayoutPresetMode {
    ord: i32
}
impl LayoutPresetMode {
    #[doc(alias = "PRESET_MODE_MINSIZE")]
    #[doc = "Godot enumerator name: `PRESET_MODE_MINSIZE`"]
    pub const MINSIZE: LayoutPresetMode = LayoutPresetMode {
        ord: 0i32
    };
    #[doc(alias = "PRESET_MODE_KEEP_WIDTH")]
    #[doc = "Godot enumerator name: `PRESET_MODE_KEEP_WIDTH`"]
    pub const KEEP_WIDTH: LayoutPresetMode = LayoutPresetMode {
        ord: 1i32
    };
    #[doc(alias = "PRESET_MODE_KEEP_HEIGHT")]
    #[doc = "Godot enumerator name: `PRESET_MODE_KEEP_HEIGHT`"]
    pub const KEEP_HEIGHT: LayoutPresetMode = LayoutPresetMode {
        ord: 2i32
    };
    #[doc(alias = "PRESET_MODE_KEEP_SIZE")]
    #[doc = "Godot enumerator name: `PRESET_MODE_KEEP_SIZE`"]
    pub const KEEP_SIZE: LayoutPresetMode = LayoutPresetMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for LayoutPresetMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LayoutPresetMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LayoutPresetMode {
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
            Self::MINSIZE => "MINSIZE", Self::KEEP_WIDTH => "KEEP_WIDTH", Self::KEEP_HEIGHT => "KEEP_HEIGHT", Self::KEEP_SIZE => "KEEP_SIZE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LayoutPresetMode::MINSIZE, LayoutPresetMode::KEEP_WIDTH, LayoutPresetMode::KEEP_HEIGHT, LayoutPresetMode::KEEP_SIZE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LayoutPresetMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MINSIZE", "PRESET_MODE_MINSIZE", LayoutPresetMode::MINSIZE), crate::meta::inspect::EnumConstant::new("KEEP_WIDTH", "PRESET_MODE_KEEP_WIDTH", LayoutPresetMode::KEEP_WIDTH), crate::meta::inspect::EnumConstant::new("KEEP_HEIGHT", "PRESET_MODE_KEEP_HEIGHT", LayoutPresetMode::KEEP_HEIGHT), crate::meta::inspect::EnumConstant::new("KEEP_SIZE", "PRESET_MODE_KEEP_SIZE", LayoutPresetMode::KEEP_SIZE)]
        }
    }
}
impl crate::meta::GodotConvert for LayoutPresetMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Preset Mode Minsize", 0i64), EnumeratorShape::new_int("Preset Mode Keep Width", 1i64), EnumeratorShape::new_int("Preset Mode Keep Height", 2i64), EnumeratorShape::new_int("Preset Mode Keep Size", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Control.LayoutPresetMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LayoutPresetMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LayoutPresetMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LayoutPresetMode {
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
impl crate::registry::property::Export for LayoutPresetMode {
    
}
impl crate::meta::Element for LayoutPresetMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct SizeFlags {
    ord: u64
}
impl SizeFlags {
    #[doc(alias = "SIZE_SHRINK_BEGIN")]
    #[doc = "Godot enumerator name: `SIZE_SHRINK_BEGIN`"]
    pub const SHRINK_BEGIN: SizeFlags = SizeFlags {
        ord: 0u64
    };
    #[doc(alias = "SIZE_FILL")]
    #[doc = "Godot enumerator name: `SIZE_FILL`"]
    pub const FILL: SizeFlags = SizeFlags {
        ord: 1u64
    };
    #[doc(alias = "SIZE_EXPAND")]
    #[doc = "Godot enumerator name: `SIZE_EXPAND`"]
    pub const EXPAND: SizeFlags = SizeFlags {
        ord: 2u64
    };
    #[doc(alias = "SIZE_EXPAND_FILL")]
    #[doc = "Godot enumerator name: `SIZE_EXPAND_FILL`"]
    pub const EXPAND_FILL: SizeFlags = SizeFlags {
        ord: 3u64
    };
    #[doc(alias = "SIZE_SHRINK_CENTER")]
    #[doc = "Godot enumerator name: `SIZE_SHRINK_CENTER`"]
    pub const SHRINK_CENTER: SizeFlags = SizeFlags {
        ord: 4u64
    };
    #[doc(alias = "SIZE_SHRINK_END")]
    #[doc = "Godot enumerator name: `SIZE_SHRINK_END`"]
    pub const SHRINK_END: SizeFlags = SizeFlags {
        ord: 8u64
    };
    
}
impl std::fmt::Debug for SizeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for SizeFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SizeFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SHRINK_BEGIN", "SIZE_SHRINK_BEGIN", SizeFlags::SHRINK_BEGIN), crate::meta::inspect::EnumConstant::new("FILL", "SIZE_FILL", SizeFlags::FILL), crate::meta::inspect::EnumConstant::new("EXPAND", "SIZE_EXPAND", SizeFlags::EXPAND), crate::meta::inspect::EnumConstant::new("EXPAND_FILL", "SIZE_EXPAND_FILL", SizeFlags::EXPAND_FILL), crate::meta::inspect::EnumConstant::new("SHRINK_CENTER", "SIZE_SHRINK_CENTER", SizeFlags::SHRINK_CENTER), crate::meta::inspect::EnumConstant::new("SHRINK_END", "SIZE_SHRINK_END", SizeFlags::SHRINK_END)]
        }
    }
}
impl std::ops::BitOr for SizeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for SizeFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for SizeFlags {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Size Shrink Begin", 0i64), EnumeratorShape::new_int("Size Fill", 1i64), EnumeratorShape::new_int("Size Expand", 2i64), EnumeratorShape::new_int("Size Expand Fill", 3i64), EnumeratorShape::new_int("Size Shrink Center", 4i64), EnumeratorShape::new_int("Size Shrink End", 8i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Control.SizeFlags")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for SizeFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SizeFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SizeFlags {
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
impl crate::registry::property::Export for SizeFlags {
    
}
impl crate::meta::Element for SizeFlags {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MouseFilter {
    ord: i32
}
impl MouseFilter {
    #[doc(alias = "MOUSE_FILTER_STOP")]
    #[doc = "Godot enumerator name: `MOUSE_FILTER_STOP`"]
    pub const STOP: MouseFilter = MouseFilter {
        ord: 0i32
    };
    #[doc(alias = "MOUSE_FILTER_PASS")]
    #[doc = "Godot enumerator name: `MOUSE_FILTER_PASS`"]
    pub const PASS: MouseFilter = MouseFilter {
        ord: 1i32
    };
    #[doc(alias = "MOUSE_FILTER_IGNORE")]
    #[doc = "Godot enumerator name: `MOUSE_FILTER_IGNORE`"]
    pub const IGNORE: MouseFilter = MouseFilter {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MouseFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MouseFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MouseFilter {
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
            Self::STOP => "STOP", Self::PASS => "PASS", Self::IGNORE => "IGNORE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MouseFilter::STOP, MouseFilter::PASS, MouseFilter::IGNORE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MouseFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STOP", "MOUSE_FILTER_STOP", MouseFilter::STOP), crate::meta::inspect::EnumConstant::new("PASS", "MOUSE_FILTER_PASS", MouseFilter::PASS), crate::meta::inspect::EnumConstant::new("IGNORE", "MOUSE_FILTER_IGNORE", MouseFilter::IGNORE)]
        }
    }
}
impl crate::meta::GodotConvert for MouseFilter {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Mouse Filter Stop", 0i64), EnumeratorShape::new_int("Mouse Filter Pass", 1i64), EnumeratorShape::new_int("Mouse Filter Ignore", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Control.MouseFilter")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for MouseFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MouseFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for MouseFilter {
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
impl crate::registry::property::Export for MouseFilter {
    
}
impl crate::meta::Element for MouseFilter {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GrowDirection {
    ord: i32
}
impl GrowDirection {
    #[doc(alias = "GROW_DIRECTION_BEGIN")]
    #[doc = "Godot enumerator name: `GROW_DIRECTION_BEGIN`"]
    pub const BEGIN: GrowDirection = GrowDirection {
        ord: 0i32
    };
    #[doc(alias = "GROW_DIRECTION_END")]
    #[doc = "Godot enumerator name: `GROW_DIRECTION_END`"]
    pub const END: GrowDirection = GrowDirection {
        ord: 1i32
    };
    #[doc(alias = "GROW_DIRECTION_BOTH")]
    #[doc = "Godot enumerator name: `GROW_DIRECTION_BOTH`"]
    pub const BOTH: GrowDirection = GrowDirection {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for GrowDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GrowDirection") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GrowDirection {
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
            Self::BEGIN => "BEGIN", Self::END => "END", Self::BOTH => "BOTH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GrowDirection::BEGIN, GrowDirection::END, GrowDirection::BOTH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GrowDirection >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BEGIN", "GROW_DIRECTION_BEGIN", GrowDirection::BEGIN), crate::meta::inspect::EnumConstant::new("END", "GROW_DIRECTION_END", GrowDirection::END), crate::meta::inspect::EnumConstant::new("BOTH", "GROW_DIRECTION_BOTH", GrowDirection::BOTH)]
        }
    }
}
impl crate::meta::GodotConvert for GrowDirection {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Grow Direction Begin", 0i64), EnumeratorShape::new_int("Grow Direction End", 1i64), EnumeratorShape::new_int("Grow Direction Both", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Control.GrowDirection")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for GrowDirection {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GrowDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for GrowDirection {
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
impl crate::registry::property::Export for GrowDirection {
    
}
impl crate::meta::Element for GrowDirection {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Anchor {
    ord: i32
}
impl Anchor {
    #[doc(alias = "ANCHOR_BEGIN")]
    #[doc = "Godot enumerator name: `ANCHOR_BEGIN`"]
    pub const BEGIN: Anchor = Anchor {
        ord: 0i32
    };
    #[doc(alias = "ANCHOR_END")]
    #[doc = "Godot enumerator name: `ANCHOR_END`"]
    pub const END: Anchor = Anchor {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Anchor") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Anchor {
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
            Self::BEGIN => "BEGIN", Self::END => "END", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Anchor::BEGIN, Anchor::END]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Anchor >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BEGIN", "ANCHOR_BEGIN", Anchor::BEGIN), crate::meta::inspect::EnumConstant::new("END", "ANCHOR_END", Anchor::END)]
        }
    }
}
impl crate::meta::GodotConvert for Anchor {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Anchor Begin", 0i64), EnumeratorShape::new_int("Anchor End", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Control.Anchor")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Anchor {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Anchor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Anchor {
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
impl crate::registry::property::Export for Anchor {
    
}
impl crate::meta::Element for Anchor {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LayoutDirection {
    ord: i32
}
impl LayoutDirection {
    #[doc(alias = "LAYOUT_DIRECTION_INHERITED")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_INHERITED`"]
    pub const INHERITED: LayoutDirection = LayoutDirection {
        ord: 0i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_APPLICATION_LOCALE")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_APPLICATION_LOCALE`"]
    pub const APPLICATION_LOCALE: LayoutDirection = LayoutDirection {
        ord: 1i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_LTR")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_LTR`"]
    pub const LTR: LayoutDirection = LayoutDirection {
        ord: 2i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_RTL")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_RTL`"]
    pub const RTL: LayoutDirection = LayoutDirection {
        ord: 3i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_SYSTEM_LOCALE")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_SYSTEM_LOCALE`"]
    pub const SYSTEM_LOCALE: LayoutDirection = LayoutDirection {
        ord: 4i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_MAX")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_MAX`"]
    pub const MAX: LayoutDirection = LayoutDirection {
        ord: 5i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_LOCALE")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_LOCALE`"]
    pub const LOCALE: LayoutDirection = LayoutDirection {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for LayoutDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LayoutDirection") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LayoutDirection {
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
            Self::INHERITED => "INHERITED", Self::APPLICATION_LOCALE => "APPLICATION_LOCALE", Self::LTR => "LTR", Self::RTL => "RTL", Self::SYSTEM_LOCALE => "SYSTEM_LOCALE", Self::MAX => "MAX", Self::LOCALE => "LOCALE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LayoutDirection::INHERITED, LayoutDirection::APPLICATION_LOCALE, LayoutDirection::LTR, LayoutDirection::RTL, LayoutDirection::SYSTEM_LOCALE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LayoutDirection >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERITED", "LAYOUT_DIRECTION_INHERITED", LayoutDirection::INHERITED), crate::meta::inspect::EnumConstant::new("APPLICATION_LOCALE", "LAYOUT_DIRECTION_APPLICATION_LOCALE", LayoutDirection::APPLICATION_LOCALE), crate::meta::inspect::EnumConstant::new("LTR", "LAYOUT_DIRECTION_LTR", LayoutDirection::LTR), crate::meta::inspect::EnumConstant::new("RTL", "LAYOUT_DIRECTION_RTL", LayoutDirection::RTL), crate::meta::inspect::EnumConstant::new("SYSTEM_LOCALE", "LAYOUT_DIRECTION_SYSTEM_LOCALE", LayoutDirection::SYSTEM_LOCALE), crate::meta::inspect::EnumConstant::new("MAX", "LAYOUT_DIRECTION_MAX", LayoutDirection::MAX), crate::meta::inspect::EnumConstant::new("LOCALE", "LAYOUT_DIRECTION_LOCALE", LayoutDirection::LOCALE)]
        }
    }
}
impl crate::obj::IndexEnum for LayoutDirection {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for LayoutDirection {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Layout Direction Inherited", 0i64), EnumeratorShape::new_int("Layout Direction Application Locale", 1i64), EnumeratorShape::new_int("Layout Direction Ltr", 2i64), EnumeratorShape::new_int("Layout Direction Rtl", 3i64), EnumeratorShape::new_int("Layout Direction System Locale", 4i64), EnumeratorShape::new_int("Layout Direction Max", 5i64), EnumeratorShape::new_int("Layout Direction Locale", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Control.LayoutDirection")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LayoutDirection {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LayoutDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LayoutDirection {
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
impl crate::registry::property::Export for LayoutDirection {
    
}
impl crate::meta::Element for LayoutDirection {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextDirection {
    ord: i32
}
impl TextDirection {
    #[doc(alias = "TEXT_DIRECTION_INHERITED")]
    #[doc = "Godot enumerator name: `TEXT_DIRECTION_INHERITED`"]
    pub const INHERITED: TextDirection = TextDirection {
        ord: 3i32
    };
    #[doc(alias = "TEXT_DIRECTION_AUTO")]
    #[doc = "Godot enumerator name: `TEXT_DIRECTION_AUTO`"]
    pub const AUTO: TextDirection = TextDirection {
        ord: 0i32
    };
    #[doc(alias = "TEXT_DIRECTION_LTR")]
    #[doc = "Godot enumerator name: `TEXT_DIRECTION_LTR`"]
    pub const LTR: TextDirection = TextDirection {
        ord: 1i32
    };
    #[doc(alias = "TEXT_DIRECTION_RTL")]
    #[doc = "Godot enumerator name: `TEXT_DIRECTION_RTL`"]
    pub const RTL: TextDirection = TextDirection {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TextDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextDirection") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextDirection {
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
            Self::INHERITED => "INHERITED", Self::AUTO => "AUTO", Self::LTR => "LTR", Self::RTL => "RTL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextDirection::INHERITED, TextDirection::AUTO, TextDirection::LTR, TextDirection::RTL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextDirection >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERITED", "TEXT_DIRECTION_INHERITED", TextDirection::INHERITED), crate::meta::inspect::EnumConstant::new("AUTO", "TEXT_DIRECTION_AUTO", TextDirection::AUTO), crate::meta::inspect::EnumConstant::new("LTR", "TEXT_DIRECTION_LTR", TextDirection::LTR), crate::meta::inspect::EnumConstant::new("RTL", "TEXT_DIRECTION_RTL", TextDirection::RTL)]
        }
    }
}
impl crate::meta::GodotConvert for TextDirection {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Text Direction Inherited", 3i64), EnumeratorShape::new_int("Text Direction Auto", 0i64), EnumeratorShape::new_int("Text Direction Ltr", 1i64), EnumeratorShape::new_int("Text Direction Rtl", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Control.TextDirection")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TextDirection {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TextDirection {
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
impl crate::registry::property::Export for TextDirection {
    
}
impl crate::meta::Element for TextDirection {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Control;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Control`][crate::classes::Control] class."]
    pub struct SignalsOfControl < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfControl < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn resized(&mut self) -> SigResized < 'c, C > {
            SigResized {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "resized")
            }
        }
        #[doc = "Signature: `(event: Gd<InputEvent>)`"]
        pub fn gui_input(&mut self) -> SigGuiInput < 'c, C > {
            SigGuiInput {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "gui_input")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn mouse_entered(&mut self) -> SigMouseEntered < 'c, C > {
            SigMouseEntered {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "mouse_entered")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn mouse_exited(&mut self) -> SigMouseExited < 'c, C > {
            SigMouseExited {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "mouse_exited")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn focus_entered(&mut self) -> SigFocusEntered < 'c, C > {
            SigFocusEntered {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "focus_entered")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn focus_exited(&mut self) -> SigFocusExited < 'c, C > {
            SigFocusExited {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "focus_exited")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn size_flags_changed(&mut self) -> SigSizeFlagsChanged < 'c, C > {
            SigSizeFlagsChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "size_flags_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn minimum_size_changed(&mut self) -> SigMinimumSizeChanged < 'c, C > {
            SigMinimumSizeChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "minimum_size_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn theme_changed(&mut self) -> SigThemeChanged < 'c, C > {
            SigThemeChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "theme_changed")
            }
        }
    }
    type TypedSigResized < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigResized < 'c, C: WithSignals > {
        typed: TypedSigResized < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigResized < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigResized < 'c, C > {
        type Target = TypedSigResized < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigResized < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigGuiInput < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::InputEvent >,) >;
    pub struct SigGuiInput < 'c, C: WithSignals > {
        typed: TypedSigGuiInput < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigGuiInput < 'c, C > {
        pub fn emit(&mut self, event: Gd < crate::classes::InputEvent >,) {
            self.typed.emit_tuple((event,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigGuiInput < 'c, C > {
        type Target = TypedSigGuiInput < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigGuiInput < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMouseEntered < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMouseEntered < 'c, C: WithSignals > {
        typed: TypedSigMouseEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMouseEntered < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMouseEntered < 'c, C > {
        type Target = TypedSigMouseEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMouseEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMouseExited < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMouseExited < 'c, C: WithSignals > {
        typed: TypedSigMouseExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMouseExited < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMouseExited < 'c, C > {
        type Target = TypedSigMouseExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMouseExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFocusEntered < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFocusEntered < 'c, C: WithSignals > {
        typed: TypedSigFocusEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFocusEntered < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFocusEntered < 'c, C > {
        type Target = TypedSigFocusEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFocusEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFocusExited < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFocusExited < 'c, C: WithSignals > {
        typed: TypedSigFocusExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFocusExited < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFocusExited < 'c, C > {
        type Target = TypedSigFocusExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFocusExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSizeFlagsChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSizeFlagsChanged < 'c, C: WithSignals > {
        typed: TypedSigSizeFlagsChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSizeFlagsChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSizeFlagsChanged < 'c, C > {
        type Target = TypedSigSizeFlagsChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSizeFlagsChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMinimumSizeChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMinimumSizeChanged < 'c, C: WithSignals > {
        typed: TypedSigMinimumSizeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMinimumSizeChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMinimumSizeChanged < 'c, C > {
        type Target = TypedSigMinimumSizeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMinimumSizeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigThemeChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigThemeChanged < 'c, C: WithSignals > {
        typed: TypedSigThemeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigThemeChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigThemeChanged < 'c, C > {
        type Target = TypedSigThemeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigThemeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Control {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfControl < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfControl < 'c, C > {
        type Target = < < Control as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Control;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfControl < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Control;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
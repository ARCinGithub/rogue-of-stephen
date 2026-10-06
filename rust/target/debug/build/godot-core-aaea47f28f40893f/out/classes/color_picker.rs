#![doc = "Sidecar module for class [`ColorPicker`][crate::classes::ColorPicker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ColorPicker` enums](https://docs.godotengine.org/en/stable/classes/class_colorpicker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ColorPicker`.\n\nInherits [`VBoxContainer`][crate::classes::VBoxContainer].\n\nRelated symbols:\n\n* [`color_picker`][crate::classes::color_picker]: sidecar module with related enum/flag types\n* [`IColorPicker`][crate::classes::IColorPicker]: virtual methods\n* [`SignalsOfColorPicker`][crate::classes::color_picker::SignalsOfColorPicker]: signal collection\n\n\nSee also [Godot docs for `ColorPicker`](https://docs.godotengine.org/en/stable/classes/class_colorpicker.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ColorPicker::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nA widget that provides an interface for selecting or modifying a color. It can optionally provide functionalities like a color sampler (eyedropper), color modes, and presets.\n\n**Note:** This control is the color picker widget itself. You can use a [`ColorPickerButton`][crate::classes::ColorPickerButton] instead if you need a button that brings up a `ColorPicker` in a popup."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ColorPicker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ColorPicker`][crate::classes::ColorPicker].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IVBoxContainer`][crate::classes::IVBoxContainer] > [`IBoxContainer`][crate::classes::IBoxContainer] > [`IContainer`][crate::classes::IContainer] > [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `ColorPicker` methods](https://docs.godotengine.org/en/stable/classes/class_colorpicker.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IColorPicker: crate::obj::GodotClass < Base = ColorPicker > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ContainerNotification) {
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
        #[doc = "Implement to return a list of allowed horizontal \\[enum Control.SizeFlags] for child nodes. This doesn't technically prevent the usages of any other size flags, if your implementation requires that. This only limits the options available to the user in the Inspector dock.\n\n**Note:** Having no size flags is equal to having [`SizeFlags::SHRINK_BEGIN`][`crate::classes::control::SizeFlags::SHRINK_BEGIN`]. As such, this value is always implicitly allowed."]
        fn get_allowed_size_flags_horizontal(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        #[doc = "Implement to return a list of allowed vertical \\[enum Control.SizeFlags] for child nodes. This doesn't technically prevent the usages of any other size flags, if your implementation requires that. This only limits the options available to the user in the Inspector dock.\n\n**Note:** Having no size flags is equal to having [`SizeFlags::SHRINK_BEGIN`][`crate::classes::control::SizeFlags::SHRINK_BEGIN`]. As such, this value is always implicitly allowed."]
        fn get_allowed_size_flags_vertical(&self,) -> PackedInt32Array {
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
    impl ColorPicker {
        pub fn set_pick_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "set_pick_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pick_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "get_pick_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_deferred_mode(&mut self, mode: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "set_deferred_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_deferred_mode(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "is_deferred_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color_mode(&mut self, color_mode: crate::classes::color_picker::ColorModeType,) {
            type CallRet = ();
            type CallParams = (crate::classes::color_picker::ColorModeType,);
            let args = (color_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "set_color_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_mode(&self,) -> crate::classes::color_picker::ColorModeType {
            type CallRet = crate::classes::color_picker::ColorModeType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "get_color_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_edit_alpha(&mut self, show: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (show,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "set_edit_alpha", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editing_alpha(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "is_editing_alpha", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_edit_intensity(&mut self, show: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (show,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "set_edit_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editing_intensity(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "is_editing_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_can_add_swatches(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "set_can_add_swatches", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_swatches_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "are_swatches_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_presets_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "set_presets_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_presets_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "are_presets_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_modes_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "set_modes_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_modes_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "are_modes_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sampler_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "set_sampler_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_sampler_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "is_sampler_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sliders_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "set_sliders_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_sliders_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "are_sliders_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hex_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "set_hex_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_hex_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "is_hex_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds the given color to a list of color presets. The presets are displayed in the color picker and the user will be able to select them.\n\n**Note:** The presets list is only for _this_ color picker."]
        pub fn add_preset(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "add_preset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the given color from the list of color presets of this color picker."]
        pub fn erase_preset(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "erase_preset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of colors in the presets of the color picker."]
        pub fn get_presets(&self,) -> PackedColorArray {
            type CallRet = PackedColorArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "get_presets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds the given color to a list of color recent presets so that it can be picked later. Recent presets are the colors that were picked recently, a new preset is automatically created and added to recent presets when you pick a new color.\n\n**Note:** The recent presets list is only for _this_ color picker."]
        pub fn add_recent_preset(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "add_recent_preset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the given color from the list of color recent presets of this color picker."]
        pub fn erase_recent_preset(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "erase_recent_preset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of colors in the recent presets of the color picker."]
        pub fn get_recent_presets(&self,) -> PackedColorArray {
            type CallRet = PackedColorArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "get_recent_presets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_picker_shape(&mut self, shape: crate::classes::color_picker::PickerShapeType,) {
            type CallRet = ();
            type CallParams = (crate::classes::color_picker::PickerShapeType,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "set_picker_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_picker_shape(&self,) -> crate::classes::color_picker::PickerShapeType {
            type CallRet = crate::classes::color_picker::PickerShapeType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ColorPicker", "get_picker_shape", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ColorPicker {
        type Base = crate::classes::VBoxContainer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ColorPicker"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ColorPicker {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VBoxContainer > for ColorPicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BoxContainer > for ColorPicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for ColorPicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for ColorPicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for ColorPicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ColorPicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ColorPicker {
        
    }
    impl crate::obj::cap::GodotDefault for ColorPicker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ColorPicker {
        type Target = crate::classes::VBoxContainer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ColorPicker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ColorPicker`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ColorPicker__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ColorPicker > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VBoxContainer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::BoxContainer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Container > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ColorModeType {
    ord: i32
}
impl ColorModeType {
    #[doc(alias = "MODE_RGB")]
    #[doc = "Godot enumerator name: `MODE_RGB`"]
    pub const RGB: ColorModeType = ColorModeType {
        ord: 0i32
    };
    #[doc(alias = "MODE_HSV")]
    #[doc = "Godot enumerator name: `MODE_HSV`"]
    pub const HSV: ColorModeType = ColorModeType {
        ord: 1i32
    };
    #[doc(alias = "MODE_RAW")]
    #[doc = "Godot enumerator name: `MODE_RAW`"]
    pub const RAW: ColorModeType = ColorModeType {
        ord: 2i32
    };
    #[doc(alias = "MODE_LINEAR")]
    #[doc = "Godot enumerator name: `MODE_LINEAR`"]
    pub const LINEAR: ColorModeType = ColorModeType {
        ord: 2i32
    };
    #[doc(alias = "MODE_OKHSL")]
    #[doc = "Godot enumerator name: `MODE_OKHSL`"]
    pub const OKHSL: ColorModeType = ColorModeType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ColorModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ColorModeType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ColorModeType {
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
            Self::RGB => "RGB", Self::HSV => "HSV", Self::RAW => "RAW", Self::LINEAR => "LINEAR", Self::OKHSL => "OKHSL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ColorModeType::RGB, ColorModeType::HSV, ColorModeType::RAW, ColorModeType::OKHSL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ColorModeType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RGB", "MODE_RGB", ColorModeType::RGB), crate::meta::inspect::EnumConstant::new("HSV", "MODE_HSV", ColorModeType::HSV), crate::meta::inspect::EnumConstant::new("RAW", "MODE_RAW", ColorModeType::RAW), crate::meta::inspect::EnumConstant::new("LINEAR", "MODE_LINEAR", ColorModeType::LINEAR), crate::meta::inspect::EnumConstant::new("OKHSL", "MODE_OKHSL", ColorModeType::OKHSL)]
        }
    }
}
impl crate::meta::GodotConvert for ColorModeType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Mode Rgb", 0i64), EnumeratorShape::new_int("Mode Hsv", 1i64), EnumeratorShape::new_int("Mode Raw", 2i64), EnumeratorShape::new_int("Mode Linear", 2i64), EnumeratorShape::new_int("Mode Okhsl", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("ColorPicker.ColorModeType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ColorModeType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ColorModeType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ColorModeType {
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
impl crate::registry::property::Export for ColorModeType {
    
}
impl crate::meta::Element for ColorModeType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PickerShapeType {
    ord: i32
}
impl PickerShapeType {
    #[doc(alias = "SHAPE_HSV_RECTANGLE")]
    #[doc = "Godot enumerator name: `SHAPE_HSV_RECTANGLE`"]
    pub const HSV_RECTANGLE: PickerShapeType = PickerShapeType {
        ord: 0i32
    };
    #[doc(alias = "SHAPE_HSV_WHEEL")]
    #[doc = "Godot enumerator name: `SHAPE_HSV_WHEEL`"]
    pub const HSV_WHEEL: PickerShapeType = PickerShapeType {
        ord: 1i32
    };
    #[doc(alias = "SHAPE_VHS_CIRCLE")]
    #[doc = "Godot enumerator name: `SHAPE_VHS_CIRCLE`"]
    pub const VHS_CIRCLE: PickerShapeType = PickerShapeType {
        ord: 2i32
    };
    #[doc(alias = "SHAPE_OKHSL_CIRCLE")]
    #[doc = "Godot enumerator name: `SHAPE_OKHSL_CIRCLE`"]
    pub const OKHSL_CIRCLE: PickerShapeType = PickerShapeType {
        ord: 3i32
    };
    #[doc(alias = "SHAPE_NONE")]
    #[doc = "Godot enumerator name: `SHAPE_NONE`"]
    pub const NONE: PickerShapeType = PickerShapeType {
        ord: 4i32
    };
    #[doc(alias = "SHAPE_OK_HS_RECTANGLE")]
    #[doc = "Godot enumerator name: `SHAPE_OK_HS_RECTANGLE`"]
    pub const OK_HS_RECTANGLE: PickerShapeType = PickerShapeType {
        ord: 5i32
    };
    #[doc(alias = "SHAPE_OK_HL_RECTANGLE")]
    #[doc = "Godot enumerator name: `SHAPE_OK_HL_RECTANGLE`"]
    pub const OK_HL_RECTANGLE: PickerShapeType = PickerShapeType {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for PickerShapeType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PickerShapeType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PickerShapeType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::HSV_RECTANGLE => "HSV_RECTANGLE", Self::HSV_WHEEL => "HSV_WHEEL", Self::VHS_CIRCLE => "VHS_CIRCLE", Self::OKHSL_CIRCLE => "OKHSL_CIRCLE", Self::NONE => "NONE", Self::OK_HS_RECTANGLE => "OK_HS_RECTANGLE", Self::OK_HL_RECTANGLE => "OK_HL_RECTANGLE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PickerShapeType::HSV_RECTANGLE, PickerShapeType::HSV_WHEEL, PickerShapeType::VHS_CIRCLE, PickerShapeType::OKHSL_CIRCLE, PickerShapeType::NONE, PickerShapeType::OK_HS_RECTANGLE, PickerShapeType::OK_HL_RECTANGLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PickerShapeType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("HSV_RECTANGLE", "SHAPE_HSV_RECTANGLE", PickerShapeType::HSV_RECTANGLE), crate::meta::inspect::EnumConstant::new("HSV_WHEEL", "SHAPE_HSV_WHEEL", PickerShapeType::HSV_WHEEL), crate::meta::inspect::EnumConstant::new("VHS_CIRCLE", "SHAPE_VHS_CIRCLE", PickerShapeType::VHS_CIRCLE), crate::meta::inspect::EnumConstant::new("OKHSL_CIRCLE", "SHAPE_OKHSL_CIRCLE", PickerShapeType::OKHSL_CIRCLE), crate::meta::inspect::EnumConstant::new("NONE", "SHAPE_NONE", PickerShapeType::NONE), crate::meta::inspect::EnumConstant::new("OK_HS_RECTANGLE", "SHAPE_OK_HS_RECTANGLE", PickerShapeType::OK_HS_RECTANGLE), crate::meta::inspect::EnumConstant::new("OK_HL_RECTANGLE", "SHAPE_OK_HL_RECTANGLE", PickerShapeType::OK_HL_RECTANGLE)]
        }
    }
}
impl crate::meta::GodotConvert for PickerShapeType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Shape Hsv Rectangle", 0i64), EnumeratorShape::new_int("Shape Hsv Wheel", 1i64), EnumeratorShape::new_int("Shape Vhs Circle", 2i64), EnumeratorShape::new_int("Shape Okhsl Circle", 3i64), EnumeratorShape::new_int("Shape None", 4i64), EnumeratorShape::new_int("Shape Ok Hs Rectangle", 5i64), EnumeratorShape::new_int("Shape Ok Hl Rectangle", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("ColorPicker.PickerShapeType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PickerShapeType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PickerShapeType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PickerShapeType {
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
impl crate::registry::property::Export for PickerShapeType {
    
}
impl crate::meta::Element for PickerShapeType {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ColorPicker;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`ColorPicker`][crate::classes::ColorPicker] class."]
    pub struct SignalsOfColorPicker < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfColorPicker < 'c, C > {
        #[doc = "Signature: `(color: Color)`"]
        pub fn color_changed(&mut self) -> SigColorChanged < 'c, C > {
            SigColorChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "color_changed")
            }
        }
        #[doc = "Signature: `(color: Color)`"]
        pub fn preset_added(&mut self) -> SigPresetAdded < 'c, C > {
            SigPresetAdded {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "preset_added")
            }
        }
        #[doc = "Signature: `(color: Color)`"]
        pub fn preset_removed(&mut self) -> SigPresetRemoved < 'c, C > {
            SigPresetRemoved {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "preset_removed")
            }
        }
    }
    type TypedSigColorChanged < 'c, C > = TypedSignal < 'c, C, (Color,) >;
    pub struct SigColorChanged < 'c, C: WithSignals > {
        typed: TypedSigColorChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigColorChanged < 'c, C > {
        pub fn emit(&mut self, color: Color,) {
            self.typed.emit_tuple((color,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigColorChanged < 'c, C > {
        type Target = TypedSigColorChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigColorChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPresetAdded < 'c, C > = TypedSignal < 'c, C, (Color,) >;
    pub struct SigPresetAdded < 'c, C: WithSignals > {
        typed: TypedSigPresetAdded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPresetAdded < 'c, C > {
        pub fn emit(&mut self, color: Color,) {
            self.typed.emit_tuple((color,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPresetAdded < 'c, C > {
        type Target = TypedSigPresetAdded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPresetAdded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPresetRemoved < 'c, C > = TypedSignal < 'c, C, (Color,) >;
    pub struct SigPresetRemoved < 'c, C: WithSignals > {
        typed: TypedSigPresetRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPresetRemoved < 'c, C > {
        pub fn emit(&mut self, color: Color,) {
            self.typed.emit_tuple((color,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPresetRemoved < 'c, C > {
        type Target = TypedSigPresetRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPresetRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for ColorPicker {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfColorPicker < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfColorPicker < 'c, C > {
        type Target = < < ColorPicker as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = ColorPicker;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfColorPicker < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = ColorPicker;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
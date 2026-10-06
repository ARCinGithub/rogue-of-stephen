#![doc = "Sidecar module for class [`OptionButton`][crate::classes::OptionButton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OptionButton` enums](https://docs.godotengine.org/en/stable/classes/class_optionbutton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OptionButton`.\n\nInherits [`Button`][crate::classes::Button].\n\nRelated symbols:\n\n* [`option_button`][crate::classes::option_button]: sidecar module with related enum/flag types\n* [`IOptionButton`][crate::classes::IOptionButton]: virtual methods\n* [`SignalsOfOptionButton`][crate::classes::option_button::SignalsOfOptionButton]: signal collection\n\n\nSee also [Godot docs for `OptionButton`](https://docs.godotengine.org/en/stable/classes/class_optionbutton.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`OptionButton::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\n`OptionButton` is a type of button that brings up a dropdown with selectable items when pressed. The item selected becomes the \"current\" item and is displayed as the button text.\n\nSee also [`BaseButton`][crate::classes::BaseButton] which contains common properties and methods associated with this node.\n\n**Note:** The IDs used for items are limited to signed 32-bit integers, not the full 64 bits of `int`. These have a range of `-2^31` to `2^31 - 1`, that is, `-2147483648` to `2147483647`.\n\n**Note:** The \\[member Button.text] and \\[member Button.icon] properties are set automatically based on the selected item. They shouldn't be changed manually."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OptionButton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OptionButton`][crate::classes::OptionButton].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IButton`][crate::classes::IButton] > [`IBaseButton`][crate::classes::IBaseButton] > [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `OptionButton` methods](https://docs.godotengine.org/en/stable/classes/class_optionbutton.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOptionButton: crate::obj::GodotClass < Base = OptionButton > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Called when the button is pressed. If you need to know the button's pressed state (and \\[member toggle_mode] is active), use [`toggled`][`crate::classes::IBaseButton::toggled`] instead."]
        fn pressed(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the button is toggled (only if \\[member toggle_mode] is active)."]
        fn toggled(&mut self, toggled_on: bool,) {
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
    impl OptionButton {
        #[doc = "Adds an item, with text `label` and (optionally) `id`. If no `id` is passed, the item index will be used as the item's ID. New items are appended at the end.\n\n**Note:** The item will be selected if there are no other items."]
        pub(crate) fn add_item_full(&mut self, label: CowArg < GString >, id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (label, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "add_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_item_ex`][Self::add_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an item, with text `label` and (optionally) `id`. If no `id` is passed, the item index will be used as the item's ID. New items are appended at the end.\n\n**Note:** The item will be selected if there are no other items."]
        #[inline]
        pub fn add_item(&mut self, label: impl AsArg < GString >,) {
            self.add_item_ex(label,) . done()
        }
        #[doc = "Adds an item, with text `label` and (optionally) `id`. If no `id` is passed, the item index will be used as the item's ID. New items are appended at the end.\n\n**Note:** The item will be selected if there are no other items."]
        #[inline]
        pub fn add_item_ex < 'ex > (&'ex mut self, label: impl AsArg < GString > + 'ex,) -> ExAddItem < 'ex > {
            ExAddItem::new(self, label,)
        }
        #[doc = "Adds an item, with a `texture` icon, text `label` and (optionally) `id`. If no `id` is passed, the item index will be used as the item's ID. New items are appended at the end.\n\n**Note:** The item will be selected if there are no other items."]
        pub(crate) fn add_icon_item_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, i32,);
            let args = (texture, label, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "add_icon_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_icon_item_ex`][Self::add_icon_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an item, with a `texture` icon, text `label` and (optionally) `id`. If no `id` is passed, the item index will be used as the item's ID. New items are appended at the end.\n\n**Note:** The item will be selected if there are no other items."]
        #[inline]
        pub fn add_icon_item(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) {
            self.add_icon_item_ex(texture, label,) . done()
        }
        #[doc = "Adds an item, with a `texture` icon, text `label` and (optionally) `id`. If no `id` is passed, the item index will be used as the item's ID. New items are appended at the end.\n\n**Note:** The item will be selected if there are no other items."]
        #[inline]
        pub fn add_icon_item_ex < 'ex > (&'ex mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExAddIconItem < 'ex > {
            ExAddIconItem::new(self, texture, label,)
        }
        #[doc = "Sets the text of the item at index `idx`."]
        pub fn set_item_text(&mut self, idx: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (idx, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the icon of the item at index `idx`."]
        pub fn set_item_icon(&mut self, idx: i32, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (idx, texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether the item at index `idx` is disabled.\n\nDisabled items are drawn differently in the dropdown and are not selectable by the user. If the current selected item is set as disabled, it will remain selected."]
        pub fn set_item_disabled(&mut self, idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the ID of the item at index `idx`."]
        pub fn set_item_id(&mut self, idx: i32, id: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (idx, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the metadata of an item. Metadata may be of any type and can be used to store extra information about an item, such as an external string ID."]
        pub fn set_item_metadata(&mut self, idx: i32, metadata: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (idx, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the tooltip of the item at index `idx`."]
        pub fn set_item_tooltip(&mut self, idx: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the auto translate mode of the item at index `idx`.\n\nItems use [`AutoTranslateMode::INHERIT`][`crate::classes::node::AutoTranslateMode::INHERIT`] by default, which uses the same auto translate mode as the `OptionButton` itself."]
        pub fn set_item_auto_translate_mode(&mut self, idx: i32, mode: crate::classes::node::AutoTranslateMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::node::AutoTranslateMode,);
            let args = (idx, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text of the item at index `idx`."]
        pub fn get_item_text(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the icon of the item at index `idx`."]
        pub fn get_item_icon(&self, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID of the item at index `idx`."]
        pub fn get_item_id(&self, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the item with the given `id`."]
        pub fn get_item_index(&self, id: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Retrieves the metadata of an item. Metadata may be any type and can be used to store extra information about an item, such as an external string ID."]
        pub fn get_item_metadata(&self, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tooltip of the item at index `idx`."]
        pub fn get_item_tooltip(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the auto translate mode of the item at index `idx`."]
        pub fn get_item_auto_translate_mode(&self, idx: i32,) -> crate::classes::node::AutoTranslateMode {
            type CallRet = crate::classes::node::AutoTranslateMode;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at index `idx` is disabled."]
        pub fn is_item_disabled(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "is_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at index `idx` is marked as a separator."]
        pub fn is_item_separator(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "is_item_separator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a separator to the list of items. Separators help to group items, and can optionally be given a `text` header. A separator also gets an index assigned, and is appended at the end of the item list."]
        pub(crate) fn add_separator_full(&mut self, text: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "add_separator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_separator_ex`][Self::add_separator_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a separator to the list of items. Separators help to group items, and can optionally be given a `text` header. A separator also gets an index assigned, and is appended at the end of the item list."]
        #[inline]
        pub fn add_separator(&mut self,) {
            self.add_separator_ex() . done()
        }
        #[doc = "Adds a separator to the list of items. Separators help to group items, and can optionally be given a `text` header. A separator also gets an index assigned, and is appended at the end of the item list."]
        #[inline]
        pub fn add_separator_ex < 'ex > (&'ex mut self,) -> ExAddSeparator < 'ex > {
            ExAddSeparator::new(self,)
        }
        #[doc = "Clears all the items in the `OptionButton`."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Selects an item by index and makes it the current item. This will work even if the item is disabled.\n\nPassing `-1` as the index deselects any currently selected item."]
        pub fn select(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "select", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_selected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID of the selected item, or `-1` if no item is selected."]
        pub fn get_selected_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_selected_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the metadata of the selected item. Metadata for items can be set using [`set_item_metadata`][`crate::classes::OptionButton::set_item_metadata`]."]
        pub fn get_selected_metadata(&self,) -> Variant {
            type CallRet = Variant;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_selected_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the item at index `idx`."]
        pub fn remove_item(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "remove_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`PopupMenu`][crate::classes::PopupMenu] contained in this button.\n\n**Warning:** This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their \\[member Window.visible] property."]
        pub fn get_popup(&self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallRet = Option < Gd < crate::classes::PopupMenu > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_popup", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adjusts popup position and sizing for the `OptionButton`, then shows the [`PopupMenu`][crate::classes::PopupMenu]. Prefer this over using `get_popup().popup()`."]
        pub fn show_popup(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "show_popup", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this button contains at least one item which is not disabled, or marked as a separator."]
        pub fn has_selectable_items(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "has_selectable_items", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the first item which is not disabled, or marked as a separator. If `from_last` is `true`, the items will be searched in reverse order.\n\nReturns `-1` if no item is found."]
        pub(crate) fn get_selectable_item_full(&self, from_last: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (bool,);
            let args = (from_last,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_selectable_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_selectable_item_ex`][Self::get_selectable_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the index of the first item which is not disabled, or marked as a separator. If `from_last` is `true`, the items will be searched in reverse order.\n\nReturns `-1` if no item is found."]
        #[inline]
        pub fn get_selectable_item(&self,) -> i32 {
            self.get_selectable_item_ex() . done()
        }
        #[doc = "Returns the index of the first item which is not disabled, or marked as a separator. If `from_last` is `true`, the items will be searched in reverse order.\n\nReturns `-1` if no item is found."]
        #[inline]
        pub fn get_selectable_item_ex < 'ex > (&'ex self,) -> ExGetSelectableItem < 'ex > {
            ExGetSelectableItem::new(self,)
        }
        pub fn set_fit_to_longest_item(&mut self, fit: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (fit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_fit_to_longest_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_fit_to_longest_item(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "is_fit_to_longest_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_reselect(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_allow_reselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_reselect(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_allow_reselect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, shortcuts are disabled and cannot be used to trigger the button."]
        pub fn set_disable_shortcuts(&mut self, disabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_disable_shortcuts", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OptionButton {
        type Base = crate::classes::Button;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OptionButton"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OptionButton {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Button > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BaseButton > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OptionButton {
        
    }
    impl crate::obj::cap::GodotDefault for OptionButton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OptionButton {
        type Target = crate::classes::Button;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OptionButton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OptionButton`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OptionButton__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OptionButton > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Button > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::BaseButton > for $Class {
                
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
#[doc = "Default-param extender for [`OptionButton::add_item_ex`][super::OptionButton::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::OptionButton, label: CowArg < 'ex, GString >, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::OptionButton, label: impl AsArg < GString > + 'ex,) -> Self {
        let id = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), id: id,
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
        re_export::OptionButton::add_item_full(surround_object, label, id,)
    }
}
#[doc = "Default-param extender for [`OptionButton::add_icon_item_ex`][super::OptionButton::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::OptionButton, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'ex, GString >, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddIconItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::OptionButton, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let id = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), label: label.into_arg(), id: id,
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
            _phantom, surround_object, texture, label, id,
        }
        = self;
        re_export::OptionButton::add_icon_item_full(surround_object, texture, label, id,)
    }
}
#[doc = "Default-param extender for [`OptionButton::add_separator_ex`][super::OptionButton::add_separator_ex]."]
#[must_use]
pub struct ExAddSeparator < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::OptionButton, text: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddSeparator < 'ex > {
    fn new(surround_object: &'ex mut re_export::OptionButton,) -> Self {
        let text = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: CowArg::Owned(text),
        }
    }
    #[inline]
    pub fn text(self, text: impl AsArg < GString > + 'ex) -> Self {
        Self {
            text: text.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text,
        }
        = self;
        re_export::OptionButton::add_separator_full(surround_object, text,)
    }
}
#[doc = "Default-param extender for [`OptionButton::get_selectable_item_ex`][super::OptionButton::get_selectable_item_ex]."]
#[must_use]
pub struct ExGetSelectableItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::OptionButton, from_last: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSelectableItem < 'ex > {
    fn new(surround_object: &'ex re_export::OptionButton,) -> Self {
        let from_last = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_last: from_last,
        }
    }
    #[inline]
    pub fn from_last(self, from_last: bool) -> Self {
        Self {
            from_last: from_last, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, from_last,
        }
        = self;
        re_export::OptionButton::get_selectable_item_full(surround_object, from_last,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OptionButton;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`OptionButton`][crate::classes::OptionButton] class."]
    pub struct SignalsOfOptionButton < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfOptionButton < 'c, C > {
        #[doc = "Signature: `(index: i64)`"]
        pub fn item_selected(&mut self) -> SigItemSelected < 'c, C > {
            SigItemSelected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "item_selected")
            }
        }
        #[doc = "Signature: `(index: i64)`"]
        pub fn item_focused(&mut self) -> SigItemFocused < 'c, C > {
            SigItemFocused {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "item_focused")
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
    type TypedSigItemFocused < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigItemFocused < 'c, C: WithSignals > {
        typed: TypedSigItemFocused < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemFocused < 'c, C > {
        pub fn emit(&mut self, index: i64,) {
            self.typed.emit_tuple((index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemFocused < 'c, C > {
        type Target = TypedSigItemFocused < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemFocused < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for OptionButton {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfOptionButton < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfOptionButton < 'c, C > {
        type Target = < < OptionButton as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = OptionButton;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfOptionButton < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = OptionButton;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
#![doc = "Sidecar module for class [`RichTextLabel`][crate::classes::RichTextLabel].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RichTextLabel` enums](https://docs.godotengine.org/en/stable/classes/class_richtextlabel.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RichTextLabel`.\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`rich_text_label`][crate::classes::rich_text_label]: sidecar module with related enum/flag types\n* [`IRichTextLabel`][crate::classes::IRichTextLabel]: virtual methods\n* [`SignalsOfRichTextLabel`][crate::classes::rich_text_label::SignalsOfRichTextLabel]: signal collection\n\n\nSee also [Godot docs for `RichTextLabel`](https://docs.godotengine.org/en/stable/classes/class_richtextlabel.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`RichTextLabel::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nA control for displaying text that can contain custom fonts, images, and basic formatting. `RichTextLabel` manages these as an internal tag stack. It also adapts itself to given width/heights.\n\n**Note:** [`newline`][`crate::classes::RichTextLabel::newline`], [`push_paragraph`][`crate::classes::RichTextLabel::push_paragraph`], `\"\\n\"`, `\"\\r\\n\"`, `p` tag, and alignment tags start a new paragraph. Each paragraph is processed independently, in its own BiDi context. If you want to force line wrapping within paragraph, any other line breaking character can be used, for example, Form Feed (U+000C), Next Line (U+0085), Line Separator (U+2028).\n\n**Note:** Assignments to \\[member text] clear the tag stack and reconstruct it from the property's contents. Any edits made to \\[member text] will erase previous edits made from other manual sources such as [`append_text`][`crate::classes::RichTextLabel::append_text`] and the `push_*` / [`pop`][`crate::classes::RichTextLabel::pop`] methods.\n\n**Note:** RichTextLabel doesn't support entangled BBCode tags. For example, instead of using `[b]bold[i]bold italic[/b]italic[/i]`, use `[b]bold[i]bold italic[/i][/b][i]italic[/i]`.\n\n**Note:** `push_*/pop_*` functions won't affect BBCode.\n\n**Note:** While \\[member bbcode_enabled] is enabled, alignment tags such as `[center]` will take priority over the \\[member horizontal_alignment] setting which determines the default text alignment."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RichTextLabel {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`RichTextLabel`][crate::classes::RichTextLabel].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `RichTextLabel` methods](https://docs.godotengine.org/en/stable/classes/class_richtextlabel.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRichTextLabel: crate::obj::GodotClass < Base = RichTextLabel > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RichTextLabel {
        #[doc = "Returns the text without BBCode mark-up."]
        pub fn get_parsed_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_parsed_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds raw non-BBCode-parsed text to the tag stack."]
        pub fn add_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "add_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a horizontal rule that can be used to separate content.\n\nIf `width_in_percent` is set, `width` values are percentages of the control width instead of pixels.\n\nIf `height_in_percent` is set, `height` values are percentages of the control width instead of pixels."]
        pub(crate) fn add_hr_full(&mut self, width: i32, height: i32, color: Color, alignment: crate::global::HorizontalAlignment, width_in_percent: bool, height_in_percent: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, Color, crate::global::HorizontalAlignment, bool, bool,);
            let args = (width, height, color, alignment, width_in_percent, height_in_percent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "add_hr", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_hr_ex`][Self::add_hr_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a horizontal rule that can be used to separate content.\n\nIf `width_in_percent` is set, `width` values are percentages of the control width instead of pixels.\n\nIf `height_in_percent` is set, `height` values are percentages of the control width instead of pixels."]
        #[inline]
        pub fn add_hr(&mut self,) {
            self.add_hr_ex() . done()
        }
        #[doc = "Adds a horizontal rule that can be used to separate content.\n\nIf `width_in_percent` is set, `width` values are percentages of the control width instead of pixels.\n\nIf `height_in_percent` is set, `height` values are percentages of the control width instead of pixels."]
        #[inline]
        pub fn add_hr_ex < 'ex > (&'ex mut self,) -> ExAddHr < 'ex > {
            ExAddHr::new(self,)
        }
        #[doc = "Adds an image's opening and closing tags to the tag stack, optionally providing a `width` and `height` to resize the image, a `color` to tint the image and a `region` to only use parts of the image.\n\nIf `width` or `height` is set to 0, the image size will be adjusted in order to keep the original aspect ratio.\n\nIf `width` and `height` are not set, but `region` is, the region's rect will be used.\n\n`key` is an optional identifier, that can be used to modify the image via [`update_image`][`crate::classes::RichTextLabel::update_image`].\n\nIf `pad` is set, and the image is smaller than the size specified by `width` and `height`, the image padding is added to match the size instead of upscaling.\n\nIf `width_in_percent` is set, `width` values are percentages of the control width instead of pixels.\n\nIf `height_in_percent` is set, `height` values are percentages of the control width instead of pixels.\n\n`alt_text` is used as the image description for assistive apps."]
        pub(crate) fn add_image_full(&mut self, image: CowArg < Option < Gd < crate::classes::Texture2D > > >, width: i32, height: i32, color: Color, inline_align: crate::global::InlineAlignment, region: Rect2, key: RefArg < Variant >, pad: bool, tooltip: CowArg < GString >, width_in_percent: bool, height_in_percent: bool, alt_text: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, i32, i32, Color, crate::global::InlineAlignment, Rect2, RefArg < 'a1, Variant >, bool, CowArg < 'a2, GString >, bool, bool, CowArg < 'a3, GString >,);
            let args = (image, width, height, color, inline_align, region, key, pad, tooltip, width_in_percent, height_in_percent, alt_text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "add_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_image_ex`][Self::add_image_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an image's opening and closing tags to the tag stack, optionally providing a `width` and `height` to resize the image, a `color` to tint the image and a `region` to only use parts of the image.\n\nIf `width` or `height` is set to 0, the image size will be adjusted in order to keep the original aspect ratio.\n\nIf `width` and `height` are not set, but `region` is, the region's rect will be used.\n\n`key` is an optional identifier, that can be used to modify the image via [`update_image`][`crate::classes::RichTextLabel::update_image`].\n\nIf `pad` is set, and the image is smaller than the size specified by `width` and `height`, the image padding is added to match the size instead of upscaling.\n\nIf `width_in_percent` is set, `width` values are percentages of the control width instead of pixels.\n\nIf `height_in_percent` is set, `height` values are percentages of the control width instead of pixels.\n\n`alt_text` is used as the image description for assistive apps."]
        #[inline]
        pub fn add_image(&mut self, image: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            self.add_image_ex(image,) . done()
        }
        #[doc = "Adds an image's opening and closing tags to the tag stack, optionally providing a `width` and `height` to resize the image, a `color` to tint the image and a `region` to only use parts of the image.\n\nIf `width` or `height` is set to 0, the image size will be adjusted in order to keep the original aspect ratio.\n\nIf `width` and `height` are not set, but `region` is, the region's rect will be used.\n\n`key` is an optional identifier, that can be used to modify the image via [`update_image`][`crate::classes::RichTextLabel::update_image`].\n\nIf `pad` is set, and the image is smaller than the size specified by `width` and `height`, the image padding is added to match the size instead of upscaling.\n\nIf `width_in_percent` is set, `width` values are percentages of the control width instead of pixels.\n\nIf `height_in_percent` is set, `height` values are percentages of the control width instead of pixels.\n\n`alt_text` is used as the image description for assistive apps."]
        #[inline]
        pub fn add_image_ex < 'ex > (&'ex mut self, image: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> ExAddImage < 'ex > {
            ExAddImage::new(self, image,)
        }
        #[doc = "Updates the existing images with the key `key`. Only properties specified by `mask` bits are updated. See [`add_image`][`crate::classes::RichTextLabel::add_image`]."]
        pub(crate) fn update_image_full(&mut self, key: RefArg < Variant >, mask: crate::classes::rich_text_label::ImageUpdateMask, image: CowArg < Option < Gd < crate::classes::Texture2D > > >, width: i32, height: i32, color: Color, inline_align: crate::global::InlineAlignment, region: Rect2, pad: bool, tooltip: CowArg < GString >, width_in_percent: bool, height_in_percent: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Variant >, crate::classes::rich_text_label::ImageUpdateMask, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, i32, i32, Color, crate::global::InlineAlignment, Rect2, bool, CowArg < 'a2, GString >, bool, bool,);
            let args = (key, mask, image, width, height, color, inline_align, region, pad, tooltip, width_in_percent, height_in_percent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "update_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`update_image_ex`][Self::update_image_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Updates the existing images with the key `key`. Only properties specified by `mask` bits are updated. See [`add_image`][`crate::classes::RichTextLabel::add_image`]."]
        #[inline]
        pub fn update_image(&mut self, key: &Variant, mask: crate::classes::rich_text_label::ImageUpdateMask, image: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            self.update_image_ex(key, mask, image,) . done()
        }
        #[doc = "Updates the existing images with the key `key`. Only properties specified by `mask` bits are updated. See [`add_image`][`crate::classes::RichTextLabel::add_image`]."]
        #[inline]
        pub fn update_image_ex < 'ex > (&'ex mut self, key: &'ex Variant, mask: crate::classes::rich_text_label::ImageUpdateMask, image: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> ExUpdateImage < 'ex > {
            ExUpdateImage::new(self, key, mask, image,)
        }
        #[doc = "Adds a newline tag to the tag stack."]
        pub fn newline(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "newline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a paragraph of content from the label. Returns `true` if the paragraph exists.\n\nThe `paragraph` argument is the index of the paragraph to remove, it can take values in the interval `[0, get_paragraph_count() - 1]`.\n\nIf `no_invalidate` is set to `true`, cache for the subsequent paragraphs is not invalidated. Use it for faster updates if deleted paragraph is fully self-contained (have no unclosed tags), or this call is part of the complex edit operation and [`invalidate_paragraph`][`crate::classes::RichTextLabel::invalidate_paragraph`] will be called at the end of operation."]
        pub(crate) fn remove_paragraph_full(&mut self, paragraph: i32, no_invalidate: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, bool,);
            let args = (paragraph, no_invalidate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "remove_paragraph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`remove_paragraph_ex`][Self::remove_paragraph_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Removes a paragraph of content from the label. Returns `true` if the paragraph exists.\n\nThe `paragraph` argument is the index of the paragraph to remove, it can take values in the interval `[0, get_paragraph_count() - 1]`.\n\nIf `no_invalidate` is set to `true`, cache for the subsequent paragraphs is not invalidated. Use it for faster updates if deleted paragraph is fully self-contained (have no unclosed tags), or this call is part of the complex edit operation and [`invalidate_paragraph`][`crate::classes::RichTextLabel::invalidate_paragraph`] will be called at the end of operation."]
        #[inline]
        pub fn remove_paragraph(&mut self, paragraph: i32,) -> bool {
            self.remove_paragraph_ex(paragraph,) . done()
        }
        #[doc = "Removes a paragraph of content from the label. Returns `true` if the paragraph exists.\n\nThe `paragraph` argument is the index of the paragraph to remove, it can take values in the interval `[0, get_paragraph_count() - 1]`.\n\nIf `no_invalidate` is set to `true`, cache for the subsequent paragraphs is not invalidated. Use it for faster updates if deleted paragraph is fully self-contained (have no unclosed tags), or this call is part of the complex edit operation and [`invalidate_paragraph`][`crate::classes::RichTextLabel::invalidate_paragraph`] will be called at the end of operation."]
        #[inline]
        pub fn remove_paragraph_ex < 'ex > (&'ex mut self, paragraph: i32,) -> ExRemoveParagraph < 'ex > {
            ExRemoveParagraph::new(self, paragraph,)
        }
        #[doc = "Invalidates `paragraph` and all subsequent paragraphs cache."]
        pub fn invalidate_paragraph(&mut self, paragraph: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (paragraph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "invalidate_paragraph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[font]` tag to the tag stack. Overrides default fonts for its duration.\n\nPassing `0` to `font_size` will use the existing default font size."]
        pub(crate) fn push_font_full(&mut self, font: CowArg < Option < Gd < crate::classes::Font > > >, font_size: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >, i32,);
            let args = (font, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`push_font_ex`][Self::push_font_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a `[font]` tag to the tag stack. Overrides default fonts for its duration.\n\nPassing `0` to `font_size` will use the existing default font size."]
        #[inline]
        pub fn push_font(&mut self, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            self.push_font_ex(font,) . done()
        }
        #[doc = "Adds a `[font]` tag to the tag stack. Overrides default fonts for its duration.\n\nPassing `0` to `font_size` will use the existing default font size."]
        #[inline]
        pub fn push_font_ex < 'ex > (&'ex mut self, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'ex,) -> ExPushFont < 'ex > {
            ExPushFont::new(self, font,)
        }
        #[doc = "Adds a `[font_size]` tag to the tag stack. Overrides default font size for its duration."]
        pub fn push_font_size(&mut self, font_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[font]` tag with a normal font to the tag stack."]
        pub fn push_normal(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_normal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[font]` tag with a bold font to the tag stack. This is the same as adding a `[b]` tag if not currently in a `[i]` tag."]
        pub fn push_bold(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_bold", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[font]` tag with a bold italics font to the tag stack."]
        pub fn push_bold_italics(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_bold_italics", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[font]` tag with an italics font to the tag stack. This is the same as adding an `[i]` tag if not currently in a `[b]` tag."]
        pub fn push_italics(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_italics", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[font]` tag with a monospace font to the tag stack."]
        pub fn push_mono(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_mono", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[color]` tag to the tag stack."]
        pub fn push_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[outline_size]` tag to the tag stack. Overrides default text outline size for its duration."]
        pub fn push_outline_size(&mut self, outline_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (outline_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[outline_color]` tag to the tag stack. Adds text outline for its duration."]
        pub fn push_outline_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_outline_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[p]` tag to the tag stack."]
        pub(crate) fn push_paragraph_full(&mut self, alignment: crate::global::HorizontalAlignment, base_direction: crate::classes::control::TextDirection, language: CowArg < GString >, st_parser: crate::classes::text_server::StructuredTextParser, justification_flags: crate::classes::text_server::JustificationFlag, tab_stops: RefArg < PackedFloat32Array >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (crate::global::HorizontalAlignment, crate::classes::control::TextDirection, CowArg < 'a0, GString >, crate::classes::text_server::StructuredTextParser, crate::classes::text_server::JustificationFlag, RefArg < 'a1, PackedFloat32Array >,);
            let args = (alignment, base_direction, language, st_parser, justification_flags, tab_stops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_paragraph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`push_paragraph_ex`][Self::push_paragraph_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a `[p]` tag to the tag stack."]
        #[inline]
        pub fn push_paragraph(&mut self, alignment: crate::global::HorizontalAlignment,) {
            self.push_paragraph_ex(alignment,) . done()
        }
        #[doc = "Adds a `[p]` tag to the tag stack."]
        #[inline]
        pub fn push_paragraph_ex < 'ex > (&'ex mut self, alignment: crate::global::HorizontalAlignment,) -> ExPushParagraph < 'ex > {
            ExPushParagraph::new(self, alignment,)
        }
        #[doc = "Adds an `[indent]` tag to the tag stack. Multiplies `level` by current \\[member tab_size] to determine new margin length."]
        pub fn push_indent(&mut self, level: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_indent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds `[ol]` or `[ul]` tag to the tag stack. Multiplies `level` by current \\[member tab_size] to determine new margin length."]
        pub(crate) fn push_list_full(&mut self, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool, bullet: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, crate::classes::rich_text_label::ListType, bool, CowArg < 'a0, GString >,);
            let args = (level, type_, capitalize, bullet,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`push_list_ex`][Self::push_list_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds `[ol]` or `[ul]` tag to the tag stack. Multiplies `level` by current \\[member tab_size] to determine new margin length."]
        #[inline]
        pub fn push_list(&mut self, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool,) {
            self.push_list_ex(level, type_, capitalize,) . done()
        }
        #[doc = "Adds `[ol]` or `[ul]` tag to the tag stack. Multiplies `level` by current \\[member tab_size] to determine new margin length."]
        #[inline]
        pub fn push_list_ex < 'ex > (&'ex mut self, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool,) -> ExPushList < 'ex > {
            ExPushList::new(self, level, type_, capitalize,)
        }
        #[doc = "Adds a meta tag to the tag stack. Similar to the BBCode `[url=something]{text}[/url]`, but supports non-[`String`][crate::builtin::GString] metadata types.\n\nIf \\[member meta_underlined] is `true`, meta tags display an underline. This behavior can be customized with `underline_mode`.\n\n**Note:** Meta tags do nothing by default when clicked. To assign behavior when clicked, connect `meta_clicked` to a function that is called when the meta tag is clicked."]
        pub(crate) fn push_meta_full(&mut self, data: RefArg < Variant >, underline_mode: crate::classes::rich_text_label::MetaUnderline, tooltip: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, crate::classes::rich_text_label::MetaUnderline, CowArg < 'a1, GString >,);
            let args = (data, underline_mode, tooltip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`push_meta_ex`][Self::push_meta_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a meta tag to the tag stack. Similar to the BBCode `[url=something]{text}[/url]`, but supports non-[`String`][crate::builtin::GString] metadata types.\n\nIf \\[member meta_underlined] is `true`, meta tags display an underline. This behavior can be customized with `underline_mode`.\n\n**Note:** Meta tags do nothing by default when clicked. To assign behavior when clicked, connect `meta_clicked` to a function that is called when the meta tag is clicked."]
        #[inline]
        pub fn push_meta(&mut self, data: &Variant,) {
            self.push_meta_ex(data,) . done()
        }
        #[doc = "Adds a meta tag to the tag stack. Similar to the BBCode `[url=something]{text}[/url]`, but supports non-[`String`][crate::builtin::GString] metadata types.\n\nIf \\[member meta_underlined] is `true`, meta tags display an underline. This behavior can be customized with `underline_mode`.\n\n**Note:** Meta tags do nothing by default when clicked. To assign behavior when clicked, connect `meta_clicked` to a function that is called when the meta tag is clicked."]
        #[inline]
        pub fn push_meta_ex < 'ex > (&'ex mut self, data: &'ex Variant,) -> ExPushMeta < 'ex > {
            ExPushMeta::new(self, data,)
        }
        #[doc = "Adds a `[hint]` tag to the tag stack. Same as BBCode `[hint=something]{text}[/hint]`."]
        pub fn push_hint(&mut self, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_hint", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds language code used for text shaping algorithm and Open-Type font features."]
        pub fn push_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[u]` tag to the tag stack. If `color`'s alpha value is `0.0`, the current font's color with its alpha multiplied by [theme_item underline_alpha] is used."]
        pub(crate) fn push_underline_full(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_underline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`push_underline_ex`][Self::push_underline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a `[u]` tag to the tag stack. If `color`'s alpha value is `0.0`, the current font's color with its alpha multiplied by [theme_item underline_alpha] is used."]
        #[inline]
        pub fn push_underline(&mut self,) {
            self.push_underline_ex() . done()
        }
        #[doc = "Adds a `[u]` tag to the tag stack. If `color`'s alpha value is `0.0`, the current font's color with its alpha multiplied by [theme_item underline_alpha] is used."]
        #[inline]
        pub fn push_underline_ex < 'ex > (&'ex mut self,) -> ExPushUnderline < 'ex > {
            ExPushUnderline::new(self,)
        }
        #[doc = "Adds a `[s]` tag to the tag stack. If `color`'s alpha value is `0.0`, the current font's color with its alpha multiplied by [theme_item strikethrough_alpha] is used."]
        pub(crate) fn push_strikethrough_full(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_strikethrough", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`push_strikethrough_ex`][Self::push_strikethrough_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a `[s]` tag to the tag stack. If `color`'s alpha value is `0.0`, the current font's color with its alpha multiplied by [theme_item strikethrough_alpha] is used."]
        #[inline]
        pub fn push_strikethrough(&mut self,) {
            self.push_strikethrough_ex() . done()
        }
        #[doc = "Adds a `[s]` tag to the tag stack. If `color`'s alpha value is `0.0`, the current font's color with its alpha multiplied by [theme_item strikethrough_alpha] is used."]
        #[inline]
        pub fn push_strikethrough_ex < 'ex > (&'ex mut self,) -> ExPushStrikethrough < 'ex > {
            ExPushStrikethrough::new(self,)
        }
        #[doc = "Adds a `[table=columns,inline_align]` tag to the tag stack. Use [`set_table_column_expand`][`crate::classes::RichTextLabel::set_table_column_expand`] to set column expansion ratio. Use [`push_cell`][`crate::classes::RichTextLabel::push_cell`] to add cells. `name` is used as the table name for assistive apps."]
        pub(crate) fn push_table_full(&mut self, columns: i32, inline_align: crate::global::InlineAlignment, align_to_row: i32, name: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, crate::global::InlineAlignment, i32, CowArg < 'a0, GString >,);
            let args = (columns, inline_align, align_to_row, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_table", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`push_table_ex`][Self::push_table_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a `[table=columns,inline_align]` tag to the tag stack. Use [`set_table_column_expand`][`crate::classes::RichTextLabel::set_table_column_expand`] to set column expansion ratio. Use [`push_cell`][`crate::classes::RichTextLabel::push_cell`] to add cells. `name` is used as the table name for assistive apps."]
        #[inline]
        pub fn push_table(&mut self, columns: i32,) {
            self.push_table_ex(columns,) . done()
        }
        #[doc = "Adds a `[table=columns,inline_align]` tag to the tag stack. Use [`set_table_column_expand`][`crate::classes::RichTextLabel::set_table_column_expand`] to set column expansion ratio. Use [`push_cell`][`crate::classes::RichTextLabel::push_cell`] to add cells. `name` is used as the table name for assistive apps."]
        #[inline]
        pub fn push_table_ex < 'ex > (&'ex mut self, columns: i32,) -> ExPushTable < 'ex > {
            ExPushTable::new(self, columns,)
        }
        #[doc = "Adds a `[dropcap]` tag to the tag stack. Drop cap (dropped capital) is a decorative element at the beginning of a paragraph that is larger than the rest of the text."]
        pub(crate) fn push_dropcap_full(&mut self, string: CowArg < GString >, font: CowArg < Option < Gd < crate::classes::Font > > >, size: i32, dropcap_margins: Rect2, color: Color, outline_size: i32, outline_color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Font > > >, i32, Rect2, Color, i32, Color,);
            let args = (string, font, size, dropcap_margins, color, outline_size, outline_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_dropcap", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`push_dropcap_ex`][Self::push_dropcap_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a `[dropcap]` tag to the tag stack. Drop cap (dropped capital) is a decorative element at the beginning of a paragraph that is larger than the rest of the text."]
        #[inline]
        pub fn push_dropcap(&mut self, string: impl AsArg < GString >, font: impl AsArg < Option < Gd < crate::classes::Font >> >, size: i32,) {
            self.push_dropcap_ex(string, font, size,) . done()
        }
        #[doc = "Adds a `[dropcap]` tag to the tag stack. Drop cap (dropped capital) is a decorative element at the beginning of a paragraph that is larger than the rest of the text."]
        #[inline]
        pub fn push_dropcap_ex < 'ex > (&'ex mut self, string: impl AsArg < GString > + 'ex, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'ex, size: i32,) -> ExPushDropcap < 'ex > {
            ExPushDropcap::new(self, string, font, size,)
        }
        #[doc = "Edits the selected column's expansion options. If `expand` is `true`, the column expands in proportion to its expansion ratio versus the other columns' ratios.\n\nFor example, 2 columns with ratios 3 and 4 plus 70 pixels in available width would expand 30 and 40 pixels, respectively.\n\nIf `expand` is `false`, the column will not contribute to the total ratio."]
        pub(crate) fn set_table_column_expand_full(&mut self, column: i32, expand: bool, ratio: i32, shrink: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool, i32, bool,);
            let args = (column, expand, ratio, shrink,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_table_column_expand", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_table_column_expand_ex`][Self::set_table_column_expand_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Edits the selected column's expansion options. If `expand` is `true`, the column expands in proportion to its expansion ratio versus the other columns' ratios.\n\nFor example, 2 columns with ratios 3 and 4 plus 70 pixels in available width would expand 30 and 40 pixels, respectively.\n\nIf `expand` is `false`, the column will not contribute to the total ratio."]
        #[inline]
        pub fn set_table_column_expand(&mut self, column: i32, expand: bool,) {
            self.set_table_column_expand_ex(column, expand,) . done()
        }
        #[doc = "Edits the selected column's expansion options. If `expand` is `true`, the column expands in proportion to its expansion ratio versus the other columns' ratios.\n\nFor example, 2 columns with ratios 3 and 4 plus 70 pixels in available width would expand 30 and 40 pixels, respectively.\n\nIf `expand` is `false`, the column will not contribute to the total ratio."]
        #[inline]
        pub fn set_table_column_expand_ex < 'ex > (&'ex mut self, column: i32, expand: bool,) -> ExSetTableColumnExpand < 'ex > {
            ExSetTableColumnExpand::new(self, column, expand,)
        }
        #[doc = "Sets table column name for assistive apps."]
        pub fn set_table_column_name(&mut self, column: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_table_column_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets color of a table cell. Separate colors for alternating rows can be specified."]
        pub fn set_cell_row_background_color(&mut self, odd_row_bg: Color, even_row_bg: Color,) {
            type CallRet = ();
            type CallParams = (Color, Color,);
            let args = (odd_row_bg, even_row_bg,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_cell_row_background_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets color of a table cell border."]
        pub fn set_cell_border_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_cell_border_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets minimum and maximum size overrides for a table cell."]
        pub fn set_cell_size_override(&mut self, min_size: Vector2, max_size: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (min_size, max_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_cell_size_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets inner padding of a table cell."]
        pub fn set_cell_padding(&mut self, padding: Rect2,) {
            type CallRet = ();
            type CallParams = (Rect2,);
            let args = (padding,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_cell_padding", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[cell]` tag to the tag stack. Must be inside a `[table]` tag. See [`push_table`][`crate::classes::RichTextLabel::push_table`] for details. Use [`set_table_column_expand`][`crate::classes::RichTextLabel::set_table_column_expand`] to set column expansion ratio, [`set_cell_border_color`][`crate::classes::RichTextLabel::set_cell_border_color`] to set cell border, [`set_cell_row_background_color`][`crate::classes::RichTextLabel::set_cell_row_background_color`] to set cell background, [`set_cell_size_override`][`crate::classes::RichTextLabel::set_cell_size_override`] to override cell size, and [`set_cell_padding`][`crate::classes::RichTextLabel::set_cell_padding`] to set padding."]
        pub fn push_cell(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_cell", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[fgcolor]` tag to the tag stack.\n\n**Note:** The foreground color has padding applied by default, which is controlled using [theme_item text_highlight_h_padding] and [theme_item text_highlight_v_padding]. This can lead to overlapping highlights if foreground colors are placed on neighboring lines/columns, so consider setting those theme items to `0` if you want to avoid this."]
        pub fn push_fgcolor(&mut self, fgcolor: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (fgcolor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8718usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_fgcolor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a `[bgcolor]` tag to the tag stack.\n\n**Note:** The background color has padding applied by default, which is controlled using [theme_item text_highlight_h_padding] and [theme_item text_highlight_v_padding]. This can lead to overlapping highlights if background colors are placed on neighboring lines/columns, so consider setting those theme items to `0` if you want to avoid this."]
        pub fn push_bgcolor(&mut self, bgcolor: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (bgcolor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_bgcolor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a custom effect tag to the tag stack. The effect does not need to be in \\[member custom_effects]. The environment is directly passed to the effect."]
        pub fn push_customfx(&mut self, effect: impl AsArg < Option < Gd < crate::classes::RichTextEffect >> >, env: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::RichTextEffect > > >, RefArg < 'a1, AnyDictionary >,);
            let args = (effect.into_arg(), RefArg::new(env),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8720usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_customfx", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a context marker to the tag stack. See [`pop_context`][`crate::classes::RichTextLabel::pop_context`]."]
        pub fn push_context(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_context", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Terminates tags opened after the last [`push_context`][`crate::classes::RichTextLabel::push_context`] call (including context marker), or all tags if there's no context marker on the stack."]
        pub fn pop_context(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8722usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "pop_context", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Terminates the current tag. Use after `push_*` methods to close BBCodes manually. Does not need to follow `add_*` methods."]
        pub fn pop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8723usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "pop", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Terminates all tags opened by `push_*` methods."]
        pub fn pop_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "pop_all", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears the tag stack, causing the label to display nothing.\n\n**Note:** This method does not affect \\[member text], and its contents will show again if the label is redrawn. However, setting \\[member text] to an empty [`String`][crate::builtin::GString] also clears the stack."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::StructuredTextParser,);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8726usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallRet = crate::classes::text_server::StructuredTextParser;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8727usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8728usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8729usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::TextDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8730usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::HorizontalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8734usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8735usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertical_alignment(&mut self, alignment: crate::global::VerticalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::VerticalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8736usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_vertical_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertical_alignment(&self,) -> crate::global::VerticalAlignment {
            type CallRet = crate::global::VerticalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8737usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_vertical_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_justification_flags(&mut self, justification_flags: crate::classes::text_server::JustificationFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::JustificationFlag,);
            let args = (justification_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8738usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_justification_flags(&self,) -> crate::classes::text_server::JustificationFlag {
            type CallRet = crate::classes::text_server::JustificationFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8739usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_stops(&mut self, tab_stops: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat32Array >,);
            let args = (RefArg::new(tab_stops),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8740usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_tab_stops", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_stops(&self,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_tab_stops", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::AutowrapMode,);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::classes::text_server::AutowrapMode {
            type CallRet = crate::classes::text_server::AutowrapMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8743usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_trim_flags(&mut self, autowrap_trim_flags: crate::classes::text_server::LineBreakFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::LineBreakFlag,);
            let args = (autowrap_trim_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8744usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_autowrap_trim_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_trim_flags(&self,) -> crate::classes::text_server::LineBreakFlag {
            type CallRet = crate::classes::text_server::LineBreakFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8745usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_autowrap_trim_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_meta_underline(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8746usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_meta_underline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_meta_underlined(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8747usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_meta_underlined", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hint_underline(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8748usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_hint_underline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_hint_underlined(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8749usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_hint_underlined", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scroll_active(&mut self, active: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8750usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_scroll_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scroll_active(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8751usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_scroll_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scroll_follow_visible_characters(&mut self, follow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (follow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8752usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_scroll_follow_visible_characters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scroll_following_visible_characters(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_scroll_following_visible_characters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scroll_follow(&mut self, follow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (follow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_scroll_follow", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scroll_following(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_scroll_following", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the vertical scrollbar.\n\n**Warning:** This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their \\[member CanvasItem.visible] property."]
        pub fn get_v_scroll_bar(&self,) -> Option < Gd < crate::classes::VScrollBar > > {
            type CallRet = Option < Gd < crate::classes::VScrollBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_v_scroll_bar", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Scrolls the window's top line to match `line`."]
        pub fn scroll_to_line(&mut self, line: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "scroll_to_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Scrolls the window's top line to match first line of the `paragraph`."]
        pub fn scroll_to_paragraph(&mut self, paragraph: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (paragraph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "scroll_to_paragraph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Scrolls to the beginning of the current selection."]
        pub fn scroll_to_selection(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "scroll_to_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_size(&mut self, spaces: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (spaces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_tab_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_tab_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fit_content(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_fit_content", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_fit_content_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_fit_content_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_selection_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_selection_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_context_menu_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_context_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_context_menu_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_context_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shortcut_keys_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_shortcut_keys_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_shortcut_keys_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_shortcut_keys_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_deselect_on_focus_loss_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_deselect_on_focus_loss_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_deselect_on_focus_loss_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_deselect_on_focus_loss_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_and_drop_selection_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_drag_and_drop_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drag_and_drop_selection_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_drag_and_drop_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current selection first character index if a selection is active, `-1` otherwise. Does not include BBCodes."]
        pub fn get_selection_from(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_selection_from", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current selection last character index if a selection is active, `-1` otherwise. Does not include BBCodes."]
        pub fn get_selection_to(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_selection_to", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current selection vertical line offset if a selection is active, `-1.0` otherwise."]
        pub fn get_selection_line_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_selection_line_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Select all the text.\n\nIf \\[member selection_enabled] is `false`, no selection will occur."]
        pub fn select_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "select_all", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current selection text. Does not include BBCodes."]
        pub fn get_selected_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_selected_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears the current selection."]
        pub fn deselect(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "deselect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "The assignment version of [`append_text`][`crate::classes::RichTextLabel::append_text`]. Clears the tag stack and inserts the new content."]
        pub fn parse_bbcode(&mut self, bbcode: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (bbcode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "parse_bbcode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Parses `bbcode` and adds tags to the tag stack as needed.\n\n**Note:** Using this method, you can't close a tag that was opened in a previous [`append_text`][`crate::classes::RichTextLabel::append_text`] call. This is done to improve performance, especially when updating large RichTextLabels since rebuilding the whole BBCode every time would be slower. If you absolutely need to close a tag in a future method call, append the \\[member text] instead of using [`append_text`][`crate::classes::RichTextLabel::append_text`]."]
        pub fn append_text(&mut self, bbcode: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (bbcode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "append_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If \\[member threaded] is enabled, returns `true` if the background thread has finished text processing, otherwise always return `true`."]
        pub fn is_ready(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_ready", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If \\[member threaded] is enabled, returns `true` if the background thread has finished text processing, otherwise always return `true`."]
        pub fn is_finished(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_finished", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_threaded(&mut self, threaded: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (threaded,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_threaded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_threaded(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_threaded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_progress_bar_delay(&mut self, delay_ms: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (delay_ms,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_progress_bar_delay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_progress_bar_delay(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_progress_bar_delay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_characters(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_visible_characters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_characters(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_characters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_characters_behavior(&self,) -> crate::classes::text_server::VisibleCharactersBehavior {
            type CallRet = crate::classes::text_server::VisibleCharactersBehavior;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_characters_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_characters_behavior(&mut self, behavior: crate::classes::text_server::VisibleCharactersBehavior,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::VisibleCharactersBehavior,);
            let args = (behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_visible_characters_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_visible_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_ratio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the line number of the character position provided. Line and character numbers are both zero-indexed.\n\n**Note:** If \\[member threaded] is enabled, this method returns a value for the loaded part of the document. Use [`is_finished`][`crate::classes::RichTextLabel::is_finished`] or `finished` to determine whether document is fully loaded."]
        pub fn get_character_line(&self, character: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (character,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_character_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the paragraph number of the character position provided. Paragraph and character numbers are both zero-indexed.\n\n**Note:** If \\[member threaded] is enabled, this method returns a value for the loaded part of the document. Use [`is_finished`][`crate::classes::RichTextLabel::is_finished`] or `finished` to determine whether document is fully loaded."]
        pub fn get_character_paragraph(&self, character: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (character,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_character_paragraph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total number of characters from text tags. Does not include BBCodes."]
        pub fn get_total_character_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_total_character_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_bbcode(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_use_bbcode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_bbcode(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_using_bbcode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total number of lines in the text. Wrapped text is counted as multiple lines.\n\n**Note:** If \\[member threaded] is enabled, this method returns a value for the loaded part of the document. Use [`is_finished`][`crate::classes::RichTextLabel::is_finished`] or `finished` to determine whether document is fully loaded."]
        pub fn get_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_line_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the indexes of the first and last visible characters for the given `line`, as a [`Vector2i`][crate::builtin::Vector2i].\n\n**Note:** If \\[member visible_characters_behavior] is set to [`VisibleCharactersBehavior::CHARS_BEFORE_SHAPING`][`crate::classes::text_server::VisibleCharactersBehavior::CHARS_BEFORE_SHAPING`] only visible wrapped lines are counted.\n\n**Note:** If \\[member threaded] is enabled, this method returns a value for the loaded part of the document. Use [`is_finished`][`crate::classes::RichTextLabel::is_finished`] or `finished` to determine whether document is fully loaded."]
        pub fn get_line_range(&self, line: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_line_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of visible lines.\n\n**Note:** This method returns a correct value only after the label has been drawn.\n\n**Note:** If \\[member threaded] is enabled, this method returns a value for the loaded part of the document. Use [`is_finished`][`crate::classes::RichTextLabel::is_finished`] or `finished` to determine whether document is fully loaded."]
        pub fn get_visible_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_line_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total number of paragraphs (newlines or `p` tags in the tag stack's text tags). Considers wrapped text as one paragraph."]
        pub fn get_paragraph_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_paragraph_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of visible paragraphs. A paragraph is considered visible if at least one of its lines is visible.\n\n**Note:** This method returns a correct value only after the label has been drawn.\n\n**Note:** If \\[member threaded] is enabled, this method returns a value for the loaded part of the document. Use [`is_finished`][`crate::classes::RichTextLabel::is_finished`] or `finished` to determine whether document is fully loaded."]
        pub fn get_visible_paragraph_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_paragraph_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the height of the content.\n\n**Note:** This method always returns the full content size, and is not affected by \\[member visible_ratio] and \\[member visible_characters]. To get the visible content size, use [`get_visible_content_rect`][`crate::classes::RichTextLabel::get_visible_content_rect`].\n\n**Note:** If \\[member threaded] is enabled, this method returns a value for the loaded part of the document. Use [`is_finished`][`crate::classes::RichTextLabel::is_finished`] or `finished` to determine whether document is fully loaded."]
        pub fn get_content_height(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_content_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the width of the content.\n\n**Note:** This method always returns the full content size, and is not affected by \\[member visible_ratio] and \\[member visible_characters]. To get the visible content size, use [`get_visible_content_rect`][`crate::classes::RichTextLabel::get_visible_content_rect`].\n\n**Note:** If \\[member threaded] is enabled, this method returns a value for the loaded part of the document. Use [`is_finished`][`crate::classes::RichTextLabel::is_finished`] or `finished` to determine whether document is fully loaded."]
        pub fn get_content_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8806usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_content_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the height of the line found at the provided index.\n\n**Note:** If \\[member threaded] is enabled, this method returns a value for the loaded part of the document. Use [`is_finished`][`crate::classes::RichTextLabel::is_finished`] or `finished` to determine whether the document is fully loaded."]
        pub fn get_line_height(&self, line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8807usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_line_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the width of the line found at the provided index.\n\n**Note:** If \\[member threaded] is enabled, this method returns a value for the loaded part of the document. Use [`is_finished`][`crate::classes::RichTextLabel::is_finished`] or `finished` to determine whether the document is fully loaded."]
        pub fn get_line_width(&self, line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8808usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_line_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the bounding rectangle of the visible content.\n\n**Note:** This method returns a correct value only after the label has been drawn.\n\n\n```gdscript\nextends RichTextLabel\n\n@export var background_panel: Panel\n\nfunc _ready():\n\tawait draw\n\tbackground_panel.position = get_visible_content_rect().position\n\tbackground_panel.size = get_visible_content_rect().size\n```\n"]
        pub fn get_visible_content_rect(&self,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8809usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_content_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the vertical offset of the line found at the provided index.\n\n**Note:** If \\[member threaded] is enabled, this method returns a value for the loaded part of the document. Use [`is_finished`][`crate::classes::RichTextLabel::is_finished`] or `finished` to determine whether document is fully loaded."]
        pub fn get_line_offset(&self, line: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_line_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the vertical offset of the paragraph found at the provided index.\n\n**Note:** If \\[member threaded] is enabled, this method returns a value for the loaded part of the document. Use [`is_finished`][`crate::classes::RichTextLabel::is_finished`] or `finished` to determine whether document is fully loaded."]
        pub fn get_paragraph_offset(&self, paragraph: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (paragraph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_paragraph_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Parses BBCode parameter `expressions` into a dictionary."]
        pub fn parse_expressions_for_values(&mut self, expressions: &PackedStringArray,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(expressions),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8812usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "parse_expressions_for_values", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_effects(&mut self, effects: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyArray >,);
            let args = (RefArg::new(effects),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8813usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_effects", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_effects(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8814usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_effects", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Installs a custom effect. This can also be done in the Inspector through the \\[member custom_effects] property. `effect` should be a valid [`RichTextEffect`][crate::classes::RichTextEffect].\n\n**Example:** With the following script extending from [`RichTextEffect`][crate::classes::RichTextEffect]:\n\n```gdscript\n# effect.gd\nclass_name MyCustomEffect\nextends RichTextEffect\n\nvar bbcode = \"my_custom_effect\"\n\n# ...\n```\n\nThe above effect can be installed in `RichTextLabel` from a script:\n\n```gdscript\n# rich_text_label.gd\nextends RichTextLabel\n\nfunc _ready():\n\tinstall_effect(MyCustomEffect.new())\n\n\t# Alternatively, if not using `class_name` in the script that extends RichTextEffect:\n\tinstall_effect(preload(\"res://effect.gd\").new())\n```"]
        pub fn install_effect(&mut self, effect: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (RefArg::new(effect),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8815usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "install_effect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Reloads custom effects. Useful when \\[member custom_effects] is modified manually."]
        pub fn reload_effects(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8816usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "reload_effects", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`PopupMenu`][crate::classes::PopupMenu] of this `RichTextLabel`. By default, this menu is displayed when right-clicking on the `RichTextLabel`.\n\nYou can add custom menu items or remove standard ones. Make sure your IDs don't conflict with the standard ones (see \\[enum MenuItems]). For example:\n\n\n```gdscript\nfunc _ready():\n\tvar menu = get_menu()\n\t# Remove \"Select All\" item.\n\tmenu.remove_item(MENU_SELECT_ALL)\n\t# Add custom items.\n\tmenu.add_separator()\n\tmenu.add_item(\"Duplicate Text\", MENU_MAX + 1)\n\t# Connect callback.\n\tmenu.id_pressed.connect(_on_item_pressed)\n\nfunc _on_item_pressed(id):\n\tif id == MENU_MAX + 1:\n\t\tadd_text(\"\\n\" + get_parsed_text())\n```\n\n\n**Warning:** This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their \\[member Window.visible] property."]
        pub fn get_menu(&self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallRet = Option < Gd < crate::classes::PopupMenu > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether the menu is visible. Use this instead of `get_menu().visible` to improve performance (so the creation of the menu is avoided)."]
        pub fn is_menu_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_menu_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Executes a given action as defined in the \\[enum MenuItems] enum."]
        pub fn menu_option(&mut self, option: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "menu_option", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for RichTextLabel {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RichTextLabel"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RichTextLabel {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for RichTextLabel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for RichTextLabel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for RichTextLabel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RichTextLabel {
        
    }
    impl crate::obj::cap::GodotDefault for RichTextLabel {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RichTextLabel {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RichTextLabel {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RichTextLabel`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RichTextLabel__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RichTextLabel > for $Class {
                
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
#[doc = "Default-param extender for [`RichTextLabel::add_hr_ex`][super::RichTextLabel::add_hr_ex]."]
#[must_use]
pub struct ExAddHr < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, width: i32, height: i32, color: Color, alignment: crate::global::HorizontalAlignment, width_in_percent: bool, height_in_percent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddHr < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel,) -> Self {
        let width = 90i32;
        let height = 2i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let alignment = crate::obj::EngineEnum::from_ord(1);
        let width_in_percent = true;
        let height_in_percent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, width: width, height: height, color: color, alignment: alignment, width_in_percent: width_in_percent, height_in_percent: height_in_percent,
        }
    }
    #[inline]
    pub fn width(self, width: i32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn height(self, height: i32) -> Self {
        Self {
            height: height, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width_in_percent(self, width_in_percent: bool) -> Self {
        Self {
            width_in_percent: width_in_percent, .. self
        }
    }
    #[inline]
    pub fn height_in_percent(self, height_in_percent: bool) -> Self {
        Self {
            height_in_percent: height_in_percent, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, width, height, color, alignment, width_in_percent, height_in_percent,
        }
        = self;
        re_export::RichTextLabel::add_hr_full(surround_object, width, height, color, alignment, width_in_percent, height_in_percent,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::add_image_ex`][super::RichTextLabel::add_image_ex]."]
#[must_use]
pub struct ExAddImage < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, image: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, width: i32, height: i32, color: Color, inline_align: crate::global::InlineAlignment, region: Rect2, key: CowArg < 'ex, Variant >, pad: bool, tooltip: CowArg < 'ex, GString >, width_in_percent: bool, height_in_percent: bool, alt_text: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddImage < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel, image: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> Self {
        let width = 0i32;
        let height = 0i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let region = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let key = Variant::nil();
        let pad = false;
        let tooltip = GString::from("");
        let width_in_percent = false;
        let height_in_percent = false;
        let alt_text = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, image: image.into_arg(), width: width, height: height, color: color, inline_align: inline_align, region: region, key: CowArg::Owned(key), pad: pad, tooltip: CowArg::Owned(tooltip), width_in_percent: width_in_percent, height_in_percent: height_in_percent, alt_text: CowArg::Owned(alt_text),
        }
    }
    #[inline]
    pub fn width(self, width: i32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn height(self, height: i32) -> Self {
        Self {
            height: height, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn region(self, region: Rect2) -> Self {
        Self {
            region: region, .. self
        }
    }
    #[inline]
    pub fn key(self, key: &'ex Variant) -> Self {
        Self {
            key: CowArg::Borrowed(key), .. self
        }
    }
    #[inline]
    pub fn pad(self, pad: bool) -> Self {
        Self {
            pad: pad, .. self
        }
    }
    #[inline]
    pub fn tooltip(self, tooltip: impl AsArg < GString > + 'ex) -> Self {
        Self {
            tooltip: tooltip.into_arg(), .. self
        }
    }
    #[inline]
    pub fn width_in_percent(self, width_in_percent: bool) -> Self {
        Self {
            width_in_percent: width_in_percent, .. self
        }
    }
    #[inline]
    pub fn height_in_percent(self, height_in_percent: bool) -> Self {
        Self {
            height_in_percent: height_in_percent, .. self
        }
    }
    #[inline]
    pub fn alt_text(self, alt_text: impl AsArg < GString > + 'ex) -> Self {
        Self {
            alt_text: alt_text.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, image, width, height, color, inline_align, region, key, pad, tooltip, width_in_percent, height_in_percent, alt_text,
        }
        = self;
        re_export::RichTextLabel::add_image_full(surround_object, image, width, height, color, inline_align, region, key.cow_as_arg(), pad, tooltip, width_in_percent, height_in_percent, alt_text,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::update_image_ex`][super::RichTextLabel::update_image_ex]."]
#[must_use]
pub struct ExUpdateImage < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, key: CowArg < 'ex, Variant >, mask: crate::classes::rich_text_label::ImageUpdateMask, image: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, width: i32, height: i32, color: Color, inline_align: crate::global::InlineAlignment, region: Rect2, pad: bool, tooltip: CowArg < 'ex, GString >, width_in_percent: bool, height_in_percent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExUpdateImage < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel, key: &'ex Variant, mask: crate::classes::rich_text_label::ImageUpdateMask, image: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> Self {
        let width = 0i32;
        let height = 0i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let region = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let pad = false;
        let tooltip = GString::from("");
        let width_in_percent = false;
        let height_in_percent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, key: CowArg::Borrowed(key), mask: mask, image: image.into_arg(), width: width, height: height, color: color, inline_align: inline_align, region: region, pad: pad, tooltip: CowArg::Owned(tooltip), width_in_percent: width_in_percent, height_in_percent: height_in_percent,
        }
    }
    #[inline]
    pub fn width(self, width: i32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn height(self, height: i32) -> Self {
        Self {
            height: height, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn region(self, region: Rect2) -> Self {
        Self {
            region: region, .. self
        }
    }
    #[inline]
    pub fn pad(self, pad: bool) -> Self {
        Self {
            pad: pad, .. self
        }
    }
    #[inline]
    pub fn tooltip(self, tooltip: impl AsArg < GString > + 'ex) -> Self {
        Self {
            tooltip: tooltip.into_arg(), .. self
        }
    }
    #[inline]
    pub fn width_in_percent(self, width_in_percent: bool) -> Self {
        Self {
            width_in_percent: width_in_percent, .. self
        }
    }
    #[inline]
    pub fn height_in_percent(self, height_in_percent: bool) -> Self {
        Self {
            height_in_percent: height_in_percent, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, key, mask, image, width, height, color, inline_align, region, pad, tooltip, width_in_percent, height_in_percent,
        }
        = self;
        re_export::RichTextLabel::update_image_full(surround_object, key.cow_as_arg(), mask, image, width, height, color, inline_align, region, pad, tooltip, width_in_percent, height_in_percent,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::remove_paragraph_ex`][super::RichTextLabel::remove_paragraph_ex]."]
#[must_use]
pub struct ExRemoveParagraph < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, paragraph: i32, no_invalidate: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExRemoveParagraph < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel, paragraph: i32,) -> Self {
        let no_invalidate = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, paragraph: paragraph, no_invalidate: no_invalidate,
        }
    }
    #[inline]
    pub fn no_invalidate(self, no_invalidate: bool) -> Self {
        Self {
            no_invalidate: no_invalidate, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, paragraph, no_invalidate,
        }
        = self;
        re_export::RichTextLabel::remove_paragraph_full(surround_object, paragraph, no_invalidate,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_font_ex`][super::RichTextLabel::push_font_ex]."]
#[must_use]
pub struct ExPushFont < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, font: CowArg < 'ex, Option < Gd < crate::classes::Font > > >, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPushFont < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'ex,) -> Self {
        let font_size = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, font_size,
        }
        = self;
        re_export::RichTextLabel::push_font_full(surround_object, font, font_size,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_paragraph_ex`][super::RichTextLabel::push_paragraph_ex]."]
#[must_use]
pub struct ExPushParagraph < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, alignment: crate::global::HorizontalAlignment, base_direction: crate::classes::control::TextDirection, language: CowArg < 'ex, GString >, st_parser: crate::classes::text_server::StructuredTextParser, justification_flags: crate::classes::text_server::JustificationFlag, tab_stops: CowArg < 'ex, PackedFloat32Array >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPushParagraph < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel, alignment: crate::global::HorizontalAlignment,) -> Self {
        let base_direction = crate::obj::EngineEnum::from_ord(0);
        let language = GString::from("");
        let st_parser = crate::obj::EngineEnum::from_ord(0);
        let justification_flags = crate::obj::EngineBitfield::from_ord(163);
        let tab_stops = PackedFloat32Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, alignment: alignment, base_direction: base_direction, language: CowArg::Owned(language), st_parser: st_parser, justification_flags: justification_flags, tab_stops: CowArg::Owned(tab_stops),
        }
    }
    #[inline]
    pub fn base_direction(self, base_direction: crate::classes::control::TextDirection) -> Self {
        Self {
            base_direction: base_direction, .. self
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'ex) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn st_parser(self, st_parser: crate::classes::text_server::StructuredTextParser) -> Self {
        Self {
            st_parser: st_parser, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn tab_stops(self, tab_stops: &'ex PackedFloat32Array) -> Self {
        Self {
            tab_stops: CowArg::Borrowed(tab_stops), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, alignment, base_direction, language, st_parser, justification_flags, tab_stops,
        }
        = self;
        re_export::RichTextLabel::push_paragraph_full(surround_object, alignment, base_direction, language, st_parser, justification_flags, tab_stops.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_list_ex`][super::RichTextLabel::push_list_ex]."]
#[must_use]
pub struct ExPushList < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool, bullet: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPushList < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool,) -> Self {
        let bullet = GString::from("•");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, level: level, type_: type_, capitalize: capitalize, bullet: CowArg::Owned(bullet),
        }
    }
    #[inline]
    pub fn bullet(self, bullet: impl AsArg < GString > + 'ex) -> Self {
        Self {
            bullet: bullet.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, level, type_, capitalize, bullet,
        }
        = self;
        re_export::RichTextLabel::push_list_full(surround_object, level, type_, capitalize, bullet,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_meta_ex`][super::RichTextLabel::push_meta_ex]."]
#[must_use]
pub struct ExPushMeta < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, data: CowArg < 'ex, Variant >, underline_mode: crate::classes::rich_text_label::MetaUnderline, tooltip: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPushMeta < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel, data: &'ex Variant,) -> Self {
        let underline_mode = crate::obj::EngineEnum::from_ord(1);
        let tooltip = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, data: CowArg::Borrowed(data), underline_mode: underline_mode, tooltip: CowArg::Owned(tooltip),
        }
    }
    #[inline]
    pub fn underline_mode(self, underline_mode: crate::classes::rich_text_label::MetaUnderline) -> Self {
        Self {
            underline_mode: underline_mode, .. self
        }
    }
    #[inline]
    pub fn tooltip(self, tooltip: impl AsArg < GString > + 'ex) -> Self {
        Self {
            tooltip: tooltip.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, data, underline_mode, tooltip,
        }
        = self;
        re_export::RichTextLabel::push_meta_full(surround_object, data.cow_as_arg(), underline_mode, tooltip,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_underline_ex`][super::RichTextLabel::push_underline_ex]."]
#[must_use]
pub struct ExPushUnderline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPushUnderline < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel,) -> Self {
        let color = Color::from_rgba(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, color: color,
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, color,
        }
        = self;
        re_export::RichTextLabel::push_underline_full(surround_object, color,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_strikethrough_ex`][super::RichTextLabel::push_strikethrough_ex]."]
#[must_use]
pub struct ExPushStrikethrough < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPushStrikethrough < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel,) -> Self {
        let color = Color::from_rgba(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, color: color,
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, color,
        }
        = self;
        re_export::RichTextLabel::push_strikethrough_full(surround_object, color,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_table_ex`][super::RichTextLabel::push_table_ex]."]
#[must_use]
pub struct ExPushTable < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, columns: i32, inline_align: crate::global::InlineAlignment, align_to_row: i32, name: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPushTable < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel, columns: i32,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(0);
        let align_to_row = - 1i32;
        let name = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, columns: columns, inline_align: inline_align, align_to_row: align_to_row, name: CowArg::Owned(name),
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn align_to_row(self, align_to_row: i32) -> Self {
        Self {
            align_to_row: align_to_row, .. self
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < GString > + 'ex) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, columns, inline_align, align_to_row, name,
        }
        = self;
        re_export::RichTextLabel::push_table_full(surround_object, columns, inline_align, align_to_row, name,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_dropcap_ex`][super::RichTextLabel::push_dropcap_ex]."]
#[must_use]
pub struct ExPushDropcap < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, string: CowArg < 'ex, GString >, font: CowArg < 'ex, Option < Gd < crate::classes::Font > > >, size: i32, dropcap_margins: Rect2, color: Color, outline_size: i32, outline_color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPushDropcap < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel, string: impl AsArg < GString > + 'ex, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'ex, size: i32,) -> Self {
        let dropcap_margins = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let outline_size = 0i32;
        let outline_color = Color::from_rgba(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), font: font.into_arg(), size: size, dropcap_margins: dropcap_margins, color: color, outline_size: outline_size, outline_color: outline_color,
        }
    }
    #[inline]
    pub fn dropcap_margins(self, dropcap_margins: Rect2) -> Self {
        Self {
            dropcap_margins: dropcap_margins, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i32) -> Self {
        Self {
            outline_size: outline_size, .. self
        }
    }
    #[inline]
    pub fn outline_color(self, outline_color: Color) -> Self {
        Self {
            outline_color: outline_color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, string, font, size, dropcap_margins, color, outline_size, outline_color,
        }
        = self;
        re_export::RichTextLabel::push_dropcap_full(surround_object, string, font, size, dropcap_margins, color, outline_size, outline_color,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::set_table_column_expand_ex`][super::RichTextLabel::set_table_column_expand_ex]."]
#[must_use]
pub struct ExSetTableColumnExpand < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RichTextLabel, column: i32, expand: bool, ratio: i32, shrink: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetTableColumnExpand < 'ex > {
    fn new(surround_object: &'ex mut re_export::RichTextLabel, column: i32, expand: bool,) -> Self {
        let ratio = 1i32;
        let shrink = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, expand: expand, ratio: ratio, shrink: shrink,
        }
    }
    #[inline]
    pub fn ratio(self, ratio: i32) -> Self {
        Self {
            ratio: ratio, .. self
        }
    }
    #[inline]
    pub fn shrink(self, shrink: bool) -> Self {
        Self {
            shrink: shrink, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, expand, ratio, shrink,
        }
        = self;
        re_export::RichTextLabel::set_table_column_expand_full(surround_object, column, expand, ratio, shrink,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ListType {
    ord: i32
}
impl ListType {
    #[doc(alias = "LIST_NUMBERS")]
    #[doc = "Godot enumerator name: `LIST_NUMBERS`"]
    pub const NUMBERS: ListType = ListType {
        ord: 0i32
    };
    #[doc(alias = "LIST_LETTERS")]
    #[doc = "Godot enumerator name: `LIST_LETTERS`"]
    pub const LETTERS: ListType = ListType {
        ord: 1i32
    };
    #[doc(alias = "LIST_ROMAN")]
    #[doc = "Godot enumerator name: `LIST_ROMAN`"]
    pub const ROMAN: ListType = ListType {
        ord: 2i32
    };
    #[doc(alias = "LIST_DOTS")]
    #[doc = "Godot enumerator name: `LIST_DOTS`"]
    pub const DOTS: ListType = ListType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ListType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ListType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ListType {
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
            Self::NUMBERS => "NUMBERS", Self::LETTERS => "LETTERS", Self::ROMAN => "ROMAN", Self::DOTS => "DOTS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ListType::NUMBERS, ListType::LETTERS, ListType::ROMAN, ListType::DOTS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ListType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NUMBERS", "LIST_NUMBERS", ListType::NUMBERS), crate::meta::inspect::EnumConstant::new("LETTERS", "LIST_LETTERS", ListType::LETTERS), crate::meta::inspect::EnumConstant::new("ROMAN", "LIST_ROMAN", ListType::ROMAN), crate::meta::inspect::EnumConstant::new("DOTS", "LIST_DOTS", ListType::DOTS)]
        }
    }
}
impl crate::meta::GodotConvert for ListType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("List Numbers", 0i64), EnumeratorShape::new_int("List Letters", 1i64), EnumeratorShape::new_int("List Roman", 2i64), EnumeratorShape::new_int("List Dots", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RichTextLabel.ListType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ListType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ListType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ListType {
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
impl crate::registry::property::Export for ListType {
    
}
impl crate::meta::Element for ListType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MenuItems {
    ord: i32
}
impl MenuItems {
    #[doc(alias = "MENU_COPY")]
    #[doc = "Godot enumerator name: `MENU_COPY`"]
    pub const COPY: MenuItems = MenuItems {
        ord: 0i32
    };
    #[doc(alias = "MENU_SELECT_ALL")]
    #[doc = "Godot enumerator name: `MENU_SELECT_ALL`"]
    pub const SELECT_ALL: MenuItems = MenuItems {
        ord: 1i32
    };
    #[doc(alias = "MENU_MAX")]
    #[doc = "Godot enumerator name: `MENU_MAX`"]
    pub const MAX: MenuItems = MenuItems {
        ord: 2i32
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
            Self::COPY => "COPY", Self::SELECT_ALL => "SELECT_ALL", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MenuItems::COPY, MenuItems::SELECT_ALL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MenuItems >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("COPY", "MENU_COPY", MenuItems::COPY), crate::meta::inspect::EnumConstant::new("SELECT_ALL", "MENU_SELECT_ALL", MenuItems::SELECT_ALL), crate::meta::inspect::EnumConstant::new("MAX", "MENU_MAX", MenuItems::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for MenuItems {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for MenuItems {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Menu Copy", 0i64), EnumeratorShape::new_int("Menu Select All", 1i64), EnumeratorShape::new_int("Menu Max", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RichTextLabel.MenuItems")), is_bitfield: false,
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
pub struct MetaUnderline {
    ord: i32
}
impl MetaUnderline {
    #[doc(alias = "META_UNDERLINE_NEVER")]
    #[doc = "Godot enumerator name: `META_UNDERLINE_NEVER`"]
    pub const NEVER: MetaUnderline = MetaUnderline {
        ord: 0i32
    };
    #[doc(alias = "META_UNDERLINE_ALWAYS")]
    #[doc = "Godot enumerator name: `META_UNDERLINE_ALWAYS`"]
    pub const ALWAYS: MetaUnderline = MetaUnderline {
        ord: 1i32
    };
    #[doc(alias = "META_UNDERLINE_ON_HOVER")]
    #[doc = "Godot enumerator name: `META_UNDERLINE_ON_HOVER`"]
    pub const ON_HOVER: MetaUnderline = MetaUnderline {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MetaUnderline {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MetaUnderline") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MetaUnderline {
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
            Self::NEVER => "NEVER", Self::ALWAYS => "ALWAYS", Self::ON_HOVER => "ON_HOVER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MetaUnderline::NEVER, MetaUnderline::ALWAYS, MetaUnderline::ON_HOVER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MetaUnderline >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEVER", "META_UNDERLINE_NEVER", MetaUnderline::NEVER), crate::meta::inspect::EnumConstant::new("ALWAYS", "META_UNDERLINE_ALWAYS", MetaUnderline::ALWAYS), crate::meta::inspect::EnumConstant::new("ON_HOVER", "META_UNDERLINE_ON_HOVER", MetaUnderline::ON_HOVER)]
        }
    }
}
impl crate::meta::GodotConvert for MetaUnderline {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Meta Underline Never", 0i64), EnumeratorShape::new_int("Meta Underline Always", 1i64), EnumeratorShape::new_int("Meta Underline On Hover", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RichTextLabel.MetaUnderline")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for MetaUnderline {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MetaUnderline {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for MetaUnderline {
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
impl crate::registry::property::Export for MetaUnderline {
    
}
impl crate::meta::Element for MetaUnderline {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ImageUpdateMask {
    ord: u64
}
impl ImageUpdateMask {
    #[doc(alias = "UPDATE_TEXTURE")]
    #[doc = "Godot enumerator name: `UPDATE_TEXTURE`"]
    pub const TEXTURE: ImageUpdateMask = ImageUpdateMask {
        ord: 1u64
    };
    #[doc(alias = "UPDATE_SIZE")]
    #[doc = "Godot enumerator name: `UPDATE_SIZE`"]
    pub const SIZE: ImageUpdateMask = ImageUpdateMask {
        ord: 2u64
    };
    #[doc(alias = "UPDATE_COLOR")]
    #[doc = "Godot enumerator name: `UPDATE_COLOR`"]
    pub const COLOR: ImageUpdateMask = ImageUpdateMask {
        ord: 4u64
    };
    #[doc(alias = "UPDATE_ALIGNMENT")]
    #[doc = "Godot enumerator name: `UPDATE_ALIGNMENT`"]
    pub const ALIGNMENT: ImageUpdateMask = ImageUpdateMask {
        ord: 8u64
    };
    #[doc(alias = "UPDATE_REGION")]
    #[doc = "Godot enumerator name: `UPDATE_REGION`"]
    pub const REGION: ImageUpdateMask = ImageUpdateMask {
        ord: 16u64
    };
    #[doc(alias = "UPDATE_PAD")]
    #[doc = "Godot enumerator name: `UPDATE_PAD`"]
    pub const PAD: ImageUpdateMask = ImageUpdateMask {
        ord: 32u64
    };
    #[doc(alias = "UPDATE_TOOLTIP")]
    #[doc = "Godot enumerator name: `UPDATE_TOOLTIP`"]
    pub const TOOLTIP: ImageUpdateMask = ImageUpdateMask {
        ord: 64u64
    };
    #[doc(alias = "UPDATE_WIDTH_IN_PERCENT")]
    #[doc = "Godot enumerator name: `UPDATE_WIDTH_IN_PERCENT`"]
    pub const WIDTH_IN_PERCENT: ImageUpdateMask = ImageUpdateMask {
        ord: 128u64
    };
    
}
impl std::fmt::Debug for ImageUpdateMask {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for ImageUpdateMask {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ImageUpdateMask >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TEXTURE", "UPDATE_TEXTURE", ImageUpdateMask::TEXTURE), crate::meta::inspect::EnumConstant::new("SIZE", "UPDATE_SIZE", ImageUpdateMask::SIZE), crate::meta::inspect::EnumConstant::new("COLOR", "UPDATE_COLOR", ImageUpdateMask::COLOR), crate::meta::inspect::EnumConstant::new("ALIGNMENT", "UPDATE_ALIGNMENT", ImageUpdateMask::ALIGNMENT), crate::meta::inspect::EnumConstant::new("REGION", "UPDATE_REGION", ImageUpdateMask::REGION), crate::meta::inspect::EnumConstant::new("PAD", "UPDATE_PAD", ImageUpdateMask::PAD), crate::meta::inspect::EnumConstant::new("TOOLTIP", "UPDATE_TOOLTIP", ImageUpdateMask::TOOLTIP), crate::meta::inspect::EnumConstant::new("WIDTH_IN_PERCENT", "UPDATE_WIDTH_IN_PERCENT", ImageUpdateMask::WIDTH_IN_PERCENT)]
        }
    }
}
impl std::ops::BitOr for ImageUpdateMask {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for ImageUpdateMask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for ImageUpdateMask {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Update Texture", 1i64), EnumeratorShape::new_int("Update Size", 2i64), EnumeratorShape::new_int("Update Color", 4i64), EnumeratorShape::new_int("Update Alignment", 8i64), EnumeratorShape::new_int("Update Region", 16i64), EnumeratorShape::new_int("Update Pad", 32i64), EnumeratorShape::new_int("Update Tooltip", 64i64), EnumeratorShape::new_int("Update Width In Percent", 128i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RichTextLabel.ImageUpdateMask")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for ImageUpdateMask {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ImageUpdateMask {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ImageUpdateMask {
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
impl crate::registry::property::Export for ImageUpdateMask {
    
}
impl crate::meta::Element for ImageUpdateMask {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RichTextLabel;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`RichTextLabel`][crate::classes::RichTextLabel] class."]
    pub struct SignalsOfRichTextLabel < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfRichTextLabel < 'c, C > {
        #[doc = "Signature: `(meta: Variant)`"]
        pub fn meta_clicked(&mut self) -> SigMetaClicked < 'c, C > {
            SigMetaClicked {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "meta_clicked")
            }
        }
        #[doc = "Signature: `(meta: Variant)`"]
        pub fn meta_hover_started(&mut self) -> SigMetaHoverStarted < 'c, C > {
            SigMetaHoverStarted {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "meta_hover_started")
            }
        }
        #[doc = "Signature: `(meta: Variant)`"]
        pub fn meta_hover_ended(&mut self) -> SigMetaHoverEnded < 'c, C > {
            SigMetaHoverEnded {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "meta_hover_ended")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn finished(&mut self) -> SigFinished < 'c, C > {
            SigFinished {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "finished")
            }
        }
    }
    type TypedSigMetaClicked < 'c, C > = TypedSignal < 'c, C, (Variant,) >;
    pub struct SigMetaClicked < 'c, C: WithSignals > {
        typed: TypedSigMetaClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMetaClicked < 'c, C > {
        pub fn emit(&mut self, meta: Variant,) {
            self.typed.emit_tuple((meta,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMetaClicked < 'c, C > {
        type Target = TypedSigMetaClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMetaClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMetaHoverStarted < 'c, C > = TypedSignal < 'c, C, (Variant,) >;
    pub struct SigMetaHoverStarted < 'c, C: WithSignals > {
        typed: TypedSigMetaHoverStarted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMetaHoverStarted < 'c, C > {
        pub fn emit(&mut self, meta: Variant,) {
            self.typed.emit_tuple((meta,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMetaHoverStarted < 'c, C > {
        type Target = TypedSigMetaHoverStarted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMetaHoverStarted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMetaHoverEnded < 'c, C > = TypedSignal < 'c, C, (Variant,) >;
    pub struct SigMetaHoverEnded < 'c, C: WithSignals > {
        typed: TypedSigMetaHoverEnded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMetaHoverEnded < 'c, C > {
        pub fn emit(&mut self, meta: Variant,) {
            self.typed.emit_tuple((meta,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMetaHoverEnded < 'c, C > {
        type Target = TypedSigMetaHoverEnded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMetaHoverEnded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFinished < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFinished < 'c, C: WithSignals > {
        typed: TypedSigFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFinished < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFinished < 'c, C > {
        type Target = TypedSigFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for RichTextLabel {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfRichTextLabel < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfRichTextLabel < 'c, C > {
        type Target = < < RichTextLabel as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = RichTextLabel;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfRichTextLabel < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = RichTextLabel;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
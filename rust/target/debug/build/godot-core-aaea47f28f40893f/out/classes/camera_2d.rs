#![doc = "Sidecar module for class [`Camera2D`][crate::classes::Camera2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Camera2D` enums](https://docs.godotengine.org/en/stable/classes/class_camera2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Camera2D`.\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`camera_2d`][crate::classes::camera_2d]: sidecar module with related enum/flag types\n* [`ICamera2D`][crate::classes::ICamera2D]: virtual methods\n\n\nSee also [Godot docs for `Camera2D`](https://docs.godotengine.org/en/stable/classes/class_camera2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Camera2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nCamera node for 2D scenes. It forces the screen (current layer) to scroll following this node. This makes it easier (and faster) to program scrollable scenes than manually changing the position of [`CanvasItem`][crate::classes::CanvasItem]-based nodes.\n\nCameras register themselves in the nearest [`Viewport`][crate::classes::Viewport] node (when ascending the tree). Only one camera can be active per viewport. If no viewport is available ascending the tree, the camera will register in the global viewport.\n\nThis node is intended to be a simple helper to get things going quickly, but more functionality may be desired to change how the camera works. To make your own custom camera node, inherit it from [`Node2D`][crate::classes::Node2D] and change the transform of the canvas by setting \\[member Viewport.canvas_transform] in [`Viewport`][crate::classes::Viewport] (you can obtain the current [`Viewport`][crate::classes::Viewport] by using [`get_viewport`][`crate::classes::Node::get_viewport`]).\n\nNote that the `Camera2D` node's \\[member Node2D.global_position] doesn't represent the actual position of the screen, which may differ due to applied smoothing or limits. You can use [`get_screen_center_position`][`crate::classes::Camera2D::get_screen_center_position`] to get the real position. Same for the node's \\[member Node2D.global_rotation] which may be different due to applied rotation smoothing. You can use [`get_screen_rotation`][`crate::classes::Camera2D::get_screen_rotation`] to get the current rotation of the screen."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Camera2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Camera2D`][crate::classes::Camera2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Camera2D` methods](https://docs.godotengine.org/en/stable/classes/class_camera2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICamera2D: crate::obj::GodotClass < Base = Camera2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
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
    impl Camera2D {
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_anchor_mode(&mut self, anchor_mode: crate::classes::camera_2d::AnchorMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::camera_2d::AnchorMode,);
            let args = (anchor_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_anchor_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_anchor_mode(&self,) -> crate::classes::camera_2d::AnchorMode {
            type CallRet = crate::classes::camera_2d::AnchorMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_anchor_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ignore_rotation(&mut self, ignore: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (ignore,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_ignore_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ignoring_rotation(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_ignoring_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_callback(&mut self, mode: crate::classes::camera_2d::Camera2DProcessCallback,) {
            type CallRet = ();
            type CallParams = (crate::classes::camera_2d::Camera2DProcessCallback,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_process_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_callback(&self,) -> crate::classes::camera_2d::Camera2DProcessCallback {
            type CallRet = crate::classes::camera_2d::Camera2DProcessCallback;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_process_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Forces this `Camera2D` to become the current active one. \\[member enabled] must be `true`."]
        pub fn make_current(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "make_current", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this `Camera2D` is the active camera (see [`get_camera_2d`][`crate::classes::Viewport::get_camera_2d`])."]
        pub fn is_current(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_current", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_limit_enabled(&mut self, limit_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (limit_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_limit_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_limit_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_limit_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the camera limit for the specified \\[enum Side]. See also \\[member limit_bottom], \\[member limit_top], \\[member limit_left], and \\[member limit_right]."]
        pub fn set_limit(&mut self, margin: crate::builtin::Side, limit: i32,) {
            type CallRet = ();
            type CallParams = (crate::builtin::Side, i32,);
            let args = (margin, limit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_limit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the camera limit for the specified \\[enum Side]. See also \\[member limit_bottom], \\[member limit_top], \\[member limit_left], and \\[member limit_right]."]
        pub fn get_limit(&self, margin: crate::builtin::Side,) -> i32 {
            type CallRet = i32;
            type CallParams = (crate::builtin::Side,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_limit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_limit_smoothing_enabled(&mut self, limit_smoothing_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (limit_smoothing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_limit_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_limit_smoothing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_limit_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_vertical_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_vertical_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drag_vertical_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_drag_vertical_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_horizontal_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_horizontal_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drag_horizontal_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_drag_horizontal_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_vertical_offset(&mut self, offset: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_vertical_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drag_vertical_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_drag_vertical_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_horizontal_offset(&mut self, offset: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_horizontal_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drag_horizontal_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_drag_horizontal_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the specified \\[enum Side]'s margin. See also \\[member drag_bottom_margin], \\[member drag_top_margin], \\[member drag_left_margin], and \\[member drag_right_margin]."]
        pub fn set_drag_margin(&mut self, margin: crate::builtin::Side, drag_margin: f32,) {
            type CallRet = ();
            type CallParams = (crate::builtin::Side, f32,);
            let args = (margin, drag_margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_margin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the specified \\[enum Side]'s margin. See also \\[member drag_bottom_margin], \\[member drag_top_margin], \\[member drag_left_margin], and \\[member drag_right_margin]."]
        pub fn get_drag_margin(&self, margin: crate::builtin::Side,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::builtin::Side,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_drag_margin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns this camera's target position, in global coordinates.\n\n**Note:** The returned value is not the same as \\[member Node2D.global_position], as it is affected by the drag properties. It is also not the same as the current position if \\[member position_smoothing_enabled] is `true` (see [`get_screen_center_position`][`crate::classes::Camera2D::get_screen_center_position`])."]
        pub fn get_target_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_target_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the center of the screen from this camera's point of view, in global coordinates.\n\n**Note:** The exact targeted position of the camera may be different. See [`get_target_position`][`crate::classes::Camera2D::get_target_position`]."]
        pub fn get_screen_center_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_screen_center_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current screen rotation from this camera's point of view.\n\n**Note:** The screen rotation can be different from \\[member Node2D.global_rotation] if the camera is rotating smoothly due to \\[member rotation_smoothing_enabled]."]
        pub fn get_screen_rotation(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_screen_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_zoom(&mut self, zoom: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (zoom,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_zoom", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_zoom(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_zoom", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_viewport(&mut self, viewport: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (viewport.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_custom_viewport", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_viewport(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_custom_viewport", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_position_smoothing_speed(&mut self, position_smoothing_speed: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (position_smoothing_speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_position_smoothing_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_position_smoothing_speed(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_position_smoothing_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_position_smoothing_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_position_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_position_smoothing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_position_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation_smoothing_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_rotation_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_rotation_smoothing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_rotation_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation_smoothing_speed(&mut self, speed: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_rotation_smoothing_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation_smoothing_speed(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_rotation_smoothing_speed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Forces the camera to update scroll immediately."]
        pub fn force_update_scroll(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "force_update_scroll", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the camera's position immediately to its current smoothing destination.\n\nThis method has no effect if \\[member position_smoothing_enabled] is `false`."]
        pub fn reset_smoothing(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "reset_smoothing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Aligns the camera to the tracked node.\n\n**Note:** Calling [`force_update_scroll`][`crate::classes::Camera2D::force_update_scroll`] after this method is not required."]
        pub fn align(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "align", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_screen_drawing_enabled(&mut self, screen_drawing_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (screen_drawing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_screen_drawing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_screen_drawing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_screen_drawing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_limit_drawing_enabled(&mut self, limit_drawing_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (limit_drawing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_limit_drawing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_limit_drawing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_limit_drawing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_margin_drawing_enabled(&mut self, margin_drawing_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (margin_drawing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_margin_drawing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_margin_drawing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_margin_drawing_enabled", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Camera2D {
        type Base = crate::classes::Node2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Camera2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Camera2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for Camera2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Camera2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Camera2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Camera2D {
        
    }
    impl crate::obj::cap::GodotDefault for Camera2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Camera2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Camera2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Camera2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Camera2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Camera2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node2D > for $Class {
                
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
pub struct AnchorMode {
    ord: i32
}
impl AnchorMode {
    #[doc(alias = "ANCHOR_MODE_FIXED_TOP_LEFT")]
    #[doc = "Godot enumerator name: `ANCHOR_MODE_FIXED_TOP_LEFT`"]
    pub const FIXED_TOP_LEFT: AnchorMode = AnchorMode {
        ord: 0i32
    };
    #[doc(alias = "ANCHOR_MODE_DRAG_CENTER")]
    #[doc = "Godot enumerator name: `ANCHOR_MODE_DRAG_CENTER`"]
    pub const DRAG_CENTER: AnchorMode = AnchorMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AnchorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnchorMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnchorMode {
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
            Self::FIXED_TOP_LEFT => "FIXED_TOP_LEFT", Self::DRAG_CENTER => "DRAG_CENTER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AnchorMode::FIXED_TOP_LEFT, AnchorMode::DRAG_CENTER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AnchorMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FIXED_TOP_LEFT", "ANCHOR_MODE_FIXED_TOP_LEFT", AnchorMode::FIXED_TOP_LEFT), crate::meta::inspect::EnumConstant::new("DRAG_CENTER", "ANCHOR_MODE_DRAG_CENTER", AnchorMode::DRAG_CENTER)]
        }
    }
}
impl crate::meta::GodotConvert for AnchorMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Anchor Mode Fixed Top Left", 0i64), EnumeratorShape::new_int("Anchor Mode Drag Center", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Camera2D.AnchorMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AnchorMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnchorMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AnchorMode {
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
impl crate::registry::property::Export for AnchorMode {
    
}
impl crate::meta::Element for AnchorMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Camera2DProcessCallback {
    ord: i32
}
impl Camera2DProcessCallback {
    #[doc(alias = "CAMERA2D_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `CAMERA2D_PROCESS_PHYSICS`"]
    pub const PHYSICS: Camera2DProcessCallback = Camera2DProcessCallback {
        ord: 0i32
    };
    #[doc(alias = "CAMERA2D_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `CAMERA2D_PROCESS_IDLE`"]
    pub const IDLE: Camera2DProcessCallback = Camera2DProcessCallback {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for Camera2DProcessCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Camera2DProcessCallback") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Camera2DProcessCallback {
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
            Self::PHYSICS => "PHYSICS", Self::IDLE => "IDLE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Camera2DProcessCallback::PHYSICS, Camera2DProcessCallback::IDLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Camera2DProcessCallback >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PHYSICS", "CAMERA2D_PROCESS_PHYSICS", Camera2DProcessCallback::PHYSICS), crate::meta::inspect::EnumConstant::new("IDLE", "CAMERA2D_PROCESS_IDLE", Camera2DProcessCallback::IDLE)]
        }
    }
}
impl crate::meta::GodotConvert for Camera2DProcessCallback {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Camera2d Process Physics", 0i64), EnumeratorShape::new_int("Camera2d Process Idle", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Camera2D.Camera2DProcessCallback")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Camera2DProcessCallback {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Camera2DProcessCallback {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Camera2DProcessCallback {
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
impl crate::registry::property::Export for Camera2DProcessCallback {
    
}
impl crate::meta::Element for Camera2DProcessCallback {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Camera2D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::canvas_item::SignalsOfCanvasItem;
    impl WithSignals for Camera2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCanvasItem < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
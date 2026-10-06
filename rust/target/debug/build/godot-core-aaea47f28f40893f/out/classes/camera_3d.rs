#![doc = "Sidecar module for class [`Camera3D`][crate::classes::Camera3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Camera3D` enums](https://docs.godotengine.org/en/stable/classes/class_camera3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Camera3D`.\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`camera_3d`][crate::classes::camera_3d]: sidecar module with related enum/flag types\n* [`ICamera3D`][crate::classes::ICamera3D]: virtual methods\n\n\nSee also [Godot docs for `Camera3D`](https://docs.godotengine.org/en/stable/classes/class_camera3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Camera3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\n`Camera3D` is a special node that displays what is visible from its current location. Cameras register themselves in the nearest [`Viewport`][crate::classes::Viewport] node (when ascending the tree). Only one camera can be active per viewport. If no viewport is available ascending the tree, the camera will register in the global viewport. In other words, a camera just provides 3D display capabilities to a [`Viewport`][crate::classes::Viewport], and, without one, a scene registered in that [`Viewport`][crate::classes::Viewport] (or higher viewports) can't be displayed."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Camera3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Camera3D`][crate::classes::Camera3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Camera3D` methods](https://docs.godotengine.org/en/stable/classes/class_camera3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICamera3D: crate::obj::GodotClass < Base = Camera3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
    impl Camera3D {
        #[doc = "Returns a normal vector in world space, that is the result of projecting a point on the [`Viewport`][crate::classes::Viewport] rectangle by the inverse camera projection. This is useful for casting rays in the form of (origin, normal) for object intersection or picking."]
        pub fn project_ray_normal(&self, screen_point: Vector2,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector2,);
            let args = (screen_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3813usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "project_ray_normal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a normal vector from the screen point location directed along the camera. Orthogonal cameras are normalized. Perspective cameras account for perspective, screen width/height, etc."]
        pub fn project_local_ray_normal(&self, screen_point: Vector2,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector2,);
            let args = (screen_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3814usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "project_local_ray_normal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a 3D position in world space, that is the result of projecting a point on the [`Viewport`][crate::classes::Viewport] rectangle by the inverse camera projection. This is useful for casting rays in the form of (origin, normal) for object intersection or picking."]
        pub fn project_ray_origin(&self, screen_point: Vector2,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector2,);
            let args = (screen_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3815usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "project_ray_origin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the 2D coordinate in the [`Viewport`][crate::classes::Viewport] rectangle that maps to the given 3D point in world space.\n\n**Note:** When using this to position GUI elements over a 3D viewport, use [`is_position_behind`][`crate::classes::Camera3D::is_position_behind`] to prevent them from appearing if the 3D point is behind the camera:\n\n```gdscript\n# This code block is part of a script that inherits from Node3D.\n# `control` is a reference to a node inheriting from Control.\ncontrol.visible = not get_viewport().get_camera_3d().is_position_behind(global_transform.origin)\ncontrol.position = get_viewport().get_camera_3d().unproject_position(global_transform.origin)\n```"]
        pub fn unproject_position(&self, world_point: Vector3,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector3,);
            let args = (world_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3816usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "unproject_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given position is behind the camera (the blue part of the linked diagram). [See this diagram](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/camera3d_position_frustum.png) for an overview of position query methods.\n\n**Note:** A position which returns `false` may still be outside the camera's field of view."]
        pub fn is_position_behind(&self, world_point: Vector3,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector3,);
            let args = (world_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "is_position_behind", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the 3D point in world space that maps to the given 2D coordinate in the [`Viewport`][crate::classes::Viewport] rectangle on a plane that is the given `z_depth` distance into the scene away from the camera."]
        pub fn project_position(&self, screen_point: Vector2, z_depth: f32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector2, f32,);
            let args = (screen_point, z_depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "project_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the camera projection to perspective mode (see [`ProjectionType::PERSPECTIVE`][`crate::classes::camera_3d::ProjectionType::PERSPECTIVE`]), by specifying a `fov` (field of view) angle in degrees, and the `z_near` and `z_far` clip planes in world space units."]
        pub fn set_perspective(&mut self, fov: f32, z_near: f32, z_far: f32,) {
            type CallRet = ();
            type CallParams = (f32, f32, f32,);
            let args = (fov, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_perspective", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the camera projection to orthogonal mode (see [`ProjectionType::ORTHOGONAL`][`crate::classes::camera_3d::ProjectionType::ORTHOGONAL`]), by specifying a `size`, and the `z_near` and `z_far` clip planes in world space units.\n\nAs a hint, 3D games that look 2D often use this projection, with `size` specified in pixels."]
        pub fn set_orthogonal(&mut self, size: f32, z_near: f32, z_far: f32,) {
            type CallRet = ();
            type CallParams = (f32, f32, f32,);
            let args = (size, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_orthogonal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the camera projection to frustum mode (see [`ProjectionType::FRUSTUM`][`crate::classes::camera_3d::ProjectionType::FRUSTUM`]), by specifying a `size`, an `offset`, and the `z_near` and `z_far` clip planes in world space units. See also \\[member frustum_offset]."]
        pub fn set_frustum(&mut self, size: f32, offset: Vector2, z_near: f32, z_far: f32,) {
            type CallRet = ();
            type CallParams = (f32, Vector2, f32, f32,);
            let args = (size, offset, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_frustum", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Makes this camera the current camera for the [`Viewport`][crate::classes::Viewport] (see class description). If the camera node is outside the scene tree, it will attempt to become current once it's added."]
        pub fn make_current(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "make_current", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If this is the current camera, remove it from being current. If `enable_next` is `true`, request to make the next camera current, if any."]
        pub(crate) fn clear_current_full(&mut self, enable_next: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable_next,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "clear_current", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`clear_current_ex`][Self::clear_current_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "If this is the current camera, remove it from being current. If `enable_next` is `true`, request to make the next camera current, if any."]
        #[inline]
        pub fn clear_current(&mut self,) {
            self.clear_current_ex() . done()
        }
        #[doc = "If this is the current camera, remove it from being current. If `enable_next` is `true`, request to make the next camera current, if any."]
        #[inline]
        pub fn clear_current_ex < 'ex > (&'ex mut self,) -> ExClearCurrent < 'ex > {
            ExClearCurrent::new(self,)
        }
        pub fn set_current(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_current", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_current(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "is_current", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transform of the camera plus the vertical (\\[member v_offset]) and horizontal (\\[member h_offset]) offsets; and any other adjustments made to the position and orientation of the camera by subclassed cameras such as [`XRCamera3D`][crate::classes::XrCamera3D]."]
        pub fn get_camera_transform(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_camera_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the projection matrix that this camera uses to render to its associated viewport. The camera must be part of the scene tree to function."]
        pub fn get_camera_projection(&self,) -> Projection {
            type CallRet = Projection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_camera_projection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fov(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_fov", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frustum_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_frustum_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_far(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_far", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_near(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_near", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fov(&mut self, fov: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (fov,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_fov", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_frustum_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_frustum_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_size(&mut self, size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_far(&mut self, far: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_far", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_near(&mut self, near: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (near,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_near", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_projection(&self,) -> crate::classes::camera_3d::ProjectionType {
            type CallRet = crate::classes::camera_3d::ProjectionType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_projection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_projection(&mut self, mode: crate::classes::camera_3d::ProjectionType,) {
            type CallRet = ();
            type CallParams = (crate::classes::camera_3d::ProjectionType,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_projection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_h_offset(&mut self, offset: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_h_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_h_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_h_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_v_offset(&mut self, offset: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_v_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_v_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cull_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cull_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_environment(&mut self, env: impl AsArg < Option < Gd < crate::classes::Environment >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Environment > > >,);
            let args = (env.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_environment(&self,) -> Option < Gd < crate::classes::Environment > > {
            type CallRet = Option < Gd < crate::classes::Environment > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_attributes(&mut self, env: impl AsArg < Option < Gd < crate::classes::CameraAttributes >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::CameraAttributes > > >,);
            let args = (env.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_attributes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_attributes(&self,) -> Option < Gd < crate::classes::CameraAttributes > > {
            type CallRet = Option < Gd < crate::classes::CameraAttributes > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_attributes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keep_aspect_mode(&mut self, mode: crate::classes::camera_3d::KeepAspect,) {
            type CallRet = ();
            type CallParams = (crate::classes::camera_3d::KeepAspect,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_keep_aspect_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keep_aspect_mode(&self,) -> crate::classes::camera_3d::KeepAspect {
            type CallRet = crate::classes::camera_3d::KeepAspect;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_keep_aspect_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_doppler_tracking(&mut self, mode: crate::classes::camera_3d::DopplerTracking,) {
            type CallRet = ();
            type CallParams = (crate::classes::camera_3d::DopplerTracking,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_doppler_tracking", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_doppler_tracking(&self,) -> crate::classes::camera_3d::DopplerTracking {
            type CallRet = crate::classes::camera_3d::DopplerTracking;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_doppler_tracking", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the camera's frustum planes in world space units as an array of [`Plane`][crate::builtin::Plane]s in the following order: near, far, left, top, right, bottom. Not to be confused with \\[member frustum_offset]."]
        pub fn get_frustum(&self,) -> Array < Plane > {
            type CallRet = Array < Plane >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_frustum", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given position is inside the camera's frustum (the green part of the linked diagram). [See this diagram](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/camera3d_position_frustum.png) for an overview of position query methods."]
        pub fn is_position_in_frustum(&self, world_point: Vector3,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector3,);
            let args = (world_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "is_position_in_frustum", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the camera's RID from the [`RenderingServer`][crate::classes::RenderingServer]."]
        pub fn get_camera_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_camera_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the RID of a pyramid shape encompassing the camera's view frustum, ignoring the camera's near plane. The tip of the pyramid represents the position of the camera."]
        pub fn get_pyramid_shape_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_pyramid_shape_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Based on `value`, enables or disables the specified layer in the \\[member cull_mask], given a `layer_number` between 1 and 20."]
        pub fn set_cull_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_cull_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether or not the specified layer of the \\[member cull_mask] is enabled, given a `layer_number` between 1 and 20."]
        pub fn get_cull_mask_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_cull_mask_value", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Camera3D {
        type Base = crate::classes::Node3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Camera3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Camera3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for Camera3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Camera3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Camera3D {
        
    }
    impl crate::obj::cap::GodotDefault for Camera3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Camera3D {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Camera3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Camera3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Camera3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Camera3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Camera3D::clear_current_ex`][super::Camera3D::clear_current_ex]."]
#[must_use]
pub struct ExClearCurrent < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Camera3D, enable_next: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExClearCurrent < 'ex > {
    fn new(surround_object: &'ex mut re_export::Camera3D,) -> Self {
        let enable_next = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, enable_next: enable_next,
        }
    }
    #[inline]
    pub fn enable_next(self, enable_next: bool) -> Self {
        Self {
            enable_next: enable_next, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, enable_next,
        }
        = self;
        re_export::Camera3D::clear_current_full(surround_object, enable_next,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProjectionType {
    ord: i32
}
impl ProjectionType {
    #[doc(alias = "PROJECTION_PERSPECTIVE")]
    #[doc = "Godot enumerator name: `PROJECTION_PERSPECTIVE`"]
    pub const PERSPECTIVE: ProjectionType = ProjectionType {
        ord: 0i32
    };
    #[doc(alias = "PROJECTION_ORTHOGONAL")]
    #[doc = "Godot enumerator name: `PROJECTION_ORTHOGONAL`"]
    pub const ORTHOGONAL: ProjectionType = ProjectionType {
        ord: 1i32
    };
    #[doc(alias = "PROJECTION_FRUSTUM")]
    #[doc = "Godot enumerator name: `PROJECTION_FRUSTUM`"]
    pub const FRUSTUM: ProjectionType = ProjectionType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ProjectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ProjectionType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ProjectionType {
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
            Self::PERSPECTIVE => "PERSPECTIVE", Self::ORTHOGONAL => "ORTHOGONAL", Self::FRUSTUM => "FRUSTUM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ProjectionType::PERSPECTIVE, ProjectionType::ORTHOGONAL, ProjectionType::FRUSTUM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ProjectionType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PERSPECTIVE", "PROJECTION_PERSPECTIVE", ProjectionType::PERSPECTIVE), crate::meta::inspect::EnumConstant::new("ORTHOGONAL", "PROJECTION_ORTHOGONAL", ProjectionType::ORTHOGONAL), crate::meta::inspect::EnumConstant::new("FRUSTUM", "PROJECTION_FRUSTUM", ProjectionType::FRUSTUM)]
        }
    }
}
impl crate::meta::GodotConvert for ProjectionType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Projection Perspective", 0i64), EnumeratorShape::new_int("Projection Orthogonal", 1i64), EnumeratorShape::new_int("Projection Frustum", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Camera3D.ProjectionType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ProjectionType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProjectionType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ProjectionType {
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
impl crate::registry::property::Export for ProjectionType {
    
}
impl crate::meta::Element for ProjectionType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct KeepAspect {
    ord: i32
}
impl KeepAspect {
    #[doc(alias = "KEEP_WIDTH")]
    #[doc = "Godot enumerator name: `KEEP_WIDTH`"]
    pub const WIDTH: KeepAspect = KeepAspect {
        ord: 0i32
    };
    #[doc(alias = "KEEP_HEIGHT")]
    #[doc = "Godot enumerator name: `KEEP_HEIGHT`"]
    pub const HEIGHT: KeepAspect = KeepAspect {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for KeepAspect {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("KeepAspect") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for KeepAspect {
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
            Self::WIDTH => "WIDTH", Self::HEIGHT => "HEIGHT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[KeepAspect::WIDTH, KeepAspect::HEIGHT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < KeepAspect >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("WIDTH", "KEEP_WIDTH", KeepAspect::WIDTH), crate::meta::inspect::EnumConstant::new("HEIGHT", "KEEP_HEIGHT", KeepAspect::HEIGHT)]
        }
    }
}
impl crate::meta::GodotConvert for KeepAspect {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Keep Width", 0i64), EnumeratorShape::new_int("Keep Height", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Camera3D.KeepAspect")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for KeepAspect {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for KeepAspect {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for KeepAspect {
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
impl crate::registry::property::Export for KeepAspect {
    
}
impl crate::meta::Element for KeepAspect {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DopplerTracking {
    ord: i32
}
impl DopplerTracking {
    #[doc(alias = "DOPPLER_TRACKING_DISABLED")]
    #[doc = "Godot enumerator name: `DOPPLER_TRACKING_DISABLED`"]
    pub const DISABLED: DopplerTracking = DopplerTracking {
        ord: 0i32
    };
    #[doc(alias = "DOPPLER_TRACKING_IDLE_STEP")]
    #[doc = "Godot enumerator name: `DOPPLER_TRACKING_IDLE_STEP`"]
    pub const IDLE_STEP: DopplerTracking = DopplerTracking {
        ord: 1i32
    };
    #[doc(alias = "DOPPLER_TRACKING_PHYSICS_STEP")]
    #[doc = "Godot enumerator name: `DOPPLER_TRACKING_PHYSICS_STEP`"]
    pub const PHYSICS_STEP: DopplerTracking = DopplerTracking {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DopplerTracking {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DopplerTracking") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DopplerTracking {
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
            Self::DISABLED => "DISABLED", Self::IDLE_STEP => "IDLE_STEP", Self::PHYSICS_STEP => "PHYSICS_STEP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DopplerTracking::DISABLED, DopplerTracking::IDLE_STEP, DopplerTracking::PHYSICS_STEP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DopplerTracking >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "DOPPLER_TRACKING_DISABLED", DopplerTracking::DISABLED), crate::meta::inspect::EnumConstant::new("IDLE_STEP", "DOPPLER_TRACKING_IDLE_STEP", DopplerTracking::IDLE_STEP), crate::meta::inspect::EnumConstant::new("PHYSICS_STEP", "DOPPLER_TRACKING_PHYSICS_STEP", DopplerTracking::PHYSICS_STEP)]
        }
    }
}
impl crate::meta::GodotConvert for DopplerTracking {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Doppler Tracking Disabled", 0i64), EnumeratorShape::new_int("Doppler Tracking Idle Step", 1i64), EnumeratorShape::new_int("Doppler Tracking Physics Step", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Camera3D.DopplerTracking")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DopplerTracking {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DopplerTracking {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DopplerTracking {
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
impl crate::registry::property::Export for DopplerTracking {
    
}
impl crate::meta::Element for DopplerTracking {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Camera3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for Camera3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
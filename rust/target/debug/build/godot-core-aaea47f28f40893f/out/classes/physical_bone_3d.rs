#![doc = "Sidecar module for class [`PhysicalBone3D`][crate::classes::PhysicalBone3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicalBone3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicalbone3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicalBone3D`.\n\nInherits [`PhysicsBody3D`][crate::classes::PhysicsBody3D].\n\nRelated symbols:\n\n* [`physical_bone_3d`][crate::classes::physical_bone_3d]: sidecar module with related enum/flag types\n* [`IPhysicalBone3D`][crate::classes::IPhysicalBone3D]: virtual methods\n\n\nSee also [Godot docs for `PhysicalBone3D`](https://docs.godotengine.org/en/stable/classes/class_physicalbone3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`PhysicalBone3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nThe `PhysicalBone3D` node is a physics body that can be used to make bones in a [`Skeleton3D`][crate::classes::Skeleton3D] react to physics.\n\n**Note:** In order to detect physical bones with raycasts, the \\[member SkeletonModifier3D.active] property of the parent [`PhysicalBoneSimulator3D`][crate::classes::PhysicalBoneSimulator3D] must be `true` and the [`Skeleton3D`][crate::classes::Skeleton3D]'s bone must be assigned to `PhysicalBone3D` correctly; it means that [`get_bone_id`][`crate::classes::PhysicalBone3D::get_bone_id`] should return a valid id (`>= 0`)."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicalBone3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PhysicalBone3D`][crate::classes::PhysicalBone3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IPhysicsBody3D`~~ > ~~`ICollisionObject3D`~~ > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `PhysicalBone3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicalbone3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicalBone3D: crate::obj::GodotClass < Base = PhysicalBone3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Called during physics processing, allowing you to read and safely modify the simulation state for the object. By default, it is called before the standard force integration, but the \\[member custom_integrator] property allows you to disable the standard force integration and do fully custom force integration for a body."]
        fn integrate_forces(&mut self, state: Option < Gd < crate::classes::PhysicsDirectBodyState3D > >,) {
            unimplemented !()
        }
        #[doc = "Receives unhandled [`InputEvent`][crate::classes::InputEvent]s. `event_position` is the location in world space of the mouse pointer on the surface of the shape with index `shape_idx` and `normal` is the normal vector of the surface at that point. Connect to the `input_event` signal to easily pick up these events.\n\n**Note:** \\[method _input_event] requires \\[member input_ray_pickable] to be `true` and at least one \\[member collision_layer] bit to be set."]
        fn input_event(&mut self, camera: Option < Gd < crate::classes::Camera3D > >, event: Option < Gd < crate::classes::InputEvent > >, event_position: Vector3, normal: Vector3, shape_idx: i32,) {
            unimplemented !()
        }
        #[doc = "Called when the mouse pointer enters any of this object's shapes. Requires \\[member input_ray_pickable] to be `true` and at least one \\[member collision_layer] bit to be set. Note that moving between different shapes within a single `CollisionObject3D` won't cause this function to be called."]
        fn mouse_enter(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the mouse pointer exits all this object's shapes. Requires \\[member input_ray_pickable] to be `true` and at least one \\[member collision_layer] bit to be set. Note that moving between different shapes within a single `CollisionObject3D` won't cause this function to be called."]
        fn mouse_exit(&mut self,) {
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
    impl PhysicalBone3D {
        #[doc = "Applies a directional impulse without affecting rotation.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_integrate_forces\" functions otherwise).\n\nThis is equivalent to using [`apply_impulse`][`crate::classes::PhysicalBone3D::apply_impulse`] at the body's center of mass."]
        pub fn apply_central_impulse(&mut self, impulse: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "apply_central_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a positioned impulse to the PhysicsBone3D.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_integrate_forces\" functions otherwise).\n\n`position` is the offset from the PhysicsBone3D origin in global coordinates."]
        pub(crate) fn apply_impulse_full(&mut self, impulse: Vector3, position: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3, Vector3,);
            let args = (impulse, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "apply_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`apply_impulse_ex`][Self::apply_impulse_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a positioned impulse to the PhysicsBone3D.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_integrate_forces\" functions otherwise).\n\n`position` is the offset from the PhysicsBone3D origin in global coordinates."]
        #[inline]
        pub fn apply_impulse(&mut self, impulse: Vector3,) {
            self.apply_impulse_ex(impulse,) . done()
        }
        #[doc = "Applies a positioned impulse to the PhysicsBone3D.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_integrate_forces\" functions otherwise).\n\n`position` is the offset from the PhysicsBone3D origin in global coordinates."]
        #[inline]
        pub fn apply_impulse_ex < 'ex > (&'ex mut self, impulse: Vector3,) -> ExApplyImpulse < 'ex > {
            ExApplyImpulse::new(self, impulse,)
        }
        pub fn set_joint_type(&mut self, joint_type: crate::classes::physical_bone_3d::JointType,) {
            type CallRet = ();
            type CallParams = (crate::classes::physical_bone_3d::JointType,);
            let args = (joint_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_joint_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_type(&self,) -> crate::classes::physical_bone_3d::JointType {
            type CallRet = crate::classes::physical_bone_3d::JointType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_joint_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_joint_offset(&mut self, offset: Transform3D,) {
            type CallRet = ();
            type CallParams = (Transform3D,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_joint_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_offset(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_joint_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_joint_rotation(&mut self, euler: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (euler,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_joint_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_rotation(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_joint_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_body_offset(&mut self, offset: Transform3D,) {
            type CallRet = ();
            type CallParams = (Transform3D,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_body_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_body_offset(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_body_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the PhysicsBone3D is allowed to simulate physics."]
        pub fn get_simulate_physics(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_simulate_physics", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the PhysicsBone3D is currently simulating physics."]
        pub fn is_simulating_physics(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "is_simulating_physics", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the unique identifier of the PhysicsBone3D."]
        pub fn get_bone_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_bone_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mass(&mut self, mass: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mass(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_friction(&mut self, friction: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (friction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_friction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_friction(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_friction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bounce(&mut self, bounce: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bounce,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_bounce", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bounce(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_bounce", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_scale(&mut self, gravity_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (gravity_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_gravity_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_gravity_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_damp_mode(&mut self, linear_damp_mode: crate::classes::physical_bone_3d::DampMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::physical_bone_3d::DampMode,);
            let args = (linear_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_linear_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_damp_mode(&self,) -> crate::classes::physical_bone_3d::DampMode {
            type CallRet = crate::classes::physical_bone_3d::DampMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_linear_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_damp_mode(&mut self, angular_damp_mode: crate::classes::physical_bone_3d::DampMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::physical_bone_3d::DampMode,);
            let args = (angular_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_angular_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_damp_mode(&self,) -> crate::classes::physical_bone_3d::DampMode {
            type CallRet = crate::classes::physical_bone_3d::DampMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_angular_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_damp(&mut self, linear_damp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (linear_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_damp(&mut self, angular_damp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angular_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, linear_velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (linear_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, angular_velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (angular_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_custom_integrator(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_use_custom_integrator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_custom_integrator(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "is_using_custom_integrator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_can_sleep(&mut self, able_to_sleep: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (able_to_sleep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_can_sleep", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_able_to_sleep(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "is_able_to_sleep", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicalBone3D {
        type Base = crate::classes::PhysicsBody3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicalBone3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicalBone3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsBody3D > for PhysicalBone3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject3D > for PhysicalBone3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for PhysicalBone3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for PhysicalBone3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicalBone3D {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicalBone3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicalBone3D {
        type Target = crate::classes::PhysicsBody3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicalBone3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicalBone3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicalBone3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicalBone3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsBody3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CollisionObject3D > for $Class {
                
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
#[doc = "Default-param extender for [`PhysicalBone3D::apply_impulse_ex`][super::PhysicalBone3D::apply_impulse_ex]."]
#[must_use]
pub struct ExApplyImpulse < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicalBone3D, impulse: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExApplyImpulse < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicalBone3D, impulse: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector3) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, impulse, position,
        }
        = self;
        re_export::PhysicalBone3D::apply_impulse_full(surround_object, impulse, position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DampMode {
    ord: i32
}
impl DampMode {
    #[doc(alias = "DAMP_MODE_COMBINE")]
    #[doc = "Godot enumerator name: `DAMP_MODE_COMBINE`"]
    pub const COMBINE: DampMode = DampMode {
        ord: 0i32
    };
    #[doc(alias = "DAMP_MODE_REPLACE")]
    #[doc = "Godot enumerator name: `DAMP_MODE_REPLACE`"]
    pub const REPLACE: DampMode = DampMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DampMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DampMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DampMode {
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
            Self::COMBINE => "COMBINE", Self::REPLACE => "REPLACE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DampMode::COMBINE, DampMode::REPLACE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DampMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("COMBINE", "DAMP_MODE_COMBINE", DampMode::COMBINE), crate::meta::inspect::EnumConstant::new("REPLACE", "DAMP_MODE_REPLACE", DampMode::REPLACE)]
        }
    }
}
impl crate::meta::GodotConvert for DampMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Damp Mode Combine", 0i64), EnumeratorShape::new_int("Damp Mode Replace", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicalBone3D.DampMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DampMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DampMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DampMode {
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
impl crate::registry::property::Export for DampMode {
    
}
impl crate::meta::Element for DampMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct JointType {
    ord: i32
}
impl JointType {
    #[doc(alias = "JOINT_TYPE_NONE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_NONE`"]
    pub const NONE: JointType = JointType {
        ord: 0i32
    };
    #[doc(alias = "JOINT_TYPE_PIN")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_PIN`"]
    pub const PIN: JointType = JointType {
        ord: 1i32
    };
    #[doc(alias = "JOINT_TYPE_CONE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_CONE`"]
    pub const CONE: JointType = JointType {
        ord: 2i32
    };
    #[doc(alias = "JOINT_TYPE_HINGE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_HINGE`"]
    pub const HINGE: JointType = JointType {
        ord: 3i32
    };
    #[doc(alias = "JOINT_TYPE_SLIDER")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_SLIDER`"]
    pub const SLIDER: JointType = JointType {
        ord: 4i32
    };
    #[doc(alias = "JOINT_TYPE_6DOF")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_6DOF`"]
    pub const TYPE_6DOF: JointType = JointType {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for JointType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("JointType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for JointType {
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
            Self::NONE => "NONE", Self::PIN => "PIN", Self::CONE => "CONE", Self::HINGE => "HINGE", Self::SLIDER => "SLIDER", Self::TYPE_6DOF => "TYPE_6DOF", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[JointType::NONE, JointType::PIN, JointType::CONE, JointType::HINGE, JointType::SLIDER, JointType::TYPE_6DOF]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < JointType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "JOINT_TYPE_NONE", JointType::NONE), crate::meta::inspect::EnumConstant::new("PIN", "JOINT_TYPE_PIN", JointType::PIN), crate::meta::inspect::EnumConstant::new("CONE", "JOINT_TYPE_CONE", JointType::CONE), crate::meta::inspect::EnumConstant::new("HINGE", "JOINT_TYPE_HINGE", JointType::HINGE), crate::meta::inspect::EnumConstant::new("SLIDER", "JOINT_TYPE_SLIDER", JointType::SLIDER), crate::meta::inspect::EnumConstant::new("TYPE_6DOF", "JOINT_TYPE_6DOF", JointType::TYPE_6DOF)]
        }
    }
}
impl crate::meta::GodotConvert for JointType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Joint Type None", 0i64), EnumeratorShape::new_int("Joint Type Pin", 1i64), EnumeratorShape::new_int("Joint Type Cone", 2i64), EnumeratorShape::new_int("Joint Type Hinge", 3i64), EnumeratorShape::new_int("Joint Type Slider", 4i64), EnumeratorShape::new_int("Joint Type 6dof", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PhysicalBone3D.JointType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for JointType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for JointType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for JointType {
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
impl crate::registry::property::Export for JointType {
    
}
impl crate::meta::Element for JointType {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicalBone3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::collision_object_3d::SignalsOfCollisionObject3D;
    impl WithSignals for PhysicalBone3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCollisionObject3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
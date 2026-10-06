#![doc = "Sidecar module for class [`RigidBody2D`][crate::classes::RigidBody2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RigidBody2D` enums](https://docs.godotengine.org/en/stable/classes/class_rigidbody2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RigidBody2D`.\n\nInherits [`PhysicsBody2D`][crate::classes::PhysicsBody2D].\n\nRelated symbols:\n\n* [`rigid_body_2d`][crate::classes::rigid_body_2d]: sidecar module with related enum/flag types\n* [`IRigidBody2D`][crate::classes::IRigidBody2D]: virtual methods\n* [`SignalsOfRigidBody2D`][crate::classes::rigid_body_2d::SignalsOfRigidBody2D]: signal collection\n\n\nSee also [Godot docs for `RigidBody2D`](https://docs.godotengine.org/en/stable/classes/class_rigidbody2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`RigidBody2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\n`RigidBody2D` implements full 2D physics. It cannot be controlled directly, instead, you must apply forces to it (gravity, impulses, etc.), and the physics simulation will calculate the resulting movement, rotation, react to collisions, and affect other physics bodies in its path.\n\nThe body's behavior can be adjusted via \\[member lock_rotation], \\[member freeze], and \\[member freeze_mode]. By changing various properties of the object, such as \\[member mass], you can control how the physics simulation acts on it.\n\nA rigid body will always maintain its shape and size, even when forces are applied to it. It is useful for objects that can be interacted with in an environment, such as a tree that can be knocked over or a stack of crates that can be pushed around.\n\nIf you need to directly affect the body, prefer [`integrate_forces`][`crate::classes::IRigidBody2D::integrate_forces`] as it allows you to directly access the physics state.\n\nIf you need to override the default physics behavior, you can write a custom force integration function. See \\[member custom_integrator].\n\n**Note:** Changing the 2D transform or \\[member linear_velocity] of a `RigidBody2D` very often may lead to some unpredictable behaviors. This also happens when a `RigidBody2D` is the descendant of a constantly moving node, like another `RigidBody2D`, as that will cause its global transform to be set whenever its ancestor moves."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RigidBody2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`RigidBody2D`][crate::classes::RigidBody2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IPhysicsBody2D`~~ > ~~`ICollisionObject2D`~~ > [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `RigidBody2D` methods](https://docs.godotengine.org/en/stable/classes/class_rigidbody2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRigidBody2D: crate::obj::GodotClass < Base = RigidBody2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Called during physics processing, allowing you to read and safely modify the simulation state for the object. By default, it is called before the standard force integration, but the \\[member custom_integrator] property allows you to disable the standard force integration and do fully custom force integration for a body."]
        fn integrate_forces(&mut self, state: Option < Gd < crate::classes::PhysicsDirectBodyState2D > >,) {
            unimplemented !()
        }
        #[doc = "Accepts unhandled [`InputEvent`][crate::classes::InputEvent]s. `shape_idx` is the child index of the clicked [`Shape2D`][crate::classes::Shape2D]. Connect to `input_event` to easily pick up these events.\n\n**Note:** \\[method _input_event] requires \\[member input_pickable] to be `true` and at least one \\[member collision_layer] bit to be set."]
        fn input_event(&mut self, viewport: Gd < crate::classes::Viewport >, event: Gd < crate::classes::InputEvent >, shape_idx: i32,) {
            unimplemented !()
        }
        #[doc = "Called when the mouse pointer enters any of this object's shapes. Requires \\[member input_pickable] to be `true` and at least one \\[member collision_layer] bit to be set. Note that moving between different shapes within a single `CollisionObject2D` won't cause this function to be called."]
        fn mouse_enter(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the mouse pointer exits all this object's shapes. Requires \\[member input_pickable] to be `true` and at least one \\[member collision_layer] bit to be set. Note that moving between different shapes within a single `CollisionObject2D` won't cause this function to be called."]
        fn mouse_exit(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the mouse pointer enters any of this object's shapes or moves from one shape to another. `shape_idx` is the child index of the newly entered [`Shape2D`][crate::classes::Shape2D]. Requires \\[member input_pickable] to be `true` and at least one \\[member collision_layer] bit to be called."]
        fn mouse_shape_enter(&mut self, shape_idx: i32,) {
            unimplemented !()
        }
        #[doc = "Called when the mouse pointer exits any of this object's shapes. `shape_idx` is the child index of the exited [`Shape2D`][crate::classes::Shape2D]. Requires \\[member input_pickable] to be `true` and at least one \\[member collision_layer] bit to be called."]
        fn mouse_shape_exit(&mut self, shape_idx: i32,) {
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
    impl RigidBody2D {
        pub fn set_mass(&mut self, mass: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8624usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mass(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inertia(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_inertia", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_inertia(&mut self, inertia: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (inertia,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_inertia", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_of_mass_mode(&mut self, mode: crate::classes::rigid_body_2d::CenterOfMassMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::rigid_body_2d::CenterOfMassMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_center_of_mass_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_of_mass_mode(&self,) -> crate::classes::rigid_body_2d::CenterOfMassMode {
            type CallRet = crate::classes::rigid_body_2d::CenterOfMassMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_center_of_mass_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_of_mass(&mut self, center_of_mass: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (center_of_mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_center_of_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_of_mass(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_center_of_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_material_override(&mut self, physics_material_override: impl AsArg < Option < Gd < crate::classes::PhysicsMaterial >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PhysicsMaterial > > >,);
            let args = (physics_material_override.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_physics_material_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_material_override(&self,) -> Option < Gd < crate::classes::PhysicsMaterial > > {
            type CallRet = Option < Gd < crate::classes::PhysicsMaterial > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8633usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_physics_material_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_scale(&mut self, gravity_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (gravity_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8634usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_gravity_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_gravity_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_damp_mode(&mut self, linear_damp_mode: crate::classes::rigid_body_2d::DampMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::rigid_body_2d::DampMode,);
            let args = (linear_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_linear_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_damp_mode(&self,) -> crate::classes::rigid_body_2d::DampMode {
            type CallRet = crate::classes::rigid_body_2d::DampMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_linear_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_damp_mode(&mut self, angular_damp_mode: crate::classes::rigid_body_2d::DampMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::rigid_body_2d::DampMode,);
            let args = (angular_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_angular_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_damp_mode(&self,) -> crate::classes::rigid_body_2d::DampMode {
            type CallRet = crate::classes::rigid_body_2d::DampMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_angular_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_damp(&mut self, linear_damp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (linear_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_damp(&mut self, angular_damp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angular_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, linear_velocity: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (linear_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, angular_velocity: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angular_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_contacts_reported(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_max_contacts_reported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_contacts_reported(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_max_contacts_reported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of contacts this body has with other bodies. By default, this returns 0 unless bodies are configured to monitor contacts (see \\[member contact_monitor]).\n\n**Note:** To retrieve the colliding bodies, use [`get_colliding_bodies`][`crate::classes::RigidBody2D::get_colliding_bodies`]."]
        pub fn get_contact_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_contact_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_custom_integrator(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_use_custom_integrator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_custom_integrator(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8652usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "is_using_custom_integrator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_contact_monitor(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8653usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_contact_monitor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_contact_monitor_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8654usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "is_contact_monitor_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_continuous_collision_detection_mode(&mut self, mode: crate::classes::rigid_body_2d::CcdMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::rigid_body_2d::CcdMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8655usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_continuous_collision_detection_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_continuous_collision_detection_mode(&self,) -> crate::classes::rigid_body_2d::CcdMode {
            type CallRet = crate::classes::rigid_body_2d::CcdMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8656usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_continuous_collision_detection_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's velocity on the given axis. The velocity in the given vector axis will be set as the given vector length. This is useful for jumping behavior."]
        pub fn set_axis_velocity(&mut self, axis_velocity: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (axis_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8657usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_axis_velocity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a directional impulse without affecting rotation.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\nThis is equivalent to using [`apply_impulse`][`crate::classes::RigidBody2D::apply_impulse`] at the body's center of mass."]
        pub(crate) fn apply_central_impulse_full(&mut self, impulse: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8658usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "apply_central_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`apply_central_impulse_ex`][Self::apply_central_impulse_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a directional impulse without affecting rotation.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\nThis is equivalent to using [`apply_impulse`][`crate::classes::RigidBody2D::apply_impulse`] at the body's center of mass."]
        #[inline]
        pub fn apply_central_impulse(&mut self,) {
            self.apply_central_impulse_ex() . done()
        }
        #[doc = "Applies a directional impulse without affecting rotation.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\nThis is equivalent to using [`apply_impulse`][`crate::classes::RigidBody2D::apply_impulse`] at the body's center of mass."]
        #[inline]
        pub fn apply_central_impulse_ex < 'ex > (&'ex mut self,) -> ExApplyCentralImpulse < 'ex > {
            ExApplyCentralImpulse::new(self,)
        }
        #[doc = "Applies a positioned impulse to the body.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n`position` is the offset from the body origin in global coordinates."]
        pub(crate) fn apply_impulse_full(&mut self, impulse: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (impulse, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "apply_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`apply_impulse_ex`][Self::apply_impulse_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a positioned impulse to the body.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn apply_impulse(&mut self, impulse: Vector2,) {
            self.apply_impulse_ex(impulse,) . done()
        }
        #[doc = "Applies a positioned impulse to the body.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn apply_impulse_ex < 'ex > (&'ex mut self, impulse: Vector2,) -> ExApplyImpulse < 'ex > {
            ExApplyImpulse::new(self, impulse,)
        }
        #[doc = "Applies a rotational impulse to the body without affecting the position.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n**Note:** \\[member inertia] is required for this to work. To have \\[member inertia], an active [`CollisionShape2D`][crate::classes::CollisionShape2D] must be a child of the node, or you can manually set \\[member inertia]."]
        pub fn apply_torque_impulse(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "apply_torque_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a directional force without affecting rotation. A force is time dependent and meant to be applied every physics update.\n\nThis is equivalent to using [`apply_force`][`crate::classes::RigidBody2D::apply_force`] at the body's center of mass."]
        pub fn apply_central_force(&mut self, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "apply_central_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a positioned force to the body. A force is time dependent and meant to be applied every physics update.\n\n`position` is the offset from the body origin in global coordinates."]
        pub(crate) fn apply_force_full(&mut self, force: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "apply_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`apply_force_ex`][Self::apply_force_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a positioned force to the body. A force is time dependent and meant to be applied every physics update.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn apply_force(&mut self, force: Vector2,) {
            self.apply_force_ex(force,) . done()
        }
        #[doc = "Applies a positioned force to the body. A force is time dependent and meant to be applied every physics update.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn apply_force_ex < 'ex > (&'ex mut self, force: Vector2,) -> ExApplyForce < 'ex > {
            ExApplyForce::new(self, force,)
        }
        #[doc = "Applies a rotational force without affecting position. A force is time dependent and meant to be applied every physics update.\n\n**Note:** \\[member inertia] is required for this to work. To have \\[member inertia], an active [`CollisionShape2D`][crate::classes::CollisionShape2D] must be a child of the node, or you can manually set \\[member inertia]."]
        pub fn apply_torque(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "apply_torque", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a constant directional force without affecting rotation that keeps being applied over time until cleared with `constant_force = Vector2(0, 0)`.\n\nThis is equivalent to using [`add_constant_force`][`crate::classes::RigidBody2D::add_constant_force`] at the body's center of mass."]
        pub fn add_constant_central_force(&mut self, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "add_constant_central_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a constant positioned force to the body that keeps being applied over time until cleared with `constant_force = Vector2(0, 0)`.\n\n`position` is the offset from the body origin in global coordinates."]
        pub(crate) fn add_constant_force_full(&mut self, force: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "add_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_constant_force_ex`][Self::add_constant_force_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a constant positioned force to the body that keeps being applied over time until cleared with `constant_force = Vector2(0, 0)`.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn add_constant_force(&mut self, force: Vector2,) {
            self.add_constant_force_ex(force,) . done()
        }
        #[doc = "Adds a constant positioned force to the body that keeps being applied over time until cleared with `constant_force = Vector2(0, 0)`.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn add_constant_force_ex < 'ex > (&'ex mut self, force: Vector2,) -> ExAddConstantForce < 'ex > {
            ExAddConstantForce::new(self, force,)
        }
        #[doc = "Adds a constant rotational force without affecting position that keeps being applied over time until cleared with `constant_torque = 0`."]
        pub fn add_constant_torque(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "add_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_constant_force(&mut self, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant_force(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_constant_torque(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant_torque(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sleeping(&mut self, sleeping: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (sleeping,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_sleeping", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_sleeping(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "is_sleeping", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_can_sleep(&mut self, able_to_sleep: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (able_to_sleep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_can_sleep", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_able_to_sleep(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "is_able_to_sleep", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lock_rotation_enabled(&mut self, lock_rotation: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (lock_rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_lock_rotation_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_lock_rotation_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "is_lock_rotation_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_freeze_enabled(&mut self, freeze_mode: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (freeze_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_freeze_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_freeze_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "is_freeze_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_freeze_mode(&mut self, freeze_mode: crate::classes::rigid_body_2d::FreezeMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::rigid_body_2d::FreezeMode,);
            let args = (freeze_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_freeze_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_freeze_mode(&self,) -> crate::classes::rigid_body_2d::FreezeMode {
            type CallRet = crate::classes::rigid_body_2d::FreezeMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_freeze_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of the bodies colliding with this one. Requires \\[member contact_monitor] to be set to `true` and \\[member max_contacts_reported] to be set high enough to detect all the collisions.\n\n**Note:** The result of this test is not immediate after moving objects. For performance, list of collisions is updated once per frame and before the physics step. Consider using signals instead."]
        pub fn get_colliding_bodies(&self,) -> Array < Gd < crate::classes::Node2D > > {
            type CallRet = Array < Gd < crate::classes::Node2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_colliding_bodies", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for RigidBody2D {
        type Base = crate::classes::PhysicsBody2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RigidBody2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RigidBody2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsBody2D > for RigidBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject2D > for RigidBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for RigidBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for RigidBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for RigidBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RigidBody2D {
        
    }
    impl crate::obj::cap::GodotDefault for RigidBody2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RigidBody2D {
        type Target = crate::classes::PhysicsBody2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RigidBody2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RigidBody2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RigidBody2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RigidBody2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsBody2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CollisionObject2D > for $Class {
                
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
#[doc = "Default-param extender for [`RigidBody2D::apply_central_impulse_ex`][super::RigidBody2D::apply_central_impulse_ex]."]
#[must_use]
pub struct ExApplyCentralImpulse < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RigidBody2D, impulse: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExApplyCentralImpulse < 'ex > {
    fn new(surround_object: &'ex mut re_export::RigidBody2D,) -> Self {
        let impulse = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse,
        }
    }
    #[inline]
    pub fn impulse(self, impulse: Vector2) -> Self {
        Self {
            impulse: impulse, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, impulse,
        }
        = self;
        re_export::RigidBody2D::apply_central_impulse_full(surround_object, impulse,)
    }
}
#[doc = "Default-param extender for [`RigidBody2D::apply_impulse_ex`][super::RigidBody2D::apply_impulse_ex]."]
#[must_use]
pub struct ExApplyImpulse < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RigidBody2D, impulse: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExApplyImpulse < 'ex > {
    fn new(surround_object: &'ex mut re_export::RigidBody2D, impulse: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
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
        re_export::RigidBody2D::apply_impulse_full(surround_object, impulse, position,)
    }
}
#[doc = "Default-param extender for [`RigidBody2D::apply_force_ex`][super::RigidBody2D::apply_force_ex]."]
#[must_use]
pub struct ExApplyForce < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RigidBody2D, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExApplyForce < 'ex > {
    fn new(surround_object: &'ex mut re_export::RigidBody2D, force: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force, position,
        }
        = self;
        re_export::RigidBody2D::apply_force_full(surround_object, force, position,)
    }
}
#[doc = "Default-param extender for [`RigidBody2D::add_constant_force_ex`][super::RigidBody2D::add_constant_force_ex]."]
#[must_use]
pub struct ExAddConstantForce < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RigidBody2D, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddConstantForce < 'ex > {
    fn new(surround_object: &'ex mut re_export::RigidBody2D, force: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force, position,
        }
        = self;
        re_export::RigidBody2D::add_constant_force_full(surround_object, force, position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FreezeMode {
    ord: i32
}
impl FreezeMode {
    #[doc(alias = "FREEZE_MODE_STATIC")]
    #[doc = "Godot enumerator name: `FREEZE_MODE_STATIC`"]
    pub const STATIC: FreezeMode = FreezeMode {
        ord: 0i32
    };
    #[doc(alias = "FREEZE_MODE_KINEMATIC")]
    #[doc = "Godot enumerator name: `FREEZE_MODE_KINEMATIC`"]
    pub const KINEMATIC: FreezeMode = FreezeMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for FreezeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FreezeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FreezeMode {
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
            Self::STATIC => "STATIC", Self::KINEMATIC => "KINEMATIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FreezeMode::STATIC, FreezeMode::KINEMATIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FreezeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STATIC", "FREEZE_MODE_STATIC", FreezeMode::STATIC), crate::meta::inspect::EnumConstant::new("KINEMATIC", "FREEZE_MODE_KINEMATIC", FreezeMode::KINEMATIC)]
        }
    }
}
impl crate::meta::GodotConvert for FreezeMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Freeze Mode Static", 0i64), EnumeratorShape::new_int("Freeze Mode Kinematic", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RigidBody2D.FreezeMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for FreezeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FreezeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for FreezeMode {
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
impl crate::registry::property::Export for FreezeMode {
    
}
impl crate::meta::Element for FreezeMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CenterOfMassMode {
    ord: i32
}
impl CenterOfMassMode {
    #[doc(alias = "CENTER_OF_MASS_MODE_AUTO")]
    #[doc = "Godot enumerator name: `CENTER_OF_MASS_MODE_AUTO`"]
    pub const AUTO: CenterOfMassMode = CenterOfMassMode {
        ord: 0i32
    };
    #[doc(alias = "CENTER_OF_MASS_MODE_CUSTOM")]
    #[doc = "Godot enumerator name: `CENTER_OF_MASS_MODE_CUSTOM`"]
    pub const CUSTOM: CenterOfMassMode = CenterOfMassMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for CenterOfMassMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CenterOfMassMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CenterOfMassMode {
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
            Self::AUTO => "AUTO", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CenterOfMassMode::AUTO, CenterOfMassMode::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CenterOfMassMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("AUTO", "CENTER_OF_MASS_MODE_AUTO", CenterOfMassMode::AUTO), crate::meta::inspect::EnumConstant::new("CUSTOM", "CENTER_OF_MASS_MODE_CUSTOM", CenterOfMassMode::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for CenterOfMassMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Center Of Mass Mode Auto", 0i64), EnumeratorShape::new_int("Center Of Mass Mode Custom", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RigidBody2D.CenterOfMassMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CenterOfMassMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CenterOfMassMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CenterOfMassMode {
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
impl crate::registry::property::Export for CenterOfMassMode {
    
}
impl crate::meta::Element for CenterOfMassMode {
    
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RigidBody2D.DampMode")), is_bitfield: false,
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
#[doc = "Godot enum name: `CCDMode`."]
pub struct CcdMode {
    ord: i32
}
impl CcdMode {
    #[doc(alias = "CCD_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `CCD_MODE_DISABLED`"]
    pub const DISABLED: CcdMode = CcdMode {
        ord: 0i32
    };
    #[doc(alias = "CCD_MODE_CAST_RAY")]
    #[doc = "Godot enumerator name: `CCD_MODE_CAST_RAY`"]
    pub const CAST_RAY: CcdMode = CcdMode {
        ord: 1i32
    };
    #[doc(alias = "CCD_MODE_CAST_SHAPE")]
    #[doc = "Godot enumerator name: `CCD_MODE_CAST_SHAPE`"]
    pub const CAST_SHAPE: CcdMode = CcdMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CcdMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CcdMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CcdMode {
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
            Self::DISABLED => "DISABLED", Self::CAST_RAY => "CAST_RAY", Self::CAST_SHAPE => "CAST_SHAPE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CcdMode::DISABLED, CcdMode::CAST_RAY, CcdMode::CAST_SHAPE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CcdMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "CCD_MODE_DISABLED", CcdMode::DISABLED), crate::meta::inspect::EnumConstant::new("CAST_RAY", "CCD_MODE_CAST_RAY", CcdMode::CAST_RAY), crate::meta::inspect::EnumConstant::new("CAST_SHAPE", "CCD_MODE_CAST_SHAPE", CcdMode::CAST_SHAPE)]
        }
    }
}
impl crate::meta::GodotConvert for CcdMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Ccd Mode Disabled", 0i64), EnumeratorShape::new_int("Ccd Mode Cast Ray", 1i64), EnumeratorShape::new_int("Ccd Mode Cast Shape", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RigidBody2D.CCDMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CcdMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CcdMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CcdMode {
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
impl crate::registry::property::Export for CcdMode {
    
}
impl crate::meta::Element for CcdMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RigidBody2D;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`RigidBody2D`][crate::classes::RigidBody2D] class."]
    pub struct SignalsOfRigidBody2D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfRigidBody2D < 'c, C > {
        #[doc = "Signature: `(body_rid: Rid, body: Gd<Node>, body_shape_index: i64, local_shape_index: i64)`"]
        pub fn body_shape_entered(&mut self) -> SigBodyShapeEntered < 'c, C > {
            SigBodyShapeEntered {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "body_shape_entered")
            }
        }
        #[doc = "Signature: `(body_rid: Rid, body: Gd<Node>, body_shape_index: i64, local_shape_index: i64)`"]
        pub fn body_shape_exited(&mut self) -> SigBodyShapeExited < 'c, C > {
            SigBodyShapeExited {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "body_shape_exited")
            }
        }
        #[doc = "Signature: `(body: Gd<Node>)`"]
        pub fn body_entered(&mut self) -> SigBodyEntered < 'c, C > {
            SigBodyEntered {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "body_entered")
            }
        }
        #[doc = "Signature: `(body: Gd<Node>)`"]
        pub fn body_exited(&mut self) -> SigBodyExited < 'c, C > {
            SigBodyExited {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "body_exited")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn sleeping_state_changed(&mut self) -> SigSleepingStateChanged < 'c, C > {
            SigSleepingStateChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "sleeping_state_changed")
            }
        }
    }
    type TypedSigBodyShapeEntered < 'c, C > = TypedSignal < 'c, C, (Rid, Gd < crate::classes::Node >, i64, i64,) >;
    pub struct SigBodyShapeEntered < 'c, C: WithSignals > {
        typed: TypedSigBodyShapeEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyShapeEntered < 'c, C > {
        pub fn emit(&mut self, body_rid: Rid, body: Gd < crate::classes::Node >, body_shape_index: i64, local_shape_index: i64,) {
            self.typed.emit_tuple((body_rid, body, body_shape_index, local_shape_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBodyShapeEntered < 'c, C > {
        type Target = TypedSigBodyShapeEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBodyShapeEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigBodyShapeExited < 'c, C > = TypedSignal < 'c, C, (Rid, Gd < crate::classes::Node >, i64, i64,) >;
    pub struct SigBodyShapeExited < 'c, C: WithSignals > {
        typed: TypedSigBodyShapeExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyShapeExited < 'c, C > {
        pub fn emit(&mut self, body_rid: Rid, body: Gd < crate::classes::Node >, body_shape_index: i64, local_shape_index: i64,) {
            self.typed.emit_tuple((body_rid, body, body_shape_index, local_shape_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBodyShapeExited < 'c, C > {
        type Target = TypedSigBodyShapeExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBodyShapeExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigBodyEntered < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigBodyEntered < 'c, C: WithSignals > {
        typed: TypedSigBodyEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyEntered < 'c, C > {
        pub fn emit(&mut self, body: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((body,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBodyEntered < 'c, C > {
        type Target = TypedSigBodyEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBodyEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigBodyExited < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigBodyExited < 'c, C: WithSignals > {
        typed: TypedSigBodyExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyExited < 'c, C > {
        pub fn emit(&mut self, body: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((body,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBodyExited < 'c, C > {
        type Target = TypedSigBodyExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBodyExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSleepingStateChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSleepingStateChanged < 'c, C: WithSignals > {
        typed: TypedSigSleepingStateChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSleepingStateChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSleepingStateChanged < 'c, C > {
        type Target = TypedSigSleepingStateChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSleepingStateChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for RigidBody2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfRigidBody2D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfRigidBody2D < 'c, C > {
        type Target = < < RigidBody2D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = RigidBody2D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfRigidBody2D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = RigidBody2D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
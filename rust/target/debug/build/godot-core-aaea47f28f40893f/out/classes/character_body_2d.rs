#![doc = "Sidecar module for class [`CharacterBody2D`][crate::classes::CharacterBody2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CharacterBody2D` enums](https://docs.godotengine.org/en/stable/classes/class_characterbody2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CharacterBody2D`.\n\nInherits [`PhysicsBody2D`][crate::classes::PhysicsBody2D].\n\nRelated symbols:\n\n* [`character_body_2d`][crate::classes::character_body_2d]: sidecar module with related enum/flag types\n* [`ICharacterBody2D`][crate::classes::ICharacterBody2D]: virtual methods\n\n\nSee also [Godot docs for `CharacterBody2D`](https://docs.godotengine.org/en/stable/classes/class_characterbody2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CharacterBody2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\n`CharacterBody2D` is a specialized class for physics bodies that are meant to be user-controlled. They are not affected by physics at all, but they affect other physics bodies in their path. They are mainly used to provide high-level API to move objects with wall and slope detection ([`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`] method) in addition to the general collision detection provided by [`move_and_collide`][`crate::classes::PhysicsBody2D::move_and_collide`]. This makes it useful for highly configurable physics bodies that must move in specific ways and collide with the world, as is often the case with user-controlled characters.\n\nFor game objects that don't require complex movement or collision detection, such as moving platforms, [`AnimatableBody2D`][crate::classes::AnimatableBody2D] is simpler to configure."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CharacterBody2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CharacterBody2D`][crate::classes::CharacterBody2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IPhysicsBody2D`~~ > ~~`ICollisionObject2D`~~ > [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `CharacterBody2D` methods](https://docs.godotengine.org/en/stable/classes/class_characterbody2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICharacterBody2D: crate::obj::GodotClass < Base = CharacterBody2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CharacterBody2D {
        #[doc = "Moves the body based on \\[member velocity]. If the body collides with another, it will slide along the other body (by default only on floor) rather than stop immediately. If the other body is a `CharacterBody2D` or [`RigidBody2D`][crate::classes::RigidBody2D], it will also be affected by the motion of the other body. You can use this to make moving and rotating platforms, or to make nodes push other nodes.\n\nThis method should be used in [`physics_process`][`crate::classes::INode::physics_process`] (or in a method called by [`physics_process`][`crate::classes::INode::physics_process`]), as it uses the physics step's `delta` value automatically in calculations. Otherwise, the simulation will run at an incorrect speed.\n\nModifies \\[member velocity] if a slide collision occurred. To get the latest collision call [`get_last_slide_collision`][`crate::classes::CharacterBody2D::get_last_slide_collision`], for detailed information about collisions that occurred, use [`get_slide_collision`][`crate::classes::CharacterBody2D::get_slide_collision`].\n\nWhen the body touches a moving platform, the platform's velocity is automatically added to the body motion. If a collision occurs due to the platform's motion, it will always be first in the slide collisions.\n\nThe general behavior and available properties change according to the \\[member motion_mode].\n\nReturns `true` if the body collided, otherwise, returns `false`."]
        pub fn move_and_slide(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "move_and_slide", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Allows to manually apply a snap to the floor regardless of the body's velocity. This function does nothing when [`is_on_floor`][`crate::classes::CharacterBody2D::is_on_floor`] returns `true`."]
        pub fn apply_floor_snap(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "apply_floor_snap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_velocity(&mut self, velocity: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_velocity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_safe_margin(&mut self, margin: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_safe_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_safe_margin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_safe_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_floor_stop_on_slope_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "is_floor_stop_on_slope_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_floor_stop_on_slope_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_floor_stop_on_slope_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_floor_constant_speed_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_floor_constant_speed_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_floor_constant_speed_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "is_floor_constant_speed_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_floor_block_on_wall_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_floor_block_on_wall_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_floor_block_on_wall_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "is_floor_block_on_wall_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_slide_on_ceiling_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_slide_on_ceiling_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_slide_on_ceiling_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "is_slide_on_ceiling_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_platform_floor_layers(&mut self, exclude_layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (exclude_layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_platform_floor_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_platform_floor_layers(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_platform_floor_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_platform_wall_layers(&mut self, exclude_layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (exclude_layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1474usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_platform_wall_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_platform_wall_layers(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_platform_wall_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_slides(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_max_slides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_slides(&mut self, max_slides: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_slides,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_max_slides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_floor_max_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_floor_max_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_floor_max_angle(&mut self, radians: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_floor_max_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_floor_snap_length(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_floor_snap_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_floor_snap_length(&mut self, floor_snap_length: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (floor_snap_length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_floor_snap_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_wall_min_slide_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_wall_min_slide_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_wall_min_slide_angle(&mut self, radians: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_wall_min_slide_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_up_direction(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1484usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_up_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_up_direction(&mut self, up_direction: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (up_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1485usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_up_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_motion_mode(&mut self, mode: crate::classes::character_body_2d::MotionMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::character_body_2d::MotionMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1486usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_motion_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_motion_mode(&self,) -> crate::classes::character_body_2d::MotionMode {
            type CallRet = crate::classes::character_body_2d::MotionMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1487usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_motion_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_platform_on_leave(&mut self, on_leave_apply_velocity: crate::classes::character_body_2d::PlatformOnLeave,) {
            type CallRet = ();
            type CallParams = (crate::classes::character_body_2d::PlatformOnLeave,);
            let args = (on_leave_apply_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1488usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "set_platform_on_leave", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_platform_on_leave(&self,) -> crate::classes::character_body_2d::PlatformOnLeave {
            type CallRet = crate::classes::character_body_2d::PlatformOnLeave;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1489usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_platform_on_leave", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the body collided with the floor on the last call of [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]. Otherwise, returns `false`. The \\[member up_direction] and \\[member floor_max_angle] are used to determine whether a surface is \"floor\" or not."]
        pub fn is_on_floor(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1490usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "is_on_floor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the body collided only with the floor on the last call of [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]. Otherwise, returns `false`. The \\[member up_direction] and \\[member floor_max_angle] are used to determine whether a surface is \"floor\" or not."]
        pub fn is_on_floor_only(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1491usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "is_on_floor_only", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the body collided with the ceiling on the last call of [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]. Otherwise, returns `false`. The \\[member up_direction] and \\[member floor_max_angle] are used to determine whether a surface is \"ceiling\" or not."]
        pub fn is_on_ceiling(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1492usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "is_on_ceiling", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the body collided only with the ceiling on the last call of [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]. Otherwise, returns `false`. The \\[member up_direction] and \\[member floor_max_angle] are used to determine whether a surface is \"ceiling\" or not."]
        pub fn is_on_ceiling_only(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1493usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "is_on_ceiling_only", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the body collided with a wall on the last call of [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]. Otherwise, returns `false`. The \\[member up_direction] and \\[member floor_max_angle] are used to determine whether a surface is \"wall\" or not."]
        pub fn is_on_wall(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1494usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "is_on_wall", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the body collided only with a wall on the last call of [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]. Otherwise, returns `false`. The \\[member up_direction] and \\[member floor_max_angle] are used to determine whether a surface is \"wall\" or not."]
        pub fn is_on_wall_only(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1495usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "is_on_wall_only", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the collision normal of the floor at the last collision point. Only valid after calling [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`] and when [`is_on_floor`][`crate::classes::CharacterBody2D::is_on_floor`] returns `true`.\n\n**Warning:** The collision normal is not always the same as the surface normal."]
        pub fn get_floor_normal(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1496usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_floor_normal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the collision normal of the wall at the last collision point. Only valid after calling [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`] and when [`is_on_wall`][`crate::classes::CharacterBody2D::is_on_wall`] returns `true`.\n\n**Warning:** The collision normal is not always the same as the surface normal."]
        pub fn get_wall_normal(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1497usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_wall_normal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the last motion applied to the `CharacterBody2D` during the last call to [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]. The movement can be split into multiple motions when sliding occurs, and this method return the last one, which is useful to retrieve the current direction of the movement."]
        pub fn get_last_motion(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1498usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_last_motion", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the travel (position delta) that occurred during the last call to [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]."]
        pub fn get_position_delta(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1499usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_position_delta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current real velocity since the last call to [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]. For example, when you climb a slope, you will move diagonally even though the velocity is horizontal. This method returns the diagonal movement, as opposed to \\[member velocity] which returns the requested velocity."]
        pub fn get_real_velocity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1500usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_real_velocity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the floor's collision angle at the last collision point according to `up_direction`, which is `Vector2.UP` by default. This value is always positive and only valid after calling [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`] and when [`is_on_floor`][`crate::classes::CharacterBody2D::is_on_floor`] returns `true`."]
        pub(crate) fn get_floor_angle_full(&self, up_direction: Vector2,) -> f32 {
            type CallRet = f32;
            type CallParams = (Vector2,);
            let args = (up_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1501usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_floor_angle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_floor_angle_ex`][Self::get_floor_angle_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the floor's collision angle at the last collision point according to `up_direction`, which is `Vector2.UP` by default. This value is always positive and only valid after calling [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`] and when [`is_on_floor`][`crate::classes::CharacterBody2D::is_on_floor`] returns `true`."]
        #[inline]
        pub fn get_floor_angle(&self,) -> f32 {
            self.get_floor_angle_ex() . done()
        }
        #[doc = "Returns the floor's collision angle at the last collision point according to `up_direction`, which is `Vector2.UP` by default. This value is always positive and only valid after calling [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`] and when [`is_on_floor`][`crate::classes::CharacterBody2D::is_on_floor`] returns `true`."]
        #[inline]
        pub fn get_floor_angle_ex < 'ex > (&'ex self,) -> ExGetFloorAngle < 'ex > {
            ExGetFloorAngle::new(self,)
        }
        #[doc = "Returns the linear velocity of the platform at the last collision point. Only valid after calling [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]."]
        pub fn get_platform_velocity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1502usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_platform_velocity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of times the body collided and changed direction during the last call to [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]."]
        pub fn get_slide_collision_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1503usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_slide_collision_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`KinematicCollision2D`][crate::classes::KinematicCollision2D], which contains information about a collision that occurred during the last call to [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]. Since the body can collide several times in a single call to [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`], you must specify the index of the collision in the range 0 to ([`get_slide_collision_count`][`crate::classes::CharacterBody2D::get_slide_collision_count`] - 1). See also [`get_last_slide_collision`][`crate::classes::CharacterBody2D::get_last_slide_collision`].\n\n**Example:** Iterate through the collisions with a `for` loop:\n\n\n```gdscript\nfor i in get_slide_collision_count():\n\tvar collision = get_slide_collision(i)\n\tprint(\"Collided with: \", collision.get_collider().name)\n```\n"]
        pub fn get_slide_collision(&self, slide_idx: i32,) -> Option < Gd < crate::classes::KinematicCollision2D > > {
            type CallRet = Option < Gd < crate::classes::KinematicCollision2D > >;
            type CallParams = (i32,);
            let args = (slide_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1504usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_slide_collision", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`KinematicCollision2D`][crate::classes::KinematicCollision2D] if a collision occurred. The returned value contains information about the latest collision that occurred during the last call to [`move_and_slide`][`crate::classes::CharacterBody2D::move_and_slide`]. Returns `null` if no collision occurred. See also [`get_slide_collision`][`crate::classes::CharacterBody2D::get_slide_collision`]."]
        pub fn get_last_slide_collision(&self,) -> Option < Gd < crate::classes::KinematicCollision2D > > {
            type CallRet = Option < Gd < crate::classes::KinematicCollision2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1505usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody2D", "get_last_slide_collision", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CharacterBody2D {
        type Base = crate::classes::PhysicsBody2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CharacterBody2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CharacterBody2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsBody2D > for CharacterBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject2D > for CharacterBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for CharacterBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for CharacterBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CharacterBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CharacterBody2D {
        
    }
    impl crate::obj::cap::GodotDefault for CharacterBody2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CharacterBody2D {
        type Target = crate::classes::PhysicsBody2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CharacterBody2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CharacterBody2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CharacterBody2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CharacterBody2D > for $Class {
                
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
#[doc = "Default-param extender for [`CharacterBody2D::get_floor_angle_ex`][super::CharacterBody2D::get_floor_angle_ex]."]
#[must_use]
pub struct ExGetFloorAngle < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::CharacterBody2D, up_direction: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetFloorAngle < 'ex > {
    fn new(surround_object: &'ex re_export::CharacterBody2D,) -> Self {
        let up_direction = Vector2::new(0 as _, - 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, up_direction: up_direction,
        }
    }
    #[inline]
    pub fn up_direction(self, up_direction: Vector2) -> Self {
        Self {
            up_direction: up_direction, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, up_direction,
        }
        = self;
        re_export::CharacterBody2D::get_floor_angle_full(surround_object, up_direction,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MotionMode {
    ord: i32
}
impl MotionMode {
    #[doc(alias = "MOTION_MODE_GROUNDED")]
    #[doc = "Godot enumerator name: `MOTION_MODE_GROUNDED`"]
    pub const GROUNDED: MotionMode = MotionMode {
        ord: 0i32
    };
    #[doc(alias = "MOTION_MODE_FLOATING")]
    #[doc = "Godot enumerator name: `MOTION_MODE_FLOATING`"]
    pub const FLOATING: MotionMode = MotionMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for MotionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MotionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MotionMode {
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
            Self::GROUNDED => "GROUNDED", Self::FLOATING => "FLOATING", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MotionMode::GROUNDED, MotionMode::FLOATING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MotionMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GROUNDED", "MOTION_MODE_GROUNDED", MotionMode::GROUNDED), crate::meta::inspect::EnumConstant::new("FLOATING", "MOTION_MODE_FLOATING", MotionMode::FLOATING)]
        }
    }
}
impl crate::meta::GodotConvert for MotionMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Motion Mode Grounded", 0i64), EnumeratorShape::new_int("Motion Mode Floating", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CharacterBody2D.MotionMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for MotionMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MotionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for MotionMode {
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
impl crate::registry::property::Export for MotionMode {
    
}
impl crate::meta::Element for MotionMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlatformOnLeave {
    ord: i32
}
impl PlatformOnLeave {
    #[doc(alias = "PLATFORM_ON_LEAVE_ADD_VELOCITY")]
    #[doc = "Godot enumerator name: `PLATFORM_ON_LEAVE_ADD_VELOCITY`"]
    pub const ADD_VELOCITY: PlatformOnLeave = PlatformOnLeave {
        ord: 0i32
    };
    #[doc(alias = "PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY")]
    #[doc = "Godot enumerator name: `PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY`"]
    pub const ADD_UPWARD_VELOCITY: PlatformOnLeave = PlatformOnLeave {
        ord: 1i32
    };
    #[doc(alias = "PLATFORM_ON_LEAVE_DO_NOTHING")]
    #[doc = "Godot enumerator name: `PLATFORM_ON_LEAVE_DO_NOTHING`"]
    pub const DO_NOTHING: PlatformOnLeave = PlatformOnLeave {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PlatformOnLeave {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PlatformOnLeave") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PlatformOnLeave {
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
            Self::ADD_VELOCITY => "ADD_VELOCITY", Self::ADD_UPWARD_VELOCITY => "ADD_UPWARD_VELOCITY", Self::DO_NOTHING => "DO_NOTHING", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PlatformOnLeave::ADD_VELOCITY, PlatformOnLeave::ADD_UPWARD_VELOCITY, PlatformOnLeave::DO_NOTHING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PlatformOnLeave >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ADD_VELOCITY", "PLATFORM_ON_LEAVE_ADD_VELOCITY", PlatformOnLeave::ADD_VELOCITY), crate::meta::inspect::EnumConstant::new("ADD_UPWARD_VELOCITY", "PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY", PlatformOnLeave::ADD_UPWARD_VELOCITY), crate::meta::inspect::EnumConstant::new("DO_NOTHING", "PLATFORM_ON_LEAVE_DO_NOTHING", PlatformOnLeave::DO_NOTHING)]
        }
    }
}
impl crate::meta::GodotConvert for PlatformOnLeave {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Platform On Leave Add Velocity", 0i64), EnumeratorShape::new_int("Platform On Leave Add Upward Velocity", 1i64), EnumeratorShape::new_int("Platform On Leave Do Nothing", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CharacterBody2D.PlatformOnLeave")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PlatformOnLeave {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PlatformOnLeave {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PlatformOnLeave {
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
impl crate::registry::property::Export for PlatformOnLeave {
    
}
impl crate::meta::Element for PlatformOnLeave {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CharacterBody2D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::collision_object_2d::SignalsOfCollisionObject2D;
    impl WithSignals for CharacterBody2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCollisionObject2D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
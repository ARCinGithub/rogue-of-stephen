#![doc = "Sidecar module for class [`Area2D`][crate::classes::Area2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Area2D` enums](https://docs.godotengine.org/en/stable/classes/class_area2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Area2D`.\n\nInherits [`CollisionObject2D`][crate::classes::CollisionObject2D].\n\nRelated symbols:\n\n* [`area_2d`][crate::classes::area_2d]: sidecar module with related enum/flag types\n* [`IArea2D`][crate::classes::IArea2D]: virtual methods\n* [`SignalsOfArea2D`][crate::classes::area_2d::SignalsOfArea2D]: signal collection\n\n\nSee also [Godot docs for `Area2D`](https://docs.godotengine.org/en/stable/classes/class_area2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Area2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\n`Area2D` is a region of 2D space defined by one or multiple [`CollisionShape2D`][crate::classes::CollisionShape2D] or [`CollisionPolygon2D`][crate::classes::CollisionPolygon2D] child nodes. It detects when other [`CollisionObject2D`][crate::classes::CollisionObject2D]s enter or exit it, and it also keeps track of which collision objects haven't exited it yet (i.e. which one are overlapping it).\n\nThis node can also locally alter or override physics parameters (gravity, damping) and route audio to custom audio buses.\n\n**Note:** Areas and bodies created with [`PhysicsServer2D`][crate::classes::PhysicsServer2D] might not interact as expected with `Area2D`s, and might not emit signals or track objects correctly."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Area2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Area2D`][crate::classes::Area2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`ICollisionObject2D`~~ > [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Area2D` methods](https://docs.godotengine.org/en/stable/classes/class_area2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IArea2D: crate::obj::GodotClass < Base = Area2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Area2D {
        pub fn set_gravity_space_override_mode(&mut self, space_override_mode: crate::classes::area_2d::SpaceOverride,) {
            type CallRet = ();
            type CallParams = (crate::classes::area_2d::SpaceOverride,);
            let args = (space_override_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_gravity_space_override_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_space_override_mode(&self,) -> crate::classes::area_2d::SpaceOverride {
            type CallRet = crate::classes::area_2d::SpaceOverride;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_gravity_space_override_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_is_point(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_gravity_is_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_gravity_a_point(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "is_gravity_a_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_point_unit_distance(&mut self, distance_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_gravity_point_unit_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_point_unit_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_gravity_point_unit_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_point_center(&mut self, center: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (center,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_gravity_point_center", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_point_center(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_gravity_point_center", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_direction(&mut self, direction: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_gravity_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_direction(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_gravity_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity(&mut self, gravity: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (gravity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_damp_space_override_mode(&mut self, space_override_mode: crate::classes::area_2d::SpaceOverride,) {
            type CallRet = ();
            type CallParams = (crate::classes::area_2d::SpaceOverride,);
            let args = (space_override_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_linear_damp_space_override_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_damp_space_override_mode(&self,) -> crate::classes::area_2d::SpaceOverride {
            type CallRet = crate::classes::area_2d::SpaceOverride;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_linear_damp_space_override_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_damp_space_override_mode(&mut self, space_override_mode: crate::classes::area_2d::SpaceOverride,) {
            type CallRet = ();
            type CallParams = (crate::classes::area_2d::SpaceOverride,);
            let args = (space_override_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_angular_damp_space_override_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_damp_space_override_mode(&self,) -> crate::classes::area_2d::SpaceOverride {
            type CallRet = crate::classes::area_2d::SpaceOverride;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_angular_damp_space_override_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_damp(&mut self, linear_damp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (linear_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_damp(&mut self, angular_damp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angular_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_priority(&mut self, priority: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_priority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_monitoring(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_monitoring", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_monitoring(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "is_monitoring", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_monitorable(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_monitorable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_monitorable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "is_monitorable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of intersecting [`PhysicsBody2D`][crate::classes::PhysicsBody2D]s and [`TileMap`][crate::classes::TileMap]s. The overlapping body's \\[member CollisionObject2D.collision_layer] must be part of this area's \\[member CollisionObject2D.collision_mask] in order to be detected.\n\nFor performance reasons (collisions are all processed at the same time) this list is modified once during the physics step, not immediately after objects are moved. Consider using signals instead."]
        pub fn get_overlapping_bodies(&self,) -> Array < Gd < crate::classes::Node2D > > {
            type CallRet = Array < Gd < crate::classes::Node2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_overlapping_bodies", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of intersecting `Area2D`s. The overlapping area's \\[member CollisionObject2D.collision_layer] must be part of this area's \\[member CollisionObject2D.collision_mask] in order to be detected.\n\nFor performance reasons (collisions are all processed at the same time) this list is modified once during the physics step, not immediately after objects are moved. Consider using signals instead."]
        pub fn get_overlapping_areas(&self,) -> Array < Gd < crate::classes::Area2D > > {
            type CallRet = Array < Gd < crate::classes::Area2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1436usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_overlapping_areas", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if intersecting any [`PhysicsBody2D`][crate::classes::PhysicsBody2D]s or [`TileMap`][crate::classes::TileMap]s, otherwise returns `false`. The overlapping body's \\[member CollisionObject2D.collision_layer] must be part of this area's \\[member CollisionObject2D.collision_mask] in order to be detected.\n\nFor performance reasons (collisions are all processed at the same time) the list of overlapping bodies is modified once during the physics step, not immediately after objects are moved. Consider using signals instead."]
        pub fn has_overlapping_bodies(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "has_overlapping_bodies", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if intersecting any `Area2D`s, otherwise returns `false`. The overlapping area's \\[member CollisionObject2D.collision_layer] must be part of this area's \\[member CollisionObject2D.collision_mask] in order to be detected.\n\nFor performance reasons (collisions are all processed at the same time) the list of overlapping areas is modified once during the physics step, not immediately after objects are moved. Consider using signals instead."]
        pub fn has_overlapping_areas(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1438usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "has_overlapping_areas", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given physics body intersects or overlaps this `Area2D`, `false` otherwise.\n\n**Note:** The result of this test is not immediate after moving objects. For performance, list of overlaps is updated once per frame and before the physics step. Consider using signals instead.\n\nThe `body` argument can either be a [`PhysicsBody2D`][crate::classes::PhysicsBody2D] or a [`TileMap`][crate::classes::TileMap] instance. While TileMaps are not physics bodies themselves, they register their tiles with collision shapes as a virtual physics body."]
        pub fn overlaps_body(&self, body: impl AsArg < Gd < crate::classes::Node >>,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >,);
            let args = (body.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "overlaps_body", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `Area2D` intersects or overlaps this `Area2D`, `false` otherwise.\n\n**Note:** The result of this test is not immediate after moving objects. For performance, the list of overlaps is updated once per frame and before the physics step. Consider using signals instead."]
        pub fn overlaps_area(&self, area: impl AsArg < Gd < crate::classes::Node >>,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >,);
            let args = (area.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "overlaps_area", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_audio_bus_name(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_audio_bus_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_audio_bus_name(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_audio_bus_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_audio_bus_override(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_audio_bus_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_overriding_audio_bus(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "is_overriding_audio_bus", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Area2D {
        type Base = crate::classes::CollisionObject2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Area2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Area2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject2D > for Area2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for Area2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Area2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Area2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Area2D {
        
    }
    impl crate::obj::cap::GodotDefault for Area2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Area2D {
        type Target = crate::classes::CollisionObject2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Area2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Area2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Area2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Area2D > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpaceOverride {
    ord: i32
}
impl SpaceOverride {
    #[doc(alias = "SPACE_OVERRIDE_DISABLED")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_DISABLED`"]
    pub const DISABLED: SpaceOverride = SpaceOverride {
        ord: 0i32
    };
    #[doc(alias = "SPACE_OVERRIDE_COMBINE")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_COMBINE`"]
    pub const COMBINE: SpaceOverride = SpaceOverride {
        ord: 1i32
    };
    #[doc(alias = "SPACE_OVERRIDE_COMBINE_REPLACE")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_COMBINE_REPLACE`"]
    pub const COMBINE_REPLACE: SpaceOverride = SpaceOverride {
        ord: 2i32
    };
    #[doc(alias = "SPACE_OVERRIDE_REPLACE")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_REPLACE`"]
    pub const REPLACE: SpaceOverride = SpaceOverride {
        ord: 3i32
    };
    #[doc(alias = "SPACE_OVERRIDE_REPLACE_COMBINE")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_REPLACE_COMBINE`"]
    pub const REPLACE_COMBINE: SpaceOverride = SpaceOverride {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for SpaceOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpaceOverride") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpaceOverride {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::COMBINE => "COMBINE", Self::COMBINE_REPLACE => "COMBINE_REPLACE", Self::REPLACE => "REPLACE", Self::REPLACE_COMBINE => "REPLACE_COMBINE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SpaceOverride::DISABLED, SpaceOverride::COMBINE, SpaceOverride::COMBINE_REPLACE, SpaceOverride::REPLACE, SpaceOverride::REPLACE_COMBINE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SpaceOverride >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "SPACE_OVERRIDE_DISABLED", SpaceOverride::DISABLED), crate::meta::inspect::EnumConstant::new("COMBINE", "SPACE_OVERRIDE_COMBINE", SpaceOverride::COMBINE), crate::meta::inspect::EnumConstant::new("COMBINE_REPLACE", "SPACE_OVERRIDE_COMBINE_REPLACE", SpaceOverride::COMBINE_REPLACE), crate::meta::inspect::EnumConstant::new("REPLACE", "SPACE_OVERRIDE_REPLACE", SpaceOverride::REPLACE), crate::meta::inspect::EnumConstant::new("REPLACE_COMBINE", "SPACE_OVERRIDE_REPLACE_COMBINE", SpaceOverride::REPLACE_COMBINE)]
        }
    }
}
impl crate::meta::GodotConvert for SpaceOverride {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Space Override Disabled", 0i64), EnumeratorShape::new_int("Space Override Combine", 1i64), EnumeratorShape::new_int("Space Override Combine Replace", 2i64), EnumeratorShape::new_int("Space Override Replace", 3i64), EnumeratorShape::new_int("Space Override Replace Combine", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Area2D.SpaceOverride")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SpaceOverride {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpaceOverride {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SpaceOverride {
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
impl crate::registry::property::Export for SpaceOverride {
    
}
impl crate::meta::Element for SpaceOverride {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Area2D;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Area2D`][crate::classes::Area2D] class."]
    pub struct SignalsOfArea2D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfArea2D < 'c, C > {
        #[doc = "Signature: `(body_rid: Rid, body: Gd<Node2D>, body_shape_index: i64, local_shape_index: i64)`"]
        pub fn body_shape_entered(&mut self) -> SigBodyShapeEntered < 'c, C > {
            SigBodyShapeEntered {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "body_shape_entered")
            }
        }
        #[doc = "Signature: `(body_rid: Rid, body: Gd<Node2D>, body_shape_index: i64, local_shape_index: i64)`"]
        pub fn body_shape_exited(&mut self) -> SigBodyShapeExited < 'c, C > {
            SigBodyShapeExited {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "body_shape_exited")
            }
        }
        #[doc = "Signature: `(body: Gd<Node2D>)`"]
        pub fn body_entered(&mut self) -> SigBodyEntered < 'c, C > {
            SigBodyEntered {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "body_entered")
            }
        }
        #[doc = "Signature: `(body: Gd<Node2D>)`"]
        pub fn body_exited(&mut self) -> SigBodyExited < 'c, C > {
            SigBodyExited {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "body_exited")
            }
        }
        #[doc = "Signature: `(area_rid: Rid, area: Gd<Area2D>, area_shape_index: i64, local_shape_index: i64)`"]
        pub fn area_shape_entered(&mut self) -> SigAreaShapeEntered < 'c, C > {
            SigAreaShapeEntered {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "area_shape_entered")
            }
        }
        #[doc = "Signature: `(area_rid: Rid, area: Gd<Area2D>, area_shape_index: i64, local_shape_index: i64)`"]
        pub fn area_shape_exited(&mut self) -> SigAreaShapeExited < 'c, C > {
            SigAreaShapeExited {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "area_shape_exited")
            }
        }
        #[doc = "Signature: `(area: Gd<Area2D>)`"]
        pub fn area_entered(&mut self) -> SigAreaEntered < 'c, C > {
            SigAreaEntered {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "area_entered")
            }
        }
        #[doc = "Signature: `(area: Gd<Area2D>)`"]
        pub fn area_exited(&mut self) -> SigAreaExited < 'c, C > {
            SigAreaExited {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "area_exited")
            }
        }
    }
    type TypedSigBodyShapeEntered < 'c, C > = TypedSignal < 'c, C, (Rid, Gd < crate::classes::Node2D >, i64, i64,) >;
    pub struct SigBodyShapeEntered < 'c, C: WithSignals > {
        typed: TypedSigBodyShapeEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyShapeEntered < 'c, C > {
        pub fn emit(&mut self, body_rid: Rid, body: Gd < crate::classes::Node2D >, body_shape_index: i64, local_shape_index: i64,) {
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
    type TypedSigBodyShapeExited < 'c, C > = TypedSignal < 'c, C, (Rid, Gd < crate::classes::Node2D >, i64, i64,) >;
    pub struct SigBodyShapeExited < 'c, C: WithSignals > {
        typed: TypedSigBodyShapeExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyShapeExited < 'c, C > {
        pub fn emit(&mut self, body_rid: Rid, body: Gd < crate::classes::Node2D >, body_shape_index: i64, local_shape_index: i64,) {
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
    type TypedSigBodyEntered < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node2D >,) >;
    pub struct SigBodyEntered < 'c, C: WithSignals > {
        typed: TypedSigBodyEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyEntered < 'c, C > {
        pub fn emit(&mut self, body: Gd < crate::classes::Node2D >,) {
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
    type TypedSigBodyExited < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node2D >,) >;
    pub struct SigBodyExited < 'c, C: WithSignals > {
        typed: TypedSigBodyExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyExited < 'c, C > {
        pub fn emit(&mut self, body: Gd < crate::classes::Node2D >,) {
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
    type TypedSigAreaShapeEntered < 'c, C > = TypedSignal < 'c, C, (Rid, Gd < crate::classes::Area2D >, i64, i64,) >;
    pub struct SigAreaShapeEntered < 'c, C: WithSignals > {
        typed: TypedSigAreaShapeEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAreaShapeEntered < 'c, C > {
        pub fn emit(&mut self, area_rid: Rid, area: Gd < crate::classes::Area2D >, area_shape_index: i64, local_shape_index: i64,) {
            self.typed.emit_tuple((area_rid, area, area_shape_index, local_shape_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAreaShapeEntered < 'c, C > {
        type Target = TypedSigAreaShapeEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAreaShapeEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAreaShapeExited < 'c, C > = TypedSignal < 'c, C, (Rid, Gd < crate::classes::Area2D >, i64, i64,) >;
    pub struct SigAreaShapeExited < 'c, C: WithSignals > {
        typed: TypedSigAreaShapeExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAreaShapeExited < 'c, C > {
        pub fn emit(&mut self, area_rid: Rid, area: Gd < crate::classes::Area2D >, area_shape_index: i64, local_shape_index: i64,) {
            self.typed.emit_tuple((area_rid, area, area_shape_index, local_shape_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAreaShapeExited < 'c, C > {
        type Target = TypedSigAreaShapeExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAreaShapeExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAreaEntered < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Area2D >,) >;
    pub struct SigAreaEntered < 'c, C: WithSignals > {
        typed: TypedSigAreaEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAreaEntered < 'c, C > {
        pub fn emit(&mut self, area: Gd < crate::classes::Area2D >,) {
            self.typed.emit_tuple((area,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAreaEntered < 'c, C > {
        type Target = TypedSigAreaEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAreaEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAreaExited < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Area2D >,) >;
    pub struct SigAreaExited < 'c, C: WithSignals > {
        typed: TypedSigAreaExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAreaExited < 'c, C > {
        pub fn emit(&mut self, area: Gd < crate::classes::Area2D >,) {
            self.typed.emit_tuple((area,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAreaExited < 'c, C > {
        type Target = TypedSigAreaExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAreaExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Area2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfArea2D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfArea2D < 'c, C > {
        type Target = < < Area2D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Area2D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfArea2D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Area2D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
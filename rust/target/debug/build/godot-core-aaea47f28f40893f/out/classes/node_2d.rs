#![doc = "Sidecar module for class [`Node2D`][crate::classes::Node2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Node2D` enums](https://docs.godotengine.org/en/stable/classes/class_node2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Node2D`.\n\nInherits [`CanvasItem`][crate::classes::CanvasItem].\n\nRelated symbols:\n\n* [`node_2d`][crate::classes::node_2d]: sidecar module with related enum/flag types\n* [`INode2D`][crate::classes::INode2D]: virtual methods\n\n\nSee also [Godot docs for `Node2D`](https://docs.godotengine.org/en/stable/classes/class_node2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Node2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nA 2D game object, with a transform (position, rotation, and scale). All 2D nodes, including physics objects and sprites, inherit from Node2D. Use Node2D as a parent node to move, scale and rotate children in a 2D project. Also gives control of the node's render order.\n\n**Note:** Since both `Node2D` and [`Control`][crate::classes::Control] inherit from [`CanvasItem`][crate::classes::CanvasItem], they share several concepts from the class such as the \\[member CanvasItem.z_index] and \\[member CanvasItem.visible] properties."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Node2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Node2D`][crate::classes::Node2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Node2D` methods](https://docs.godotengine.org/en/stable/classes/class_node2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INode2D: crate::obj::GodotClass < Base = Node2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Node2D {
        pub fn set_position(&mut self, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "set_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation(&mut self, radians: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "set_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation_degrees(&mut self, degrees: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1726usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "set_rotation_degrees", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_skew(&mut self, radians: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1727usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "set_skew", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scale(&mut self, scale: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1728usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "set_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1729usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "get_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1730usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "get_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation_degrees(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "get_rotation_degrees", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_skew(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "get_skew", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scale(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "get_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a rotation to the node, in radians, starting from its current rotation. This is equivalent to `rotation += radians`."]
        pub fn rotate(&mut self, radians: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1734usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "rotate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a local translation on the node's X axis with the amount specified in `delta`. If `scaled` is `false`, normalizes the movement to occur independently of the node's \\[member scale]."]
        pub(crate) fn move_local_x_full(&mut self, delta: f32, scaled: bool,) {
            type CallRet = ();
            type CallParams = (f32, bool,);
            let args = (delta, scaled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1735usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "move_local_x", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`move_local_x_ex`][Self::move_local_x_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a local translation on the node's X axis with the amount specified in `delta`. If `scaled` is `false`, normalizes the movement to occur independently of the node's \\[member scale]."]
        #[inline]
        pub fn move_local_x(&mut self, delta: f32,) {
            self.move_local_x_ex(delta,) . done()
        }
        #[doc = "Applies a local translation on the node's X axis with the amount specified in `delta`. If `scaled` is `false`, normalizes the movement to occur independently of the node's \\[member scale]."]
        #[inline]
        pub fn move_local_x_ex < 'ex > (&'ex mut self, delta: f32,) -> ExMoveLocalX < 'ex > {
            ExMoveLocalX::new(self, delta,)
        }
        #[doc = "Applies a local translation on the node's Y axis with the amount specified in `delta`. If `scaled` is `false`, normalizes the movement to occur independently of the node's \\[member scale]."]
        pub(crate) fn move_local_y_full(&mut self, delta: f32, scaled: bool,) {
            type CallRet = ();
            type CallParams = (f32, bool,);
            let args = (delta, scaled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1736usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "move_local_y", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`move_local_y_ex`][Self::move_local_y_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a local translation on the node's Y axis with the amount specified in `delta`. If `scaled` is `false`, normalizes the movement to occur independently of the node's \\[member scale]."]
        #[inline]
        pub fn move_local_y(&mut self, delta: f32,) {
            self.move_local_y_ex(delta,) . done()
        }
        #[doc = "Applies a local translation on the node's Y axis with the amount specified in `delta`. If `scaled` is `false`, normalizes the movement to occur independently of the node's \\[member scale]."]
        #[inline]
        pub fn move_local_y_ex < 'ex > (&'ex mut self, delta: f32,) -> ExMoveLocalY < 'ex > {
            ExMoveLocalY::new(self, delta,)
        }
        #[doc = "Translates the node by the given `offset` in local coordinates. This is equivalent to `position += offset`."]
        pub fn translate(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1737usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "translate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds the `offset` vector to the node's global position."]
        pub fn global_translate(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1738usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "global_translate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Multiplies the current scale by the `ratio` vector."]
        pub fn apply_scale(&mut self, ratio: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1739usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "apply_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_position(&mut self, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1740usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "set_global_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "get_global_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_rotation(&mut self, radians: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "set_global_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_rotation_degrees(&mut self, degrees: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1743usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "set_global_rotation_degrees", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_rotation(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1744usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "get_global_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_rotation_degrees(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1745usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "get_global_rotation_degrees", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_skew(&mut self, radians: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1746usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "set_global_skew", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_skew(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1747usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "get_global_skew", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_scale(&mut self, scale: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1748usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "set_global_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_scale(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1749usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "get_global_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transform(&mut self, xform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Transform2D,);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1750usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_transform(&mut self, xform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Transform2D,);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1751usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "set_global_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Rotates the node so that its local +X axis points towards the `point`, which is expected to use global coordinates. This method is a combination of both [`rotate`][`crate::classes::Node2D::rotate`] and [`get_angle_to`][`crate::classes::Node2D::get_angle_to`].\n\n`point` should not be the same as the node's position, otherwise the node always looks to the right."]
        pub fn look_at(&mut self, point: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1752usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "look_at", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the angle between the node and the `point` in radians. See also [`look_at`][`crate::classes::Node2D::look_at`].\n\n[Illustration of the returned angle.](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/node2d_get_angle_to.png)"]
        pub fn get_angle_to(&self, point: Vector2,) -> f32 {
            type CallRet = f32;
            type CallParams = (Vector2,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "get_angle_to", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Transforms the provided global position into a position in local coordinate space. The output will be local relative to the `Node2D` it is called on. e.g. It is appropriate for determining the positions of child nodes, but it is not appropriate for determining its own position relative to its parent."]
        pub fn to_local(&self, global_point: Vector2,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2,);
            let args = (global_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "to_local", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Transforms the provided local position into a position in global coordinate space. The input is expected to be local relative to the `Node2D` it is called on. e.g. Applying this method to the positions of child nodes will correctly transform their positions into the global coordinate space, but applying it to a node's own position will give an incorrect result, as it will incorporate the node's own transformation into its global position."]
        pub fn to_global(&self, local_point: Vector2,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2,);
            let args = (local_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "to_global", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Transform2D`][crate::builtin::Transform2D] relative to this node's parent."]
        pub fn get_relative_transform_to_parent(&self, parent: impl AsArg < Option < Gd < crate::classes::Node >> >,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (parent.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node2D", "get_relative_transform_to_parent", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Node2D {
        type Base = crate::classes::CanvasItem;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Node2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Node2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Node2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Node2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Node2D {
        
    }
    impl crate::obj::cap::GodotDefault for Node2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Node2D {
        type Target = crate::classes::CanvasItem;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Node2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Node2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Node2D__ensure_class_exists {
        ($Class: ident) => {
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
#[doc = "Default-param extender for [`Node2D::move_local_x_ex`][super::Node2D::move_local_x_ex]."]
#[must_use]
pub struct ExMoveLocalX < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Node2D, delta: f32, scaled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExMoveLocalX < 'ex > {
    fn new(surround_object: &'ex mut re_export::Node2D, delta: f32,) -> Self {
        let scaled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, delta: delta, scaled: scaled,
        }
    }
    #[inline]
    pub fn scaled(self, scaled: bool) -> Self {
        Self {
            scaled: scaled, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, delta, scaled,
        }
        = self;
        re_export::Node2D::move_local_x_full(surround_object, delta, scaled,)
    }
}
#[doc = "Default-param extender for [`Node2D::move_local_y_ex`][super::Node2D::move_local_y_ex]."]
#[must_use]
pub struct ExMoveLocalY < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Node2D, delta: f32, scaled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExMoveLocalY < 'ex > {
    fn new(surround_object: &'ex mut re_export::Node2D, delta: f32,) -> Self {
        let scaled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, delta: delta, scaled: scaled,
        }
    }
    #[inline]
    pub fn scaled(self, scaled: bool) -> Self {
        Self {
            scaled: scaled, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, delta, scaled,
        }
        = self;
        re_export::Node2D::move_local_y_full(surround_object, delta, scaled,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Node2D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::canvas_item::SignalsOfCanvasItem;
    impl WithSignals for Node2D {
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
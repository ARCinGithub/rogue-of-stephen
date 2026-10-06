#![doc = "Sidecar module for class [`Node`][crate::classes::Node].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Node` enums](https://docs.godotengine.org/en/stable/classes/class_node.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Node`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`node`][crate::classes::node]: sidecar module with related enum/flag types\n* [`INode`][crate::classes::INode]: virtual methods\n* [`SignalsOfNode`][crate::classes::node::SignalsOfNode]: signal collection\n* [`NodeNotification`][crate::classes::notify::NodeNotification]: notification type\n\n\nSee also [Godot docs for `Node`](https://docs.godotengine.org/en/stable/classes/class_node.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Node::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nNodes are Godot's building blocks. They can be assigned as the child of another node, resulting in a tree arrangement. A given node can contain any number of nodes as children with the requirement that all siblings (direct children of a node) should have unique names.\n\nA tree of nodes is called a _scene_. Scenes can be saved to the disk and then instantiated into other scenes. This allows for very high flexibility in the architecture and data model of Godot projects.\n\n**Scene tree:** The [`SceneTree`][crate::classes::SceneTree] contains the active tree of nodes. When a node is added to the scene tree, it receives the [`NodeNotification::ENTER_TREE`][`crate::classes::notify::NodeNotification::ENTER_TREE`] notification and its [`enter_tree`][`crate::classes::INode::enter_tree`] callback is triggered. Child nodes are always added _after_ their parent node, i.e. the [`enter_tree`][`crate::classes::INode::enter_tree`] callback of a parent node will be triggered before its child's.\n\nOnce all nodes have been added in the scene tree, they receive the [`NodeNotification::READY`][`crate::classes::notify::NodeNotification::READY`] notification and their respective [`ready`][`crate::classes::INode::ready`] callbacks are triggered. For groups of nodes, the [`ready`][`crate::classes::INode::ready`] callback is called in reverse order, starting with the children and moving up to the parent nodes.\n\nThis means that when adding a node to the scene tree, the following order will be used for the callbacks: [`enter_tree`][`crate::classes::INode::enter_tree`] of the parent, [`enter_tree`][`crate::classes::INode::enter_tree`] of the children, [`ready`][`crate::classes::INode::ready`] of the children and finally [`ready`][`crate::classes::INode::ready`] of the parent (recursively for the entire scene tree).\n\n**Processing:** Nodes can override the \"process\" state, so that they receive a callback on each frame requesting them to process (do something). Normal processing (callback [`process`][`crate::classes::INode::process`], toggled with [`set_process`][`crate::classes::Node::set_process`]) happens as fast as possible and is dependent on the frame rate, so the processing time _delta_ (in seconds) is passed as an argument. Physics processing (callback [`physics_process`][`crate::classes::INode::physics_process`], toggled with [`set_physics_process`][`crate::classes::Node::set_physics_process`]) happens a fixed number of times per second (60 by default) and is useful for code related to the physics engine.\n\nNodes can also process input events. When present, the [`input`][`crate::classes::INode::input`] function will be called for each input that the program receives. In many cases, this can be overkill (unless used for simple projects), and the [`unhandled_input`][`crate::classes::INode::unhandled_input`] function might be preferred; it is called when the input event was not handled by anyone else (typically, GUI [`Control`][crate::classes::Control] nodes), ensuring that the node only receives the events that were meant for it.\n\nTo keep track of the scene hierarchy (especially when instantiating scenes into other scenes), an \"owner\" can be set for the node with the \\[member owner] property. This keeps track of who instantiated what. This is mostly useful when writing editors and tools, though.\n\nFinally, when a node is freed with [`free`][`crate::obj::Gd::free`] or [`queue_free`][`crate::classes::Node::queue_free`], it will also free all its children.\n\n**Groups:** Nodes can be added to as many groups as you want to be easy to manage, you could create groups like \"enemies\" or \"collectables\" for example, depending on your game. See [`add_to_group`][`crate::classes::Node::add_to_group`], [`is_in_group`][`crate::classes::Node::is_in_group`] and [`remove_from_group`][`crate::classes::Node::remove_from_group`]. You can then retrieve all nodes in these groups, iterate them and even call methods on groups via the methods on [`SceneTree`][crate::classes::SceneTree].\n\n**Networking with nodes:** After connecting to a server (or making one, see [`ENetMultiplayerPeer`][crate::classes::ENetMultiplayerPeer]), it is possible to use the built-in RPC (remote procedure call) system to communicate over the network. By calling [`rpc`][`crate::classes::Node::rpc`] with a method name, it will be called locally and in all connected peers (peers = clients and the server that accepts connections). To identify which node receives the RPC call, Godot will use its [`NodePath`][crate::builtin::NodePath] (make sure node names are the same on all peers). Also, take a look at the high-level networking tutorial and corresponding demos.\n\n**Note:** The `script` property is part of the [`Object`][crate::classes::Object] class, not `Node`. It isn't exposed like most properties but does have a setter and getter (see [`set_script`][`crate::classes::Object::set_script`] and [`get_script`][`crate::classes::Object::get_script`])."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Node {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Node`][crate::classes::Node].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Node` methods](https://docs.godotengine.org/en/stable/classes/class_node.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INode: crate::obj::GodotClass < Base = Node > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
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
    #[doc = "Notification type for class [`Node`][crate::classes::Node]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum NodeNotification {
        ENTER_TREE = 10i32, EXIT_TREE = 11i32, MOVED_IN_PARENT = 12i32, READY = 13i32, PAUSED = 14i32, UNPAUSED = 15i32, PHYSICS_PROCESS = 16i32, PROCESS = 17i32, PARENTED = 18i32, UNPARENTED = 19i32, SCENE_INSTANTIATED = 20i32, DRAG_BEGIN = 21i32, DRAG_END = 22i32, PATH_RENAMED = 23i32, CHILD_ORDER_CHANGED = 24i32, INTERNAL_PROCESS = 25i32, INTERNAL_PHYSICS_PROCESS = 26i32, POST_ENTER_TREE = 27i32, DISABLED = 28i32, ENABLED = 29i32, RESET_PHYSICS_INTERPOLATION = 2001i32, EDITOR_PRE_SAVE = 9001i32, EDITOR_POST_SAVE = 9002i32, WM_MOUSE_ENTER = 1002i32, WM_MOUSE_EXIT = 1003i32, WM_WINDOW_FOCUS_IN = 1004i32, WM_WINDOW_FOCUS_OUT = 1005i32, WM_CLOSE_REQUEST = 1006i32, WM_GO_BACK_REQUEST = 1007i32, WM_SIZE_CHANGED = 1008i32, WM_DPI_CHANGE = 1009i32, VP_MOUSE_ENTER = 1010i32, VP_MOUSE_EXIT = 1011i32, WM_POSITION_CHANGED = 1012i32, OS_MEMORY_WARNING = 2009i32, TRANSLATION_CHANGED = 2010i32, WM_ABOUT = 2011i32, CRASH = 2012i32, OS_IME_UPDATE = 2013i32, APPLICATION_RESUMED = 2014i32, APPLICATION_PAUSED = 2015i32, APPLICATION_FOCUS_IN = 2016i32, APPLICATION_FOCUS_OUT = 2017i32, TEXT_SERVER_CHANGED = 2018i32, ACCESSIBILITY_UPDATE = 3000i32, ACCESSIBILITY_INVALIDATE = 3001i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for NodeNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                10i32 => Self::ENTER_TREE, 11i32 => Self::EXIT_TREE, 12i32 => Self::MOVED_IN_PARENT, 13i32 => Self::READY, 14i32 => Self::PAUSED, 15i32 => Self::UNPAUSED, 16i32 => Self::PHYSICS_PROCESS, 17i32 => Self::PROCESS, 18i32 => Self::PARENTED, 19i32 => Self::UNPARENTED, 20i32 => Self::SCENE_INSTANTIATED, 21i32 => Self::DRAG_BEGIN, 22i32 => Self::DRAG_END, 23i32 => Self::PATH_RENAMED, 24i32 => Self::CHILD_ORDER_CHANGED, 25i32 => Self::INTERNAL_PROCESS, 26i32 => Self::INTERNAL_PHYSICS_PROCESS, 27i32 => Self::POST_ENTER_TREE, 28i32 => Self::DISABLED, 29i32 => Self::ENABLED, 2001i32 => Self::RESET_PHYSICS_INTERPOLATION, 9001i32 => Self::EDITOR_PRE_SAVE, 9002i32 => Self::EDITOR_POST_SAVE, 1002i32 => Self::WM_MOUSE_ENTER, 1003i32 => Self::WM_MOUSE_EXIT, 1004i32 => Self::WM_WINDOW_FOCUS_IN, 1005i32 => Self::WM_WINDOW_FOCUS_OUT, 1006i32 => Self::WM_CLOSE_REQUEST, 1007i32 => Self::WM_GO_BACK_REQUEST, 1008i32 => Self::WM_SIZE_CHANGED, 1009i32 => Self::WM_DPI_CHANGE, 1010i32 => Self::VP_MOUSE_ENTER, 1011i32 => Self::VP_MOUSE_EXIT, 1012i32 => Self::WM_POSITION_CHANGED, 2009i32 => Self::OS_MEMORY_WARNING, 2010i32 => Self::TRANSLATION_CHANGED, 2011i32 => Self::WM_ABOUT, 2012i32 => Self::CRASH, 2013i32 => Self::OS_IME_UPDATE, 2014i32 => Self::APPLICATION_RESUMED, 2015i32 => Self::APPLICATION_PAUSED, 2016i32 => Self::APPLICATION_FOCUS_IN, 2017i32 => Self::APPLICATION_FOCUS_OUT, 2018i32 => Self::TEXT_SERVER_CHANGED, 3000i32 => Self::ACCESSIBILITY_UPDATE, 3001i32 => Self::ACCESSIBILITY_INVALIDATE, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < NodeNotification > for i32 {
        fn from(notification: NodeNotification) -> i32 {
            match notification {
                NodeNotification::ENTER_TREE => 10i32, NodeNotification::EXIT_TREE => 11i32, NodeNotification::MOVED_IN_PARENT => 12i32, NodeNotification::READY => 13i32, NodeNotification::PAUSED => 14i32, NodeNotification::UNPAUSED => 15i32, NodeNotification::PHYSICS_PROCESS => 16i32, NodeNotification::PROCESS => 17i32, NodeNotification::PARENTED => 18i32, NodeNotification::UNPARENTED => 19i32, NodeNotification::SCENE_INSTANTIATED => 20i32, NodeNotification::DRAG_BEGIN => 21i32, NodeNotification::DRAG_END => 22i32, NodeNotification::PATH_RENAMED => 23i32, NodeNotification::CHILD_ORDER_CHANGED => 24i32, NodeNotification::INTERNAL_PROCESS => 25i32, NodeNotification::INTERNAL_PHYSICS_PROCESS => 26i32, NodeNotification::POST_ENTER_TREE => 27i32, NodeNotification::DISABLED => 28i32, NodeNotification::ENABLED => 29i32, NodeNotification::RESET_PHYSICS_INTERPOLATION => 2001i32, NodeNotification::EDITOR_PRE_SAVE => 9001i32, NodeNotification::EDITOR_POST_SAVE => 9002i32, NodeNotification::WM_MOUSE_ENTER => 1002i32, NodeNotification::WM_MOUSE_EXIT => 1003i32, NodeNotification::WM_WINDOW_FOCUS_IN => 1004i32, NodeNotification::WM_WINDOW_FOCUS_OUT => 1005i32, NodeNotification::WM_CLOSE_REQUEST => 1006i32, NodeNotification::WM_GO_BACK_REQUEST => 1007i32, NodeNotification::WM_SIZE_CHANGED => 1008i32, NodeNotification::WM_DPI_CHANGE => 1009i32, NodeNotification::VP_MOUSE_ENTER => 1010i32, NodeNotification::VP_MOUSE_EXIT => 1011i32, NodeNotification::WM_POSITION_CHANGED => 1012i32, NodeNotification::OS_MEMORY_WARNING => 2009i32, NodeNotification::TRANSLATION_CHANGED => 2010i32, NodeNotification::WM_ABOUT => 2011i32, NodeNotification::CRASH => 2012i32, NodeNotification::OS_IME_UPDATE => 2013i32, NodeNotification::APPLICATION_RESUMED => 2014i32, NodeNotification::APPLICATION_PAUSED => 2015i32, NodeNotification::APPLICATION_FOCUS_IN => 2016i32, NodeNotification::APPLICATION_FOCUS_OUT => 2017i32, NodeNotification::TEXT_SERVER_CHANGED => 2018i32, NodeNotification::ACCESSIBILITY_UPDATE => 3000i32, NodeNotification::ACCESSIBILITY_INVALIDATE => 3001i32, NodeNotification::POSTINITIALIZE => 0i32, NodeNotification::PREDELETE => 1i32, NodeNotification::EXTENSION_RELOADED => 2i32, NodeNotification::Unknown(int) => int,
            }
        }
    }
    impl Node {
        #[doc = "Prints all orphan nodes (nodes outside the [`SceneTree`][crate::classes::SceneTree]). Useful for debugging.\n\n**Note:** This method only works in debug builds. It does nothing in a project exported in release mode."]
        pub fn print_orphan_nodes() {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "print_orphan_nodes", None, args,)
            }
        }
        #[doc = "Returns object IDs of all orphan nodes (nodes outside the [`SceneTree`][crate::classes::SceneTree]). Used for debugging.\n\n**Note:** [`get_orphan_node_ids`][`crate::classes::Node::get_orphan_node_ids`] only works in debug builds. When called in a project exported in release mode, [`get_orphan_node_ids`][`crate::classes::Node::get_orphan_node_ids`] will return an empty array."]
        pub fn get_orphan_node_ids() -> Array < i64 > {
            type CallRet = Array < i64 >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_orphan_node_ids", None, args,)
            }
        }
        #[doc = "Adds a `sibling` node to this node's parent, and moves the added sibling right below this node.\n\nIf `force_readable_name` is `true`, improves the readability of the added `sibling`. If not named, the `sibling` is renamed to its type, and if it shares \\[member name] with a sibling, a number is suffixed more appropriately. This operation is very slow. As such, it is recommended leaving this to `false`, which assigns a dummy name featuring `@` in both situations.\n\nUse [`add_child`][`crate::classes::Node::add_child`] instead of this method if you don't need the child node to be added below a specific node in the list of children.\n\n**Note:** If this node is internal, the added sibling will be internal too (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter)."]
        pub(crate) fn add_sibling_full(&mut self, sibling: CowArg < Gd < crate::classes::Node > >, force_readable_name: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >, bool,);
            let args = (sibling, force_readable_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "add_sibling", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_sibling_ex`][Self::add_sibling_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a `sibling` node to this node's parent, and moves the added sibling right below this node.\n\nIf `force_readable_name` is `true`, improves the readability of the added `sibling`. If not named, the `sibling` is renamed to its type, and if it shares \\[member name] with a sibling, a number is suffixed more appropriately. This operation is very slow. As such, it is recommended leaving this to `false`, which assigns a dummy name featuring `@` in both situations.\n\nUse [`add_child`][`crate::classes::Node::add_child`] instead of this method if you don't need the child node to be added below a specific node in the list of children.\n\n**Note:** If this node is internal, the added sibling will be internal too (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter)."]
        #[inline]
        pub fn add_sibling(&mut self, sibling: impl AsArg < Gd < crate::classes::Node >>,) {
            self.add_sibling_ex(sibling,) . done()
        }
        #[doc = "Adds a `sibling` node to this node's parent, and moves the added sibling right below this node.\n\nIf `force_readable_name` is `true`, improves the readability of the added `sibling`. If not named, the `sibling` is renamed to its type, and if it shares \\[member name] with a sibling, a number is suffixed more appropriately. This operation is very slow. As such, it is recommended leaving this to `false`, which assigns a dummy name featuring `@` in both situations.\n\nUse [`add_child`][`crate::classes::Node::add_child`] instead of this method if you don't need the child node to be added below a specific node in the list of children.\n\n**Note:** If this node is internal, the added sibling will be internal too (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter)."]
        #[inline]
        pub fn add_sibling_ex < 'ex > (&'ex mut self, sibling: impl AsArg < Gd < crate::classes::Node >> + 'ex,) -> ExAddSibling < 'ex > {
            ExAddSibling::new(self, sibling,)
        }
        pub fn set_name(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_name(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a child `node`. Nodes can have any number of children, but every child must have a unique name. Child nodes are automatically deleted when the parent node is deleted, so an entire scene can be removed by deleting its topmost node.\n\nIf `force_readable_name` is `true`, improves the readability of the added `node`. If not named, the `node` is renamed to its type, and if it shares \\[member name] with a sibling, a number is suffixed more appropriately. This operation is very slow. As such, it is recommended leaving this to `false`, which assigns a dummy name featuring `@` in both situations.\n\nIf `internal` is different than [`InternalMode::DISABLED`][`crate::classes::node::InternalMode::DISABLED`], the child will be added as internal node. These nodes are ignored by methods like [`get_children`][`crate::classes::Node::get_children`], unless their parameter `include_internal` is `true`. It also prevents these nodes being duplicated with their parent. The intended usage is to hide the internal nodes from the user, so the user won't accidentally delete or modify them. Used by some GUI nodes, e.g. [`ColorPicker`][crate::classes::ColorPicker].\n\n**Note:** If `node` already has a parent, this method will fail. Use [`remove_child`][`crate::classes::Node::remove_child`] first to remove `node` from its current parent. For example:\n\n\n```gdscript\nvar child_node = get_child(0)\nif child_node.get_parent():\n\tchild_node.get_parent().remove_child(child_node)\nadd_child(child_node)\n```\n\n\nIf you need the child node to be added below a specific node in the list of children, use [`add_sibling`][`crate::classes::Node::add_sibling`] instead of this method.\n\n**Note:** If you want a child to be persisted to a [`PackedScene`][crate::classes::PackedScene], you must set \\[member owner] in addition to calling [`add_child`][`crate::classes::Node::add_child`]. This is typically relevant for [tool scripts]($DOCS_URL/tutorials/plugins/running_code_in_the_editor.html) and [editor plugins]($DOCS_URL/tutorials/plugins/editor/index.html). If [`add_child`][`crate::classes::Node::add_child`] is called without setting \\[member owner], the newly added `Node` will not be visible in the scene tree, though it will be visible in the 2D/3D view."]
        pub(crate) fn add_child_full(&mut self, node: CowArg < Gd < crate::classes::Node > >, force_readable_name: bool, internal: crate::classes::node::InternalMode,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >, bool, crate::classes::node::InternalMode,);
            let args = (node, force_readable_name, internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "add_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_child_ex`][Self::add_child_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a child `node`. Nodes can have any number of children, but every child must have a unique name. Child nodes are automatically deleted when the parent node is deleted, so an entire scene can be removed by deleting its topmost node.\n\nIf `force_readable_name` is `true`, improves the readability of the added `node`. If not named, the `node` is renamed to its type, and if it shares \\[member name] with a sibling, a number is suffixed more appropriately. This operation is very slow. As such, it is recommended leaving this to `false`, which assigns a dummy name featuring `@` in both situations.\n\nIf `internal` is different than [`InternalMode::DISABLED`][`crate::classes::node::InternalMode::DISABLED`], the child will be added as internal node. These nodes are ignored by methods like [`get_children`][`crate::classes::Node::get_children`], unless their parameter `include_internal` is `true`. It also prevents these nodes being duplicated with their parent. The intended usage is to hide the internal nodes from the user, so the user won't accidentally delete or modify them. Used by some GUI nodes, e.g. [`ColorPicker`][crate::classes::ColorPicker].\n\n**Note:** If `node` already has a parent, this method will fail. Use [`remove_child`][`crate::classes::Node::remove_child`] first to remove `node` from its current parent. For example:\n\n\n```gdscript\nvar child_node = get_child(0)\nif child_node.get_parent():\n\tchild_node.get_parent().remove_child(child_node)\nadd_child(child_node)\n```\n\n\nIf you need the child node to be added below a specific node in the list of children, use [`add_sibling`][`crate::classes::Node::add_sibling`] instead of this method.\n\n**Note:** If you want a child to be persisted to a [`PackedScene`][crate::classes::PackedScene], you must set \\[member owner] in addition to calling [`add_child`][`crate::classes::Node::add_child`]. This is typically relevant for [tool scripts]($DOCS_URL/tutorials/plugins/running_code_in_the_editor.html) and [editor plugins]($DOCS_URL/tutorials/plugins/editor/index.html). If [`add_child`][`crate::classes::Node::add_child`] is called without setting \\[member owner], the newly added `Node` will not be visible in the scene tree, though it will be visible in the 2D/3D view."]
        #[inline]
        pub fn add_child(&mut self, node: impl AsArg < Gd < crate::classes::Node >>,) {
            self.add_child_ex(node,) . done()
        }
        #[doc = "Adds a child `node`. Nodes can have any number of children, but every child must have a unique name. Child nodes are automatically deleted when the parent node is deleted, so an entire scene can be removed by deleting its topmost node.\n\nIf `force_readable_name` is `true`, improves the readability of the added `node`. If not named, the `node` is renamed to its type, and if it shares \\[member name] with a sibling, a number is suffixed more appropriately. This operation is very slow. As such, it is recommended leaving this to `false`, which assigns a dummy name featuring `@` in both situations.\n\nIf `internal` is different than [`InternalMode::DISABLED`][`crate::classes::node::InternalMode::DISABLED`], the child will be added as internal node. These nodes are ignored by methods like [`get_children`][`crate::classes::Node::get_children`], unless their parameter `include_internal` is `true`. It also prevents these nodes being duplicated with their parent. The intended usage is to hide the internal nodes from the user, so the user won't accidentally delete or modify them. Used by some GUI nodes, e.g. [`ColorPicker`][crate::classes::ColorPicker].\n\n**Note:** If `node` already has a parent, this method will fail. Use [`remove_child`][`crate::classes::Node::remove_child`] first to remove `node` from its current parent. For example:\n\n\n```gdscript\nvar child_node = get_child(0)\nif child_node.get_parent():\n\tchild_node.get_parent().remove_child(child_node)\nadd_child(child_node)\n```\n\n\nIf you need the child node to be added below a specific node in the list of children, use [`add_sibling`][`crate::classes::Node::add_sibling`] instead of this method.\n\n**Note:** If you want a child to be persisted to a [`PackedScene`][crate::classes::PackedScene], you must set \\[member owner] in addition to calling [`add_child`][`crate::classes::Node::add_child`]. This is typically relevant for [tool scripts]($DOCS_URL/tutorials/plugins/running_code_in_the_editor.html) and [editor plugins]($DOCS_URL/tutorials/plugins/editor/index.html). If [`add_child`][`crate::classes::Node::add_child`] is called without setting \\[member owner], the newly added `Node` will not be visible in the scene tree, though it will be visible in the 2D/3D view."]
        #[inline]
        pub fn add_child_ex < 'ex > (&'ex mut self, node: impl AsArg < Gd < crate::classes::Node >> + 'ex,) -> ExAddChild < 'ex > {
            ExAddChild::new(self, node,)
        }
        #[doc = "Removes a child `node`. The `node`, along with its children, are **not** deleted. To delete a node, see [`queue_free`][`crate::classes::Node::queue_free`].\n\n**Note:** When this node is inside the tree, this method sets the \\[member owner] of the removed `node` (or its descendants) to `null`, if their \\[member owner] is no longer an ancestor (see [`is_ancestor_of`][`crate::classes::Node::is_ancestor_of`])."]
        pub fn remove_child(&mut self, node: impl AsArg < Gd < crate::classes::Node >>,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "remove_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Changes the parent of this `Node` to the `new_parent`. The node needs to already have a parent. The node's \\[member owner] is preserved if its owner is still reachable from the new location (i.e., the node is still a descendant of the new parent after the operation).\n\nIf `keep_global_transform` is `true`, the node's global transform will be preserved if supported. [`Node2D`][crate::classes::Node2D], [`Node3D`][crate::classes::Node3D] and [`Control`][crate::classes::Control] support this argument (but [`Control`][crate::classes::Control] keeps only position)."]
        pub(crate) fn reparent_full(&mut self, new_parent: CowArg < Gd < crate::classes::Node > >, keep_global_transform: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >, bool,);
            let args = (new_parent, keep_global_transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "reparent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`reparent_ex`][Self::reparent_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Changes the parent of this `Node` to the `new_parent`. The node needs to already have a parent. The node's \\[member owner] is preserved if its owner is still reachable from the new location (i.e., the node is still a descendant of the new parent after the operation).\n\nIf `keep_global_transform` is `true`, the node's global transform will be preserved if supported. [`Node2D`][crate::classes::Node2D], [`Node3D`][crate::classes::Node3D] and [`Control`][crate::classes::Control] support this argument (but [`Control`][crate::classes::Control] keeps only position)."]
        #[inline]
        pub fn reparent(&mut self, new_parent: impl AsArg < Gd < crate::classes::Node >>,) {
            self.reparent_ex(new_parent,) . done()
        }
        #[doc = "Changes the parent of this `Node` to the `new_parent`. The node needs to already have a parent. The node's \\[member owner] is preserved if its owner is still reachable from the new location (i.e., the node is still a descendant of the new parent after the operation).\n\nIf `keep_global_transform` is `true`, the node's global transform will be preserved if supported. [`Node2D`][crate::classes::Node2D], [`Node3D`][crate::classes::Node3D] and [`Control`][crate::classes::Control] support this argument (but [`Control`][crate::classes::Control] keeps only position)."]
        #[inline]
        pub fn reparent_ex < 'ex > (&'ex mut self, new_parent: impl AsArg < Gd < crate::classes::Node >> + 'ex,) -> ExReparent < 'ex > {
            ExReparent::new(self, new_parent,)
        }
        #[doc = "Returns the number of children of this node.\n\nIf `include_internal` is `false`, internal children are not counted (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter)."]
        pub(crate) fn get_child_count_full(&self, include_internal: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (bool,);
            let args = (include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_child_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_child_count_ex`][Self::get_child_count_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the number of children of this node.\n\nIf `include_internal` is `false`, internal children are not counted (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter)."]
        #[inline]
        pub fn get_child_count(&self,) -> i32 {
            self.get_child_count_ex() . done()
        }
        #[doc = "Returns the number of children of this node.\n\nIf `include_internal` is `false`, internal children are not counted (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter)."]
        #[inline]
        pub fn get_child_count_ex < 'ex > (&'ex self,) -> ExGetChildCount < 'ex > {
            ExGetChildCount::new(self,)
        }
        #[doc = "Returns all children of this node inside an [`Array`][crate::builtin::Array].\n\nIf `include_internal` is `false`, excludes internal children from the returned array (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter)."]
        pub(crate) fn get_children_full(&self, include_internal: bool,) -> Array < Gd < crate::classes::Node > > {
            type CallRet = Array < Gd < crate::classes::Node > >;
            type CallParams = (bool,);
            let args = (include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_children", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_children_ex`][Self::get_children_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns all children of this node inside an [`Array`][crate::builtin::Array].\n\nIf `include_internal` is `false`, excludes internal children from the returned array (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter)."]
        #[inline]
        pub fn get_children(&self,) -> Array < Gd < crate::classes::Node > > {
            self.get_children_ex() . done()
        }
        #[doc = "Returns all children of this node inside an [`Array`][crate::builtin::Array].\n\nIf `include_internal` is `false`, excludes internal children from the returned array (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter)."]
        #[inline]
        pub fn get_children_ex < 'ex > (&'ex self,) -> ExGetChildren < 'ex > {
            ExGetChildren::new(self,)
        }
        #[doc = "Fetches a child node by its index. Each child node has an index relative to its siblings (see [`get_index`][`crate::classes::Node::get_index`]). The first child is at index 0. Negative values can also be used to start from the end of the list. This method can be used in combination with [`get_child_count`][`crate::classes::Node::get_child_count`] to iterate over this node's children. If no child exists at the given index, this method returns `null` and an error is generated.\n\nIf `include_internal` is `false`, internal children are ignored (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter).\n\n```gdscript\n# Assuming the following are children of this node, in order:\n# First, Middle, Last.\n\nvar a = get_child(0).name  # a is \"First\"\nvar b = get_child(1).name  # b is \"Middle\"\nvar b = get_child(2).name  # b is \"Last\"\nvar c = get_child(-1).name # c is \"Last\"\n```\n\n**Note:** To fetch a node by [`NodePath`][crate::builtin::NodePath], use [`get_node_as`][`crate::classes::Node::get_node_as`]."]
        pub(crate) fn get_child_full(&self, idx: i32, include_internal: bool,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = (i32, bool,);
            let args = (idx, include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_child_ex`][Self::get_child_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Fetches a child node by its index. Each child node has an index relative to its siblings (see [`get_index`][`crate::classes::Node::get_index`]). The first child is at index 0. Negative values can also be used to start from the end of the list. This method can be used in combination with [`get_child_count`][`crate::classes::Node::get_child_count`] to iterate over this node's children. If no child exists at the given index, this method returns `null` and an error is generated.\n\nIf `include_internal` is `false`, internal children are ignored (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter).\n\n```gdscript\n# Assuming the following are children of this node, in order:\n# First, Middle, Last.\n\nvar a = get_child(0).name  # a is \"First\"\nvar b = get_child(1).name  # b is \"Middle\"\nvar b = get_child(2).name  # b is \"Last\"\nvar c = get_child(-1).name # c is \"Last\"\n```\n\n**Note:** To fetch a node by [`NodePath`][crate::builtin::NodePath], use [`get_node_as`][`crate::classes::Node::get_node_as`]."]
        #[inline]
        pub fn get_child(&self, idx: i32,) -> Option < Gd < crate::classes::Node > > {
            self.get_child_ex(idx,) . done()
        }
        #[doc = "Fetches a child node by its index. Each child node has an index relative to its siblings (see [`get_index`][`crate::classes::Node::get_index`]). The first child is at index 0. Negative values can also be used to start from the end of the list. This method can be used in combination with [`get_child_count`][`crate::classes::Node::get_child_count`] to iterate over this node's children. If no child exists at the given index, this method returns `null` and an error is generated.\n\nIf `include_internal` is `false`, internal children are ignored (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter).\n\n```gdscript\n# Assuming the following are children of this node, in order:\n# First, Middle, Last.\n\nvar a = get_child(0).name  # a is \"First\"\nvar b = get_child(1).name  # b is \"Middle\"\nvar b = get_child(2).name  # b is \"Last\"\nvar c = get_child(-1).name # c is \"Last\"\n```\n\n**Note:** To fetch a node by [`NodePath`][crate::builtin::NodePath], use [`get_node_as`][`crate::classes::Node::get_node_as`]."]
        #[inline]
        pub fn get_child_ex < 'ex > (&'ex self, idx: i32,) -> ExGetChild < 'ex > {
            ExGetChild::new(self, idx,)
        }
        #[doc = "Returns `true` if the `path` points to a valid node. See also [`get_node_as`][`crate::classes::Node::get_node_as`]."]
        pub fn has_node(&self, path: impl AsArg < NodePath >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "has_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Fetches a node by [`NodePath`][crate::builtin::NodePath]. Similar to [`get_node_as`][`crate::classes::Node::get_node_as`], but does not generate an error if `path` does not point to a valid node."]
        pub fn get_node_or_null(&self, path: impl AsArg < NodePath >,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_node_or_null", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns this node's parent node, or `null` if the node doesn't have a parent."]
        pub fn get_parent(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_parent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Finds the first descendant of this node whose \\[member name] matches `pattern`, returning `null` if no match is found. The matching is done against node names, _not_ their paths, through [`match_glob`][`crate::builtin::GString::match_glob`]. As such, it is case-sensitive, `\"*\"` matches zero or more characters, and `\"?\"` matches any single character.\n\nIf `recursive` is `false`, only this node's direct children are checked. Nodes are checked in tree order, so this node's first direct child is checked first, then its own direct children, etc., before moving to the second direct child, and so on. Internal children are also included in the search (see `internal` parameter in [`add_child`][`crate::classes::Node::add_child`]).\n\nIf `owned` is `true`, only descendants with a valid \\[member owner] node are checked.\n\n**Note:** This method can be very slow. Consider storing a reference to the found node in a variable. Alternatively, use [`get_node_as`][`crate::classes::Node::get_node_as`] with unique names (see \\[member unique_name_in_owner]).\n\n**Note:** To find all descendant nodes matching a pattern or a class type, see [`find_children`][`crate::classes::Node::find_children`]."]
        pub(crate) fn find_child_full(&self, pattern: CowArg < GString >, recursive: bool, owned: bool,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool, bool,);
            let args = (pattern, recursive, owned,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "find_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`find_child_ex`][Self::find_child_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Finds the first descendant of this node whose \\[member name] matches `pattern`, returning `null` if no match is found. The matching is done against node names, _not_ their paths, through [`match_glob`][`crate::builtin::GString::match_glob`]. As such, it is case-sensitive, `\"*\"` matches zero or more characters, and `\"?\"` matches any single character.\n\nIf `recursive` is `false`, only this node's direct children are checked. Nodes are checked in tree order, so this node's first direct child is checked first, then its own direct children, etc., before moving to the second direct child, and so on. Internal children are also included in the search (see `internal` parameter in [`add_child`][`crate::classes::Node::add_child`]).\n\nIf `owned` is `true`, only descendants with a valid \\[member owner] node are checked.\n\n**Note:** This method can be very slow. Consider storing a reference to the found node in a variable. Alternatively, use [`get_node_as`][`crate::classes::Node::get_node_as`] with unique names (see \\[member unique_name_in_owner]).\n\n**Note:** To find all descendant nodes matching a pattern or a class type, see [`find_children`][`crate::classes::Node::find_children`]."]
        #[inline]
        pub fn find_child(&self, pattern: impl AsArg < GString >,) -> Option < Gd < crate::classes::Node > > {
            self.find_child_ex(pattern,) . done()
        }
        #[doc = "Finds the first descendant of this node whose \\[member name] matches `pattern`, returning `null` if no match is found. The matching is done against node names, _not_ their paths, through [`match_glob`][`crate::builtin::GString::match_glob`]. As such, it is case-sensitive, `\"*\"` matches zero or more characters, and `\"?\"` matches any single character.\n\nIf `recursive` is `false`, only this node's direct children are checked. Nodes are checked in tree order, so this node's first direct child is checked first, then its own direct children, etc., before moving to the second direct child, and so on. Internal children are also included in the search (see `internal` parameter in [`add_child`][`crate::classes::Node::add_child`]).\n\nIf `owned` is `true`, only descendants with a valid \\[member owner] node are checked.\n\n**Note:** This method can be very slow. Consider storing a reference to the found node in a variable. Alternatively, use [`get_node_as`][`crate::classes::Node::get_node_as`] with unique names (see \\[member unique_name_in_owner]).\n\n**Note:** To find all descendant nodes matching a pattern or a class type, see [`find_children`][`crate::classes::Node::find_children`]."]
        #[inline]
        pub fn find_child_ex < 'ex > (&'ex self, pattern: impl AsArg < GString > + 'ex,) -> ExFindChild < 'ex > {
            ExFindChild::new(self, pattern,)
        }
        #[doc = "Finds all descendants of this node whose names match `pattern`, returning an empty [`Array`][crate::builtin::Array] if no match is found. The matching is done against node names, _not_ their paths, through [`match_glob`][`crate::builtin::GString::match_glob`]. As such, it is case-sensitive, `\"*\"` matches zero or more characters, and `\"?\"` matches any single character.\n\nIf `type` is not empty, only ancestors inheriting from `type` are included (see [`is_class`][`crate::classes::Object::is_class`]).\n\nIf `recursive` is `false`, only this node's direct children are checked. Nodes are checked in tree order, so this node's first direct child is checked first, then its own direct children, etc., before moving to the second direct child, and so on. Internal children are also included in the search (see `internal` parameter in [`add_child`][`crate::classes::Node::add_child`]).\n\nIf `owned` is `true`, only descendants with a valid \\[member owner] node are checked.\n\n**Note:** This method can be very slow. Consider storing references to the found nodes in a variable.\n\n**Note:** To find a single descendant node matching a pattern, see [`find_child`][`crate::classes::Node::find_child`]."]
        pub(crate) fn find_children_full(&self, pattern: CowArg < GString >, type_: CowArg < GString >, recursive: bool, owned: bool,) -> Array < Gd < crate::classes::Node > > {
            type CallRet = Array < Gd < crate::classes::Node > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool, bool,);
            let args = (pattern, type_, recursive, owned,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "find_children", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`find_children_ex`][Self::find_children_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Finds all descendants of this node whose names match `pattern`, returning an empty [`Array`][crate::builtin::Array] if no match is found. The matching is done against node names, _not_ their paths, through [`match_glob`][`crate::builtin::GString::match_glob`]. As such, it is case-sensitive, `\"*\"` matches zero or more characters, and `\"?\"` matches any single character.\n\nIf `type` is not empty, only ancestors inheriting from `type` are included (see [`is_class`][`crate::classes::Object::is_class`]).\n\nIf `recursive` is `false`, only this node's direct children are checked. Nodes are checked in tree order, so this node's first direct child is checked first, then its own direct children, etc., before moving to the second direct child, and so on. Internal children are also included in the search (see `internal` parameter in [`add_child`][`crate::classes::Node::add_child`]).\n\nIf `owned` is `true`, only descendants with a valid \\[member owner] node are checked.\n\n**Note:** This method can be very slow. Consider storing references to the found nodes in a variable.\n\n**Note:** To find a single descendant node matching a pattern, see [`find_child`][`crate::classes::Node::find_child`]."]
        #[inline]
        pub fn find_children(&self, pattern: impl AsArg < GString >,) -> Array < Gd < crate::classes::Node > > {
            self.find_children_ex(pattern,) . done()
        }
        #[doc = "Finds all descendants of this node whose names match `pattern`, returning an empty [`Array`][crate::builtin::Array] if no match is found. The matching is done against node names, _not_ their paths, through [`match_glob`][`crate::builtin::GString::match_glob`]. As such, it is case-sensitive, `\"*\"` matches zero or more characters, and `\"?\"` matches any single character.\n\nIf `type` is not empty, only ancestors inheriting from `type` are included (see [`is_class`][`crate::classes::Object::is_class`]).\n\nIf `recursive` is `false`, only this node's direct children are checked. Nodes are checked in tree order, so this node's first direct child is checked first, then its own direct children, etc., before moving to the second direct child, and so on. Internal children are also included in the search (see `internal` parameter in [`add_child`][`crate::classes::Node::add_child`]).\n\nIf `owned` is `true`, only descendants with a valid \\[member owner] node are checked.\n\n**Note:** This method can be very slow. Consider storing references to the found nodes in a variable.\n\n**Note:** To find a single descendant node matching a pattern, see [`find_child`][`crate::classes::Node::find_child`]."]
        #[inline]
        pub fn find_children_ex < 'ex > (&'ex self, pattern: impl AsArg < GString > + 'ex,) -> ExFindChildren < 'ex > {
            ExFindChildren::new(self, pattern,)
        }
        #[doc = "Finds the first ancestor of this node whose \\[member name] matches `pattern`, returning `null` if no match is found. The matching is done through [`match_glob`][`crate::builtin::GString::match_glob`]. As such, it is case-sensitive, `\"*\"` matches zero or more characters, and `\"?\"` matches any single character. See also [`find_child`][`crate::classes::Node::find_child`] and [`find_children`][`crate::classes::Node::find_children`].\n\n**Note:** As this method walks upwards in the scene tree, it can be slow in large, deeply nested nodes. Consider storing a reference to the found node in a variable. Alternatively, use [`get_node_as`][`crate::classes::Node::get_node_as`] with unique names (see \\[member unique_name_in_owner])."]
        pub fn find_parent(&self, pattern: impl AsArg < GString >,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (pattern.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "find_parent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if `path` points to a valid node and its subnames point to a valid [`Resource`][crate::classes::Resource], e.g. `Area2D/CollisionShape2D:shape`. Properties that are not [`Resource`][crate::classes::Resource] types (such as nodes or other [`Variant`][crate::builtin::Variant] types) are not considered. See also [`get_node_and_resource`][`crate::classes::Node::get_node_and_resource`]."]
        pub fn has_node_and_resource(&self, path: impl AsArg < NodePath >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "has_node_and_resource", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Fetches a node and its most nested resource as specified by the [`NodePath`][crate::builtin::NodePath]'s subname. Returns an [`Array`][crate::builtin::Array] of size `3` where:\n\n- Element `0` is the `Node`, or `null` if not found;\n\n- Element `1` is the subname's last nested [`Resource`][crate::classes::Resource], or `null` if not found;\n\n- Element `2` is the remaining [`NodePath`][crate::builtin::NodePath], referring to an existing, non-[`Resource`][crate::classes::Resource] property (see [`get_indexed`][`crate::classes::Object::get_indexed`]).\n\n**Example:** Assume that the child's \\[member Sprite2D.texture] has been assigned an [`AtlasTexture`][crate::classes::AtlasTexture]:\n\n\n```gdscript\nvar a = get_node_and_resource(\"Area2D/Sprite2D\")\nprint(a[0].name) # Prints Sprite2D\nprint(a[1])      # Prints <null>\nprint(a[2])      # Prints ^\"\"\n\nvar b = get_node_and_resource(\"Area2D/Sprite2D:texture:atlas\")\nprint(b[0].name)        # Prints Sprite2D\nprint(b[1].get_class()) # Prints AtlasTexture\nprint(b[2])             # Prints ^\"\"\n\nvar c = get_node_and_resource(\"Area2D/Sprite2D:texture:atlas:region\")\nprint(c[0].name)        # Prints Sprite2D\nprint(c[1].get_class()) # Prints AtlasTexture\nprint(c[2])             # Prints ^\":region\"\n```\n"]
        pub fn get_node_and_resource(&self, path: impl AsArg < NodePath >,) -> VarArray {
            type CallRet = VarArray;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_node_and_resource", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this node is currently inside a [`SceneTree`][crate::classes::SceneTree]. See also [`get_tree`][`crate::classes::Node::get_tree`]."]
        pub fn is_inside_tree(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_inside_tree", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the node is part of the scene currently opened in the editor."]
        pub fn is_part_of_edited_scene(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_part_of_edited_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `node` is a direct or indirect child of this node."]
        pub fn is_ancestor_of(&self, node: impl AsArg < Gd < crate::classes::Node >>,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_ancestor_of", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `node` occurs later in the scene hierarchy than this node. A node occurring later is usually processed last."]
        pub fn is_greater_than(&self, node: impl AsArg < Gd < crate::classes::Node >>,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_greater_than", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the node's absolute path, relative to the \\[member SceneTree.root]. If the node is not inside the scene tree, this method fails and returns an empty [`NodePath`][crate::builtin::NodePath]."]
        pub fn get_path(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the relative [`NodePath`][crate::builtin::NodePath] from this node to the specified `node`. Both nodes must be in the same [`SceneTree`][crate::classes::SceneTree] or scene hierarchy, otherwise this method fails and returns an empty [`NodePath`][crate::builtin::NodePath].\n\nIf `use_unique_path` is `true`, returns the shortest path accounting for this node's unique name (see \\[member unique_name_in_owner]).\n\n**Note:** If you get a relative path which starts from a unique node, the path may be longer than a normal relative path, due to the addition of the unique node's name."]
        pub(crate) fn get_path_to_full(&self, node: CowArg < Gd < crate::classes::Node > >, use_unique_path: bool,) -> NodePath {
            type CallRet = NodePath;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >, bool,);
            let args = (node, use_unique_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_path_to", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_path_to_ex`][Self::get_path_to_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the relative [`NodePath`][crate::builtin::NodePath] from this node to the specified `node`. Both nodes must be in the same [`SceneTree`][crate::classes::SceneTree] or scene hierarchy, otherwise this method fails and returns an empty [`NodePath`][crate::builtin::NodePath].\n\nIf `use_unique_path` is `true`, returns the shortest path accounting for this node's unique name (see \\[member unique_name_in_owner]).\n\n**Note:** If you get a relative path which starts from a unique node, the path may be longer than a normal relative path, due to the addition of the unique node's name."]
        #[inline]
        pub fn get_path_to(&self, node: impl AsArg < Gd < crate::classes::Node >>,) -> NodePath {
            self.get_path_to_ex(node,) . done()
        }
        #[doc = "Returns the relative [`NodePath`][crate::builtin::NodePath] from this node to the specified `node`. Both nodes must be in the same [`SceneTree`][crate::classes::SceneTree] or scene hierarchy, otherwise this method fails and returns an empty [`NodePath`][crate::builtin::NodePath].\n\nIf `use_unique_path` is `true`, returns the shortest path accounting for this node's unique name (see \\[member unique_name_in_owner]).\n\n**Note:** If you get a relative path which starts from a unique node, the path may be longer than a normal relative path, due to the addition of the unique node's name."]
        #[inline]
        pub fn get_path_to_ex < 'ex > (&'ex self, node: impl AsArg < Gd < crate::classes::Node >> + 'ex,) -> ExGetPathTo < 'ex > {
            ExGetPathTo::new(self, node,)
        }
        #[doc = "Adds the node to the `group`. Groups can be helpful to organize a subset of nodes, for example `\"enemies\"` or `\"collectables\"`. See notes in the description, and the group methods in [`SceneTree`][crate::classes::SceneTree].\n\nIf `persistent` is `true`, the group will be stored when saved inside a [`PackedScene`][crate::classes::PackedScene]. All groups created and displayed in the Groups dock are persistent.\n\n**Note:** To improve performance, the order of group names is _not_ guaranteed and may vary between project runs. Therefore, do not rely on the group order.\n\n**Note:** [`SceneTree`][crate::classes::SceneTree]'s group methods will _not_ work on this node if not inside the tree (see [`is_inside_tree`][`crate::classes::Node::is_inside_tree`])."]
        pub(crate) fn add_to_group_full(&mut self, group: CowArg < StringName >, persistent: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (group, persistent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "add_to_group", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_to_group_ex`][Self::add_to_group_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds the node to the `group`. Groups can be helpful to organize a subset of nodes, for example `\"enemies\"` or `\"collectables\"`. See notes in the description, and the group methods in [`SceneTree`][crate::classes::SceneTree].\n\nIf `persistent` is `true`, the group will be stored when saved inside a [`PackedScene`][crate::classes::PackedScene]. All groups created and displayed in the Groups dock are persistent.\n\n**Note:** To improve performance, the order of group names is _not_ guaranteed and may vary between project runs. Therefore, do not rely on the group order.\n\n**Note:** [`SceneTree`][crate::classes::SceneTree]'s group methods will _not_ work on this node if not inside the tree (see [`is_inside_tree`][`crate::classes::Node::is_inside_tree`])."]
        #[inline]
        pub fn add_to_group(&mut self, group: impl AsArg < StringName >,) {
            self.add_to_group_ex(group,) . done()
        }
        #[doc = "Adds the node to the `group`. Groups can be helpful to organize a subset of nodes, for example `\"enemies\"` or `\"collectables\"`. See notes in the description, and the group methods in [`SceneTree`][crate::classes::SceneTree].\n\nIf `persistent` is `true`, the group will be stored when saved inside a [`PackedScene`][crate::classes::PackedScene]. All groups created and displayed in the Groups dock are persistent.\n\n**Note:** To improve performance, the order of group names is _not_ guaranteed and may vary between project runs. Therefore, do not rely on the group order.\n\n**Note:** [`SceneTree`][crate::classes::SceneTree]'s group methods will _not_ work on this node if not inside the tree (see [`is_inside_tree`][`crate::classes::Node::is_inside_tree`])."]
        #[inline]
        pub fn add_to_group_ex < 'ex > (&'ex mut self, group: impl AsArg < StringName > + 'ex,) -> ExAddToGroup < 'ex > {
            ExAddToGroup::new(self, group,)
        }
        #[doc = "Removes the node from the given `group`. Does nothing if the node is not in the `group`. See also notes in the description, and the [`SceneTree`][crate::classes::SceneTree]'s group methods."]
        pub fn remove_from_group(&mut self, group: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "remove_from_group", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this node has been added to the given `group`. See [`add_to_group`][`crate::classes::Node::add_to_group`] and [`remove_from_group`][`crate::classes::Node::remove_from_group`]. See also notes in the description, and the [`SceneTree`][crate::classes::SceneTree]'s group methods."]
        pub fn is_in_group(&self, group: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_in_group", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves `child_node` to the given index. A node's index is the order among its siblings. If `to_index` is negative, the index is counted from the end of the list. See also [`get_child`][`crate::classes::Node::get_child`] and [`get_index`][`crate::classes::Node::get_index`].\n\n**Note:** The processing order of several engine callbacks ([`ready`][`crate::classes::INode::ready`], [`process`][`crate::classes::INode::process`], etc.) and notifications sent through [`propagate_notification`][`crate::classes::Node::propagate_notification`] is affected by tree order. [`CanvasItem`][crate::classes::CanvasItem] nodes are also rendered in tree order. See also \\[member process_priority]."]
        pub fn move_child(&mut self, child_node: impl AsArg < Gd < crate::classes::Node >>, to_index: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >, i32,);
            let args = (child_node.into_arg(), to_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "move_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of group names that the node has been added to.\n\n**Note:** To improve performance, the order of group names is _not_ guaranteed and may vary between project runs. Therefore, do not rely on the group order.\n\n**Note:** This method may also return some group names starting with an underscore (`_`). These are internally used by the engine. To avoid conflicts, do not use custom groups starting with underscores. To exclude internal groups, see the following code snippet:\n\n\n```gdscript\n# Stores the node's non-internal groups only (as an array of StringNames).\nvar non_internal_groups = []\nfor group in get_groups():\n\tif not str(group).begins_with(\"_\"):\n\t\tnon_internal_groups.push_back(group)\n```\n"]
        pub fn get_groups(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_groups", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_owner(&mut self, owner: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (owner.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_owner", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_owner(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_owner", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns this node's order among its siblings. The first node's index is `0`. See also [`get_child`][`crate::classes::Node::get_child`].\n\nIf `include_internal` is `false`, returns the index ignoring internal children. The first, non-internal child will have an index of `0` (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter)."]
        pub(crate) fn get_index_full(&self, include_internal: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (bool,);
            let args = (include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_index_ex`][Self::get_index_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns this node's order among its siblings. The first node's index is `0`. See also [`get_child`][`crate::classes::Node::get_child`].\n\nIf `include_internal` is `false`, returns the index ignoring internal children. The first, non-internal child will have an index of `0` (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter)."]
        #[inline]
        pub fn get_index(&self,) -> i32 {
            self.get_index_ex() . done()
        }
        #[doc = "Returns this node's order among its siblings. The first node's index is `0`. See also [`get_child`][`crate::classes::Node::get_child`].\n\nIf `include_internal` is `false`, returns the index ignoring internal children. The first, non-internal child will have an index of `0` (see [`add_child`][`crate::classes::Node::add_child`]'s `internal` parameter)."]
        #[inline]
        pub fn get_index_ex < 'ex > (&'ex self,) -> ExGetIndex < 'ex > {
            ExGetIndex::new(self,)
        }
        #[doc = "Prints the node and its children to the console, recursively. The node does not have to be inside the tree. This method outputs [`NodePath`][crate::builtin::NodePath]s relative to this node, and is good for copy/pasting into [`get_node_as`][`crate::classes::Node::get_node_as`]. See also [`print_tree_pretty`][`crate::classes::Node::print_tree_pretty`].\n\nMay print, for example:\n\n```text\n.\nMenu\nMenu/Label\nMenu/Camera2D\nSplashScreen\nSplashScreen/Camera2D\n```"]
        pub fn print_tree(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "print_tree", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Prints the node and its children to the console, recursively. The node does not have to be inside the tree. Similar to [`print_tree`][`crate::classes::Node::print_tree`], but the graphical representation looks like what is displayed in the editor's Scene dock. It is useful for inspecting larger trees.\n\nMay print, for example:\n\n```text\n ┖╴TheGame\n    ┠╴Menu\n    ┃  ┠╴Label\n    ┃  ┖╴Camera2D\n    ┖╴SplashScreen\n       ┖╴Camera2D\n```"]
        pub fn print_tree_pretty(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "print_tree_pretty", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tree as a [`String`][crate::builtin::GString]. Used mainly for debugging purposes. This version displays the path relative to the current node, and is good for copy/pasting into the [`get_node_as`][`crate::classes::Node::get_node_as`] function. It also can be used in game UI/UX.\n\nMay print, for example:\n\n```text\nTheGame\nTheGame/Menu\nTheGame/Menu/Label\nTheGame/Menu/Camera2D\nTheGame/SplashScreen\nTheGame/SplashScreen/Camera2D\n```"]
        pub fn get_tree_string(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_tree_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Similar to [`get_tree_string`][`crate::classes::Node::get_tree_string`], this returns the tree as a [`String`][crate::builtin::GString]. This version displays a more graphical representation similar to what is displayed in the Scene Dock. It is useful for inspecting larger trees.\n\nMay print, for example:\n\n```text\n ┖╴TheGame\n    ┠╴Menu\n    ┃  ┠╴Label\n    ┃  ┖╴Camera2D\n    ┖╴SplashScreen\n       ┖╴Camera2D\n```"]
        pub fn get_tree_string_pretty(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_tree_string_pretty", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scene_file_path(&mut self, scene_file_path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (scene_file_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_scene_file_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scene_file_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_scene_file_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calls [`notify`][`crate::classes::Object::notify`] with `what` on this node and all of its children, recursively."]
        pub fn propagate_notification(&mut self, what: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (what,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "propagate_notification", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calls the given `method` name, passing `args` as arguments, on this node and all of its children, recursively.\n\nIf `parent_first` is `true`, the method is called on this node first, then on all of its children. If `false`, the children's methods are called first."]
        pub(crate) fn propagate_call_full(&mut self, method: CowArg < StringName >, args: RefArg < AnyArray >, parent_first: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, AnyArray >, bool,);
            let args = (method, args, parent_first,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "propagate_call", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`propagate_call_ex`][Self::propagate_call_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Calls the given `method` name, passing `args` as arguments, on this node and all of its children, recursively.\n\nIf `parent_first` is `true`, the method is called on this node first, then on all of its children. If `false`, the children's methods are called first."]
        #[inline]
        pub fn propagate_call(&mut self, method: impl AsArg < StringName >,) {
            self.propagate_call_ex(method,) . done()
        }
        #[doc = "Calls the given `method` name, passing `args` as arguments, on this node and all of its children, recursively.\n\nIf `parent_first` is `true`, the method is called on this node first, then on all of its children. If `false`, the children's methods are called first."]
        #[inline]
        pub fn propagate_call_ex < 'ex > (&'ex mut self, method: impl AsArg < StringName > + 'ex,) -> ExPropagateCall < 'ex > {
            ExPropagateCall::new(self, method,)
        }
        #[doc = "If set to `true`, enables physics (fixed framerate) processing. When a node is being processed, it will receive a [`NodeNotification::PHYSICS_PROCESS`][`crate::classes::notify::NodeNotification::PHYSICS_PROCESS`] at a fixed (usually 60 FPS, see \\[member Engine.physics_ticks_per_second] to change) interval (and the [`physics_process`][`crate::classes::INode::physics_process`] callback will be called if it exists).\n\n**Note:** If [`physics_process`][`crate::classes::INode::physics_process`] is overridden, this will be automatically enabled before [`ready`][`crate::classes::INode::ready`] is called."]
        pub fn set_physics_process(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_physics_process", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the time elapsed (in seconds) since the last physics callback. This value is identical to [`physics_process`][`crate::classes::INode::physics_process`]'s `delta` parameter, and is often consistent at run-time, unless \\[member Engine.physics_ticks_per_second] is changed. See also [`NodeNotification::PHYSICS_PROCESS`][`crate::classes::notify::NodeNotification::PHYSICS_PROCESS`].\n\n**Note:** The returned value will be larger than expected if running at a framerate lower than \\[member Engine.physics_ticks_per_second] / \\[member Engine.max_physics_steps_per_frame] FPS. This is done to avoid \"spiral of death\" scenarios where performance would plummet due to an ever-increasing number of physics steps per frame. This behavior affects both [`process`][`crate::classes::INode::process`] and [`physics_process`][`crate::classes::INode::physics_process`]. As a result, avoid using `delta` for time measurements in real-world seconds. Use the [`Time`][crate::classes::Time] singleton's methods for this purpose instead, such as [`get_ticks_usec`][`crate::classes::Time::get_ticks_usec`]."]
        pub fn get_physics_process_delta_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9369usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_physics_process_delta_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if physics processing is enabled (see [`set_physics_process`][`crate::classes::Node::set_physics_process`])."]
        pub fn is_physics_processing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9370usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_physics_processing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the time elapsed (in seconds) since the last process callback. This value is identical to [`process`][`crate::classes::INode::process`]'s `delta` parameter, and may vary from frame to frame. See also [`NodeNotification::PROCESS`][`crate::classes::notify::NodeNotification::PROCESS`].\n\n**Note:** The returned value will be larger than expected if running at a framerate lower than \\[member Engine.physics_ticks_per_second] / \\[member Engine.max_physics_steps_per_frame] FPS. This is done to avoid \"spiral of death\" scenarios where performance would plummet due to an ever-increasing number of physics steps per frame. This behavior affects both [`process`][`crate::classes::INode::process`] and [`physics_process`][`crate::classes::INode::physics_process`]. As a result, avoid using `delta` for time measurements in real-world seconds. Use the [`Time`][crate::classes::Time] singleton's methods for this purpose instead, such as [`get_ticks_usec`][`crate::classes::Time::get_ticks_usec`]."]
        pub fn get_process_delta_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9371usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_process_delta_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, enables processing. When a node is being processed, it will receive a [`NodeNotification::PROCESS`][`crate::classes::notify::NodeNotification::PROCESS`] on every drawn frame (and the [`process`][`crate::classes::INode::process`] callback will be called if it exists).\n\n**Note:** If [`process`][`crate::classes::INode::process`] is overridden, this will be automatically enabled before [`ready`][`crate::classes::INode::ready`] is called.\n\n**Note:** This method only affects the [`process`][`crate::classes::INode::process`] callback, i.e. it has no effect on other callbacks like [`physics_process`][`crate::classes::INode::physics_process`]. If you want to disable all processing for the node, set \\[member process_mode] to [`ProcessMode::DISABLED`][`crate::classes::node::ProcessMode::DISABLED`]."]
        pub fn set_process(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9372usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_priority(&mut self, priority: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9373usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_priority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9374usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_process_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_process_priority(&mut self, priority: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9375usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_physics_process_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_process_priority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_physics_process_priority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if processing is enabled (see [`set_process`][`crate::classes::Node::set_process`])."]
        pub fn is_processing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_processing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, enables input processing.\n\n**Note:** If [`input`][`crate::classes::INode::input`] is overridden, this will be automatically enabled before [`ready`][`crate::classes::INode::ready`] is called. Input processing is also already enabled for GUI controls, such as [`Button`][crate::classes::Button] and [`TextEdit`][crate::classes::TextEdit]."]
        pub fn set_process_input(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the node is processing input (see [`set_process_input`][`crate::classes::Node::set_process_input`])."]
        pub fn is_processing_input(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_processing_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, enables shortcut processing for this node.\n\n**Note:** If [`shortcut_input`][`crate::classes::INode::shortcut_input`] is overridden, this will be automatically enabled before [`ready`][`crate::classes::INode::ready`] is called."]
        pub fn set_process_shortcut_input(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_shortcut_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the node is processing shortcuts (see [`set_process_shortcut_input`][`crate::classes::Node::set_process_shortcut_input`])."]
        pub fn is_processing_shortcut_input(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_processing_shortcut_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, enables unhandled input processing. It enables the node to receive all input that was not previously handled (usually by a [`Control`][crate::classes::Control]).\n\n**Note:** If [`unhandled_input`][`crate::classes::INode::unhandled_input`] is overridden, this will be automatically enabled before [`ready`][`crate::classes::INode::ready`] is called. Unhandled input processing is also already enabled for GUI controls, such as [`Button`][crate::classes::Button] and [`TextEdit`][crate::classes::TextEdit]."]
        pub fn set_process_unhandled_input(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_unhandled_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the node is processing unhandled input (see [`set_process_unhandled_input`][`crate::classes::Node::set_process_unhandled_input`])."]
        pub fn is_processing_unhandled_input(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_processing_unhandled_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, enables unhandled key input processing.\n\n**Note:** If [`unhandled_key_input`][`crate::classes::INode::unhandled_key_input`] is overridden, this will be automatically enabled before [`ready`][`crate::classes::INode::ready`] is called."]
        pub fn set_process_unhandled_key_input(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_unhandled_key_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the node is processing unhandled key input (see [`set_process_unhandled_key_input`][`crate::classes::Node::set_process_unhandled_key_input`])."]
        pub fn is_processing_unhandled_key_input(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_processing_unhandled_key_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_mode(&mut self, mode: crate::classes::node::ProcessMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::node::ProcessMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_mode(&self,) -> crate::classes::node::ProcessMode {
            type CallRet = crate::classes::node::ProcessMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_process_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the node can receive processing notifications and input callbacks ([`NodeNotification::PROCESS`][`crate::classes::notify::NodeNotification::PROCESS`], [`input`][`crate::classes::INode::input`], etc.) from the [`SceneTree`][crate::classes::SceneTree] and [`Viewport`][crate::classes::Viewport]. The returned value depends on \\[member process_mode]:\n\n- If set to [`ProcessMode::PAUSABLE`][`crate::classes::node::ProcessMode::PAUSABLE`], returns `true` when the game is processing, i.e. \\[member SceneTree.paused] is `false`;\n\n- If set to [`ProcessMode::WHEN_PAUSED`][`crate::classes::node::ProcessMode::WHEN_PAUSED`], returns `true` when the game is paused, i.e. \\[member SceneTree.paused] is `true`;\n\n- If set to [`ProcessMode::ALWAYS`][`crate::classes::node::ProcessMode::ALWAYS`], always returns `true`;\n\n- If set to [`ProcessMode::DISABLED`][`crate::classes::node::ProcessMode::DISABLED`], always returns `false`;\n\n- If set to [`ProcessMode::INHERIT`][`crate::classes::node::ProcessMode::INHERIT`], use the parent node's \\[member process_mode] to determine the result.\n\nIf the node is not inside the tree, returns `false` no matter the value of \\[member process_mode]."]
        pub fn can_process(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "can_process", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_thread_group(&mut self, mode: crate::classes::node::ProcessThreadGroup,) {
            type CallRet = ();
            type CallParams = (crate::classes::node::ProcessThreadGroup,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_thread_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_thread_group(&self,) -> crate::classes::node::ProcessThreadGroup {
            type CallRet = crate::classes::node::ProcessThreadGroup;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_process_thread_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_thread_messages(&mut self, flags: crate::classes::node::ProcessThreadMessages,) {
            type CallRet = ();
            type CallParams = (crate::classes::node::ProcessThreadMessages,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_thread_messages", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_thread_messages(&self,) -> crate::classes::node::ProcessThreadMessages {
            type CallRet = crate::classes::node::ProcessThreadMessages;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_process_thread_messages", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_thread_group_order(&mut self, order: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_thread_group_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_thread_group_order(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_process_thread_group_order", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Queues an accessibility information update for this node."]
        pub fn queue_accessibility_update(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "queue_accessibility_update", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns main accessibility element RID.\n\n**Note:** This method should be called only during accessibility information updates ([`NodeNotification::ACCESSIBILITY_UPDATE`][`crate::classes::notify::NodeNotification::ACCESSIBILITY_UPDATE`])."]
        pub fn get_accessibility_element(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_accessibility_element", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, the node appears folded in the Scene dock. As a result, all of its children are hidden. This method is intended to be used in editor plugins and tools, but it also works in release builds. See also [`is_displayed_folded`][`crate::classes::Node::is_displayed_folded`]."]
        pub fn set_display_folded(&mut self, fold: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (fold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_display_folded", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the node is folded (collapsed) in the Scene dock. This method is intended to be used in editor plugins and tools. See also [`set_display_folded`][`crate::classes::Node::set_display_folded`]."]
        pub fn is_displayed_folded(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_displayed_folded", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, enables internal processing for this node. Internal processing happens in isolation from the normal [`process`][`crate::classes::INode::process`] calls and is used by some nodes internally to guarantee proper functioning even if the node is paused or processing is disabled for scripting ([`set_process`][`crate::classes::Node::set_process`]).\n\n**Warning:** Built-in nodes rely on internal processing for their internal logic. Disabling it is unsafe and may lead to unexpected behavior. Use this method if you know what you are doing."]
        pub fn set_process_internal(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_internal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if internal processing is enabled (see [`set_process_internal`][`crate::classes::Node::set_process_internal`])."]
        pub fn is_processing_internal(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_processing_internal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, enables internal physics for this node. Internal physics processing happens in isolation from the normal [`physics_process`][`crate::classes::INode::physics_process`] calls and is used by some nodes internally to guarantee proper functioning even if the node is paused or physics processing is disabled for scripting ([`set_physics_process`][`crate::classes::Node::set_physics_process`]).\n\n**Warning:** Built-in nodes rely on internal processing for their internal logic. Disabling it is unsafe and may lead to unexpected behavior. Use this method if you know what you are doing."]
        pub fn set_physics_process_internal(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_physics_process_internal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if internal physics processing is enabled (see [`set_physics_process_internal`][`crate::classes::Node::set_physics_process_internal`])."]
        pub fn is_physics_processing_internal(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_physics_processing_internal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_interpolation_mode(&mut self, mode: crate::classes::node::PhysicsInterpolationMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::node::PhysicsInterpolationMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_physics_interpolation_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_interpolation_mode(&self,) -> crate::classes::node::PhysicsInterpolationMode {
            type CallRet = crate::classes::node::PhysicsInterpolationMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_physics_interpolation_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if physics interpolation is enabled for this node (see \\[member physics_interpolation_mode]).\n\n**Note:** Interpolation will only be active if both the flag is set **and** physics interpolation is enabled within the [`SceneTree`][crate::classes::SceneTree]. This can be tested using [`is_physics_interpolated_and_enabled`][`crate::classes::Node::is_physics_interpolated_and_enabled`]."]
        pub fn is_physics_interpolated(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_physics_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if physics interpolation is enabled (see \\[member physics_interpolation_mode]) **and** enabled in the [`SceneTree`][crate::classes::SceneTree].\n\nThis is a convenience version of [`is_physics_interpolated`][`crate::classes::Node::is_physics_interpolated`] that also checks whether physics interpolation is enabled globally.\n\nSee \\[member SceneTree.physics_interpolation] and \\[member ProjectSettings.physics/common/physics_interpolation]."]
        pub fn is_physics_interpolated_and_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_physics_interpolated_and_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "When physics interpolation is active, moving a node to a radically different transform (such as placement within a level) can result in a visible glitch as the object is rendered moving from the old to new position over the physics tick.\n\nThat glitch can be prevented by calling this method, which temporarily disables interpolation until the physics tick is complete.\n\nThe notification [`NodeNotification::RESET_PHYSICS_INTERPOLATION`][`crate::classes::notify::NodeNotification::RESET_PHYSICS_INTERPOLATION`] will be received by the node and all children recursively.\n\n**Note:** This function should be called **after** moving the node, rather than before."]
        pub fn reset_physics_interpolation(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "reset_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_translate_mode(&mut self, mode: crate::classes::node::AutoTranslateMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::node::AutoTranslateMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_translate_mode(&self,) -> crate::classes::node::AutoTranslateMode {
            type CallRet = crate::classes::node::AutoTranslateMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this node can automatically translate messages depending on the current locale. See \\[member auto_translate_mode], [`atr`][`crate::classes::Node::atr`], and [`atr_n`][`crate::classes::Node::atr_n`]."]
        pub fn can_auto_translate(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "can_auto_translate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Makes this node inherit the translation domain from its parent node. If this node has no parent, the main translation domain will be used.\n\nThis is the default behavior for all nodes. Calling [`set_translation_domain`][`crate::classes::Object::set_translation_domain`] disables this behavior."]
        pub fn set_translation_domain_inherited(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_translation_domain_inherited", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Window`][crate::classes::Window] that contains this node. If the node is in the main window, this is equivalent to getting the root node (`get_tree().get_root()`)."]
        pub fn get_window(&self,) -> Option < Gd < crate::classes::Window > > {
            type CallRet = Option < Gd < crate::classes::Window > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_window", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Window`][crate::classes::Window] that contains this node, or the last exclusive child in a chain of windows starting with the one that contains this node."]
        pub fn get_last_exclusive_window(&self,) -> Option < Gd < crate::classes::Window > > {
            type CallRet = Option < Gd < crate::classes::Window > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_last_exclusive_window", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`SceneTree`][crate::classes::SceneTree] that contains this node. If this node is not inside the tree, generates an error and returns `null`. See also [`is_inside_tree`][`crate::classes::Node::is_inside_tree`]."]
        pub(crate) fn raw_get_tree(&self,) -> Option < Gd < crate::classes::SceneTree > > {
            type CallRet = Option < Gd < crate::classes::SceneTree > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "raw_get_tree", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new [`Tween`][crate::classes::Tween] and binds it to this node.\n\nThis is the equivalent of doing:\n\n\n```gdscript\nget_tree().create_tween().bind_node(self)\n```\n\n\nThe Tween will start automatically on the next process frame or physics frame (depending on \\[enum Tween.TweenProcessMode]). See [`bind_node`][`crate::classes::Tween::bind_node`] for more info on Tweens bound to nodes.\n\n**Note:** The method can still be used when the node is not inside [`SceneTree`][crate::classes::SceneTree]. It can fail in an unlikely case of using a custom [`MainLoop`][crate::classes::MainLoop]."]
        pub fn create_tween(&mut self,) -> Gd < crate::classes::Tween > {
            type CallRet = Gd < crate::classes::Tween >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "create_tween", Some(self.__validated_obj()), args,)
            }
        }
        #[deprecated = "Use `Gd::duplicate_node()` or `Gd::duplicate_node_ex()`."]
        #[doc = "Duplicates the node, returning a new node with all of its properties, signals, groups, and children copied from the original, recursively. The behavior can be tweaked through the `flags` (see \\[enum DuplicateFlags]). Internal nodes are not duplicated.\n\n**Note:** For nodes with a [`Script`][crate::classes::Script] attached, if [`init`][`crate::classes::IObject::init`] has been defined with required parameters, the duplicated node will not have a [`Script`][crate::classes::Script].\n\n**Note:** By default, this method will duplicate only properties marked for serialization (i.e. using `@GlobalScope.PROPERTY_USAGE_STORAGE`, or in GDScript, `@GDScript.@export`). If you want to duplicate all properties, use [`DuplicateFlags::INTERNAL_STATE`][`crate::classes::node::DuplicateFlags::INTERNAL_STATE`]."]
        pub(crate) fn duplicate_full(&self, flags: crate::classes::node::DuplicateFlags,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = (crate::classes::node::DuplicateFlags,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "duplicate", Some(self.__validated_obj()), args,)
            }
        }
        #[deprecated = "Use `Gd::duplicate_node()` or `Gd::duplicate_node_ex()`."]
        #[expect(deprecated)]
        #[doc = "To set the default parameters, use [`duplicate_ex`][Self::duplicate_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Duplicates the node, returning a new node with all of its properties, signals, groups, and children copied from the original, recursively. The behavior can be tweaked through the `flags` (see \\[enum DuplicateFlags]). Internal nodes are not duplicated.\n\n**Note:** For nodes with a [`Script`][crate::classes::Script] attached, if [`init`][`crate::classes::IObject::init`] has been defined with required parameters, the duplicated node will not have a [`Script`][crate::classes::Script].\n\n**Note:** By default, this method will duplicate only properties marked for serialization (i.e. using `@GlobalScope.PROPERTY_USAGE_STORAGE`, or in GDScript, `@GDScript.@export`). If you want to duplicate all properties, use [`DuplicateFlags::INTERNAL_STATE`][`crate::classes::node::DuplicateFlags::INTERNAL_STATE`]."]
        #[inline]
        pub fn duplicate(&self,) -> Option < Gd < crate::classes::Node > > {
            self.duplicate_ex() . done()
        }
        #[deprecated = "Use `Gd::duplicate_node()` or `Gd::duplicate_node_ex()`."]
        #[doc = "Duplicates the node, returning a new node with all of its properties, signals, groups, and children copied from the original, recursively. The behavior can be tweaked through the `flags` (see \\[enum DuplicateFlags]). Internal nodes are not duplicated.\n\n**Note:** For nodes with a [`Script`][crate::classes::Script] attached, if [`init`][`crate::classes::IObject::init`] has been defined with required parameters, the duplicated node will not have a [`Script`][crate::classes::Script].\n\n**Note:** By default, this method will duplicate only properties marked for serialization (i.e. using `@GlobalScope.PROPERTY_USAGE_STORAGE`, or in GDScript, `@GDScript.@export`). If you want to duplicate all properties, use [`DuplicateFlags::INTERNAL_STATE`][`crate::classes::node::DuplicateFlags::INTERNAL_STATE`]."]
        #[inline]
        pub fn duplicate_ex < 'ex > (&'ex self,) -> ExDuplicate < 'ex > {
            ExDuplicate::new(self,)
        }
        #[doc = "Replaces this node by the given `node`. All children of this node are moved to `node`.\n\nIf `keep_groups` is `true`, the `node` is added to the same groups that the replaced node is in (see [`add_to_group`][`crate::classes::Node::add_to_group`]).\n\n**Warning:** The replaced node is removed from the tree, but it is **not** deleted. To prevent memory leaks, store a reference to the node in a variable, or use [`free`][`crate::obj::Gd::free`]."]
        pub(crate) fn replace_by_full(&mut self, node: CowArg < Gd < crate::classes::Node > >, keep_groups: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >, bool,);
            let args = (node, keep_groups,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "replace_by", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`replace_by_ex`][Self::replace_by_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Replaces this node by the given `node`. All children of this node are moved to `node`.\n\nIf `keep_groups` is `true`, the `node` is added to the same groups that the replaced node is in (see [`add_to_group`][`crate::classes::Node::add_to_group`]).\n\n**Warning:** The replaced node is removed from the tree, but it is **not** deleted. To prevent memory leaks, store a reference to the node in a variable, or use [`free`][`crate::obj::Gd::free`]."]
        #[inline]
        pub fn replace_by(&mut self, node: impl AsArg < Gd < crate::classes::Node >>,) {
            self.replace_by_ex(node,) . done()
        }
        #[doc = "Replaces this node by the given `node`. All children of this node are moved to `node`.\n\nIf `keep_groups` is `true`, the `node` is added to the same groups that the replaced node is in (see [`add_to_group`][`crate::classes::Node::add_to_group`]).\n\n**Warning:** The replaced node is removed from the tree, but it is **not** deleted. To prevent memory leaks, store a reference to the node in a variable, or use [`free`][`crate::obj::Gd::free`]."]
        #[inline]
        pub fn replace_by_ex < 'ex > (&'ex mut self, node: impl AsArg < Gd < crate::classes::Node >> + 'ex,) -> ExReplaceBy < 'ex > {
            ExReplaceBy::new(self, node,)
        }
        #[doc = "If set to `true`, the node becomes an [`InstancePlaceholder`][crate::classes::InstancePlaceholder] when packed and instantiated from a [`PackedScene`][crate::classes::PackedScene]. See also [`get_scene_instance_load_placeholder`][`crate::classes::Node::get_scene_instance_load_placeholder`]."]
        pub fn set_scene_instance_load_placeholder(&mut self, load_placeholder: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (load_placeholder,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_scene_instance_load_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this node is an instance load placeholder. See [`InstancePlaceholder`][crate::classes::InstancePlaceholder] and [`set_scene_instance_load_placeholder`][`crate::classes::Node::set_scene_instance_load_placeholder`]."]
        pub fn get_scene_instance_load_placeholder(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_scene_instance_load_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set to `true` to allow all nodes owned by `node` to be available, and editable, in the Scene dock, even if their \\[member owner] is not the scene root. This method is intended to be used in editor plugins and tools, but it also works in release builds. See also [`is_editable_instance`][`crate::classes::Node::is_editable_instance`]."]
        pub fn set_editable_instance(&mut self, node: impl AsArg < Gd < crate::classes::Node >>, is_editable: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >, bool,);
            let args = (node.into_arg(), is_editable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_editable_instance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if `node` has editable children enabled relative to this node. This method is intended to be used in editor plugins and tools. See also [`set_editable_instance`][`crate::classes::Node::set_editable_instance`]."]
        pub fn is_editable_instance(&self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_editable_instance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the node's closest [`Viewport`][crate::classes::Viewport] ancestor, if the node is inside the tree. Otherwise, returns `null`."]
        pub fn get_viewport(&self,) -> Option < Gd < crate::classes::Viewport > > {
            type CallRet = Option < Gd < crate::classes::Viewport > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_viewport", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Queues this node to be deleted at the end of the current frame. When deleted, all of its children are deleted as well, and all references to the node and its children become invalid.\n\nUnlike with [`free`][`crate::obj::Gd::free`], the node is not deleted instantly, and it can still be accessed before deletion. It is also safe to call [`queue_free`][`crate::classes::Node::queue_free`] multiple times. Use [`is_queued_for_deletion`][`crate::classes::Object::is_queued_for_deletion`] to check if the node will be deleted at the end of the frame.\n\n**Note:** The node will only be freed after all other deferred calls are finished. Using this method is not always the same as calling [`free`][`crate::obj::Gd::free`] through [`call_deferred`][`crate::classes::Object::call_deferred`]."]
        pub fn queue_free(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "queue_free", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Requests [`ready`][`crate::classes::INode::ready`] to be called again the next time the node enters the tree. Does **not** immediately call [`ready`][`crate::classes::INode::ready`].\n\n**Note:** This method only affects the current node. If the node's children also need to request ready, this method needs to be called for each one of them. When the node and its children enter the tree again, the order of [`ready`][`crate::classes::INode::ready`] callbacks will be the same as normal."]
        pub fn request_ready(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "request_ready", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the node is ready, i.e. it's inside scene tree and all its children are initialized.\n\n[`request_ready`][`crate::classes::Node::request_ready`] resets it back to `false`."]
        pub fn is_node_ready(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_node_ready", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the node's multiplayer authority to the peer with the given peer `id`. The multiplayer authority is the peer that has authority over the node on the network. Defaults to peer ID 1 (the server). Useful in conjunction with [`rpc_config`][`crate::classes::Node::rpc_config`] and the [`MultiplayerAPI`][crate::classes::MultiplayerApi].\n\nIf `recursive` is `true`, the given peer is recursively set as the authority for all children of this node.\n\n**Warning:** This does **not** automatically replicate the new authority to other peers. It is the developer's responsibility to do so. You may replicate the new authority's information using \\[member MultiplayerSpawner.spawn_function], an RPC, or a [`MultiplayerSynchronizer`][crate::classes::MultiplayerSynchronizer]. Furthermore, the parent's authority does **not** propagate to newly added children."]
        pub(crate) fn set_multiplayer_authority_full(&mut self, id: i32, recursive: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (id, recursive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_multiplayer_authority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_multiplayer_authority_ex`][Self::set_multiplayer_authority_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the node's multiplayer authority to the peer with the given peer `id`. The multiplayer authority is the peer that has authority over the node on the network. Defaults to peer ID 1 (the server). Useful in conjunction with [`rpc_config`][`crate::classes::Node::rpc_config`] and the [`MultiplayerAPI`][crate::classes::MultiplayerApi].\n\nIf `recursive` is `true`, the given peer is recursively set as the authority for all children of this node.\n\n**Warning:** This does **not** automatically replicate the new authority to other peers. It is the developer's responsibility to do so. You may replicate the new authority's information using \\[member MultiplayerSpawner.spawn_function], an RPC, or a [`MultiplayerSynchronizer`][crate::classes::MultiplayerSynchronizer]. Furthermore, the parent's authority does **not** propagate to newly added children."]
        #[inline]
        pub fn set_multiplayer_authority(&mut self, id: i32,) {
            self.set_multiplayer_authority_ex(id,) . done()
        }
        #[doc = "Sets the node's multiplayer authority to the peer with the given peer `id`. The multiplayer authority is the peer that has authority over the node on the network. Defaults to peer ID 1 (the server). Useful in conjunction with [`rpc_config`][`crate::classes::Node::rpc_config`] and the [`MultiplayerAPI`][crate::classes::MultiplayerApi].\n\nIf `recursive` is `true`, the given peer is recursively set as the authority for all children of this node.\n\n**Warning:** This does **not** automatically replicate the new authority to other peers. It is the developer's responsibility to do so. You may replicate the new authority's information using \\[member MultiplayerSpawner.spawn_function], an RPC, or a [`MultiplayerSynchronizer`][crate::classes::MultiplayerSynchronizer]. Furthermore, the parent's authority does **not** propagate to newly added children."]
        #[inline]
        pub fn set_multiplayer_authority_ex < 'ex > (&'ex mut self, id: i32,) -> ExSetMultiplayerAuthority < 'ex > {
            ExSetMultiplayerAuthority::new(self, id,)
        }
        #[doc = "Returns the peer ID of the multiplayer authority for this node. See [`set_multiplayer_authority`][`crate::classes::Node::set_multiplayer_authority`]."]
        pub fn get_multiplayer_authority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_multiplayer_authority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the local system is the multiplayer authority of this node."]
        pub fn is_multiplayer_authority(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_multiplayer_authority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_multiplayer(&self,) -> Option < Gd < crate::classes::MultiplayerApi > > {
            type CallRet = Option < Gd < crate::classes::MultiplayerApi > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_multiplayer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Changes the RPC configuration for the given `method`. `config` should either be `null` to disable the feature (as by default), or a [`Dictionary`][crate::builtin::Dictionary] containing the following entries:\n\n- `rpc_mode`: see \\[enum MultiplayerAPI.RPCMode];\n\n- `transfer_mode`: see \\[enum MultiplayerPeer.TransferMode];\n\n- `call_local`: if `true`, the method will also be called locally;\n\n- `channel`: an `int` representing the channel to send the RPC on.\n\n**Note:** In GDScript, this method corresponds to the `@GDScript.@rpc` annotation, with various parameters passed (`@rpc(any)`, `@rpc(authority)`...). See also the [high-level multiplayer]($DOCS_URL/tutorials/networking/high_level_multiplayer.html) tutorial."]
        pub fn rpc_config(&mut self, method: impl AsArg < StringName >, config: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (method.into_arg(), RefArg::new(config),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "rpc_config", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Dictionary`][crate::builtin::Dictionary] mapping method names to their RPC configuration defined for this node using [`rpc_config`][`crate::classes::Node::rpc_config`].\n\n**Note:** This method only returns the RPC configuration assigned via [`rpc_config`][`crate::classes::Node::rpc_config`]. See [`get_rpc_config`][`crate::classes::Script::get_rpc_config`] to retrieve the RPCs defined by the [`Script`][crate::classes::Script]."]
        pub fn get_node_rpc_config(&self,) -> Variant {
            type CallRet = Variant;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_node_rpc_config", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_editor_description(&mut self, editor_description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (editor_description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_editor_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_editor_description(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_editor_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_unique_name_in_owner(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_unique_name_in_owner", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_unique_name_in_owner(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_unique_name_in_owner", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Translates a `message`, using the translation catalogs configured in the Project Settings. Further `context` can be specified to help with the translation. Note that most [`Control`][crate::classes::Control] nodes automatically translate their strings, so this method is mostly useful for formatted strings or custom drawn text.\n\nThis method works the same as [`tr`][`crate::classes::Object::tr`], with the addition of respecting the \\[member auto_translate_mode] state.\n\nIf [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] is `false`, or no translation is available, this method returns the `message` without changes. See [`set_message_translation`][`crate::classes::Object::set_message_translation`].\n\nFor detailed examples, see [Internationalizing games]($DOCS_URL/tutorials/i18n/internationalizing_games.html)."]
        pub(crate) fn atr_full(&self, message: CowArg < GString >, context: CowArg < StringName >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, StringName >,);
            let args = (message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9436usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "atr", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`atr_ex`][Self::atr_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Translates a `message`, using the translation catalogs configured in the Project Settings. Further `context` can be specified to help with the translation. Note that most [`Control`][crate::classes::Control] nodes automatically translate their strings, so this method is mostly useful for formatted strings or custom drawn text.\n\nThis method works the same as [`tr`][`crate::classes::Object::tr`], with the addition of respecting the \\[member auto_translate_mode] state.\n\nIf [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] is `false`, or no translation is available, this method returns the `message` without changes. See [`set_message_translation`][`crate::classes::Object::set_message_translation`].\n\nFor detailed examples, see [Internationalizing games]($DOCS_URL/tutorials/i18n/internationalizing_games.html)."]
        #[inline]
        pub fn atr(&self, message: impl AsArg < GString >,) -> GString {
            self.atr_ex(message,) . done()
        }
        #[doc = "Translates a `message`, using the translation catalogs configured in the Project Settings. Further `context` can be specified to help with the translation. Note that most [`Control`][crate::classes::Control] nodes automatically translate their strings, so this method is mostly useful for formatted strings or custom drawn text.\n\nThis method works the same as [`tr`][`crate::classes::Object::tr`], with the addition of respecting the \\[member auto_translate_mode] state.\n\nIf [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] is `false`, or no translation is available, this method returns the `message` without changes. See [`set_message_translation`][`crate::classes::Object::set_message_translation`].\n\nFor detailed examples, see [Internationalizing games]($DOCS_URL/tutorials/i18n/internationalizing_games.html)."]
        #[inline]
        pub fn atr_ex < 'ex > (&'ex self, message: impl AsArg < GString > + 'ex,) -> ExAtr < 'ex > {
            ExAtr::new(self, message,)
        }
        #[doc = "Translates a `message` or `plural_message`, using the translation catalogs configured in the Project Settings. Further `context` can be specified to help with the translation.\n\nThis method works the same as [`tr_n`][`crate::classes::Object::tr_n`], with the addition of respecting the \\[member auto_translate_mode] state.\n\nIf [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] is `false`, or no translation is available, this method returns `message` or `plural_message`, without changes. See [`set_message_translation`][`crate::classes::Object::set_message_translation`].\n\nThe `n` is the number, or amount, of the message's subject. It is used by the translation system to fetch the correct plural form for the current language.\n\nFor detailed examples, see [Localization using gettext]($DOCS_URL/tutorials/i18n/localization_using_gettext.html).\n\n**Note:** Negative and `float` numbers may not properly apply to some countable subjects. It's recommended to handle these cases with [`atr`][`crate::classes::Node::atr`]."]
        pub(crate) fn atr_n_full(&self, message: CowArg < GString >, plural_message: CowArg < StringName >, n: i32, context: CowArg < StringName >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, StringName >, i32, CowArg < 'a2, StringName >,);
            let args = (message, plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "atr_n", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`atr_n_ex`][Self::atr_n_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Translates a `message` or `plural_message`, using the translation catalogs configured in the Project Settings. Further `context` can be specified to help with the translation.\n\nThis method works the same as [`tr_n`][`crate::classes::Object::tr_n`], with the addition of respecting the \\[member auto_translate_mode] state.\n\nIf [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] is `false`, or no translation is available, this method returns `message` or `plural_message`, without changes. See [`set_message_translation`][`crate::classes::Object::set_message_translation`].\n\nThe `n` is the number, or amount, of the message's subject. It is used by the translation system to fetch the correct plural form for the current language.\n\nFor detailed examples, see [Localization using gettext]($DOCS_URL/tutorials/i18n/localization_using_gettext.html).\n\n**Note:** Negative and `float` numbers may not properly apply to some countable subjects. It's recommended to handle these cases with [`atr`][`crate::classes::Node::atr`]."]
        #[inline]
        pub fn atr_n(&self, message: impl AsArg < GString >, plural_message: impl AsArg < StringName >, n: i32,) -> GString {
            self.atr_n_ex(message, plural_message, n,) . done()
        }
        #[doc = "Translates a `message` or `plural_message`, using the translation catalogs configured in the Project Settings. Further `context` can be specified to help with the translation.\n\nThis method works the same as [`tr_n`][`crate::classes::Object::tr_n`], with the addition of respecting the \\[member auto_translate_mode] state.\n\nIf [`can_translate_messages`][`crate::classes::Object::can_translate_messages`] is `false`, or no translation is available, this method returns `message` or `plural_message`, without changes. See [`set_message_translation`][`crate::classes::Object::set_message_translation`].\n\nThe `n` is the number, or amount, of the message's subject. It is used by the translation system to fetch the correct plural form for the current language.\n\nFor detailed examples, see [Localization using gettext]($DOCS_URL/tutorials/i18n/localization_using_gettext.html).\n\n**Note:** Negative and `float` numbers may not properly apply to some countable subjects. It's recommended to handle these cases with [`atr`][`crate::classes::Node::atr`]."]
        #[inline]
        pub fn atr_n_ex < 'ex > (&'ex self, message: impl AsArg < GString > + 'ex, plural_message: impl AsArg < StringName > + 'ex, n: i32,) -> ExAtrN < 'ex > {
            ExAtrN::new(self, message, plural_message, n,)
        }
        #[doc = "Sends a remote procedure call request for the given `method` to peers on the network (and locally), sending additional arguments to the method called by the RPC. The call request will only be received by nodes with the same [`NodePath`][crate::builtin::NodePath], including the exact same \\[member name]. Behavior depends on the RPC configuration for the given `method` (see [`rpc_config`][`crate::classes::Node::rpc_config`] and `@GDScript.@rpc`). By default, methods are not exposed to RPCs.\n\nMay return [`Error::OK`][`crate::global::Error::OK`] if the call is successful, [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if the arguments passed in the `method` do not match, [`Error::ERR_UNCONFIGURED`][`crate::global::Error::ERR_UNCONFIGURED`] if the node's \\[member multiplayer] cannot be fetched (such as when the node is not inside the tree), [`Error::ERR_CONNECTION_ERROR`][`crate::global::Error::ERR_CONNECTION_ERROR`] if \\[member multiplayer]'s connection is not available.\n\n**Note:** You can only safely use RPCs on clients after you received the `MultiplayerAPI.connected_to_server` signal from the [`MultiplayerAPI`][crate::classes::MultiplayerApi]. You also need to keep track of the connection state, either by the [`MultiplayerAPI`][crate::classes::MultiplayerApi] signals like `MultiplayerAPI.server_disconnected` or by checking (`get_multiplayer().peer.get_connection_status() == CONNECTION_CONNECTED`)."]
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn rpc(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> crate::global::Error {
            Self::try_rpc(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_rpc(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < crate::global::Error, crate::meta::error::CallError > {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9438usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Node", "rpc", Some(self.__validated_obj()), args, varargs)
            }
        }
        #[doc = "Sends a [`rpc`][`crate::classes::Node::rpc`] to a specific peer identified by `peer_id` (see [`set_target_peer`][`crate::classes::MultiplayerPeer::set_target_peer`]).\n\nMay return [`Error::OK`][`crate::global::Error::OK`] if the call is successful, [`Error::ERR_INVALID_PARAMETER`][`crate::global::Error::ERR_INVALID_PARAMETER`] if the arguments passed in the `method` do not match, [`Error::ERR_UNCONFIGURED`][`crate::global::Error::ERR_UNCONFIGURED`] if the node's \\[member multiplayer] cannot be fetched (such as when the node is not inside the tree), [`Error::ERR_CONNECTION_ERROR`][`crate::global::Error::ERR_CONNECTION_ERROR`] if \\[member multiplayer]'s connection is not available."]
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn rpc_id(&mut self, peer_id: i64, method: impl AsArg < StringName >, varargs: &[Variant]) -> crate::global::Error {
            Self::try_rpc_id(self, peer_id, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_rpc_id(&mut self, peer_id: i64, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < crate::global::Error, crate::meta::error::CallError > {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (i64, CowArg < 'a0, StringName >,);
            let args = (peer_id, method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9439usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Node", "rpc_id", Some(self.__validated_obj()), args, varargs)
            }
        }
        #[doc = "Refreshes the warnings displayed for this node in the Scene dock. Use [`get_configuration_warnings`][`crate::classes::INode::get_configuration_warnings`] to customize the warning messages to display."]
        pub fn update_configuration_warnings(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "update_configuration_warnings", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This function is similar to [`call_deferred`][`crate::classes::Object::call_deferred`] except that the call will take place when the node thread group is processed. If the node thread group processes in sub-threads, then the call will be done on that thread, right before [`NodeNotification::PROCESS`][`crate::classes::notify::NodeNotification::PROCESS`] or [`NodeNotification::PHYSICS_PROCESS`][`crate::classes::notify::NodeNotification::PHYSICS_PROCESS`], the [`process`][`crate::classes::INode::process`] or [`physics_process`][`crate::classes::INode::physics_process`] or their internal versions are called."]
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_deferred_thread_group(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_call_deferred_thread_group(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_deferred_thread_group(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9441usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Node", "call_deferred_thread_group", Some(self.__validated_obj()), args, varargs)
            }
        }
        #[doc = "Similar to [`call_deferred_thread_group`][`crate::classes::Node::call_deferred_thread_group`], but for setting properties."]
        pub fn set_deferred_thread_group(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_deferred_thread_group", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Similar to [`call_deferred_thread_group`][`crate::classes::Node::call_deferred_thread_group`], but for notifications."]
        pub fn notify_deferred_thread_group(&mut self, what: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (what,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "notify_deferred_thread_group", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This function ensures that the calling of this function will succeed, no matter whether it's being done from a thread or not. If called from a thread that is not allowed to call the function, the call will become deferred. Otherwise, the call will go through directly."]
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_thread_safe(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_call_thread_safe(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_thread_safe(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9444usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Node", "call_thread_safe", Some(self.__validated_obj()), args, varargs)
            }
        }
        #[doc = "Similar to [`call_thread_safe`][`crate::classes::Node::call_thread_safe`], but for setting properties."]
        pub fn set_thread_safe(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_thread_safe", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Similar to [`call_thread_safe`][`crate::classes::Node::call_thread_safe`], but for notifications."]
        pub fn notify_thread_safe(&mut self, what: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (what,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "notify_thread_safe", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: NodeNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: NodeNotification) {
            self.notification(i32::from(what), true);
            
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
        pub(crate) const NOTIFICATION_ENTER_TREE: i32 = 10i32;
        pub(crate) const NOTIFICATION_EXIT_TREE: i32 = 11i32;
        pub(crate) const NOTIFICATION_MOVED_IN_PARENT: i32 = 12i32;
        pub(crate) const NOTIFICATION_READY: i32 = 13i32;
        pub(crate) const NOTIFICATION_PAUSED: i32 = 14i32;
        pub(crate) const NOTIFICATION_UNPAUSED: i32 = 15i32;
        pub(crate) const NOTIFICATION_PHYSICS_PROCESS: i32 = 16i32;
        pub(crate) const NOTIFICATION_PROCESS: i32 = 17i32;
        pub(crate) const NOTIFICATION_PARENTED: i32 = 18i32;
        pub(crate) const NOTIFICATION_UNPARENTED: i32 = 19i32;
        pub(crate) const NOTIFICATION_SCENE_INSTANTIATED: i32 = 20i32;
        pub(crate) const NOTIFICATION_DRAG_BEGIN: i32 = 21i32;
        pub(crate) const NOTIFICATION_DRAG_END: i32 = 22i32;
        pub(crate) const NOTIFICATION_PATH_RENAMED: i32 = 23i32;
        pub(crate) const NOTIFICATION_CHILD_ORDER_CHANGED: i32 = 24i32;
        pub(crate) const NOTIFICATION_INTERNAL_PROCESS: i32 = 25i32;
        pub(crate) const NOTIFICATION_INTERNAL_PHYSICS_PROCESS: i32 = 26i32;
        pub(crate) const NOTIFICATION_POST_ENTER_TREE: i32 = 27i32;
        pub(crate) const NOTIFICATION_DISABLED: i32 = 28i32;
        pub(crate) const NOTIFICATION_ENABLED: i32 = 29i32;
        pub(crate) const NOTIFICATION_RESET_PHYSICS_INTERPOLATION: i32 = 2001i32;
        pub(crate) const NOTIFICATION_EDITOR_PRE_SAVE: i32 = 9001i32;
        pub(crate) const NOTIFICATION_EDITOR_POST_SAVE: i32 = 9002i32;
        pub(crate) const NOTIFICATION_WM_MOUSE_ENTER: i32 = 1002i32;
        pub(crate) const NOTIFICATION_WM_MOUSE_EXIT: i32 = 1003i32;
        pub(crate) const NOTIFICATION_WM_WINDOW_FOCUS_IN: i32 = 1004i32;
        pub(crate) const NOTIFICATION_WM_WINDOW_FOCUS_OUT: i32 = 1005i32;
        pub(crate) const NOTIFICATION_WM_CLOSE_REQUEST: i32 = 1006i32;
        pub(crate) const NOTIFICATION_WM_GO_BACK_REQUEST: i32 = 1007i32;
        pub(crate) const NOTIFICATION_WM_SIZE_CHANGED: i32 = 1008i32;
        pub(crate) const NOTIFICATION_WM_DPI_CHANGE: i32 = 1009i32;
        pub(crate) const NOTIFICATION_VP_MOUSE_ENTER: i32 = 1010i32;
        pub(crate) const NOTIFICATION_VP_MOUSE_EXIT: i32 = 1011i32;
        pub(crate) const NOTIFICATION_WM_POSITION_CHANGED: i32 = 1012i32;
        pub(crate) const NOTIFICATION_OS_MEMORY_WARNING: i32 = 2009i32;
        pub(crate) const NOTIFICATION_TRANSLATION_CHANGED: i32 = 2010i32;
        pub(crate) const NOTIFICATION_WM_ABOUT: i32 = 2011i32;
        pub(crate) const NOTIFICATION_CRASH: i32 = 2012i32;
        pub(crate) const NOTIFICATION_OS_IME_UPDATE: i32 = 2013i32;
        pub(crate) const NOTIFICATION_APPLICATION_RESUMED: i32 = 2014i32;
        pub(crate) const NOTIFICATION_APPLICATION_PAUSED: i32 = 2015i32;
        pub(crate) const NOTIFICATION_APPLICATION_FOCUS_IN: i32 = 2016i32;
        pub(crate) const NOTIFICATION_APPLICATION_FOCUS_OUT: i32 = 2017i32;
        pub(crate) const NOTIFICATION_TEXT_SERVER_CHANGED: i32 = 2018i32;
        pub(crate) const NOTIFICATION_ACCESSIBILITY_UPDATE: i32 = 3000i32;
        pub(crate) const NOTIFICATION_ACCESSIBILITY_INVALIDATE: i32 = 3001i32;
        
    }
    impl crate::obj::GodotClass for Node {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Node"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Node {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Node {
        
    }
    impl crate::obj::cap::GodotDefault for Node {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Node {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Node {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Node`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Node__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Node::add_sibling_ex`][super::Node::add_sibling_ex]."]
#[must_use]
pub struct ExAddSibling < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Node, sibling: CowArg < 'ex, Gd < crate::classes::Node > >, force_readable_name: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddSibling < 'ex > {
    fn new(surround_object: &'ex mut re_export::Node, sibling: impl AsArg < Gd < crate::classes::Node >> + 'ex,) -> Self {
        let force_readable_name = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, sibling: sibling.into_arg(), force_readable_name: force_readable_name,
        }
    }
    #[inline]
    pub fn force_readable_name(self, force_readable_name: bool) -> Self {
        Self {
            force_readable_name: force_readable_name, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, sibling, force_readable_name,
        }
        = self;
        re_export::Node::add_sibling_full(surround_object, sibling, force_readable_name,)
    }
}
#[doc = "Default-param extender for [`Node::add_child_ex`][super::Node::add_child_ex]."]
#[must_use]
pub struct ExAddChild < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Node, node: CowArg < 'ex, Gd < crate::classes::Node > >, force_readable_name: bool, internal: crate::classes::node::InternalMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddChild < 'ex > {
    fn new(surround_object: &'ex mut re_export::Node, node: impl AsArg < Gd < crate::classes::Node >> + 'ex,) -> Self {
        let force_readable_name = false;
        let internal = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.into_arg(), force_readable_name: force_readable_name, internal: internal,
        }
    }
    #[inline]
    pub fn force_readable_name(self, force_readable_name: bool) -> Self {
        Self {
            force_readable_name: force_readable_name, .. self
        }
    }
    #[inline]
    pub fn internal(self, internal: crate::classes::node::InternalMode) -> Self {
        Self {
            internal: internal, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, node, force_readable_name, internal,
        }
        = self;
        re_export::Node::add_child_full(surround_object, node, force_readable_name, internal,)
    }
}
#[doc = "Default-param extender for [`Node::reparent_ex`][super::Node::reparent_ex]."]
#[must_use]
pub struct ExReparent < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Node, new_parent: CowArg < 'ex, Gd < crate::classes::Node > >, keep_global_transform: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExReparent < 'ex > {
    fn new(surround_object: &'ex mut re_export::Node, new_parent: impl AsArg < Gd < crate::classes::Node >> + 'ex,) -> Self {
        let keep_global_transform = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, new_parent: new_parent.into_arg(), keep_global_transform: keep_global_transform,
        }
    }
    #[inline]
    pub fn keep_global_transform(self, keep_global_transform: bool) -> Self {
        Self {
            keep_global_transform: keep_global_transform, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, new_parent, keep_global_transform,
        }
        = self;
        re_export::Node::reparent_full(surround_object, new_parent, keep_global_transform,)
    }
}
#[doc = "Default-param extender for [`Node::get_child_count_ex`][super::Node::get_child_count_ex]."]
#[must_use]
pub struct ExGetChildCount < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Node, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetChildCount < 'ex > {
    fn new(surround_object: &'ex re_export::Node,) -> Self {
        let include_internal = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_internal: include_internal,
        }
    }
    #[inline]
    pub fn include_internal(self, include_internal: bool) -> Self {
        Self {
            include_internal: include_internal, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, include_internal,
        }
        = self;
        re_export::Node::get_child_count_full(surround_object, include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::get_children_ex`][super::Node::get_children_ex]."]
#[must_use]
pub struct ExGetChildren < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Node, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetChildren < 'ex > {
    fn new(surround_object: &'ex re_export::Node,) -> Self {
        let include_internal = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_internal: include_internal,
        }
    }
    #[inline]
    pub fn include_internal(self, include_internal: bool) -> Self {
        Self {
            include_internal: include_internal, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, include_internal,
        }
        = self;
        re_export::Node::get_children_full(surround_object, include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::get_child_ex`][super::Node::get_child_ex]."]
#[must_use]
pub struct ExGetChild < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Node, idx: i32, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetChild < 'ex > {
    fn new(surround_object: &'ex re_export::Node, idx: i32,) -> Self {
        let include_internal = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx, include_internal: include_internal,
        }
    }
    #[inline]
    pub fn include_internal(self, include_internal: bool) -> Self {
        Self {
            include_internal: include_internal, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, idx, include_internal,
        }
        = self;
        re_export::Node::get_child_full(surround_object, idx, include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::find_child_ex`][super::Node::find_child_ex]."]
#[must_use]
pub struct ExFindChild < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Node, pattern: CowArg < 'ex, GString >, recursive: bool, owned: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExFindChild < 'ex > {
    fn new(surround_object: &'ex re_export::Node, pattern: impl AsArg < GString > + 'ex,) -> Self {
        let recursive = true;
        let owned = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pattern: pattern.into_arg(), recursive: recursive, owned: owned,
        }
    }
    #[inline]
    pub fn recursive(self, recursive: bool) -> Self {
        Self {
            recursive: recursive, .. self
        }
    }
    #[inline]
    pub fn owned(self, owned: bool) -> Self {
        Self {
            owned: owned, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, pattern, recursive, owned,
        }
        = self;
        re_export::Node::find_child_full(surround_object, pattern, recursive, owned,)
    }
}
#[doc = "Default-param extender for [`Node::find_children_ex`][super::Node::find_children_ex]."]
#[must_use]
pub struct ExFindChildren < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Node, pattern: CowArg < 'ex, GString >, type_: CowArg < 'ex, GString >, recursive: bool, owned: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExFindChildren < 'ex > {
    fn new(surround_object: &'ex re_export::Node, pattern: impl AsArg < GString > + 'ex,) -> Self {
        let type_ = GString::from("");
        let recursive = true;
        let owned = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pattern: pattern.into_arg(), type_: CowArg::Owned(type_), recursive: recursive, owned: owned,
        }
    }
    #[inline]
    pub fn type_(self, type_: impl AsArg < GString > + 'ex) -> Self {
        Self {
            type_: type_.into_arg(), .. self
        }
    }
    #[inline]
    pub fn recursive(self, recursive: bool) -> Self {
        Self {
            recursive: recursive, .. self
        }
    }
    #[inline]
    pub fn owned(self, owned: bool) -> Self {
        Self {
            owned: owned, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, pattern, type_, recursive, owned,
        }
        = self;
        re_export::Node::find_children_full(surround_object, pattern, type_, recursive, owned,)
    }
}
#[doc = "Default-param extender for [`Node::get_path_to_ex`][super::Node::get_path_to_ex]."]
#[must_use]
pub struct ExGetPathTo < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Node, node: CowArg < 'ex, Gd < crate::classes::Node > >, use_unique_path: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetPathTo < 'ex > {
    fn new(surround_object: &'ex re_export::Node, node: impl AsArg < Gd < crate::classes::Node >> + 'ex,) -> Self {
        let use_unique_path = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.into_arg(), use_unique_path: use_unique_path,
        }
    }
    #[inline]
    pub fn use_unique_path(self, use_unique_path: bool) -> Self {
        Self {
            use_unique_path: use_unique_path, .. self
        }
    }
    #[inline]
    pub fn done(self) -> NodePath {
        let Self {
            _phantom, surround_object, node, use_unique_path,
        }
        = self;
        re_export::Node::get_path_to_full(surround_object, node, use_unique_path,)
    }
}
#[doc = "Default-param extender for [`Node::add_to_group_ex`][super::Node::add_to_group_ex]."]
#[must_use]
pub struct ExAddToGroup < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Node, group: CowArg < 'ex, StringName >, persistent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddToGroup < 'ex > {
    fn new(surround_object: &'ex mut re_export::Node, group: impl AsArg < StringName > + 'ex,) -> Self {
        let persistent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, group: group.into_arg(), persistent: persistent,
        }
    }
    #[inline]
    pub fn persistent(self, persistent: bool) -> Self {
        Self {
            persistent: persistent, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, group, persistent,
        }
        = self;
        re_export::Node::add_to_group_full(surround_object, group, persistent,)
    }
}
#[doc = "Default-param extender for [`Node::get_index_ex`][super::Node::get_index_ex]."]
#[must_use]
pub struct ExGetIndex < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Node, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetIndex < 'ex > {
    fn new(surround_object: &'ex re_export::Node,) -> Self {
        let include_internal = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_internal: include_internal,
        }
    }
    #[inline]
    pub fn include_internal(self, include_internal: bool) -> Self {
        Self {
            include_internal: include_internal, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, include_internal,
        }
        = self;
        re_export::Node::get_index_full(surround_object, include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::propagate_call_ex`][super::Node::propagate_call_ex]."]
#[must_use]
pub struct ExPropagateCall < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Node, method: CowArg < 'ex, StringName >, args: CowArg < 'ex, AnyArray >, parent_first: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPropagateCall < 'ex > {
    fn new(surround_object: &'ex mut re_export::Node, method: impl AsArg < StringName > + 'ex,) -> Self {
        let args = AnyArray::new_untyped();
        let parent_first = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, method: method.into_arg(), args: CowArg::Owned(args), parent_first: parent_first,
        }
    }
    #[inline]
    pub fn args(self, args: &'ex AnyArray) -> Self {
        Self {
            args: CowArg::Borrowed(args), .. self
        }
    }
    #[inline]
    pub fn parent_first(self, parent_first: bool) -> Self {
        Self {
            parent_first: parent_first, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, method, args, parent_first,
        }
        = self;
        re_export::Node::propagate_call_full(surround_object, method, args.cow_as_arg(), parent_first,)
    }
}
#[doc = "Default-param extender for [`Node::duplicate_ex`][super::Node::duplicate_ex]."]
#[must_use]
pub struct ExDuplicate < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Node, flags: crate::classes::node::DuplicateFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDuplicate < 'ex > {
    fn new(surround_object: &'ex re_export::Node,) -> Self {
        let flags = crate::obj::EngineBitfield::from_ord(15);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::node::DuplicateFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    #[expect(deprecated)]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, flags,
        }
        = self;
        re_export::Node::duplicate_full(surround_object, flags,)
    }
}
#[doc = "Default-param extender for [`Node::replace_by_ex`][super::Node::replace_by_ex]."]
#[must_use]
pub struct ExReplaceBy < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Node, node: CowArg < 'ex, Gd < crate::classes::Node > >, keep_groups: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExReplaceBy < 'ex > {
    fn new(surround_object: &'ex mut re_export::Node, node: impl AsArg < Gd < crate::classes::Node >> + 'ex,) -> Self {
        let keep_groups = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.into_arg(), keep_groups: keep_groups,
        }
    }
    #[inline]
    pub fn keep_groups(self, keep_groups: bool) -> Self {
        Self {
            keep_groups: keep_groups, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, node, keep_groups,
        }
        = self;
        re_export::Node::replace_by_full(surround_object, node, keep_groups,)
    }
}
#[doc = "Default-param extender for [`Node::set_multiplayer_authority_ex`][super::Node::set_multiplayer_authority_ex]."]
#[must_use]
pub struct ExSetMultiplayerAuthority < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Node, id: i32, recursive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetMultiplayerAuthority < 'ex > {
    fn new(surround_object: &'ex mut re_export::Node, id: i32,) -> Self {
        let recursive = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, recursive: recursive,
        }
    }
    #[inline]
    pub fn recursive(self, recursive: bool) -> Self {
        Self {
            recursive: recursive, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, recursive,
        }
        = self;
        re_export::Node::set_multiplayer_authority_full(surround_object, id, recursive,)
    }
}
#[doc = "Default-param extender for [`Node::atr_ex`][super::Node::atr_ex]."]
#[must_use]
pub struct ExAtr < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Node, message: CowArg < 'ex, GString >, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAtr < 'ex > {
    fn new(surround_object: &'ex re_export::Node, message: impl AsArg < GString > + 'ex,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, message, context,
        }
        = self;
        re_export::Node::atr_full(surround_object, message, context,)
    }
}
#[doc = "Default-param extender for [`Node::atr_n_ex`][super::Node::atr_n_ex]."]
#[must_use]
pub struct ExAtrN < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Node, message: CowArg < 'ex, GString >, plural_message: CowArg < 'ex, StringName >, n: i32, context: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAtrN < 'ex > {
    fn new(surround_object: &'ex re_export::Node, message: impl AsArg < GString > + 'ex, plural_message: impl AsArg < StringName > + 'ex, n: i32,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), plural_message: plural_message.into_arg(), n: n, context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, message, plural_message, n, context,
        }
        = self;
        re_export::Node::atr_n_full(surround_object, message, plural_message, n, context,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProcessMode {
    ord: i32
}
impl ProcessMode {
    #[doc(alias = "PROCESS_MODE_INHERIT")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_INHERIT`"]
    pub const INHERIT: ProcessMode = ProcessMode {
        ord: 0i32
    };
    #[doc(alias = "PROCESS_MODE_PAUSABLE")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_PAUSABLE`"]
    pub const PAUSABLE: ProcessMode = ProcessMode {
        ord: 1i32
    };
    #[doc(alias = "PROCESS_MODE_WHEN_PAUSED")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_WHEN_PAUSED`"]
    pub const WHEN_PAUSED: ProcessMode = ProcessMode {
        ord: 2i32
    };
    #[doc(alias = "PROCESS_MODE_ALWAYS")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_ALWAYS`"]
    pub const ALWAYS: ProcessMode = ProcessMode {
        ord: 3i32
    };
    #[doc(alias = "PROCESS_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_DISABLED`"]
    pub const DISABLED: ProcessMode = ProcessMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ProcessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ProcessMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ProcessMode {
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
            Self::INHERIT => "INHERIT", Self::PAUSABLE => "PAUSABLE", Self::WHEN_PAUSED => "WHEN_PAUSED", Self::ALWAYS => "ALWAYS", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ProcessMode::INHERIT, ProcessMode::PAUSABLE, ProcessMode::WHEN_PAUSED, ProcessMode::ALWAYS, ProcessMode::DISABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ProcessMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERIT", "PROCESS_MODE_INHERIT", ProcessMode::INHERIT), crate::meta::inspect::EnumConstant::new("PAUSABLE", "PROCESS_MODE_PAUSABLE", ProcessMode::PAUSABLE), crate::meta::inspect::EnumConstant::new("WHEN_PAUSED", "PROCESS_MODE_WHEN_PAUSED", ProcessMode::WHEN_PAUSED), crate::meta::inspect::EnumConstant::new("ALWAYS", "PROCESS_MODE_ALWAYS", ProcessMode::ALWAYS), crate::meta::inspect::EnumConstant::new("DISABLED", "PROCESS_MODE_DISABLED", ProcessMode::DISABLED)]
        }
    }
}
impl crate::meta::GodotConvert for ProcessMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Process Mode Inherit", 0i64), EnumeratorShape::new_int("Process Mode Pausable", 1i64), EnumeratorShape::new_int("Process Mode When Paused", 2i64), EnumeratorShape::new_int("Process Mode Always", 3i64), EnumeratorShape::new_int("Process Mode Disabled", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Node.ProcessMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ProcessMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProcessMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ProcessMode {
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
impl crate::registry::property::Export for ProcessMode {
    
}
impl crate::meta::Element for ProcessMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProcessThreadGroup {
    ord: i32
}
impl ProcessThreadGroup {
    #[doc(alias = "PROCESS_THREAD_GROUP_INHERIT")]
    #[doc = "Godot enumerator name: `PROCESS_THREAD_GROUP_INHERIT`"]
    pub const INHERIT: ProcessThreadGroup = ProcessThreadGroup {
        ord: 0i32
    };
    #[doc(alias = "PROCESS_THREAD_GROUP_MAIN_THREAD")]
    #[doc = "Godot enumerator name: `PROCESS_THREAD_GROUP_MAIN_THREAD`"]
    pub const MAIN_THREAD: ProcessThreadGroup = ProcessThreadGroup {
        ord: 1i32
    };
    #[doc(alias = "PROCESS_THREAD_GROUP_SUB_THREAD")]
    #[doc = "Godot enumerator name: `PROCESS_THREAD_GROUP_SUB_THREAD`"]
    pub const SUB_THREAD: ProcessThreadGroup = ProcessThreadGroup {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ProcessThreadGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ProcessThreadGroup") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ProcessThreadGroup {
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
            Self::INHERIT => "INHERIT", Self::MAIN_THREAD => "MAIN_THREAD", Self::SUB_THREAD => "SUB_THREAD", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ProcessThreadGroup::INHERIT, ProcessThreadGroup::MAIN_THREAD, ProcessThreadGroup::SUB_THREAD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ProcessThreadGroup >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERIT", "PROCESS_THREAD_GROUP_INHERIT", ProcessThreadGroup::INHERIT), crate::meta::inspect::EnumConstant::new("MAIN_THREAD", "PROCESS_THREAD_GROUP_MAIN_THREAD", ProcessThreadGroup::MAIN_THREAD), crate::meta::inspect::EnumConstant::new("SUB_THREAD", "PROCESS_THREAD_GROUP_SUB_THREAD", ProcessThreadGroup::SUB_THREAD)]
        }
    }
}
impl crate::meta::GodotConvert for ProcessThreadGroup {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Process Thread Group Inherit", 0i64), EnumeratorShape::new_int("Process Thread Group Main Thread", 1i64), EnumeratorShape::new_int("Process Thread Group Sub Thread", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Node.ProcessThreadGroup")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ProcessThreadGroup {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProcessThreadGroup {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ProcessThreadGroup {
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
impl crate::registry::property::Export for ProcessThreadGroup {
    
}
impl crate::meta::Element for ProcessThreadGroup {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ProcessThreadMessages {
    ord: u64
}
impl ProcessThreadMessages {
    #[doc(alias = "FLAG_PROCESS_THREAD_MESSAGES")]
    #[doc = "Godot enumerator name: `FLAG_PROCESS_THREAD_MESSAGES`"]
    pub const MESSAGES: ProcessThreadMessages = ProcessThreadMessages {
        ord: 1u64
    };
    #[doc(alias = "FLAG_PROCESS_THREAD_MESSAGES_PHYSICS")]
    #[doc = "Godot enumerator name: `FLAG_PROCESS_THREAD_MESSAGES_PHYSICS`"]
    pub const MESSAGES_PHYSICS: ProcessThreadMessages = ProcessThreadMessages {
        ord: 2u64
    };
    #[doc(alias = "FLAG_PROCESS_THREAD_MESSAGES_ALL")]
    #[doc = "Godot enumerator name: `FLAG_PROCESS_THREAD_MESSAGES_ALL`"]
    pub const MESSAGES_ALL: ProcessThreadMessages = ProcessThreadMessages {
        ord: 3u64
    };
    
}
impl std::fmt::Debug for ProcessThreadMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for ProcessThreadMessages {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ProcessThreadMessages >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MESSAGES", "FLAG_PROCESS_THREAD_MESSAGES", ProcessThreadMessages::MESSAGES), crate::meta::inspect::EnumConstant::new("MESSAGES_PHYSICS", "FLAG_PROCESS_THREAD_MESSAGES_PHYSICS", ProcessThreadMessages::MESSAGES_PHYSICS), crate::meta::inspect::EnumConstant::new("MESSAGES_ALL", "FLAG_PROCESS_THREAD_MESSAGES_ALL", ProcessThreadMessages::MESSAGES_ALL)]
        }
    }
}
impl std::ops::BitOr for ProcessThreadMessages {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for ProcessThreadMessages {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for ProcessThreadMessages {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Flag Process Thread Messages", 1i64), EnumeratorShape::new_int("Flag Process Thread Messages Physics", 2i64), EnumeratorShape::new_int("Flag Process Thread Messages All", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Node.ProcessThreadMessages")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for ProcessThreadMessages {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProcessThreadMessages {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ProcessThreadMessages {
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
impl crate::registry::property::Export for ProcessThreadMessages {
    
}
impl crate::meta::Element for ProcessThreadMessages {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PhysicsInterpolationMode {
    ord: i32
}
impl PhysicsInterpolationMode {
    #[doc(alias = "PHYSICS_INTERPOLATION_MODE_INHERIT")]
    #[doc = "Godot enumerator name: `PHYSICS_INTERPOLATION_MODE_INHERIT`"]
    pub const INHERIT: PhysicsInterpolationMode = PhysicsInterpolationMode {
        ord: 0i32
    };
    #[doc(alias = "PHYSICS_INTERPOLATION_MODE_ON")]
    #[doc = "Godot enumerator name: `PHYSICS_INTERPOLATION_MODE_ON`"]
    pub const ON: PhysicsInterpolationMode = PhysicsInterpolationMode {
        ord: 1i32
    };
    #[doc(alias = "PHYSICS_INTERPOLATION_MODE_OFF")]
    #[doc = "Godot enumerator name: `PHYSICS_INTERPOLATION_MODE_OFF`"]
    pub const OFF: PhysicsInterpolationMode = PhysicsInterpolationMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PhysicsInterpolationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PhysicsInterpolationMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PhysicsInterpolationMode {
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
            Self::INHERIT => "INHERIT", Self::ON => "ON", Self::OFF => "OFF", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PhysicsInterpolationMode::INHERIT, PhysicsInterpolationMode::ON, PhysicsInterpolationMode::OFF]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PhysicsInterpolationMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERIT", "PHYSICS_INTERPOLATION_MODE_INHERIT", PhysicsInterpolationMode::INHERIT), crate::meta::inspect::EnumConstant::new("ON", "PHYSICS_INTERPOLATION_MODE_ON", PhysicsInterpolationMode::ON), crate::meta::inspect::EnumConstant::new("OFF", "PHYSICS_INTERPOLATION_MODE_OFF", PhysicsInterpolationMode::OFF)]
        }
    }
}
impl crate::meta::GodotConvert for PhysicsInterpolationMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Physics Interpolation Mode Inherit", 0i64), EnumeratorShape::new_int("Physics Interpolation Mode On", 1i64), EnumeratorShape::new_int("Physics Interpolation Mode Off", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Node.PhysicsInterpolationMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PhysicsInterpolationMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PhysicsInterpolationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PhysicsInterpolationMode {
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
impl crate::registry::property::Export for PhysicsInterpolationMode {
    
}
impl crate::meta::Element for PhysicsInterpolationMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct DuplicateFlags {
    ord: u64
}
impl DuplicateFlags {
    #[doc(alias = "DUPLICATE_SIGNALS")]
    #[doc = "Godot enumerator name: `DUPLICATE_SIGNALS`"]
    pub const SIGNALS: DuplicateFlags = DuplicateFlags {
        ord: 1u64
    };
    #[doc(alias = "DUPLICATE_GROUPS")]
    #[doc = "Godot enumerator name: `DUPLICATE_GROUPS`"]
    pub const GROUPS: DuplicateFlags = DuplicateFlags {
        ord: 2u64
    };
    #[doc(alias = "DUPLICATE_SCRIPTS")]
    #[doc = "Godot enumerator name: `DUPLICATE_SCRIPTS`"]
    pub const SCRIPTS: DuplicateFlags = DuplicateFlags {
        ord: 4u64
    };
    #[doc(alias = "DUPLICATE_USE_INSTANTIATION")]
    #[doc = "Godot enumerator name: `DUPLICATE_USE_INSTANTIATION`"]
    pub const USE_INSTANTIATION: DuplicateFlags = DuplicateFlags {
        ord: 8u64
    };
    #[doc(alias = "DUPLICATE_INTERNAL_STATE")]
    #[doc = "Godot enumerator name: `DUPLICATE_INTERNAL_STATE`"]
    pub const INTERNAL_STATE: DuplicateFlags = DuplicateFlags {
        ord: 16u64
    };
    #[doc(alias = "DUPLICATE_DEFAULT")]
    #[doc = "Godot enumerator name: `DUPLICATE_DEFAULT`"]
    pub const DEFAULT: DuplicateFlags = DuplicateFlags {
        ord: 15u64
    };
    
}
impl std::fmt::Debug for DuplicateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for DuplicateFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DuplicateFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SIGNALS", "DUPLICATE_SIGNALS", DuplicateFlags::SIGNALS), crate::meta::inspect::EnumConstant::new("GROUPS", "DUPLICATE_GROUPS", DuplicateFlags::GROUPS), crate::meta::inspect::EnumConstant::new("SCRIPTS", "DUPLICATE_SCRIPTS", DuplicateFlags::SCRIPTS), crate::meta::inspect::EnumConstant::new("USE_INSTANTIATION", "DUPLICATE_USE_INSTANTIATION", DuplicateFlags::USE_INSTANTIATION), crate::meta::inspect::EnumConstant::new("INTERNAL_STATE", "DUPLICATE_INTERNAL_STATE", DuplicateFlags::INTERNAL_STATE), crate::meta::inspect::EnumConstant::new("DEFAULT", "DUPLICATE_DEFAULT", DuplicateFlags::DEFAULT)]
        }
    }
}
impl std::ops::BitOr for DuplicateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for DuplicateFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for DuplicateFlags {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Duplicate Signals", 1i64), EnumeratorShape::new_int("Duplicate Groups", 2i64), EnumeratorShape::new_int("Duplicate Scripts", 4i64), EnumeratorShape::new_int("Duplicate Use Instantiation", 8i64), EnumeratorShape::new_int("Duplicate Internal State", 16i64), EnumeratorShape::new_int("Duplicate Default", 15i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Node.DuplicateFlags")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for DuplicateFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DuplicateFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DuplicateFlags {
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
impl crate::registry::property::Export for DuplicateFlags {
    
}
impl crate::meta::Element for DuplicateFlags {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct InternalMode {
    ord: i32
}
impl InternalMode {
    #[doc(alias = "INTERNAL_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `INTERNAL_MODE_DISABLED`"]
    pub const DISABLED: InternalMode = InternalMode {
        ord: 0i32
    };
    #[doc(alias = "INTERNAL_MODE_FRONT")]
    #[doc = "Godot enumerator name: `INTERNAL_MODE_FRONT`"]
    pub const FRONT: InternalMode = InternalMode {
        ord: 1i32
    };
    #[doc(alias = "INTERNAL_MODE_BACK")]
    #[doc = "Godot enumerator name: `INTERNAL_MODE_BACK`"]
    pub const BACK: InternalMode = InternalMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for InternalMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("InternalMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for InternalMode {
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
            Self::DISABLED => "DISABLED", Self::FRONT => "FRONT", Self::BACK => "BACK", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[InternalMode::DISABLED, InternalMode::FRONT, InternalMode::BACK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < InternalMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "INTERNAL_MODE_DISABLED", InternalMode::DISABLED), crate::meta::inspect::EnumConstant::new("FRONT", "INTERNAL_MODE_FRONT", InternalMode::FRONT), crate::meta::inspect::EnumConstant::new("BACK", "INTERNAL_MODE_BACK", InternalMode::BACK)]
        }
    }
}
impl crate::meta::GodotConvert for InternalMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Internal Mode Disabled", 0i64), EnumeratorShape::new_int("Internal Mode Front", 1i64), EnumeratorShape::new_int("Internal Mode Back", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Node.InternalMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for InternalMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for InternalMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for InternalMode {
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
impl crate::registry::property::Export for InternalMode {
    
}
impl crate::meta::Element for InternalMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AutoTranslateMode {
    ord: i32
}
impl AutoTranslateMode {
    #[doc(alias = "AUTO_TRANSLATE_MODE_INHERIT")]
    #[doc = "Godot enumerator name: `AUTO_TRANSLATE_MODE_INHERIT`"]
    pub const INHERIT: AutoTranslateMode = AutoTranslateMode {
        ord: 0i32
    };
    #[doc(alias = "AUTO_TRANSLATE_MODE_ALWAYS")]
    #[doc = "Godot enumerator name: `AUTO_TRANSLATE_MODE_ALWAYS`"]
    pub const ALWAYS: AutoTranslateMode = AutoTranslateMode {
        ord: 1i32
    };
    #[doc(alias = "AUTO_TRANSLATE_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `AUTO_TRANSLATE_MODE_DISABLED`"]
    pub const DISABLED: AutoTranslateMode = AutoTranslateMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AutoTranslateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AutoTranslateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AutoTranslateMode {
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
            Self::INHERIT => "INHERIT", Self::ALWAYS => "ALWAYS", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AutoTranslateMode::INHERIT, AutoTranslateMode::ALWAYS, AutoTranslateMode::DISABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AutoTranslateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERIT", "AUTO_TRANSLATE_MODE_INHERIT", AutoTranslateMode::INHERIT), crate::meta::inspect::EnumConstant::new("ALWAYS", "AUTO_TRANSLATE_MODE_ALWAYS", AutoTranslateMode::ALWAYS), crate::meta::inspect::EnumConstant::new("DISABLED", "AUTO_TRANSLATE_MODE_DISABLED", AutoTranslateMode::DISABLED)]
        }
    }
}
impl crate::meta::GodotConvert for AutoTranslateMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Auto Translate Mode Inherit", 0i64), EnumeratorShape::new_int("Auto Translate Mode Always", 1i64), EnumeratorShape::new_int("Auto Translate Mode Disabled", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Node.AutoTranslateMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AutoTranslateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AutoTranslateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AutoTranslateMode {
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
impl crate::registry::property::Export for AutoTranslateMode {
    
}
impl crate::meta::Element for AutoTranslateMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Node;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Node`][crate::classes::Node] class."]
    pub struct SignalsOfNode < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfNode < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn ready(&mut self) -> SigReady < 'c, C > {
            SigReady {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "ready")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn renamed(&mut self) -> SigRenamed < 'c, C > {
            SigRenamed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "renamed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn tree_entered(&mut self) -> SigTreeEntered < 'c, C > {
            SigTreeEntered {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "tree_entered")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn tree_exiting(&mut self) -> SigTreeExiting < 'c, C > {
            SigTreeExiting {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "tree_exiting")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn tree_exited(&mut self) -> SigTreeExited < 'c, C > {
            SigTreeExited {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "tree_exited")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn child_entered_tree(&mut self) -> SigChildEnteredTree < 'c, C > {
            SigChildEnteredTree {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "child_entered_tree")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn child_exiting_tree(&mut self) -> SigChildExitingTree < 'c, C > {
            SigChildExitingTree {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "child_exiting_tree")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn child_order_changed(&mut self) -> SigChildOrderChanged < 'c, C > {
            SigChildOrderChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "child_order_changed")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn replacing_by(&mut self) -> SigReplacingBy < 'c, C > {
            SigReplacingBy {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "replacing_by")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn editor_description_changed(&mut self) -> SigEditorDescriptionChanged < 'c, C > {
            SigEditorDescriptionChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "editor_description_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn editor_state_changed(&mut self) -> SigEditorStateChanged < 'c, C > {
            SigEditorStateChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "editor_state_changed")
            }
        }
    }
    type TypedSigReady < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigReady < 'c, C: WithSignals > {
        typed: TypedSigReady < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigReady < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigReady < 'c, C > {
        type Target = TypedSigReady < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigReady < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigRenamed < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigRenamed < 'c, C: WithSignals > {
        typed: TypedSigRenamed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRenamed < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRenamed < 'c, C > {
        type Target = TypedSigRenamed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRenamed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTreeEntered < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTreeEntered < 'c, C: WithSignals > {
        typed: TypedSigTreeEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTreeEntered < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTreeEntered < 'c, C > {
        type Target = TypedSigTreeEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTreeEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTreeExiting < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTreeExiting < 'c, C: WithSignals > {
        typed: TypedSigTreeExiting < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTreeExiting < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTreeExiting < 'c, C > {
        type Target = TypedSigTreeExiting < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTreeExiting < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTreeExited < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTreeExited < 'c, C: WithSignals > {
        typed: TypedSigTreeExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTreeExited < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTreeExited < 'c, C > {
        type Target = TypedSigTreeExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTreeExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigChildEnteredTree < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigChildEnteredTree < 'c, C: WithSignals > {
        typed: TypedSigChildEnteredTree < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigChildEnteredTree < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigChildEnteredTree < 'c, C > {
        type Target = TypedSigChildEnteredTree < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigChildEnteredTree < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigChildExitingTree < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigChildExitingTree < 'c, C: WithSignals > {
        typed: TypedSigChildExitingTree < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigChildExitingTree < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigChildExitingTree < 'c, C > {
        type Target = TypedSigChildExitingTree < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigChildExitingTree < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigChildOrderChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigChildOrderChanged < 'c, C: WithSignals > {
        typed: TypedSigChildOrderChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigChildOrderChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigChildOrderChanged < 'c, C > {
        type Target = TypedSigChildOrderChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigChildOrderChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigReplacingBy < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigReplacingBy < 'c, C: WithSignals > {
        typed: TypedSigReplacingBy < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigReplacingBy < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigReplacingBy < 'c, C > {
        type Target = TypedSigReplacingBy < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigReplacingBy < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigEditorDescriptionChanged < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigEditorDescriptionChanged < 'c, C: WithSignals > {
        typed: TypedSigEditorDescriptionChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEditorDescriptionChanged < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigEditorDescriptionChanged < 'c, C > {
        type Target = TypedSigEditorDescriptionChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigEditorDescriptionChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigEditorStateChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigEditorStateChanged < 'c, C: WithSignals > {
        typed: TypedSigEditorStateChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEditorStateChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigEditorStateChanged < 'c, C > {
        type Target = TypedSigEditorStateChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigEditorStateChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Node {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfNode < 'c, C > {
        type Target = < < Node as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Node;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfNode < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Node;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
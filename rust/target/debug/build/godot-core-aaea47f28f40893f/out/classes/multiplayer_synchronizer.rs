#![doc = "Sidecar module for class [`MultiplayerSynchronizer`][crate::classes::MultiplayerSynchronizer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiplayerSynchronizer` enums](https://docs.godotengine.org/en/stable/classes/class_multiplayersynchronizer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiplayerSynchronizer`.\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`multiplayer_synchronizer`][crate::classes::multiplayer_synchronizer]: sidecar module with related enum/flag types\n* [`IMultiplayerSynchronizer`][crate::classes::IMultiplayerSynchronizer]: virtual methods\n* [`SignalsOfMultiplayerSynchronizer`][crate::classes::multiplayer_synchronizer::SignalsOfMultiplayerSynchronizer]: signal collection\n\n\nSee also [Godot docs for `MultiplayerSynchronizer`](https://docs.godotengine.org/en/stable/classes/class_multiplayersynchronizer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`MultiplayerSynchronizer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nBy default, `MultiplayerSynchronizer` synchronizes configured properties to all peers.\n\nVisibility can be handled directly with [`set_visibility_for`][`crate::classes::MultiplayerSynchronizer::set_visibility_for`] or as-needed with [`add_visibility_filter`][`crate::classes::MultiplayerSynchronizer::add_visibility_filter`] and [`update_visibility`][`crate::classes::MultiplayerSynchronizer::update_visibility`].\n\n[`MultiplayerSpawner`][crate::classes::MultiplayerSpawner]s will handle nodes according to visibility of synchronizers as long as the node at \\[member root_path] was spawned by one.\n\nInternally, `MultiplayerSynchronizer` uses [`object_configuration_add`][`crate::classes::MultiplayerApi::object_configuration_add`] to notify synchronization start passing the [`Node`][crate::classes::Node] at \\[member root_path] as the `object` and itself as the `configuration`, and uses [`object_configuration_remove`][`crate::classes::MultiplayerApi::object_configuration_remove`] to notify synchronization end in a similar way.\n\n**Note:** Synchronization is not supported for [`Object`][crate::classes::Object] type properties, like [`Resource`][crate::classes::Resource]. Properties that are unique to each peer, like the instance IDs of [`Object`][crate::classes::Object]s (see [`instance_id`][`crate::obj::Gd::instance_id`]) or [`RID`][crate::builtin::Rid]s, will also not work in synchronization."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiplayerSynchronizer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MultiplayerSynchronizer`][crate::classes::MultiplayerSynchronizer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `MultiplayerSynchronizer` methods](https://docs.godotengine.org/en/stable/classes/class_multiplayersynchronizer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMultiplayerSynchronizer: crate::obj::GodotClass < Base = MultiplayerSynchronizer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MultiplayerSynchronizer {
        pub fn set_root_path(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_root_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_path(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "get_root_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_replication_interval(&mut self, milliseconds: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (milliseconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_replication_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_replication_interval(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "get_replication_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_delta_interval(&mut self, milliseconds: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (milliseconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_delta_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_delta_interval(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "get_delta_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_replication_config(&mut self, config: impl AsArg < Option < Gd < crate::classes::SceneReplicationConfig >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::SceneReplicationConfig > > >,);
            let args = (config.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_replication_config", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_replication_config(&self,) -> Option < Gd < crate::classes::SceneReplicationConfig > > {
            type CallRet = Option < Gd < crate::classes::SceneReplicationConfig > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "get_replication_config", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_update_mode(&mut self, mode: crate::classes::multiplayer_synchronizer::VisibilityUpdateMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::multiplayer_synchronizer::VisibilityUpdateMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6009usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_visibility_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_update_mode(&self,) -> crate::classes::multiplayer_synchronizer::VisibilityUpdateMode {
            type CallRet = crate::classes::multiplayer_synchronizer::VisibilityUpdateMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6010usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "get_visibility_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Updates the visibility of `for_peer` according to visibility filters. If `for_peer` is `0` (the default), all peers' visibilties are updated."]
        pub(crate) fn update_visibility_full(&mut self, for_peer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (for_peer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6011usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "update_visibility", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`update_visibility_ex`][Self::update_visibility_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Updates the visibility of `for_peer` according to visibility filters. If `for_peer` is `0` (the default), all peers' visibilties are updated."]
        #[inline]
        pub fn update_visibility(&mut self,) {
            self.update_visibility_ex() . done()
        }
        #[doc = "Updates the visibility of `for_peer` according to visibility filters. If `for_peer` is `0` (the default), all peers' visibilties are updated."]
        #[inline]
        pub fn update_visibility_ex < 'ex > (&'ex mut self,) -> ExUpdateVisibility < 'ex > {
            ExUpdateVisibility::new(self,)
        }
        pub fn set_visibility_public(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_visibility_public", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_visibility_public(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6013usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "is_visibility_public", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a peer visibility filter for this synchronizer.\n\n`filter` should take a peer ID `int` and return a `bool`."]
        pub fn add_visibility_filter(&mut self, filter: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(filter),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6014usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "add_visibility_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a peer visibility filter from this synchronizer."]
        pub fn remove_visibility_filter(&mut self, filter: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(filter),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6015usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "remove_visibility_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the visibility of `peer` to `visible`. If `peer` is `0`, the value of \\[member public_visibility] will be updated instead."]
        pub fn set_visibility_for(&mut self, peer: i32, visible: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (peer, visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_visibility_for", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Queries the current visibility for peer `peer`."]
        pub fn get_visibility_for(&self, peer: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (peer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "get_visibility_for", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for MultiplayerSynchronizer {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MultiplayerSynchronizer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiplayerSynchronizer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for MultiplayerSynchronizer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MultiplayerSynchronizer {
        
    }
    impl crate::obj::cap::GodotDefault for MultiplayerSynchronizer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MultiplayerSynchronizer {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiplayerSynchronizer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MultiplayerSynchronizer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MultiplayerSynchronizer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MultiplayerSynchronizer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`MultiplayerSynchronizer::update_visibility_ex`][super::MultiplayerSynchronizer::update_visibility_ex]."]
#[must_use]
pub struct ExUpdateVisibility < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::MultiplayerSynchronizer, for_peer: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExUpdateVisibility < 'ex > {
    fn new(surround_object: &'ex mut re_export::MultiplayerSynchronizer,) -> Self {
        let for_peer = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, for_peer: for_peer,
        }
    }
    #[inline]
    pub fn for_peer(self, for_peer: i32) -> Self {
        Self {
            for_peer: for_peer, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, for_peer,
        }
        = self;
        re_export::MultiplayerSynchronizer::update_visibility_full(surround_object, for_peer,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibilityUpdateMode {
    ord: i32
}
impl VisibilityUpdateMode {
    #[doc(alias = "VISIBILITY_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `VISIBILITY_PROCESS_IDLE`"]
    pub const IDLE: VisibilityUpdateMode = VisibilityUpdateMode {
        ord: 0i32
    };
    #[doc(alias = "VISIBILITY_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `VISIBILITY_PROCESS_PHYSICS`"]
    pub const PHYSICS: VisibilityUpdateMode = VisibilityUpdateMode {
        ord: 1i32
    };
    #[doc(alias = "VISIBILITY_PROCESS_NONE")]
    #[doc = "Godot enumerator name: `VISIBILITY_PROCESS_NONE`"]
    pub const NONE: VisibilityUpdateMode = VisibilityUpdateMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for VisibilityUpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibilityUpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibilityUpdateMode {
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
            Self::IDLE => "IDLE", Self::PHYSICS => "PHYSICS", Self::NONE => "NONE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VisibilityUpdateMode::IDLE, VisibilityUpdateMode::PHYSICS, VisibilityUpdateMode::NONE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VisibilityUpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IDLE", "VISIBILITY_PROCESS_IDLE", VisibilityUpdateMode::IDLE), crate::meta::inspect::EnumConstant::new("PHYSICS", "VISIBILITY_PROCESS_PHYSICS", VisibilityUpdateMode::PHYSICS), crate::meta::inspect::EnumConstant::new("NONE", "VISIBILITY_PROCESS_NONE", VisibilityUpdateMode::NONE)]
        }
    }
}
impl crate::meta::GodotConvert for VisibilityUpdateMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Visibility Process Idle", 0i64), EnumeratorShape::new_int("Visibility Process Physics", 1i64), EnumeratorShape::new_int("Visibility Process None", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("MultiplayerSynchronizer.VisibilityUpdateMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for VisibilityUpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibilityUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for VisibilityUpdateMode {
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
impl crate::registry::property::Export for VisibilityUpdateMode {
    
}
impl crate::meta::Element for VisibilityUpdateMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MultiplayerSynchronizer;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`MultiplayerSynchronizer`][crate::classes::MultiplayerSynchronizer] class."]
    pub struct SignalsOfMultiplayerSynchronizer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfMultiplayerSynchronizer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn synchronized(&mut self) -> SigSynchronized < 'c, C > {
            SigSynchronized {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "synchronized")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn delta_synchronized(&mut self) -> SigDeltaSynchronized < 'c, C > {
            SigDeltaSynchronized {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "delta_synchronized")
            }
        }
        #[doc = "Signature: `(for_peer: i64)`"]
        pub fn visibility_changed(&mut self) -> SigVisibilityChanged < 'c, C > {
            SigVisibilityChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "visibility_changed")
            }
        }
    }
    type TypedSigSynchronized < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSynchronized < 'c, C: WithSignals > {
        typed: TypedSigSynchronized < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSynchronized < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSynchronized < 'c, C > {
        type Target = TypedSigSynchronized < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSynchronized < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigDeltaSynchronized < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigDeltaSynchronized < 'c, C: WithSignals > {
        typed: TypedSigDeltaSynchronized < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDeltaSynchronized < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDeltaSynchronized < 'c, C > {
        type Target = TypedSigDeltaSynchronized < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDeltaSynchronized < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigVisibilityChanged < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigVisibilityChanged < 'c, C: WithSignals > {
        typed: TypedSigVisibilityChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigVisibilityChanged < 'c, C > {
        pub fn emit(&mut self, for_peer: i64,) {
            self.typed.emit_tuple((for_peer,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigVisibilityChanged < 'c, C > {
        type Target = TypedSigVisibilityChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigVisibilityChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for MultiplayerSynchronizer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfMultiplayerSynchronizer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfMultiplayerSynchronizer < 'c, C > {
        type Target = < < MultiplayerSynchronizer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = MultiplayerSynchronizer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfMultiplayerSynchronizer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = MultiplayerSynchronizer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
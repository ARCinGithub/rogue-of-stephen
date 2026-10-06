#![doc = "Sidecar module for class [`TwoBoneIk3d`][crate::classes::TwoBoneIk3d].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TwoBoneIK3D` enums](https://docs.godotengine.org/en/stable/classes/class_twoboneik3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TwoBoneIK3D`.\n\nInherits [`IkModifier3D`][crate::classes::IkModifier3D].\n\nRelated symbols:\n\n* [`ITwoBoneIk3d`][crate::classes::ITwoBoneIk3d]: virtual methods\n\n\nSee also [Godot docs for `TwoBoneIK3D`](https://docs.godotengine.org/en/stable/classes/class_twoboneik3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TwoBoneIk3d::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nThis [`IKModifier3D`][crate::classes::IkModifier3D] requires a pole target. It provides deterministic results by constructing a plane from each joint and pole target and finding the intersection of two circles (disks in 3D).\n\nThis IK can handle twist by setting the pole direction. If there are more than one bone between each set bone, their rotations are ignored, and the straight line connecting the root-middle and middle-end joints are treated as virtual bones."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TwoBoneIk3d {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TwoBoneIk3d`][crate::classes::TwoBoneIk3d].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IIkModifier3D`][crate::classes::IIkModifier3D] > [`ISkeletonModifier3D`][crate::classes::ISkeletonModifier3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `TwoBoneIK3D` methods](https://docs.godotengine.org/en/stable/classes/class_twoboneik3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITwoBoneIk3d: crate::obj::GodotClass < Base = TwoBoneIk3d > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Override this virtual method to implement a custom skeleton modifier. You should do things like get the [`Skeleton3D`][crate::classes::Skeleton3D]'s current pose and apply the pose here.\n\n[`process_modification_with_delta`][`crate::classes::ISkeletonModifier3D::process_modification_with_delta`] must not apply \\[member influence] to bone poses because the [`Skeleton3D`][crate::classes::Skeleton3D] automatically applies influence to all bone poses set by the modifier.\n\n`delta` is passed from parent [`Skeleton3D`][crate::classes::Skeleton3D]. See also [`advance`][`crate::classes::Skeleton3D::advance`].\n\n**Note:** This method may be called outside [`process`][`crate::classes::INode::process`] and [`physics_process`][`crate::classes::INode::physics_process`] with `delta` is `0.0`, since the modification should be processed immediately after initialization of the [`Skeleton3D`][crate::classes::Skeleton3D]."]
        fn process_modification_with_delta(&mut self, delta: f64,) {
            unimplemented !()
        }
        #[doc = "Override this virtual method to implement a custom skeleton modifier. You should do things like get the [`Skeleton3D`][crate::classes::Skeleton3D]'s current pose and apply the pose here.\n\n[`process_modification`][`crate::classes::ISkeletonModifier3D::process_modification`] must not apply \\[member influence] to bone poses because the [`Skeleton3D`][crate::classes::Skeleton3D] automatically applies influence to all bone poses set by the modifier."]
        fn process_modification(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called when the skeleton is changed."]
        fn skeleton_changed(&mut self, old_skeleton: Option < Gd < crate::classes::Skeleton3D > >, new_skeleton: Option < Gd < crate::classes::Skeleton3D > >,) {
            unimplemented !()
        }
        #[doc = "Called when bone names and indices need to be validated, such as when entering the scene tree or changing skeleton."]
        fn validate_bone_names(&mut self,) {
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
    impl TwoBoneIk3d {
        #[doc = "Sets the target node that the end bone is trying to reach."]
        pub fn set_target_node(&mut self, index: i32, target_node: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, NodePath >,);
            let args = (index, target_node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_target_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the target node that the end bone is trying to reach."]
        pub fn get_target_node(&self, index: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "get_target_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the pole target node that constructs a plane which the joints are all on and the pole is trying to direct."]
        pub fn set_pole_node(&mut self, index: i32, pole_node: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, NodePath >,);
            let args = (index, pole_node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_pole_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the pole target node that constructs a plane which the joints are all on and the pole is trying to direct."]
        pub fn get_pole_node(&self, index: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "get_pole_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the root bone name."]
        pub fn set_root_bone_name(&mut self, index: i32, bone_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (index, bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_root_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the root bone name."]
        pub fn get_root_bone_name(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "get_root_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the root bone index."]
        pub fn set_root_bone(&mut self, index: i32, bone: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_root_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the root bone index."]
        pub fn get_root_bone(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "get_root_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the middle bone name.\n\n**Note:** The middle bone must be a child of the root bone."]
        pub fn set_middle_bone_name(&mut self, index: i32, bone_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (index, bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_middle_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the middle bone name."]
        pub fn get_middle_bone_name(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "get_middle_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the middle bone index."]
        pub fn set_middle_bone(&mut self, index: i32, bone: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_middle_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the middle bone index."]
        pub fn get_middle_bone(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "get_middle_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the pole direction.\n\nThe pole is on the middle bone and will direct to the pole target.\n\nThe rotation axis is a vector that is orthogonal to this and the forward vector.\n\n**Note:** The pole direction and the forward vector shouldn't be colinear to avoid unintended rotation."]
        pub fn set_pole_direction(&mut self, index: i32, direction: crate::classes::skeleton_modifier_3d::SecondaryDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::skeleton_modifier_3d::SecondaryDirection,);
            let args = (index, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_pole_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the pole direction."]
        pub fn get_pole_direction(&self, index: i32,) -> crate::classes::skeleton_modifier_3d::SecondaryDirection {
            type CallRet = crate::classes::skeleton_modifier_3d::SecondaryDirection;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "get_pole_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the pole direction vector.\n\nThis vector is normalized by an internal process.\n\nIf the vector length is `0`, it is considered synonymous with [`SecondaryDirection::NONE`][`crate::classes::skeleton_modifier_3d::SecondaryDirection::NONE`]."]
        pub fn set_pole_direction_vector(&mut self, index: i32, vector: Vector3,) {
            type CallRet = ();
            type CallParams = (i32, Vector3,);
            let args = (index, vector,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_pole_direction_vector", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the pole direction vector.\n\nIf [`get_pole_direction`][`crate::classes::TwoBoneIk3d::get_pole_direction`] is [`SecondaryDirection::NONE`][`crate::classes::skeleton_modifier_3d::SecondaryDirection::NONE`], this method returns `Vector3(0, 0, 0)`."]
        pub fn get_pole_direction_vector(&self, index: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "get_pole_direction_vector", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the end bone name.\n\n**Note:** The end bone must be a child of the middle bone."]
        pub fn set_end_bone_name(&mut self, index: i32, bone_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (index, bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_end_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the end bone name."]
        pub fn get_end_bone_name(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "get_end_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the end bone index."]
        pub fn set_end_bone(&mut self, index: i32, bone: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_end_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the end bone index."]
        pub fn get_end_bone(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "get_end_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `enabled` is `true`, the end bone is extended from the middle bone as a virtual bone."]
        pub fn set_use_virtual_end(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_use_virtual_end", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the end bone is extended from the middle bone as a virtual bone."]
        pub fn is_using_virtual_end(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "is_using_virtual_end", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `enabled` is `true`, the end bone is extended to have a tail."]
        pub fn set_extend_end_bone(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_extend_end_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the end bone is extended to have a tail."]
        pub fn is_end_bone_extended(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "is_end_bone_extended", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the end bone tail direction when [`is_end_bone_extended`][`crate::classes::TwoBoneIk3d::is_end_bone_extended`] is `true`."]
        pub fn set_end_bone_direction(&mut self, index: i32, bone_direction: crate::classes::skeleton_modifier_3d::BoneDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::skeleton_modifier_3d::BoneDirection,);
            let args = (index, bone_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_end_bone_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the end bone's tail direction when [`is_end_bone_extended`][`crate::classes::TwoBoneIk3d::is_end_bone_extended`] is `true`."]
        pub fn get_end_bone_direction(&self, index: i32,) -> crate::classes::skeleton_modifier_3d::BoneDirection {
            type CallRet = crate::classes::skeleton_modifier_3d::BoneDirection;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "get_end_bone_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the end bone tail length when [`is_end_bone_extended`][`crate::classes::TwoBoneIk3d::is_end_bone_extended`] is `true`."]
        pub fn set_end_bone_length(&mut self, index: i32, length: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (index, length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "set_end_bone_length", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the end bone tail length of the bone chain when [`is_end_bone_extended`][`crate::classes::TwoBoneIk3d::is_end_bone_extended`] is `true`."]
        pub fn get_end_bone_length(&self, index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TwoBoneIk3d", "get_end_bone_length", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TwoBoneIk3d {
        type Base = crate::classes::IkModifier3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TwoBoneIK3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TwoBoneIk3d {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::IkModifier3D > for TwoBoneIk3d {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::SkeletonModifier3D > for TwoBoneIk3d {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for TwoBoneIk3d {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TwoBoneIk3d {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TwoBoneIk3d {
        
    }
    impl crate::obj::cap::GodotDefault for TwoBoneIk3d {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TwoBoneIk3d {
        type Target = crate::classes::IkModifier3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TwoBoneIk3d {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TwoBoneIk3d`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TwoBoneIk3d__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TwoBoneIk3d > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::IkModifier3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::SkeletonModifier3D > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TwoBoneIk3d;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::skeleton_modifier_3d::SignalsOfSkeletonModifier3D;
    impl WithSignals for TwoBoneIk3d {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfSkeletonModifier3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
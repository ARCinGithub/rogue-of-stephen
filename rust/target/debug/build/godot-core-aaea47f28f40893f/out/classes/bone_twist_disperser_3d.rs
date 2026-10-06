#![doc = "Sidecar module for class [`BoneTwistDisperser3D`][crate::classes::BoneTwistDisperser3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `BoneTwistDisperser3D` enums](https://docs.godotengine.org/en/stable/classes/class_bonetwistdisperser3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `BoneTwistDisperser3D`.\n\nInherits [`SkeletonModifier3D`][crate::classes::SkeletonModifier3D].\n\nRelated symbols:\n\n* [`bone_twist_disperser_3d`][crate::classes::bone_twist_disperser_3d]: sidecar module with related enum/flag types\n* [`IBoneTwistDisperser3D`][crate::classes::IBoneTwistDisperser3D]: virtual methods\n\n\nSee also [Godot docs for `BoneTwistDisperser3D`](https://docs.godotengine.org/en/stable/classes/class_bonetwistdisperser3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`BoneTwistDisperser3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nThis `BoneTwistDisperser3D` allows for smooth twist interpolation between multiple bones by dispersing the end bone's twist to the parents. This only changes the twist without changing the global position of each joint.\n\nThis is useful for smoothly twisting bones in combination with [`CopyTransformModifier3D`][crate::classes::CopyTransformModifier3D] and IK.\n\n**Note:** If an extracted twist is greater than 180 degrees, flipping occurs. This is similar to [`ConvertTransformModifier3D`][crate::classes::ConvertTransformModifier3D]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct BoneTwistDisperser3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`BoneTwistDisperser3D`][crate::classes::BoneTwistDisperser3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ISkeletonModifier3D`][crate::classes::ISkeletonModifier3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `BoneTwistDisperser3D` methods](https://docs.godotengine.org/en/stable/classes/class_bonetwistdisperser3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IBoneTwistDisperser3D: crate::obj::GodotClass < Base = BoneTwistDisperser3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl BoneTwistDisperser3D {
        pub fn set_setting_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_setting_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_setting_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_setting_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all settings."]
        pub fn clear_settings(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "clear_settings", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mutable_bone_axes(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_mutable_bone_axes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_bone_axes_mutable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "are_bone_axes_mutable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the root bone name of the bone chain."]
        pub fn set_root_bone_name(&mut self, index: i32, bone_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (index, bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_root_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the root bone name of the bone chain."]
        pub fn get_root_bone_name(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_root_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the root bone index of the bone chain."]
        pub fn set_root_bone(&mut self, index: i32, bone: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_root_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the root bone index of the bone chain."]
        pub fn get_root_bone(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_root_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the end bone name of the bone chain.\n\n**Note:** The end bone must be a child of the root bone."]
        pub fn set_end_bone_name(&mut self, index: i32, bone_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (index, bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_end_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the end bone name of the bone chain."]
        pub fn get_end_bone_name(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_end_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the end bone index of the bone chain."]
        pub fn set_end_bone(&mut self, index: i32, bone: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_end_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the end bone index of the bone chain."]
        pub fn get_end_bone(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_end_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the reference bone name to extract twist of the setting at `index`.\n\nThis bone is either the end of the chain or its parent, depending on [`is_end_bone_extended`][`crate::classes::BoneTwistDisperser3D::is_end_bone_extended`]."]
        pub fn get_reference_bone_name(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_reference_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the reference bone to extract twist of the setting at `index`.\n\nThis bone is either the end of the chain or its parent, depending on [`is_end_bone_extended`][`crate::classes::BoneTwistDisperser3D::is_end_bone_extended`]."]
        pub fn get_reference_bone(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_reference_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `enabled` is `true`, the end bone is extended to have a tail.\n\nIf `enabled` is `false`, [`get_reference_bone`][`crate::classes::BoneTwistDisperser3D::get_reference_bone`] becomes a parent of the end bone and it uses the vector to the end bone as a twist axis."]
        pub fn set_extend_end_bone(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_extend_end_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the end bone is extended to have a tail."]
        pub fn is_end_bone_extended(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "is_end_bone_extended", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the end bone tail direction of the bone chain when [`is_end_bone_extended`][`crate::classes::BoneTwistDisperser3D::is_end_bone_extended`] is `true`."]
        pub fn set_end_bone_direction(&mut self, index: i32, bone_direction: crate::classes::skeleton_modifier_3d::BoneDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::skeleton_modifier_3d::BoneDirection,);
            let args = (index, bone_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_end_bone_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tail direction of the end bone of the bone chain when [`is_end_bone_extended`][`crate::classes::BoneTwistDisperser3D::is_end_bone_extended`] is `true`."]
        pub fn get_end_bone_direction(&self, index: i32,) -> crate::classes::skeleton_modifier_3d::BoneDirection {
            type CallRet = crate::classes::skeleton_modifier_3d::BoneDirection;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_end_bone_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `enabled` is `true`, it extracts the twist amount from the difference between the bone rest and the current bone pose.\n\nIf `enabled` is `false`, it extracts the twist amount from the difference between [`get_twist_from`][`crate::classes::BoneTwistDisperser3D::get_twist_from`] and the current bone pose. See also [`set_twist_from`][`crate::classes::BoneTwistDisperser3D::set_twist_from`]."]
        pub fn set_twist_from_rest(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_twist_from_rest", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if extracting the twist amount from the difference between the bone rest and the current bone pose."]
        pub fn is_twist_from_rest(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "is_twist_from_rest", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the rotation to an arbitrary state before twisting for the current bone pose to extract the twist when [`is_twist_from_rest`][`crate::classes::BoneTwistDisperser3D::is_twist_from_rest`] is `false`.\n\nIn other words, by calling [`set_twist_from`][`crate::classes::BoneTwistDisperser3D::set_twist_from`] by `SkeletonModifier3D.modification_processed` of a specific [`SkeletonModifier3D`][crate::classes::SkeletonModifier3D], you can extract only the twists generated by modifiers processed after that but before this `BoneTwistDisperser3D`."]
        pub fn set_twist_from(&mut self, index: i32, from: Quaternion,) {
            type CallRet = ();
            type CallParams = (i32, Quaternion,);
            let args = (index, from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_twist_from", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the rotation to an arbitrary state before twisting for the current bone pose to extract the twist when [`is_twist_from_rest`][`crate::classes::BoneTwistDisperser3D::is_twist_from_rest`] is `false`."]
        pub fn get_twist_from(&self, index: i32,) -> Quaternion {
            type CallRet = Quaternion;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_twist_from", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether to use automatic amount assignment or to allow manual assignment."]
        pub fn set_disperse_mode(&mut self, index: i32, disperse_mode: crate::classes::bone_twist_disperser_3d::DisperseMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::bone_twist_disperser_3d::DisperseMode,);
            let args = (index, disperse_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_disperse_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether to use automatic amount assignment or to allow manual assignment."]
        pub fn get_disperse_mode(&self, index: i32,) -> crate::classes::bone_twist_disperser_3d::DisperseMode {
            type CallRet = crate::classes::bone_twist_disperser_3d::DisperseMode;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_disperse_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the position at which to divide the segment between joints for weight assignment when [`get_disperse_mode`][`crate::classes::BoneTwistDisperser3D::get_disperse_mode`] is [`DisperseMode::WEIGHTED`][`crate::classes::bone_twist_disperser_3d::DisperseMode::WEIGHTED`].\n\nFor example, when `weight_position` is `0.5`, if two bone segments with a length of `1.0` exist between three joints, weights are assigned to each joint from root to end at ratios of `0.5`, `1.0`, and `0.5`. Then amounts become `0.25`, `0.75`, and `1.0` respectively."]
        pub fn set_weight_position(&mut self, index: i32, weight_position: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (index, weight_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_weight_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the position at which to divide the segment between joints for weight assignment when [`get_disperse_mode`][`crate::classes::BoneTwistDisperser3D::get_disperse_mode`] is [`DisperseMode::WEIGHTED`][`crate::classes::bone_twist_disperser_3d::DisperseMode::WEIGHTED`]."]
        pub fn get_weight_position(&self, index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_weight_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the damping curve when [`get_disperse_mode`][`crate::classes::BoneTwistDisperser3D::get_disperse_mode`] is [`DisperseMode::CUSTOM`][`crate::classes::bone_twist_disperser_3d::DisperseMode::CUSTOM`]."]
        pub fn set_damping_curve(&mut self, index: i32, curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (index, curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_damping_curve", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the damping curve when [`get_disperse_mode`][`crate::classes::BoneTwistDisperser3D::get_disperse_mode`] is [`DisperseMode::CUSTOM`][`crate::classes::bone_twist_disperser_3d::DisperseMode::CUSTOM`]."]
        pub fn get_damping_curve(&self, index: i32,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_damping_curve", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the bone name at `joint` in the bone chain's joint list."]
        pub fn get_joint_bone_name(&self, index: i32, joint: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (index, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_joint_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the bone index at `joint` in the bone chain's joint list."]
        pub fn get_joint_bone(&self, index: i32, joint: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (index, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_joint_bone", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the twist amount at `joint` in the bone chain's joint list when [`get_disperse_mode`][`crate::classes::BoneTwistDisperser3D::get_disperse_mode`] is [`DisperseMode::CUSTOM`][`crate::classes::bone_twist_disperser_3d::DisperseMode::CUSTOM`]."]
        pub fn get_joint_twist_amount(&self, index: i32, joint: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (index, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_joint_twist_amount", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the twist amount at `joint` in the bone chain's joint list when [`get_disperse_mode`][`crate::classes::BoneTwistDisperser3D::get_disperse_mode`] is [`DisperseMode::CUSTOM`][`crate::classes::bone_twist_disperser_3d::DisperseMode::CUSTOM`]."]
        pub fn set_joint_twist_amount(&mut self, index: i32, joint: i32, twist_amount: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (index, joint, twist_amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "set_joint_twist_amount", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the joint count of the bone chain's joint list."]
        pub fn get_joint_count(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BoneTwistDisperser3D", "get_joint_count", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for BoneTwistDisperser3D {
        type Base = crate::classes::SkeletonModifier3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("BoneTwistDisperser3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for BoneTwistDisperser3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::SkeletonModifier3D > for BoneTwistDisperser3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for BoneTwistDisperser3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for BoneTwistDisperser3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for BoneTwistDisperser3D {
        
    }
    impl crate::obj::cap::GodotDefault for BoneTwistDisperser3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for BoneTwistDisperser3D {
        type Target = crate::classes::SkeletonModifier3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for BoneTwistDisperser3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`BoneTwistDisperser3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_BoneTwistDisperser3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::BoneTwistDisperser3D > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DisperseMode {
    ord: i32
}
impl DisperseMode {
    #[doc(alias = "DISPERSE_MODE_EVEN")]
    #[doc = "Godot enumerator name: `DISPERSE_MODE_EVEN`"]
    pub const EVEN: DisperseMode = DisperseMode {
        ord: 0i32
    };
    #[doc(alias = "DISPERSE_MODE_WEIGHTED")]
    #[doc = "Godot enumerator name: `DISPERSE_MODE_WEIGHTED`"]
    pub const WEIGHTED: DisperseMode = DisperseMode {
        ord: 1i32
    };
    #[doc(alias = "DISPERSE_MODE_CUSTOM")]
    #[doc = "Godot enumerator name: `DISPERSE_MODE_CUSTOM`"]
    pub const CUSTOM: DisperseMode = DisperseMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DisperseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DisperseMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DisperseMode {
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
            Self::EVEN => "EVEN", Self::WEIGHTED => "WEIGHTED", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DisperseMode::EVEN, DisperseMode::WEIGHTED, DisperseMode::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DisperseMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("EVEN", "DISPERSE_MODE_EVEN", DisperseMode::EVEN), crate::meta::inspect::EnumConstant::new("WEIGHTED", "DISPERSE_MODE_WEIGHTED", DisperseMode::WEIGHTED), crate::meta::inspect::EnumConstant::new("CUSTOM", "DISPERSE_MODE_CUSTOM", DisperseMode::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for DisperseMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Disperse Mode Even", 0i64), EnumeratorShape::new_int("Disperse Mode Weighted", 1i64), EnumeratorShape::new_int("Disperse Mode Custom", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("BoneTwistDisperser3D.DisperseMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DisperseMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DisperseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DisperseMode {
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
impl crate::registry::property::Export for DisperseMode {
    
}
impl crate::meta::Element for DisperseMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::BoneTwistDisperser3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::skeleton_modifier_3d::SignalsOfSkeletonModifier3D;
    impl WithSignals for BoneTwistDisperser3D {
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
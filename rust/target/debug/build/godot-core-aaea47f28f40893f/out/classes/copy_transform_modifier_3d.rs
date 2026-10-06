#![doc = "Sidecar module for class [`CopyTransformModifier3D`][crate::classes::CopyTransformModifier3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CopyTransformModifier3D` enums](https://docs.godotengine.org/en/stable/classes/class_copytransformmodifier3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CopyTransformModifier3D`.\n\nInherits [`BoneConstraint3D`][crate::classes::BoneConstraint3D].\n\nRelated symbols:\n\n* [`copy_transform_modifier_3d`][crate::classes::copy_transform_modifier_3d]: sidecar module with related enum/flag types\n* [`ICopyTransformModifier3D`][crate::classes::ICopyTransformModifier3D]: virtual methods\n\n\nSee also [Godot docs for `CopyTransformModifier3D`](https://docs.godotengine.org/en/stable/classes/class_copytransformmodifier3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CopyTransformModifier3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nApply the copied transform of the bone set by [`set_reference_bone`][`crate::classes::BoneConstraint3D::set_reference_bone`] to the bone set by [`set_apply_bone`][`crate::classes::BoneConstraint3D::set_apply_bone`] with processing it with some masks and options.\n\nThere are 4 ways to apply the transform, depending on the combination of [`set_relative`][`crate::classes::CopyTransformModifier3D::set_relative`] and [`set_additive`][`crate::classes::CopyTransformModifier3D::set_additive`].\n\n**Relative + Additive:**\n\n- Extract reference pose relative to the rest and add it to the apply bone's pose.\n\n**Relative + Not Additive:**\n\n- Extract reference pose relative to the rest and add it to the apply bone's rest.\n\n**Not Relative + Additive:**\n\n- Extract reference pose absolutely and add it to the apply bone's pose.\n\n**Not Relative + Not Additive:**\n\n- Extract reference pose absolutely and the apply bone's pose is replaced with it.\n\n**Note:** Relative option is available only in the case [`get_reference_type`][`crate::classes::BoneConstraint3D::get_reference_type`] is [`ReferenceType::BONE`][`crate::classes::bone_constraint_3d::ReferenceType::BONE`]. See also \\[enum BoneConstraint3D.ReferenceType]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CopyTransformModifier3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CopyTransformModifier3D`][crate::classes::CopyTransformModifier3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IBoneConstraint3D`][crate::classes::IBoneConstraint3D] > [`ISkeletonModifier3D`][crate::classes::ISkeletonModifier3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `CopyTransformModifier3D` methods](https://docs.godotengine.org/en/stable/classes/class_copytransformmodifier3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICopyTransformModifier3D: crate::obj::GodotClass < Base = CopyTransformModifier3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CopyTransformModifier3D {
        #[doc = "Sets the flags to process the transform operations. If the flag is valid, the transform operation is processed.\n\n**Note:** If the rotation is valid for only one axis, it respects the roll of the valid axis. If the rotation is valid for two axes, it discards the roll of the invalid axis."]
        pub fn set_copy_flags(&mut self, index: i32, copy_flags: crate::classes::copy_transform_modifier_3d::TransformFlag,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::copy_transform_modifier_3d::TransformFlag,);
            let args = (index, copy_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_copy_flags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the copy flags of the setting at `index`."]
        pub fn get_copy_flags(&self, index: i32,) -> crate::classes::copy_transform_modifier_3d::TransformFlag {
            type CallRet = crate::classes::copy_transform_modifier_3d::TransformFlag;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "get_copy_flags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the flags to copy axes. If the flag is valid, the axis is copied."]
        pub fn set_axis_flags(&mut self, index: i32, axis_flags: crate::classes::copy_transform_modifier_3d::AxisFlag,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::copy_transform_modifier_3d::AxisFlag,);
            let args = (index, axis_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_flags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the axis flags of the setting at `index`."]
        pub fn get_axis_flags(&self, index: i32,) -> crate::classes::copy_transform_modifier_3d::AxisFlag {
            type CallRet = crate::classes::copy_transform_modifier_3d::AxisFlag;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "get_axis_flags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the flags to inverte axes. If the flag is valid, the axis is copied.\n\n**Note:** An inverted scale means an inverse number, not a negative scale. For example, inverting `2.0` means `0.5`.\n\n**Note:** An inverted rotation flips the elements of the quaternion. For example, a two-axis inversion will flip the roll of each axis, and a three-axis inversion will flip the final orientation. However, be aware that flipping only one axis may cause unintended rotation by the unflipped axes, due to the characteristics of the quaternion."]
        pub fn set_invert_flags(&mut self, index: i32, axis_flags: crate::classes::copy_transform_modifier_3d::AxisFlag,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::copy_transform_modifier_3d::AxisFlag,);
            let args = (index, axis_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_invert_flags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the invert flags of the setting at `index`."]
        pub fn get_invert_flags(&self, index: i32,) -> crate::classes::copy_transform_modifier_3d::AxisFlag {
            type CallRet = crate::classes::copy_transform_modifier_3d::AxisFlag;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "get_invert_flags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If sets `enabled` to `true`, the position will be copied."]
        pub fn set_copy_position(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_copy_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the copy flags has the flag for the position in the setting at `index`. See also [`set_copy_flags`][`crate::classes::CopyTransformModifier3D::set_copy_flags`]."]
        pub fn is_position_copying(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_position_copying", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If sets `enabled` to `true`, the rotation will be copied."]
        pub fn set_copy_rotation(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_copy_rotation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the copy flags has the flag for the rotation in the setting at `index`. See also [`set_copy_flags`][`crate::classes::CopyTransformModifier3D::set_copy_flags`]."]
        pub fn is_rotation_copying(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_rotation_copying", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If sets `enabled` to `true`, the scale will be copied."]
        pub fn set_copy_scale(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_copy_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the copy flags has the flag for the scale in the setting at `index`. See also [`set_copy_flags`][`crate::classes::CopyTransformModifier3D::set_copy_flags`]."]
        pub fn is_scale_copying(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_scale_copying", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If sets `enabled` to `true`, the X-axis will be copied."]
        pub fn set_axis_x_enabled(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_x_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the enable flags has the flag for the X-axis in the setting at `index`. See also [`set_axis_flags`][`crate::classes::CopyTransformModifier3D::set_axis_flags`]."]
        pub fn is_axis_x_enabled(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_axis_x_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If sets `enabled` to `true`, the Y-axis will be copied."]
        pub fn set_axis_y_enabled(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_y_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the enable flags has the flag for the Y-axis in the setting at `index`. See also [`set_axis_flags`][`crate::classes::CopyTransformModifier3D::set_axis_flags`]."]
        pub fn is_axis_y_enabled(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_axis_y_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If sets `enabled` to `true`, the Z-axis will be copied."]
        pub fn set_axis_z_enabled(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_z_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the enable flags has the flag for the Z-axis in the setting at `index`. See also [`set_axis_flags`][`crate::classes::CopyTransformModifier3D::set_axis_flags`]."]
        pub fn is_axis_z_enabled(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_axis_z_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If sets `enabled` to `true`, the X-axis will be inverted."]
        pub fn set_axis_x_inverted(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_x_inverted", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the invert flags has the flag for the X-axis in the setting at `index`. See also [`set_invert_flags`][`crate::classes::CopyTransformModifier3D::set_invert_flags`]."]
        pub fn is_axis_x_inverted(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_axis_x_inverted", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If sets `enabled` to `true`, the Y-axis will be inverted."]
        pub fn set_axis_y_inverted(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_y_inverted", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the invert flags has the flag for the Y-axis in the setting at `index`. See also [`set_invert_flags`][`crate::classes::CopyTransformModifier3D::set_invert_flags`]."]
        pub fn is_axis_y_inverted(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_axis_y_inverted", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If sets `enabled` to `true`, the Z-axis will be inverted."]
        pub fn set_axis_z_inverted(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_z_inverted", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the invert flags has the flag for the Z-axis in the setting at `index`. See also [`set_invert_flags`][`crate::classes::CopyTransformModifier3D::set_invert_flags`]."]
        pub fn is_axis_z_inverted(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_axis_z_inverted", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets relative option in the setting at `index` to `enabled`.\n\nIf sets `enabled` to `true`, the extracted and applying transform is relative to the rest.\n\nIf sets `enabled` to `false`, the extracted transform is absolute."]
        pub fn set_relative(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_relative", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the relative option is enabled in the setting at `index`."]
        pub fn is_relative(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_relative", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets additive option in the setting at `index` to `enabled`. This mainly affects the process of applying transform to the [`set_apply_bone`][`crate::classes::BoneConstraint3D::set_apply_bone`].\n\nIf sets `enabled` to `true`, the processed transform is added to the pose of the current apply bone.\n\nIf sets `enabled` to `false`, the pose of the current apply bone is replaced with the processed transform. However, if set [`set_relative`][`crate::classes::CopyTransformModifier3D::set_relative`] to `true`, the transform is relative to rest."]
        pub fn set_additive(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_additive", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the additive option is enabled in the setting at `index`."]
        pub fn is_additive(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_additive", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CopyTransformModifier3D {
        type Base = crate::classes::BoneConstraint3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CopyTransformModifier3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CopyTransformModifier3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BoneConstraint3D > for CopyTransformModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::SkeletonModifier3D > for CopyTransformModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for CopyTransformModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CopyTransformModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CopyTransformModifier3D {
        
    }
    impl crate::obj::cap::GodotDefault for CopyTransformModifier3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CopyTransformModifier3D {
        type Target = crate::classes::BoneConstraint3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CopyTransformModifier3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CopyTransformModifier3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CopyTransformModifier3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CopyTransformModifier3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::BoneConstraint3D > for $Class {
                
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
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct TransformFlag {
    ord: u64
}
impl TransformFlag {
    #[doc(alias = "TRANSFORM_FLAG_POSITION")]
    #[doc = "Godot enumerator name: `TRANSFORM_FLAG_POSITION`"]
    pub const POSITION: TransformFlag = TransformFlag {
        ord: 1u64
    };
    #[doc(alias = "TRANSFORM_FLAG_ROTATION")]
    #[doc = "Godot enumerator name: `TRANSFORM_FLAG_ROTATION`"]
    pub const ROTATION: TransformFlag = TransformFlag {
        ord: 2u64
    };
    #[doc(alias = "TRANSFORM_FLAG_SCALE")]
    #[doc = "Godot enumerator name: `TRANSFORM_FLAG_SCALE`"]
    pub const SCALE: TransformFlag = TransformFlag {
        ord: 4u64
    };
    #[doc(alias = "TRANSFORM_FLAG_ALL")]
    #[doc = "Godot enumerator name: `TRANSFORM_FLAG_ALL`"]
    pub const ALL: TransformFlag = TransformFlag {
        ord: 7u64
    };
    
}
impl std::fmt::Debug for TransformFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for TransformFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TransformFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POSITION", "TRANSFORM_FLAG_POSITION", TransformFlag::POSITION), crate::meta::inspect::EnumConstant::new("ROTATION", "TRANSFORM_FLAG_ROTATION", TransformFlag::ROTATION), crate::meta::inspect::EnumConstant::new("SCALE", "TRANSFORM_FLAG_SCALE", TransformFlag::SCALE), crate::meta::inspect::EnumConstant::new("ALL", "TRANSFORM_FLAG_ALL", TransformFlag::ALL)]
        }
    }
}
impl std::ops::BitOr for TransformFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for TransformFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for TransformFlag {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Transform Flag Position", 1i64), EnumeratorShape::new_int("Transform Flag Rotation", 2i64), EnumeratorShape::new_int("Transform Flag Scale", 4i64), EnumeratorShape::new_int("Transform Flag All", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CopyTransformModifier3D.TransformFlag")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for TransformFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransformFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TransformFlag {
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
impl crate::registry::property::Export for TransformFlag {
    
}
impl crate::meta::Element for TransformFlag {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct AxisFlag {
    ord: u64
}
impl AxisFlag {
    #[doc(alias = "AXIS_FLAG_X")]
    #[doc = "Godot enumerator name: `AXIS_FLAG_X`"]
    pub const X: AxisFlag = AxisFlag {
        ord: 1u64
    };
    #[doc(alias = "AXIS_FLAG_Y")]
    #[doc = "Godot enumerator name: `AXIS_FLAG_Y`"]
    pub const Y: AxisFlag = AxisFlag {
        ord: 2u64
    };
    #[doc(alias = "AXIS_FLAG_Z")]
    #[doc = "Godot enumerator name: `AXIS_FLAG_Z`"]
    pub const Z: AxisFlag = AxisFlag {
        ord: 4u64
    };
    #[doc(alias = "AXIS_FLAG_ALL")]
    #[doc = "Godot enumerator name: `AXIS_FLAG_ALL`"]
    pub const ALL: AxisFlag = AxisFlag {
        ord: 7u64
    };
    
}
impl std::fmt::Debug for AxisFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for AxisFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AxisFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("X", "AXIS_FLAG_X", AxisFlag::X), crate::meta::inspect::EnumConstant::new("Y", "AXIS_FLAG_Y", AxisFlag::Y), crate::meta::inspect::EnumConstant::new("Z", "AXIS_FLAG_Z", AxisFlag::Z), crate::meta::inspect::EnumConstant::new("ALL", "AXIS_FLAG_ALL", AxisFlag::ALL)]
        }
    }
}
impl std::ops::BitOr for AxisFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for AxisFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for AxisFlag {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Axis Flag X", 1i64), EnumeratorShape::new_int("Axis Flag Y", 2i64), EnumeratorShape::new_int("Axis Flag Z", 4i64), EnumeratorShape::new_int("Axis Flag All", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CopyTransformModifier3D.AxisFlag")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for AxisFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AxisFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AxisFlag {
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
impl crate::registry::property::Export for AxisFlag {
    
}
impl crate::meta::Element for AxisFlag {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CopyTransformModifier3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::skeleton_modifier_3d::SignalsOfSkeletonModifier3D;
    impl WithSignals for CopyTransformModifier3D {
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
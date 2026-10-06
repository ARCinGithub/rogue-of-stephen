#![doc = "Sidecar module for class [`SkeletonModifier3D`][crate::classes::SkeletonModifier3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SkeletonModifier3D` enums](https://docs.godotengine.org/en/stable/classes/class_skeletonmodifier3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SkeletonModifier3D`.\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`skeleton_modifier_3d`][crate::classes::skeleton_modifier_3d]: sidecar module with related enum/flag types\n* [`ISkeletonModifier3D`][crate::classes::ISkeletonModifier3D]: virtual methods\n* [`SignalsOfSkeletonModifier3D`][crate::classes::skeleton_modifier_3d::SignalsOfSkeletonModifier3D]: signal collection\n\n\nSee also [Godot docs for `SkeletonModifier3D`](https://docs.godotengine.org/en/stable/classes/class_skeletonmodifier3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`SkeletonModifier3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\n`SkeletonModifier3D` retrieves a target [`Skeleton3D`][crate::classes::Skeleton3D] by having a [`Skeleton3D`][crate::classes::Skeleton3D] parent.\n\nIf there is an [`AnimationMixer`][crate::classes::AnimationMixer], a modification always performs after playback process of the [`AnimationMixer`][crate::classes::AnimationMixer].\n\nThis node should be used to implement custom IK solvers, constraints, or skeleton physics."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SkeletonModifier3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`SkeletonModifier3D`][crate::classes::SkeletonModifier3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `SkeletonModifier3D` methods](https://docs.godotengine.org/en/stable/classes/class_skeletonmodifier3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISkeletonModifier3D: crate::obj::GodotClass < Base = SkeletonModifier3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SkeletonModifier3D {
        #[doc = "Returns the parent [`Skeleton3D`][crate::classes::Skeleton3D] node if it exists. Otherwise, returns `null`."]
        pub fn get_skeleton(&self,) -> Option < Gd < crate::classes::Skeleton3D > > {
            type CallRet = Option < Gd < crate::classes::Skeleton3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonModifier3D", "get_skeleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_active(&mut self, active: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonModifier3D", "set_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_active(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonModifier3D", "is_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_influence(&mut self, influence: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (influence,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonModifier3D", "set_influence", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_influence(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonModifier3D", "get_influence", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SkeletonModifier3D {
        type Base = crate::classes::Node3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SkeletonModifier3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SkeletonModifier3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for SkeletonModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for SkeletonModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SkeletonModifier3D {
        
    }
    impl crate::obj::cap::GodotDefault for SkeletonModifier3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SkeletonModifier3D {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SkeletonModifier3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SkeletonModifier3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SkeletonModifier3D__ensure_class_exists {
        ($Class: ident) => {
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
pub struct BoneAxis {
    ord: i32
}
impl BoneAxis {
    #[doc(alias = "BONE_AXIS_PLUS_X")]
    #[doc = "Godot enumerator name: `BONE_AXIS_PLUS_X`"]
    pub const PLUS_X: BoneAxis = BoneAxis {
        ord: 0i32
    };
    #[doc(alias = "BONE_AXIS_MINUS_X")]
    #[doc = "Godot enumerator name: `BONE_AXIS_MINUS_X`"]
    pub const MINUS_X: BoneAxis = BoneAxis {
        ord: 1i32
    };
    #[doc(alias = "BONE_AXIS_PLUS_Y")]
    #[doc = "Godot enumerator name: `BONE_AXIS_PLUS_Y`"]
    pub const PLUS_Y: BoneAxis = BoneAxis {
        ord: 2i32
    };
    #[doc(alias = "BONE_AXIS_MINUS_Y")]
    #[doc = "Godot enumerator name: `BONE_AXIS_MINUS_Y`"]
    pub const MINUS_Y: BoneAxis = BoneAxis {
        ord: 3i32
    };
    #[doc(alias = "BONE_AXIS_PLUS_Z")]
    #[doc = "Godot enumerator name: `BONE_AXIS_PLUS_Z`"]
    pub const PLUS_Z: BoneAxis = BoneAxis {
        ord: 4i32
    };
    #[doc(alias = "BONE_AXIS_MINUS_Z")]
    #[doc = "Godot enumerator name: `BONE_AXIS_MINUS_Z`"]
    pub const MINUS_Z: BoneAxis = BoneAxis {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for BoneAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BoneAxis") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BoneAxis {
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
            Self::PLUS_X => "PLUS_X", Self::MINUS_X => "MINUS_X", Self::PLUS_Y => "PLUS_Y", Self::MINUS_Y => "MINUS_Y", Self::PLUS_Z => "PLUS_Z", Self::MINUS_Z => "MINUS_Z", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BoneAxis::PLUS_X, BoneAxis::MINUS_X, BoneAxis::PLUS_Y, BoneAxis::MINUS_Y, BoneAxis::PLUS_Z, BoneAxis::MINUS_Z]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BoneAxis >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PLUS_X", "BONE_AXIS_PLUS_X", BoneAxis::PLUS_X), crate::meta::inspect::EnumConstant::new("MINUS_X", "BONE_AXIS_MINUS_X", BoneAxis::MINUS_X), crate::meta::inspect::EnumConstant::new("PLUS_Y", "BONE_AXIS_PLUS_Y", BoneAxis::PLUS_Y), crate::meta::inspect::EnumConstant::new("MINUS_Y", "BONE_AXIS_MINUS_Y", BoneAxis::MINUS_Y), crate::meta::inspect::EnumConstant::new("PLUS_Z", "BONE_AXIS_PLUS_Z", BoneAxis::PLUS_Z), crate::meta::inspect::EnumConstant::new("MINUS_Z", "BONE_AXIS_MINUS_Z", BoneAxis::MINUS_Z)]
        }
    }
}
impl crate::meta::GodotConvert for BoneAxis {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Bone Axis Plus X", 0i64), EnumeratorShape::new_int("Bone Axis Minus X", 1i64), EnumeratorShape::new_int("Bone Axis Plus Y", 2i64), EnumeratorShape::new_int("Bone Axis Minus Y", 3i64), EnumeratorShape::new_int("Bone Axis Plus Z", 4i64), EnumeratorShape::new_int("Bone Axis Minus Z", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("SkeletonModifier3D.BoneAxis")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BoneAxis {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BoneAxis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BoneAxis {
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
impl crate::registry::property::Export for BoneAxis {
    
}
impl crate::meta::Element for BoneAxis {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BoneDirection {
    ord: i32
}
impl BoneDirection {
    #[doc(alias = "BONE_DIRECTION_PLUS_X")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_PLUS_X`"]
    pub const PLUS_X: BoneDirection = BoneDirection {
        ord: 0i32
    };
    #[doc(alias = "BONE_DIRECTION_MINUS_X")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_MINUS_X`"]
    pub const MINUS_X: BoneDirection = BoneDirection {
        ord: 1i32
    };
    #[doc(alias = "BONE_DIRECTION_PLUS_Y")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_PLUS_Y`"]
    pub const PLUS_Y: BoneDirection = BoneDirection {
        ord: 2i32
    };
    #[doc(alias = "BONE_DIRECTION_MINUS_Y")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_MINUS_Y`"]
    pub const MINUS_Y: BoneDirection = BoneDirection {
        ord: 3i32
    };
    #[doc(alias = "BONE_DIRECTION_PLUS_Z")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_PLUS_Z`"]
    pub const PLUS_Z: BoneDirection = BoneDirection {
        ord: 4i32
    };
    #[doc(alias = "BONE_DIRECTION_MINUS_Z")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_MINUS_Z`"]
    pub const MINUS_Z: BoneDirection = BoneDirection {
        ord: 5i32
    };
    #[doc(alias = "BONE_DIRECTION_FROM_PARENT")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_FROM_PARENT`"]
    pub const FROM_PARENT: BoneDirection = BoneDirection {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for BoneDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BoneDirection") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BoneDirection {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::PLUS_X => "PLUS_X", Self::MINUS_X => "MINUS_X", Self::PLUS_Y => "PLUS_Y", Self::MINUS_Y => "MINUS_Y", Self::PLUS_Z => "PLUS_Z", Self::MINUS_Z => "MINUS_Z", Self::FROM_PARENT => "FROM_PARENT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BoneDirection::PLUS_X, BoneDirection::MINUS_X, BoneDirection::PLUS_Y, BoneDirection::MINUS_Y, BoneDirection::PLUS_Z, BoneDirection::MINUS_Z, BoneDirection::FROM_PARENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BoneDirection >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PLUS_X", "BONE_DIRECTION_PLUS_X", BoneDirection::PLUS_X), crate::meta::inspect::EnumConstant::new("MINUS_X", "BONE_DIRECTION_MINUS_X", BoneDirection::MINUS_X), crate::meta::inspect::EnumConstant::new("PLUS_Y", "BONE_DIRECTION_PLUS_Y", BoneDirection::PLUS_Y), crate::meta::inspect::EnumConstant::new("MINUS_Y", "BONE_DIRECTION_MINUS_Y", BoneDirection::MINUS_Y), crate::meta::inspect::EnumConstant::new("PLUS_Z", "BONE_DIRECTION_PLUS_Z", BoneDirection::PLUS_Z), crate::meta::inspect::EnumConstant::new("MINUS_Z", "BONE_DIRECTION_MINUS_Z", BoneDirection::MINUS_Z), crate::meta::inspect::EnumConstant::new("FROM_PARENT", "BONE_DIRECTION_FROM_PARENT", BoneDirection::FROM_PARENT)]
        }
    }
}
impl crate::meta::GodotConvert for BoneDirection {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Bone Direction Plus X", 0i64), EnumeratorShape::new_int("Bone Direction Minus X", 1i64), EnumeratorShape::new_int("Bone Direction Plus Y", 2i64), EnumeratorShape::new_int("Bone Direction Minus Y", 3i64), EnumeratorShape::new_int("Bone Direction Plus Z", 4i64), EnumeratorShape::new_int("Bone Direction Minus Z", 5i64), EnumeratorShape::new_int("Bone Direction From Parent", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("SkeletonModifier3D.BoneDirection")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BoneDirection {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BoneDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BoneDirection {
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
impl crate::registry::property::Export for BoneDirection {
    
}
impl crate::meta::Element for BoneDirection {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SecondaryDirection {
    ord: i32
}
impl SecondaryDirection {
    #[doc(alias = "SECONDARY_DIRECTION_NONE")]
    #[doc = "Godot enumerator name: `SECONDARY_DIRECTION_NONE`"]
    pub const NONE: SecondaryDirection = SecondaryDirection {
        ord: 0i32
    };
    #[doc(alias = "SECONDARY_DIRECTION_PLUS_X")]
    #[doc = "Godot enumerator name: `SECONDARY_DIRECTION_PLUS_X`"]
    pub const PLUS_X: SecondaryDirection = SecondaryDirection {
        ord: 1i32
    };
    #[doc(alias = "SECONDARY_DIRECTION_MINUS_X")]
    #[doc = "Godot enumerator name: `SECONDARY_DIRECTION_MINUS_X`"]
    pub const MINUS_X: SecondaryDirection = SecondaryDirection {
        ord: 2i32
    };
    #[doc(alias = "SECONDARY_DIRECTION_PLUS_Y")]
    #[doc = "Godot enumerator name: `SECONDARY_DIRECTION_PLUS_Y`"]
    pub const PLUS_Y: SecondaryDirection = SecondaryDirection {
        ord: 3i32
    };
    #[doc(alias = "SECONDARY_DIRECTION_MINUS_Y")]
    #[doc = "Godot enumerator name: `SECONDARY_DIRECTION_MINUS_Y`"]
    pub const MINUS_Y: SecondaryDirection = SecondaryDirection {
        ord: 4i32
    };
    #[doc(alias = "SECONDARY_DIRECTION_PLUS_Z")]
    #[doc = "Godot enumerator name: `SECONDARY_DIRECTION_PLUS_Z`"]
    pub const PLUS_Z: SecondaryDirection = SecondaryDirection {
        ord: 5i32
    };
    #[doc(alias = "SECONDARY_DIRECTION_MINUS_Z")]
    #[doc = "Godot enumerator name: `SECONDARY_DIRECTION_MINUS_Z`"]
    pub const MINUS_Z: SecondaryDirection = SecondaryDirection {
        ord: 6i32
    };
    #[doc(alias = "SECONDARY_DIRECTION_CUSTOM")]
    #[doc = "Godot enumerator name: `SECONDARY_DIRECTION_CUSTOM`"]
    pub const CUSTOM: SecondaryDirection = SecondaryDirection {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for SecondaryDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SecondaryDirection") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SecondaryDirection {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::NONE => "NONE", Self::PLUS_X => "PLUS_X", Self::MINUS_X => "MINUS_X", Self::PLUS_Y => "PLUS_Y", Self::MINUS_Y => "MINUS_Y", Self::PLUS_Z => "PLUS_Z", Self::MINUS_Z => "MINUS_Z", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SecondaryDirection::NONE, SecondaryDirection::PLUS_X, SecondaryDirection::MINUS_X, SecondaryDirection::PLUS_Y, SecondaryDirection::MINUS_Y, SecondaryDirection::PLUS_Z, SecondaryDirection::MINUS_Z, SecondaryDirection::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SecondaryDirection >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "SECONDARY_DIRECTION_NONE", SecondaryDirection::NONE), crate::meta::inspect::EnumConstant::new("PLUS_X", "SECONDARY_DIRECTION_PLUS_X", SecondaryDirection::PLUS_X), crate::meta::inspect::EnumConstant::new("MINUS_X", "SECONDARY_DIRECTION_MINUS_X", SecondaryDirection::MINUS_X), crate::meta::inspect::EnumConstant::new("PLUS_Y", "SECONDARY_DIRECTION_PLUS_Y", SecondaryDirection::PLUS_Y), crate::meta::inspect::EnumConstant::new("MINUS_Y", "SECONDARY_DIRECTION_MINUS_Y", SecondaryDirection::MINUS_Y), crate::meta::inspect::EnumConstant::new("PLUS_Z", "SECONDARY_DIRECTION_PLUS_Z", SecondaryDirection::PLUS_Z), crate::meta::inspect::EnumConstant::new("MINUS_Z", "SECONDARY_DIRECTION_MINUS_Z", SecondaryDirection::MINUS_Z), crate::meta::inspect::EnumConstant::new("CUSTOM", "SECONDARY_DIRECTION_CUSTOM", SecondaryDirection::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for SecondaryDirection {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Secondary Direction None", 0i64), EnumeratorShape::new_int("Secondary Direction Plus X", 1i64), EnumeratorShape::new_int("Secondary Direction Minus X", 2i64), EnumeratorShape::new_int("Secondary Direction Plus Y", 3i64), EnumeratorShape::new_int("Secondary Direction Minus Y", 4i64), EnumeratorShape::new_int("Secondary Direction Plus Z", 5i64), EnumeratorShape::new_int("Secondary Direction Minus Z", 6i64), EnumeratorShape::new_int("Secondary Direction Custom", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("SkeletonModifier3D.SecondaryDirection")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SecondaryDirection {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SecondaryDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SecondaryDirection {
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
impl crate::registry::property::Export for SecondaryDirection {
    
}
impl crate::meta::Element for SecondaryDirection {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RotationAxis {
    ord: i32
}
impl RotationAxis {
    #[doc(alias = "ROTATION_AXIS_X")]
    #[doc = "Godot enumerator name: `ROTATION_AXIS_X`"]
    pub const X: RotationAxis = RotationAxis {
        ord: 0i32
    };
    #[doc(alias = "ROTATION_AXIS_Y")]
    #[doc = "Godot enumerator name: `ROTATION_AXIS_Y`"]
    pub const Y: RotationAxis = RotationAxis {
        ord: 1i32
    };
    #[doc(alias = "ROTATION_AXIS_Z")]
    #[doc = "Godot enumerator name: `ROTATION_AXIS_Z`"]
    pub const Z: RotationAxis = RotationAxis {
        ord: 2i32
    };
    #[doc(alias = "ROTATION_AXIS_ALL")]
    #[doc = "Godot enumerator name: `ROTATION_AXIS_ALL`"]
    pub const ALL: RotationAxis = RotationAxis {
        ord: 3i32
    };
    #[doc(alias = "ROTATION_AXIS_CUSTOM")]
    #[doc = "Godot enumerator name: `ROTATION_AXIS_CUSTOM`"]
    pub const CUSTOM: RotationAxis = RotationAxis {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for RotationAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RotationAxis") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RotationAxis {
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
            Self::X => "X", Self::Y => "Y", Self::Z => "Z", Self::ALL => "ALL", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RotationAxis::X, RotationAxis::Y, RotationAxis::Z, RotationAxis::ALL, RotationAxis::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RotationAxis >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("X", "ROTATION_AXIS_X", RotationAxis::X), crate::meta::inspect::EnumConstant::new("Y", "ROTATION_AXIS_Y", RotationAxis::Y), crate::meta::inspect::EnumConstant::new("Z", "ROTATION_AXIS_Z", RotationAxis::Z), crate::meta::inspect::EnumConstant::new("ALL", "ROTATION_AXIS_ALL", RotationAxis::ALL), crate::meta::inspect::EnumConstant::new("CUSTOM", "ROTATION_AXIS_CUSTOM", RotationAxis::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for RotationAxis {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Rotation Axis X", 0i64), EnumeratorShape::new_int("Rotation Axis Y", 1i64), EnumeratorShape::new_int("Rotation Axis Z", 2i64), EnumeratorShape::new_int("Rotation Axis All", 3i64), EnumeratorShape::new_int("Rotation Axis Custom", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("SkeletonModifier3D.RotationAxis")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for RotationAxis {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RotationAxis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for RotationAxis {
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
impl crate::registry::property::Export for RotationAxis {
    
}
impl crate::meta::Element for RotationAxis {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SkeletonModifier3D;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`SkeletonModifier3D`][crate::classes::SkeletonModifier3D] class."]
    pub struct SignalsOfSkeletonModifier3D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfSkeletonModifier3D < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn modification_processed(&mut self) -> SigModificationProcessed < 'c, C > {
            SigModificationProcessed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "modification_processed")
            }
        }
    }
    type TypedSigModificationProcessed < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigModificationProcessed < 'c, C: WithSignals > {
        typed: TypedSigModificationProcessed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigModificationProcessed < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigModificationProcessed < 'c, C > {
        type Target = TypedSigModificationProcessed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigModificationProcessed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for SkeletonModifier3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfSkeletonModifier3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfSkeletonModifier3D < 'c, C > {
        type Target = < < SkeletonModifier3D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = SkeletonModifier3D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfSkeletonModifier3D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = SkeletonModifier3D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
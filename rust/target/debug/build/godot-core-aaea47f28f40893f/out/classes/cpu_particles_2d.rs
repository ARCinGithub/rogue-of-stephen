#![doc = "Sidecar module for class [`CpuParticles2D`][crate::classes::CpuParticles2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CPUParticles2D` enums](https://docs.godotengine.org/en/stable/classes/class_cpuparticles2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CPUParticles2D`.\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`cpu_particles_2d`][crate::classes::cpu_particles_2d]: sidecar module with related enum/flag types\n* [`ICpuParticles2D`][crate::classes::ICpuParticles2D]: virtual methods\n* [`SignalsOfCpuParticles2D`][crate::classes::cpu_particles_2d::SignalsOfCpuParticles2D]: signal collection\n\n\nSee also [Godot docs for `CPUParticles2D`](https://docs.godotengine.org/en/stable/classes/class_cpuparticles2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CpuParticles2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nCPU-based 2D particle node used to create a variety of particle systems and effects.\n\nSee also [`GPUParticles2D`][crate::classes::GpuParticles2D], which provides the same functionality with hardware acceleration, but may not run on older devices."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CpuParticles2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CpuParticles2D`][crate::classes::CpuParticles2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `CPUParticles2D` methods](https://docs.godotengine.org/en/stable/classes/class_cpuparticles2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICpuParticles2D: crate::obj::GodotClass < Base = CpuParticles2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CpuParticles2D {
        pub fn set_emitting(&mut self, emitting: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (emitting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emitting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_amount(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_amount", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lifetime(&mut self, secs: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_lifetime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_one_shot(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_one_shot", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pre_process_time(&mut self, secs: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_pre_process_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_explosiveness_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_explosiveness_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_randomness_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_randomness_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lifetime_randomness(&mut self, random: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (random,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_lifetime_randomness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_local_coordinates(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_use_local_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fixed_fps(&mut self, fps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (fps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_fixed_fps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fractional_delta(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_fractional_delta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_speed_scale(&mut self, scale: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1652usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Requests the particles to process for extra process time during a single frame.\n\nUseful for particle playback, if used in combination with \\[member use_fixed_seed] or by calling [`restart`][`crate::classes::CpuParticles2D::restart`] with parameter `keep_seed` set to `true`."]
        pub fn request_particles_process(&mut self, process_time: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (process_time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1653usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "request_particles_process", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_emitting(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1654usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "is_emitting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_amount(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1655usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_amount", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lifetime(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1656usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_lifetime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_one_shot(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1657usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_one_shot", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pre_process_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1658usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_pre_process_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_explosiveness_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_explosiveness_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_randomness_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_randomness_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lifetime_randomness(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_lifetime_randomness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_local_coordinates(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_use_local_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fixed_fps(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_fixed_fps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fractional_delta(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_fractional_delta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_fixed_seed(&mut self, use_fixed_seed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_fixed_seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_use_fixed_seed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_fixed_seed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_use_fixed_seed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_seed(&mut self, seed: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_seed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_seed(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_seed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_order(&mut self, order: crate::classes::cpu_particles_2d::DrawOrder,) {
            type CallRet = ();
            type CallParams = (crate::classes::cpu_particles_2d::DrawOrder,);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_draw_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_draw_order(&self,) -> crate::classes::cpu_particles_2d::DrawOrder {
            type CallRet = crate::classes::cpu_particles_2d::DrawOrder;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_draw_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Restarts the particle emitter.\n\nIf `keep_seed` is `true`, the current random seed will be preserved. Useful for seeking and playback."]
        pub(crate) fn restart_full(&mut self, keep_seed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (keep_seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "restart", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`restart_ex`][Self::restart_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Restarts the particle emitter.\n\nIf `keep_seed` is `true`, the current random seed will be preserved. Useful for seeking and playback."]
        #[inline]
        pub fn restart(&mut self,) {
            self.restart_ex() . done()
        }
        #[doc = "Restarts the particle emitter.\n\nIf `keep_seed` is `true`, the current random seed will be preserved. Useful for seeking and playback."]
        #[inline]
        pub fn restart_ex < 'ex > (&'ex mut self,) -> ExRestart < 'ex > {
            ExRestart::new(self,)
        }
        pub fn set_direction(&mut self, direction: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_direction(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_spread(&mut self, spread: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (spread,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_spread", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_spread(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_spread", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the minimum value for the given parameter."]
        pub fn set_param_min(&mut self, param: crate::classes::cpu_particles_2d::Parameter, value: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::cpu_particles_2d::Parameter, f32,);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_param_min", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the minimum value range for the given parameter."]
        pub fn get_param_min(&self, param: crate::classes::cpu_particles_2d::Parameter,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::cpu_particles_2d::Parameter,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_param_min", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the maximum value for the given parameter."]
        pub fn set_param_max(&mut self, param: crate::classes::cpu_particles_2d::Parameter, value: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::cpu_particles_2d::Parameter, f32,);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_param_max", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the maximum value range for the given parameter."]
        pub fn get_param_max(&self, param: crate::classes::cpu_particles_2d::Parameter,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::cpu_particles_2d::Parameter,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_param_max", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`Curve`][crate::classes::Curve] of the parameter specified by \\[enum Parameter]. Should be a unit [`Curve`][crate::classes::Curve]."]
        pub fn set_param_curve(&mut self, param: crate::classes::cpu_particles_2d::Parameter, curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::cpu_particles_2d::Parameter, CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (param, curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_param_curve", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Curve`][crate::classes::Curve] of the parameter specified by \\[enum Parameter]."]
        pub fn get_param_curve(&self, param: crate::classes::cpu_particles_2d::Parameter,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = (crate::classes::cpu_particles_2d::Parameter,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_param_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color_ramp(&mut self, ramp: impl AsArg < Option < Gd < crate::classes::Gradient >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Gradient > > >,);
            let args = (ramp.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_color_ramp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_ramp(&self,) -> Option < Gd < crate::classes::Gradient > > {
            type CallRet = Option < Gd < crate::classes::Gradient > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_color_ramp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color_initial_ramp(&mut self, ramp: impl AsArg < Option < Gd < crate::classes::Gradient >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Gradient > > >,);
            let args = (ramp.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_color_initial_ramp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_initial_ramp(&self,) -> Option < Gd < crate::classes::Gradient > > {
            type CallRet = Option < Gd < crate::classes::Gradient > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_color_initial_ramp", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables or disables the given particle flag."]
        pub fn set_particle_flag(&mut self, particle_flag: crate::classes::cpu_particles_2d::ParticleFlags, enable: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::cpu_particles_2d::ParticleFlags, bool,);
            let args = (particle_flag, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_particle_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the enabled state of the given particle flag."]
        pub fn get_particle_flag(&self, particle_flag: crate::classes::cpu_particles_2d::ParticleFlags,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::cpu_particles_2d::ParticleFlags,);
            let args = (particle_flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_particle_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_shape(&mut self, shape: crate::classes::cpu_particles_2d::EmissionShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::cpu_particles_2d::EmissionShape,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_shape(&self,) -> crate::classes::cpu_particles_2d::EmissionShape {
            type CallRet = crate::classes::cpu_particles_2d::EmissionShape;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_sphere_radius(&mut self, radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_sphere_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_sphere_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_sphere_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_rect_extents(&mut self, extents: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (extents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_rect_extents", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_rect_extents(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_rect_extents", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_points(&mut self, array: &PackedVector2Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_points", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_points(&self,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_points", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_normals(&mut self, array: &PackedVector2Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_normals", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_normals(&self,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_normals", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_colors(&mut self, array: &PackedColorArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedColorArray >,);
            let args = (RefArg::new(array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_colors(&self,) -> PackedColorArray {
            type CallRet = PackedColorArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_ring_inner_radius(&mut self, inner_radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (inner_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_ring_inner_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_ring_inner_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_ring_inner_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_ring_radius(&mut self, radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_emission_ring_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_ring_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_emission_ring_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity(&mut self, accel_vec: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (accel_vec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_split_scale(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_split_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_split_scale(&mut self, split_scale: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (split_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_split_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scale_curve_x(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_scale_curve_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scale_curve_x(&mut self, scale_curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (scale_curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_scale_curve_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scale_curve_y(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "get_scale_curve_y", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scale_curve_y(&mut self, scale_curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (scale_curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "set_scale_curve_y", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets this node's properties to match a given [`GPUParticles2D`][crate::classes::GpuParticles2D] node with an assigned [`ParticleProcessMaterial`][crate::classes::ParticleProcessMaterial]."]
        pub fn convert_from_particles(&mut self, particles: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (particles.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CpuParticles2D", "convert_from_particles", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CpuParticles2D {
        type Base = crate::classes::Node2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CPUParticles2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CpuParticles2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for CpuParticles2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for CpuParticles2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CpuParticles2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CpuParticles2D {
        
    }
    impl crate::obj::cap::GodotDefault for CpuParticles2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CpuParticles2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CpuParticles2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CpuParticles2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CpuParticles2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CpuParticles2D > for $Class {
                
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
#[doc = "Default-param extender for [`CpuParticles2D::restart_ex`][super::CpuParticles2D::restart_ex]."]
#[must_use]
pub struct ExRestart < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CpuParticles2D, keep_seed: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExRestart < 'ex > {
    fn new(surround_object: &'ex mut re_export::CpuParticles2D,) -> Self {
        let keep_seed = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, keep_seed: keep_seed,
        }
    }
    #[inline]
    pub fn keep_seed(self, keep_seed: bool) -> Self {
        Self {
            keep_seed: keep_seed, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, keep_seed,
        }
        = self;
        re_export::CpuParticles2D::restart_full(surround_object, keep_seed,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DrawOrder {
    ord: i32
}
impl DrawOrder {
    #[doc(alias = "DRAW_ORDER_INDEX")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_INDEX`"]
    pub const INDEX: DrawOrder = DrawOrder {
        ord: 0i32
    };
    #[doc(alias = "DRAW_ORDER_LIFETIME")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_LIFETIME`"]
    pub const LIFETIME: DrawOrder = DrawOrder {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DrawOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DrawOrder") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DrawOrder {
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
            Self::INDEX => "INDEX", Self::LIFETIME => "LIFETIME", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DrawOrder::INDEX, DrawOrder::LIFETIME]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DrawOrder >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INDEX", "DRAW_ORDER_INDEX", DrawOrder::INDEX), crate::meta::inspect::EnumConstant::new("LIFETIME", "DRAW_ORDER_LIFETIME", DrawOrder::LIFETIME)]
        }
    }
}
impl crate::meta::GodotConvert for DrawOrder {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Draw Order Index", 0i64), EnumeratorShape::new_int("Draw Order Lifetime", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CPUParticles2D.DrawOrder")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DrawOrder {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DrawOrder {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DrawOrder {
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
impl crate::registry::property::Export for DrawOrder {
    
}
impl crate::meta::Element for DrawOrder {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Parameter {
    ord: i32
}
impl Parameter {
    #[doc(alias = "PARAM_INITIAL_LINEAR_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_INITIAL_LINEAR_VELOCITY`"]
    pub const INITIAL_LINEAR_VELOCITY: Parameter = Parameter {
        ord: 0i32
    };
    #[doc(alias = "PARAM_ANGULAR_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_VELOCITY`"]
    pub const ANGULAR_VELOCITY: Parameter = Parameter {
        ord: 1i32
    };
    #[doc(alias = "PARAM_ORBIT_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_ORBIT_VELOCITY`"]
    pub const ORBIT_VELOCITY: Parameter = Parameter {
        ord: 2i32
    };
    #[doc(alias = "PARAM_LINEAR_ACCEL")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_ACCEL`"]
    pub const LINEAR_ACCEL: Parameter = Parameter {
        ord: 3i32
    };
    #[doc(alias = "PARAM_RADIAL_ACCEL")]
    #[doc = "Godot enumerator name: `PARAM_RADIAL_ACCEL`"]
    pub const RADIAL_ACCEL: Parameter = Parameter {
        ord: 4i32
    };
    #[doc(alias = "PARAM_TANGENTIAL_ACCEL")]
    #[doc = "Godot enumerator name: `PARAM_TANGENTIAL_ACCEL`"]
    pub const TANGENTIAL_ACCEL: Parameter = Parameter {
        ord: 5i32
    };
    #[doc(alias = "PARAM_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_DAMPING`"]
    pub const DAMPING: Parameter = Parameter {
        ord: 6i32
    };
    #[doc(alias = "PARAM_ANGLE")]
    #[doc = "Godot enumerator name: `PARAM_ANGLE`"]
    pub const ANGLE: Parameter = Parameter {
        ord: 7i32
    };
    #[doc(alias = "PARAM_SCALE")]
    #[doc = "Godot enumerator name: `PARAM_SCALE`"]
    pub const SCALE: Parameter = Parameter {
        ord: 8i32
    };
    #[doc(alias = "PARAM_HUE_VARIATION")]
    #[doc = "Godot enumerator name: `PARAM_HUE_VARIATION`"]
    pub const HUE_VARIATION: Parameter = Parameter {
        ord: 9i32
    };
    #[doc(alias = "PARAM_ANIM_SPEED")]
    #[doc = "Godot enumerator name: `PARAM_ANIM_SPEED`"]
    pub const ANIM_SPEED: Parameter = Parameter {
        ord: 10i32
    };
    #[doc(alias = "PARAM_ANIM_OFFSET")]
    #[doc = "Godot enumerator name: `PARAM_ANIM_OFFSET`"]
    pub const ANIM_OFFSET: Parameter = Parameter {
        ord: 11i32
    };
    #[doc(alias = "PARAM_MAX")]
    #[doc = "Godot enumerator name: `PARAM_MAX`"]
    pub const MAX: Parameter = Parameter {
        ord: 12i32
    };
    
}
impl std::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Parameter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Parameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 => Some(Self {
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
            Self::INITIAL_LINEAR_VELOCITY => "INITIAL_LINEAR_VELOCITY", Self::ANGULAR_VELOCITY => "ANGULAR_VELOCITY", Self::ORBIT_VELOCITY => "ORBIT_VELOCITY", Self::LINEAR_ACCEL => "LINEAR_ACCEL", Self::RADIAL_ACCEL => "RADIAL_ACCEL", Self::TANGENTIAL_ACCEL => "TANGENTIAL_ACCEL", Self::DAMPING => "DAMPING", Self::ANGLE => "ANGLE", Self::SCALE => "SCALE", Self::HUE_VARIATION => "HUE_VARIATION", Self::ANIM_SPEED => "ANIM_SPEED", Self::ANIM_OFFSET => "ANIM_OFFSET", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Parameter::INITIAL_LINEAR_VELOCITY, Parameter::ANGULAR_VELOCITY, Parameter::ORBIT_VELOCITY, Parameter::LINEAR_ACCEL, Parameter::RADIAL_ACCEL, Parameter::TANGENTIAL_ACCEL, Parameter::DAMPING, Parameter::ANGLE, Parameter::SCALE, Parameter::HUE_VARIATION, Parameter::ANIM_SPEED, Parameter::ANIM_OFFSET]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Parameter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INITIAL_LINEAR_VELOCITY", "PARAM_INITIAL_LINEAR_VELOCITY", Parameter::INITIAL_LINEAR_VELOCITY), crate::meta::inspect::EnumConstant::new("ANGULAR_VELOCITY", "PARAM_ANGULAR_VELOCITY", Parameter::ANGULAR_VELOCITY), crate::meta::inspect::EnumConstant::new("ORBIT_VELOCITY", "PARAM_ORBIT_VELOCITY", Parameter::ORBIT_VELOCITY), crate::meta::inspect::EnumConstant::new("LINEAR_ACCEL", "PARAM_LINEAR_ACCEL", Parameter::LINEAR_ACCEL), crate::meta::inspect::EnumConstant::new("RADIAL_ACCEL", "PARAM_RADIAL_ACCEL", Parameter::RADIAL_ACCEL), crate::meta::inspect::EnumConstant::new("TANGENTIAL_ACCEL", "PARAM_TANGENTIAL_ACCEL", Parameter::TANGENTIAL_ACCEL), crate::meta::inspect::EnumConstant::new("DAMPING", "PARAM_DAMPING", Parameter::DAMPING), crate::meta::inspect::EnumConstant::new("ANGLE", "PARAM_ANGLE", Parameter::ANGLE), crate::meta::inspect::EnumConstant::new("SCALE", "PARAM_SCALE", Parameter::SCALE), crate::meta::inspect::EnumConstant::new("HUE_VARIATION", "PARAM_HUE_VARIATION", Parameter::HUE_VARIATION), crate::meta::inspect::EnumConstant::new("ANIM_SPEED", "PARAM_ANIM_SPEED", Parameter::ANIM_SPEED), crate::meta::inspect::EnumConstant::new("ANIM_OFFSET", "PARAM_ANIM_OFFSET", Parameter::ANIM_OFFSET), crate::meta::inspect::EnumConstant::new("MAX", "PARAM_MAX", Parameter::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Parameter {
    const ENUMERATOR_COUNT: usize = 12usize;
    
}
impl crate::meta::GodotConvert for Parameter {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Param Initial Linear Velocity", 0i64), EnumeratorShape::new_int("Param Angular Velocity", 1i64), EnumeratorShape::new_int("Param Orbit Velocity", 2i64), EnumeratorShape::new_int("Param Linear Accel", 3i64), EnumeratorShape::new_int("Param Radial Accel", 4i64), EnumeratorShape::new_int("Param Tangential Accel", 5i64), EnumeratorShape::new_int("Param Damping", 6i64), EnumeratorShape::new_int("Param Angle", 7i64), EnumeratorShape::new_int("Param Scale", 8i64), EnumeratorShape::new_int("Param Hue Variation", 9i64), EnumeratorShape::new_int("Param Anim Speed", 10i64), EnumeratorShape::new_int("Param Anim Offset", 11i64), EnumeratorShape::new_int("Param Max", 12i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CPUParticles2D.Parameter")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Parameter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Parameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Parameter {
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
impl crate::registry::property::Export for Parameter {
    
}
impl crate::meta::Element for Parameter {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticleFlags {
    ord: i32
}
impl ParticleFlags {
    #[doc(alias = "PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY`"]
    pub const ALIGN_Y_TO_VELOCITY: ParticleFlags = ParticleFlags {
        ord: 0i32
    };
    #[doc(alias = "PARTICLE_FLAG_ROTATE_Y")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_ROTATE_Y`"]
    pub const ROTATE_Y: ParticleFlags = ParticleFlags {
        ord: 1i32
    };
    #[doc(alias = "PARTICLE_FLAG_DISABLE_Z")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_DISABLE_Z`"]
    pub const DISABLE_Z: ParticleFlags = ParticleFlags {
        ord: 2i32
    };
    #[doc(alias = "PARTICLE_FLAG_MAX")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_MAX`"]
    pub const MAX: ParticleFlags = ParticleFlags {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ParticleFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticleFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticleFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::ALIGN_Y_TO_VELOCITY => "ALIGN_Y_TO_VELOCITY", Self::ROTATE_Y => "ROTATE_Y", Self::DISABLE_Z => "DISABLE_Z", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ParticleFlags::ALIGN_Y_TO_VELOCITY, ParticleFlags::ROTATE_Y, ParticleFlags::DISABLE_Z]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticleFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALIGN_Y_TO_VELOCITY", "PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY", ParticleFlags::ALIGN_Y_TO_VELOCITY), crate::meta::inspect::EnumConstant::new("ROTATE_Y", "PARTICLE_FLAG_ROTATE_Y", ParticleFlags::ROTATE_Y), crate::meta::inspect::EnumConstant::new("DISABLE_Z", "PARTICLE_FLAG_DISABLE_Z", ParticleFlags::DISABLE_Z), crate::meta::inspect::EnumConstant::new("MAX", "PARTICLE_FLAG_MAX", ParticleFlags::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ParticleFlags {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ParticleFlags {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Particle Flag Align Y To Velocity", 0i64), EnumeratorShape::new_int("Particle Flag Rotate Y", 1i64), EnumeratorShape::new_int("Particle Flag Disable Z", 2i64), EnumeratorShape::new_int("Particle Flag Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CPUParticles2D.ParticleFlags")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ParticleFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticleFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ParticleFlags {
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
impl crate::registry::property::Export for ParticleFlags {
    
}
impl crate::meta::Element for ParticleFlags {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EmissionShape {
    ord: i32
}
impl EmissionShape {
    #[doc(alias = "EMISSION_SHAPE_POINT")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_POINT`"]
    pub const POINT: EmissionShape = EmissionShape {
        ord: 0i32
    };
    #[doc(alias = "EMISSION_SHAPE_SPHERE")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_SPHERE`"]
    pub const SPHERE: EmissionShape = EmissionShape {
        ord: 1i32
    };
    #[doc(alias = "EMISSION_SHAPE_SPHERE_SURFACE")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_SPHERE_SURFACE`"]
    pub const SPHERE_SURFACE: EmissionShape = EmissionShape {
        ord: 2i32
    };
    #[doc(alias = "EMISSION_SHAPE_RECTANGLE")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_RECTANGLE`"]
    pub const RECTANGLE: EmissionShape = EmissionShape {
        ord: 3i32
    };
    #[doc(alias = "EMISSION_SHAPE_POINTS")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_POINTS`"]
    pub const POINTS: EmissionShape = EmissionShape {
        ord: 4i32
    };
    #[doc(alias = "EMISSION_SHAPE_DIRECTED_POINTS")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_DIRECTED_POINTS`"]
    pub const DIRECTED_POINTS: EmissionShape = EmissionShape {
        ord: 5i32
    };
    #[doc(alias = "EMISSION_SHAPE_RING")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_RING`"]
    pub const RING: EmissionShape = EmissionShape {
        ord: 6i32
    };
    #[doc(alias = "EMISSION_SHAPE_MAX")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_MAX`"]
    pub const MAX: EmissionShape = EmissionShape {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for EmissionShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EmissionShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EmissionShape {
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
            Self::POINT => "POINT", Self::SPHERE => "SPHERE", Self::SPHERE_SURFACE => "SPHERE_SURFACE", Self::RECTANGLE => "RECTANGLE", Self::POINTS => "POINTS", Self::DIRECTED_POINTS => "DIRECTED_POINTS", Self::RING => "RING", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EmissionShape::POINT, EmissionShape::SPHERE, EmissionShape::SPHERE_SURFACE, EmissionShape::RECTANGLE, EmissionShape::POINTS, EmissionShape::DIRECTED_POINTS, EmissionShape::RING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EmissionShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POINT", "EMISSION_SHAPE_POINT", EmissionShape::POINT), crate::meta::inspect::EnumConstant::new("SPHERE", "EMISSION_SHAPE_SPHERE", EmissionShape::SPHERE), crate::meta::inspect::EnumConstant::new("SPHERE_SURFACE", "EMISSION_SHAPE_SPHERE_SURFACE", EmissionShape::SPHERE_SURFACE), crate::meta::inspect::EnumConstant::new("RECTANGLE", "EMISSION_SHAPE_RECTANGLE", EmissionShape::RECTANGLE), crate::meta::inspect::EnumConstant::new("POINTS", "EMISSION_SHAPE_POINTS", EmissionShape::POINTS), crate::meta::inspect::EnumConstant::new("DIRECTED_POINTS", "EMISSION_SHAPE_DIRECTED_POINTS", EmissionShape::DIRECTED_POINTS), crate::meta::inspect::EnumConstant::new("RING", "EMISSION_SHAPE_RING", EmissionShape::RING), crate::meta::inspect::EnumConstant::new("MAX", "EMISSION_SHAPE_MAX", EmissionShape::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for EmissionShape {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for EmissionShape {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Emission Shape Point", 0i64), EnumeratorShape::new_int("Emission Shape Sphere", 1i64), EnumeratorShape::new_int("Emission Shape Sphere Surface", 2i64), EnumeratorShape::new_int("Emission Shape Rectangle", 3i64), EnumeratorShape::new_int("Emission Shape Points", 4i64), EnumeratorShape::new_int("Emission Shape Directed Points", 5i64), EnumeratorShape::new_int("Emission Shape Ring", 6i64), EnumeratorShape::new_int("Emission Shape Max", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CPUParticles2D.EmissionShape")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EmissionShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EmissionShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EmissionShape {
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
impl crate::registry::property::Export for EmissionShape {
    
}
impl crate::meta::Element for EmissionShape {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CpuParticles2D;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`CpuParticles2D`][crate::classes::CpuParticles2D] class."]
    pub struct SignalsOfCpuParticles2D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfCpuParticles2D < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn finished(&mut self) -> SigFinished < 'c, C > {
            SigFinished {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "finished")
            }
        }
    }
    type TypedSigFinished < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFinished < 'c, C: WithSignals > {
        typed: TypedSigFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFinished < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFinished < 'c, C > {
        type Target = TypedSigFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for CpuParticles2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCpuParticles2D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfCpuParticles2D < 'c, C > {
        type Target = < < CpuParticles2D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = CpuParticles2D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfCpuParticles2D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = CpuParticles2D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
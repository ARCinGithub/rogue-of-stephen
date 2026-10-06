#![doc = "Sidecar module for class [`ReflectionProbe`][crate::classes::ReflectionProbe].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ReflectionProbe` enums](https://docs.godotengine.org/en/stable/classes/class_reflectionprobe.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ReflectionProbe`.\n\nInherits [`VisualInstance3D`][crate::classes::VisualInstance3D].\n\nRelated symbols:\n\n* [`reflection_probe`][crate::classes::reflection_probe]: sidecar module with related enum/flag types\n* [`IReflectionProbe`][crate::classes::IReflectionProbe]: virtual methods\n\n\nSee also [Godot docs for `ReflectionProbe`](https://docs.godotengine.org/en/stable/classes/class_reflectionprobe.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ReflectionProbe::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nCaptures its surroundings as a cubemap, and stores versions of it with increasing levels of blur to simulate different material roughnesses.\n\nThe `ReflectionProbe` is used to create high-quality reflections at a low performance cost (when \\[member update_mode] is [`UpdateMode::ONCE`][`crate::classes::reflection_probe::UpdateMode::ONCE`]). `ReflectionProbe`s can be blended together and with the rest of the scene smoothly. `ReflectionProbe`s can also be combined with [`VoxelGI`][crate::classes::VoxelGi], SDFGI (\\[member Environment.sdfgi_enabled]) and screen-space reflections (\\[member Environment.ssr_enabled]) to get more accurate reflections in specific areas. `ReflectionProbe`s render all objects within their \\[member cull_mask], so updating them can be quite expensive. It is best to update them once with the important static objects and then leave them as-is.\n\n**Note:** Unlike [`VoxelGI`][crate::classes::VoxelGi] and SDFGI, `ReflectionProbe`s only source their environment from a [`WorldEnvironment`][crate::classes::WorldEnvironment] node. If you specify an [`Environment`][crate::classes::Environment] resource within a [`Camera3D`][crate::classes::Camera3D] node, it will be ignored by the `ReflectionProbe`. This can lead to incorrect lighting within the `ReflectionProbe`.\n\n**Note:** When using the Mobile rendering method, only `8` reflection probes can be displayed on each mesh resource, while the Compatibility rendering method only supports up to `2` reflection probes on each mesh. Attempting to display more than `8` reflection probes on a single mesh resource using the Mobile renderer will result in reflection probes flickering in and out as the camera moves, while the Compatibility renderer will not render any additional probes if more than `2` reflection probes are being used.\n\n**Note:** When using the Mobile rendering method, reflection probes will only correctly affect meshes whose visibility AABB intersects with the reflection probe's AABB. If using a shader to deform the mesh in a way that makes it go outside its AABB, \\[member GeometryInstance3D.extra_cull_margin] must be increased on the mesh. Otherwise, the reflection probe may not be visible on the mesh."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ReflectionProbe {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ReflectionProbe`][crate::classes::ReflectionProbe].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ReflectionProbe` methods](https://docs.godotengine.org/en/stable/classes/class_reflectionprobe.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IReflectionProbe: crate::obj::GodotClass < Base = ReflectionProbe > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_aabb(&self,) -> Aabb {
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
    impl ReflectionProbe {
        pub fn set_intensity(&mut self, intensity: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (intensity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_intensity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_blend_distance(&mut self, blend_distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (blend_distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_blend_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_blend_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ambient_mode(&mut self, ambient: crate::classes::reflection_probe::AmbientMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::reflection_probe::AmbientMode,);
            let args = (ambient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_ambient_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ambient_mode(&self,) -> crate::classes::reflection_probe::AmbientMode {
            type CallRet = crate::classes::reflection_probe::AmbientMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_ambient_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ambient_color(&mut self, ambient: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (ambient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_ambient_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ambient_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_ambient_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ambient_color_energy(&mut self, ambient_energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ambient_energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_ambient_color_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ambient_color_energy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_ambient_color_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_distance(&mut self, max_distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (max_distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_max_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_max_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mesh_lod_threshold(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_mesh_lod_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mesh_lod_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_mesh_lod_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_origin_offset(&mut self, origin_offset: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (origin_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_origin_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_origin_offset(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_origin_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_as_interior(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_as_interior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_set_as_interior(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "is_set_as_interior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_box_projection(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_enable_box_projection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_box_projection_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "is_box_projection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_shadows(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_enable_shadows", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_shadows_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "are_shadows_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cull_mask(&mut self, layers: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cull_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_reflection_mask(&mut self, layers: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_reflection_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_reflection_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_reflection_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_update_mode(&mut self, mode: crate::classes::reflection_probe::UpdateMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::reflection_probe::UpdateMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_update_mode(&self,) -> crate::classes::reflection_probe::UpdateMode {
            type CallRet = crate::classes::reflection_probe::UpdateMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_update_mode", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ReflectionProbe {
        type Base = crate::classes::VisualInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ReflectionProbe"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ReflectionProbe {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for ReflectionProbe {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for ReflectionProbe {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ReflectionProbe {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ReflectionProbe {
        
    }
    impl crate::obj::cap::GodotDefault for ReflectionProbe {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ReflectionProbe {
        type Target = crate::classes::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ReflectionProbe {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ReflectionProbe`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ReflectionProbe__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ReflectionProbe > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
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
pub struct UpdateMode {
    ord: i32
}
impl UpdateMode {
    #[doc(alias = "UPDATE_ONCE")]
    #[doc = "Godot enumerator name: `UPDATE_ONCE`"]
    pub const ONCE: UpdateMode = UpdateMode {
        ord: 0i32
    };
    #[doc(alias = "UPDATE_ALWAYS")]
    #[doc = "Godot enumerator name: `UPDATE_ALWAYS`"]
    pub const ALWAYS: UpdateMode = UpdateMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for UpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("UpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for UpdateMode {
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
            Self::ONCE => "ONCE", Self::ALWAYS => "ALWAYS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[UpdateMode::ONCE, UpdateMode::ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < UpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ONCE", "UPDATE_ONCE", UpdateMode::ONCE), crate::meta::inspect::EnumConstant::new("ALWAYS", "UPDATE_ALWAYS", UpdateMode::ALWAYS)]
        }
    }
}
impl crate::meta::GodotConvert for UpdateMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Update Once", 0i64), EnumeratorShape::new_int("Update Always", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("ReflectionProbe.UpdateMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for UpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for UpdateMode {
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
impl crate::registry::property::Export for UpdateMode {
    
}
impl crate::meta::Element for UpdateMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AmbientMode {
    ord: i32
}
impl AmbientMode {
    #[doc(alias = "AMBIENT_DISABLED")]
    #[doc = "Godot enumerator name: `AMBIENT_DISABLED`"]
    pub const DISABLED: AmbientMode = AmbientMode {
        ord: 0i32
    };
    #[doc(alias = "AMBIENT_ENVIRONMENT")]
    #[doc = "Godot enumerator name: `AMBIENT_ENVIRONMENT`"]
    pub const ENVIRONMENT: AmbientMode = AmbientMode {
        ord: 1i32
    };
    #[doc(alias = "AMBIENT_COLOR")]
    #[doc = "Godot enumerator name: `AMBIENT_COLOR`"]
    pub const COLOR: AmbientMode = AmbientMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AmbientMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AmbientMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AmbientMode {
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
            Self::DISABLED => "DISABLED", Self::ENVIRONMENT => "ENVIRONMENT", Self::COLOR => "COLOR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AmbientMode::DISABLED, AmbientMode::ENVIRONMENT, AmbientMode::COLOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AmbientMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "AMBIENT_DISABLED", AmbientMode::DISABLED), crate::meta::inspect::EnumConstant::new("ENVIRONMENT", "AMBIENT_ENVIRONMENT", AmbientMode::ENVIRONMENT), crate::meta::inspect::EnumConstant::new("COLOR", "AMBIENT_COLOR", AmbientMode::COLOR)]
        }
    }
}
impl crate::meta::GodotConvert for AmbientMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Ambient Disabled", 0i64), EnumeratorShape::new_int("Ambient Environment", 1i64), EnumeratorShape::new_int("Ambient Color", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("ReflectionProbe.AmbientMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AmbientMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AmbientMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AmbientMode {
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
impl crate::registry::property::Export for AmbientMode {
    
}
impl crate::meta::Element for AmbientMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ReflectionProbe;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for ReflectionProbe {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
#![doc = "Sidecar module for class [`VoxelGi`][crate::classes::VoxelGi].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VoxelGI` enums](https://docs.godotengine.org/en/stable/classes/class_voxelgi.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VoxelGI`.\n\nInherits [`VisualInstance3D`][crate::classes::VisualInstance3D].\n\nRelated symbols:\n\n* [`voxel_gi`][crate::classes::voxel_gi]: sidecar module with related enum/flag types\n* [`IVoxelGi`][crate::classes::IVoxelGi]: virtual methods\n\n\nSee also [Godot docs for `VoxelGI`](https://docs.godotengine.org/en/stable/classes/class_voxelgi.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`VoxelGi::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\n`VoxelGI`s are used to provide high-quality real-time indirect light and reflections to scenes. They precompute the effect of objects that emit light and the effect of static geometry to simulate the behavior of complex light in real-time. `VoxelGI`s need to be baked before having a visible effect. However, once baked, dynamic objects will receive light from them. Furthermore, lights can be fully dynamic or baked.\n\n**Note:** `VoxelGI` is only supported in the Forward+ rendering method, not Mobile or Compatibility.\n\n**Procedural generation:** `VoxelGI` can be baked in an exported project, which makes it suitable for procedurally generated or user-built levels as long as all the geometry is generated in advance. For games where geometry is generated at any time during gameplay, SDFGI is more suitable (see \\[member Environment.sdfgi_enabled]).\n\n**Performance:** `VoxelGI` is relatively demanding on the GPU and is not suited to low-end hardware such as integrated graphics (consider [`LightmapGI`][crate::classes::LightmapGi] instead). To improve performance, adjust \\[member ProjectSettings.rendering/global_illumination/voxel_gi/quality] and enable \\[member ProjectSettings.rendering/global_illumination/gi/use_half_resolution] in the Project Settings. To provide a fallback for low-end hardware, consider adding an option to disable `VoxelGI` in your project's options menus. A `VoxelGI` node can be disabled by hiding it.\n\n**Note:** Meshes should have sufficiently thick walls to avoid light leaks (avoid one-sided walls). For interior levels, enclose your level geometry in a sufficiently large box and bridge the loops to close the mesh. To further prevent light leaks, you can also strategically place temporary [`MeshInstance3D`][crate::classes::MeshInstance3D] nodes with their \\[member GeometryInstance3D.gi_mode] set to [`GiMode::STATIC`][`crate::classes::geometry_instance_3d::GiMode::STATIC`]. These temporary nodes can then be hidden after baking the `VoxelGI` node."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VoxelGi {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VoxelGi`][crate::classes::VoxelGi].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `VoxelGI` methods](https://docs.godotengine.org/en/stable/classes/class_voxelgi.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVoxelGi: crate::obj::GodotClass < Base = VoxelGi > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VoxelGi {
        pub fn set_probe_data(&mut self, data: impl AsArg < Option < Gd < crate::classes::VoxelGiData >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::VoxelGiData > > >,);
            let args = (data.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "set_probe_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_probe_data(&self,) -> Option < Gd < crate::classes::VoxelGiData > > {
            type CallRet = Option < Gd < crate::classes::VoxelGiData > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "get_probe_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_subdiv(&mut self, subdiv: crate::classes::voxel_gi::Subdiv,) {
            type CallRet = ();
            type CallParams = (crate::classes::voxel_gi::Subdiv,);
            let args = (subdiv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "set_subdiv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_subdiv(&self,) -> crate::classes::voxel_gi::Subdiv {
            type CallRet = crate::classes::voxel_gi::Subdiv;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "get_subdiv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_camera_attributes(&mut self, camera_attributes: impl AsArg < Option < Gd < crate::classes::CameraAttributes >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::CameraAttributes > > >,);
            let args = (camera_attributes.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "set_camera_attributes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_camera_attributes(&self,) -> Option < Gd < crate::classes::CameraAttributes > > {
            type CallRet = Option < Gd < crate::classes::CameraAttributes > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "get_camera_attributes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Bakes the effect from all [`GeometryInstance3D`][crate::classes::GeometryInstance3D]s marked with [`GiMode::STATIC`][`crate::classes::geometry_instance_3d::GiMode::STATIC`] and [`Light3D`][crate::classes::Light3D]s marked with either [`BakeMode::STATIC`][`crate::classes::light_3d::BakeMode::STATIC`] or [`BakeMode::DYNAMIC`][`crate::classes::light_3d::BakeMode::DYNAMIC`]. If `create_visual_debug` is `true`, after baking the light, this will generate a [`MultiMesh`][crate::classes::MultiMesh] that has a cube representing each solid cell with each cube colored to the cell's albedo color. This can be used to visualize the `VoxelGI`'s data and debug any issues that may be occurring.\n\n**Note:** [`bake`][`crate::classes::VoxelGi::bake`] works from the editor and in exported projects. This makes it suitable for procedurally generated or user-built levels. Baking a `VoxelGI` node generally takes from 5 to 20 seconds in most scenes. Reducing \\[member subdiv] can speed up baking.\n\n**Note:** [`GeometryInstance3D`][crate::classes::GeometryInstance3D]s and [`Light3D`][crate::classes::Light3D]s must be fully ready before [`bake`][`crate::classes::VoxelGi::bake`] is called. If you are procedurally creating those and some meshes or lights are missing from your baked `VoxelGI`, use `call_deferred(\"bake\")` instead of calling [`bake`][`crate::classes::VoxelGi::bake`] directly."]
        pub(crate) fn bake_full(&mut self, from_node: CowArg < Option < Gd < crate::classes::Node > > >, create_visual_debug: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, bool,);
            let args = (from_node, create_visual_debug,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "bake", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`bake_ex`][Self::bake_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Bakes the effect from all [`GeometryInstance3D`][crate::classes::GeometryInstance3D]s marked with [`GiMode::STATIC`][`crate::classes::geometry_instance_3d::GiMode::STATIC`] and [`Light3D`][crate::classes::Light3D]s marked with either [`BakeMode::STATIC`][`crate::classes::light_3d::BakeMode::STATIC`] or [`BakeMode::DYNAMIC`][`crate::classes::light_3d::BakeMode::DYNAMIC`]. If `create_visual_debug` is `true`, after baking the light, this will generate a [`MultiMesh`][crate::classes::MultiMesh] that has a cube representing each solid cell with each cube colored to the cell's albedo color. This can be used to visualize the `VoxelGI`'s data and debug any issues that may be occurring.\n\n**Note:** [`bake`][`crate::classes::VoxelGi::bake`] works from the editor and in exported projects. This makes it suitable for procedurally generated or user-built levels. Baking a `VoxelGI` node generally takes from 5 to 20 seconds in most scenes. Reducing \\[member subdiv] can speed up baking.\n\n**Note:** [`GeometryInstance3D`][crate::classes::GeometryInstance3D]s and [`Light3D`][crate::classes::Light3D]s must be fully ready before [`bake`][`crate::classes::VoxelGi::bake`] is called. If you are procedurally creating those and some meshes or lights are missing from your baked `VoxelGI`, use `call_deferred(\"bake\")` instead of calling [`bake`][`crate::classes::VoxelGi::bake`] directly."]
        #[inline]
        pub fn bake(&mut self,) {
            self.bake_ex() . done()
        }
        #[doc = "Bakes the effect from all [`GeometryInstance3D`][crate::classes::GeometryInstance3D]s marked with [`GiMode::STATIC`][`crate::classes::geometry_instance_3d::GiMode::STATIC`] and [`Light3D`][crate::classes::Light3D]s marked with either [`BakeMode::STATIC`][`crate::classes::light_3d::BakeMode::STATIC`] or [`BakeMode::DYNAMIC`][`crate::classes::light_3d::BakeMode::DYNAMIC`]. If `create_visual_debug` is `true`, after baking the light, this will generate a [`MultiMesh`][crate::classes::MultiMesh] that has a cube representing each solid cell with each cube colored to the cell's albedo color. This can be used to visualize the `VoxelGI`'s data and debug any issues that may be occurring.\n\n**Note:** [`bake`][`crate::classes::VoxelGi::bake`] works from the editor and in exported projects. This makes it suitable for procedurally generated or user-built levels. Baking a `VoxelGI` node generally takes from 5 to 20 seconds in most scenes. Reducing \\[member subdiv] can speed up baking.\n\n**Note:** [`GeometryInstance3D`][crate::classes::GeometryInstance3D]s and [`Light3D`][crate::classes::Light3D]s must be fully ready before [`bake`][`crate::classes::VoxelGi::bake`] is called. If you are procedurally creating those and some meshes or lights are missing from your baked `VoxelGI`, use `call_deferred(\"bake\")` instead of calling [`bake`][`crate::classes::VoxelGi::bake`] directly."]
        #[inline]
        pub fn bake_ex < 'ex > (&'ex mut self,) -> ExBake < 'ex > {
            ExBake::new(self,)
        }
        #[doc = "Calls [`bake`][`crate::classes::VoxelGi::bake`] with `create_visual_debug` enabled."]
        pub fn debug_bake(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "debug_bake", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VoxelGi {
        type Base = crate::classes::VisualInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VoxelGI"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VoxelGi {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for VoxelGi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for VoxelGi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for VoxelGi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VoxelGi {
        
    }
    impl crate::obj::cap::GodotDefault for VoxelGi {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VoxelGi {
        type Target = crate::classes::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VoxelGi {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VoxelGi`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VoxelGi__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VoxelGi > for $Class {
                
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
#[doc = "Default-param extender for [`VoxelGi::bake_ex`][super::VoxelGi::bake_ex]."]
#[must_use]
pub struct ExBake < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::VoxelGi, from_node: CowArg < 'ex, Option < Gd < crate::classes::Node > > >, create_visual_debug: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBake < 'ex > {
    fn new(surround_object: &'ex mut re_export::VoxelGi,) -> Self {
        let from_node = Gd::null_arg();
        let create_visual_debug = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_node: from_node.into_arg(), create_visual_debug: create_visual_debug,
        }
    }
    #[inline]
    pub fn from_node(self, from_node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'ex) -> Self {
        Self {
            from_node: from_node.into_arg(), .. self
        }
    }
    #[inline]
    pub fn create_visual_debug(self, create_visual_debug: bool) -> Self {
        Self {
            create_visual_debug: create_visual_debug, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_node, create_visual_debug,
        }
        = self;
        re_export::VoxelGi::bake_full(surround_object, from_node, create_visual_debug,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Subdiv {
    ord: i32
}
impl Subdiv {
    pub const SUBDIV_64: Subdiv = Subdiv {
        ord: 0i32
    };
    pub const SUBDIV_128: Subdiv = Subdiv {
        ord: 1i32
    };
    pub const SUBDIV_256: Subdiv = Subdiv {
        ord: 2i32
    };
    pub const SUBDIV_512: Subdiv = Subdiv {
        ord: 3i32
    };
    #[doc(alias = "SUBDIV_MAX")]
    #[doc = "Godot enumerator name: `SUBDIV_MAX`"]
    pub const MAX: Subdiv = Subdiv {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Subdiv {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Subdiv") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Subdiv {
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
            Self::SUBDIV_64 => "SUBDIV_64", Self::SUBDIV_128 => "SUBDIV_128", Self::SUBDIV_256 => "SUBDIV_256", Self::SUBDIV_512 => "SUBDIV_512", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Subdiv::SUBDIV_64, Subdiv::SUBDIV_128, Subdiv::SUBDIV_256, Subdiv::SUBDIV_512]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Subdiv >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SUBDIV_64", "SUBDIV_64", Subdiv::SUBDIV_64), crate::meta::inspect::EnumConstant::new("SUBDIV_128", "SUBDIV_128", Subdiv::SUBDIV_128), crate::meta::inspect::EnumConstant::new("SUBDIV_256", "SUBDIV_256", Subdiv::SUBDIV_256), crate::meta::inspect::EnumConstant::new("SUBDIV_512", "SUBDIV_512", Subdiv::SUBDIV_512), crate::meta::inspect::EnumConstant::new("MAX", "SUBDIV_MAX", Subdiv::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Subdiv {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for Subdiv {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Subdiv 64", 0i64), EnumeratorShape::new_int("Subdiv 128", 1i64), EnumeratorShape::new_int("Subdiv 256", 2i64), EnumeratorShape::new_int("Subdiv 512", 3i64), EnumeratorShape::new_int("Subdiv Max", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("VoxelGI.Subdiv")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Subdiv {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Subdiv {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Subdiv {
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
impl crate::registry::property::Export for Subdiv {
    
}
impl crate::meta::Element for Subdiv {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VoxelGi;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for VoxelGi {
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
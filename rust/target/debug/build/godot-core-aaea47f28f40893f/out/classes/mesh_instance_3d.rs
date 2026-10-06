#![doc = "Sidecar module for class [`MeshInstance3D`][crate::classes::MeshInstance3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MeshInstance3D` enums](https://docs.godotengine.org/en/stable/classes/class_meshinstance3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MeshInstance3D`.\n\nInherits [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nRelated symbols:\n\n* [`mesh_instance_3d`][crate::classes::mesh_instance_3d]: sidecar module with related enum/flag types\n* [`IMeshInstance3D`][crate::classes::IMeshInstance3D]: virtual methods\n\n\nSee also [Godot docs for `MeshInstance3D`](https://docs.godotengine.org/en/stable/classes/class_meshinstance3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`MeshInstance3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nMeshInstance3D is a node that takes a [`Mesh`][crate::classes::Mesh] resource and adds it to the current scenario by creating an instance of it. This is the class most often used to render 3D geometry and can be used to instance a single [`Mesh`][crate::classes::Mesh] in many places. This allows reusing geometry, which can save on resources. When a [`Mesh`][crate::classes::Mesh] has to be instantiated more than thousands of times at close proximity, consider using a [`MultiMesh`][crate::classes::MultiMesh] in a [`MultiMeshInstance3D`][crate::classes::MultiMeshInstance3D] instead."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MeshInstance3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MeshInstance3D`][crate::classes::MeshInstance3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IGeometryInstance3D`][crate::classes::IGeometryInstance3D] > [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `MeshInstance3D` methods](https://docs.godotengine.org/en/stable/classes/class_meshinstance3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMeshInstance3D: crate::obj::GodotClass < Base = MeshInstance3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MeshInstance3D {
        pub fn set_mesh(&mut self, mesh: impl AsArg < Option < Gd < crate::classes::Mesh >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Mesh > > >,);
            let args = (mesh.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3598usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "set_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mesh(&self,) -> Option < Gd < crate::classes::Mesh > > {
            type CallRet = Option < Gd < crate::classes::Mesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3599usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_skeleton_path(&mut self, skeleton_path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (skeleton_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3600usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "set_skeleton_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_skeleton_path(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3601usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_skeleton_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_skin(&mut self, skin: impl AsArg < Option < Gd < crate::classes::Skin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Skin > > >,);
            let args = (skin.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3602usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "set_skin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_skin(&self,) -> Option < Gd < crate::classes::Skin > > {
            type CallRet = Option < Gd < crate::classes::Skin > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3603usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_skin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the internal [`SkinReference`][crate::classes::SkinReference] containing the skeleton's [`RID`][crate::builtin::Rid] attached to this RID. See also [`get_rid`][`crate::classes::Resource::get_rid`], [`get_skeleton`][`crate::classes::SkinReference::get_skeleton`], and [`instance_attach_skeleton`][`crate::classes::RenderingServer::instance_attach_skeleton`]."]
        pub fn get_skin_reference(&self,) -> Option < Gd < crate::classes::SkinReference > > {
            type CallRet = Option < Gd < crate::classes::SkinReference > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3604usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_skin_reference", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of surface override materials. This is equivalent to [`get_surface_count`][`crate::classes::Mesh::get_surface_count`]. See also [`get_surface_override_material`][`crate::classes::MeshInstance3D::get_surface_override_material`]."]
        pub fn get_surface_override_material_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3605usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_surface_override_material_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the override `material` for the specified `surface` of the [`Mesh`][crate::classes::Mesh] resource. This material is associated with this `MeshInstance3D` rather than with \\[member mesh].\n\n**Note:** This assigns the [`Material`][crate::classes::Material] associated to the `MeshInstance3D`'s Surface Material Override properties, not the material within the [`Mesh`][crate::classes::Mesh] resource. To set the material within the [`Mesh`][crate::classes::Mesh] resource, use [`surface_set_material`][`crate::classes::Mesh::surface_set_material`] instead."]
        pub fn set_surface_override_material(&mut self, surface: i32, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (surface, material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "set_surface_override_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the override [`Material`][crate::classes::Material] for the specified `surface` of the [`Mesh`][crate::classes::Mesh] resource. See also [`get_surface_override_material_count`][`crate::classes::MeshInstance3D::get_surface_override_material_count`].\n\n**Note:** This returns the [`Material`][crate::classes::Material] associated to the `MeshInstance3D`'s Surface Material Override properties, not the material within the [`Mesh`][crate::classes::Mesh] resource. To get the material within the [`Mesh`][crate::classes::Mesh] resource, use [`surface_get_material`][`crate::classes::Mesh::surface_get_material`] instead."]
        pub fn get_surface_override_material(&self, surface: i32,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = (i32,);
            let args = (surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_surface_override_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Material`][crate::classes::Material] that will be used by the [`Mesh`][crate::classes::Mesh] when drawing. This can return the \\[member GeometryInstance3D.material_override], the surface override [`Material`][crate::classes::Material] defined in this `MeshInstance3D`, or the surface [`Material`][crate::classes::Material] defined in the \\[member mesh]. For example, if \\[member GeometryInstance3D.material_override] is used, all surfaces will return the override material.\n\nReturns `null` if no material is active, including when \\[member mesh] is `null`."]
        pub fn get_active_material(&self, surface: i32,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = (i32,);
            let args = (surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_active_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This helper creates a [`StaticBody3D`][crate::classes::StaticBody3D] child node with a [`ConcavePolygonShape3D`][crate::classes::ConcavePolygonShape3D] collision shape calculated from the mesh geometry. It's mainly used for testing."]
        pub fn create_trimesh_collision(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "create_trimesh_collision", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This helper creates a [`StaticBody3D`][crate::classes::StaticBody3D] child node with a [`ConvexPolygonShape3D`][crate::classes::ConvexPolygonShape3D] collision shape calculated from the mesh geometry. It's mainly used for testing.\n\nIf `clean` is `true` (default), duplicate and interior vertices are removed automatically. You can set it to `false` to make the process faster if not needed.\n\nIf `simplify` is `true`, the geometry can be further simplified to reduce the number of vertices. Disabled by default."]
        pub(crate) fn create_convex_collision_full(&mut self, clean: bool, simplify: bool,) {
            type CallRet = ();
            type CallParams = (bool, bool,);
            let args = (clean, simplify,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "create_convex_collision", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_convex_collision_ex`][Self::create_convex_collision_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "This helper creates a [`StaticBody3D`][crate::classes::StaticBody3D] child node with a [`ConvexPolygonShape3D`][crate::classes::ConvexPolygonShape3D] collision shape calculated from the mesh geometry. It's mainly used for testing.\n\nIf `clean` is `true` (default), duplicate and interior vertices are removed automatically. You can set it to `false` to make the process faster if not needed.\n\nIf `simplify` is `true`, the geometry can be further simplified to reduce the number of vertices. Disabled by default."]
        #[inline]
        pub fn create_convex_collision(&mut self,) {
            self.create_convex_collision_ex() . done()
        }
        #[doc = "This helper creates a [`StaticBody3D`][crate::classes::StaticBody3D] child node with a [`ConvexPolygonShape3D`][crate::classes::ConvexPolygonShape3D] collision shape calculated from the mesh geometry. It's mainly used for testing.\n\nIf `clean` is `true` (default), duplicate and interior vertices are removed automatically. You can set it to `false` to make the process faster if not needed.\n\nIf `simplify` is `true`, the geometry can be further simplified to reduce the number of vertices. Disabled by default."]
        #[inline]
        pub fn create_convex_collision_ex < 'ex > (&'ex mut self,) -> ExCreateConvexCollision < 'ex > {
            ExCreateConvexCollision::new(self,)
        }
        #[doc = "This helper creates a [`StaticBody3D`][crate::classes::StaticBody3D] child node with multiple [`ConvexPolygonShape3D`][crate::classes::ConvexPolygonShape3D] collision shapes calculated from the mesh geometry via convex decomposition. The convex decomposition operation can be controlled with parameters from the optional `settings`."]
        pub(crate) fn create_multiple_convex_collisions_full(&mut self, settings: CowArg < Option < Gd < crate::classes::MeshConvexDecompositionSettings > > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::MeshConvexDecompositionSettings > > >,);
            let args = (settings,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "create_multiple_convex_collisions", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_multiple_convex_collisions_ex`][Self::create_multiple_convex_collisions_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "This helper creates a [`StaticBody3D`][crate::classes::StaticBody3D] child node with multiple [`ConvexPolygonShape3D`][crate::classes::ConvexPolygonShape3D] collision shapes calculated from the mesh geometry via convex decomposition. The convex decomposition operation can be controlled with parameters from the optional `settings`."]
        #[inline]
        pub fn create_multiple_convex_collisions(&mut self,) {
            self.create_multiple_convex_collisions_ex() . done()
        }
        #[doc = "This helper creates a [`StaticBody3D`][crate::classes::StaticBody3D] child node with multiple [`ConvexPolygonShape3D`][crate::classes::ConvexPolygonShape3D] collision shapes calculated from the mesh geometry via convex decomposition. The convex decomposition operation can be controlled with parameters from the optional `settings`."]
        #[inline]
        pub fn create_multiple_convex_collisions_ex < 'ex > (&'ex mut self,) -> ExCreateMultipleConvexCollisions < 'ex > {
            ExCreateMultipleConvexCollisions::new(self,)
        }
        #[doc = "Returns the number of blend shapes available. Produces an error if \\[member mesh] is `null`."]
        pub fn get_blend_shape_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_blend_shape_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the blend shape with the given `name`. Returns `-1` if no blend shape with this name exists, including when \\[member mesh] is `null`."]
        pub fn find_blend_shape_by_name(&mut self, name: impl AsArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "find_blend_shape_by_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the blend shape at the given `blend_shape_idx`. Returns `0.0` and produces an error if \\[member mesh] is `null` or doesn't have a blend shape at that index."]
        pub fn get_blend_shape_value(&self, blend_shape_idx: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (blend_shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "get_blend_shape_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of the blend shape at `blend_shape_idx` to `value`. Produces an error if \\[member mesh] is `null` or doesn't have a blend shape at that index."]
        pub fn set_blend_shape_value(&mut self, blend_shape_idx: i32, value: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (blend_shape_idx, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3615usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "set_blend_shape_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This helper creates a `MeshInstance3D` child node with gizmos at every vertex calculated from the mesh geometry. It's mainly used for testing."]
        pub fn create_debug_tangents(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3616usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "create_debug_tangents", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Takes a snapshot from the current [`ArrayMesh`][crate::classes::ArrayMesh] with all blend shapes applied according to their current weights and bakes it to the provided `existing` mesh. If no `existing` mesh is provided a new [`ArrayMesh`][crate::classes::ArrayMesh] is created, baked and returned. Mesh surface materials are not copied.\n\n**Performance:** [`Mesh`][crate::classes::Mesh] data needs to be received from the GPU, stalling the [`RenderingServer`][crate::classes::RenderingServer] in the process."]
        pub(crate) fn bake_mesh_from_current_blend_shape_mix_full(&mut self, existing: CowArg < Option < Gd < crate::classes::ArrayMesh > > >,) -> Option < Gd < crate::classes::ArrayMesh > > {
            type CallRet = Option < Gd < crate::classes::ArrayMesh > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ArrayMesh > > >,);
            let args = (existing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3617usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "bake_mesh_from_current_blend_shape_mix", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`bake_mesh_from_current_blend_shape_mix_ex`][Self::bake_mesh_from_current_blend_shape_mix_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Takes a snapshot from the current [`ArrayMesh`][crate::classes::ArrayMesh] with all blend shapes applied according to their current weights and bakes it to the provided `existing` mesh. If no `existing` mesh is provided a new [`ArrayMesh`][crate::classes::ArrayMesh] is created, baked and returned. Mesh surface materials are not copied.\n\n**Performance:** [`Mesh`][crate::classes::Mesh] data needs to be received from the GPU, stalling the [`RenderingServer`][crate::classes::RenderingServer] in the process."]
        #[inline]
        pub fn bake_mesh_from_current_blend_shape_mix(&mut self,) -> Option < Gd < crate::classes::ArrayMesh > > {
            self.bake_mesh_from_current_blend_shape_mix_ex() . done()
        }
        #[doc = "Takes a snapshot from the current [`ArrayMesh`][crate::classes::ArrayMesh] with all blend shapes applied according to their current weights and bakes it to the provided `existing` mesh. If no `existing` mesh is provided a new [`ArrayMesh`][crate::classes::ArrayMesh] is created, baked and returned. Mesh surface materials are not copied.\n\n**Performance:** [`Mesh`][crate::classes::Mesh] data needs to be received from the GPU, stalling the [`RenderingServer`][crate::classes::RenderingServer] in the process."]
        #[inline]
        pub fn bake_mesh_from_current_blend_shape_mix_ex < 'ex > (&'ex mut self,) -> ExBakeMeshFromCurrentBlendShapeMix < 'ex > {
            ExBakeMeshFromCurrentBlendShapeMix::new(self,)
        }
        #[doc = "Takes a snapshot of the current animated skeleton pose of the skinned mesh and bakes it to the provided `existing` mesh. If no `existing` mesh is provided a new [`ArrayMesh`][crate::classes::ArrayMesh] is created, baked, and returned. Requires a skeleton with a registered skin to work. Blendshapes are ignored. Mesh surface materials are not copied.\n\n**Performance:** [`Mesh`][crate::classes::Mesh] data needs to be retrieved from the GPU, stalling the [`RenderingServer`][crate::classes::RenderingServer] in the process."]
        pub(crate) fn bake_mesh_from_current_skeleton_pose_full(&mut self, existing: CowArg < Option < Gd < crate::classes::ArrayMesh > > >,) -> Option < Gd < crate::classes::ArrayMesh > > {
            type CallRet = Option < Gd < crate::classes::ArrayMesh > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ArrayMesh > > >,);
            let args = (existing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3618usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshInstance3D", "bake_mesh_from_current_skeleton_pose", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`bake_mesh_from_current_skeleton_pose_ex`][Self::bake_mesh_from_current_skeleton_pose_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Takes a snapshot of the current animated skeleton pose of the skinned mesh and bakes it to the provided `existing` mesh. If no `existing` mesh is provided a new [`ArrayMesh`][crate::classes::ArrayMesh] is created, baked, and returned. Requires a skeleton with a registered skin to work. Blendshapes are ignored. Mesh surface materials are not copied.\n\n**Performance:** [`Mesh`][crate::classes::Mesh] data needs to be retrieved from the GPU, stalling the [`RenderingServer`][crate::classes::RenderingServer] in the process."]
        #[inline]
        pub fn bake_mesh_from_current_skeleton_pose(&mut self,) -> Option < Gd < crate::classes::ArrayMesh > > {
            self.bake_mesh_from_current_skeleton_pose_ex() . done()
        }
        #[doc = "Takes a snapshot of the current animated skeleton pose of the skinned mesh and bakes it to the provided `existing` mesh. If no `existing` mesh is provided a new [`ArrayMesh`][crate::classes::ArrayMesh] is created, baked, and returned. Requires a skeleton with a registered skin to work. Blendshapes are ignored. Mesh surface materials are not copied.\n\n**Performance:** [`Mesh`][crate::classes::Mesh] data needs to be retrieved from the GPU, stalling the [`RenderingServer`][crate::classes::RenderingServer] in the process."]
        #[inline]
        pub fn bake_mesh_from_current_skeleton_pose_ex < 'ex > (&'ex mut self,) -> ExBakeMeshFromCurrentSkeletonPose < 'ex > {
            ExBakeMeshFromCurrentSkeletonPose::new(self,)
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
    impl crate::obj::GodotClass for MeshInstance3D {
        type Base = crate::classes::GeometryInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MeshInstance3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MeshInstance3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for MeshInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for MeshInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for MeshInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for MeshInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MeshInstance3D {
        
    }
    impl crate::obj::cap::GodotDefault for MeshInstance3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MeshInstance3D {
        type Target = crate::classes::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MeshInstance3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MeshInstance3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MeshInstance3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MeshInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GeometryInstance3D > for $Class {
                
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
#[doc = "Default-param extender for [`MeshInstance3D::create_convex_collision_ex`][super::MeshInstance3D::create_convex_collision_ex]."]
#[must_use]
pub struct ExCreateConvexCollision < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::MeshInstance3D, clean: bool, simplify: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateConvexCollision < 'ex > {
    fn new(surround_object: &'ex mut re_export::MeshInstance3D,) -> Self {
        let clean = true;
        let simplify = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, clean: clean, simplify: simplify,
        }
    }
    #[inline]
    pub fn clean(self, clean: bool) -> Self {
        Self {
            clean: clean, .. self
        }
    }
    #[inline]
    pub fn simplify(self, simplify: bool) -> Self {
        Self {
            simplify: simplify, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, clean, simplify,
        }
        = self;
        re_export::MeshInstance3D::create_convex_collision_full(surround_object, clean, simplify,)
    }
}
#[doc = "Default-param extender for [`MeshInstance3D::create_multiple_convex_collisions_ex`][super::MeshInstance3D::create_multiple_convex_collisions_ex]."]
#[must_use]
pub struct ExCreateMultipleConvexCollisions < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::MeshInstance3D, settings: CowArg < 'ex, Option < Gd < crate::classes::MeshConvexDecompositionSettings > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateMultipleConvexCollisions < 'ex > {
    fn new(surround_object: &'ex mut re_export::MeshInstance3D,) -> Self {
        let settings = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, settings: settings.into_arg(),
        }
    }
    #[inline]
    pub fn settings(self, settings: impl AsArg < Option < Gd < crate::classes::MeshConvexDecompositionSettings >> > + 'ex) -> Self {
        Self {
            settings: settings.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, settings,
        }
        = self;
        re_export::MeshInstance3D::create_multiple_convex_collisions_full(surround_object, settings,)
    }
}
#[doc = "Default-param extender for [`MeshInstance3D::bake_mesh_from_current_blend_shape_mix_ex`][super::MeshInstance3D::bake_mesh_from_current_blend_shape_mix_ex]."]
#[must_use]
pub struct ExBakeMeshFromCurrentBlendShapeMix < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::MeshInstance3D, existing: CowArg < 'ex, Option < Gd < crate::classes::ArrayMesh > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBakeMeshFromCurrentBlendShapeMix < 'ex > {
    fn new(surround_object: &'ex mut re_export::MeshInstance3D,) -> Self {
        let existing = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, existing: existing.into_arg(),
        }
    }
    #[inline]
    pub fn existing(self, existing: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> > + 'ex) -> Self {
        Self {
            existing: existing.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::ArrayMesh > > {
        let Self {
            _phantom, surround_object, existing,
        }
        = self;
        re_export::MeshInstance3D::bake_mesh_from_current_blend_shape_mix_full(surround_object, existing,)
    }
}
#[doc = "Default-param extender for [`MeshInstance3D::bake_mesh_from_current_skeleton_pose_ex`][super::MeshInstance3D::bake_mesh_from_current_skeleton_pose_ex]."]
#[must_use]
pub struct ExBakeMeshFromCurrentSkeletonPose < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::MeshInstance3D, existing: CowArg < 'ex, Option < Gd < crate::classes::ArrayMesh > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBakeMeshFromCurrentSkeletonPose < 'ex > {
    fn new(surround_object: &'ex mut re_export::MeshInstance3D,) -> Self {
        let existing = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, existing: existing.into_arg(),
        }
    }
    #[inline]
    pub fn existing(self, existing: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> > + 'ex) -> Self {
        Self {
            existing: existing.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::ArrayMesh > > {
        let Self {
            _phantom, surround_object, existing,
        }
        = self;
        re_export::MeshInstance3D::bake_mesh_from_current_skeleton_pose_full(surround_object, existing,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MeshInstance3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for MeshInstance3D {
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
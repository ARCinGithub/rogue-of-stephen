#![doc = "Sidecar module for class [`GridMap`][crate::classes::GridMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GridMap` enums](https://docs.godotengine.org/en/stable/classes/class_gridmap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GridMap`.\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`grid_map`][crate::classes::grid_map]: sidecar module with related enum/flag types\n* [`IGridMap`][crate::classes::IGridMap]: virtual methods\n* [`SignalsOfGridMap`][crate::classes::grid_map::SignalsOfGridMap]: signal collection\n\n\nSee also [Godot docs for `GridMap`](https://docs.godotengine.org/en/stable/classes/class_gridmap.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`GridMap::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nGridMap lets you place meshes on a grid interactively. It works both from the editor and from scripts, which can help you create in-game level editors.\n\nGridMaps use a [`MeshLibrary`][crate::classes::MeshLibrary] which contains a list of tiles. Each tile is a mesh with materials plus optional collision and navigation shapes.\n\nA GridMap contains a collection of cells. Each grid cell refers to a tile in the [`MeshLibrary`][crate::classes::MeshLibrary]. All cells in the map have the same dimensions.\n\nInternally, a GridMap is split into a sparse collection of octants for efficient rendering and physics processing. Every octant has the same dimensions and can contain several cells.\n\n**Note:** GridMap doesn't extend [`VisualInstance3D`][crate::classes::VisualInstance3D] and therefore can't be hidden or cull masked based on \\[member VisualInstance3D.layers]. If you make a light not affect the first layer, the whole GridMap won't be lit by the light in question."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GridMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GridMap`][crate::classes::GridMap].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GridMap` methods](https://docs.godotengine.org/en/stable/classes/class_gridmap.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGridMap: crate::obj::GodotClass < Base = GridMap > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GridMap {
        pub fn set_collision_layer(&mut self, layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Based on `value`, enables or disables the specified layer in the \\[member collision_mask], given a `layer_number` between 1 and 32."]
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether or not the specified layer of the \\[member collision_mask] is enabled, given a `layer_number` between 1 and 32."]
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Based on `value`, enables or disables the specified layer in the \\[member collision_layer], given a `layer_number` between 1 and 32."]
        pub fn set_collision_layer_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether or not the specified layer of the \\[member collision_layer] is enabled, given a `layer_number` between 1 and 32."]
        pub fn get_collision_layer_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_priority(&mut self, priority: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_priority(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_material(&mut self, material: impl AsArg < Option < Gd < crate::classes::PhysicsMaterial >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PhysicsMaterial > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_physics_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_material(&self,) -> Option < Gd < crate::classes::PhysicsMaterial > > {
            type CallRet = Option < Gd < crate::classes::PhysicsMaterial > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_physics_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bake_navigation(&mut self, bake_navigation: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (bake_navigation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_bake_navigation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_baking_navigation(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "is_baking_navigation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`RID`][crate::builtin::Rid] of the navigation map this GridMap node should use for its cell baked navigation meshes."]
        pub fn set_navigation_map(&mut self, navigation_map: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (navigation_map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the navigation map this GridMap node uses for its cell baked navigation meshes.\n\nThis function returns always the map set on the GridMap node and not the map on the NavigationServer. If the map is changed directly with the NavigationServer API the GridMap node will not be aware of the map change."]
        pub fn get_navigation_map(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mesh_library(&mut self, mesh_library: impl AsArg < Option < Gd < crate::classes::MeshLibrary >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::MeshLibrary > > >,);
            let args = (mesh_library.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_mesh_library", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mesh_library(&self,) -> Option < Gd < crate::classes::MeshLibrary > > {
            type CallRet = Option < Gd < crate::classes::MeshLibrary > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_mesh_library", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cell_size(&mut self, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_cell_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cell_size(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cell_scale(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9806usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_cell_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cell_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9807usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_octant_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9808usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_octant_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_octant_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9809usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_octant_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the mesh index for the cell referenced by its grid coordinates.\n\nA negative item index such as `INVALID_CELL_ITEM` will clear the cell.\n\nOptionally, the item's orientation can be passed. For valid orientation values, see [`get_orthogonal_index_from_basis`][`crate::classes::GridMap::get_orthogonal_index_from_basis`]."]
        pub(crate) fn set_cell_item_full(&mut self, position: Vector3i, item: i32, orientation: i32,) {
            type CallRet = ();
            type CallParams = (Vector3i, i32, i32,);
            let args = (position, item, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_cell_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_cell_item_ex`][Self::set_cell_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the mesh index for the cell referenced by its grid coordinates.\n\nA negative item index such as `INVALID_CELL_ITEM` will clear the cell.\n\nOptionally, the item's orientation can be passed. For valid orientation values, see [`get_orthogonal_index_from_basis`][`crate::classes::GridMap::get_orthogonal_index_from_basis`]."]
        #[inline]
        pub fn set_cell_item(&mut self, position: Vector3i, item: i32,) {
            self.set_cell_item_ex(position, item,) . done()
        }
        #[doc = "Sets the mesh index for the cell referenced by its grid coordinates.\n\nA negative item index such as `INVALID_CELL_ITEM` will clear the cell.\n\nOptionally, the item's orientation can be passed. For valid orientation values, see [`get_orthogonal_index_from_basis`][`crate::classes::GridMap::get_orthogonal_index_from_basis`]."]
        #[inline]
        pub fn set_cell_item_ex < 'ex > (&'ex mut self, position: Vector3i, item: i32,) -> ExSetCellItem < 'ex > {
            ExSetCellItem::new(self, position, item,)
        }
        #[doc = "The [`MeshLibrary`][crate::classes::MeshLibrary] item index located at the given grid coordinates. If the cell is empty, `INVALID_CELL_ITEM` will be returned."]
        pub fn get_cell_item(&self, position: Vector3i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector3i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "The orientation of the cell at the given grid coordinates. `-1` is returned if the cell is empty."]
        pub fn get_cell_item_orientation(&self, position: Vector3i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector3i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9812usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_item_orientation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the basis that gives the specified cell its orientation."]
        pub fn get_cell_item_basis(&self, position: Vector3i,) -> Basis {
            type CallRet = Basis;
            type CallParams = (Vector3i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9813usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_item_basis", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns one of 24 possible rotations that lie along the vectors (x,y,z) with each component being either -1, 0, or 1. For further details, refer to the Godot source code."]
        pub fn get_basis_with_orthogonal_index(&self, index: i32,) -> Basis {
            type CallRet = Basis;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9814usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_basis_with_orthogonal_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This function considers a discretization of rotations into 24 points on unit sphere, lying along the vectors (x,y,z) with each component being either -1, 0, or 1, and returns the index (in the range from 0 to 23) of the point best representing the orientation of the object. For further details, refer to the Godot source code."]
        pub fn get_orthogonal_index_from_basis(&self, basis: Basis,) -> i32 {
            type CallRet = i32;
            type CallParams = (Basis,);
            let args = (basis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9815usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_orthogonal_index_from_basis", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the map coordinates of the cell containing the given `local_position`. If `local_position` is in global coordinates, consider using [`to_local`][`crate::classes::Node3D::to_local`] before passing it to this method. See also [`map_to_local`][`crate::classes::GridMap::map_to_local`]."]
        pub fn local_to_map(&self, local_position: Vector3,) -> Vector3i {
            type CallRet = Vector3i;
            type CallParams = (Vector3,);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9816usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "local_to_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the position of a grid cell in the GridMap's local coordinate space. To convert the returned value into global coordinates, use [`to_global`][`crate::classes::Node3D::to_global`]. See also [`local_to_map`][`crate::classes::GridMap::local_to_map`]."]
        pub fn map_to_local(&self, map_position: Vector3i,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector3i,);
            let args = (map_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "map_to_local", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This method does nothing."]
        pub fn resource_changed(&mut self, resource: impl AsArg < Option < Gd < crate::classes::Resource >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >,);
            let args = (resource.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "resource_changed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_x(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_center_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_x(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_center_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_y(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_center_y", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_y(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_center_y", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_z(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_center_z", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_z(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_center_z", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clear all cells."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of [`Vector3`][crate::builtin::Vector3] with the non-empty cell coordinates in the grid map."]
        pub fn get_used_cells(&self,) -> Array < Vector3i > {
            type CallRet = Array < Vector3i >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_used_cells", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all cells with the given item index specified in `item`."]
        pub fn get_used_cells_by_item(&self, item: i32,) -> Array < Vector3i > {
            type CallRet = Array < Vector3i >;
            type CallParams = (i32,);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_used_cells_by_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of [`Transform3D`][crate::builtin::Transform3D] and [`Mesh`][crate::classes::Mesh] references corresponding to the non-empty cells in the grid. The transforms are specified in local space. Even indices contain [`Transform3D`][crate::builtin::Transform3D]s, while odd indices contain [`Mesh`][crate::classes::Mesh]es related to the [`Transform3D`][crate::builtin::Transform3D] in the index preceding it."]
        pub fn get_meshes(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_meshes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of [`ArrayMesh`][crate::classes::ArrayMesh]es and [`Transform3D`][crate::builtin::Transform3D] references of all bake meshes that exist within the current GridMap. Even indices contain [`ArrayMesh`][crate::classes::ArrayMesh]es, while odd indices contain [`Transform3D`][crate::builtin::Transform3D]s that are always equal to `Transform3D.IDENTITY`.\n\nThis method relies on the output of [`make_baked_meshes`][`crate::classes::GridMap::make_baked_meshes`], which will be called with `gen_lightmap_uv` set to `true` and `lightmap_uv_texel_size` set to `0.1` if it hasn't been called yet."]
        pub fn get_bake_meshes(&mut self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_bake_meshes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns [`RID`][crate::builtin::Rid] of a baked mesh with the given `idx`."]
        pub fn get_bake_mesh_instance(&self, idx: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_bake_mesh_instance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all baked meshes. See [`make_baked_meshes`][`crate::classes::GridMap::make_baked_meshes`]."]
        pub fn clear_baked_meshes(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "clear_baked_meshes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Generates a baked mesh that represents all meshes in the assigned [`MeshLibrary`][crate::classes::MeshLibrary] for use with [`LightmapGI`][crate::classes::LightmapGi]. If `gen_lightmap_uv` is `true`, UV2 data will be generated for each mesh currently used in the `GridMap`. Otherwise, only meshes that already have UV2 data present will be able to use baked lightmaps. When generating UV2, `lightmap_uv_texel_size` controls the texel density for lightmaps, with lower values resulting in more detailed lightmaps. `lightmap_uv_texel_size` is ignored if `gen_lightmap_uv` is `false`. See also [`get_bake_meshes`][`crate::classes::GridMap::get_bake_meshes`], which relies on the output of this method.\n\n**Note:** Calling this method will not actually bake lightmaps, as lightmap baking is performed using the [`LightmapGI`][crate::classes::LightmapGi] node."]
        pub(crate) fn make_baked_meshes_full(&mut self, gen_lightmap_uv: bool, lightmap_uv_texel_size: f32,) {
            type CallRet = ();
            type CallParams = (bool, f32,);
            let args = (gen_lightmap_uv, lightmap_uv_texel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "make_baked_meshes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`make_baked_meshes_ex`][Self::make_baked_meshes_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Generates a baked mesh that represents all meshes in the assigned [`MeshLibrary`][crate::classes::MeshLibrary] for use with [`LightmapGI`][crate::classes::LightmapGi]. If `gen_lightmap_uv` is `true`, UV2 data will be generated for each mesh currently used in the `GridMap`. Otherwise, only meshes that already have UV2 data present will be able to use baked lightmaps. When generating UV2, `lightmap_uv_texel_size` controls the texel density for lightmaps, with lower values resulting in more detailed lightmaps. `lightmap_uv_texel_size` is ignored if `gen_lightmap_uv` is `false`. See also [`get_bake_meshes`][`crate::classes::GridMap::get_bake_meshes`], which relies on the output of this method.\n\n**Note:** Calling this method will not actually bake lightmaps, as lightmap baking is performed using the [`LightmapGI`][crate::classes::LightmapGi] node."]
        #[inline]
        pub fn make_baked_meshes(&mut self,) {
            self.make_baked_meshes_ex() . done()
        }
        #[doc = "Generates a baked mesh that represents all meshes in the assigned [`MeshLibrary`][crate::classes::MeshLibrary] for use with [`LightmapGI`][crate::classes::LightmapGi]. If `gen_lightmap_uv` is `true`, UV2 data will be generated for each mesh currently used in the `GridMap`. Otherwise, only meshes that already have UV2 data present will be able to use baked lightmaps. When generating UV2, `lightmap_uv_texel_size` controls the texel density for lightmaps, with lower values resulting in more detailed lightmaps. `lightmap_uv_texel_size` is ignored if `gen_lightmap_uv` is `false`. See also [`get_bake_meshes`][`crate::classes::GridMap::get_bake_meshes`], which relies on the output of this method.\n\n**Note:** Calling this method will not actually bake lightmaps, as lightmap baking is performed using the [`LightmapGI`][crate::classes::LightmapGi] node."]
        #[inline]
        pub fn make_baked_meshes_ex < 'ex > (&'ex mut self,) -> ExMakeBakedMeshes < 'ex > {
            ExMakeBakedMeshes::new(self,)
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
        pub const INVALID_CELL_ITEM: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for GridMap {
        type Base = crate::classes::Node3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GridMap"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GridMap {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for GridMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for GridMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GridMap {
        
    }
    impl crate::obj::cap::GodotDefault for GridMap {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GridMap {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GridMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GridMap`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GridMap__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GridMap > for $Class {
                
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
#[doc = "Default-param extender for [`GridMap::set_cell_item_ex`][super::GridMap::set_cell_item_ex]."]
#[must_use]
pub struct ExSetCellItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::GridMap, position: Vector3i, item: i32, orientation: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetCellItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::GridMap, position: Vector3i, item: i32,) -> Self {
        let orientation = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, item: item, orientation: orientation,
        }
    }
    #[inline]
    pub fn orientation(self, orientation: i32) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, item, orientation,
        }
        = self;
        re_export::GridMap::set_cell_item_full(surround_object, position, item, orientation,)
    }
}
#[doc = "Default-param extender for [`GridMap::make_baked_meshes_ex`][super::GridMap::make_baked_meshes_ex]."]
#[must_use]
pub struct ExMakeBakedMeshes < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::GridMap, gen_lightmap_uv: bool, lightmap_uv_texel_size: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExMakeBakedMeshes < 'ex > {
    fn new(surround_object: &'ex mut re_export::GridMap,) -> Self {
        let gen_lightmap_uv = false;
        let lightmap_uv_texel_size = 0.1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, gen_lightmap_uv: gen_lightmap_uv, lightmap_uv_texel_size: lightmap_uv_texel_size,
        }
    }
    #[inline]
    pub fn gen_lightmap_uv(self, gen_lightmap_uv: bool) -> Self {
        Self {
            gen_lightmap_uv: gen_lightmap_uv, .. self
        }
    }
    #[inline]
    pub fn lightmap_uv_texel_size(self, lightmap_uv_texel_size: f32) -> Self {
        Self {
            lightmap_uv_texel_size: lightmap_uv_texel_size, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, gen_lightmap_uv, lightmap_uv_texel_size,
        }
        = self;
        re_export::GridMap::make_baked_meshes_full(surround_object, gen_lightmap_uv, lightmap_uv_texel_size,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GridMap;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`GridMap`][crate::classes::GridMap] class."]
    pub struct SignalsOfGridMap < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfGridMap < 'c, C > {
        #[doc = "Signature: `(cell_size: Vector3)`"]
        pub fn cell_size_changed(&mut self) -> SigCellSizeChanged < 'c, C > {
            SigCellSizeChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "cell_size_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn changed(&mut self) -> SigChanged < 'c, C > {
            SigChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "changed")
            }
        }
    }
    type TypedSigCellSizeChanged < 'c, C > = TypedSignal < 'c, C, (Vector3,) >;
    pub struct SigCellSizeChanged < 'c, C: WithSignals > {
        typed: TypedSigCellSizeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCellSizeChanged < 'c, C > {
        pub fn emit(&mut self, cell_size: Vector3,) {
            self.typed.emit_tuple((cell_size,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCellSizeChanged < 'c, C > {
        type Target = TypedSigCellSizeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCellSizeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigChanged < 'c, C: WithSignals > {
        typed: TypedSigChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigChanged < 'c, C > {
        type Target = TypedSigChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for GridMap {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfGridMap < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfGridMap < 'c, C > {
        type Target = < < GridMap as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = GridMap;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfGridMap < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = GridMap;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
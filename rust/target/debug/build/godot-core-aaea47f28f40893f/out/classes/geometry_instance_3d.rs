#![doc = "Sidecar module for class [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GeometryInstance3D` enums](https://docs.godotengine.org/en/stable/classes/class_geometryinstance3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GeometryInstance3D`.\n\nInherits [`VisualInstance3D`][crate::classes::VisualInstance3D].\n\nRelated symbols:\n\n* [`geometry_instance_3d`][crate::classes::geometry_instance_3d]: sidecar module with related enum/flag types\n* [`IGeometryInstance3D`][crate::classes::IGeometryInstance3D]: virtual methods\n\n\nSee also [Godot docs for `GeometryInstance3D`](https://docs.godotengine.org/en/stable/classes/class_geometryinstance3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`GeometryInstance3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nBase node for geometry-based visual instances. Shares some common functionality like visibility and custom materials."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GeometryInstance3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GeometryInstance3D` methods](https://docs.godotengine.org/en/stable/classes/class_geometryinstance3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGeometryInstance3D: crate::obj::GodotClass < Base = GeometryInstance3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GeometryInstance3D {
        pub fn set_material_override(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_material_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_material_override(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_material_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_material_overlay(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_material_overlay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_material_overlay(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_material_overlay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cast_shadows_setting(&mut self, shadow_casting_setting: crate::classes::geometry_instance_3d::ShadowCastingSetting,) {
            type CallRet = ();
            type CallParams = (crate::classes::geometry_instance_3d::ShadowCastingSetting,);
            let args = (shadow_casting_setting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_cast_shadows_setting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cast_shadows_setting(&self,) -> crate::classes::geometry_instance_3d::ShadowCastingSetting {
            type CallRet = crate::classes::geometry_instance_3d::ShadowCastingSetting;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_cast_shadows_setting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lod_bias(&mut self, bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_lod_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lod_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_lod_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transparency(&mut self, transparency: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (transparency,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_transparency", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transparency(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_transparency", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_range_end_margin(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_end_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_range_end_margin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_end_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_range_end(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_range_end(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_range_begin_margin(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_begin_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_range_begin_margin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_begin_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_range_begin(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_range_begin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_range_fade_mode(&mut self, mode: crate::classes::geometry_instance_3d::VisibilityRangeFadeMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::geometry_instance_3d::VisibilityRangeFadeMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_fade_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_range_fade_mode(&self,) -> crate::classes::geometry_instance_3d::VisibilityRangeFadeMode {
            type CallRet = crate::classes::geometry_instance_3d::VisibilityRangeFadeMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_fade_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set the value of a shader uniform for this instance only ([per-instance uniform]($DOCS_URL/tutorials/shaders/shader_reference/shading_language.html#per-instance-uniforms)). See also [`set_shader_parameter`][`crate::classes::ShaderMaterial::set_shader_parameter`] to assign a uniform on all instances using the same [`ShaderMaterial`][crate::classes::ShaderMaterial].\n\n**Note:** For a shader uniform to be assignable on a per-instance basis, it _must_ be defined with `instance uniform ...` rather than `uniform ...` in the shader code.\n\n**Note:** `name` is case-sensitive and must match the name of the uniform in the code exactly (not the capitalized name in the inspector).\n\n**Note:** Per-instance shader uniforms are only available in Spatial and CanvasItem shaders, but not for Fog, Sky, or Particles shaders."]
        pub fn set_instance_shader_parameter(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_instance_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Get the value of a shader parameter as set on this instance."]
        pub fn get_instance_shader_parameter(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_instance_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_extra_cull_margin(&mut self, margin: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_extra_cull_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_extra_cull_margin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_extra_cull_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lightmap_texel_scale(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_lightmap_texel_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lightmap_texel_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_lightmap_texel_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lightmap_scale(&mut self, scale: crate::classes::geometry_instance_3d::LightmapScale,) {
            type CallRet = ();
            type CallParams = (crate::classes::geometry_instance_3d::LightmapScale,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_lightmap_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lightmap_scale(&self,) -> crate::classes::geometry_instance_3d::LightmapScale {
            type CallRet = crate::classes::geometry_instance_3d::LightmapScale;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_lightmap_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gi_mode(&mut self, mode: crate::classes::geometry_instance_3d::GiMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::geometry_instance_3d::GiMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_gi_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gi_mode(&self,) -> crate::classes::geometry_instance_3d::GiMode {
            type CallRet = crate::classes::geometry_instance_3d::GiMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_gi_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ignore_occlusion_culling(&mut self, ignore_culling: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (ignore_culling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_ignore_occlusion_culling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ignoring_occlusion_culling(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "is_ignoring_occlusion_culling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_aabb(&mut self, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Aabb,);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_custom_aabb", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for GeometryInstance3D {
        type Base = crate::classes::VisualInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GeometryInstance3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GeometryInstance3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for GeometryInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for GeometryInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for GeometryInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GeometryInstance3D {
        
    }
    impl crate::obj::cap::GodotDefault for GeometryInstance3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GeometryInstance3D {
        type Target = crate::classes::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GeometryInstance3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GeometryInstance3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GeometryInstance3D__ensure_class_exists {
        ($Class: ident) => {
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShadowCastingSetting {
    ord: i32
}
impl ShadowCastingSetting {
    #[doc(alias = "SHADOW_CASTING_SETTING_OFF")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_OFF`"]
    pub const OFF: ShadowCastingSetting = ShadowCastingSetting {
        ord: 0i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_ON")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_ON`"]
    pub const ON: ShadowCastingSetting = ShadowCastingSetting {
        ord: 1i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_DOUBLE_SIDED")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_DOUBLE_SIDED`"]
    pub const DOUBLE_SIDED: ShadowCastingSetting = ShadowCastingSetting {
        ord: 2i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_SHADOWS_ONLY")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_SHADOWS_ONLY`"]
    pub const SHADOWS_ONLY: ShadowCastingSetting = ShadowCastingSetting {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ShadowCastingSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShadowCastingSetting") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShadowCastingSetting {
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
            Self::OFF => "OFF", Self::ON => "ON", Self::DOUBLE_SIDED => "DOUBLE_SIDED", Self::SHADOWS_ONLY => "SHADOWS_ONLY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShadowCastingSetting::OFF, ShadowCastingSetting::ON, ShadowCastingSetting::DOUBLE_SIDED, ShadowCastingSetting::SHADOWS_ONLY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShadowCastingSetting >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OFF", "SHADOW_CASTING_SETTING_OFF", ShadowCastingSetting::OFF), crate::meta::inspect::EnumConstant::new("ON", "SHADOW_CASTING_SETTING_ON", ShadowCastingSetting::ON), crate::meta::inspect::EnumConstant::new("DOUBLE_SIDED", "SHADOW_CASTING_SETTING_DOUBLE_SIDED", ShadowCastingSetting::DOUBLE_SIDED), crate::meta::inspect::EnumConstant::new("SHADOWS_ONLY", "SHADOW_CASTING_SETTING_SHADOWS_ONLY", ShadowCastingSetting::SHADOWS_ONLY)]
        }
    }
}
impl crate::meta::GodotConvert for ShadowCastingSetting {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Shadow Casting Setting Off", 0i64), EnumeratorShape::new_int("Shadow Casting Setting On", 1i64), EnumeratorShape::new_int("Shadow Casting Setting Double Sided", 2i64), EnumeratorShape::new_int("Shadow Casting Setting Shadows Only", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("GeometryInstance3D.ShadowCastingSetting")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ShadowCastingSetting {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShadowCastingSetting {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ShadowCastingSetting {
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
impl crate::registry::property::Export for ShadowCastingSetting {
    
}
impl crate::meta::Element for ShadowCastingSetting {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `GIMode`."]
pub struct GiMode {
    ord: i32
}
impl GiMode {
    #[doc(alias = "GI_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `GI_MODE_DISABLED`"]
    pub const DISABLED: GiMode = GiMode {
        ord: 0i32
    };
    #[doc(alias = "GI_MODE_STATIC")]
    #[doc = "Godot enumerator name: `GI_MODE_STATIC`"]
    pub const STATIC: GiMode = GiMode {
        ord: 1i32
    };
    #[doc(alias = "GI_MODE_DYNAMIC")]
    #[doc = "Godot enumerator name: `GI_MODE_DYNAMIC`"]
    pub const DYNAMIC: GiMode = GiMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for GiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GiMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GiMode {
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
            Self::DISABLED => "DISABLED", Self::STATIC => "STATIC", Self::DYNAMIC => "DYNAMIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GiMode::DISABLED, GiMode::STATIC, GiMode::DYNAMIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GiMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "GI_MODE_DISABLED", GiMode::DISABLED), crate::meta::inspect::EnumConstant::new("STATIC", "GI_MODE_STATIC", GiMode::STATIC), crate::meta::inspect::EnumConstant::new("DYNAMIC", "GI_MODE_DYNAMIC", GiMode::DYNAMIC)]
        }
    }
}
impl crate::meta::GodotConvert for GiMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Gi Mode Disabled", 0i64), EnumeratorShape::new_int("Gi Mode Static", 1i64), EnumeratorShape::new_int("Gi Mode Dynamic", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("GeometryInstance3D.GIMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for GiMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GiMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for GiMode {
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
impl crate::registry::property::Export for GiMode {
    
}
impl crate::meta::Element for GiMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightmapScale {
    ord: i32
}
impl LightmapScale {
    #[doc(alias = "LIGHTMAP_SCALE_1X")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_1X`"]
    pub const SCALE_1X: LightmapScale = LightmapScale {
        ord: 0i32
    };
    #[doc(alias = "LIGHTMAP_SCALE_2X")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_2X`"]
    pub const SCALE_2X: LightmapScale = LightmapScale {
        ord: 1i32
    };
    #[doc(alias = "LIGHTMAP_SCALE_4X")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_4X`"]
    pub const SCALE_4X: LightmapScale = LightmapScale {
        ord: 2i32
    };
    #[doc(alias = "LIGHTMAP_SCALE_8X")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_8X`"]
    pub const SCALE_8X: LightmapScale = LightmapScale {
        ord: 3i32
    };
    #[doc(alias = "LIGHTMAP_SCALE_MAX")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_MAX`"]
    pub const MAX: LightmapScale = LightmapScale {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for LightmapScale {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightmapScale") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightmapScale {
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
            Self::SCALE_1X => "SCALE_1X", Self::SCALE_2X => "SCALE_2X", Self::SCALE_4X => "SCALE_4X", Self::SCALE_8X => "SCALE_8X", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightmapScale::SCALE_1X, LightmapScale::SCALE_2X, LightmapScale::SCALE_4X, LightmapScale::SCALE_8X]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightmapScale >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALE_1X", "LIGHTMAP_SCALE_1X", LightmapScale::SCALE_1X), crate::meta::inspect::EnumConstant::new("SCALE_2X", "LIGHTMAP_SCALE_2X", LightmapScale::SCALE_2X), crate::meta::inspect::EnumConstant::new("SCALE_4X", "LIGHTMAP_SCALE_4X", LightmapScale::SCALE_4X), crate::meta::inspect::EnumConstant::new("SCALE_8X", "LIGHTMAP_SCALE_8X", LightmapScale::SCALE_8X), crate::meta::inspect::EnumConstant::new("MAX", "LIGHTMAP_SCALE_MAX", LightmapScale::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for LightmapScale {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for LightmapScale {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Lightmap Scale 1x", 0i64), EnumeratorShape::new_int("Lightmap Scale 2x", 1i64), EnumeratorShape::new_int("Lightmap Scale 4x", 2i64), EnumeratorShape::new_int("Lightmap Scale 8x", 3i64), EnumeratorShape::new_int("Lightmap Scale Max", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("GeometryInstance3D.LightmapScale")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LightmapScale {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightmapScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LightmapScale {
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
impl crate::registry::property::Export for LightmapScale {
    
}
impl crate::meta::Element for LightmapScale {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibilityRangeFadeMode {
    ord: i32
}
impl VisibilityRangeFadeMode {
    #[doc(alias = "VISIBILITY_RANGE_FADE_DISABLED")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_DISABLED`"]
    pub const DISABLED: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 0i32
    };
    #[doc(alias = "VISIBILITY_RANGE_FADE_SELF")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_SELF`"]
    pub const SELF: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 1i32
    };
    #[doc(alias = "VISIBILITY_RANGE_FADE_DEPENDENCIES")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_DEPENDENCIES`"]
    pub const DEPENDENCIES: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for VisibilityRangeFadeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibilityRangeFadeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibilityRangeFadeMode {
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
            Self::DISABLED => "DISABLED", Self::SELF => "SELF", Self::DEPENDENCIES => "DEPENDENCIES", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VisibilityRangeFadeMode::DISABLED, VisibilityRangeFadeMode::SELF, VisibilityRangeFadeMode::DEPENDENCIES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VisibilityRangeFadeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VISIBILITY_RANGE_FADE_DISABLED", VisibilityRangeFadeMode::DISABLED), crate::meta::inspect::EnumConstant::new("SELF", "VISIBILITY_RANGE_FADE_SELF", VisibilityRangeFadeMode::SELF), crate::meta::inspect::EnumConstant::new("DEPENDENCIES", "VISIBILITY_RANGE_FADE_DEPENDENCIES", VisibilityRangeFadeMode::DEPENDENCIES)]
        }
    }
}
impl crate::meta::GodotConvert for VisibilityRangeFadeMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Visibility Range Fade Disabled", 0i64), EnumeratorShape::new_int("Visibility Range Fade Self", 1i64), EnumeratorShape::new_int("Visibility Range Fade Dependencies", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("GeometryInstance3D.VisibilityRangeFadeMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for VisibilityRangeFadeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibilityRangeFadeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for VisibilityRangeFadeMode {
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
impl crate::registry::property::Export for VisibilityRangeFadeMode {
    
}
impl crate::meta::Element for VisibilityRangeFadeMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GeometryInstance3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for GeometryInstance3D {
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
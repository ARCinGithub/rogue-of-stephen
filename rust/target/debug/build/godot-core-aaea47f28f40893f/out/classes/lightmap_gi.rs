#![doc = "Sidecar module for class [`LightmapGi`][crate::classes::LightmapGi].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `LightmapGI` enums](https://docs.godotengine.org/en/stable/classes/class_lightmapgi.html#enumerations).\n\n"]
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
    #[doc = "Godot class `LightmapGI`.\n\nInherits [`VisualInstance3D`][crate::classes::VisualInstance3D].\n\nRelated symbols:\n\n* [`lightmap_gi`][crate::classes::lightmap_gi]: sidecar module with related enum/flag types\n* [`ILightmapGi`][crate::classes::ILightmapGi]: virtual methods\n\n\nSee also [Godot docs for `LightmapGI`](https://docs.godotengine.org/en/stable/classes/class_lightmapgi.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`LightmapGi::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nThe `LightmapGI` node is used to compute and store baked lightmaps. Lightmaps are used to provide high-quality indirect lighting with very little light leaking. `LightmapGI` can also provide rough reflections using spherical harmonics if \\[member directional] is enabled. Dynamic objects can receive indirect lighting thanks to _light probes_, which can be automatically placed by setting \\[member generate_probes_subdiv] to a value other than [`GenerateProbes::DISABLED`][`crate::classes::lightmap_gi::GenerateProbes::DISABLED`]. Additional lightmap probes can also be added by creating [`LightmapProbe`][crate::classes::LightmapProbe] nodes. The downside is that lightmaps are fully static and cannot be baked in an exported project. Baking a `LightmapGI` node is also slower compared to [`VoxelGI`][crate::classes::VoxelGi].\n\n**Procedural generation:** Lightmap baking functionality is only available in the editor. This means `LightmapGI` is not suited to procedurally generated or user-built levels. For procedurally generated or user-built levels, use [`VoxelGI`][crate::classes::VoxelGi] or SDFGI instead (see \\[member Environment.sdfgi_enabled]).\n\n**Performance:** `LightmapGI` provides the best possible run-time performance for global illumination. It is suitable for low-end hardware including integrated graphics and mobile devices.\n\n**Note:** Due to how lightmaps work, most properties only have a visible effect once lightmaps are baked again.\n\n**Note:** Lightmap baking on [`CSGShape3D`][crate::classes::CsgShape3D]s and [`PrimitiveMesh`][crate::classes::PrimitiveMesh]es is not supported, as these cannot store UV2 data required for baking.\n\n**Note:** If no custom lightmappers are installed, `LightmapGI` can only be baked from devices that support the Forward+ or Mobile renderers.\n\n**Note:** The `LightmapGI` node only bakes light data for child nodes of its parent. Nodes further up the hierarchy of the scene will not be baked."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct LightmapGi {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`LightmapGi`][crate::classes::LightmapGi].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `LightmapGI` methods](https://docs.godotengine.org/en/stable/classes/class_lightmapgi.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILightmapGi: crate::obj::GodotClass < Base = LightmapGi > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl LightmapGi {
        pub fn set_light_data(&mut self, data: impl AsArg < Option < Gd < crate::classes::LightmapGiData >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::LightmapGiData > > >,);
            let args = (data.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_light_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_light_data(&self,) -> Option < Gd < crate::classes::LightmapGiData > > {
            type CallRet = Option < Gd < crate::classes::LightmapGiData > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_light_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bake_quality(&mut self, bake_quality: crate::classes::lightmap_gi::BakeQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::lightmap_gi::BakeQuality,);
            let args = (bake_quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_bake_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bake_quality(&self,) -> crate::classes::lightmap_gi::BakeQuality {
            type CallRet = crate::classes::lightmap_gi::BakeQuality;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_bake_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bounces(&mut self, bounces: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (bounces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_bounces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bounces(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_bounces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bounce_indirect_energy(&mut self, bounce_indirect_energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bounce_indirect_energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_bounce_indirect_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bounce_indirect_energy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_bounce_indirect_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_generate_probes(&mut self, subdivision: crate::classes::lightmap_gi::GenerateProbes,) {
            type CallRet = ();
            type CallParams = (crate::classes::lightmap_gi::GenerateProbes,);
            let args = (subdivision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_generate_probes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_generate_probes(&self,) -> crate::classes::lightmap_gi::GenerateProbes {
            type CallRet = crate::classes::lightmap_gi::GenerateProbes;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_generate_probes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bias(&mut self, bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_environment_mode(&mut self, mode: crate::classes::lightmap_gi::EnvironmentMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::lightmap_gi::EnvironmentMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_environment_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_environment_mode(&self,) -> crate::classes::lightmap_gi::EnvironmentMode {
            type CallRet = crate::classes::lightmap_gi::EnvironmentMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_environment_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_environment_custom_sky(&mut self, sky: impl AsArg < Option < Gd < crate::classes::Sky >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Sky > > >,);
            let args = (sky.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_environment_custom_sky", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_environment_custom_sky(&self,) -> Option < Gd < crate::classes::Sky > > {
            type CallRet = Option < Gd < crate::classes::Sky > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_environment_custom_sky", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_environment_custom_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_environment_custom_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_environment_custom_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_environment_custom_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_environment_custom_energy(&mut self, energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_environment_custom_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_environment_custom_energy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_environment_custom_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texel_scale(&mut self, texel_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (texel_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_texel_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texel_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_texel_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_texture_size(&mut self, max_texture_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_texture_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_max_texture_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_texture_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_max_texture_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_supersampling_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_supersampling_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_supersampling_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "is_supersampling_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_supersampling_factor(&mut self, factor: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (factor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_supersampling_factor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_supersampling_factor(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_supersampling_factor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_denoiser(&mut self, use_denoiser: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_denoiser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_use_denoiser", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_denoiser(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "is_using_denoiser", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_denoiser_strength(&mut self, denoiser_strength: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (denoiser_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_denoiser_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_denoiser_strength(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_denoiser_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_denoiser_range(&mut self, denoiser_range: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (denoiser_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_denoiser_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_denoiser_range(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_denoiser_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_interior(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_interior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_interior(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "is_interior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_directional(&mut self, directional: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (directional,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_directional", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_directional(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "is_directional", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadowmask_mode(&mut self, mode: crate::classes::lightmap_gi_data::ShadowmaskMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::lightmap_gi_data::ShadowmaskMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_shadowmask_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadowmask_mode(&self,) -> crate::classes::lightmap_gi_data::ShadowmaskMode {
            type CallRet = crate::classes::lightmap_gi_data::ShadowmaskMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_shadowmask_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_texture_for_bounces(&mut self, use_texture_for_bounces: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_texture_for_bounces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_use_texture_for_bounces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_texture_for_bounces(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "is_using_texture_for_bounces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_camera_attributes(&mut self, camera_attributes: impl AsArg < Option < Gd < crate::classes::CameraAttributes >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::CameraAttributes > > >,);
            let args = (camera_attributes.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "set_camera_attributes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_camera_attributes(&self,) -> Option < Gd < crate::classes::CameraAttributes > > {
            type CallRet = Option < Gd < crate::classes::CameraAttributes > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LightmapGi", "get_camera_attributes", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for LightmapGi {
        type Base = crate::classes::VisualInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("LightmapGI"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for LightmapGi {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for LightmapGi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for LightmapGi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for LightmapGi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for LightmapGi {
        
    }
    impl crate::obj::cap::GodotDefault for LightmapGi {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for LightmapGi {
        type Target = crate::classes::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for LightmapGi {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`LightmapGi`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_LightmapGi__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::LightmapGi > for $Class {
                
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
pub struct BakeQuality {
    ord: i32
}
impl BakeQuality {
    #[doc(alias = "BAKE_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `BAKE_QUALITY_LOW`"]
    pub const LOW: BakeQuality = BakeQuality {
        ord: 0i32
    };
    #[doc(alias = "BAKE_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `BAKE_QUALITY_MEDIUM`"]
    pub const MEDIUM: BakeQuality = BakeQuality {
        ord: 1i32
    };
    #[doc(alias = "BAKE_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `BAKE_QUALITY_HIGH`"]
    pub const HIGH: BakeQuality = BakeQuality {
        ord: 2i32
    };
    #[doc(alias = "BAKE_QUALITY_ULTRA")]
    #[doc = "Godot enumerator name: `BAKE_QUALITY_ULTRA`"]
    pub const ULTRA: BakeQuality = BakeQuality {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for BakeQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BakeQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BakeQuality {
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
            Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", Self::ULTRA => "ULTRA", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BakeQuality::LOW, BakeQuality::MEDIUM, BakeQuality::HIGH, BakeQuality::ULTRA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BakeQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LOW", "BAKE_QUALITY_LOW", BakeQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "BAKE_QUALITY_MEDIUM", BakeQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "BAKE_QUALITY_HIGH", BakeQuality::HIGH), crate::meta::inspect::EnumConstant::new("ULTRA", "BAKE_QUALITY_ULTRA", BakeQuality::ULTRA)]
        }
    }
}
impl crate::meta::GodotConvert for BakeQuality {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Bake Quality Low", 0i64), EnumeratorShape::new_int("Bake Quality Medium", 1i64), EnumeratorShape::new_int("Bake Quality High", 2i64), EnumeratorShape::new_int("Bake Quality Ultra", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("LightmapGI.BakeQuality")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BakeQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BakeQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BakeQuality {
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
impl crate::registry::property::Export for BakeQuality {
    
}
impl crate::meta::Element for BakeQuality {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GenerateProbes {
    ord: i32
}
impl GenerateProbes {
    #[doc(alias = "GENERATE_PROBES_DISABLED")]
    #[doc = "Godot enumerator name: `GENERATE_PROBES_DISABLED`"]
    pub const DISABLED: GenerateProbes = GenerateProbes {
        ord: 0i32
    };
    #[doc(alias = "GENERATE_PROBES_SUBDIV_4")]
    #[doc = "Godot enumerator name: `GENERATE_PROBES_SUBDIV_4`"]
    pub const SUBDIV_4: GenerateProbes = GenerateProbes {
        ord: 1i32
    };
    #[doc(alias = "GENERATE_PROBES_SUBDIV_8")]
    #[doc = "Godot enumerator name: `GENERATE_PROBES_SUBDIV_8`"]
    pub const SUBDIV_8: GenerateProbes = GenerateProbes {
        ord: 2i32
    };
    #[doc(alias = "GENERATE_PROBES_SUBDIV_16")]
    #[doc = "Godot enumerator name: `GENERATE_PROBES_SUBDIV_16`"]
    pub const SUBDIV_16: GenerateProbes = GenerateProbes {
        ord: 3i32
    };
    #[doc(alias = "GENERATE_PROBES_SUBDIV_32")]
    #[doc = "Godot enumerator name: `GENERATE_PROBES_SUBDIV_32`"]
    pub const SUBDIV_32: GenerateProbes = GenerateProbes {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for GenerateProbes {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GenerateProbes") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GenerateProbes {
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
            Self::DISABLED => "DISABLED", Self::SUBDIV_4 => "SUBDIV_4", Self::SUBDIV_8 => "SUBDIV_8", Self::SUBDIV_16 => "SUBDIV_16", Self::SUBDIV_32 => "SUBDIV_32", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GenerateProbes::DISABLED, GenerateProbes::SUBDIV_4, GenerateProbes::SUBDIV_8, GenerateProbes::SUBDIV_16, GenerateProbes::SUBDIV_32]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GenerateProbes >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "GENERATE_PROBES_DISABLED", GenerateProbes::DISABLED), crate::meta::inspect::EnumConstant::new("SUBDIV_4", "GENERATE_PROBES_SUBDIV_4", GenerateProbes::SUBDIV_4), crate::meta::inspect::EnumConstant::new("SUBDIV_8", "GENERATE_PROBES_SUBDIV_8", GenerateProbes::SUBDIV_8), crate::meta::inspect::EnumConstant::new("SUBDIV_16", "GENERATE_PROBES_SUBDIV_16", GenerateProbes::SUBDIV_16), crate::meta::inspect::EnumConstant::new("SUBDIV_32", "GENERATE_PROBES_SUBDIV_32", GenerateProbes::SUBDIV_32)]
        }
    }
}
impl crate::meta::GodotConvert for GenerateProbes {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Generate Probes Disabled", 0i64), EnumeratorShape::new_int("Generate Probes Subdiv 4", 1i64), EnumeratorShape::new_int("Generate Probes Subdiv 8", 2i64), EnumeratorShape::new_int("Generate Probes Subdiv 16", 3i64), EnumeratorShape::new_int("Generate Probes Subdiv 32", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("LightmapGI.GenerateProbes")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for GenerateProbes {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GenerateProbes {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for GenerateProbes {
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
impl crate::registry::property::Export for GenerateProbes {
    
}
impl crate::meta::Element for GenerateProbes {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BakeError {
    ord: i32
}
impl BakeError {
    #[doc(alias = "BAKE_ERROR_OK")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_OK`"]
    pub const OK: BakeError = BakeError {
        ord: 0i32
    };
    #[doc(alias = "BAKE_ERROR_NO_SCENE_ROOT")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_NO_SCENE_ROOT`"]
    pub const NO_SCENE_ROOT: BakeError = BakeError {
        ord: 1i32
    };
    #[doc(alias = "BAKE_ERROR_FOREIGN_DATA")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_FOREIGN_DATA`"]
    pub const FOREIGN_DATA: BakeError = BakeError {
        ord: 2i32
    };
    #[doc(alias = "BAKE_ERROR_NO_LIGHTMAPPER")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_NO_LIGHTMAPPER`"]
    pub const NO_LIGHTMAPPER: BakeError = BakeError {
        ord: 3i32
    };
    #[doc(alias = "BAKE_ERROR_NO_SAVE_PATH")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_NO_SAVE_PATH`"]
    pub const NO_SAVE_PATH: BakeError = BakeError {
        ord: 4i32
    };
    #[doc(alias = "BAKE_ERROR_NO_MESHES")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_NO_MESHES`"]
    pub const NO_MESHES: BakeError = BakeError {
        ord: 5i32
    };
    #[doc(alias = "BAKE_ERROR_MESHES_INVALID")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_MESHES_INVALID`"]
    pub const MESHES_INVALID: BakeError = BakeError {
        ord: 6i32
    };
    #[doc(alias = "BAKE_ERROR_CANT_CREATE_IMAGE")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_CANT_CREATE_IMAGE`"]
    pub const CANT_CREATE_IMAGE: BakeError = BakeError {
        ord: 7i32
    };
    #[doc(alias = "BAKE_ERROR_USER_ABORTED")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_USER_ABORTED`"]
    pub const USER_ABORTED: BakeError = BakeError {
        ord: 8i32
    };
    #[doc(alias = "BAKE_ERROR_TEXTURE_SIZE_TOO_SMALL")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_TEXTURE_SIZE_TOO_SMALL`"]
    pub const TEXTURE_SIZE_TOO_SMALL: BakeError = BakeError {
        ord: 9i32
    };
    #[doc(alias = "BAKE_ERROR_LIGHTMAP_TOO_SMALL")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_LIGHTMAP_TOO_SMALL`"]
    pub const LIGHTMAP_TOO_SMALL: BakeError = BakeError {
        ord: 10i32
    };
    #[doc(alias = "BAKE_ERROR_ATLAS_TOO_SMALL")]
    #[doc = "Godot enumerator name: `BAKE_ERROR_ATLAS_TOO_SMALL`"]
    pub const ATLAS_TOO_SMALL: BakeError = BakeError {
        ord: 11i32
    };
    
}
impl std::fmt::Debug for BakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BakeError") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BakeError {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
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
            Self::OK => "OK", Self::NO_SCENE_ROOT => "NO_SCENE_ROOT", Self::FOREIGN_DATA => "FOREIGN_DATA", Self::NO_LIGHTMAPPER => "NO_LIGHTMAPPER", Self::NO_SAVE_PATH => "NO_SAVE_PATH", Self::NO_MESHES => "NO_MESHES", Self::MESHES_INVALID => "MESHES_INVALID", Self::CANT_CREATE_IMAGE => "CANT_CREATE_IMAGE", Self::USER_ABORTED => "USER_ABORTED", Self::TEXTURE_SIZE_TOO_SMALL => "TEXTURE_SIZE_TOO_SMALL", Self::LIGHTMAP_TOO_SMALL => "LIGHTMAP_TOO_SMALL", Self::ATLAS_TOO_SMALL => "ATLAS_TOO_SMALL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BakeError::OK, BakeError::NO_SCENE_ROOT, BakeError::FOREIGN_DATA, BakeError::NO_LIGHTMAPPER, BakeError::NO_SAVE_PATH, BakeError::NO_MESHES, BakeError::MESHES_INVALID, BakeError::CANT_CREATE_IMAGE, BakeError::USER_ABORTED, BakeError::TEXTURE_SIZE_TOO_SMALL, BakeError::LIGHTMAP_TOO_SMALL, BakeError::ATLAS_TOO_SMALL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BakeError >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OK", "BAKE_ERROR_OK", BakeError::OK), crate::meta::inspect::EnumConstant::new("NO_SCENE_ROOT", "BAKE_ERROR_NO_SCENE_ROOT", BakeError::NO_SCENE_ROOT), crate::meta::inspect::EnumConstant::new("FOREIGN_DATA", "BAKE_ERROR_FOREIGN_DATA", BakeError::FOREIGN_DATA), crate::meta::inspect::EnumConstant::new("NO_LIGHTMAPPER", "BAKE_ERROR_NO_LIGHTMAPPER", BakeError::NO_LIGHTMAPPER), crate::meta::inspect::EnumConstant::new("NO_SAVE_PATH", "BAKE_ERROR_NO_SAVE_PATH", BakeError::NO_SAVE_PATH), crate::meta::inspect::EnumConstant::new("NO_MESHES", "BAKE_ERROR_NO_MESHES", BakeError::NO_MESHES), crate::meta::inspect::EnumConstant::new("MESHES_INVALID", "BAKE_ERROR_MESHES_INVALID", BakeError::MESHES_INVALID), crate::meta::inspect::EnumConstant::new("CANT_CREATE_IMAGE", "BAKE_ERROR_CANT_CREATE_IMAGE", BakeError::CANT_CREATE_IMAGE), crate::meta::inspect::EnumConstant::new("USER_ABORTED", "BAKE_ERROR_USER_ABORTED", BakeError::USER_ABORTED), crate::meta::inspect::EnumConstant::new("TEXTURE_SIZE_TOO_SMALL", "BAKE_ERROR_TEXTURE_SIZE_TOO_SMALL", BakeError::TEXTURE_SIZE_TOO_SMALL), crate::meta::inspect::EnumConstant::new("LIGHTMAP_TOO_SMALL", "BAKE_ERROR_LIGHTMAP_TOO_SMALL", BakeError::LIGHTMAP_TOO_SMALL), crate::meta::inspect::EnumConstant::new("ATLAS_TOO_SMALL", "BAKE_ERROR_ATLAS_TOO_SMALL", BakeError::ATLAS_TOO_SMALL)]
        }
    }
}
impl crate::meta::GodotConvert for BakeError {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Bake Error Ok", 0i64), EnumeratorShape::new_int("Bake Error No Scene Root", 1i64), EnumeratorShape::new_int("Bake Error Foreign Data", 2i64), EnumeratorShape::new_int("Bake Error No Lightmapper", 3i64), EnumeratorShape::new_int("Bake Error No Save Path", 4i64), EnumeratorShape::new_int("Bake Error No Meshes", 5i64), EnumeratorShape::new_int("Bake Error Meshes Invalid", 6i64), EnumeratorShape::new_int("Bake Error Cant Create Image", 7i64), EnumeratorShape::new_int("Bake Error User Aborted", 8i64), EnumeratorShape::new_int("Bake Error Texture Size Too Small", 9i64), EnumeratorShape::new_int("Bake Error Lightmap Too Small", 10i64), EnumeratorShape::new_int("Bake Error Atlas Too Small", 11i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("LightmapGI.BakeError")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BakeError {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BakeError {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BakeError {
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
impl crate::registry::property::Export for BakeError {
    
}
impl crate::meta::Element for BakeError {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentMode {
    ord: i32
}
impl EnvironmentMode {
    #[doc(alias = "ENVIRONMENT_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `ENVIRONMENT_MODE_DISABLED`"]
    pub const DISABLED: EnvironmentMode = EnvironmentMode {
        ord: 0i32
    };
    #[doc(alias = "ENVIRONMENT_MODE_SCENE")]
    #[doc = "Godot enumerator name: `ENVIRONMENT_MODE_SCENE`"]
    pub const SCENE: EnvironmentMode = EnvironmentMode {
        ord: 1i32
    };
    #[doc(alias = "ENVIRONMENT_MODE_CUSTOM_SKY")]
    #[doc = "Godot enumerator name: `ENVIRONMENT_MODE_CUSTOM_SKY`"]
    pub const CUSTOM_SKY: EnvironmentMode = EnvironmentMode {
        ord: 2i32
    };
    #[doc(alias = "ENVIRONMENT_MODE_CUSTOM_COLOR")]
    #[doc = "Godot enumerator name: `ENVIRONMENT_MODE_CUSTOM_COLOR`"]
    pub const CUSTOM_COLOR: EnvironmentMode = EnvironmentMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EnvironmentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentMode {
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
            Self::DISABLED => "DISABLED", Self::SCENE => "SCENE", Self::CUSTOM_SKY => "CUSTOM_SKY", Self::CUSTOM_COLOR => "CUSTOM_COLOR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentMode::DISABLED, EnvironmentMode::SCENE, EnvironmentMode::CUSTOM_SKY, EnvironmentMode::CUSTOM_COLOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "ENVIRONMENT_MODE_DISABLED", EnvironmentMode::DISABLED), crate::meta::inspect::EnumConstant::new("SCENE", "ENVIRONMENT_MODE_SCENE", EnvironmentMode::SCENE), crate::meta::inspect::EnumConstant::new("CUSTOM_SKY", "ENVIRONMENT_MODE_CUSTOM_SKY", EnvironmentMode::CUSTOM_SKY), crate::meta::inspect::EnumConstant::new("CUSTOM_COLOR", "ENVIRONMENT_MODE_CUSTOM_COLOR", EnvironmentMode::CUSTOM_COLOR)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Environment Mode Disabled", 0i64), EnumeratorShape::new_int("Environment Mode Scene", 1i64), EnumeratorShape::new_int("Environment Mode Custom Sky", 2i64), EnumeratorShape::new_int("Environment Mode Custom Color", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("LightmapGI.EnvironmentMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentMode {
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
impl crate::registry::property::Export for EnvironmentMode {
    
}
impl crate::meta::Element for EnvironmentMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::LightmapGi;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for LightmapGi {
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
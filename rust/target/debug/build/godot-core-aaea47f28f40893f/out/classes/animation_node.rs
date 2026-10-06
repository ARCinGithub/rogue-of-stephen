#![doc = "Sidecar module for class [`AnimationNode`][crate::classes::AnimationNode].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNode` enums](https://docs.godotengine.org/en/stable/classes/class_animationnode.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNode`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`animation_node`][crate::classes::animation_node]: sidecar module with related enum/flag types\n* [`IAnimationNode`][crate::classes::IAnimationNode]: virtual methods\n* [`SignalsOfAnimationNode`][crate::classes::animation_node::SignalsOfAnimationNode]: signal collection\n\n\nSee also [Godot docs for `AnimationNode`](https://docs.godotengine.org/en/stable/classes/class_animationnode.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimationNode::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nBase resource for [`AnimationTree`][crate::classes::AnimationTree] nodes. In general, it's not used directly, but you can create custom ones with custom blending formulas.\n\nInherit this when creating animation nodes mainly for use in [`AnimationNodeBlendTree`][crate::classes::AnimationNodeBlendTree], otherwise [`AnimationRootNode`][crate::classes::AnimationRootNode] should be used instead.\n\nYou can access the time information as read-only parameter which is processed and stored in the previous frame for all nodes except [`AnimationNodeOutput`][crate::classes::AnimationNodeOutput].\n\n**Note:** If multiple inputs exist in the `AnimationNode`, which time information takes precedence depends on the type of `AnimationNode`.\n\n```gdscript\nvar current_length = $AnimationTree[\"parameters/AnimationNodeName/current_length\"]\nvar current_position = $AnimationTree[\"parameters/AnimationNodeName/current_position\"]\nvar current_delta = $AnimationTree[\"parameters/AnimationNodeName/current_delta\"]\n```"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNode {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimationNode`][crate::classes::AnimationNode].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AnimationNode` methods](https://docs.godotengine.org/en/stable/classes/class_animationnode.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNode: crate::obj::GodotClass < Base = AnimationNode > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "When inheriting from [`AnimationRootNode`][crate::classes::AnimationRootNode], implement this virtual method to run some code when this animation node is processed. The `time` parameter is a relative delta, unless `seek` is `true`, in which case it is absolute.\n\nHere, call the [`blend_input`][`crate::classes::AnimationNode::blend_input`], [`blend_node`][`crate::classes::AnimationNode::blend_node`] or [`blend_animation`][`crate::classes::AnimationNode::blend_animation`] functions. You can also use [`get_parameter`][`crate::classes::AnimationNode::get_parameter`] and [`set_parameter`][`crate::classes::AnimationNode::set_parameter`] to modify local memory.\n\nThis function should return the delta."]
        fn process(&mut self, time: f64, seek: bool, is_external_seeking: bool, test_only: bool,) -> f64 {
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
        fn on_notification(&mut self, what: ObjectNotification) {
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
        #[doc = "When inheriting from [`AnimationRootNode`][crate::classes::AnimationRootNode], implement this virtual method to return all child animation nodes in order as a `name: node` dictionary."]
        fn get_child_nodes(&self,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "When inheriting from [`AnimationRootNode`][crate::classes::AnimationRootNode], implement this virtual method to return a list of the properties on this animation node. Parameters are custom local memory used for your animation nodes, given a resource can be reused in multiple trees. Format is similar to [`get_property_list`][`crate::classes::Object::get_property_list`]."]
        fn get_parameter_list(&self,) -> AnyArray {
            unimplemented !()
        }
        #[doc = "When inheriting from [`AnimationRootNode`][crate::classes::AnimationRootNode], implement this virtual method to return a child animation node by its `name`."]
        fn get_child_by_name(&self, name: StringName,) -> Option < Gd < crate::classes::AnimationNode > > {
            unimplemented !()
        }
        #[doc = "When inheriting from [`AnimationRootNode`][crate::classes::AnimationRootNode], implement this virtual method to return the default value of a `parameter`. Parameters are custom local memory used for your animation nodes, given a resource can be reused in multiple trees."]
        fn get_parameter_default_value(&self, parameter: StringName,) -> Variant {
            unimplemented !()
        }
        #[doc = "When inheriting from [`AnimationRootNode`][crate::classes::AnimationRootNode], implement this virtual method to return whether the `parameter` is read-only. Parameters are custom local memory used for your animation nodes, given a resource can be reused in multiple trees."]
        fn is_parameter_read_only(&self, parameter: StringName,) -> bool {
            unimplemented !()
        }
        #[doc = "When inheriting from [`AnimationRootNode`][crate::classes::AnimationRootNode], implement this virtual method to override the text caption for this animation node."]
        fn get_caption(&self,) -> GString {
            unimplemented !()
        }
        #[doc = "When inheriting from [`AnimationRootNode`][crate::classes::AnimationRootNode], implement this virtual method to return whether the blend tree editor should display filter editing on this animation node."]
        fn has_filter(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Override this method to customize the newly duplicated resource created from [`instantiate`][`crate::classes::PackedScene::instantiate`], if the original's \\[member resource_local_to_scene] is set to `true`.\n\n**Example:** Set a random `damage` value to every local resource from an instantiated scene:\n\n```gdscript\nextends Resource\n\nvar damage = 0\n\nfunc _setup_local_to_scene():\n\tdamage = randi_range(10, 40)\n```"]
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to return a custom [`RID`][crate::builtin::Rid] when [`get_rid`][`crate::classes::Resource::get_rid`] is called."]
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        #[doc = "For resources that store state in non-exported properties, such as via [`on_validate_property`][`crate::classes::IObject::on_validate_property`] or [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`], this method must be implemented to clear them."]
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to execute additional logic after [`set_path_cache`][`crate::classes::Resource::set_path_cache`] is called on this object."]
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl AnimationNode {
        #[doc = "Adds an input to the animation node. This is only useful for animation nodes created for use in an [`AnimationNodeBlendTree`][crate::classes::AnimationNodeBlendTree]. If the addition fails, returns `false`."]
        pub fn add_input(&mut self, name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "add_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes an input, call this only when inactive."]
        pub fn remove_input(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "remove_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the name of the input at the given `input` index. If the setting fails, returns `false`."]
        pub fn set_input_name(&mut self, input: i32, name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (input, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "set_input_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the name of an input by index."]
        pub fn get_input_name(&self, input: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (input,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "get_input_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Amount of inputs in this animation node, only useful for animation nodes that go into [`AnimationNodeBlendTree`][crate::classes::AnimationNodeBlendTree]."]
        pub fn get_input_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "get_input_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the input index which corresponds to `name`. If not found, returns `-1`."]
        pub fn find_input(&self, name: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "find_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds or removes a path for the filter."]
        pub fn set_filter_path(&mut self, path: impl AsArg < NodePath >, enable: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >, bool,);
            let args = (path.into_arg(), enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "set_filter_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given path is filtered."]
        pub fn is_path_filtered(&self, path: impl AsArg < NodePath >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "is_path_filtered", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_filter_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "set_filter_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_filter_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "is_filter_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the object id of the [`AnimationTree`][crate::classes::AnimationTree] that owns this node.\n\n**Note:** This method should only be called from within the [`process_animation_node`][`crate::classes::IAnimationNodeExtension::process_animation_node`] method, and will return an invalid id otherwise."]
        pub fn get_processing_animation_tree_instance_id(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "get_processing_animation_tree_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this animation node is being processed in test-only mode."]
        pub fn is_process_testing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "is_process_testing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Blends an animation by `blend` amount (name must be valid in the linked [`AnimationPlayer`][crate::classes::AnimationPlayer]). A `time` and `delta` may be passed, as well as whether `seeked` happened.\n\nA `looped_flag` is used by internal processing immediately after the loop."]
        pub(crate) fn blend_animation_full(&mut self, animation: CowArg < StringName >, time: f64, delta: f64, seeked: bool, is_external_seeking: bool, blend: f32, looped_flag: crate::classes::animation::LoopedFlag,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f64, f64, bool, bool, f32, crate::classes::animation::LoopedFlag,);
            let args = (animation, time, delta, seeked, is_external_seeking, blend, looped_flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "blend_animation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`blend_animation_ex`][Self::blend_animation_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Blends an animation by `blend` amount (name must be valid in the linked [`AnimationPlayer`][crate::classes::AnimationPlayer]). A `time` and `delta` may be passed, as well as whether `seeked` happened.\n\nA `looped_flag` is used by internal processing immediately after the loop."]
        #[inline]
        pub fn blend_animation(&mut self, animation: impl AsArg < StringName >, time: f64, delta: f64, seeked: bool, is_external_seeking: bool, blend: f32,) {
            self.blend_animation_ex(animation, time, delta, seeked, is_external_seeking, blend,) . done()
        }
        #[doc = "Blends an animation by `blend` amount (name must be valid in the linked [`AnimationPlayer`][crate::classes::AnimationPlayer]). A `time` and `delta` may be passed, as well as whether `seeked` happened.\n\nA `looped_flag` is used by internal processing immediately after the loop."]
        #[inline]
        pub fn blend_animation_ex < 'ex > (&'ex mut self, animation: impl AsArg < StringName > + 'ex, time: f64, delta: f64, seeked: bool, is_external_seeking: bool, blend: f32,) -> ExBlendAnimation < 'ex > {
            ExBlendAnimation::new(self, animation, time, delta, seeked, is_external_seeking, blend,)
        }
        #[doc = "Blend another animation node (in case this animation node contains child animation nodes). This function is only useful if you inherit from [`AnimationRootNode`][crate::classes::AnimationRootNode] instead, otherwise editors will not display your animation node for addition."]
        pub(crate) fn blend_node_full(&mut self, name: CowArg < StringName >, node: CowArg < Option < Gd < crate::classes::AnimationNode > > >, time: f64, seek: bool, is_external_seeking: bool, blend: f32, filter: crate::classes::animation_node::FilterAction, sync: bool, test_only: bool,) -> f64 {
            type CallRet = f64;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::AnimationNode > > >, f64, bool, bool, f32, crate::classes::animation_node::FilterAction, bool, bool,);
            let args = (name, node, time, seek, is_external_seeking, blend, filter, sync, test_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "blend_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`blend_node_ex`][Self::blend_node_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Blend another animation node (in case this animation node contains child animation nodes). This function is only useful if you inherit from [`AnimationRootNode`][crate::classes::AnimationRootNode] instead, otherwise editors will not display your animation node for addition."]
        #[inline]
        pub fn blend_node(&mut self, name: impl AsArg < StringName >, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> >, time: f64, seek: bool, is_external_seeking: bool, blend: f32,) -> f64 {
            self.blend_node_ex(name, node, time, seek, is_external_seeking, blend,) . done()
        }
        #[doc = "Blend another animation node (in case this animation node contains child animation nodes). This function is only useful if you inherit from [`AnimationRootNode`][crate::classes::AnimationRootNode] instead, otherwise editors will not display your animation node for addition."]
        #[inline]
        pub fn blend_node_ex < 'ex > (&'ex mut self, name: impl AsArg < StringName > + 'ex, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> > + 'ex, time: f64, seek: bool, is_external_seeking: bool, blend: f32,) -> ExBlendNode < 'ex > {
            ExBlendNode::new(self, name, node, time, seek, is_external_seeking, blend,)
        }
        #[doc = "Blends an input. This is only useful for animation nodes created for an [`AnimationNodeBlendTree`][crate::classes::AnimationNodeBlendTree]. The `time` parameter is a relative delta, unless `seek` is `true`, in which case it is absolute. A filter mode may be optionally passed."]
        pub(crate) fn blend_input_full(&mut self, input_index: i32, time: f64, seek: bool, is_external_seeking: bool, blend: f32, filter: crate::classes::animation_node::FilterAction, sync: bool, test_only: bool,) -> f64 {
            type CallRet = f64;
            type CallParams = (i32, f64, bool, bool, f32, crate::classes::animation_node::FilterAction, bool, bool,);
            let args = (input_index, time, seek, is_external_seeking, blend, filter, sync, test_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "blend_input", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`blend_input_ex`][Self::blend_input_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Blends an input. This is only useful for animation nodes created for an [`AnimationNodeBlendTree`][crate::classes::AnimationNodeBlendTree]. The `time` parameter is a relative delta, unless `seek` is `true`, in which case it is absolute. A filter mode may be optionally passed."]
        #[inline]
        pub fn blend_input(&mut self, input_index: i32, time: f64, seek: bool, is_external_seeking: bool, blend: f32,) -> f64 {
            self.blend_input_ex(input_index, time, seek, is_external_seeking, blend,) . done()
        }
        #[doc = "Blends an input. This is only useful for animation nodes created for an [`AnimationNodeBlendTree`][crate::classes::AnimationNodeBlendTree]. The `time` parameter is a relative delta, unless `seek` is `true`, in which case it is absolute. A filter mode may be optionally passed."]
        #[inline]
        pub fn blend_input_ex < 'ex > (&'ex mut self, input_index: i32, time: f64, seek: bool, is_external_seeking: bool, blend: f32,) -> ExBlendInput < 'ex > {
            ExBlendInput::new(self, input_index, time, seek, is_external_seeking, blend,)
        }
        #[doc = "Sets a custom parameter. These are used as local memory, because resources can be reused across the tree or scenes."]
        pub fn set_parameter(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "set_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the value of a parameter. Parameters are custom local memory used for your animation nodes, given a resource can be reused in multiple trees."]
        pub fn get_parameter(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNode", "get_parameter", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationNode {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationNode"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNode {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimationNode {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimationNode {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationNode {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNode {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNode {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNode {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationNode`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationNode__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNode > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AnimationNode::blend_animation_ex`][super::AnimationNode::blend_animation_ex]."]
#[must_use]
pub struct ExBlendAnimation < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationNode, animation: CowArg < 'ex, StringName >, time: f64, delta: f64, seeked: bool, is_external_seeking: bool, blend: f32, looped_flag: crate::classes::animation::LoopedFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBlendAnimation < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationNode, animation: impl AsArg < StringName > + 'ex, time: f64, delta: f64, seeked: bool, is_external_seeking: bool, blend: f32,) -> Self {
        let looped_flag = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, animation: animation.into_arg(), time: time, delta: delta, seeked: seeked, is_external_seeking: is_external_seeking, blend: blend, looped_flag: looped_flag,
        }
    }
    #[inline]
    pub fn looped_flag(self, looped_flag: crate::classes::animation::LoopedFlag) -> Self {
        Self {
            looped_flag: looped_flag, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, animation, time, delta, seeked, is_external_seeking, blend, looped_flag,
        }
        = self;
        re_export::AnimationNode::blend_animation_full(surround_object, animation, time, delta, seeked, is_external_seeking, blend, looped_flag,)
    }
}
#[doc = "Default-param extender for [`AnimationNode::blend_node_ex`][super::AnimationNode::blend_node_ex]."]
#[must_use]
pub struct ExBlendNode < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationNode, name: CowArg < 'ex, StringName >, node: CowArg < 'ex, Option < Gd < crate::classes::AnimationNode > > >, time: f64, seek: bool, is_external_seeking: bool, blend: f32, filter: crate::classes::animation_node::FilterAction, sync: bool, test_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBlendNode < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationNode, name: impl AsArg < StringName > + 'ex, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> > + 'ex, time: f64, seek: bool, is_external_seeking: bool, blend: f32,) -> Self {
        let filter = crate::obj::EngineEnum::from_ord(0);
        let sync = true;
        let test_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), node: node.into_arg(), time: time, seek: seek, is_external_seeking: is_external_seeking, blend: blend, filter: filter, sync: sync, test_only: test_only,
        }
    }
    #[inline]
    pub fn filter(self, filter: crate::classes::animation_node::FilterAction) -> Self {
        Self {
            filter: filter, .. self
        }
    }
    #[inline]
    pub fn sync(self, sync: bool) -> Self {
        Self {
            sync: sync, .. self
        }
    }
    #[inline]
    pub fn test_only(self, test_only: bool) -> Self {
        Self {
            test_only: test_only, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f64 {
        let Self {
            _phantom, surround_object, name, node, time, seek, is_external_seeking, blend, filter, sync, test_only,
        }
        = self;
        re_export::AnimationNode::blend_node_full(surround_object, name, node, time, seek, is_external_seeking, blend, filter, sync, test_only,)
    }
}
#[doc = "Default-param extender for [`AnimationNode::blend_input_ex`][super::AnimationNode::blend_input_ex]."]
#[must_use]
pub struct ExBlendInput < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationNode, input_index: i32, time: f64, seek: bool, is_external_seeking: bool, blend: f32, filter: crate::classes::animation_node::FilterAction, sync: bool, test_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBlendInput < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationNode, input_index: i32, time: f64, seek: bool, is_external_seeking: bool, blend: f32,) -> Self {
        let filter = crate::obj::EngineEnum::from_ord(0);
        let sync = true;
        let test_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, input_index: input_index, time: time, seek: seek, is_external_seeking: is_external_seeking, blend: blend, filter: filter, sync: sync, test_only: test_only,
        }
    }
    #[inline]
    pub fn filter(self, filter: crate::classes::animation_node::FilterAction) -> Self {
        Self {
            filter: filter, .. self
        }
    }
    #[inline]
    pub fn sync(self, sync: bool) -> Self {
        Self {
            sync: sync, .. self
        }
    }
    #[inline]
    pub fn test_only(self, test_only: bool) -> Self {
        Self {
            test_only: test_only, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f64 {
        let Self {
            _phantom, surround_object, input_index, time, seek, is_external_seeking, blend, filter, sync, test_only,
        }
        = self;
        re_export::AnimationNode::blend_input_full(surround_object, input_index, time, seek, is_external_seeking, blend, filter, sync, test_only,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FilterAction {
    ord: i32
}
impl FilterAction {
    #[doc(alias = "FILTER_IGNORE")]
    #[doc = "Godot enumerator name: `FILTER_IGNORE`"]
    pub const IGNORE: FilterAction = FilterAction {
        ord: 0i32
    };
    #[doc(alias = "FILTER_PASS")]
    #[doc = "Godot enumerator name: `FILTER_PASS`"]
    pub const PASS: FilterAction = FilterAction {
        ord: 1i32
    };
    #[doc(alias = "FILTER_STOP")]
    #[doc = "Godot enumerator name: `FILTER_STOP`"]
    pub const STOP: FilterAction = FilterAction {
        ord: 2i32
    };
    #[doc(alias = "FILTER_BLEND")]
    #[doc = "Godot enumerator name: `FILTER_BLEND`"]
    pub const BLEND: FilterAction = FilterAction {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for FilterAction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FilterAction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FilterAction {
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
            Self::IGNORE => "IGNORE", Self::PASS => "PASS", Self::STOP => "STOP", Self::BLEND => "BLEND", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FilterAction::IGNORE, FilterAction::PASS, FilterAction::STOP, FilterAction::BLEND]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FilterAction >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IGNORE", "FILTER_IGNORE", FilterAction::IGNORE), crate::meta::inspect::EnumConstant::new("PASS", "FILTER_PASS", FilterAction::PASS), crate::meta::inspect::EnumConstant::new("STOP", "FILTER_STOP", FilterAction::STOP), crate::meta::inspect::EnumConstant::new("BLEND", "FILTER_BLEND", FilterAction::BLEND)]
        }
    }
}
impl crate::meta::GodotConvert for FilterAction {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Filter Ignore", 0i64), EnumeratorShape::new_int("Filter Pass", 1i64), EnumeratorShape::new_int("Filter Stop", 2i64), EnumeratorShape::new_int("Filter Blend", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AnimationNode.FilterAction")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for FilterAction {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FilterAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for FilterAction {
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
impl crate::registry::property::Export for FilterAction {
    
}
impl crate::meta::Element for FilterAction {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationNode;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AnimationNode`][crate::classes::AnimationNode] class."]
    pub struct SignalsOfAnimationNode < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAnimationNode < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn tree_changed(&mut self) -> SigTreeChanged < 'c, C > {
            SigTreeChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "tree_changed")
            }
        }
        #[doc = "Signature: `(object_id: i64, old_name: GString, new_name: GString)`"]
        pub fn animation_node_renamed(&mut self) -> SigAnimationNodeRenamed < 'c, C > {
            SigAnimationNodeRenamed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "animation_node_renamed")
            }
        }
        #[doc = "Signature: `(object_id: i64, name: GString)`"]
        pub fn animation_node_removed(&mut self) -> SigAnimationNodeRemoved < 'c, C > {
            SigAnimationNodeRemoved {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "animation_node_removed")
            }
        }
    }
    type TypedSigTreeChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTreeChanged < 'c, C: WithSignals > {
        typed: TypedSigTreeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTreeChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTreeChanged < 'c, C > {
        type Target = TypedSigTreeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTreeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationNodeRenamed < 'c, C > = TypedSignal < 'c, C, (i64, GString, GString,) >;
    pub struct SigAnimationNodeRenamed < 'c, C: WithSignals > {
        typed: TypedSigAnimationNodeRenamed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationNodeRenamed < 'c, C > {
        pub fn emit(&mut self, object_id: i64, old_name: GString, new_name: GString,) {
            self.typed.emit_tuple((object_id, old_name, new_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationNodeRenamed < 'c, C > {
        type Target = TypedSigAnimationNodeRenamed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationNodeRenamed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationNodeRemoved < 'c, C > = TypedSignal < 'c, C, (i64, GString,) >;
    pub struct SigAnimationNodeRemoved < 'c, C: WithSignals > {
        typed: TypedSigAnimationNodeRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationNodeRemoved < 'c, C > {
        pub fn emit(&mut self, object_id: i64, name: GString,) {
            self.typed.emit_tuple((object_id, name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationNodeRemoved < 'c, C > {
        type Target = TypedSigAnimationNodeRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationNodeRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AnimationNode {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationNode < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAnimationNode < 'c, C > {
        type Target = < < AnimationNode as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AnimationNode;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAnimationNode < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AnimationNode;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
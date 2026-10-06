#![doc = "Sidecar module for class [`AnimationNodeStateMachine`][crate::classes::AnimationNodeStateMachine].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeStateMachine` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachine.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeStateMachine`.\n\nInherits [`AnimationRootNode`][crate::classes::AnimationRootNode].\n\nRelated symbols:\n\n* [`animation_node_state_machine`][crate::classes::animation_node_state_machine]: sidecar module with related enum/flag types\n* [`IAnimationNodeStateMachine`][crate::classes::IAnimationNodeStateMachine]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeStateMachine`](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachine.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimationNodeStateMachine::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nContains multiple [`AnimationRootNode`][crate::classes::AnimationRootNode]s representing animation states, connected in a graph. State transitions can be configured to happen automatically or via code, using a shortest-path algorithm. Retrieve the [`AnimationNodeStateMachinePlayback`][crate::classes::AnimationNodeStateMachinePlayback] object from the [`AnimationTree`][crate::classes::AnimationTree] node to control it programmatically.\n\n\n```gdscript\nvar state_machine = $AnimationTree.get(\"parameters/playback\")\nstate_machine.travel(\"some_state\")\n```\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeStateMachine {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimationNodeStateMachine`][crate::classes::AnimationNodeStateMachine].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAnimationRootNode`][crate::classes::IAnimationRootNode] > [`IAnimationNode`][crate::classes::IAnimationNode] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AnimationNodeStateMachine` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachine.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeStateMachine: crate::obj::GodotClass < Base = AnimationNodeStateMachine > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationNodeStateMachine {
        #[doc = "Adds a new animation node to the graph. The `position` is used for display in the editor."]
        pub(crate) fn add_node_full(&mut self, name: CowArg < StringName >, node: CowArg < Option < Gd < crate::classes::AnimationNode > > >, position: Vector2,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::AnimationNode > > >, Vector2,);
            let args = (name, node, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "add_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_node_ex`][Self::add_node_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new animation node to the graph. The `position` is used for display in the editor."]
        #[inline]
        pub fn add_node(&mut self, name: impl AsArg < StringName >, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> >,) {
            self.add_node_ex(name, node,) . done()
        }
        #[doc = "Adds a new animation node to the graph. The `position` is used for display in the editor."]
        #[inline]
        pub fn add_node_ex < 'ex > (&'ex mut self, name: impl AsArg < StringName > + 'ex, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> > + 'ex,) -> ExAddNode < 'ex > {
            ExAddNode::new(self, name, node,)
        }
        #[doc = "Replaces the given animation node with a new animation node."]
        pub fn replace_node(&mut self, name: impl AsArg < StringName >, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::AnimationNode > > >,);
            let args = (name.into_arg(), node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "replace_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the animation node with the given name."]
        pub fn get_node(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::AnimationNode > > {
            type CallRet = Option < Gd < crate::classes::AnimationNode > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deletes the given animation node from the graph."]
        pub fn remove_node(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "remove_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Renames the given animation node."]
        pub fn rename_node(&mut self, name: impl AsArg < StringName >, new_name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), new_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "rename_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the graph contains the given animation node."]
        pub fn has_node(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "has_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given animation node's name."]
        pub fn get_node_name(&self, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::AnimationNode > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_node_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list containing the names of all animation nodes in this state machine."]
        pub fn get_node_list(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_node_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the animation node's coordinates. Used for display in the editor."]
        pub fn set_node_position(&mut self, name: impl AsArg < StringName >, position: Vector2,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, Vector2,);
            let args = (name.into_arg(), position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10615usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "set_node_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given animation node's coordinates. Used for display in the editor."]
        pub fn get_node_position(&self, name: impl AsArg < StringName >,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10616usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_node_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there is a transition between the given animation nodes."]
        pub fn has_transition(&self, from: impl AsArg < StringName >, to: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (from.into_arg(), to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10617usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "has_transition", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a transition between the given animation nodes."]
        pub fn add_transition(&mut self, from: impl AsArg < StringName >, to: impl AsArg < StringName >, transition: impl AsArg < Option < Gd < crate::classes::AnimationNodeStateMachineTransition >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, Option < Gd < crate::classes::AnimationNodeStateMachineTransition > > >,);
            let args = (from.into_arg(), to.into_arg(), transition.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10618usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "add_transition", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given transition."]
        pub fn get_transition(&self, idx: i32,) -> Option < Gd < crate::classes::AnimationNodeStateMachineTransition > > {
            type CallRet = Option < Gd < crate::classes::AnimationNodeStateMachineTransition > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10619usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_transition", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given transition's start node."]
        pub fn get_transition_from(&self, idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10620usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_transition_from", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given transition's end node."]
        pub fn get_transition_to(&self, idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10621usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_transition_to", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of connections in the graph."]
        pub fn get_transition_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10622usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_transition_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deletes the given transition by index."]
        pub fn remove_transition_by_index(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10623usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "remove_transition_by_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deletes the transition between the two specified animation nodes."]
        pub fn remove_transition(&mut self, from: impl AsArg < StringName >, to: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (from.into_arg(), to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10624usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "remove_transition", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the draw offset of the graph. Used for display in the editor."]
        pub fn set_graph_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "set_graph_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the draw offset of the graph. Used for display in the editor."]
        pub fn get_graph_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_graph_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_state_machine_type(&mut self, state_machine_type: crate::classes::animation_node_state_machine::StateMachineType,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_node_state_machine::StateMachineType,);
            let args = (state_machine_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "set_state_machine_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_state_machine_type(&self,) -> crate::classes::animation_node_state_machine::StateMachineType {
            type CallRet = crate::classes::animation_node_state_machine::StateMachineType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_state_machine_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_transition_to_self(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "set_allow_transition_to_self", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_allow_transition_to_self(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "is_allow_transition_to_self", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_reset_ends(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "set_reset_ends", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_ends_reset(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "are_ends_reset", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationNodeStateMachine {
        type Base = crate::classes::AnimationRootNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationNodeStateMachine"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeStateMachine {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationRootNode > for AnimationNodeStateMachine {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationNode > for AnimationNodeStateMachine {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimationNodeStateMachine {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimationNodeStateMachine {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationNodeStateMachine {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeStateMachine {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeStateMachine {
        type Target = crate::classes::AnimationRootNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeStateMachine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationNodeStateMachine`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationNodeStateMachine__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNodeStateMachine > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationRootNode > for $Class {
                
            }
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
#[doc = "Default-param extender for [`AnimationNodeStateMachine::add_node_ex`][super::AnimationNodeStateMachine::add_node_ex]."]
#[must_use]
pub struct ExAddNode < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationNodeStateMachine, name: CowArg < 'ex, StringName >, node: CowArg < 'ex, Option < Gd < crate::classes::AnimationNode > > >, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddNode < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationNodeStateMachine, name: impl AsArg < StringName > + 'ex, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> > + 'ex,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), node: node.into_arg(), position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, node, position,
        }
        = self;
        re_export::AnimationNodeStateMachine::add_node_full(surround_object, name, node, position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StateMachineType {
    ord: i32
}
impl StateMachineType {
    #[doc(alias = "STATE_MACHINE_TYPE_ROOT")]
    #[doc = "Godot enumerator name: `STATE_MACHINE_TYPE_ROOT`"]
    pub const ROOT: StateMachineType = StateMachineType {
        ord: 0i32
    };
    #[doc(alias = "STATE_MACHINE_TYPE_NESTED")]
    #[doc = "Godot enumerator name: `STATE_MACHINE_TYPE_NESTED`"]
    pub const NESTED: StateMachineType = StateMachineType {
        ord: 1i32
    };
    #[doc(alias = "STATE_MACHINE_TYPE_GROUPED")]
    #[doc = "Godot enumerator name: `STATE_MACHINE_TYPE_GROUPED`"]
    pub const GROUPED: StateMachineType = StateMachineType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for StateMachineType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StateMachineType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StateMachineType {
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
            Self::ROOT => "ROOT", Self::NESTED => "NESTED", Self::GROUPED => "GROUPED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[StateMachineType::ROOT, StateMachineType::NESTED, StateMachineType::GROUPED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < StateMachineType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ROOT", "STATE_MACHINE_TYPE_ROOT", StateMachineType::ROOT), crate::meta::inspect::EnumConstant::new("NESTED", "STATE_MACHINE_TYPE_NESTED", StateMachineType::NESTED), crate::meta::inspect::EnumConstant::new("GROUPED", "STATE_MACHINE_TYPE_GROUPED", StateMachineType::GROUPED)]
        }
    }
}
impl crate::meta::GodotConvert for StateMachineType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("State Machine Type Root", 0i64), EnumeratorShape::new_int("State Machine Type Nested", 1i64), EnumeratorShape::new_int("State Machine Type Grouped", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AnimationNodeStateMachine.StateMachineType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for StateMachineType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StateMachineType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for StateMachineType {
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
impl crate::registry::property::Export for StateMachineType {
    
}
impl crate::meta::Element for StateMachineType {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationNodeStateMachine;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::animation_node::SignalsOfAnimationNode;
    impl WithSignals for AnimationNodeStateMachine {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationNode < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
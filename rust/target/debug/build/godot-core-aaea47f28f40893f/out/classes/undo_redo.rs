#![doc = "Sidecar module for class [`UndoRedo`][crate::classes::UndoRedo].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `UndoRedo` enums](https://docs.godotengine.org/en/stable/classes/class_undoredo.html#enumerations).\n\n"]
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
    #[doc = "Godot class `UndoRedo`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`undo_redo`][crate::classes::undo_redo]: sidecar module with related enum/flag types\n* [`IUndoRedo`][crate::classes::IUndoRedo]: virtual methods\n* [`SignalsOfUndoRedo`][crate::classes::undo_redo::SignalsOfUndoRedo]: signal collection\n\n\nSee also [Godot docs for `UndoRedo`](https://docs.godotengine.org/en/stable/classes/class_undoredo.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`UndoRedo::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nUndoRedo works by registering methods and property changes inside \"actions\". You can create an action, then provide ways to do and undo this action using function calls and property changes, then commit the action.\n\nWhen an action is committed, all of the `do_*` methods will run. If the [`undo`][`crate::classes::UndoRedo::undo`] method is used, the `undo_*` methods will run. If the [`redo`][`crate::classes::UndoRedo::redo`] method is used, once again, all of the `do_*` methods will run.\n\nHere's an example on how to add an action:\n\n\n```gdscript\nvar undo_redo = UndoRedo.new()\n\nfunc do_something():\n\tpass # Put your code here.\n\nfunc undo_something():\n\tpass # Put here the code that reverts what's done by \"do_something()\".\n\nfunc _on_my_button_pressed():\n\tvar node = get_node(\"MyNode2D\")\n\tundo_redo.create_action(\"Move the node\")\n\tundo_redo.add_do_method(do_something)\n\tundo_redo.add_undo_method(undo_something)\n\tundo_redo.add_do_property(node, \"position\", Vector2(100, 100))\n\tundo_redo.add_undo_property(node, \"position\", node.position)\n\tundo_redo.commit_action()\n```\n\n\nBefore calling any of the `add_(un)do_*` methods, you need to first call [`create_action`][`crate::classes::UndoRedo::create_action`]. Afterwards you need to call [`commit_action`][`crate::classes::UndoRedo::commit_action`].\n\nIf you don't need to register a method, you can leave [`add_do_method`][`crate::classes::UndoRedo::add_do_method`] and [`add_undo_method`][`crate::classes::UndoRedo::add_undo_method`] out; the same goes for properties. You can also register more than one method/property.\n\nIf you are making an [`EditorPlugin`][crate::classes::EditorPlugin] and want to integrate into the editor's undo history, use [`EditorUndoRedoManager`][crate::classes::EditorUndoRedoManager] instead.\n\nIf you are registering multiple properties/method which depend on one another, be aware that by default undo operation are called in the same order they have been added. Therefore instead of grouping do operation with their undo operations it is better to group do on one side and undo on the other as shown below.\n\n\n```gdscript\nundo_redo.create_action(\"Add object\")\n\n# DO\nundo_redo.add_do_method(_create_object)\nundo_redo.add_do_method(_add_object_to_singleton)\n\n# UNDO\nundo_redo.add_undo_method(_remove_object_from_singleton)\nundo_redo.add_undo_method(_destroy_that_object)\n\nundo_redo.commit_action()\n```\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct UndoRedo {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`UndoRedo`][crate::classes::UndoRedo].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `UndoRedo` methods](https://docs.godotengine.org/en/stable/classes/class_undoredo.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IUndoRedo: crate::obj::GodotClass < Base = UndoRedo > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl UndoRedo {
        #[doc = "Create a new action. After this is called, do all your calls to [`add_do_method`][`crate::classes::UndoRedo::add_do_method`], [`add_undo_method`][`crate::classes::UndoRedo::add_undo_method`], [`add_do_property`][`crate::classes::UndoRedo::add_do_property`], and [`add_undo_property`][`crate::classes::UndoRedo::add_undo_property`], then commit the action with [`commit_action`][`crate::classes::UndoRedo::commit_action`].\n\nThe way actions are merged is dictated by `merge_mode`.\n\nThe way undo operation are ordered in actions is dictated by `backward_undo_ops`. When `backward_undo_ops` is `false` undo option are ordered in the same order they were added. Which means the first operation to be added will be the first to be undone."]
        pub(crate) fn create_action_full(&mut self, name: CowArg < GString >, merge_mode: crate::classes::undo_redo::MergeMode, backward_undo_ops: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::undo_redo::MergeMode, bool,);
            let args = (name, merge_mode, backward_undo_ops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "create_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_action_ex`][Self::create_action_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Create a new action. After this is called, do all your calls to [`add_do_method`][`crate::classes::UndoRedo::add_do_method`], [`add_undo_method`][`crate::classes::UndoRedo::add_undo_method`], [`add_do_property`][`crate::classes::UndoRedo::add_do_property`], and [`add_undo_property`][`crate::classes::UndoRedo::add_undo_property`], then commit the action with [`commit_action`][`crate::classes::UndoRedo::commit_action`].\n\nThe way actions are merged is dictated by `merge_mode`.\n\nThe way undo operation are ordered in actions is dictated by `backward_undo_ops`. When `backward_undo_ops` is `false` undo option are ordered in the same order they were added. Which means the first operation to be added will be the first to be undone."]
        #[inline]
        pub fn create_action(&mut self, name: impl AsArg < GString >,) {
            self.create_action_ex(name,) . done()
        }
        #[doc = "Create a new action. After this is called, do all your calls to [`add_do_method`][`crate::classes::UndoRedo::add_do_method`], [`add_undo_method`][`crate::classes::UndoRedo::add_undo_method`], [`add_do_property`][`crate::classes::UndoRedo::add_do_property`], and [`add_undo_property`][`crate::classes::UndoRedo::add_undo_property`], then commit the action with [`commit_action`][`crate::classes::UndoRedo::commit_action`].\n\nThe way actions are merged is dictated by `merge_mode`.\n\nThe way undo operation are ordered in actions is dictated by `backward_undo_ops`. When `backward_undo_ops` is `false` undo option are ordered in the same order they were added. Which means the first operation to be added will be the first to be undone."]
        #[inline]
        pub fn create_action_ex < 'ex > (&'ex mut self, name: impl AsArg < GString > + 'ex,) -> ExCreateAction < 'ex > {
            ExCreateAction::new(self, name,)
        }
        #[doc = "Commit the action. If `execute` is `true` (which it is by default), all \"do\" methods/properties are called/set when this function is called."]
        pub(crate) fn commit_action_full(&mut self, execute: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (execute,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "commit_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`commit_action_ex`][Self::commit_action_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Commit the action. If `execute` is `true` (which it is by default), all \"do\" methods/properties are called/set when this function is called."]
        #[inline]
        pub fn commit_action(&mut self,) {
            self.commit_action_ex() . done()
        }
        #[doc = "Commit the action. If `execute` is `true` (which it is by default), all \"do\" methods/properties are called/set when this function is called."]
        #[inline]
        pub fn commit_action_ex < 'ex > (&'ex mut self,) -> ExCommitAction < 'ex > {
            ExCommitAction::new(self,)
        }
        #[doc = "Returns `true` if the `UndoRedo` is currently committing the action, i.e. running its \"do\" method or property change (see [`commit_action`][`crate::classes::UndoRedo::commit_action`])."]
        pub fn is_committing_action(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "is_committing_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Register a [`Callable`][crate::builtin::Callable] that will be called when the action is committed."]
        pub fn add_do_method(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "add_do_method", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Register a [`Callable`][crate::builtin::Callable] that will be called when the action is undone."]
        pub fn add_undo_method(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "add_undo_method", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Register a `property` that would change its value to `value` when the action is committed."]
        pub fn add_do_property(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >, RefArg < 'a2, Variant >,);
            let args = (object.into_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "add_do_property", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Register a `property` that would change its value to `value` when the action is undone."]
        pub fn add_undo_property(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >, RefArg < 'a2, Variant >,);
            let args = (object.into_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "add_undo_property", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Register a reference to an object that will be erased if the \"do\" history is deleted. This is useful for objects added by the \"do\" action and removed by the \"undo\" action.\n\nWhen the \"do\" history is deleted, if the object is a [`RefCounted`][crate::classes::RefCounted], it will be unreferenced. Otherwise, it will be freed. Do not use for resources.\n\n```gdscript\nvar node = Node2D.new()\nundo_redo.create_action(\"Add node\")\nundo_redo.add_do_method(add_child.bind(node))\nundo_redo.add_do_reference(node)\nundo_redo.add_undo_method(remove_child.bind(node))\nundo_redo.commit_action()\n```"]
        pub fn add_do_reference(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "add_do_reference", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Register a reference to an object that will be erased if the \"undo\" history is deleted. This is useful for objects added by the \"undo\" action and removed by the \"do\" action.\n\nWhen the \"undo\" history is deleted, if the object is a [`RefCounted`][crate::classes::RefCounted], it will be unreferenced. Otherwise, it will be freed. Do not use for resources.\n\n```gdscript\nvar node = $Node2D\nundo_redo.create_action(\"Remove node\")\nundo_redo.add_do_method(remove_child.bind(node))\nundo_redo.add_undo_method(add_child.bind(node))\nundo_redo.add_undo_reference(node)\nundo_redo.commit_action()\n```"]
        pub fn add_undo_reference(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "add_undo_reference", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Marks the next \"do\" and \"undo\" operations to be processed even if the action gets merged with another in the [`MergeMode::ENDS`][`crate::classes::undo_redo::MergeMode::ENDS`] mode. Return to normal operation using [`end_force_keep_in_merge_ends`][`crate::classes::UndoRedo::end_force_keep_in_merge_ends`]."]
        pub fn start_force_keep_in_merge_ends(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "start_force_keep_in_merge_ends", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stops marking operations as to be processed even if the action gets merged with another in the [`MergeMode::ENDS`][`crate::classes::undo_redo::MergeMode::ENDS`] mode. See [`start_force_keep_in_merge_ends`][`crate::classes::UndoRedo::start_force_keep_in_merge_ends`]."]
        pub fn end_force_keep_in_merge_ends(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "end_force_keep_in_merge_ends", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns how many elements are in the history."]
        pub fn get_history_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "get_history_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the index of the current action."]
        pub fn get_current_action(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "get_current_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the action name from its index."]
        pub fn get_action_name(&self, id: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "get_action_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clear the undo/redo history and associated references.\n\nPassing `false` to `increase_version` will prevent the version number from increasing when the history is cleared."]
        pub(crate) fn clear_history_full(&mut self, increase_version: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (increase_version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "clear_history", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`clear_history_ex`][Self::clear_history_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Clear the undo/redo history and associated references.\n\nPassing `false` to `increase_version` will prevent the version number from increasing when the history is cleared."]
        #[inline]
        pub fn clear_history(&mut self,) {
            self.clear_history_ex() . done()
        }
        #[doc = "Clear the undo/redo history and associated references.\n\nPassing `false` to `increase_version` will prevent the version number from increasing when the history is cleared."]
        #[inline]
        pub fn clear_history_ex < 'ex > (&'ex mut self,) -> ExClearHistory < 'ex > {
            ExClearHistory::new(self,)
        }
        #[doc = "Gets the name of the current action, equivalent to `get_action_name(get_current_action())`."]
        pub fn get_current_action_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "get_current_action_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if an \"undo\" action is available."]
        pub fn has_undo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "has_undo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a \"redo\" action is available."]
        pub fn has_redo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "has_redo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the version. Every time a new action is committed, the `UndoRedo`'s version number is increased automatically.\n\nThis is useful mostly to check if something changed from a saved version."]
        pub fn get_version(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "get_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_steps(&mut self, max_steps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_steps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "set_max_steps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_steps(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "get_max_steps", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Redo the last action."]
        pub fn redo(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "redo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Undo the last action."]
        pub fn undo(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "undo", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for UndoRedo {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("UndoRedo"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for UndoRedo {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for UndoRedo {
        
    }
    impl crate::obj::cap::GodotDefault for UndoRedo {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for UndoRedo {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for UndoRedo {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`UndoRedo`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_UndoRedo__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::UndoRedo > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`UndoRedo::create_action_ex`][super::UndoRedo::create_action_ex]."]
#[must_use]
pub struct ExCreateAction < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::UndoRedo, name: CowArg < 'ex, GString >, merge_mode: crate::classes::undo_redo::MergeMode, backward_undo_ops: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateAction < 'ex > {
    fn new(surround_object: &'ex mut re_export::UndoRedo, name: impl AsArg < GString > + 'ex,) -> Self {
        let merge_mode = crate::obj::EngineEnum::from_ord(0);
        let backward_undo_ops = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), merge_mode: merge_mode, backward_undo_ops: backward_undo_ops,
        }
    }
    #[inline]
    pub fn merge_mode(self, merge_mode: crate::classes::undo_redo::MergeMode) -> Self {
        Self {
            merge_mode: merge_mode, .. self
        }
    }
    #[inline]
    pub fn backward_undo_ops(self, backward_undo_ops: bool) -> Self {
        Self {
            backward_undo_ops: backward_undo_ops, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, merge_mode, backward_undo_ops,
        }
        = self;
        re_export::UndoRedo::create_action_full(surround_object, name, merge_mode, backward_undo_ops,)
    }
}
#[doc = "Default-param extender for [`UndoRedo::commit_action_ex`][super::UndoRedo::commit_action_ex]."]
#[must_use]
pub struct ExCommitAction < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::UndoRedo, execute: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCommitAction < 'ex > {
    fn new(surround_object: &'ex mut re_export::UndoRedo,) -> Self {
        let execute = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, execute: execute,
        }
    }
    #[inline]
    pub fn execute(self, execute: bool) -> Self {
        Self {
            execute: execute, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, execute,
        }
        = self;
        re_export::UndoRedo::commit_action_full(surround_object, execute,)
    }
}
#[doc = "Default-param extender for [`UndoRedo::clear_history_ex`][super::UndoRedo::clear_history_ex]."]
#[must_use]
pub struct ExClearHistory < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::UndoRedo, increase_version: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExClearHistory < 'ex > {
    fn new(surround_object: &'ex mut re_export::UndoRedo,) -> Self {
        let increase_version = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, increase_version: increase_version,
        }
    }
    #[inline]
    pub fn increase_version(self, increase_version: bool) -> Self {
        Self {
            increase_version: increase_version, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, increase_version,
        }
        = self;
        re_export::UndoRedo::clear_history_full(surround_object, increase_version,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MergeMode {
    ord: i32
}
impl MergeMode {
    #[doc(alias = "MERGE_DISABLE")]
    #[doc = "Godot enumerator name: `MERGE_DISABLE`"]
    pub const DISABLE: MergeMode = MergeMode {
        ord: 0i32
    };
    #[doc(alias = "MERGE_ENDS")]
    #[doc = "Godot enumerator name: `MERGE_ENDS`"]
    pub const ENDS: MergeMode = MergeMode {
        ord: 1i32
    };
    #[doc(alias = "MERGE_ALL")]
    #[doc = "Godot enumerator name: `MERGE_ALL`"]
    pub const ALL: MergeMode = MergeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MergeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MergeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MergeMode {
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
            Self::DISABLE => "DISABLE", Self::ENDS => "ENDS", Self::ALL => "ALL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MergeMode::DISABLE, MergeMode::ENDS, MergeMode::ALL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MergeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLE", "MERGE_DISABLE", MergeMode::DISABLE), crate::meta::inspect::EnumConstant::new("ENDS", "MERGE_ENDS", MergeMode::ENDS), crate::meta::inspect::EnumConstant::new("ALL", "MERGE_ALL", MergeMode::ALL)]
        }
    }
}
impl crate::meta::GodotConvert for MergeMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Merge Disable", 0i64), EnumeratorShape::new_int("Merge Ends", 1i64), EnumeratorShape::new_int("Merge All", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("UndoRedo.MergeMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for MergeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MergeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for MergeMode {
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
impl crate::registry::property::Export for MergeMode {
    
}
impl crate::meta::Element for MergeMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::UndoRedo;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`UndoRedo`][crate::classes::UndoRedo] class."]
    pub struct SignalsOfUndoRedo < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfUndoRedo < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn version_changed(&mut self) -> SigVersionChanged < 'c, C > {
            SigVersionChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "version_changed")
            }
        }
    }
    type TypedSigVersionChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigVersionChanged < 'c, C: WithSignals > {
        typed: TypedSigVersionChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigVersionChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigVersionChanged < 'c, C > {
        type Target = TypedSigVersionChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigVersionChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for UndoRedo {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfUndoRedo < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfUndoRedo < 'c, C > {
        type Target = < < UndoRedo as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = UndoRedo;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfUndoRedo < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = UndoRedo;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
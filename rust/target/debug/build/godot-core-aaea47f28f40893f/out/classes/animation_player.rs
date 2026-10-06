#![doc = "Sidecar module for class [`AnimationPlayer`][crate::classes::AnimationPlayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationPlayer` enums](https://docs.godotengine.org/en/stable/classes/class_animationplayer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationPlayer`.\n\nInherits [`AnimationMixer`][crate::classes::AnimationMixer].\n\nRelated symbols:\n\n* [`animation_player`][crate::classes::animation_player]: sidecar module with related enum/flag types\n* [`IAnimationPlayer`][crate::classes::IAnimationPlayer]: virtual methods\n* [`SignalsOfAnimationPlayer`][crate::classes::animation_player::SignalsOfAnimationPlayer]: signal collection\n\n\nSee also [Godot docs for `AnimationPlayer`](https://docs.godotengine.org/en/stable/classes/class_animationplayer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`AnimationPlayer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot.\n# Godot docs\nAn animation player is used for general-purpose playback of animations. It contains a dictionary of [`AnimationLibrary`][crate::classes::AnimationLibrary] resources and custom blend times between animation transitions.\n\nSome methods and properties use a single key to reference an animation directly. These keys are formatted as the key for the library, followed by a forward slash, then the key for the animation within the library, for example `\"movement/run\"`. If the library's key is an empty string (known as the default library), the forward slash is omitted, being the same key used by the library.\n\n`AnimationPlayer` is better-suited than [`Tween`][crate::classes::Tween] for more complex animations, for example ones with non-trivial timings. It can also be used over [`Tween`][crate::classes::Tween] if the animation track editor is more convenient than doing it in code.\n\nUpdating the target properties of animations occurs at the process frame."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationPlayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimationPlayer`][crate::classes::AnimationPlayer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IAnimationMixer`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `AnimationPlayer` methods](https://docs.godotengine.org/en/stable/classes/class_animationplayer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationPlayer: crate::obj::GodotClass < Base = AnimationPlayer > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
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
        #[doc = "A virtual function for processing after getting a key during playback."]
        fn post_process_key_value(&self, animation: Option < Gd < crate::classes::Animation > >, track: i32, value: Variant, object_id: u64, object_sub_idx: i32,) -> Variant {
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
    impl AnimationPlayer {
        #[doc = "Triggers the `animation_to` animation when the `animation_from` animation completes."]
        pub fn animation_set_next(&mut self, animation_from: impl AsArg < StringName >, animation_to: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (animation_from.into_arg(), animation_to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10539usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "animation_set_next", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the key of the animation which is queued to play after the `animation_from` animation."]
        pub fn animation_get_next(&self, animation_from: impl AsArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (animation_from.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10540usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "animation_get_next", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Specifies a blend time (in seconds) between two animations, referenced by their keys."]
        pub fn set_blend_time(&mut self, animation_from: impl AsArg < StringName >, animation_to: impl AsArg < StringName >, sec: f64,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, f64,);
            let args = (animation_from.into_arg(), animation_to.into_arg(), sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10541usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_blend_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the blend time (in seconds) between two animations, referenced by their keys."]
        pub fn get_blend_time(&self, animation_from: impl AsArg < StringName >, animation_to: impl AsArg < StringName >,) -> f64 {
            type CallRet = f64;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (animation_from.into_arg(), animation_to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_blend_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_default_blend_time(&mut self, sec: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_default_blend_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_default_blend_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_default_blend_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_capture(&mut self, auto_capture: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (auto_capture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10545usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_auto_capture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_auto_capture(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10546usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "is_auto_capture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_capture_duration(&mut self, auto_capture_duration: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (auto_capture_duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10547usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_auto_capture_duration", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_capture_duration(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10548usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_auto_capture_duration", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_capture_transition_type(&mut self, auto_capture_transition_type: crate::classes::tween::TransitionType,) {
            type CallRet = ();
            type CallParams = (crate::classes::tween::TransitionType,);
            let args = (auto_capture_transition_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10549usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_auto_capture_transition_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_capture_transition_type(&self,) -> crate::classes::tween::TransitionType {
            type CallRet = crate::classes::tween::TransitionType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10550usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_auto_capture_transition_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_capture_ease_type(&mut self, auto_capture_ease_type: crate::classes::tween::EaseType,) {
            type CallRet = ();
            type CallParams = (crate::classes::tween::EaseType,);
            let args = (auto_capture_ease_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10551usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_auto_capture_ease_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_capture_ease_type(&self,) -> crate::classes::tween::EaseType {
            type CallRet = crate::classes::tween::EaseType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10552usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_auto_capture_ease_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Plays the animation with key `name`. Custom blend times and speed can be set.\n\nThe `from_end` option only affects when switching to a new animation track, or if the same track but at the start or end. It does not affect resuming playback that was paused in the middle of an animation. If `custom_speed` is negative and `from_end` is `true`, the animation will play backwards (which is equivalent to calling [`play_backwards`][`crate::classes::AnimationPlayer::play_backwards`]).\n\nThe `AnimationPlayer` keeps track of its current or last played animation with \\[member assigned_animation]. If this method is called with that same animation `name`, or with no `name` parameter, the assigned animation will resume playing if it was paused.\n\n**Note:** The animation will be updated the next time the `AnimationPlayer` is processed. If other variables are updated at the same time this is called, they may be updated too early. To perform the update immediately, call `advance(0)`."]
        pub(crate) fn play_full(&mut self, name: CowArg < StringName >, custom_blend: f64, custom_speed: f32, from_end: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f64, f32, bool,);
            let args = (name, custom_blend, custom_speed, from_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10553usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "play", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`play_ex`][Self::play_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Plays the animation with key `name`. Custom blend times and speed can be set.\n\nThe `from_end` option only affects when switching to a new animation track, or if the same track but at the start or end. It does not affect resuming playback that was paused in the middle of an animation. If `custom_speed` is negative and `from_end` is `true`, the animation will play backwards (which is equivalent to calling [`play_backwards`][`crate::classes::AnimationPlayer::play_backwards`]).\n\nThe `AnimationPlayer` keeps track of its current or last played animation with \\[member assigned_animation]. If this method is called with that same animation `name`, or with no `name` parameter, the assigned animation will resume playing if it was paused.\n\n**Note:** The animation will be updated the next time the `AnimationPlayer` is processed. If other variables are updated at the same time this is called, they may be updated too early. To perform the update immediately, call `advance(0)`."]
        #[inline]
        pub fn play(&mut self,) {
            self.play_ex() . done()
        }
        #[doc = "Plays the animation with key `name`. Custom blend times and speed can be set.\n\nThe `from_end` option only affects when switching to a new animation track, or if the same track but at the start or end. It does not affect resuming playback that was paused in the middle of an animation. If `custom_speed` is negative and `from_end` is `true`, the animation will play backwards (which is equivalent to calling [`play_backwards`][`crate::classes::AnimationPlayer::play_backwards`]).\n\nThe `AnimationPlayer` keeps track of its current or last played animation with \\[member assigned_animation]. If this method is called with that same animation `name`, or with no `name` parameter, the assigned animation will resume playing if it was paused.\n\n**Note:** The animation will be updated the next time the `AnimationPlayer` is processed. If other variables are updated at the same time this is called, they may be updated too early. To perform the update immediately, call `advance(0)`."]
        #[inline]
        pub fn play_ex < 'ex > (&'ex mut self,) -> ExPlay < 'ex > {
            ExPlay::new(self,)
        }
        #[doc = "Plays the animation with key `name` and the section starting from `start_marker` and ending on `end_marker`.\n\nIf the start marker is empty, the section starts from the beginning of the animation. If the end marker is empty, the section ends on the end of the animation. See also [`play`][`crate::classes::AnimationPlayer::play`]."]
        pub(crate) fn play_section_with_markers_full(&mut self, name: CowArg < StringName >, start_marker: CowArg < StringName >, end_marker: CowArg < StringName >, custom_blend: f64, custom_speed: f32, from_end: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >, f64, f32, bool,);
            let args = (name, start_marker, end_marker, custom_blend, custom_speed, from_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "play_section_with_markers", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`play_section_with_markers_ex`][Self::play_section_with_markers_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Plays the animation with key `name` and the section starting from `start_marker` and ending on `end_marker`.\n\nIf the start marker is empty, the section starts from the beginning of the animation. If the end marker is empty, the section ends on the end of the animation. See also [`play`][`crate::classes::AnimationPlayer::play`]."]
        #[inline]
        pub fn play_section_with_markers(&mut self,) {
            self.play_section_with_markers_ex() . done()
        }
        #[doc = "Plays the animation with key `name` and the section starting from `start_marker` and ending on `end_marker`.\n\nIf the start marker is empty, the section starts from the beginning of the animation. If the end marker is empty, the section ends on the end of the animation. See also [`play`][`crate::classes::AnimationPlayer::play`]."]
        #[inline]
        pub fn play_section_with_markers_ex < 'ex > (&'ex mut self,) -> ExPlaySectionWithMarkers < 'ex > {
            ExPlaySectionWithMarkers::new(self,)
        }
        #[doc = "Plays the animation with key `name` and the section starting from `start_time` and ending on `end_time`. See also [`play`][`crate::classes::AnimationPlayer::play`].\n\nSetting `start_time` to a value outside the range of the animation means the start of the animation will be used instead, and setting `end_time` to a value outside the range of the animation means the end of the animation will be used instead. `start_time` cannot be equal to `end_time`."]
        pub(crate) fn play_section_full(&mut self, name: CowArg < StringName >, start_time: f64, end_time: f64, custom_blend: f64, custom_speed: f32, from_end: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f64, f64, f64, f32, bool,);
            let args = (name, start_time, end_time, custom_blend, custom_speed, from_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "play_section", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`play_section_ex`][Self::play_section_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Plays the animation with key `name` and the section starting from `start_time` and ending on `end_time`. See also [`play`][`crate::classes::AnimationPlayer::play`].\n\nSetting `start_time` to a value outside the range of the animation means the start of the animation will be used instead, and setting `end_time` to a value outside the range of the animation means the end of the animation will be used instead. `start_time` cannot be equal to `end_time`."]
        #[inline]
        pub fn play_section(&mut self,) {
            self.play_section_ex() . done()
        }
        #[doc = "Plays the animation with key `name` and the section starting from `start_time` and ending on `end_time`. See also [`play`][`crate::classes::AnimationPlayer::play`].\n\nSetting `start_time` to a value outside the range of the animation means the start of the animation will be used instead, and setting `end_time` to a value outside the range of the animation means the end of the animation will be used instead. `start_time` cannot be equal to `end_time`."]
        #[inline]
        pub fn play_section_ex < 'ex > (&'ex mut self,) -> ExPlaySection < 'ex > {
            ExPlaySection::new(self,)
        }
        #[doc = "Plays the animation with key `name` in reverse.\n\nThis method is a shorthand for [`play`][`crate::classes::AnimationPlayer::play`] with `custom_speed = -1.0` and `from_end = true`, so see its description for more information."]
        pub(crate) fn play_backwards_full(&mut self, name: CowArg < StringName >, custom_blend: f64,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f64,);
            let args = (name, custom_blend,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "play_backwards", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`play_backwards_ex`][Self::play_backwards_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Plays the animation with key `name` in reverse.\n\nThis method is a shorthand for [`play`][`crate::classes::AnimationPlayer::play`] with `custom_speed = -1.0` and `from_end = true`, so see its description for more information."]
        #[inline]
        pub fn play_backwards(&mut self,) {
            self.play_backwards_ex() . done()
        }
        #[doc = "Plays the animation with key `name` in reverse.\n\nThis method is a shorthand for [`play`][`crate::classes::AnimationPlayer::play`] with `custom_speed = -1.0` and `from_end = true`, so see its description for more information."]
        #[inline]
        pub fn play_backwards_ex < 'ex > (&'ex mut self,) -> ExPlayBackwards < 'ex > {
            ExPlayBackwards::new(self,)
        }
        #[doc = "Plays the animation with key `name` and the section starting from `start_marker` and ending on `end_marker` in reverse.\n\nThis method is a shorthand for [`play_section_with_markers`][`crate::classes::AnimationPlayer::play_section_with_markers`] with `custom_speed = -1.0` and `from_end = true`, see its description for more information."]
        pub(crate) fn play_section_with_markers_backwards_full(&mut self, name: CowArg < StringName >, start_marker: CowArg < StringName >, end_marker: CowArg < StringName >, custom_blend: f64,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >, f64,);
            let args = (name, start_marker, end_marker, custom_blend,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "play_section_with_markers_backwards", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`play_section_with_markers_backwards_ex`][Self::play_section_with_markers_backwards_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Plays the animation with key `name` and the section starting from `start_marker` and ending on `end_marker` in reverse.\n\nThis method is a shorthand for [`play_section_with_markers`][`crate::classes::AnimationPlayer::play_section_with_markers`] with `custom_speed = -1.0` and `from_end = true`, see its description for more information."]
        #[inline]
        pub fn play_section_with_markers_backwards(&mut self,) {
            self.play_section_with_markers_backwards_ex() . done()
        }
        #[doc = "Plays the animation with key `name` and the section starting from `start_marker` and ending on `end_marker` in reverse.\n\nThis method is a shorthand for [`play_section_with_markers`][`crate::classes::AnimationPlayer::play_section_with_markers`] with `custom_speed = -1.0` and `from_end = true`, see its description for more information."]
        #[inline]
        pub fn play_section_with_markers_backwards_ex < 'ex > (&'ex mut self,) -> ExPlaySectionWithMarkersBackwards < 'ex > {
            ExPlaySectionWithMarkersBackwards::new(self,)
        }
        #[doc = "Plays the animation with key `name` and the section starting from `start_time` and ending on `end_time` in reverse.\n\nThis method is a shorthand for [`play_section`][`crate::classes::AnimationPlayer::play_section`] with `custom_speed = -1.0` and `from_end = true`, see its description for more information."]
        pub(crate) fn play_section_backwards_full(&mut self, name: CowArg < StringName >, start_time: f64, end_time: f64, custom_blend: f64,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f64, f64, f64,);
            let args = (name, start_time, end_time, custom_blend,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "play_section_backwards", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`play_section_backwards_ex`][Self::play_section_backwards_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Plays the animation with key `name` and the section starting from `start_time` and ending on `end_time` in reverse.\n\nThis method is a shorthand for [`play_section`][`crate::classes::AnimationPlayer::play_section`] with `custom_speed = -1.0` and `from_end = true`, see its description for more information."]
        #[inline]
        pub fn play_section_backwards(&mut self,) {
            self.play_section_backwards_ex() . done()
        }
        #[doc = "Plays the animation with key `name` and the section starting from `start_time` and ending on `end_time` in reverse.\n\nThis method is a shorthand for [`play_section`][`crate::classes::AnimationPlayer::play_section`] with `custom_speed = -1.0` and `from_end = true`, see its description for more information."]
        #[inline]
        pub fn play_section_backwards_ex < 'ex > (&'ex mut self,) -> ExPlaySectionBackwards < 'ex > {
            ExPlaySectionBackwards::new(self,)
        }
        #[doc = "See also [`capture`][`crate::classes::AnimationMixer::capture`].\n\nYou can use this method to use more detailed options for capture than those performed by \\[member playback_auto_capture]. When \\[member playback_auto_capture] is `false`, this method is almost the same as the following:\n\n```gdscript\ncapture(name, duration, trans_type, ease_type)\nplay(name, custom_blend, custom_speed, from_end)\n```\n\nIf `name` is blank, it specifies \\[member assigned_animation].\n\nIf `duration` is a negative value, the duration is set to the interval between the current position and the first key, when `from_end` is `true`, uses the interval between the current position and the last key instead.\n\n**Note:** The `duration` takes \\[member speed_scale] into account, but `custom_speed` does not, because the capture cache is interpolated with the blend result and the result may contain multiple animations."]
        pub(crate) fn play_with_capture_full(&mut self, name: CowArg < StringName >, duration: f64, custom_blend: f64, custom_speed: f32, from_end: bool, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f64, f64, f32, bool, crate::classes::tween::TransitionType, crate::classes::tween::EaseType,);
            let args = (name, duration, custom_blend, custom_speed, from_end, trans_type, ease_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "play_with_capture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`play_with_capture_ex`][Self::play_with_capture_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "See also [`capture`][`crate::classes::AnimationMixer::capture`].\n\nYou can use this method to use more detailed options for capture than those performed by \\[member playback_auto_capture]. When \\[member playback_auto_capture] is `false`, this method is almost the same as the following:\n\n```gdscript\ncapture(name, duration, trans_type, ease_type)\nplay(name, custom_blend, custom_speed, from_end)\n```\n\nIf `name` is blank, it specifies \\[member assigned_animation].\n\nIf `duration` is a negative value, the duration is set to the interval between the current position and the first key, when `from_end` is `true`, uses the interval between the current position and the last key instead.\n\n**Note:** The `duration` takes \\[member speed_scale] into account, but `custom_speed` does not, because the capture cache is interpolated with the blend result and the result may contain multiple animations."]
        #[inline]
        pub fn play_with_capture(&mut self,) {
            self.play_with_capture_ex() . done()
        }
        #[doc = "See also [`capture`][`crate::classes::AnimationMixer::capture`].\n\nYou can use this method to use more detailed options for capture than those performed by \\[member playback_auto_capture]. When \\[member playback_auto_capture] is `false`, this method is almost the same as the following:\n\n```gdscript\ncapture(name, duration, trans_type, ease_type)\nplay(name, custom_blend, custom_speed, from_end)\n```\n\nIf `name` is blank, it specifies \\[member assigned_animation].\n\nIf `duration` is a negative value, the duration is set to the interval between the current position and the first key, when `from_end` is `true`, uses the interval between the current position and the last key instead.\n\n**Note:** The `duration` takes \\[member speed_scale] into account, but `custom_speed` does not, because the capture cache is interpolated with the blend result and the result may contain multiple animations."]
        #[inline]
        pub fn play_with_capture_ex < 'ex > (&'ex mut self,) -> ExPlayWithCapture < 'ex > {
            ExPlayWithCapture::new(self,)
        }
        #[doc = "Pauses the currently playing animation. The \\[member current_animation_position] will be kept and calling [`play`][`crate::classes::AnimationPlayer::play`] or [`play_backwards`][`crate::classes::AnimationPlayer::play_backwards`] without arguments or with the same animation name as \\[member assigned_animation] will resume the animation.\n\nSee also [`stop`][`crate::classes::AnimationPlayer::stop`]."]
        pub fn pause(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "pause", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stops the currently playing animation. The animation position is reset to `0` and the `custom_speed` is reset to `1.0`. See also [`pause`][`crate::classes::AnimationPlayer::pause`].\n\nIf `keep_state` is `true`, the animation state is not updated visually.\n\n**Note:** The method / audio / animation playback tracks will not be processed by this method."]
        pub(crate) fn stop_full(&mut self, keep_state: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (keep_state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10561usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "stop", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`stop_ex`][Self::stop_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Stops the currently playing animation. The animation position is reset to `0` and the `custom_speed` is reset to `1.0`. See also [`pause`][`crate::classes::AnimationPlayer::pause`].\n\nIf `keep_state` is `true`, the animation state is not updated visually.\n\n**Note:** The method / audio / animation playback tracks will not be processed by this method."]
        #[inline]
        pub fn stop(&mut self,) {
            self.stop_ex() . done()
        }
        #[doc = "Stops the currently playing animation. The animation position is reset to `0` and the `custom_speed` is reset to `1.0`. See also [`pause`][`crate::classes::AnimationPlayer::pause`].\n\nIf `keep_state` is `true`, the animation state is not updated visually.\n\n**Note:** The method / audio / animation playback tracks will not be processed by this method."]
        #[inline]
        pub fn stop_ex < 'ex > (&'ex mut self,) -> ExStop < 'ex > {
            ExStop::new(self,)
        }
        #[doc = "Returns `true` if an animation is currently playing (even if \\[member speed_scale] and/or `custom_speed` are `0`)."]
        pub fn is_playing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10562usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "is_playing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the an animation is currently active. An animation is active if it was played by calling [`play`][`crate::classes::AnimationPlayer::play`] and was not finished yet, or was stopped by calling [`stop`][`crate::classes::AnimationPlayer::stop`].\n\nThis can be used to check whether an animation is currently paused or stopped.\n\n```gdscript\nvar is_paused = not is_playing() and is_animation_active()\nvar is_stopped = not is_playing() and not is_animation_active()\n```"]
        pub fn is_animation_active(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10563usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "is_animation_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_animation(&mut self, animation: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (animation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10564usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_current_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_animation(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10565usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_current_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_assigned_animation(&mut self, animation: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (animation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10566usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_assigned_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_assigned_animation(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10567usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_assigned_animation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Queues an animation for playback once the current animation and all previously queued animations are done.\n\n**Note:** If a looped animation is currently playing, the queued animation will never play unless the looped animation is stopped somehow."]
        pub fn queue(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10568usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "queue", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a list of the animation keys that are currently queued to play."]
        pub fn get_queue(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10569usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_queue", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all queued, unplayed animations."]
        pub fn clear_queue(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10570usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "clear_queue", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_speed_scale(&mut self, speed: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the actual playing speed of current animation or `0` if not playing. This speed is the \\[member speed_scale] property multiplied by `custom_speed` argument specified when calling the [`play`][`crate::classes::AnimationPlayer::play`] method.\n\nReturns a negative value if the current animation is playing backwards."]
        pub fn get_playing_speed(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10573usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_playing_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autoplay(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10574usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_autoplay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autoplay(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10575usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_autoplay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_movie_quit_on_finish_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10576usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_movie_quit_on_finish_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_movie_quit_on_finish_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10577usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "is_movie_quit_on_finish_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_animation_position(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10578usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_current_animation_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_animation_length(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10579usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_current_animation_length", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Changes the start and end markers of the section being played. The current playback position will be clamped within the new section. See also [`play_section_with_markers`][`crate::classes::AnimationPlayer::play_section_with_markers`].\n\nIf the argument is empty, the section uses the beginning or end of the animation. If both are empty, it means that the section is not set."]
        pub(crate) fn set_section_with_markers_full(&mut self, start_marker: CowArg < StringName >, end_marker: CowArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (start_marker, end_marker,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10580usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_section_with_markers", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_section_with_markers_ex`][Self::set_section_with_markers_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Changes the start and end markers of the section being played. The current playback position will be clamped within the new section. See also [`play_section_with_markers`][`crate::classes::AnimationPlayer::play_section_with_markers`].\n\nIf the argument is empty, the section uses the beginning or end of the animation. If both are empty, it means that the section is not set."]
        #[inline]
        pub fn set_section_with_markers(&mut self,) {
            self.set_section_with_markers_ex() . done()
        }
        #[doc = "Changes the start and end markers of the section being played. The current playback position will be clamped within the new section. See also [`play_section_with_markers`][`crate::classes::AnimationPlayer::play_section_with_markers`].\n\nIf the argument is empty, the section uses the beginning or end of the animation. If both are empty, it means that the section is not set."]
        #[inline]
        pub fn set_section_with_markers_ex < 'ex > (&'ex mut self,) -> ExSetSectionWithMarkers < 'ex > {
            ExSetSectionWithMarkers::new(self,)
        }
        #[doc = "Changes the start and end times of the section being played. The current playback position will be clamped within the new section. See also [`play_section`][`crate::classes::AnimationPlayer::play_section`]."]
        pub(crate) fn set_section_full(&mut self, start_time: f64, end_time: f64,) {
            type CallRet = ();
            type CallParams = (f64, f64,);
            let args = (start_time, end_time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_section", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_section_ex`][Self::set_section_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Changes the start and end times of the section being played. The current playback position will be clamped within the new section. See also [`play_section`][`crate::classes::AnimationPlayer::play_section`]."]
        #[inline]
        pub fn set_section(&mut self,) {
            self.set_section_ex() . done()
        }
        #[doc = "Changes the start and end times of the section being played. The current playback position will be clamped within the new section. See also [`play_section`][`crate::classes::AnimationPlayer::play_section`]."]
        #[inline]
        pub fn set_section_ex < 'ex > (&'ex mut self,) -> ExSetSection < 'ex > {
            ExSetSection::new(self,)
        }
        #[doc = "Resets the current section. Does nothing if a section has not been set."]
        pub fn reset_section(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "reset_section", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the start time of the section currently being played."]
        pub fn get_section_start_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_section_start_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the end time of the section currently being played."]
        pub fn get_section_end_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_section_end_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if an animation is currently playing with a section."]
        pub fn has_section(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "has_section", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Seeks the animation to the `seconds` point in time (in seconds). If `update` is `true`, the animation updates too, otherwise it updates at process time. Events between the current frame and `seconds` are skipped.\n\nIf `update_only` is `true`, the method / audio / animation playback tracks will not be processed.\n\n**Note:** Seeking to the end of the animation doesn't emit `AnimationMixer.animation_finished`. If you want to skip animation and emit the signal, use [`advance`][`crate::classes::AnimationMixer::advance`]."]
        pub(crate) fn seek_full(&mut self, seconds: f64, update: bool, update_only: bool,) {
            type CallRet = ();
            type CallParams = (f64, bool, bool,);
            let args = (seconds, update, update_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "seek", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`seek_ex`][Self::seek_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Seeks the animation to the `seconds` point in time (in seconds). If `update` is `true`, the animation updates too, otherwise it updates at process time. Events between the current frame and `seconds` are skipped.\n\nIf `update_only` is `true`, the method / audio / animation playback tracks will not be processed.\n\n**Note:** Seeking to the end of the animation doesn't emit `AnimationMixer.animation_finished`. If you want to skip animation and emit the signal, use [`advance`][`crate::classes::AnimationMixer::advance`]."]
        #[inline]
        pub fn seek(&mut self, seconds: f64,) {
            self.seek_ex(seconds,) . done()
        }
        #[doc = "Seeks the animation to the `seconds` point in time (in seconds). If `update` is `true`, the animation updates too, otherwise it updates at process time. Events between the current frame and `seconds` are skipped.\n\nIf `update_only` is `true`, the method / audio / animation playback tracks will not be processed.\n\n**Note:** Seeking to the end of the animation doesn't emit `AnimationMixer.animation_finished`. If you want to skip animation and emit the signal, use [`advance`][`crate::classes::AnimationMixer::advance`]."]
        #[inline]
        pub fn seek_ex < 'ex > (&'ex mut self, seconds: f64,) -> ExSeek < 'ex > {
            ExSeek::new(self, seconds,)
        }
        #[doc = "Sets the process notification in which to update animations."]
        pub fn set_process_callback(&mut self, mode: crate::classes::animation_player::AnimationProcessCallback,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_player::AnimationProcessCallback,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_process_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the process notification in which to update animations."]
        pub fn get_process_callback(&self,) -> crate::classes::animation_player::AnimationProcessCallback {
            type CallRet = crate::classes::animation_player::AnimationProcessCallback;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_process_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the call mode used for \"Call Method\" tracks."]
        pub fn set_method_call_mode(&mut self, mode: crate::classes::animation_player::AnimationMethodCallMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_player::AnimationMethodCallMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_method_call_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the call mode used for \"Call Method\" tracks."]
        pub fn get_method_call_mode(&self,) -> crate::classes::animation_player::AnimationMethodCallMode {
            type CallRet = crate::classes::animation_player::AnimationMethodCallMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_method_call_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the node which node path references will travel from."]
        pub fn set_root(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "set_root", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the node which node path references will travel from."]
        pub fn get_root(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationPlayer", "get_root", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationPlayer {
        type Base = crate::classes::AnimationMixer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationPlayer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationPlayer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationMixer > for AnimationPlayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AnimationPlayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationPlayer {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationPlayer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationPlayer {
        type Target = crate::classes::AnimationMixer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationPlayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationPlayer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationPlayer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationPlayer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationMixer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::play_ex`][super::AnimationPlayer::play_ex]."]
#[must_use]
pub struct ExPlay < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationPlayer, name: CowArg < 'ex, StringName >, custom_blend: f64, custom_speed: f32, from_end: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPlay < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationPlayer,) -> Self {
        let name = StringName::from("");
        let custom_blend = - 1f64;
        let custom_speed = 1f32;
        let from_end = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name), custom_blend: custom_blend, custom_speed: custom_speed, from_end: from_end,
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn custom_blend(self, custom_blend: f64) -> Self {
        Self {
            custom_blend: custom_blend, .. self
        }
    }
    #[inline]
    pub fn custom_speed(self, custom_speed: f32) -> Self {
        Self {
            custom_speed: custom_speed, .. self
        }
    }
    #[inline]
    pub fn from_end(self, from_end: bool) -> Self {
        Self {
            from_end: from_end, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, custom_blend, custom_speed, from_end,
        }
        = self;
        re_export::AnimationPlayer::play_full(surround_object, name, custom_blend, custom_speed, from_end,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::play_section_with_markers_ex`][super::AnimationPlayer::play_section_with_markers_ex]."]
#[must_use]
pub struct ExPlaySectionWithMarkers < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationPlayer, name: CowArg < 'ex, StringName >, start_marker: CowArg < 'ex, StringName >, end_marker: CowArg < 'ex, StringName >, custom_blend: f64, custom_speed: f32, from_end: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPlaySectionWithMarkers < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationPlayer,) -> Self {
        let name = StringName::from("");
        let start_marker = StringName::from("");
        let end_marker = StringName::from("");
        let custom_blend = - 1f64;
        let custom_speed = 1f32;
        let from_end = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name), start_marker: CowArg::Owned(start_marker), end_marker: CowArg::Owned(end_marker), custom_blend: custom_blend, custom_speed: custom_speed, from_end: from_end,
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn start_marker(self, start_marker: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            start_marker: start_marker.into_arg(), .. self
        }
    }
    #[inline]
    pub fn end_marker(self, end_marker: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            end_marker: end_marker.into_arg(), .. self
        }
    }
    #[inline]
    pub fn custom_blend(self, custom_blend: f64) -> Self {
        Self {
            custom_blend: custom_blend, .. self
        }
    }
    #[inline]
    pub fn custom_speed(self, custom_speed: f32) -> Self {
        Self {
            custom_speed: custom_speed, .. self
        }
    }
    #[inline]
    pub fn from_end(self, from_end: bool) -> Self {
        Self {
            from_end: from_end, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, start_marker, end_marker, custom_blend, custom_speed, from_end,
        }
        = self;
        re_export::AnimationPlayer::play_section_with_markers_full(surround_object, name, start_marker, end_marker, custom_blend, custom_speed, from_end,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::play_section_ex`][super::AnimationPlayer::play_section_ex]."]
#[must_use]
pub struct ExPlaySection < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationPlayer, name: CowArg < 'ex, StringName >, start_time: f64, end_time: f64, custom_blend: f64, custom_speed: f32, from_end: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPlaySection < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationPlayer,) -> Self {
        let name = StringName::from("");
        let start_time = - 1f64;
        let end_time = - 1f64;
        let custom_blend = - 1f64;
        let custom_speed = 1f32;
        let from_end = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name), start_time: start_time, end_time: end_time, custom_blend: custom_blend, custom_speed: custom_speed, from_end: from_end,
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn start_time(self, start_time: f64) -> Self {
        Self {
            start_time: start_time, .. self
        }
    }
    #[inline]
    pub fn end_time(self, end_time: f64) -> Self {
        Self {
            end_time: end_time, .. self
        }
    }
    #[inline]
    pub fn custom_blend(self, custom_blend: f64) -> Self {
        Self {
            custom_blend: custom_blend, .. self
        }
    }
    #[inline]
    pub fn custom_speed(self, custom_speed: f32) -> Self {
        Self {
            custom_speed: custom_speed, .. self
        }
    }
    #[inline]
    pub fn from_end(self, from_end: bool) -> Self {
        Self {
            from_end: from_end, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, start_time, end_time, custom_blend, custom_speed, from_end,
        }
        = self;
        re_export::AnimationPlayer::play_section_full(surround_object, name, start_time, end_time, custom_blend, custom_speed, from_end,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::play_backwards_ex`][super::AnimationPlayer::play_backwards_ex]."]
#[must_use]
pub struct ExPlayBackwards < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationPlayer, name: CowArg < 'ex, StringName >, custom_blend: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPlayBackwards < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationPlayer,) -> Self {
        let name = StringName::from("");
        let custom_blend = - 1f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name), custom_blend: custom_blend,
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn custom_blend(self, custom_blend: f64) -> Self {
        Self {
            custom_blend: custom_blend, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, custom_blend,
        }
        = self;
        re_export::AnimationPlayer::play_backwards_full(surround_object, name, custom_blend,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::play_section_with_markers_backwards_ex`][super::AnimationPlayer::play_section_with_markers_backwards_ex]."]
#[must_use]
pub struct ExPlaySectionWithMarkersBackwards < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationPlayer, name: CowArg < 'ex, StringName >, start_marker: CowArg < 'ex, StringName >, end_marker: CowArg < 'ex, StringName >, custom_blend: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPlaySectionWithMarkersBackwards < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationPlayer,) -> Self {
        let name = StringName::from("");
        let start_marker = StringName::from("");
        let end_marker = StringName::from("");
        let custom_blend = - 1f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name), start_marker: CowArg::Owned(start_marker), end_marker: CowArg::Owned(end_marker), custom_blend: custom_blend,
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn start_marker(self, start_marker: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            start_marker: start_marker.into_arg(), .. self
        }
    }
    #[inline]
    pub fn end_marker(self, end_marker: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            end_marker: end_marker.into_arg(), .. self
        }
    }
    #[inline]
    pub fn custom_blend(self, custom_blend: f64) -> Self {
        Self {
            custom_blend: custom_blend, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, start_marker, end_marker, custom_blend,
        }
        = self;
        re_export::AnimationPlayer::play_section_with_markers_backwards_full(surround_object, name, start_marker, end_marker, custom_blend,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::play_section_backwards_ex`][super::AnimationPlayer::play_section_backwards_ex]."]
#[must_use]
pub struct ExPlaySectionBackwards < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationPlayer, name: CowArg < 'ex, StringName >, start_time: f64, end_time: f64, custom_blend: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPlaySectionBackwards < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationPlayer,) -> Self {
        let name = StringName::from("");
        let start_time = - 1f64;
        let end_time = - 1f64;
        let custom_blend = - 1f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name), start_time: start_time, end_time: end_time, custom_blend: custom_blend,
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn start_time(self, start_time: f64) -> Self {
        Self {
            start_time: start_time, .. self
        }
    }
    #[inline]
    pub fn end_time(self, end_time: f64) -> Self {
        Self {
            end_time: end_time, .. self
        }
    }
    #[inline]
    pub fn custom_blend(self, custom_blend: f64) -> Self {
        Self {
            custom_blend: custom_blend, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, start_time, end_time, custom_blend,
        }
        = self;
        re_export::AnimationPlayer::play_section_backwards_full(surround_object, name, start_time, end_time, custom_blend,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::play_with_capture_ex`][super::AnimationPlayer::play_with_capture_ex]."]
#[must_use]
pub struct ExPlayWithCapture < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationPlayer, name: CowArg < 'ex, StringName >, duration: f64, custom_blend: f64, custom_speed: f32, from_end: bool, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPlayWithCapture < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationPlayer,) -> Self {
        let name = StringName::from("");
        let duration = - 1f64;
        let custom_blend = - 1f64;
        let custom_speed = 1f32;
        let from_end = false;
        let trans_type = crate::obj::EngineEnum::from_ord(0);
        let ease_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name), duration: duration, custom_blend: custom_blend, custom_speed: custom_speed, from_end: from_end, trans_type: trans_type, ease_type: ease_type,
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn duration(self, duration: f64) -> Self {
        Self {
            duration: duration, .. self
        }
    }
    #[inline]
    pub fn custom_blend(self, custom_blend: f64) -> Self {
        Self {
            custom_blend: custom_blend, .. self
        }
    }
    #[inline]
    pub fn custom_speed(self, custom_speed: f32) -> Self {
        Self {
            custom_speed: custom_speed, .. self
        }
    }
    #[inline]
    pub fn from_end(self, from_end: bool) -> Self {
        Self {
            from_end: from_end, .. self
        }
    }
    #[inline]
    pub fn trans_type(self, trans_type: crate::classes::tween::TransitionType) -> Self {
        Self {
            trans_type: trans_type, .. self
        }
    }
    #[inline]
    pub fn ease_type(self, ease_type: crate::classes::tween::EaseType) -> Self {
        Self {
            ease_type: ease_type, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, duration, custom_blend, custom_speed, from_end, trans_type, ease_type,
        }
        = self;
        re_export::AnimationPlayer::play_with_capture_full(surround_object, name, duration, custom_blend, custom_speed, from_end, trans_type, ease_type,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::stop_ex`][super::AnimationPlayer::stop_ex]."]
#[must_use]
pub struct ExStop < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationPlayer, keep_state: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExStop < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationPlayer,) -> Self {
        let keep_state = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, keep_state: keep_state,
        }
    }
    #[inline]
    pub fn keep_state(self, keep_state: bool) -> Self {
        Self {
            keep_state: keep_state, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, keep_state,
        }
        = self;
        re_export::AnimationPlayer::stop_full(surround_object, keep_state,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::set_section_with_markers_ex`][super::AnimationPlayer::set_section_with_markers_ex]."]
#[must_use]
pub struct ExSetSectionWithMarkers < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationPlayer, start_marker: CowArg < 'ex, StringName >, end_marker: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetSectionWithMarkers < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationPlayer,) -> Self {
        let start_marker = StringName::from("");
        let end_marker = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start_marker: CowArg::Owned(start_marker), end_marker: CowArg::Owned(end_marker),
        }
    }
    #[inline]
    pub fn start_marker(self, start_marker: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            start_marker: start_marker.into_arg(), .. self
        }
    }
    #[inline]
    pub fn end_marker(self, end_marker: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            end_marker: end_marker.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start_marker, end_marker,
        }
        = self;
        re_export::AnimationPlayer::set_section_with_markers_full(surround_object, start_marker, end_marker,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::set_section_ex`][super::AnimationPlayer::set_section_ex]."]
#[must_use]
pub struct ExSetSection < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationPlayer, start_time: f64, end_time: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetSection < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationPlayer,) -> Self {
        let start_time = - 1f64;
        let end_time = - 1f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start_time: start_time, end_time: end_time,
        }
    }
    #[inline]
    pub fn start_time(self, start_time: f64) -> Self {
        Self {
            start_time: start_time, .. self
        }
    }
    #[inline]
    pub fn end_time(self, end_time: f64) -> Self {
        Self {
            end_time: end_time, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start_time, end_time,
        }
        = self;
        re_export::AnimationPlayer::set_section_full(surround_object, start_time, end_time,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::seek_ex`][super::AnimationPlayer::seek_ex]."]
#[must_use]
pub struct ExSeek < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AnimationPlayer, seconds: f64, update: bool, update_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSeek < 'ex > {
    fn new(surround_object: &'ex mut re_export::AnimationPlayer, seconds: f64,) -> Self {
        let update = false;
        let update_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, seconds: seconds, update: update, update_only: update_only,
        }
    }
    #[inline]
    pub fn update(self, update: bool) -> Self {
        Self {
            update: update, .. self
        }
    }
    #[inline]
    pub fn update_only(self, update_only: bool) -> Self {
        Self {
            update_only: update_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, seconds, update, update_only,
        }
        = self;
        re_export::AnimationPlayer::seek_full(surround_object, seconds, update, update_only,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationProcessCallback {
    ord: i32
}
impl AnimationProcessCallback {
    #[doc(alias = "ANIMATION_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `ANIMATION_PROCESS_PHYSICS`"]
    pub const PHYSICS: AnimationProcessCallback = AnimationProcessCallback {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `ANIMATION_PROCESS_IDLE`"]
    pub const IDLE: AnimationProcessCallback = AnimationProcessCallback {
        ord: 1i32
    };
    #[doc(alias = "ANIMATION_PROCESS_MANUAL")]
    #[doc = "Godot enumerator name: `ANIMATION_PROCESS_MANUAL`"]
    pub const MANUAL: AnimationProcessCallback = AnimationProcessCallback {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AnimationProcessCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationProcessCallback") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationProcessCallback {
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
            Self::PHYSICS => "PHYSICS", Self::IDLE => "IDLE", Self::MANUAL => "MANUAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AnimationProcessCallback::PHYSICS, AnimationProcessCallback::IDLE, AnimationProcessCallback::MANUAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AnimationProcessCallback >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PHYSICS", "ANIMATION_PROCESS_PHYSICS", AnimationProcessCallback::PHYSICS), crate::meta::inspect::EnumConstant::new("IDLE", "ANIMATION_PROCESS_IDLE", AnimationProcessCallback::IDLE), crate::meta::inspect::EnumConstant::new("MANUAL", "ANIMATION_PROCESS_MANUAL", AnimationProcessCallback::MANUAL)]
        }
    }
}
impl crate::meta::GodotConvert for AnimationProcessCallback {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Animation Process Physics", 0i64), EnumeratorShape::new_int("Animation Process Idle", 1i64), EnumeratorShape::new_int("Animation Process Manual", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AnimationPlayer.AnimationProcessCallback")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AnimationProcessCallback {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationProcessCallback {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AnimationProcessCallback {
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
impl crate::registry::property::Export for AnimationProcessCallback {
    
}
impl crate::meta::Element for AnimationProcessCallback {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationMethodCallMode {
    ord: i32
}
impl AnimationMethodCallMode {
    #[doc(alias = "ANIMATION_METHOD_CALL_DEFERRED")]
    #[doc = "Godot enumerator name: `ANIMATION_METHOD_CALL_DEFERRED`"]
    pub const DEFERRED: AnimationMethodCallMode = AnimationMethodCallMode {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_METHOD_CALL_IMMEDIATE")]
    #[doc = "Godot enumerator name: `ANIMATION_METHOD_CALL_IMMEDIATE`"]
    pub const IMMEDIATE: AnimationMethodCallMode = AnimationMethodCallMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AnimationMethodCallMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationMethodCallMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationMethodCallMode {
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
            Self::DEFERRED => "DEFERRED", Self::IMMEDIATE => "IMMEDIATE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AnimationMethodCallMode::DEFERRED, AnimationMethodCallMode::IMMEDIATE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AnimationMethodCallMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFERRED", "ANIMATION_METHOD_CALL_DEFERRED", AnimationMethodCallMode::DEFERRED), crate::meta::inspect::EnumConstant::new("IMMEDIATE", "ANIMATION_METHOD_CALL_IMMEDIATE", AnimationMethodCallMode::IMMEDIATE)]
        }
    }
}
impl crate::meta::GodotConvert for AnimationMethodCallMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Animation Method Call Deferred", 0i64), EnumeratorShape::new_int("Animation Method Call Immediate", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AnimationPlayer.AnimationMethodCallMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AnimationMethodCallMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationMethodCallMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AnimationMethodCallMode {
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
impl crate::registry::property::Export for AnimationMethodCallMode {
    
}
impl crate::meta::Element for AnimationMethodCallMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationPlayer;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AnimationPlayer`][crate::classes::AnimationPlayer] class."]
    pub struct SignalsOfAnimationPlayer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAnimationPlayer < 'c, C > {
        #[doc = "Signature: `(name: StringName)`"]
        pub fn current_animation_changed(&mut self) -> SigCurrentAnimationChanged < 'c, C > {
            SigCurrentAnimationChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "current_animation_changed")
            }
        }
        #[doc = "Signature: `(old_name: StringName, new_name: StringName)`"]
        pub fn animation_changed(&mut self) -> SigAnimationChanged < 'c, C > {
            SigAnimationChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "animation_changed")
            }
        }
    }
    type TypedSigCurrentAnimationChanged < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigCurrentAnimationChanged < 'c, C: WithSignals > {
        typed: TypedSigCurrentAnimationChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCurrentAnimationChanged < 'c, C > {
        pub fn emit(&mut self, name: StringName,) {
            self.typed.emit_tuple((name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCurrentAnimationChanged < 'c, C > {
        type Target = TypedSigCurrentAnimationChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCurrentAnimationChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationChanged < 'c, C > = TypedSignal < 'c, C, (StringName, StringName,) >;
    pub struct SigAnimationChanged < 'c, C: WithSignals > {
        typed: TypedSigAnimationChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationChanged < 'c, C > {
        pub fn emit(&mut self, old_name: StringName, new_name: StringName,) {
            self.typed.emit_tuple((old_name, new_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationChanged < 'c, C > {
        type Target = TypedSigAnimationChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AnimationPlayer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationPlayer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAnimationPlayer < 'c, C > {
        type Target = < < AnimationPlayer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AnimationPlayer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAnimationPlayer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AnimationPlayer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}